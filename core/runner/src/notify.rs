use std::future::Future;
use std::pin::Pin;

use integra8_components::ComponentDescription;
use integra8_results::report::ComponentRunReport;
use integra8_results::summary::ComponentTypeCountSummary;

pub trait ComponentProgressNotify {
    fn notify_started(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_timed_out(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

pub trait RunProgressNotify {
    type ComponentProgressNotify: ComponentProgressNotify + Send + Sync;

    fn notify_run_start(
        &self,
        summary: ComponentTypeCountSummary,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_run_complete(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn notify_component_report_complete(
        &self,
        report: ComponentRunReport,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;

    fn component_process_notify(
        &self,
        description: ComponentDescription,
    ) -> Self::ComponentProgressNotify;
}


#[derive(Clone)]
pub struct NullComponentProgressChannelNotify;

impl ComponentProgressNotify for NullComponentProgressChannelNotify {
    fn notify_started(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn do_nothing() {  }
        Box::pin(do_nothing())
    }

    fn notify_timed_out(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn do_nothing() {  }
        Box::pin(do_nothing())
    }
}