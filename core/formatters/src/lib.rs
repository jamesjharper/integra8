pub mod none;

pub mod models {
    pub use integra8_components::{
        ComponentDescription, ComponentLocation, ComponentType, TestParameters,
    };
    pub use integra8_results::*;
}

use std::error::Error;
use std::io::Write;

use models::report::ComponentRunReport;
use models::summary::{ComponentTypeCountSummary, RunSummary};
use models::{ComponentDescription, TestParameters};

pub trait FormatterParameters {
    fn create_formatter(&self) -> Option<Box<dyn OutputFormatter>>;
}

pub trait OutputFormatterFactory {
    type FormatterParameters;
    fn create<T: TestParameters>(
        formatter_parameters: &Self::FormatterParameters,
        framework: &T,
    ) -> Box<dyn OutputFormatter>;

    fn default_style() -> &'static str {
        ""
    }

    fn supported_styles() -> Vec<&'static str> {
        vec![]
    }

    fn supported_detail_levels() -> Vec<&'static str> {
        vec![]
    }

    fn default_detail_levels() -> &'static str {
        ""
    }

    fn supported_encodings() -> Vec<&'static str> {
        vec![]
    }

    fn default_encoding() -> &'static str {
        ""
    }

    fn supported_ansi_modes() -> Vec<&'static str> {
        vec![]
    }

    fn default_ansi_mode() -> &'static str {
        ""
    }
}

pub trait OutputFormatter {
    // run

    fn write_run_start(
        &mut self,
        _summary: &ComponentTypeCountSummary,
    ) -> Result<(), Box<dyn Error>> {
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

    fn write_suite_timeout(&mut self, _desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_suite_report(&mut self, _report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    // Setup

    fn write_setup_start(&mut self, _desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn write_setup_timeout(&mut self, _desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
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

    fn write_test_timeout(&mut self, _desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
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
    pub fn write_pretty<S: std::fmt::Display>(
        &mut self,
        s: S,
        color: term::color::Color,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            OutputLocation::Pretty(ref mut term) => {
                term.fg(color)?;
                write!(term, "{}", s)?;
                // term.write_all(s.as_ref().as_bytes())?;
                term.reset()?;
                term.flush()?;
            }
            OutputLocation::Raw(ref mut _stdout) => {
                self.write_plain(s)?;
            }
        }
        Ok(())
    }


    pub fn write_plain<S: std::fmt::Display>(&mut self, s: S) -> Result<(), Box<dyn Error>> {
        match *self {
            OutputLocation::Pretty(ref mut term) => {
                write!(term, "{}", s)?;
            }
            OutputLocation::Raw(ref mut stdout) => {
                write!(stdout, "{}", s)?;
            }
        }

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
