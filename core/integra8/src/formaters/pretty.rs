use std::io::prelude::Write;

use crate::components::ComponentDescription;
use crate::formatters::OutputLocation;
use crate::formatters::{OutputFormatter, OutputFormatterFactory};
use crate::parameters::TestParameters;
use crate::results::summary::{RunSummary, SuiteSummary};

use crate::results::report::ComponentRunReport;

use crate::results::{
    ComponentResult, ComponentTimeResult, DidNotRunReason, FailureReason,
    PassReason,
};

use std::error::Error;

use crate::structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)] // TODO: Remove the need for clone here
pub struct PrettyFormatterParameters {}

pub struct PrettyFormatter {
    out: OutputLocation,
    use_color: bool,
    is_multithreaded: bool,
}

impl PrettyFormatter {
    pub fn new(out: OutputLocation, use_color: bool, is_multithreaded: bool) -> Self {
        PrettyFormatter {
            out,
            use_color,
            is_multithreaded,
        }
    }

    pub fn write_ok(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_short_result("ok", term::color::GREEN)
    }

    pub fn write_failed(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_short_result("FAILED", term::color::RED)
    }

    pub fn write_ignored(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_short_result("ignored", term::color::YELLOW)
    }

    pub fn write_skipped(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_short_result("skipped", term::color::YELLOW)
    }

    pub fn write_allowed_fail(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_short_result("FAILED (allowed)", term::color::YELLOW)
    }

    pub fn write_time_failed(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_short_result("FAILED (time limit exceeded)", term::color::RED)
    }

    pub fn write_short_result(
        &mut self,
        result: &str,
        color: term::color::Color,
    ) -> Result<(), Box<dyn Error>> {
        self.write_pretty(result, color)
    }

    pub fn write_pretty(
        &mut self,
        word: &str,
        color: term::color::Color,
    ) -> Result<(), Box<dyn Error>> {
        match self.out {
            OutputLocation::Pretty(ref mut term) => {
                if self.use_color {
                    term.fg(color)?;
                }

                term.write_all(word.as_bytes())?;

                if self.use_color {
                    term.reset()?;
                }
                term.flush()?;
            }
            OutputLocation::Raw(ref mut stdout) => {
                stdout.write_all(word.as_bytes())?;

                stdout.flush()?;
            }
        }
        Ok(())
    }

    pub fn write_plain<S: AsRef<str>>(&mut self, s: S) -> Result<(), Box<dyn Error>> {
        let s = s.as_ref();
        self.out.write_all(s.as_bytes())?;

        self.out.flush()?;
        Ok(())
    }

    fn write_time(&mut self, result_timings: &ComponentTimeResult) -> Result<(), Box<dyn Error>> {
        let color = if result_timings.is_critical() {
            Some(term::color::RED)
        } else if result_timings.is_warn() {
            Some(term::color::YELLOW)
        } else {
            None
        };
        let timing = format!(" {:.3}s", result_timings.duration().as_secs_f64());

        match color {
            Some(color) => self.write_pretty(&timing, color)?,
            None => self.write_plain(&timing)?,
        }

        Ok(())
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
            self.write_plain(&results_out_str)?;
            self.write_plain("\n")?;
            self.write_plain(&stdouts)?;
        }

        self.write_plain(&results_out_str)?;
        results.sort();
        for name in &results {
            self.write_plain(&format!("    {}\n", name))?;
        }
        Ok(())
    }

    pub fn write_run_successes(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
        for suite in state.suites() {
            self.write_suite_successes(suite)?;
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

    fn write_test_name(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        self.write_plain(&format!("test {} ... ", desc.identity.name))?;
        /*if let Some(test_mode) = desc.test_mode() {
            self.write_plain(&format!("test {} - {} ... ", name, test_mode))?;
        } else {
            self.write_plain(&format!("test {} ... ", name))?;
        }*/

        Ok(())
    }
}

impl OutputFormatterFactory for PrettyFormatter {
    type FormatterParameters = PrettyFormatterParameters;
    fn create<T: TestParameters>(
        _formatter_parameters: &Self::FormatterParameters,
        _test_parameters: &T,
    ) -> Box<dyn OutputFormatter> {
        let formatter = PrettyFormatter::new(
            match term::stdout() {
                None => OutputLocation::Raw(Box::new(std::io::stdout())),
                Some(t) => OutputLocation::Pretty(t),
            },
            /*use_color*/ true,
            /*is_multithreaded*/ true,
        );

        Box::new(formatter)
    }
}

impl OutputFormatter for PrettyFormatter {
    fn write_run_start(&mut self, test_count: usize) -> Result<(), Box<dyn Error>> {
        let noun = if test_count != 1 { "tests" } else { "test" };
        self.write_plain(&format!("\nrunning {} {}\n", test_count, noun))
    }

    fn write_test_start(&mut self, desc: &ComponentDescription) -> Result<(), Box<dyn Error>> {
        // When running tests concurrently, we should not print
        // the test's name as the result will be mis-aligned.
        // When running the tests serially, we print the name here so
        // that the user can see which test hangs.
        if !self.is_multithreaded {
            self.write_test_name(desc)?;
        }

        Ok(())
    }

    fn write_test_report(&mut self, report: &ComponentRunReport) -> Result<(), Box<dyn Error>> {
        if self.is_multithreaded {
            self.write_test_name(&report.description)?;
        }

        match report.result {
            ComponentResult::Pass(PassReason::Accepted) => self.write_ok()?,
            ComponentResult::Pass(PassReason::FailureAllowed) => self.write_allowed_fail()?,
            ComponentResult::Fail(FailureReason::Rejected) => self.write_failed()?,
            ComponentResult::Fail(FailureReason::Overtime) => self.write_failed()?,
            ComponentResult::Fail(FailureReason::ChildFailure) => self.write_failed()?,
            ComponentResult::DidNotRun(DidNotRunReason::Undetermined) => self.write_skipped()?,
            ComponentResult::DidNotRun(DidNotRunReason::Filtered) => self.write_skipped()?,
            ComponentResult::DidNotRun(DidNotRunReason::Ignored) => self.write_ignored()?,
            ComponentResult::DidNotRun(DidNotRunReason::ParentFailure) => self.write_ignored()?,
        }

        if report.timing.is_warn() || report.timing.is_critical() {
            self.write_time(&report.timing)?;
        }

        self.write_plain("\n")
    }

    fn write_test_timeout(
        &mut self,
        desc: &ComponentDescription,
        result_timings: &ComponentTimeResult,
    ) -> Result<(), Box<dyn Error>> {
        self.write_plain(&format!(
            "test {} has been running for over {:.2} seconds\n",
            desc.identity.name,
            result_timings.duration().as_secs_f64()
        ))
    }

    fn write_run_complete(&mut self, state: &RunSummary) -> Result<(), Box<dyn Error>> {
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
                state.tests_failed().count()
                    + state.tests_passed().due_to_allowed_failure().count(),
                state.tests_passed().due_to_allowed_failure().count(),
                state.tests_not_run().count(),
            )
        } else {
            format!(
                ". {} passed; {} failed; {} skipped",
                state.tests_passed().count(),
                state.tests_failed().count(),
                state.tests_not_run().count()
            )
        };

        //self.write_plain(&s)?;

        /*if let Some(result_timings) = &state.total_time {
            let time_str = format!("; finished in {:.2} seconds",  result_timings.duration().as_secs_f64());
            self.write_plain(&time_str)?;
        }*/

        self.write_plain("\n\n")?;
        Ok(())
    }
}
