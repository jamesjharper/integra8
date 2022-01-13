
mod test;
pub use test::{TestAttributesDecoration, TestDecoration};

mod bookends;
pub use bookends::{BookEndAttributesDecoration, BookEndDecoration, BookEndDecorationPair};

mod suite;
pub use suite::SuiteAttributesDecoration;

mod hierarchy;
pub use hierarchy::{ComponentGroup, ComponentHierarchy};

use integra8_components::ComponentType;

#[derive(Debug)]
pub enum ComponentDecoration<TParameters> {
    IntegrationTest(TestDecoration<TParameters>),
    Suite(SuiteAttributesDecoration),
    TearDown(BookEndDecoration<TParameters>),
    Setup(BookEndDecoration<TParameters>),
}

impl<TParameters> ComponentDecoration<TParameters> {
    pub fn path(&self) -> &'static str {
        match self {
            ComponentDecoration::IntegrationTest(c) => c.desc.path,
            ComponentDecoration::Suite(c) => c.path,
            ComponentDecoration::TearDown(c) => c.desc.path,
            ComponentDecoration::Setup(c) => c.desc.path,
        }
    }

    pub fn component_type(&self) -> ComponentType {
        match self {
            ComponentDecoration::IntegrationTest(_) => ComponentType::Test,
            ComponentDecoration::Suite(_) => ComponentType::Suite,
            ComponentDecoration::TearDown(_) => ComponentType::TearDown,
            ComponentDecoration::Setup(_) => ComponentType::Setup,
        }
    }
}



/*
#[cfg(test)]
mod tests {
    use super::*;


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
    */

    /*mod mock_test_app {

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
    }*/

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

    #[test]
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

}
    */
