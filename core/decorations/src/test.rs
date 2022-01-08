use std::time::Duration;

use integra8_context::delegates::Delegate;
use integra8_context::parameters::TestParameters;

use integra8_components::{
    ComponentDescription, ComponentLocation, ConcurrencyMode, SuiteAttributes, Test,
};

#[derive(Debug)]
pub struct TestAttributesDecoration {
    // The name of the test (Default: the tests namespace + test method name)
    pub name: Option<&'static str>,

    // A description of the test which can be displayed by the output formatter if it supports it
    pub description: &'static str,

    /// The test path used to calculate the test's test group
    pub path: &'static str,

    /// The source code location of this test
    pub location: ComponentLocation,

    /// Indicates that test should be run, however failures should be ignored and do not cascade.
    pub allow_fail: Option<bool>,

    /// Indicates that test should not be run.
    pub ignore: Option<bool>,

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

#[derive(Debug)]
pub struct TestDecoration<TParameters> {
    pub desc: TestAttributesDecoration,
    pub test_fn: Delegate<TParameters>,
}

impl<TParameters: TestParameters> TestDecoration<TParameters> {
    pub fn into_component(
        self,
        parent_description: &ComponentDescription,
        parent_attributes: &SuiteAttributes,
        parameters: &TParameters,
    ) -> Test<TParameters> {
        Test::new(
            parent_description,
            parent_attributes,
            parameters,
            self.desc.name,
            self.desc.path,
            self.desc.location,
            self.desc.ignore,
            self.desc.allow_fail,
            self.desc.warn_threshold,
            self.desc.critical_threshold,
            self.desc.concurrency_mode,
            self.test_fn,
        )
    }
}
