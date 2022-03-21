use crate::core::channel::TestEvent;

use crate::async_runtime::Sender;
use crate::results::report::ComponentRunReport;
use crate::results::summary::ComponentTypeCountSummary;

use crate::components::ComponentDescription;

#[derive(Clone)]
pub struct ResultsSource {
    pub tx: Sender<TestEvent>,
}

impl ResultsSource {
    // Run
    pub async fn notify_run_start(&self, summary: ComponentTypeCountSummary) {
        self.send(TestEvent::NotifyRunStart { summary }).await
    }

    pub async fn notify_run_complete(&self) {
        self.send(TestEvent::NotifyRunComplete).await
    }

    pub async fn notify_component_start(&self, description: ComponentDescription) {
        self.send(TestEvent::NotifyComponentStart { description })
            .await
    }

    pub async fn notify_component_timed_out(&self, description: ComponentDescription) {
        self.send(TestEvent::NotifyComponentTimeout { description })
            .await
    }

    pub async fn notify_component_report_complete(&self, report: ComponentRunReport) {
        self.send(TestEvent::NotifyComponentReportComplete { report })
            .await
    }

    pub async fn send(&self, event: TestEvent) {
        self.tx.send(event).await.unwrap();
    }
}
