pub mod parameters;
mod progress;
pub mod render;
mod styles;
mod tree;
mod writer;

use std::error::Error;
use std::io::{self, Stdout, Write};
use std::str::FromStr;

use crate::parameters::{AnsiMode, DetailLevel, Encoding, Style, TreeFormatterParameters};
use crate::progress::TestProgressFormatter;
use crate::styles::{ProgressBarStyle, TreeStyle};
use crate::tree::{ResultsNode, ResultsTree};

use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::summary::{ComponentTypeCountSummary, RunSummary, SuiteSummary};
use integra8_formatters::models::{ComponentDescription, ComponentResult, TestParameters};
use integra8_formatters::{OutputFormatter, OutputFormatterFactory};

pub struct TreeFormatter {
    writer: Stdout,
    tree_style: TreeStyle,
    detail_level: DetailLevel,
    progress_formatter: TestProgressFormatter,
}

impl TreeFormatter {
    pub fn new(
        writer: Stdout,
        tree_style: TreeStyle,
        progress_style: ProgressBarStyle,
        detail_level: DetailLevel,
    ) -> Self {
        Self {
            progress_formatter: TestProgressFormatter::new(progress_style),
            writer,
            tree_style,
            detail_level,
        }
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
        let mut suite_node = ResultsNode::from_report(suite_summary.suite_report().unwrap());

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
            let suite_summary = state.get_suite(&suite_report.description.id()).unwrap();
            suite_node.add_child_node(self.get_node(state, suite_summary));
        }

        suite_node
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

        let ansi_mode = if AnsiMode::from_str(parameters.console_output_ansi_mode())
            .unwrap()
            .is_enabled()
        {
            try_init_ansi()
        } else {
            AnsiMode::Disabled
        };

        Box::new(TreeFormatter::new(
            io::stdout(),
            TreeStyle::new(&style, &encoding, &ansi_mode),
            ProgressBarStyle::new(&ansi_mode),
            detail_level,
        ))
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
        self.progress_formatter
            .notify_run_start(&mut self.writer, summary)?;
        Ok(())
    }

    fn write_component_start(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        self.progress_formatter
            .notify_component_start(&mut self.writer, desc)?;
        Ok(())
    }

    fn write_component_report(
        &mut self,
        report: &ComponentRunReport,
    ) -> Result<(), Box<dyn Error>> {
        self.progress_formatter
            .notify_component_finished(&mut self.writer, report)?;
        Ok(())
    }

    fn write_run_complete(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
        self.progress_formatter
            .notify_run_finished(&mut self.writer, state)?;
        writeln!(self.writer, "\ntest result: ")?;

        match state.run_result() {
            ComponentResult::Pass(_) => write!(self.writer, "ok")?,
            ComponentResult::Warning(_) => write!(self.writer, "completed with warnings")?,
            ComponentResult::Fail(_) => write!(self.writer, "FAILED")?,
            ComponentResult::DidNotRun(_) => write!(self.writer, "undetermined")?,
        }

        if state.test_warning().has_some() {
            writeln!(
                self.writer,
                ". {} passed; {} failed ({} allowed); {} skipped",
                state.test_passed().total_count(),
                state.test_failed().total_count() + state.test_warning().total_count(),
                state.test_warning().total_count(),
                state.test_not_run().total_count(),
            )?;
        } else {
            writeln!(
                self.writer,
                ". {} passed; {} failed; {} skipped",
                state.test_passed().total_count(),
                state.test_failed().total_count(),
                state.test_not_run().total_count()
            )?;
        };
        writeln!(self.writer, "")?;

        // Just the detail level to capture most relevant details in relation to the result
        let detail_level = match (&self.detail_level, state.run_result()) {
            // If there are no errors or warnings, downgrade Error level
            (DetailLevel::Error, ComponentResult::Warning(_)) => DetailLevel::Warning,
            (DetailLevel::Error, ComponentResult::Pass(_)) => DetailLevel::Info,
            (DetailLevel::Warning, ComponentResult::Pass(_)) => DetailLevel::Info,
            _ => self.detail_level.clone(),
        };

        self.get_tree(state)
            .render_tree(&mut self.writer, &self.tree_style, &detail_level)?;

        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn try_init_ansi() -> AnsiMode {
    // Enables ANSI code support on Windows 10.
    if ansi_term::enable_ansi_support().is_err() {
        AnsiMode::Disabled // Disable ANSI if this fails
    } else {
        AnsiMode::Enabled
    }
}

#[cfg(not(target_os = "windows"))]
fn try_init_ansi() -> AnsiMode {
    AnsiMode::Enabled
}
