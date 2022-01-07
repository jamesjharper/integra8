pub mod artifacts;
use artifacts::ComponentRunArtifacts;

pub mod summary;
use crate::components::{AcceptanceCriteria, ComponentDescription};

use std::time::Duration;

#[derive(Clone, PartialEq, Debug)]
pub struct ComponentRunReport {
    pub result: ComponentResult,
    pub timing: ComponentTimeResult,
    pub description: ComponentDescription,
    pub artifacts: ComponentRunArtifacts,
}

pub struct ComponentReportBuilder {
    description: ComponentDescription,
    acceptance_criteria: AcceptanceCriteria,
    timing: Option<ComponentTimeResult>,
    result: Option<ComponentResult>,
    artifacts: Option<ComponentRunArtifacts>,
}

impl ComponentReportBuilder {
    pub fn new(test_desc: ComponentDescription, acceptance_criteria: AcceptanceCriteria) -> Self {
        Self {
            acceptance_criteria: acceptance_criteria,
            description: test_desc,
            result: None,
            timing: None,
            artifacts: None,
        }
    }

    pub fn with_result(&mut self, result: impl Into<ComponentResult>) {
        self.result = Some(result.into());
    }

    pub fn passed_result(&mut self) {
        self.result = Some(ComponentResult::passed());
    }

    pub fn rejected_result(&mut self) {
        self.result = Some(ComponentResult::rejected());
    }

    pub fn ignored_result(&mut self) {
        self.result = Some(ComponentResult::ignored());
    }

    pub fn undetermined_result(&mut self) {
        self.result = Some(ComponentResult::undetermined());
    }

    pub fn filtered_result(&mut self) {
        self.result = Some(ComponentResult::filtered());
    }

    pub fn parent_failure_result(&mut self) {
        self.result = Some(ComponentResult::parent_failure());
    }

    pub fn time_until_deadline(&self, duration: Duration) -> Option<Duration> {
        match self.acceptance_criteria.timing.critical_threshold {
            Some(critical_threshold) => Some(critical_threshold.saturating_sub(duration)),
            None => None,
        }
    }

    pub fn time_taken(&mut self, duration: Duration) {
        self.timing = Some(ComponentTimeResult::new(
            duration,
            self.acceptance_criteria.timing.warn_threshold.clone(),
            self.acceptance_criteria.timing.critical_threshold.clone(),
        ));
    }

    pub fn with_artifacts(&mut self, artifacts: impl Into<ComponentRunArtifacts>) {
        self.artifacts = Some(artifacts.into());
    }

    pub fn build(self) -> ComponentRunReport {
        ComponentRunReport {
            result: self.build_result(),
            timing: self.timing.unwrap_or_else(|| ComponentTimeResult::zero()),
            description: self.description,
            artifacts: self.artifacts.unwrap_or_else(|| ComponentRunArtifacts {
                stdio: artifacts::stdio::TestResultStdio::no_output(),
            }),
        }
    }

    fn build_result(&self) -> ComponentResult {
        match &self.result {
            Some(r) => {
                let mut result = r.clone();
                if self
                    .timing
                    .as_ref()
                    .map(|t| t.is_critical())
                    .unwrap_or(false)
                {
                    result = ComponentResult::timed_out();
                }

                if result.has_failed() && self.acceptance_criteria.allowed_fail {
                    result = ComponentResult::rejection_exempt();
                }

                result
            }
            // if the component didn't run for some unknown reason,
            // the fault the component
            None => ComponentResult::rejected(),
        }
    }
}

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
