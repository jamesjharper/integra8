
use crate::components::ComponentDescription;
use crate::channel::ResultsSource;

pub struct ComponentProgressNotify {
    result_publisher: ResultsSource,
    description: ComponentDescription,
}

impl ComponentProgressNotify {
    pub fn new(
        result_publisher: ResultsSource,
        description: ComponentDescription,
    ) -> Self {

        Self {
            result_publisher: result_publisher,
            description: description,
        }
    }

    pub async fn notify_started(&self) {
        self.result_publisher.notify_component_start(
            self.description.clone()
        ).await
    }

    pub async fn notify_timed_out(&self)  {
        /*if timing_results.is_critical() {
            self.result_publisher.notify_component_timed_out(
                self.description.clone(),
                result.clone()
            ).await;
        }*/
    }

    pub async fn notify_complete(&self)  {
        /*if timing_results.is_critical() {
            self.result_publisher.notify_component_timed_out(
                self.description.clone(),
                result.clone()
            ).await;
        }*/
    }
}
