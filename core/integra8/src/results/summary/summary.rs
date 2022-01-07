use crate::results::summary::{
    DidNotRunResultsCountSummary, FailResultsCountSummary, PassResultsCountSummary
};

use crate::results::summary::{FailedResults, PassedResults, NotRunResults};

use crate::components::{ComponentIdentity, ComponentType};


use crate::results::ComponentResult;
use crate::results::report::ComponentRunReport;

use std::collections::hash_map::Values;
use std::collections::HashMap;

/// A `ComponentResultSummary` is a collection of `ComponentRunReport` which can be queried based on 
/// wether they *passed*, *failed* or where *not run*
#[derive(Clone, Debug)]
pub struct ComponentResultSummary {
    pub reports: Vec<ComponentRunReport>,
    passed: PassResultsCountSummary,
    failed: FailResultsCountSummary,
    did_not_run: DidNotRunResultsCountSummary,
}

impl ComponentResultSummary {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
            passed: PassResultsCountSummary::new(),
            failed: FailResultsCountSummary::new(),
            did_not_run: DidNotRunResultsCountSummary::new(),
        }
    }

    pub fn push_report(&mut self, report: ComponentRunReport) {
        match &report.result {
            ComponentResult::Pass(result) => self.passed.increment(result),
            ComponentResult::Fail(result) => self.failed.increment(result),
            ComponentResult::DidNotRun(result) => self.did_not_run.increment(result),
        }
        self.reports.push(report);
    }

    pub fn passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from(self.reports.iter(), &self.passed)
    }

    pub fn failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from(self.reports.iter(), &self.failed)
    }

    pub fn not_run<'a>(&'a self) -> NotRunResults<'a> {
        NotRunResults::from(self.reports.iter(), &self.did_not_run)
    }
}

#[derive(Clone, Debug)]
pub struct SuiteSummary {
    pub suite_report: Option<ComponentRunReport>, //TODO: make this non optional

    pub suites: ComponentResultSummary,
    pub setups: ComponentResultSummary,
    pub tests: ComponentResultSummary,
    pub tear_downs: ComponentResultSummary,
}

impl SuiteSummary {
    pub fn new() -> Self {
        Self {
            suite_report: None,
            suites: ComponentResultSummary::new(),
            tests: ComponentResultSummary::new(),
            setups: ComponentResultSummary::new(),
            tear_downs: ComponentResultSummary::new(),
        }
    }

    pub fn result(&self) -> ComponentResult {
        match &self.suite_report {
            Some(report) => report.result.clone(),
            None => ComponentResult::undetermined(),
        }
    }

    pub fn is_root(&self) -> bool {
        match &self.suite_report {
            Some(report) => report.description.is_root(),
            None => false,
        }
    }

    pub fn push_suite_report(&mut self, report: ComponentRunReport) {
        self.suite_report = Some(report)
    }

    pub fn push_report(&mut self, report: ComponentRunReport) {
        match report.description.component_type {
            ComponentType::Suite => self.suites.push_report(report),
            ComponentType::Test => self.tests.push_report(report),
            ComponentType::Setup => self.setups.push_report(report),
            ComponentType::TearDown => self.tear_downs.push_report(report),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RunSummary {
    suite_summaries: HashMap<&'static str, SuiteSummary>,
}

impl RunSummary {
    pub fn new() -> Self {
        Self {
            suite_summaries: HashMap::new(),
        }
    }

    pub fn suites<'a>(&'a self) -> Values<'a, &'static str, SuiteSummary> {
        self.suite_summaries.values()
    }

    pub fn is_success(&self) -> bool {
        self.run_result().has_passed()
    }

    pub fn run_result(&self) -> ComponentResult {
        self.get_root_suite()
            .map(|root| root.result())
            .unwrap_or(ComponentResult::undetermined())
    }

    // Tests

    pub fn tests_passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tests.reports.iter(), &suite.tests.passed))
                .collect(),
        )
    }

    pub fn tests_failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tests.reports.iter(), &suite.tests.failed))
                .collect(),
        )
    }

    pub fn tests_not_run<'a>(&'a self) -> NotRunResults<'a> {
        NotRunResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tests.reports.iter(), &suite.tests.did_not_run))
                .collect(),
        )
    }

    // Setup

    pub fn setup_passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.setups.reports.iter(), &suite.setups.passed))
                .collect(),
        )
    }

    pub fn setup_failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.setups.reports.iter(), &suite.setups.failed))
                .collect(),
        )
    }

    pub fn setup_not_run<'a>(&'a self) -> NotRunResults<'a> {
        NotRunResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.setups.reports.iter(), &suite.setups.did_not_run))
                .collect(),
        )
    }

    // Tear Down

    pub fn tear_down_passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tear_downs.reports.iter(), &suite.tear_downs.passed))
                .collect(),
        )
    }

    pub fn tear_down_failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tear_downs.reports.iter(), &suite.tear_downs.failed))
                .collect(),
        )
    }

    pub fn tear_down_not_run<'a>(&'a self) -> NotRunResults<'a> {
        NotRunResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| {
                    (
                        suite.tear_downs.reports.iter(),
                        &suite.tear_downs.did_not_run,
                    )
                })
                .collect(),
        )
    }

    pub fn push_report(&mut self, report: ComponentRunReport) {
        if let ComponentType::Suite = report.description.component_type {
            self.get_suite_mut(&report.description.identity)
                .push_suite_report(report.clone());
        }

        if !report.description.is_root() {
            self.get_suite_mut(&report.description.parent_identity)
                .push_report(report);
        }
    }

    pub fn get_root_suite<'a>(&'a self) -> Option<&'a SuiteSummary> {
        self.suite_summaries.values().find(|x| x.is_root())
    }

    pub fn get_suite<'a>(&'a self, identity: &ComponentIdentity) -> Option<&'a SuiteSummary> {
        self.suite_summaries.get(identity.path)
    }

    fn get_suite_mut<'a>(&'a mut self, identity: &ComponentIdentity) -> &'a mut SuiteSummary {
        self.suite_summaries
            .entry(identity.path)
            .or_insert(SuiteSummary::new())
    }
}
