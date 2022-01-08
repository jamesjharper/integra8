use std::time::Duration;

use integra8_context::parameters::TestParameters;

use crate::{
    BookEnds, 
    Test,
    ComponentIdentity,
    ComponentDescription,
    ConcurrencyMode
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SuiteAttributes {
    /// The identity of the suite. Used for uniquely identify the suite and displaying the suite name to the end user.
    pub identity: ComponentIdentity,

    /// Indicates that this entire suite should not be run.
    pub ignore: bool,

    /// Indicates that this suite should be run, but failures should be ignored and do not cascade.
    pub allow_suite_fail: bool,

    /// The owning parent of this test suite, or None if root
    pub parent_suite_identity: Option<ComponentIdentity>,

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
        name: Option<&'static str>,
        path: &'static str,
        ignore: Option<bool>,
        allow_suite_fail: Option<bool>,
        test_warn_threshold: Option<Duration>,
        test_critical_threshold: Option<Duration>,
        suite_concurrency_mode: Option<ConcurrencyMode>,
        test_concurrency_mode: Option<ConcurrencyMode>,
    ) -> Self {
        Self {
            identity: ComponentIdentity::new(name, path),
            ignore: ignore.unwrap_or_else(|| parent_desc.map_or(false, |p| p.ignore)),

            allow_suite_fail: allow_suite_fail.unwrap_or_else(|| parent_desc.map_or(false, |p| p.allow_suite_fail)),

            parent_suite_identity: parent_desc.map(|p| p.identity.clone()),
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
                            if parameters.max_concurrency() == 1 || !parameters.run_suites_in_parallel() {
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
                            if parameters.max_concurrency() == 1 || !parameters.run_tests_in_parallel() {
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
    /*pub fn from_decorated_components<ComponentsIterator>(
        components: ComponentsIterator,
        parameters: &TParameters,
    ) -> Self
    where
        ComponentsIterator: IntoIterator<Item = ComponentDecoration<TParameters>>,
    {
        Self::from_component_groups(
            None,
            ComponentHierarchy::from_decorated_components(components).into_component_groups(),
            parameters,
        )
    }

    fn from_component_groups(
        parent_desc: Option<&SuiteAttributes>,
        group: ComponentGroup<TParameters>,
        parameters: &TParameters,
    ) -> Self {
        let parent_suite_attributes = group
            .suite
            .unwrap_or_else(|| SuiteAttributesDecoration::root(parameters.root_namespace()));

        let mut suite = Self::new(parent_desc, parent_suite_attributes, parameters);

        suite.tests = group
            .tests
            .into_iter()
            .map(|x| Test::new(&suite.description, &suite.attributes, x, parameters))
            .collect();

        suite.bookends = group
            .bookends
            .into_iter()
            .filter(|x| x.has_any())
            .map(|x| x::new(&suite.description, &suite.attributes, x))
            .collect();

        suite.suites = group
            .sub_groups
            .into_iter()
            .map(|x| Self::from_component_groups(Some(&suite.attributes), x, parameters))
            .collect();

        suite
    }

    fn new(
        parent_desc: Option<&SuiteAttributes>,
        decorations: SuiteAttributesDecoration,
        parameters: &TParameters,
    ) -> Self {
        Self {
            description: ComponentDescription {
                identity: ComponentIdentity::new(decorations.name, decorations.path),
                parent_identity: parent_desc
                    .map(|p| p.identity.clone())
                    .unwrap_or_else(|| ComponentIdentity::new(decorations.name, decorations.path)),
                component_type: ComponentType::Suite,
                location: decorations.location.clone(),
            },
            attributes: SuiteAttributes::new(parent_desc, decorations, parameters),
            tests: Vec::new(),
            bookends: Vec::new(),
            suites: Vec::new(),
        }
    }*/
}
