pub mod sink;
pub use sink::{ResultsOutputWriterSink, ResultsSink};

pub mod source;
pub use source::ResultsSource;

pub mod notify;
pub use notify::{ComponentProgressChannelNotify, RunProgressChannelNotify};

use crate::async_runtime::channel;

use crate::components::ComponentDescription;
use crate::results::report::ComponentRunReport;
use crate::results::summary::ComponentTypeCountSummary;

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
