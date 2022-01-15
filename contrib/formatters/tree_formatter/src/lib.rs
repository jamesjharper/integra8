mod styles;
mod tree;
mod writer;

use std::error::Error;
use std::io::{self, Write, Stdout};

use structopt::StructOpt;

use crate::styles::{
    TreeStyle,
};
use crate::tree::{ResultsNode, ResultsTree};


use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::summary::{ComponentTypeCountSummary, RunSummary, SuiteSummary};
use integra8_formatters::models::{ComponentDescription, TestParameters};

use integra8_formatters::OutputLocation;
use integra8_formatters::{OutputFormatter, OutputFormatterFactory};

#[derive(StructOpt, Clone, Debug)] // TODO: Remove the need for clone here
pub struct TreeFormatterParameters {
    
}

pub struct TreeFormatter {
    out: Stdout,
    tree_style: TreeStyle
}

impl TreeFormatter {
    pub fn new(out: Stdout,  tree_style: TreeStyle) -> Self {
        TreeFormatter {
            out: out,
            tree_style: tree_style
        }
    }

    fn write_results<'a>(
        &mut self,
        inputs: impl Iterator<Item = &'a ComponentRunReport>,
        results_type: &str,
    ) -> Result<(), Box<dyn Error>> {
        /*let results_out_str = format!("\n{}:\n", results_type);

        let mut results = Vec::new();
        let mut stdouts = String::new();

        for report in inputs {
            results.push(report.description.full_name());

            if !report.artifacts.stdio.stdout.is_empty() {
                stdouts.push_str(&format!(
                    "---- {} stdout ----\n",
                    report.description.full_name()
                ));
                let output = String::from_utf8_lossy(&report.artifacts.stdio.stdout);
                stdouts.push_str(&output);
                stdouts.push('\n');
            }

            if !report.artifacts.stdio.stderr.is_empty() {
                stdouts.push_str(&format!(
                    "---- {} stderr ----\n",
                    report.description.full_name()
                ));
                let output = String::from_utf8_lossy(&report.artifacts.stdio.stderr);
                stdouts.push_str(&output);
                stdouts.push('\n');
            }
        }

        if !stdouts.is_empty() {
            self.out.write_plain(&results_out_str)?;
            self.out.write_plain("\n")?;
            self.out.write_plain(&stdouts)?;
        }

        self.out.write_plain(&results_out_str)?;
        results.sort();
        for name in &results {
            self.out.write_plain(&format!("    {}\n", name))?;
        }*/
        Ok(())
    }

    pub fn write_run_failures(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
        for suite in state.suites() {
            self.write_suite_failures(suite)?;
        }
        Ok(())
    }

    pub fn write_run_time_failures(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
        for suite in state.suites() {
            self.write_suite_time_failures(suite)?;
        }
        Ok(())
    }

    pub fn write_suite_time_failures(
        &mut self,
        summary: &SuiteSummary,
    ) -> Result<(), Box<dyn Error>> {
        let failures = summary.tests.failed().due_to_timing_out();
        if failures.has_some() {
            self.write_results(failures, "failures (time limit exceeded)")?;
        }
        Ok(())
    }

    pub fn write_suite_failures(&mut self, summary: &SuiteSummary) -> Result<(), Box<dyn Error>> {
        let failures = summary.tests.failed().due_to_rejection();
        if failures.has_some() {
            self.write_results(failures, "failures")?;
        }
        Ok(())
    }

    pub fn write_suite_successes(&mut self, summary: &SuiteSummary) -> Result<(), Box<dyn Error>> {
        let successes = summary.tests.passed();
        if successes.has_some() {
            self.write_results(successes, "successes")?;
        }
        Ok(())
    }

    fn get_tree<'a>(&self, state: &'a RunSummary) -> ResultsTree<'a> {
        let suite = state.get_root_suite().unwrap();
        ResultsTree::new(self.get_node(state, suite))
    }

    fn get_node<'a>(
        &self,
        state: &'a RunSummary,
        suite_summary: &'a SuiteSummary,
    ) -> ResultsNode<'a> {
        let mut suite_node = ResultsNode::from_report(suite_summary.suite_report.as_ref().unwrap());

        for setup_report in &suite_summary.setups.reports {
            suite_node.add_child_node(ResultsNode::from_report(&setup_report));
        }

        for test_report in &suite_summary.tests.reports {
            suite_node.add_child_node(ResultsNode::from_report(&test_report));
        }

        for tear_down_report in &suite_summary.tear_downs.reports {
            suite_node.add_child_node(ResultsNode::from_report(&tear_down_report));
        }

        for suite_report in &suite_summary.suites.reports {
            let suite_summary = state.get_suite(&suite_report.description.path).unwrap();
            suite_node.add_child_node(self.get_node(state, suite_summary));
        }

        suite_node
    }
}

use std::str::FromStr;

#[derive(Clone, Eq, PartialEq)]
pub enum Style {
    Text,
    Symbols
}


impl Style {
    pub fn default_value() -> Self {
        Self::Symbols
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "Text",
            Self::Symbols => "Symbols",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Text", "Symbols"]
    }
}


impl std::str::FromStr for Style {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Text" => Ok(Style::Text),
            "Symbols" => Ok(Style::Symbols),
            _ => Err(format!("{} was not a valid style. Valid values are either \"Text\" or \"Symbols\".", s))
        }
    }
}


#[derive(Clone, Eq, PartialEq)]
pub enum DetailLevel {
    Error,
    Verbose
}

impl DetailLevel {
    pub fn default_value() -> Self {
        Self::Error
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Verbose => "Verbose",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Error", "Verbose"]
    }
}


impl std::str::FromStr for DetailLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Error" => Ok(DetailLevel::Error),
            "Verbose" => Ok(DetailLevel::Verbose),
            _ => Err(format!("{} was not a valid detail level. Valid values are either \"Error\" or \"Verbose\".", s))
        }
    }
}


#[derive(Clone, Eq, PartialEq)]
pub enum Encoding {
    Ascii,
    Utf8
}

impl Encoding {
    pub fn default_value() -> Self {
        Self::Utf8
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ascii => "Ascii",
            Self::Utf8 => "Utf8",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Ascii", "Utf8"]
    }
}


impl std::str::FromStr for Encoding {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ascii" => Ok(Encoding::Ascii),
            "Utf8" => Ok(Encoding::Utf8),
            _ => Err(format!("{} was not a valid encoding type. Valid values are either \"Ascii\" or \"Utf8\".", s))
        }
    }
}

use atty::Stream;

#[derive(Clone, Eq, PartialEq)]
pub enum AnsiMode {
    Auto,
    Enabled,
    Disabled,
}

impl AnsiMode {

    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Auto => atty::is(Stream::Stdout),
            Self::Enabled => true,
            Self::Disabled => false,
        } 
    }

    pub fn default_value() -> Self {
        Self::Auto
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        } 
    }

    pub fn list_all() -> Vec<&'static str> {
        vec!["Auto", "Enabled", "Disabled"]
    }
}


impl std::str::FromStr for AnsiMode {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Auto" => Ok(AnsiMode::Auto),
            "Enabled" => Ok(AnsiMode::Enabled),
            "Disabled" => Ok(AnsiMode::Disabled),
            _ => Err(format!("{} was not a ANSI mode. Valid values are \"Auto\", \"Enabled\" or \"Disabled\".", s))
        }
    }
}


impl OutputFormatterFactory for TreeFormatter {
    type FormatterParameters = TreeFormatterParameters;
    fn create<T: TestParameters>(
        _formatter_parameters: &Self::FormatterParameters,
        parameters: &T,
    ) -> Box<dyn OutputFormatter> {

        let style = Style::from_str(parameters.console_output_style()).unwrap();
        let detail_level = DetailLevel::from_str(parameters.console_output_detail_level()).unwrap();
        let encoding = Encoding::from_str(parameters.console_output_encoding()).unwrap();
        let ansi_mode = AnsiMode::from_str(parameters.console_output_ansi_mode()).unwrap();

        let tree_style = TreeStyle::new(
            style,
            detail_level,
            encoding,
            ansi_mode,
        );

            
        Box::new(TreeFormatter::new(io::stdout(), tree_style))
    }

    fn default_style() -> &'static str {
        Style::default_value().as_str()
    }

    fn supported_styles() -> Vec<&'static str> {
        Style::list_all()
    }

    fn default_detail_levels() -> &'static str {
        DetailLevel::default_value().as_str()
    }

    fn supported_detail_levels() -> Vec<&'static str> {
        DetailLevel::list_all()
    }

    fn default_encoding() -> &'static str {
        Encoding::default_value().as_str()
    }

    fn supported_encodings() -> Vec<&'static str> {
        Encoding::list_all()
    }

    fn default_ansi_mode() -> &'static str {
        AnsiMode::default_value().as_str()
    }
    fn supported_ansi_modes() -> Vec<&'static str> {
        AnsiMode::list_all()
    }
}

impl OutputFormatter for TreeFormatter {
    fn write_run_start(
        &mut self,
        summary: &ComponentTypeCountSummary,
    ) -> Result<(), Box<dyn Error>> {
        let noun = if summary.tests() != 1 {
            "tests"
        } else {
            "test"
        };


        writeln!(self.out,"\nrunning {} {}\n", summary.tests(), noun)?;
        Ok(())
    }

    fn write_component_start(
        &mut self,
        _desc: &ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {

        write!(self.out, ".")?;

        Ok(())
    }

    fn write_run_complete(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
        self.get_tree(state)
            .render_tree(&mut self.out, &self.tree_style)?;

        Ok(())
    }
}
