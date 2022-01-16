use crate::{ComponentResult, DidNotRunReason, FailureReason, PassReason, WarningReason};

use integra8_components::ComponentType;

/// A struct for interrogating *pass* results.
/// Implements `Iterator` and can be reduced to a filtered results set using its accompanying  `due_to...` methods
pub trait ResultReasonCounter {
    type ReasonType;
    fn total(&self) -> usize;
    fn by_reason(&self, reason: &Self::ReasonType) -> usize;
}

#[derive(Clone, Debug)]
pub struct ResultsCountSummary {
    pub passed: PassResultsCountSummary,
    pub warning: WarningResultsCountSummary,
    pub failed: FailResultsCountSummary,
    pub did_not_run: DidNotRunResultsCountSummary,
}

impl ResultsCountSummary {
    pub fn new() -> Self {
        Self {
            passed: PassResultsCountSummary::new(),
            warning: WarningResultsCountSummary::new(),
            failed: FailResultsCountSummary::new(),
            did_not_run: DidNotRunResultsCountSummary::new(),
        }
    }

    pub fn increment(&mut self, result: &ComponentResult) {
        match result {
            ComponentResult::Pass(result) => self.passed.increment(result),
            ComponentResult::Warning(result) => self.warning.increment(result),
            ComponentResult::Fail(result) => self.failed.increment(result),
            ComponentResult::DidNotRun(result) => self.did_not_run.increment(result),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PassResultsCountSummary {
    accepted: usize,
}

impl PassResultsCountSummary {
    pub fn new() -> Self {
        Self {
            accepted: 0,
        }
    }

    pub fn increment(&mut self, reason: &PassReason) {
        match reason {
            PassReason::Accepted => self.accepted += 1,
        }
    }

    pub fn accepted(&self) -> usize {
        self.accepted
    }
}

impl ResultReasonCounter for PassResultsCountSummary {
    type ReasonType = PassReason;

    fn total(&self) -> usize {
        self.accepted
    }

    fn by_reason(&self, reason: &PassReason) -> usize {
        match reason {
            PassReason::Accepted => self.accepted,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WarningResultsCountSummary {
    failure_allowed: usize,
    overtime_warning: usize,
    child_warning: usize,
}

impl WarningResultsCountSummary {
    pub fn new() -> Self {
        Self {
            failure_allowed: 0,
            overtime_warning: 0,
            child_warning: 0,
        }
    }

    pub fn increment(&mut self, reason: &WarningReason) {
        match reason {
            WarningReason::FailureAllowed => self.failure_allowed += 1,
            WarningReason::OvertimeWarning => self.overtime_warning += 1,
            WarningReason::ChildWarning => self.child_warning += 1,
        }
    }

    pub fn failure_allowed(&self) -> usize {
        self.failure_allowed
    }

    pub fn overtime_warning(&self) -> usize {
        self.overtime_warning
    }

    pub fn child_warning(&self) -> usize {
        self.child_warning
    }
}

impl ResultReasonCounter for WarningResultsCountSummary {
    type ReasonType = WarningReason;

    fn total(&self) -> usize {
        self.failure_allowed
            .saturating_add(self.overtime_warning)
            .saturating_add(self.child_warning)
    }

    fn by_reason(&self, reason: &WarningReason) -> usize {
        match reason {
            WarningReason::FailureAllowed => self.failure_allowed,
            WarningReason::OvertimeWarning => self.overtime_warning,
            WarningReason::ChildWarning => self.child_warning,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FailResultsCountSummary {
    rejected: usize,
    timed_out: usize,
    child_failure: usize,
}

impl FailResultsCountSummary {
    pub fn new() -> Self {
        Self {
            rejected: 0,
            timed_out: 0,
            child_failure: 0,
        }
    }

    pub fn increment(&mut self, reason: &FailureReason) {
        match reason {
            FailureReason::Rejected => self.rejected += 1,
            FailureReason::Overtime => self.timed_out += 1,
            FailureReason::ChildFailure => self.child_failure += 1,
        }
    }

    pub fn rejected(&self) -> usize {
        self.rejected
    }

    pub fn timed_out(&self) -> usize {
        self.timed_out
    }

    pub fn child_failure(&self) -> usize {
        self.child_failure
    }
}

impl ResultReasonCounter for FailResultsCountSummary {
    type ReasonType = FailureReason;

    fn total(&self) -> usize {
        self.child_failure
            .saturating_add(self.timed_out)
            .saturating_add(self.rejected)
    }

    fn by_reason(&self, reason: &FailureReason) -> usize {
        match reason {
            FailureReason::Rejected => self.rejected,
            FailureReason::Overtime => self.timed_out,
            FailureReason::ChildFailure => self.child_failure,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DidNotRunResultsCountSummary {
    ignored: usize,
    undetermined: usize,
    filtered: usize,
    parent_failure: usize,
}

impl DidNotRunResultsCountSummary {
    pub fn new() -> Self {
        Self {
            ignored: 0,
            undetermined: 0,
            filtered: 0,
            parent_failure: 0,
        }
    }

    pub fn increment(&mut self, reason: &DidNotRunReason) {
        match reason {
            DidNotRunReason::Undetermined => self.undetermined += 1,
            DidNotRunReason::Filtered => self.filtered += 1,
            DidNotRunReason::Ignored => self.ignored += 1,
            DidNotRunReason::ParentFailure => self.parent_failure += 1,
        }
    }

    pub fn ignored(&self) -> usize {
        self.ignored
    }

    pub fn undetermined(&self) -> usize {
        self.undetermined
    }
    pub fn filtered(&self) -> usize {
        self.filtered
    }

    pub fn parent_failure(&self) -> usize {
        self.parent_failure
    }
}

impl ResultReasonCounter for DidNotRunResultsCountSummary {
    type ReasonType = DidNotRunReason;

    fn total(&self) -> usize {
        self.ignored
            .saturating_add(self.undetermined)
            .saturating_add(self.filtered)
            .saturating_add(self.parent_failure)
    }

    fn by_reason(&self, reason: &DidNotRunReason) -> usize {
        match reason {
            DidNotRunReason::Undetermined => self.undetermined,
            DidNotRunReason::Filtered => self.filtered,
            DidNotRunReason::Ignored => self.ignored,
            DidNotRunReason::ParentFailure => self.parent_failure,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComponentTypeCountSummary {
    tests: usize,
    suites: usize,
    setups: usize,
    tear_downs: usize,
}

impl ComponentTypeCountSummary {
    pub fn new() -> Self {
        Self {
            tests: 0,
            suites: 0,
            setups: 0,
            tear_downs: 0,
        }
    }

    pub fn increment(&mut self, reason: &ComponentType) {
        match reason {
            ComponentType::Suite => self.suites += 1,
            ComponentType::Test => self.tests += 1,
            ComponentType::Setup => self.setups += 1,
            ComponentType::TearDown => self.tear_downs += 1,
        }
    }

    pub fn tests(&self) -> usize {
        self.tests
    }

    pub fn suites(&self) -> usize {
        self.suites
    }

    pub fn setups(&self) -> usize {
        self.setups
    }

    pub fn tear_downs(&self) -> usize {
        self.tear_downs
    }
}
