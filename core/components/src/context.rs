use std::time::Duration;
use std::sync::Arc;


use crate::{ComponentDescription, ConcurrencyMode};

#[derive(Clone, Debug)]
pub struct ExecutionContext<TParameters> {
    pub parameters: Arc<TParameters>,
    pub description: ComponentDescription,
}

/*
use std::collections::HashMap;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};


use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

// Create fake "file"
let mut c = Cursor::new(Vec::new());

pub struct ContextArtifacts {
    artifacts: HashMap<String, Box<dyn BufReader<>u8>>,
}

impl ContextArtifacts {

    pub fn append_artifact(&mut )
}
*/

#[derive(Clone)]
pub enum ExecutionStrategy {
    ProcessInternal,
    ProcessExternal,
}

pub trait TestParameters {
    // Parameter Projections
    fn is_multi_threaded(&self) -> bool {
        // TODO: remove
        if self.max_concurrency() == 1 {
            return false;
        }
        true
    }

    fn setup_time_limit_duration(&self) -> Duration;

    fn tear_down_time_limit_duration(&self) -> Duration;

    fn test_time_limit_duration(&self) -> Duration;

    fn test_warning_time_limit_duration(&self) -> Duration;

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

   /* fn default_setup_time_limit(&self) -> u64;
    fn tear_down_time_limit_seconds(&self) -> u64;
    fn test_time_limit_seconds(&self) -> u64;
    fn test_warning_time_threshold_seconds(&self) -> u64;*/
    fn test_concurrency(&self) -> ConcurrencyMode;
    fn suite_concurrency(&self) -> ConcurrencyMode;
    fn child_process_target(&self) -> Option<&'_ str>;

    fn max_concurrency(&self) -> usize;
    fn root_namespace(&self) -> &'static str;
    fn use_child_processes(&self) -> bool;

    fn console_output_style(&self) -> &'_ str;
    fn console_output_detail_level(&self) -> &'_ str;
    fn console_output_encoding(&self) -> &'_ str;
    fn console_output_ansi_mode(&self) -> &'_ str;
}
