mod test;
pub use test::{Test, TestAttributes};

mod bookends;
pub use bookends::{BookEnd, BookEndAttributes, BookEnds};

mod suite;
pub use suite::{Suite, SuiteAttributes};

mod acceptance_criteria;
pub use acceptance_criteria::{AcceptanceCriteria, TimingAcceptanceCriteria};

pub mod scheduling;
pub use scheduling::IntoTaskStateMachine;

use crate::decorations::{ComponentDecoration, SourceLocation};
use crate::parameters::TestParameters;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SuiteState {
    Start,
    Finish,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Component<TParameters> {
    Suite(ComponentDescription, SuiteAttributes, SuiteState),
    Test(Test<TParameters>),
    Setup(BookEnd<TParameters>),
    TearDown(BookEnd<TParameters>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentType {
    Suite,
    Test,
    Setup,
    TearDown,
}

impl ComponentType {
    pub fn is_tear_down(&self) -> bool {
        match self {
            Self::TearDown => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentDescription {
    /// The identity of the bookend. Used for uniquely identify the bookend and displaying the test name to the end user.
    pub identity: ComponentIdentity,

    pub component_type: ComponentType,

    pub parent_identity: ComponentIdentity,

    pub location: SourceLocation,
}

impl ComponentDescription {
    pub fn is_root(&self) -> bool {
        self.identity == self.parent_identity
    }

    pub fn relative_path(&self) -> String {
        if self.is_root() {
            return self.identity.path.to_string();
        }

        self.identity
            .path
            .strip_prefix(self.parent_identity.path)
            .map(|relative| {
                // Remove the :: prefix left over from the path
                relative.trim_start_matches(':').to_string()
            })
            .unwrap_or_else(|| self.identity.path.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentIdentity {
    // The friendly name of the component (Default: the namespace + ident)
    pub name: &'static str,

    /// The namespace + ident of the component
    pub path: &'static str,
}

impl ComponentIdentity {
    pub fn new(name: &'static str, path: &'static str) -> Self {
        Self { name, path }
    }
}

pub struct RootSuite();

impl RootSuite {
    pub fn from_decorated_components<ComponentsIterator, TParameters: TestParameters>(
        components: ComponentsIterator,
        parameters: &TParameters,
    ) -> Suite<TParameters>
    where
        ComponentsIterator: IntoIterator<Item = ComponentDecoration<TParameters>>,
    {
        Suite::<TParameters>::from_decorated_components(components, parameters)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    mod mock_test_app {

        use integra8_impl::integration_test;
        use integra8_impl::integration_suite;
        use integra8_impl::teardown;
        use integra8_impl::setup;

        #[integration_test]
        #[integra8(crate = crate)]
        pub fn test_c() { }

        #[integration_test]
        #[integra8(crate = crate)]
        pub fn test_b() { }

        #[integration_test]
        #[integra8(crate = crate)]
        pub fn test_a() { }

        #[integration_suite]
        #[integra8(crate = crate)]
        pub mod suite1 {
            pub use super::*;

            #[teardown]
            #[integra8(crate = crate)]
            fn teardown() { }

            #[setup]
            #[integra8(crate = crate)]
            fn setup() { }

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn suite1_test_c() { }

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn suite1_test_b() { }

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn suite1_test_a() { }

            pub mod nested_mod  {
                pub use super::*;

                #[teardown]
                #[integra8(crate = crate)]
                fn teardown() { }

                #[setup]
                #[integra8(crate = crate)]
                fn setup() { }

                #[integration_test]
                #[integra8(crate = crate)]
                pub fn suite1_nested_mod_test_d() { }
            }

            #[integration_suite]
            #[integra8(crate = crate)]
            pub mod nested_suite_1a {
                pub use super::*;

                #[integration_test]
                #[integra8(crate = crate)]
                pub fn nested_suite_1a_test_d() { }
            }
        }

        #[integration_suite]
        #[integra8(crate = crate)]
        pub mod suite2 {
            pub use super::*;

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn suite2_test_c() { }

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn suite2_test_b() { }

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn suite2_test_a() { }
        }
    }

    /*
    #[macro_export]
    macro_rules! assert_has_tests {
        (
            $tests:expr, $($test_name:expr), +,
        ) => {
            let mut _i = 0;
            $(
                assert_eq!($tests[_i].desc.name, $test_name);
                _i = _i + 1;
            )+
            assert_eq!($tests.len(), _i);
        }
    }

    #[macro_export]
    macro_rules! assert_has_suite {
        (
            $opt_suite:expr, $suite_name:expr
        ) => {

            let suite = $opt_suite.as_ref().unwrap();
            assert_eq!(suite.name, $suite_name);
        }
    }
    */



    #[macro_export]
    macro_rules! assert_has_suite {
        (
            $suite:expr,
            suite => $expected_suite_name:expr,
            tests => [ $($test_name:expr), + ],
        ) => {
            let mut _i = 0;
            $(
                assert_eq!($suite.tests[_i].desc.name, $test_name);
                _i = _i + 1;
            )+
            assert_eq!($suite.tests.len(), _i);
            assert_eq!($expected_suite_name, $suite.desc.name);
        };

        (
            $suite:expr,
            suite => $expected_suite_name:expr,
            tests => [ ],
        ) => {
            assert_eq!($suite.tests.len(), 0);
            assert_eq!($expected_suite_name, $suite.desc.name);
        };
    }


    #[test]
    fn can_build_test_groups_from_empty_tree() {

        // Act
        let suites = RootSuite::from_decorated_components(
            Vec::<ComponentDecoration<crate::MockParameters>>::new()
        );

        // Act
        assert_has_suite!(
            suites[0],
            suite => "root",
            tests => [ ],
        );

        assert_eq!(0, suites[0].tests.len());
        assert_eq!(0, suites[0].bookends.len());
    }

    #[test]
    fn can_build_test_groups_from_root_level_only_test() {
        // Act
        let suites = RootSuite::from_decorated_components(
            vec![
                // Tests
                mock_test_app::test_a::test_def()
            ]
        );

        // Assert
        assert_has_suite!(
            suites[0],
            suite => "root",
            tests => [
                "integra8::components::tests::mock_test_app::test_a"
            ],
        );
    }


    #[test]
    fn should_return_test_in_the_order_they_are_defined() {
          // Arrange
        let suites = RootSuite::from_decorated_components(
            vec![
                // Tests
                mock_test_app::test_c::test_def(),
                mock_test_app::test_b::test_def(),
                mock_test_app::test_a::test_def(),
            ],
        );

        // Assert

        // Should have tests in the expected order
        assert_has_suite!(
            suites[0],
            suite => "root",
            tests => [
                "integra8::components::tests::mock_test_app::test_c",
                "integra8::components::tests::mock_test_app::test_b",
                "integra8::components::tests::mock_test_app::test_a"
            ],
        );

    }

    #[test]
    fn can_build_test_groups_from_single_level_test_suite() {
        // Act
        let suites = RootSuite::from_decorated_components(
            vec![
                // Tests
                mock_test_app::suite1::suite1_test_c::test_def(),
                mock_test_app::suite1::suite1_test_b::test_def(),
                mock_test_app::suite1::suite1_test_a::test_def(),

                // Suites
                mock_test_app::suite1::__suite_def(),
            ]
        );

        // Assert
        // Shouldn't have a suite, tests, or bookends in root
        assert_has_suite!(
            suites[0],
            suite => "root",
            tests => [ ],
        );

        // suite1
        assert_has_suite!(
            suites[1],
            suite => "suite1",
            tests => [
                "integra8::components::tests::mock_test_app::suite1::suite1_test_c",
                "integra8::components::tests::mock_test_app::suite1::suite1_test_b",
                "integra8::components::tests::mock_test_app::suite1::suite1_test_a"
            ],
        );
    }

    #[test]
    fn can_build_test_groups_from_two_root_level_test_suites() {

        // Act
        let suites = RootSuite::from_decorated_components(
            vec![
                // Tests
                mock_test_app::suite1::suite1_test_c::test_def(),
                mock_test_app::suite1::suite1_test_b::test_def(),
                mock_test_app::suite1::suite1_test_a::test_def(),

                mock_test_app::suite2::suite2_test_c::test_def(),
                mock_test_app::suite2::suite2_test_b::test_def(),
                mock_test_app::suite2::suite2_test_a::test_def(),

                // Suites
                mock_test_app::suite1::__suite_def(),
                mock_test_app::suite2::__suite_def(),
            ]
        );

        // Assert
        // Shouldn't have a suite, tests, or bookends in root
        assert_has_suite!(
            suites[0],
            suite => "root",
            tests => [ ],
        );

        // suite1
        assert_has_suite!(
            suites[1],
            suite => "suite1",
            tests => [
                "integra8::components::tests::mock_test_app::suite1::suite1_test_c",
                "integra8::components::tests::mock_test_app::suite1::suite1_test_b",
                "integra8::components::tests::mock_test_app::suite1::suite1_test_a"
            ],
        );

        // suite2
        assert_has_suite!(
            suites[2],
            suite => "suite2",
            tests => [
                "integra8::components::tests::mock_test_app::suite2::suite2_test_c",
                "integra8::components::tests::mock_test_app::suite2::suite2_test_b",
                "integra8::components::tests::mock_test_app::suite2::suite2_test_a"
            ],
        );
    }

    /*#[test]
    fn can_build_test_groups_from_test_suite_with_nested_suite() {

        // Act
        let root_suite = RootSuite::from_decorated_components(
             vec![
                // suite1
                mock_test_app::suite1::suite1_test_c::test_def(),
                mock_test_app::suite1::suite1_test_b::test_def(),
                mock_test_app::suite1::suite1_test_a::test_def(),

                // nested_suite_1a
                mock_test_app::suite1::nested_suite_1a::nested_suite_1a_test_d::test_def(),

                // Suites
                mock_test_app::suite1::__suite_def(),
                mock_test_app::suite1::nested_suite_1a::__suite_def(),
            ]
        );


        // Act
        // should have a single test group
        assert_eq!(2, root_suite.nested_suite.len());

        // nested suite 1a
        assert_has_tests!(
            root_suite.nested_suite[0].tests,
            "integra8::components::tests::mock_test_app::suite1::nested_suite_1a::nested_suite_1a_test_d",
        );
        assert_has_suite! (
            groups[0].suite,
            "nested_suite_1a"
        );
        assert_eq!(root_suite.nested_suite[0].bookends.is_empty(), true);

        // Suite 1
        assert_has_tests!(
            groups[1].tests,
            "integra8::components::tests::mock_test_app::suite1::suite1_test_c",
            "integra8::components::tests::mock_test_app::suite1::suite1_test_b",
            "integra8::components::tests::mock_test_app::suite1::suite1_test_a",
        );
        assert_has_suite! (
            groups[1].suite,
            "suite1"
        );
        assert_eq!(groups[1].bookends.is_empty(), true);

    }

    #[test]
    fn can_build_test_groups_from_test_suite_with_nested_mod() {


        // Assert
        let sut = TestHierarchyTree::from_components(
            vec![
                // Tests
                mock_test_app::suite1::suite1_test_c::test_def(),
                mock_test_app::suite1::suite1_test_b::test_def(),
                mock_test_app::suite1::suite1_test_a::test_def(),
                mock_test_app::suite1::nested_mod::suite1_nested_mod_test_d::test_def(),

                // Suites
                mock_test_app::suite1::__suite_def(),
            ]
        );

        // Arrange
        let mut groups = sut.into_iter().collect::<Vec<TestExecutionGroup<crate::MockParameters>>>();

        // Act
        // should have a single test group
        assert_eq!(1, groups.len());

        // Suite 1
        assert_has_tests!(
            groups[0].tests,
            "integra8::components::tests::mock_test_app::suite1::suite1_test_c",
            "integra8::components::tests::mock_test_app::suite1::suite1_test_b",
            "integra8::components::tests::mock_test_app::suite1::suite1_test_a",
            "integra8::components::tests::mock_test_app::suite1::nested_mod::suite1_nested_mod_test_d",
        );
        assert_has_suite! (
            groups[0].suite,
            "suite1"
        );
        assert_eq!(groups[0].bookends.is_empty(), true);

    }
    */
}
*/
