use std::time::Duration;
use serde::{Serialize, Deserialize};

use crate::{
    ComponentDescription, ComponentId, ComponentLocation, ComponentType,
    ConcurrencyMode, Delegate, SuiteAttributes, TestParameters,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TestAttributes {
    /// Indicates that test should be run, however failures should be ignored and do not cascade.
    pub allow_fail: bool,

    /// Indicates that test should not be run.
    pub ignore: bool,

    /// The the duration after which a test is flagged as exceeded is expected duration.
    /// This can be used to give early warnings before a test exceeds some critical threshold.
    /// For example, a HTTP request time out.
    pub warning_time_limit: Duration,

    /// Describes the maximum duration a test can take before it is forcibly aborted
    pub time_limit: Duration,

    /// The concurrency mode which this test will adhere to.
    /// `ConcurrencyMode::Parallel` will allow this test for be run at the same time as other tests within this tests suite
    /// `ConcurrencyMode::Sequential` will ensure that this test wont run at the same time as any other test from this suite
    pub concurrency_mode: ConcurrencyMode,
}

impl TestAttributes {
    pub fn new<TParameters: TestParameters>(
        parent_desc: &SuiteAttributes,
        parameters: &TParameters,
        ignore: Option<bool>,
        allow_fail: Option<bool>,
        warning_time_limit: Option<Duration>,
        time_limit: Option<Duration>,
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

            warning_time_limit: warning_time_limit
                .map_or_else(|| parent_desc.test_warning_time_limit, |val| val),

            time_limit: time_limit.map_or_else(|| parent_desc.test_time_limit, |val| val),

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
        id: ComponentId,
        name: Option<&'static str>,
        description: Option<&'static str>,
        location: ComponentLocation,
        ignore: Option<bool>,
        allow_fail: Option<bool>,
        warning_time_limit: Option<Duration>,
        time_limit: Option<Duration>,
        concurrency_mode: Option<ConcurrencyMode>,
        test_fn: Delegate<TParameters>,
    ) -> Self {
        Self {
            description: ComponentDescription::new(
                name,
                id,
                parent_description.id().clone(),
                location,
                parent_description.location().clone(),
                description,
                ComponentType::Test,
            ),
            attributes: TestAttributes::new(
                parent_attributes,
                parameters,
                ignore,
                allow_fail,
                warning_time_limit,
                time_limit,
                concurrency_mode,
            ),
            test_fn: test_fn,
        }
    }
}
