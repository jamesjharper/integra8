use std::time::Duration;

use crate::{
    TestParameters, Delegate, ComponentDescription, ComponentLocation, ComponentType, ConcurrencyMode, SuiteAttributes, ComponentPath, ComponentGeneratorId
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TestAttributes {
    /// Indicates that test should be run, however failures should be ignored and do not cascade.
    pub allow_fail: bool,

    /// Indicates that test should not be run.
    pub ignore: bool,

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
        parameters: &TParameters,
        ignore: Option<bool>,
        allow_fail: Option<bool>,
        warn_threshold: Option<Duration>,
        critical_threshold: Option<Duration>,
        concurrency_mode: Option<ConcurrencyMode>,
    ) -> Self {
        Self {
            // If we are running as a child process, we need the test
            // to report as failed, so that way the process status indicates
            // an error, and the parent process will flag as allowed failure
            allow_fail: match parameters.is_child_process() {
                true => false,
                false => allow_fail.unwrap_or(false),
            },
            ignore: ignore.unwrap_or_else(|| parent_desc.ignore),

            warn_threshold: warn_threshold
                .map_or_else(|| parent_desc.test_warn_threshold, |val| val),

            critical_threshold: critical_threshold
                .map_or_else(|| parent_desc.test_critical_threshold, |val| val),

            concurrency_mode: concurrency_mode
                .map_or_else(|| parent_desc.test_concurrency_mode.clone(), |val| val),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Test<TParameters> {
    pub attributes: TestAttributes,
    pub description: ComponentDescription,
    pub test_fn: Delegate<TParameters>,
}

impl<TParameters: TestParameters> Test<TParameters> {
    pub fn new(
        parent_description: &ComponentDescription,
        parent_attributes: &SuiteAttributes,
        parameters: &TParameters,
        id_gen: &mut ComponentGeneratorId,
        name: Option<&'static str>,
        description: Option<&'static str>,
        path: &'static str,
        src: Option<ComponentLocation>,
        ignore: Option<bool>,
        allow_fail: Option<bool>,
        warn_threshold: Option<Duration>,
        critical_threshold: Option<Duration>,
        concurrency_mode: Option<ConcurrencyMode>,
        test_fn: Delegate<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription::new(
                ComponentPath::from(path),
                name,    
                id_gen.next(),
                parent_description.path.clone(),
                parent_description.id.clone(),
                description,  
                ComponentType::Test,
                src,
            ),
            attributes: TestAttributes::new(
                parent_attributes,
                parameters,
                ignore,
                allow_fail,
                warn_threshold,
                critical_threshold,
                concurrency_mode,
            ),
            test_fn: test_fn,
        }
    }
}
