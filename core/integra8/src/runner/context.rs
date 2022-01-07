use std::sync::Arc;

pub struct ExecutionContext<TParameters> {
    pub parameters: Arc<TParameters>,
}
