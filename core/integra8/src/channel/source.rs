use crate::async_runtime::Sender;
use crate::channel::TestEvent;
use crate::components::ComponentDescription;
use crate::results::report::ComponentRunReport;
use crate::results::ComponentTimeResult;

#[derive(Clone)]
pub struct ResultsSource {
    pub tx: Sender<TestEvent>,
}

impl ResultsSource {
    // Run
    pub async fn notify_run_start(
        &self,
        test_count: usize,
        suite_count: usize,
        tear_down_count: usize,
        setup_count: usize,
    ) {
        self.send(TestEvent::NotifyRunStart {
            test_count,
            suite_count,
            tear_down_count,
            setup_count,
        })
        .await
    }

    pub async fn notify_run_complete(&self) {
        self.send(TestEvent::NotifyRunComplete).await
    }

    pub async fn notify_component_start(&self, description: ComponentDescription) {
        self.send(TestEvent::NotifyComponentStart { description })
            .await
    }

    pub async fn notify_component_timed_out(
        &self,
        description: ComponentDescription,
        timing_result: ComponentTimeResult,
    ) {
        self.send(TestEvent::NotifyComponentTimeout {
            description,
            timing_result,
        })
        .await
    }

    pub async fn notify_component_complete(&self, report: ComponentRunReport) {
        self.send(TestEvent::NotifyComponentComplete { report })
            .await
    }

    pub async fn send(&self, event: TestEvent) {
        // TODO: NO
        self.tx.send(event).await.unwrap();
    }
}
