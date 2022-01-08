use crate::ExecutionContext;
use futures::executor::block_on;
use std::future::Future;
use std::pin::Pin;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Delegate<TParameters> {
    SyncWithoutContext(fn()),
    SyncWithContext(fn(ExecutionContext<TParameters>)),
    AsyncWithoutContext(fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
    AsyncWithContext(
        fn(ExecutionContext<TParameters>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ),
}

impl<TParameters> Delegate<TParameters> {
    pub fn sync_without_context(del: fn()) -> Self {
        Self::SyncWithoutContext(del)
    }

    pub fn sync_with_context(del: fn(ExecutionContext<TParameters>)) -> Self {
        Self::SyncWithContext(del)
    }

    pub fn async_without_context(
        del: fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ) -> Self {
        Self::AsyncWithoutContext(del)
    }

    pub fn async_with_context(
        del: fn(
            ExecutionContext<TParameters>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ) -> Self {
        Self::AsyncWithContext(del)
    }

    pub async fn run_async(&self, ctx: ExecutionContext<TParameters>) {
        match self {
            Self::SyncWithoutContext(del) => (del)(),
            Self::SyncWithContext(del) => (del)(ctx),
            Self::AsyncWithoutContext(del) => (del)().await,
            Self::AsyncWithContext(del) => (del)(ctx).await,
        }
    }

    pub fn run_sync(&self, ctx: ExecutionContext<TParameters>) {
        match self {
            Self::SyncWithoutContext(del) => (del)(),
            Self::SyncWithContext(del) => (del)(ctx),
            Self::AsyncWithoutContext(del) => block_on((del)()),
            Self::AsyncWithContext(del) => block_on((del)(ctx)),
        }
    }
}
