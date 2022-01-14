use futures::executor::block_on;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

use crate::ExecutionContext;

#[derive(Clone)]
pub enum Delegate<TParameters> {
    SyncWithoutContext(fn()),
    SyncWithContext(fn(ExecutionContext<'_, TParameters>)),
    AsyncWithoutContext(fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
    AsyncWithContext(
        fn(ExecutionContext<'_, TParameters>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ),
}

impl<TParameters> fmt::Debug for Delegate<TParameters> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SyncWithoutContext(del) => f
                .debug_struct("SyncWithoutContext")
                .field("del", del)
                .finish(),
            Self::SyncWithContext(_) => f.debug_struct("SyncWithContext").finish(),
            Self::AsyncWithoutContext(del) => f
                .debug_struct("AsyncWithoutContext")
                .field("del", del)
                .finish(),
            Self::AsyncWithContext(_) => f.debug_struct("AsyncWithContext").finish(),
        }
    }
}

impl<TParameters> Delegate<TParameters> {
    pub fn sync_without_context(del: fn()) -> Self {
        Self::SyncWithoutContext(del)
    }

    pub fn sync_with_context(del: fn(ExecutionContext<'_, TParameters>)) -> Self {
        Self::SyncWithContext(del)
    }

    pub fn async_without_context(
        del: fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ) -> Self {
        Self::AsyncWithoutContext(del)
    }

    pub fn async_with_context(
        del: fn(
            ExecutionContext<'_, TParameters>,
        ) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    ) -> Self {
        Self::AsyncWithContext(del)
    }

    pub async fn run_async(&self, ctx: ExecutionContext<'_, TParameters>) {
        match self {
            Self::SyncWithoutContext(del) => (del)(),
            Self::SyncWithContext(del) => (del)(ctx),
            Self::AsyncWithoutContext(del) => (del)().await,
            Self::AsyncWithContext(del) => (del)(ctx).await,
        }
    }

    pub fn run_sync(&self, ctx: ExecutionContext<'_, TParameters>) {
        match self {
            Self::SyncWithoutContext(del) => (del)(),
            Self::SyncWithContext(del) => (del)(ctx),
            Self::AsyncWithoutContext(del) => block_on((del)()),
            Self::AsyncWithContext(del) => block_on((del)(ctx)),
        }
    }
}
