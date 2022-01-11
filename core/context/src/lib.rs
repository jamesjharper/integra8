
pub mod delegates;
pub mod parameters;

use std::sync::Arc;

pub struct ExecutionContext<TParameters> {
    pub parameters: Arc<TParameters>,
}

#[derive(Clone)]
pub enum ExecutionStrategy {
    ProcessInternal,
    ProcessExternal,
}
