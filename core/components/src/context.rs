use std::time::Duration;

use crate::{ConcurrencyMode, ComponentDescription};

#[derive(Clone, Debug)]
pub struct ExecutionContext<'a, TParameters> {
    pub parameters: &'a TParameters,
    pub description: &'a ComponentDescription,
}

#[derive(Clone)]
pub enum ExecutionStrategy {
    ProcessInternal,
    ProcessExternal,
}

pub trait TestParameters {
    // Parameter Projections
    fn is_multi_threaded(&self) -> bool {
        if self.max_concurrency() == 1 {
            return false;
        }
        true
       // self.run_suites_in_parallel() || self.run_tests_in_parallel()
    }

    fn setup_critical_threshold_duration(&self) -> Duration {
        Duration::from_secs(self.setup_critical_threshold_seconds())
    }

    fn tear_down_critical_threshold_duration(&self) -> Duration {
        Duration::from_secs(self.tear_down_critical_threshold_seconds())
    }

    fn test_critical_threshold_duration(&self) -> Duration {
        Duration::from_secs(self.test_critical_threshold_seconds())
    }

    fn test_warn_threshold_duration(&self) -> Duration {
        Duration::from_secs(self.test_warn_threshold_seconds())
    }

    fn is_child_process(&self) -> bool {
        self.child_process_target().is_some()
    }

    fn execution_strategy(&self) -> ExecutionStrategy {
        if self.is_child_process() {
            return ExecutionStrategy::ProcessInternal;
        }
        if !self.use_child_processes() {
            return ExecutionStrategy::ProcessInternal;
        }
        ExecutionStrategy::ProcessExternal
    }

    fn exclude_component_predicate(&self, component_path: &str) -> bool {
        match &self.child_process_target() {
            Some(name) => name != &component_path,
            None => false,
        }
    }

    // User defined

    fn setup_critical_threshold_seconds(&self) -> u64;
    fn tear_down_critical_threshold_seconds(&self) -> u64;
    fn test_critical_threshold_seconds(&self) -> u64;
    fn test_warn_threshold_seconds(&self) -> u64;
    fn test_concurrency(&self) -> ConcurrencyMode;
    fn suite_concurrency(&self) -> ConcurrencyMode;
    fn child_process_target<'a>(&'a self) -> Option<&'a str>;

    fn max_concurrency(&self) -> usize;
    fn root_namespace(&self) -> &'static str;
    fn use_child_processes(&self) -> bool;


}
