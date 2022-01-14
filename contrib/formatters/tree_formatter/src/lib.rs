mod styles;
mod tree;
mod writer;

use std::error::Error;
use structopt::StructOpt;

use crate::styles::{
    CharacterTheme, FormattingTheme, OutputLevel, OutputTheme, StyleSettings, TreeStyle,
};
use crate::tree::{ResultsNode, ResultsTree};

use integra8_formatters::models::report::ComponentRunReport;
use integra8_formatters::models::summary::{ComponentTypeCountSummary, RunSummary, SuiteSummary};
use integra8_formatters::models::ComponentDescription;

use integra8_formatters::OutputLocation;
use integra8_formatters::{OutputFormatter, OutputFormatterFactory};

#[derive(StructOpt, Clone, Debug)] // TODO: Remove the need for clone here
pub struct TreeFormatterParameters {}

pub struct TreeFormatter {
    out: OutputLocation,
    _is_multithreaded: bool,
}

impl TreeFormatter {
    pub fn new(out: OutputLocation, is_multithreaded: bool) -> Self {
        TreeFormatter {
            out: out,
            _is_multithreaded: is_multithreaded,
        }
    }

    fn write_results<'a>(
        &mut self,
        inputs: impl Iterator<Item = &'a ComponentRunReport>,
        results_type: &str,
    ) -> Result<(), Box<dyn Error>> {
        let results_out_str = format!("\n{}:\n", results_type);

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
        }
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

impl OutputFormatterFactory for TreeFormatter {
    type FormatterParameters = TreeFormatterParameters;
    fn create<T>(
        _formatter_parameters: &Self::FormatterParameters,
        _test_parameters: &T,
    ) -> Box<dyn OutputFormatter> {
        let use_color = true;

        let formatter = TreeFormatter::new(
            match (term::stdout(), use_color) {
                (Some(t), true) => OutputLocation::Pretty(t),
                _ => OutputLocation::Raw(Box::new(std::io::stdout())),
            },
            /*is_multithreaded*/ true,
        );

        Box::new(formatter)
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
        self.out
            .write_plain(&format!("\nrunning {} {}\n", summary.tests(), noun))?;
        Ok(())
    }

    fn write_component_start(
        &mut self,
        _desc: &ComponentDescription,
    ) -> Result<(), Box<dyn Error>> {
        self.out.write_plain(".")?;
        Ok(())
    }

    fn write_run_complete(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
        let style = StyleSettings {
            formatting: FormattingTheme::Standard,
            output: OutputTheme::Symbols,
            characters: CharacterTheme::Utf8,
            level: OutputLevel::Verbose,
        };

        self.get_tree(state)
            .render_tree(&mut self.out, &TreeStyle::new(&style))?;

        Ok(())
    }
}
