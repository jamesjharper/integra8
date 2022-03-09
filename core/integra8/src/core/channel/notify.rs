use crate::components::ComponentDescription;
use crate::core::channel::ResultsSource;

use crate::runner::notify::{ComponentProgressNotify, RunProgressNotify};

use std::future::Future;
use std::pin::Pin;

use integra8_results::report::ComponentRunReport;
use integra8_results::summary::ComponentTypeCountSummary;

#[derive(Clone)]
pub struct RunProgressChannelNotify {
    result_source: ResultsSource,
}

impl RunProgressChannelNotify {
    pub fn new(result_source: ResultsSource) -> Self {
        Self {
            result_source: result_source,
        }
    }
}

impl RunProgressNotify for RunProgressChannelNotify {
    type ComponentProgressNotify = ComponentProgressChannelNotify;

    fn notify_run_start(
        &self,
        summary: ComponentTypeCountSummary,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_run_complete(
            inner_self: &RunProgressChannelNotify,
            summary: ComponentTypeCountSummary,
        ) {
            inner_self.result_source.notify_run_start(summary).await
        }

        Box::pin(notify_run_complete(self, summary))
    }

    fn notify_run_complete(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_run_complete(inner_self: &RunProgressChannelNotify) {
            inner_self.result_source.notify_run_complete().await
        }

        Box::pin(notify_run_complete(self))
    }

    fn notify_component_report_complete(
        &self,
        report: ComponentRunReport,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_component_report_complete(
            inner_self: &RunProgressChannelNotify,
            report: ComponentRunReport,
        ) {
            inner_self
                .result_source
                .notify_component_report_complete(report)
                .await
        }

        Box::pin(notify_component_report_complete(self, report))
    }

    fn component_process_notify(
        &self,
        description: ComponentDescription,
    ) -> Self::ComponentProgressNotify {
        ComponentProgressChannelNotify::new(self.result_source.clone(), description)
    }
}

#[derive(Clone)]
pub struct ComponentProgressChannelNotify {
    result_source: ResultsSource,
    description: ComponentDescription,
}

impl ComponentProgressChannelNotify {
    pub fn new(result_source: ResultsSource, description: ComponentDescription) -> Self {
        Self {
            result_source: result_source,
            description: description,
        }
    }
}

impl ComponentProgressNotify for ComponentProgressChannelNotify {
    fn notify_started(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_started(inner_self: &ComponentProgressChannelNotify) {
            inner_self
                .result_source
                .notify_component_start(inner_self.description.clone())
                .await
        }

        Box::pin(notify_started(self))
    }

    fn notify_timed_out(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_timed_out(inner_self: &ComponentProgressChannelNotify) {
            inner_self
                .result_source
                .notify_component_timed_out(inner_self.description.clone())
                .await
        }

        Box::pin(notify_timed_out(self))
    }
}
