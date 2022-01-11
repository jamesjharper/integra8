pub mod sink;
pub use sink::{ResultsOutputWriterSink, ResultsSink};

pub mod source;
pub use source::ResultsSource;

pub mod notify;
pub use notify::{ComponentProgressChannelNotify, RunProgressChannelNotify};

use integra8_async_runtime::channel;

use crate::components::ComponentDescription;
use crate::results::report::ComponentRunReport;
use crate::results::ComponentTimeResult;

#[derive(Debug)]
pub enum TestEvent {
    // Run
    NotifyRunStart {
        test_count: usize,
        suite_count: usize,
        tear_down_count: usize,
        setup_count: usize,
    },

    NotifyRunComplete,

    NotifyComponentStart {
        description: ComponentDescription,
    },

    NotifyComponentTimeout {
        description: ComponentDescription,
        timing_result: ComponentTimeResult,
    },

    NotifyComponentComplete {
        report: ComponentRunReport,
    },
}

pub struct ResultsChannel;

impl ResultsChannel {
    pub fn new(
        sink: ResultsOutputWriterSink,
        max_concurrency: usize,
    ) -> (ResultsSource, ResultsSink) {
        let (sender, receiver) = channel::<TestEvent>(max_concurrency * 2);
        (
            ResultsSource { tx: sender },
            ResultsSink {
                rx: receiver,
                sink: sink,
            },
        )
    }
}
