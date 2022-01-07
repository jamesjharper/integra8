use std::time::Duration;

use crate::decorations::ConcurrencyMode;

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
        self.run_suites_in_parallel() || self.run_tests_in_parallel()
    }

    fn critical_threshold_duration(&self) -> Duration {
        std::time::Duration::from_secs(self.critical_threshold_seconds())
    }

    fn warn_threshold_duration(&self) -> Duration {
        std::time::Duration::from_secs(self.warn_threshold_seconds())
    }

    fn suite_concurrency_mode(&self) -> ConcurrencyMode {
        if self.max_concurrency() == 1 || !self.run_suites_in_parallel() {
            ConcurrencyMode::Serial
        } else {
            ConcurrencyMode::Parallel
        }
    }

    fn test_concurrency_mode(&self) -> ConcurrencyMode {
        if self.max_concurrency() == 1 || !self.run_tests_in_parallel() {
            ConcurrencyMode::Serial
        } else {
            ConcurrencyMode::Parallel
        }
    }

    // User defined

    fn run_suites_in_parallel(&self) -> bool {
        true
    }

    fn run_tests_in_parallel(&self) -> bool {
        true
    }

    fn critical_threshold_seconds(&self) -> u64;
    fn warn_threshold_seconds(&self) -> u64;
    fn max_concurrency(&self) -> usize;

    fn is_child_process(&self) -> bool;
    fn filter(&self) -> Option<String>;

    fn root_namespace(&self) -> &'static str;

    fn output_formatter(&self) -> String;

    fn use_child_processes(&self) -> bool;

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
        match &self.filter() {
            Some(name) => name != component_path,
            None => false,
        }
    }
}

use crate::formaters::OutputFormatter;

pub trait FormatterParameters {
    fn create_formatter(&self) -> Option<Box<dyn OutputFormatter>>;
}
