pub mod artifacts;
pub mod report;
pub mod summary;

use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentResult {
    Pass(PassReason),
    Warning(WarningReason),
    Fail(FailureReason),
    DidNotRun(DidNotRunReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WarningReason {
    FailureAllowed,
    OvertimeWarning,
    ChildWarning,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PassReason {
    Accepted,
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
        Self::Warning(WarningReason::FailureAllowed)
    }

    pub fn child_failure() -> Self {
        Self::Fail(FailureReason::ChildFailure)
    }

    pub fn child_warning() -> Self {
        Self::Warning(WarningReason::ChildWarning)
    }

    pub fn time_out_warning() -> Self {
        Self::Warning(WarningReason::OvertimeWarning)
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
            Self::Fail(_) => true,
            _ => false,
        }
    }

    pub fn has_passed(&self) -> bool {
        match self {
            Self::Pass(_) => true,
            _ => false,
        }
    }

    pub fn has_warn(&self) -> bool {
        match self {
            Self::Warning(_) => true,
            _ => false,
        }
    }

    pub fn has_not_run(&self) -> bool {
        match self {
            Self::DidNotRun(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentTimeResult {
    pub time_taken: Duration,
    pub warning_time_limit: Option<Duration>,
    pub time_limit: Option<Duration>,
}

impl ComponentTimeResult {
    pub fn zero() -> Self {
        Self {
            time_taken: Duration::from_secs(0),
            warning_time_limit: None,
            time_limit: None,
        }
    }

    pub fn from_time(t: Duration) -> Self {
        Self {
            time_taken: t,
            warning_time_limit: None,
            time_limit: None,
        }
    }

    pub fn new(
        t: Duration,
        warning_time_limit: Option<Duration>,
        time_limit: Option<Duration>,
    ) -> Self {
        Self {
            time_taken: t,
            warning_time_limit: warning_time_limit,
            time_limit: time_limit,
        }
    }

    pub fn is_warn(&self) -> bool {
        match self.warning_time_limit {
            Some(warning_time_limit) => warning_time_limit < self.duration(),
            None => false,
        }
    }

    pub fn is_critical(&self) -> bool {
        match self.time_limit {
            Some(time_limit) => time_limit < self.duration(),
            None => false,
        }
    }

    pub fn duration(&self) -> Duration {
        self.time_taken
    }
}
