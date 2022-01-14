pub mod artifacts;
pub mod report;
pub mod summary;

use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentResult {
    Pass(PassReason),
    Fail(FailureReason),
    DidNotRun(DidNotRunReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PassReason {
    Accepted,
    FailureAllowed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FailureReason {
    Rejected,
    Overtime,
    ChildFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DidNotRunReason {
    Ignored,
    Filtered,
    ParentFailure,
    Undetermined,
}

impl ComponentResult {
    pub fn passed() -> Self {
        Self::Pass(PassReason::Accepted)
    }

    pub fn rejection_exempt() -> Self {
        Self::Pass(PassReason::FailureAllowed)
    }

    pub fn child_failure() -> Self {
        Self::Fail(FailureReason::ChildFailure)
    }

    pub fn timed_out() -> Self {
        Self::Fail(FailureReason::Overtime)
    }

    pub fn rejected() -> Self {
        Self::Fail(FailureReason::Rejected)
    }

    pub fn ignored() -> Self {
        Self::DidNotRun(DidNotRunReason::Ignored)
    }

    pub fn filtered() -> Self {
        Self::DidNotRun(DidNotRunReason::Filtered)
    }

    pub fn parent_failure() -> Self {
        Self::DidNotRun(DidNotRunReason::ParentFailure)
    }

    pub fn undetermined() -> Self {
        Self::DidNotRun(DidNotRunReason::Undetermined)
    }

    pub fn has_failed(&self) -> bool {
        match self {
            Self::Pass(_) | Self::DidNotRun(_) => false,
            Self::Fail(_) => true,
        }
    }

    pub fn has_passed(&self) -> bool {
        match self {
            Self::Fail(_) | Self::DidNotRun(_) => false,
            Self::Pass(_) => true,
        }
    }

    pub fn has_not_run(&self) -> bool {
        match self {
            Self::Fail(_) | Self::Pass(_) => false,
            Self::DidNotRun(_) => true,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentTimeResult {
    pub time_taken: Duration,
    pub warn_threshold: Option<Duration>,
    pub critical_threshold: Option<Duration>,
}

impl ComponentTimeResult {
    pub fn zero() -> Self {
        Self {
            time_taken: Duration::from_secs(0),
            warn_threshold: None,
            critical_threshold: None,
        }
    }

    pub fn from_time(t: Duration) -> Self {
        Self {
            time_taken: t,
            warn_threshold: None,
            critical_threshold: None,
        }
    }

    pub fn new(
        t: Duration,
        warn_threshold: Option<Duration>,
        critical_threshold: Option<Duration>,
    ) -> Self {
        Self {
            time_taken: t,
            warn_threshold: warn_threshold,
            critical_threshold: critical_threshold,
        }
    }

    pub fn is_warn(&self) -> bool {
        match self.warn_threshold {
            Some(warn_threshold) => warn_threshold < self.duration(),
            None => false,
        }
    }

    pub fn is_critical(&self) -> bool {
        match self.critical_threshold {
            Some(critical_threshold) => critical_threshold < self.duration(),
            None => false,
        }
    }

    pub fn duration(&self) -> Duration {
        self.time_taken
    }
}
