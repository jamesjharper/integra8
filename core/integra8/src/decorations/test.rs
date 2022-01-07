use std::time::Duration;

use super::{ConcurrencyMode, SourceLocation};
use crate::runner::context::ExecutionContext;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TestAttributesDecoration {
    // The name of the test (Default: the tests namespace + test method name)
    pub name: &'static str,

    /// The test path used to calculate the test's test group
    pub path: &'static str,

    /// The source code location of this test
    pub location: SourceLocation,

    /// Indicates that test should be run, however failures should be ignored and do not cascade.
    pub allow_fail: Option<bool>,

    /// Indicates that test should not be run.
    pub ignore: Option<bool>,

    /// A Cascading failure will result in automatic failure of all other yet to be run test in this test group.
    pub cascade_failure: Option<bool>,

    /// Describes the the duration after which a test is flag as exceeded is expected duration.
    /// This can be used to give early warnings that a test is going to exceed some critical threshold.
    /// For example, a HTTP request time out.
    pub warn_threshold: Option<Duration>,

    /// Describes the maximum duration a test can take before it is forcibly aborted
    pub critical_threshold: Option<Duration>,

    /// The concurrency mode which this test will adhere to.
    /// `ConcurrencyMode::Parallel` will allow this test for be run at the same time as other tests within this tests suite
    /// `ConcurrencyMode::Serial` will ensure that this test wont run at the same time as any other test from this suite
    pub concurrency_mode: Option<ConcurrencyMode>,
}

#[cfg(feature = "async")]
pub type TestDecoration<TParameters> =
    integration_test_async_impl::TestDecorationAsync<TParameters>;

#[cfg(feature = "async")]
mod integration_test_async_impl {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;

    #[derive(Debug)]
    pub struct TestDecorationAsync<TParameters> {
        pub desc: TestAttributesDecoration,
        pub test_fn:
            fn(ExecutionContext<TParameters>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    }

    impl<TParameters> TestDecorationAsync<TParameters> {
        pub async fn run(&self, params: ExecutionContext<TParameters>) {
            (self.test_fn)(params).await
        }
    }
}

#[cfg(feature = "sync")]
pub type TestDecoration<TParameters> = integration_test_sync_impl::TestDecorationSync<TParameters>;

#[cfg(feature = "sync")]
mod integration_test_sync_impl {

    use super::*;
    #[derive(Debug)]
    pub struct TestDecorationSync<TParameters> {
        pub desc: TestAttributesDecoration,
        pub test_fn: fn(ExecutionContext<TParameters>),
    }

    impl<TParameters> TestDecorationSync<TParameters> {
        pub fn run(&self, params: ExecutionContext<TParameters>) {
            (self.test_fn)(params)
        }
    }
}
