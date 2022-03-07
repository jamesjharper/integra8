use std::panic::UnwindSafe;
use std::time::Instant;

use async_process::{Command, Stdio};

use integra8_components::{ExecutionArtifacts, ChildProcessComponentArgs, ChildProcessComponentMetaArgs, ComponentType};
use integra8_results::ComponentResult;
use integra8_results::report::ComponentReportBuilder;

use crate::notify::ComponentProgressNotify;

use crate::ComponentFixture;
use integra8_components::TestParameters;

pub struct ChildProcessExecutor;

impl ChildProcessExecutor
{
    /// Executes a fixture it its own process and returns a populated report as a result
    /// 
    /// # Arguments
    ///
    /// * `progress_notify` - progress observer 
    /// 
    /// * `fixture` - the fixture to be executed
    ///
    /// * `report_builder` - a report builder, pre populated with test acceptance criteria  
    ///
    pub async fn execute<
        TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
        ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
    >(&self,
        progress_notify: ProgressNotify,
        fixture: ComponentFixture<TParameters>,
        mut report_builder: ComponentReportBuilder,
    ) -> ComponentReportBuilder {

        let child_process_target_args = fixture_into_child_process_args(fixture).to_string().unwrap();
        progress_notify.notify_started().await;
        let start_time = Instant::now();
        let mut child_process = Command::new(std::env::current_exe().unwrap())
            .kill_on_drop(true)
            .arg("--internal:child-process-target")
            .arg(child_process_target_args)
            // Replicate args passed to the original test runner
            .args(std::env::args().skip(1))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();


        let maybe_time_out = report_builder.time_until_deadline(start_time.elapsed());
        let result = match maybe_time_out {
            Some(time_out) => {
                integra8_async_runtime::timeout(time_out, child_process.status()).await
            }
            None => Ok(child_process.status().await),
        };

        report_builder.time_taken(start_time.elapsed());

        if let Err(_) = result {
            progress_notify.notify_timed_out().await;

            // Make sure the process is killed if we timed out
            child_process.kill().unwrap();
        }

        let output = child_process.output().await.unwrap();

        let execution_artifacts = ExecutionArtifacts::new();
        execution_artifacts.include_utf8_text_buffer("stdout", output.stdout);
        execution_artifacts.include_utf8_text_buffer("stderr", output.stderr);
        report_builder.with_artifacts(&execution_artifacts);

        match output.status.code() {
            Some(status) => {
                report_builder.with_result(ComponentResult::from_status_code(status));
            },
            None => {
                // On Unix, this will return None if the process was terminated by a signal.
                report_builder.rejected_result()
            }
        }

        report_builder
    }
}


fn fixture_into_child_process_args<TParameters>(fixture: ComponentFixture<TParameters>) -> ChildProcessComponentArgs {
    match fixture {
        ComponentFixture::Test { test, .. } => {
            ChildProcessComponentArgs::Test {
                attributes: test.attributes,
                meta: ChildProcessComponentMetaArgs::from_description(test.description)
            }
        },
        ComponentFixture::BookEnd { bookend, .. } => {
            match bookend.description.component_type() {
                ComponentType::Setup => {
                    ChildProcessComponentArgs::Setup {
                        attributes: bookend.attributes,
                        meta: ChildProcessComponentMetaArgs::from_description(bookend.description)
                    }
                },
                ComponentType::TearDown => {
                    ChildProcessComponentArgs::TearDown {
                        attributes: bookend.attributes,
                        meta: ChildProcessComponentMetaArgs::from_description(bookend.description)
                    }
                },
                _ => {
                    // Should be unreachable unless something is very broken
                    panic!("Description component type does not match the Fixture component type");
                }
            }
        },
        ComponentFixture::Suite { .. } => {
            panic!("Suites can not be run in a child process");
        }
    }
}