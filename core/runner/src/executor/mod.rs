use std::panic::UnwindSafe;

use integra8_context::parameters::TestParameters;
use integra8_results::report::ComponentReportBuilder;

use crate::notify::ComponentProgressNotify;
use crate::ComponentFixture;

#[cfg(feature = "async")]
pub use executor_async::Executor;

#[cfg(feature = "async")]
pub fn process_external_executor<
    TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
    ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
>() -> impl Executor<TParameters, ProgressNotify> {
    executor_async::process::AsyncProcessExecutor {}
}

#[cfg(feature = "async")]
pub fn process_internal_executor<
    TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
    ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
>() -> impl Executor<TParameters, ProgressNotify> {
    executor_async::task::AsyncTaskExecutor {}
}

#[cfg(feature = "async")]
mod executor_async {
    use super::*;

    pub mod process;
    pub mod task;

    use std::future::Future;
    use std::pin::Pin;

    pub trait Executor<
        TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
        ProgressNotify: ComponentProgressNotify + Send + Sync + 'static,
    > {
        fn execute<'async_trait>(
            &'async_trait self,
            progress_notify: ProgressNotify,
            fixture: ComponentFixture<TParameters>,
            report_builder: ComponentReportBuilder,
        ) -> Pin<Box<dyn Future<Output = ComponentReportBuilder> + Send + 'async_trait>>;
    }
}

#[cfg(feature = "sync")]
pub type Executor<TParameters, ProgressNotify> = executor_sync_impl::Executor<TParameters, ProgressNotify>;

#[cfg(feature = "sync")]
mod test_sync_impl {
    use super::*;

    pub trait Executor<
        TParameters: TestParameters + Send + Sync + UnwindSafe + 'static,
        ProgressNotify: ComponentProgressNotify + Send + Sync
    > {
        fn execute(
            &self,
            progress_notify: ProgressNotify,
            fixture: ComponentFixture<TParameters>,
            report_builder: ComponentReportBuilder,
        ) -> ComponentReportBuilder;
    }
}
