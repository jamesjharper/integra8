use std::panic::UnwindSafe;

use crate::components::{ExecutionStrategy, TestParameters};
use crate::results::report::ComponentReportBuilder;

use crate::runner::notify::ComponentProgressNotify;
use crate::runner::ComponentFixture;

#[cfg(feature = "async")]
pub use executor_async::execute;

#[cfg(feature = "async")]
mod executor_async {
    use super::*;

    pub mod child_process;
    pub mod current_thread;
    pub mod green_thread;

    pub async fn execute<
        TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
        ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
    >(
        progress_notify: ProgressNotify,
        fixture: ComponentFixture<TParameters>,
        report_builder: ComponentReportBuilder,
    ) -> ComponentReportBuilder {
        match fixture.execution_strategy() {
            ExecutionStrategy::ChildProcess => {
                executor_async::child_process::ChildProcessExecutor
                    .execute(progress_notify, fixture, report_builder)
                    .await
            }
            ExecutionStrategy::CurrentThread => {
                executor_async::current_thread::CurrentThreadExecutor
                    .execute(progress_notify, fixture, report_builder)
                    .await
            }
            ExecutionStrategy::GreenThread => {
                executor_async::green_thread::GreenThreadExecutor
                    .execute(progress_notify, fixture, report_builder)
                    .await
            }
        }
    }
}

#[cfg(feature = "sync")]
pub type Executor<TParameters, ProgressNotify> =
    executor_sync_impl::Executor<TParameters, ProgressNotify>;

#[cfg(feature = "sync")]
mod test_sync_impl {
    use super::*;

    // Not implemented
}
