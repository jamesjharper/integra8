pub mod sink;
pub use sink::{ResultsOutputWriterSink, ResultsSink};

pub mod source;
pub use source::ResultsSource;

pub mod observers;
pub use observers::ComponentProgressNotify;

use crate::async_runtime::channel;

use crate::components::ComponentDescription;
use crate::results::{ComponentRunReport, ComponentTimeResult};

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
        let (sender, receiver) = channel::<TestEvent>(max_concurrency * 10);
        (
            ResultsSource { tx: sender },
            ResultsSink {
                rx: receiver,
                sink: sink,
            },
        )
    }
}
