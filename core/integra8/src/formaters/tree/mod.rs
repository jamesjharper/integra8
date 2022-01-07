mod print_tree;
use crate::formaters::tree::print_tree::ComponentResultsTreeNode;
use crate::formaters::tree::print_tree::TreeNodePrinter;

use crate::components::ComponentDescription;
use crate::formaters::OutputLocation;
use crate::formaters::{OutputFormatter, OutputFormatterFactory};
use crate::parameters::TestParameters;
use crate::results::summary::{RunSummary, SuiteSummary};
use crate::results::report::ComponentRunReport;

use std::error::Error;

use crate::structopt::StructOpt;

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
            results.push(report.description.identity.name.to_string());

            if !report.artifacts.stdio.stdout.is_empty() {
                stdouts.push_str(&format!(
                    "---- {} stdout ----\n",
                    report.description.identity.name
                ));
                let output = String::from_utf8_lossy(&report.artifacts.stdio.stdout);
                stdouts.push_str(&output);
                stdouts.push('\n');
            }

            if !report.artifacts.stdio.stderr.is_empty() {
                stdouts.push_str(&format!(
                    "---- {} stderr ----\n",
                    report.description.identity.name
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

    fn get_tree(&self, state: &RunSummary) -> ComponentResultsTreeNode {
        let suite = state.get_root_suite().unwrap();
        self.get_node(state, suite)
    }

    fn get_node(
        &self,
        state: &RunSummary,
        suite_summary: &SuiteSummary,
    ) -> ComponentResultsTreeNode {
        let mut suite_node =
            ComponentResultsTreeNode::from_report(suite_summary.suite_report.as_ref().unwrap());

        for setup_report in &suite_summary.setups.reports {
            suite_node.add_child_report(&setup_report);
        }

        for test_report in &suite_summary.tests.reports {
            suite_node.add_child_report(&test_report);
        }

        for tear_down_report in &suite_summary.tear_downs.reports {
            suite_node.add_child_report(&tear_down_report);
        }

        for suite_report in &suite_summary.suites.reports {
            let suite_summary = state.get_suite(&suite_report.description.identity).unwrap();
            suite_node.add_child_node(self.get_node(state, suite_summary));
        }

        suite_node
    }
}

impl OutputFormatterFactory for TreeFormatter {
    type FormatterParameters = TreeFormatterParameters;
    fn create<T: TestParameters>(
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
    fn write_run_start(&mut self, test_count: usize) -> Result<(), Box<dyn Error>> {
        let noun = if test_count != 1 { "tests" } else { "test" };
        self.out
            .write_plain(&format!("\nrunning {} {}\n", test_count, noun))?;
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
        self.write_run_failures(state)?;
        self.write_run_time_failures(state)?;

        let tree = self.get_tree(state);

        TreeNodePrinter::new(&mut self.out).print_tree(&tree);

        self.out.write_plain("\n\n")?;
        Ok(())
    }

    /*fn write_run_complete(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {


        /*if state.options.display_output {
            self.write_successes(state)?;
        }*/


        let success = state.is_success();

        self.write_run_failures(state)?;
        self.write_run_time_failures(state)?;

        self.write_plain("\ntest result: ")?;

        if success {
            // There's no parallelism at this point so it's safe to use color
            self.write_pretty("ok", term::color::GREEN)?;
        } else {
            self.write_pretty("FAILED", term::color::RED)?;
        }

         if state.tests_passed().due_to_allowed_failure().has_some() {
            format!(
                ". {} passed; {} failed ({} allowed); {} skipped",
                state.tests_passed().count(),
                state.tests_failed().count() + state.tests_passed().due_to_allowed_failure().count(),
                state.tests_passed().due_to_allowed_failure().count(),
                state.tests_not_run().count(),
            )
        } else {
            format!(
                ". {} passed; {} failed; {} skipped",
                state.tests_passed().count(), state.tests_failed().count(), state.tests_not_run().count()
            )
        };

        //self.write_plain(&s)?;

        /*if let Some(result_timings) = &state.total_time {
            let time_str = format!("; finished in {:.2} seconds",  result_timings.duration().as_secs_f64());
            self.write_plain(&time_str)?;
        }*/


        self.write_plain("\n\n")?;
        Ok(())
    }*/
}
