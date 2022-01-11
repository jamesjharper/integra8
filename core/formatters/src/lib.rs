

pub mod none;
pub mod pretty;
pub mod tree;

use std::error::Error;
use std::io::Write;

use integra8_components::ComponentDescription;

use integra8_results::report::ComponentRunReport;
use integra8_results::summary::RunSummary;
use integra8_results::ComponentTimeResult;

pub trait FormatterParameters {
    fn create_formatter(&self) -> Option<Box<dyn OutputFormatter>>;
}

pub trait OutputFormatterFactory {
    type FormatterParameters;
    fn create<T>(
        formatter_parameters: &Self::FormatterParameters,
        test_parameters: &T,
    ) -> Box<dyn OutputFormatter>;
}

pub trait OutputFormatter {
    // run

    fn write_run_start(&mut self, _test_count: usize) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_run_complete(&mut self, _summary: &RunSummary) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Component

    fn write_component_start(
        &mut self,
        _desc: &ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_component_timeout(
        &mut self,
        _desc: &ComponentDescription,
        _result_timings: &ComponentTimeResult,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_component_report(
        &mut self,
        _report: &ComponentRunReport,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Suite

    fn write_suite_start(&mut self, _desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_suite_timeout(
        &mut self,
        _desc: &ComponentDescription,
        _result_timings: &ComponentTimeResult,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_suite_report(&mut self, _report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Setup

    fn write_setup_start(&mut self, _desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_setup_timeout(
        &mut self,
        _desc: &ComponentDescription,
        _result_timings: &ComponentTimeResult,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_setup_report(&mut self, _report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Tear Down

    fn write_tear_down_start(
        &mut self,
        _desc: &ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_tear_down_timeout(
        &mut self,
        _desc: &ComponentDescription,
        _result_timings: &ComponentTimeResult,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_tear_down_report(
        &mut self,
        _report: &ComponentRunReport,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Test

    fn write_test_start(&mut self, _desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_test_timeout(
        &mut self,
        _desc: &ComponentDescription,
        _result_timings: &ComponentTimeResult,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_test_report(&mut self, _report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub enum OutputLocation {
    Pretty(Box<term::StdoutTerminal>),
    Raw(Box<dyn Write>),
}

impl OutputLocation {
    pub fn write_pretty<S: AsRef<str>>(
        &mut self,
        s: S,
        color: term::color::Color,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            OutputLocation::Pretty(ref mut term) => {
                term.fg(color)?;
                term.write_all(s.as_ref().as_bytes())?;
                term.reset()?;
                term.flush()?;
            }
            OutputLocation::Raw(ref mut _stdout) => {
                self.write_plain(s)?;
            }
        }
        Ok(())
    }

    pub fn write_plain<S: AsRef<str>>(&mut self, s: S) -> Result<(), Box<dyn Error>> {
        self.write_all(s.as_ref().as_bytes())?;
        self.flush()?;
        Ok(())
    }
}

impl Write for OutputLocation {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match *self {
            OutputLocation::Pretty(ref mut term) => term.write(buf),
            OutputLocation::Raw(ref mut stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match *self {
            OutputLocation::Pretty(ref mut term) => term.flush(),
            OutputLocation::Raw(ref mut stdout) => stdout.flush(),
        }
    }
}
