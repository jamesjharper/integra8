use futures::executor::block_on;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

use crate::ExecutionContext;

#[derive(Clone)]
pub enum Delegate<TParameters> {
    SyncWithoutContext(fn()),
    SyncWithContext(fn(ExecutionContext<TParameters>)),
    AsyncWithoutContext(fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>),
    AsyncWithContext(
        fn(ExecutionContext<TParameters>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
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

    pub fn requires_parameters(&self) -> bool {
        match self {
            Self::SyncWithoutContext(_) | Self::AsyncWithoutContext(_) => false,
            Self::SyncWithContext(_) | Self::AsyncWithContext(_) => true,
        }
    }

    pub async fn run_async_without_parameters(&self) {
        match self {
            Self::SyncWithoutContext(del) => (del)(),
            Self::AsyncWithoutContext(del) => (del)().await,
            _ => panic!("Parameter are required to invoke the delegate"),
        }
    }
    pub fn run_sync_without_parameters(&self) {
        match self {
            Self::SyncWithoutContext(del) => (del)(),
            Self::AsyncWithoutContext(del) => block_on((del)()),
            _ => panic!("Parameter are required to invoke the delegate"),
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

    pub async fn run_async(&self, ctx: ExecutionContext<TParameters>) {
        match self {
            Self::SyncWithoutContext(del) => (del)(),
            Self::SyncWithContext(del) => (del)(ctx),
            Self::AsyncWithoutContext(del) => (del)().await,
            Self::AsyncWithContext(del) => (del)(ctx).await,
        }
    }
}
