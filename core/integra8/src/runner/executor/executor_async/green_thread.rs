use std::panic::UnwindSafe;
use std::time::Instant;
use std::sync::Arc;

use futures::FutureExt;

use crate::components::{TestParameters, ExecutionArtifacts};
use crate::results::report::ComponentReportBuilder;
use crate::runner::notify::ComponentProgressNotify;
use crate::runner::ComponentFixture;

pub struct GreenThreadExecutor;

impl GreenThreadExecutor
{
    /// Executes a fixture on a new green thread and returns a populated report as a result
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
    >(
        &self,
        progress_notify: ProgressNotify,
        fixture: ComponentFixture<TParameters>,
        mut report_builder: ComponentReportBuilder,
    ) -> ComponentReportBuilder {
        let execution_artifacts_local = Arc::new(ExecutionArtifacts::new());
        let execution_artifacts_test = execution_artifacts_local.clone();
        
        progress_notify.notify_started().await;
        let start_time = Instant::now();

        let may_panic = crate::async_runtime::spawn(async move {
            std::panic::AssertUnwindSafe(fixture.run(execution_artifacts_test)).catch_unwind().await
        });

        let maybe_time_out = report_builder.time_until_deadline(start_time.elapsed());

        let result = match maybe_time_out {
            Some(time_out) => crate::async_runtime::timeout(time_out, may_panic).await,
            None => Ok(may_panic.await),
        };

        report_builder.time_taken(start_time.elapsed());

        match result {
            // Panic

            #[cfg(feature = "tokio-runtime")]
            Ok(Ok(Err(panic))) => {
                execution_artifacts_local.include_panic("panic", &panic);
                report_builder.rejected_result();
            }

            Ok(Err(panic)) => {
                execution_artifacts_local.include_panic("panic", &panic);
                report_builder.rejected_result();
            }

            // Timeout
            Err(_) => {     
                progress_notify.notify_timed_out().await;      
                report_builder.timed_out_result();
            }

            Ok(Ok(_)) => {
                report_builder.passed_result();
            }
        }

        report_builder.with_artifacts(&execution_artifacts_local);
        report_builder
    }
}
