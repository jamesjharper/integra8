use crate::channel::ResultsSource;
use crate::components::ComponentDescription;

use crate::runner::notify::{ComponentProgressNotify, RunProgressNotify};

use std::future::Future;
use std::pin::Pin;

use integra8_results::ComponentTimeResult;
use integra8_results::report::ComponentRunReport;
use integra8_results::summary::ComponentTypeCountSummary;

#[derive(Clone)]
pub struct RunProgressChannelNotify {
    result_publisher: ResultsSource,
}

impl RunProgressChannelNotify {
    pub fn new(result_publisher: ResultsSource) -> Self {
        Self {
            result_publisher: result_publisher,
        }
    }
}

impl RunProgressNotify for RunProgressChannelNotify {
    type ComponentProgressNotify = ComponentProgressChannelNotify;

    fn notify_run_start(
        &self,
        summary: ComponentTypeCountSummary
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_run_complete(inner_self: &RunProgressChannelNotify, summary: ComponentTypeCountSummary) {
            inner_self.result_publisher.notify_run_start(summary).await
        }

        Box::pin(notify_run_complete(self, summary))
    }

    fn notify_run_complete(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_run_complete(inner_self: &RunProgressChannelNotify) {
            inner_self.result_publisher.notify_run_complete().await
        }

        Box::pin(notify_run_complete(self))
    }

    fn notify_component_start(
        &self,
        description: ComponentDescription,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_component_start(
            inner_self: &RunProgressChannelNotify,
            description: ComponentDescription,
        ) {
            inner_self
                .result_publisher
                .notify_component_start(description)
                .await
        }

        Box::pin(notify_component_start(self, description))
    }

    fn notify_component_timed_out(
        &self,
        description: ComponentDescription,
        timing_result: ComponentTimeResult,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_component_timed_out(
            inner_self: &RunProgressChannelNotify,
            description: ComponentDescription,
            timing_result: ComponentTimeResult,
        ) {
            inner_self
                .result_publisher
                .notify_component_timed_out(description, timing_result)
                .await
        }

        Box::pin(notify_component_timed_out(self, description, timing_result))
    }

    fn notify_component_complete(
        &self,
        report: ComponentRunReport,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_component_complete(
            inner_self: &RunProgressChannelNotify,
            report: ComponentRunReport,
        ) {
            inner_self
                .result_publisher
                .notify_component_complete(report)
                .await
        }

        Box::pin(notify_component_complete(self, report))
    }

    fn component_process_notify(
        &self,
        description: ComponentDescription,
    ) -> Self::ComponentProgressNotify {
        ComponentProgressChannelNotify::new(self.result_publisher.clone(), description)
    }
}

#[derive(Clone)]
pub struct ComponentProgressChannelNotify {
    result_publisher: ResultsSource,
    description: ComponentDescription,
}

impl ComponentProgressChannelNotify {
    pub fn new(result_publisher: ResultsSource, description: ComponentDescription) -> Self {
        Self {
            result_publisher: result_publisher,
            description: description,
        }
    }
}

impl ComponentProgressNotify for ComponentProgressChannelNotify {
    fn notify_started(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        async fn notify_started(inner_self: &ComponentProgressChannelNotify) {
            inner_self
                .result_publisher
                .notify_component_start(inner_self.description.clone())
                .await
        }

        Box::pin(notify_started(self))
    }

    fn notify_timed_out(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let fut = async {
            /*if timing_results.is_critical() {
                self.result_publisher.notify_component_timed_out(
                    self.description.clone(),
                    result.clone()
                ).await;
            }*/
        };
        Box::pin(fut)
    }

    fn notify_complete(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        let fut = async {
            //...
        };
        Box::pin(fut)
    }
}
