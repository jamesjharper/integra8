use std::time::Duration;

use integra8_components::{
    ComponentDescription, ComponentId, ComponentLocation, ComponentPath, ConcurrencyMode, Suite,
    SuiteAttributes, TestParameters,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SuiteAttributesDecoration {
    // The name of the suite (Default: the suite namespace)
    pub name: Option<&'static str>,

    // A description of the suite which can be displayed by the output formatter if it supports it
    pub description: Option<&'static str>,

    /// The source code location of this test
    pub location: ComponentLocation,

    /// Indicates that this entire suite should not be run.
    pub ignore: Option<bool>,

    /// Indicates that this suite should be run, but failures should be ignored and do not cascade.
    pub allow_suite_fail: Option<bool>,

    /// The duration after which a test is flagged as exceeded is expected duration.
    /// This can be used to give early warnings before a test exceeds some critical threshold.
    /// For example, a HTTP request time out.
    ///
    /// Tests which are a part of this suite, that do not advertize a warning time limit will inherit this value.
    pub test_warning_time_limit: Option<Duration>,

    /// Describes the maximum duration a `test` can take before it is forcibly aborted.
    /// Tests which are a part of this suite, that do not advertize a time limit will inherit this value
    pub test_time_limit: Option<Duration>,

    /// Describes the maximum duration a `setup` can take before it is forcibly aborted.
    /// Setups which are a part of this suite, that do not advertize a time limit will inherit this value
    pub setup_time_limit: Option<Duration>,

    /// Describes the maximum duration a `tear down` can take before it is forcibly aborted.
    /// Tear downs which are a part of this suite, that do not advertize a time limit will inherit this value
    pub tear_down_time_limit: Option<Duration>,

    /// The concurrency model used when executing this suite of tests.
    /// `ConcurrencyMode::Parallel` will allow this suite to be run at the same time as other suites.
    /// `ConcurrencyMode::Sequential` will ensure this suite is only run on its own
    pub suite_concurrency_mode: Option<ConcurrencyMode>,

    /// Tests which are a part of this suite, that do not advertize a concurrency model will inherit this value
    /// `ConcurrencyMode::Parallel` will allow multiple tests to run at the same time
    /// `ConcurrencyMode::Sequential` will ensure that only one test from this suite is run at the same time
    pub test_concurrency_mode: Option<ConcurrencyMode>,
}

impl SuiteAttributesDecoration {
    pub fn root(path: &'static str) -> Self {
        Self {
            name: Some(path),
            description: None,
            location: ComponentLocation {
                path: ComponentPath::from(path),
                file_name: std::borrow::Cow::from("main.rs"),
                column: 0,
                line: 0,
            },
            ignore: None,
            allow_suite_fail: None,
            test_warning_time_limit: None,
            test_time_limit: None,
            setup_time_limit: None,
            tear_down_time_limit: None,
            suite_concurrency_mode: None,
            test_concurrency_mode: None,
        }
    }

    pub fn into_component<TParameters: TestParameters>(
        self,
        id: ComponentId,
        parent: Option<(&SuiteAttributes, &ComponentDescription)>,
        parameters: &TParameters,
    ) -> Suite<TParameters> {
        Suite::new(
            parent,
            parameters,
            id,
            self.name,
            self.description,
            self.ignore,
            self.location,
            self.allow_suite_fail,
            self.test_warning_time_limit,
            self.test_time_limit,
            self.setup_time_limit,
            self.tear_down_time_limit,
            self.suite_concurrency_mode,
            self.test_concurrency_mode,
        )
    }
}
