use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;
use std::time::Instant;
use std::sync::Arc;

use futures::FutureExt;

use super::Executor;

use crate::notify::ComponentProgressNotify;
use crate::ComponentFixture;
use integra8_components::{TestParameters, ExecutionArtifacts};
use integra8_results::report::ComponentReportBuilder;

pub struct AsyncTaskExecutor;

impl<
        TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
        ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
    > Executor<TParameters, ProgressNotify> for AsyncTaskExecutor
{
    fn execute<'async_trait>(
        &'async_trait self,
        progress_notify: ProgressNotify,
        fixture: ComponentFixture<TParameters>,
        report_builder: ComponentReportBuilder,
    ) -> Pin<Box<dyn Future<Output = ComponentReportBuilder> + Send + 'async_trait>> {
        async fn run_with_new_task<
            T: TestParameters + Send + Sync + UnwindSafe + 'static,
            N: ComponentProgressNotify,
        >(
            progress_notify: N,
            fixture: ComponentFixture<T>,
            mut report_builder: ComponentReportBuilder,
        ) -> ComponentReportBuilder {
            let execution_artifacts_local = Arc::new(ExecutionArtifacts::new());
            let execution_artifacts_test = execution_artifacts_local.clone();
            
            progress_notify.notify_started().await;
            let start_time = Instant::now();

            let may_panic = integra8_async_runtime::spawn(async move {
                    std::panic::AssertUnwindSafe(fixture.run(execution_artifacts_test)).catch_unwind().await
            });

            let maybe_time_out = report_builder.time_until_deadline(start_time.elapsed());

            let result = match maybe_time_out {
                Some(time_out) => integra8_async_runtime::timeout(time_out, may_panic).await,
                None => Ok(may_panic.await),
            };

            report_builder.time_taken(start_time.elapsed());

            if let Err(_) = result {
                progress_notify.notify_timed_out().await;
            }

            match result {
                Err(_timeout) => {            
                    report_builder.rejected_result();
                }
                #[cfg(feature = "tokio-runtime")]
                Ok(Ok(Err(panic))) => {
                    execution_artifacts_local.include_panic("panic", &panic);
                    report_builder.rejected_result();
                }
                #[cfg(feature = "async-std-runtime")]
                Ok(Err(panic))=> {
                    execution_artifacts_local.include_panic("panic", &panic);
                    report_builder.rejected_result();
                }
                _ => {
                    report_builder.passed_result();
                }
            }

            report_builder.with_artifacts(&execution_artifacts_local);
            report_builder
        }

        Box::pin(run_with_new_task::<TParameters, ProgressNotify>(
            progress_notify,
            fixture,
            report_builder,
        ))
    }
}
