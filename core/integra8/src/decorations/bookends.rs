use crate::runner::context::ExecutionContext;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use super::SourceLocation;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEndDecorationPair<TParameters> {
    pub setup: Option<BookEndDecoration<TParameters>>,
    pub tear_down: Option<BookEndDecoration<TParameters>>,
}

impl<TParameters> BookEndDecorationPair<TParameters> {
    pub fn new() -> Self {
        Self {
            setup: None,
            tear_down: None,
        }
    }

    pub fn has_any(&self) -> bool {
        self.setup.is_some() || self.tear_down.is_some()
    }
}

impl<TParameters> Default for BookEndDecorationPair<TParameters> {
    fn default() -> Self {
        Self {
            setup: None,
            tear_down: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BookEndAttributesDecoration {
    /// The path used to calculate the bookends test group
    pub path: &'static str,

    /// The source code location of this bookend
    pub location: SourceLocation,

    /// Indicates that bookend should not be run.
    pub ignore: Option<bool>,

    /// A Cascading failure will result in automatic failure of all other yet to be run test in this test group.
    pub cascade_failure: Option<bool>,

    /// Describes the maximum duration a bookend can take before it is forcibly aborted
    pub critical_threshold: Option<Duration>,
}

#[cfg(feature = "async")]
pub type BookEndDecoration<TParameters> = bookend_async_impl::BookendDecorationAsync<TParameters>;

#[cfg(feature = "async")]
mod bookend_async_impl {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct BookendDecorationAsync<TParameters> {
        pub desc: BookEndAttributesDecoration,
        pub bookend_fn:
            fn(ExecutionContext<TParameters>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    }

    impl<TParameters> BookendDecorationAsync<TParameters> {
        pub async fn run(&self, params: ExecutionContext<TParameters>) {
            (self.bookend_fn)(params).await
        }
    }
}

#[cfg(feature = "sync")]
pub type BookEndDecoration<TParameters> = bookend_sync_impl::BookendDecorationSync<TParameters>;

#[cfg(feature = "sync")]
mod bookend_sync_impl {

    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct BookendDecorationSync<TParameters> {
        pub desc: BookEndAttributesDecoration,
        pub bookend_fn: fn(ExecutionContext<TParameters>),
    }

    impl<TParameters> BookendDecorationSync<TParameters> {
        pub fn run(&self, params: ExecutionContext<TParameters>) {
            (self.bookend_fn)(params)
        }
    }
}
