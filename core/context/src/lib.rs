pub mod meta;
pub mod parameters;
pub mod delegates;

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
