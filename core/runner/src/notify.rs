use std::future::Future;
use std::pin::Pin;

use integra8_context::meta::ComponentDescription;
use integra8_results::ComponentTimeResult;
use integra8_results::report::ComponentRunReport;

pub trait ComponentProgressNotify {
    fn notify_started(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_timed_out(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_complete(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}


pub trait RunProgressNotify {

    type ComponentProgressNotify: ComponentProgressNotify + Send + Sync;

    fn notify_run_start(
        &self,
        test_count: usize,
        suite_count: usize,
        tear_down_count: usize,
        setup_count: usize,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;


    fn notify_run_complete(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_component_start(&self, description: ComponentDescription) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_component_timed_out(
        &self,
        description: ComponentDescription,
        timing_result: ComponentTimeResult,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_component_complete(&self, report: ComponentRunReport) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn component_process_notify(&self, description: ComponentDescription) -> Self::ComponentProgressNotify;
}