pub mod none;


pub mod models {
    pub use integra8_results::*;
    pub use integra8_components::{ComponentDescription, ComponentLocation, ComponentType};
}

use std::error::Error;
use std::io::Write;

use models::ComponentDescription;
use models::report::ComponentRunReport;
use models::summary::{RunSummary, ComponentTypeCountSummary};
use models::ComponentTimeResult;

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

    fn write_run_start(&mut self, _summary: &ComponentTypeCountSummary) -> Result<(), Box<dyn Error>> {
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


    /*pub fn write_plain<S: AsRef<str>>(&mut self, s: S) -> Result<(), Box<dyn Error>> {
        self.write_all(s.as_ref().as_bytes())?;
        self.flush()?;
        Ok(())
    }*/

    pub fn write_plain<S: std::fmt::Display>(&mut self, s: S) -> Result<(), Box<dyn Error>> {
        match *self {
            OutputLocation::Pretty(ref mut term) => {
                write!(term, "{}", s)?;
            },
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






/*
pub struct IndentedTextWriter<W: io::Write> {
    prefix: String,
    next_prefix: String,
    w: W
}

impl<W: io::Write> IndentedTextWriter<W> {
    pub fn new(w : W) -> Self {
        Self {
            w: w
            prefix: 
        }
    }

    pub fn writeln<S: AsRef<str>>(
        &mut self,
        s: S
    ) -> Result<(), Box<dyn Error>> {
        
    }
}



pub struct PrefixSegment<'a> {
    segments: Vec<&'a str>
}

*/

/*
std::collections::VecDeque

pub struct IndentedSegment {
    deque: Vec<VecDeque>
}

impl IndentedSegment {
    pub fn with(s : impl Into<String>) -> Self {
        Self {
            seq: deque::from([s.into()])
        }
    }

    pub fn and_then(mut self, this : impl Into<String>) -> Self {
        self.deque.push_back(this.into());
        self
    }

    pub take_next(&mut self) -> String {
        pop_front
    }
}



use std::fmt; // Import `fmt`

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}
*/