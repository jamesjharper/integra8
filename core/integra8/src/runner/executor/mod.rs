use std::panic::UnwindSafe;

use crate::runner::ComponentFixture;
use crate::parameters::TestParameters;
use crate::channel::ComponentProgressNotify;
use crate::results::ComponentReportBuilder;


#[cfg(feature = "async")]
pub use executor_async::Executor;

#[cfg(feature = "async")]
pub fn process_external_executor<TParameters: TestParameters + Send + Sync + UnwindSafe + 'static>() -> impl Executor<TParameters> {
    executor_async::process::AsyncProcessExecutor {}
}

#[cfg(feature = "async")]
pub fn process_internal_executor<TParameters: TestParameters + Send + Sync + UnwindSafe + 'static>() -> impl Executor<TParameters>  {
    executor_async::task::AsyncTaskExecutor {}
}

#[cfg(feature = "async")]
mod executor_async {
    use super::*;

    pub mod process;
    pub mod task;

    use std::pin::Pin;
    use std::future::Future;

    pub trait Executor<TParameters: TestParameters + Send + Sync + UnwindSafe +'static> {
        fn execute<'async_trait>(
            &'async_trait self, 
            progress_notify: ComponentProgressNotify,
            fixture: ComponentFixture<TParameters>,
            report_builder: ComponentReportBuilder
        ) -> Pin<Box<dyn Future<Output = ComponentReportBuilder>  + Send + 'async_trait>>;
    }
}

#[cfg(feature = "sync")]
pub type Executor<TParameters> = executor_sync_impl::Executor<TParameters>;

#[cfg(feature = "sync")]
mod test_sync_impl {
    use super::*;

    pub trait Executor<TParameters: TestParameters + Send + Sync + UnwindSafe +'static> {
        fn execute(
            &self, 
            progress_notify: ComponentProgressNotify,
            fixture: ComponentFixture<TParameters>,
            report_builder: ComponentReportBuilder
        ) -> ComponentReportBuilder;
    }
}
