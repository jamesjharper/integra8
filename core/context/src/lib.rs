pub mod parameters;
pub mod meta;

use std::sync::Arc;

pub struct ExecutionContext<TParameters> {
    pub parameters: Arc<TParameters>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConcurrencyMode {
    Parallel,
    Serial,
}

#[derive(Clone)]
pub enum ExecutionStrategy {
    ProcessInternal,
    ProcessExternal,
}
