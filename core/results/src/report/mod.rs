use crate::artifacts::ComponentRunArtifacts;
use crate::{ComponentResult, ComponentTimeResult};

use integra8_components::{AcceptanceCriteria, ComponentDescription, ExecutionArtifacts};

use std::time::Duration;

#[cfg(feature = "enable_serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "enable_serde", derive(Serialize, Deserialize))]
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

    pub fn filtered_result(&mut self) {
        self.result = Some(ComponentResult::filtered());
    }

    pub fn timed_out_result(&mut self) {
        self.result = Some(ComponentResult::timed_out());
    }

    pub fn time_until_deadline(&self, duration: Duration) -> Option<Duration> {
        match self.acceptance_criteria.timing.time_limit {
            Some(time_limit) => Some(time_limit.saturating_sub(duration)),
            None => None,
        }
    }

    pub fn time_taken(&mut self, duration: Duration) {
        self.timing = Some(ComponentTimeResult::new(
            duration,
            self.acceptance_criteria.timing.warning_time_limit.clone(),
            self.acceptance_criteria.timing.time_limit.clone(),
        ));
    }

    pub fn with_artifacts(&mut self, artifacts: &ExecutionArtifacts) {
        self.artifacts = Some(
            ComponentRunArtifacts::from_execution_artifacts(artifacts)
        );
    }

    pub fn build(self) -> ComponentRunReport {
        ComponentRunReport {
            result: self.build_result(),
            timing: self.timing.unwrap_or_else(|| ComponentTimeResult::zero()),
            description: self.description,
            artifacts: self
                .artifacts
                .unwrap_or_else(|| ComponentRunArtifacts::new()),
        }
    }

    fn build_result(&self) -> ComponentResult {
        match &self.result {
            Some(r) => {
                let mut result = r.clone();

                if result.has_failed() && self.acceptance_criteria.allowed_fail {
                    result = ComponentResult::rejection_exempt();
                }

                if result.has_failed() {
                    return result;
                }

                // ** Subtle design choice here.
                // Tests which allowed_fail will still fail
                // if they time out.
                // However tests which are rejected, do not.
                if let Some(timing_results) = &self.timing {
                    if timing_results.is_critical() {
                        result = ComponentResult::timed_out();
                    } else if timing_results.is_warn() {
                        result = ComponentResult::time_out_warning();
                    }
                }

                result
            }
            // if the component didn't run for some unknown reason,
            // the fail the component
            None => ComponentResult::rejected(),
        }
    }
}
