use crate::results::{ComponentResult, DidNotRunReason, FailureReason, PassReason};


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
    pub failed: FailResultsCountSummary,
    pub did_not_run: DidNotRunResultsCountSummary,
}

impl ResultsCountSummary {
    pub fn new() -> Self {
        Self {
            passed: PassResultsCountSummary::new(),
            failed: FailResultsCountSummary::new(),
            did_not_run: DidNotRunResultsCountSummary::new(),
        }
    }

    pub fn increment(&mut self, result: &ComponentResult) {
        match result {
            ComponentResult::Pass(result) => self.passed.increment(result),
            ComponentResult::Fail(result) => self.failed.increment(result),
            ComponentResult::DidNotRun(result) => self.did_not_run.increment(result),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PassResultsCountSummary {
    accepted: usize,
    allowed_failure: usize,
}

impl PassResultsCountSummary {
    pub fn new() -> Self {
        Self {
            accepted: 0,
            allowed_failure: 0,
        }
    }

    pub fn increment(&mut self, reason: &PassReason) {
        match reason {
            PassReason::Accepted => self.accepted += 1,
            PassReason::FailureAllowed => self.allowed_failure += 1,
        }
    }

    pub fn accepted(&self) -> usize {
        self.accepted
    }

    pub fn allowed_failure(&self) -> usize {
        self.allowed_failure
    }
}

impl ResultReasonCounter for PassResultsCountSummary {
    type ReasonType = PassReason;

    fn total(&self) -> usize {
        self.accepted().saturating_add(self.allowed_failure())
    }

    fn by_reason(&self, reason: &PassReason) -> usize {
        match reason {
            PassReason::Accepted => self.accepted,
            PassReason::FailureAllowed => self.allowed_failure,
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
