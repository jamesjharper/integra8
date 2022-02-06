use std::collections::hash_map::Values;
use std::collections::HashMap;

use crate::summary::ResultsCountSummary;
use crate::report::ComponentRunReport;
use crate::summary::{FailedResults, NotRunResults, PassedResults, WarningResults, CompleteResults};
use crate::ComponentResult;

use integra8_components::{ComponentId, ComponentType};

/// A `ComponentResultSummary` is a collection of `ComponentRunReport` which can be queried based on
/// wether they *passed*, *failed* or where *not run*
#[derive(Clone, Debug)]
pub struct ComponentResultSummary {
    pub reports: Vec<ComponentRunReport>,
    pub counts: ResultsCountSummary 
}

impl ComponentResultSummary {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
            counts: ResultsCountSummary::new()
        }
    }

    pub fn push_report(&mut self, report: ComponentRunReport) {
        self.counts.increment( &report.result );
        self.reports.push(report);
    }

    pub fn passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from(self.reports.iter(), &self.counts.passed)
    }

    pub fn warning<'a>(&'a self) -> WarningResults<'a> {
        WarningResults::from(self.reports.iter(), &self.counts.warning)
    }

    pub fn failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from(self.reports.iter(), &self.counts.failed)
    }

    pub fn not_run<'a>(&'a self) -> NotRunResults<'a> {
        NotRunResults::from(self.reports.iter(), &self.counts.did_not_run)
    }
}

#[derive(Clone, Debug)]
pub struct SuiteSummary {
    pub suite_report: ComponentResultSummary,
    pub suites: ComponentResultSummary,
    pub setups: ComponentResultSummary,
    pub tests: ComponentResultSummary,
    pub tear_downs: ComponentResultSummary,
}

impl SuiteSummary {
    pub fn new() -> Self {
        Self {
            suite_report: ComponentResultSummary::new(),
            suites: ComponentResultSummary::new(),
            tests: ComponentResultSummary::new(),
            setups: ComponentResultSummary::new(),
            tear_downs: ComponentResultSummary::new(),
        }
    }

    pub fn suite_report<'a>(&'a self) -> Option<&'a ComponentRunReport> {
        self.suite_report.reports.first()
    }

    pub fn result(&self) -> ComponentResult {
        match &self.suite_report.reports.first() {
            Some(report) => report.result.clone(),
            None => ComponentResult::undetermined(),
        }
    }

    pub fn is_root(&self) -> bool {
        match &self.suite_report.reports.first() {
            Some(report) => report.description.is_root(),
            None => false,
        }
    }

    pub fn push_suite_report(&mut self, report: ComponentRunReport) {
        self.suite_report.push_report(report)
    }

    pub fn push_report(&mut self, report: ComponentRunReport) {
        match report.description.component_type() {
            ComponentType::Suite => self.suites.push_report(report),
            ComponentType::Test => self.tests.push_report(report),
            ComponentType::Setup => self.setups.push_report(report),
            ComponentType::TearDown => self.tear_downs.push_report(report),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RunSummary {
    suite_summaries: HashMap<ComponentId, SuiteSummary>,
}

impl RunSummary {
    pub fn new() -> Self {
        Self {
            suite_summaries: HashMap::new(),
        }
    }

    pub fn suites<'a>(&'a self) -> Values<'a, ComponentId, SuiteSummary> {
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


    pub fn all<'a>(&'a self) -> CompleteResults<'a> {
        CompleteResults::from_many(
            self.suite_summaries
                .values()
                .flat_map(|suite| {
                    vec![
                        (suite.suite_report.reports.iter(), &suite.suite_report.counts),
                        (suite.tests.reports.iter(), &suite.tests.counts),
                        (suite.suites.reports.iter(), &suite.suites.counts),
                        (suite.tear_downs.reports.iter(), &suite.tear_downs.counts)
                    ]
                }).collect()
        )
    }

    // Tests

    pub fn all_tests<'a>(&'a self) -> CompleteResults<'a> {
        CompleteResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite|  (suite.tests.reports.iter(), &suite.tests.counts))
                .collect(),
        )
    }

    pub fn test_passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tests.reports.iter(), &suite.tests.counts.passed))
                .collect(),
        )
    }

    pub fn test_warning<'a>(&'a self) -> WarningResults<'a> {
        WarningResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tests.reports.iter(), &suite.tests.counts.warning))
                .collect(),
        )
    }

    pub fn test_failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tests.reports.iter(), &suite.tests.counts.failed))
                .collect(),
        )
    }

    pub fn test_not_run<'a>(&'a self) -> NotRunResults<'a> {
        NotRunResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tests.reports.iter(), &suite.tests.counts.did_not_run))
                .collect(),
        )
    }

    // Setup

    pub fn all_setup<'a>(&'a self) -> CompleteResults<'a> {
        CompleteResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite|  (suite.setups.reports.iter(), &suite.setups.counts))
                .collect(),
        )
    }

    pub fn setup_passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.setups.reports.iter(), &suite.setups.counts.passed))
                .collect(),
        )
    }

    pub fn setup_warning<'a>(&'a self) -> WarningResults<'a> {
        WarningResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.setups.reports.iter(), &suite.setups.counts.warning))
                .collect(),
        )
    }

    pub fn setup_failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.setups.reports.iter(), &suite.setups.counts.failed))
                .collect(),
        )
    }

    pub fn setup_not_run<'a>(&'a self) -> NotRunResults<'a> {
        NotRunResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.setups.reports.iter(), &suite.setups.counts.did_not_run))
                .collect(),
        )
    }

    // Tear Down

    pub fn all_tear_down<'a>(&'a self) -> CompleteResults<'a> {
        CompleteResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite|  (suite.tear_downs.reports.iter(), &suite.tear_downs.counts))
                .collect(),
        )
    }

    pub fn tear_down_passed<'a>(&'a self) -> PassedResults<'a> {
        PassedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tear_downs.reports.iter(), &suite.tear_downs.counts.passed))
                .collect(),
        )
    }

    pub fn tear_down_warning<'a>(&'a self) -> WarningResults<'a> {
        WarningResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tear_downs.reports.iter(), &suite.tear_downs.counts.warning))
                .collect(),
        )
    }

    pub fn tear_down_failed<'a>(&'a self) -> FailedResults<'a> {
        FailedResults::from_many(
            self.suite_summaries
                .values()
                .map(|suite| (suite.tear_downs.reports.iter(), &suite.tear_downs.counts.failed))
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
                        &suite.tear_downs.counts.did_not_run,
                    )
                })
                .collect(),
        )
    }

    pub fn push_report(&mut self, report: ComponentRunReport) {
        if let ComponentType::Suite = report.description.component_type() {
            self.get_suite_mut(&report.description.id())
                .push_suite_report(report.clone());
        }

        if !report.description.is_root() {
            self.get_suite_mut(&report.description.parent_id())
                .push_report(report);
        }
    }

    pub fn get_root_suite<'a>(&'a self) -> Option<&'a SuiteSummary> {
        self.suite_summaries.values().find(|x| x.is_root())
    }

    pub fn get_suite<'a>(&'a self, id: &ComponentId) -> Option<&'a SuiteSummary> {
        self.suite_summaries.get(id)
    }

    fn get_suite_mut<'a>(&'a mut self, id: &ComponentId) -> &'a mut SuiteSummary {
        self.suite_summaries
            .entry(id.clone())
            .or_insert(SuiteSummary::new())
    }
}
