use std::pin::Pin;
use std::future::Future;
use std::panic::UnwindSafe;
use std::time::Instant;

use async_process::{Command, Stdio};

use super::Executor;
use crate::async_runtime;

use crate::results::stdio::TestResultStdio;
use crate::results::ComponentReportBuilder;
use crate::results::ComponentRunArtifacts;
use crate::channel::ComponentProgressNotify;

use crate::parameters::TestParameters;
use crate::runner::ComponentFixture;

pub struct AsyncProcessExecutor;

impl<TParameters: TestParameters + Send + Sync + UnwindSafe +'static> Executor<TParameters> for AsyncProcessExecutor {
    fn execute<'async_trait>(
        &'async_trait self, 
        progress_notify: ComponentProgressNotify,
        fixture: ComponentFixture<TParameters>,
        report_builder: ComponentReportBuilder
    ) -> Pin<Box<dyn Future<Output = ComponentReportBuilder>  + Send + 'async_trait>> {

        async fn run_with_new_process<T: TestParameters>(
            progress_notify: ComponentProgressNotify,
            fixture: ComponentFixture<T>,
            mut report_builder: ComponentReportBuilder
        ) -> ComponentReportBuilder {

            progress_notify.notify_started().await;
            let start_time = Instant::now();          
            
            let mut child_process = Command::new(std::env::current_exe().unwrap())
                .kill_on_drop(true)
                .arg("--child-process")
                .arg("--filter")
                .arg(fixture.component_path())
                .args(std::env::args().skip(1))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
    
            let maybe_time_out = report_builder.time_until_deadline(start_time.elapsed());
            let result = match maybe_time_out {
                Some(time_out) => async_runtime::timeout(time_out, child_process.status()).await,
                None => Ok(child_process.status().await)
            }; 
    
            report_builder.time_taken(start_time.elapsed());
            

            if let Err(_) = result {
                progress_notify.notify_timed_out().await;

                // Make sure the process is killed if we timed out
                child_process.kill().unwrap();
            } else {
                progress_notify.notify_complete().await;
            }
    
            let output = child_process.output().await.unwrap();
    
            report_builder.with_artifacts(
                ComponentRunArtifacts {
                    stdio: TestResultStdio {
                        stdout: output.stdout,
                        stderr: output.stderr,
                    }
                }
            );
    
            if output.status.success() {
                report_builder.passed_result();
            } else { 
                report_builder.rejected_result();
            }
            report_builder
        }

        Box::pin(run_with_new_process::<TParameters>(progress_notify, fixture, report_builder))      
    }
}