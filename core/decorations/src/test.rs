use std::time::Duration;

use integra8_components::{
    ComponentDescription, ComponentGeneratorId, ComponentLocation, ConcurrencyMode, Delegate,
    SuiteAttributes, Test, TestParameters,
};

#[derive(Debug)]
pub struct TestAttributesDecoration {
    // The name of the test (Default: the tests namespace + test method name)
    pub name: Option<&'static str>,

    // A description of the test which can be displayed by the output formatter if it supports it
    pub description: Option<&'static str>,

    /// The source code location of this test
    pub location: ComponentLocation,

    /// Indicates that test should be run, however failures should be ignored and do not cascade.
    pub allow_fail: Option<bool>,

    /// Indicates that test should not be run.
    pub ignore: Option<bool>,

    /// The the duration after which a test is flagged as exceeded is expected duration.
    /// This can be used to give early warnings before a test exceeds some critical threshold.
    /// For example, a HTTP request time out.
    pub warning_time_limit: Option<Duration>,

    /// The maximum duration a test can take before it is forcibly aborted
    pub time_limit: Option<Duration>,

    /// The concurrency mode which this test will adhere to.
    /// `ConcurrencyMode::Parallel` will allow this test for be run at the same time as other tests within this tests suite
    /// `ConcurrencyMode::Sequential` will ensure that this test wont run at the same time as any other test from this suite
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
        id_gen: &mut ComponentGeneratorId,
        parent_description: &ComponentDescription,
        parent_attributes: &SuiteAttributes,
        parameters: &TParameters,
    ) -> Test<TParameters> {
        Test::new(
            parent_description,
            parent_attributes,
            parameters,
            id_gen,
            self.desc.name,
            self.desc.description,
            self.desc.location,
            self.desc.ignore,
            self.desc.allow_fail,
            self.desc.warning_time_limit,
            self.desc.time_limit,
            self.desc.concurrency_mode,
            self.test_fn,
        )
    }
}
