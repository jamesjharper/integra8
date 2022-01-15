pub mod sink;
pub use sink::{ResultsOutputWriterSink, ResultsSink};

pub mod source;
pub use source::ResultsSource;

pub mod notify;
pub use notify::{ComponentProgressChannelNotify, RunProgressChannelNotify};

use integra8_async_runtime::channel;

use integra8_components::ComponentDescription;
use integra8_results::report::ComponentRunReport;
use integra8_results::summary::ComponentTypeCountSummary;

#[derive(Debug)]
pub enum TestEvent {
    // Run
    NotifyRunStart { summary: ComponentTypeCountSummary },

    NotifyRunComplete,

    NotifyComponentStart { description: ComponentDescription },

    NotifyComponentTimeout { description: ComponentDescription },
    NotifyComponentReportComplete { report: ComponentRunReport },
}

pub struct ResultsChannel;

impl ResultsChannel {
    pub fn new(
        sink: ResultsOutputWriterSink,
        max_concurrency: usize,
    ) -> (ResultsSource, ResultsSink) {
        
        let (sender, receiver) = channel::<TestEvent>(max_concurrency * 5);
        (
            ResultsSource { tx: sender },
            ResultsSink {
                rx: receiver,
                sink: sink,
            },
        )
    }
}
