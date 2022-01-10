use std::time::Duration;

use integra8_context::parameters::TestParameters;

use crate::{
    BookEnds, ComponentDescription, ComponentLocation, ComponentType, ConcurrencyMode, Test, ComponentPath, ComponentGeneratorId
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SuiteAttributes {

    /// Indicates that this entire suite should not be run.
    pub ignore: bool,

    /// Indicates that this suite should be run, but failures should be ignored and do not cascade.
    pub allow_suite_fail: bool,

    /// Describes the the default duration after which a test is flag as exceeded is expected duration.
    /// Tests which are a part of this suite, that do not advertize a warning threshold will inherit this value.
    pub test_warn_threshold: Duration,

    /// Describes the maximum duration a test can take before it is forcibly aborted.
    /// Tests which are a part of this suite, that do not advertize a critical threshold will inherit this value
    pub test_critical_threshold: Duration,

    /// The concurrency model used when executing this suite of tests.
    /// `ConcurrencyMode::Parallel` will allow this suite to be run at the same time as other suites.
    /// `ConcurrencyMode::Serial` will ensure this suite is only run on its own
    pub suite_concurrency_mode: ConcurrencyMode,

    /// Tests which are a part of this suite, that do not advertize a concurrency model will inherit this value
    /// `ConcurrencyMode::Parallel` will allow multiple tests to run at the same time
    /// `ConcurrencyMode::Serial` will ensure that only one test from this suite is run at the same time
    pub test_concurrency_mode: ConcurrencyMode,
}

impl SuiteAttributes {
    pub fn new<TParameters: TestParameters>(
        parent_desc: Option<&SuiteAttributes>,
        parameters: &TParameters,
        ignore: Option<bool>,
        allow_suite_fail: Option<bool>,
        test_warn_threshold: Option<Duration>,
        test_critical_threshold: Option<Duration>,
        suite_concurrency_mode: Option<ConcurrencyMode>,
        test_concurrency_mode: Option<ConcurrencyMode>,
    ) -> Self {
        Self {
            ignore: ignore.unwrap_or_else(|| parent_desc.map_or(false, |p| p.ignore)),

            allow_suite_fail: allow_suite_fail
                .unwrap_or_else(|| parent_desc.map_or(false, |p| p.allow_suite_fail)),

            test_warn_threshold: test_warn_threshold.map_or_else(
                || {
                    parent_desc.map_or_else(
                        || parameters.warn_threshold_duration(), // root value
                        |p| p.test_warn_threshold,
                    )
                },
                |val| val.clone(),
            ),

            test_critical_threshold: test_critical_threshold.map_or_else(
                || {
                    parent_desc.map_or_else(
                        || parameters.critical_threshold_duration(), // root value
                        |p| p.test_critical_threshold,
                    )
                },
                |val| val.clone(),
            ),

            suite_concurrency_mode: suite_concurrency_mode.map_or_else(
                || {
                    parent_desc.map_or_else(
                        || {
                            if parameters.max_concurrency() == 1
                                || !parameters.run_suites_in_parallel()
                            {
                                ConcurrencyMode::Serial
                            } else {
                                ConcurrencyMode::Parallel
                            }
                        },
                        // root value,
                        |p| p.suite_concurrency_mode.clone(),
                    )
                },
                |val| val.clone(),
            ),

            test_concurrency_mode: test_concurrency_mode.map_or_else(
                || {
                    parent_desc.map_or_else(
                        || {
                            if parameters.max_concurrency() == 1
                                || !parameters.run_tests_in_parallel()
                            {
                                ConcurrencyMode::Serial
                            } else {
                                ConcurrencyMode::Parallel
                            }
                        },
                        |p| p.test_concurrency_mode.clone(),
                    )
                },
                |val| val.clone(),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Suite<TParameters> {
    pub attributes: SuiteAttributes,
    pub description: ComponentDescription,
    pub tests: Vec<Test<TParameters>>,
    pub bookends: Vec<BookEnds<TParameters>>,
    pub suites: Vec<Suite<TParameters>>,
}

impl<TParameters: TestParameters> Suite<TParameters> {
    pub fn new(
        parent: Option<(&SuiteAttributes, &ComponentDescription)>,
        parameters: &TParameters,
        name: Option<&'static str>,
        id_gen: &mut ComponentGeneratorId,
        description: Option<&'static str>,
        path: &'static str,
        ignore: Option<bool>,
        src: Option<ComponentLocation>,
        allow_suite_fail: Option<bool>,
        test_warn_threshold: Option<Duration>,
        test_critical_threshold: Option<Duration>,
        suite_concurrency_mode: Option<ConcurrencyMode>,
        test_concurrency_mode: Option<ConcurrencyMode>,
    ) -> Suite<TParameters> {

        let id = id_gen.next();
        let (parent_path, parent_id) =
            parent.map(|p| {
                (p.1.path.clone(), p.1.id.clone())
            })
            // root nodes have themselves as their parent and an id of zero
            .unwrap_or_else(|| (ComponentPath::from(path), id.clone()));

        Suite {
            description: ComponentDescription::new(
                ComponentPath::from(path),
                name,   
                id,
                parent_path,
                parent_id,
                description,  
                ComponentType::Suite,
                src,
            ),
            attributes: SuiteAttributes::new(
                parent.map(|p| p.0),
                parameters,
                ignore,
                allow_suite_fail,
                test_warn_threshold,
                test_critical_threshold,
                suite_concurrency_mode,
                test_concurrency_mode,
            ),
            tests: Vec::new(),
            bookends: Vec::new(),
            suites: Vec::new(),
        }
    }
}
