use crate::artifacts::stdio::TestResultStdio;
use crate::artifacts::ComponentRunArtifacts;
use crate::{ComponentResult, ComponentTimeResult};

use integra8_components::AcceptanceCriteria;
use integra8_context::meta::ComponentDescription;

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
                stdio: TestResultStdio::no_output(),
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
