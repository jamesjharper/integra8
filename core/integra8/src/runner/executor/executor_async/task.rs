use std::pin::Pin;
use std::future::Future;
use std::panic::UnwindSafe;
use std::time::Instant;

use futures::FutureExt;


use super::Executor;
use crate::async_runtime;

use crate::channel::ComponentProgressNotify;
use crate::parameters::TestParameters;
use crate::runner::ComponentFixture;
use crate::results::artifacts::ComponentRunArtifacts;

use crate::results::{ComponentReportBuilder, artifacts::stdio::TestResultStdio};

pub struct AsyncTaskExecutor;

impl<TParameters: TestParameters + Send + Sync + UnwindSafe + 'static> Executor<TParameters> for AsyncTaskExecutor {
    fn execute<'async_trait>(
        &'async_trait self, 
        progress_notify: ComponentProgressNotify,
        fixture: ComponentFixture<TParameters>,
        report_builder: ComponentReportBuilder
    ) -> Pin<Box<dyn Future<Output = ComponentReportBuilder>  + Send + 'async_trait>> {

        async fn run_with_new_task<T: TestParameters + Send + Sync + UnwindSafe +'static>(
            progress_notify: ComponentProgressNotify,
            fixture: ComponentFixture<T>,
            mut report_builder: ComponentReportBuilder
        ) -> ComponentReportBuilder {

            progress_notify.notify_started().await;
            let start_time = Instant::now();   

            let may_panic = async_runtime::spawn(async move {
                std::panic::AssertUnwindSafe(
                    fixture.run()
                ).catch_unwind().await
            });

            let maybe_time_out = report_builder.time_until_deadline(start_time.elapsed());
    
            let result = match maybe_time_out {
                Some(time_out) => async_runtime::timeout(time_out, may_panic).await,
                None => Ok(may_panic.await)
            }; 

            report_builder.time_taken(start_time.elapsed());

            if let Err(_) = result {
                progress_notify.notify_timed_out().await;
            } else {
                progress_notify.notify_complete().await;
            }

            match result {
                Err(_timeout) => {
                    report_builder.rejected_result();
                },
                Ok(Ok(Err(panic))) => {
                    report_builder.with_artifacts(
                        ComponentRunArtifacts {
                            stdio: TestResultStdio::from_panic(&panic)
                        }
                    );
                    report_builder.rejected_result();
                },
                _ => {
                    report_builder.passed_result();
                }
            }
            report_builder
        }

        Box::pin(run_with_new_task::<TParameters>(progress_notify, fixture, report_builder))      
    }
}
