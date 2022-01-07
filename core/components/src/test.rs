use std::time::Duration;

use crate::{SuiteAttributes};
use integra8_decorations::{TestAttributesDecoration, TestDecoration};

use integra8_context::ConcurrencyMode;
use integra8_context::meta::{ComponentDescription, ComponentIdentity, ComponentType};
use integra8_context::ExecutionContext;
use integra8_context::parameters::TestParameters;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TestAttributes {
    /// The identity of the test. Used for uniquely identify the test and displaying the test name to the end user.
    pub identity: ComponentIdentity,

    /// Indicates that test should be run, however failures should be ignored and do not cascade.
    pub allow_fail: bool,

    /// Indicates that test should not be run.
    pub ignore: bool,

    /// The owning suite of this test
    pub parent_suite_identity: ComponentIdentity,

    /// Describes the the duration after which a test is flag as exceeded is expected duration.
    /// This can be used to give early warnings that a test is going to exceed some critical threshold.
    /// For example, a HTTP request time out.
    pub warn_threshold: Duration,

    /// Describes the maximum duration a test can take before it is forcibly aborted
    pub critical_threshold: Duration,

    /// The concurrency mode which this test will adhere to.
    /// `ConcurrencyMode::Parallel` will allow this test for be run at the same time as other tests within this tests suite
    /// `ConcurrencyMode::Serial` will ensure that this test wont run at the same time as any other test from this suite
    pub concurrency_mode: ConcurrencyMode,
}

impl TestAttributes {
    pub fn new<TParameters: TestParameters>(
        parent_desc: &SuiteAttributes,
        def: TestAttributesDecoration,
        parameters: &TParameters,
    ) -> Self {
        Self {
            identity: ComponentIdentity::new(def.name, def.path),
            // If we are running as a child process, we need the test
            // to report as failed, so that way the process status indicates
            // an error, and the parent process will flag as allowed failure
            allow_fail: match parameters.is_child_process() {
                true => false,
                false => def.allow_fail.unwrap_or(false),
            },
            ignore: def.ignore.unwrap_or_else(|| parent_desc.ignore),

            parent_suite_identity: parent_desc.identity.clone(),

            warn_threshold: def
                .warn_threshold
                .map_or_else(|| parent_desc.test_warn_threshold, |val| val),

            critical_threshold: def
                .critical_threshold
                .map_or_else(|| parent_desc.test_critical_threshold, |val| val),

            concurrency_mode: def
                .concurrency_mode
                .map_or_else(|| parent_desc.test_concurrency_mode.clone(), |val| val),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Test<TParameters> {
    pub attributes: TestAttributes,
    pub description: ComponentDescription,
    pub test_fn: TestFn<TParameters>,
}

impl<TParameters: TestParameters> Test<TParameters> {
    pub fn new(
        parent_description: &ComponentDescription,
        parent_attributes: &SuiteAttributes,
        decorations: TestDecoration<TParameters>,
        parameters: &TParameters,
    ) -> Self {
        Self {
            description: ComponentDescription {
                identity: ComponentIdentity::new(decorations.desc.path, decorations.desc.path),
                parent_identity: parent_description.identity.clone(),
                component_type: ComponentType::Test,
                location: decorations.desc.location.clone(),
            },
            attributes: TestAttributes::new(parent_attributes, decorations.desc, parameters),
            test_fn: TestFn {
                test_fn: decorations.test_fn,
            },
        }
    }
}

#[cfg(feature = "async")]
pub type TestFn<TParameters> = test_async_impl::TestFnAsync<TParameters>;

#[cfg(feature = "async")]
mod test_async_impl {
    use super::*;

    use std::future::Future;
    use std::pin::Pin;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct TestFnAsync<TParameters> {
        pub test_fn:
            fn(ExecutionContext<TParameters>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>,
    }

    impl<TParameters> TestFnAsync<TParameters> {
        pub async fn run(&self, params: ExecutionContext<TParameters>) {
            (self.test_fn)(params).await
        }
    }
}

#[cfg(feature = "sync")]
pub type TestFn<TParameters> = test_sync_impl::TestFnSync<TParameters>;

#[cfg(feature = "sync")]
mod test_sync_impl {

    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct TestFnSync<TParameters> {
        pub test_fn: fn(ExecutionContext<TParameters>),
    }

    impl<TParameters> TestFnSync<TParameters> {
        pub fn run(&self, params: ExecutionContext<TParameters>) {
            (self.test_fn)(params)
        }
    }
}
