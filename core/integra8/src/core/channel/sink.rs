use crate::formatters::OutputFormatter;
use crate::results::summary::RunSummary;
use crate::components::{ComponentDescription, ComponentType};

use crate::results::report::ComponentRunReport;
use std::error::Error;

use crate::async_runtime::Receiver;
use crate::results::summary::ComponentTypeCountSummary;

use crate::core::channel::TestEvent;

pub struct ResultsSink {
    pub sink: ResultsOutputWriterSink,
    pub rx: Receiver<TestEvent>,
}

impl ResultsSink {
    #[cfg(any(feature = "tokio-runtime", feature = "async-std-runtime"))]
    pub async fn start_listening(mut self) -> RunSummary {
        loop {
            let msg = self.rx.recv().await.unwrap();

            if self.process_message(msg) {
                return self.sink.state;
            }
        }
    }

    #[cfg(not(any(feature = "tokio-runtime", feature = "async-std-runtime")))]
    pub fn start_listening(mut self) -> RunSummary {
        loop {
            let msg = self.rx.recv().unwrap();
            if self.process_message(msg) {
                return self.sink.state;
            }
        }
    }

    fn process_message(&mut self, msg: TestEvent) -> bool {
        match msg {
            // Run
            TestEvent::NotifyRunStart { summary } => self.sink.on_run_start(summary),
            TestEvent::NotifyComponentStart { description } => {
                self.sink.on_component_start(description)
            }

            TestEvent::NotifyComponentTimeout { description } => {
                self.sink.on_component_timed_out(description)
            }

            TestEvent::NotifyComponentReportComplete { report } => {
                self.sink.on_component_report_complete(report)
            }

            TestEvent::NotifyRunComplete => {
                self.sink.on_run_complete().unwrap();
                // close down message pump
                return true;
            }
        }
        .unwrap();
        false
    }
}

pub struct ResultsOutputWriterSink {
    state: RunSummary,
    output_writer: OutputFormatterAggregator,
}

impl ResultsOutputWriterSink {
    pub fn new(output_writer: Box<dyn OutputFormatter + 'static>) -> Self {
        Self {
            state: RunSummary::new(),
            output_writer: OutputFormatterAggregator::new(output_writer),
        }
    }
}

impl ResultsOutputWriterSink {
    // Run
    pub fn on_run_start(
        &mut self,
        summary: ComponentTypeCountSummary,
    ) -> Result<(), Box<dyn Error>> {
        self.output_writer.write_run_start(&summary)
    }

    pub fn on_run_complete(&mut self) -> Result<(), Box<dyn Error>> {
        self.output_writer.write_run_complete(&self.state)
    }

    // component
    fn on_component_start(
        &mut self,
        description: ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {
        match description.component_type() {
            ComponentType::Suite => self.output_writer.write_suite_start(&description),
            ComponentType::Test => self.output_writer.write_test_start(&description),
            ComponentType::Setup => self.output_writer.write_setup_start(&description),
            ComponentType::TearDown => self.output_writer.write_tear_down_start(&description),
        }?;
        self.output_writer.write_component_start(&description)?;
        Ok(())
    }

    pub fn on_component_timed_out(
        &mut self,
        description: ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {
        match description.component_type() {
            ComponentType::Suite => self.output_writer.write_suite_timeout(&description),
            ComponentType::Test => self.output_writer.write_test_timeout(&description),
            ComponentType::Setup => self.output_writer.write_setup_timeout(&description),
            ComponentType::TearDown => self.output_writer.write_tear_down_timeout(&description),
        }?;
        self.output_writer.write_component_timeout(&description)?;
        Ok(())
    }

    pub fn on_component_report_complete(
        &mut self,
        report: ComponentRunReport,
    ) -> Result<(), Box<dyn Error>> {
        let result = match report.description.component_type() {
            ComponentType::Suite => self.output_writer.write_suite_report(&report),
            ComponentType::Test => self.output_writer.write_test_report(&report),
            ComponentType::Setup => self.output_writer.write_setup_report(&report),
            ComponentType::TearDown => self.output_writer.write_tear_down_report(&report),
        }
        .and_then(|_| self.output_writer.write_component_report(&report));

        self.state.push_report(report);
        result
    }
}

pub struct OutputFormatterAggregator {
    output_writers: Vec<Box<dyn OutputFormatter + 'static>>,
}

impl OutputFormatterAggregator {
    pub fn new(output_writer: Box<dyn OutputFormatter + 'static>) -> Self {
        Self {
            output_writers: vec![output_writer],
        }
    }
}

impl OutputFormatter for OutputFormatterAggregator {
    // run
    fn write_run_start(
        &mut self,
        summary: &ComponentTypeCountSummary,
    ) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_run_start(summary)?;
        }
        Ok(())
    }

    fn write_run_complete(&mut self, summary: &RunSummary) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_run_complete(summary)?;
        }
        Ok(())
    }

    // Component

    fn write_component_start(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_component_start(desc)?;
        }
        Ok(())
    }

    fn write_component_timeout(
        &mut self,
        desc: &ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_component_timeout(desc)?;
        }
        Ok(())
    }

    fn write_component_report(
        &mut self,
        report: &ComponentRunReport,
    ) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_component_report(report)?;
        }
        Ok(())
    }

    // Suite

    fn write_suite_start(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_suite_start(desc)?;
        }
        Ok(())
    }

    fn write_suite_timeout(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_suite_timeout(desc)?;
        }
        Ok(())
    }

    fn write_suite_report(&mut self, report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_suite_report(report)?;
        }
        Ok(())
    }

    // Setup

    fn write_setup_start(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_setup_start(desc)?;
        }
        Ok(())
    }

    fn write_setup_timeout(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_setup_timeout(desc)?;
        }
        Ok(())
    }

    fn write_setup_report(&mut self, report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_setup_report(report)?;
        }
        Ok(())
    }

    // Tear Down

    fn write_tear_down_start(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_tear_down_start(desc)?;
        }
        Ok(())
    }

    fn write_tear_down_timeout(
        &mut self,
        desc: &ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_tear_down_timeout(desc)?;
        }
        Ok(())
    }

    fn write_tear_down_report(
        &mut self,
        report: &ComponentRunReport,
    ) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_tear_down_report(report)?;
        }
        Ok(())
    }

    // Test

    fn write_test_start(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_test_start(desc)?;
        }
        Ok(())
    }

    fn write_test_timeout(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_test_timeout(desc)?;
        }
        Ok(())
    }

    fn write_test_report(&mut self, report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        for o in &mut self.output_writers {
            o.write_test_report(report)?;
        }
        Ok(())
    }
}
