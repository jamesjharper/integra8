use std::time::Duration;



use integra8_components::{TestParameters, ComponentLocation, ConcurrencyMode, Suite, SuiteAttributes, ComponentDescription, ComponentGeneratorId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SuiteAttributesDecoration {
    // The name of the suite (Default: the suite namespace)
    pub name: Option<&'static str>,

    // A description of the suite which can be displayed by the output formatter if it supports it
    pub description: Option<&'static str>,

    /// The test path used to calculate the suite's test group
    pub path: &'static str,

    /// The source code location of this test
    pub location: Option<ComponentLocation>,

    /// Indicates that this entire suite should not be run.
    pub ignore: Option<bool>,

    /// Indicates that this suite should be run, but failures should be ignored and do not cascade.
    pub allow_suite_fail: Option<bool>,

    /// Describes the the default duration after which a test is flag as exceeded is expected duration.
    /// Tests which are a part of this suite, that do not advertize a warning threshold will inherit this value.
    pub test_warn_threshold: Option<Duration>,

    /// Describes the maximum duration a test can take before it is forcibly aborted.
    /// Tests which are a part of this suite, that do not advertize a critical threshold will inherit this value
    pub test_critical_threshold: Option<Duration>,

    /// The concurrency model used when executing this suite of tests.
    /// `ConcurrencyMode::Parallel` will allow this suite to be run at the same time as other suites.
    /// `ConcurrencyMode::Serial` will ensure this suite is only run on its own
    pub suite_concurrency_mode: Option<ConcurrencyMode>,

    /// Tests which are a part of this suite, that do not advertize a concurrency model will inherit this value
    /// `ConcurrencyMode::Parallel` will allow multiple tests to run at the same time
    /// `ConcurrencyMode::Serial` will ensure that only one test from this suite is run at the same time
    pub test_concurrency_mode: Option<ConcurrencyMode>,
}

impl SuiteAttributesDecoration {
    pub fn root(namespace: &'static str) -> Self {
        Self {
            name: Some(namespace),
            path: namespace,
            description: None,
            location: None,
            ignore: None,
            allow_suite_fail: None,
            test_warn_threshold: None,
            test_critical_threshold: None,
            suite_concurrency_mode: None,
            test_concurrency_mode: None,
        }
    }

    pub fn into_component<TParameters: TestParameters>(
        self,
        id_gen: &mut ComponentGeneratorId,
        parent: Option<(&SuiteAttributes, &ComponentDescription)>,
        parameters: &TParameters,
    ) -> Suite<TParameters> {
        Suite::new(
            parent,
            parameters,
            id_gen,
            self.name,
            self.description,
            self.path,
            self.ignore,
            self.location,
            self.allow_suite_fail,
            self.test_warn_threshold,
            self.test_critical_threshold,
            self.suite_concurrency_mode,
            self.test_concurrency_mode,
        )
    }
}
