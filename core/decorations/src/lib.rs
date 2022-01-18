mod test;
pub use test::{TestAttributesDecoration, TestDecoration};

mod bookends;
pub use bookends::{BookEndAttributesDecoration, BookEndDecoration};

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


// Test rigging to replicate what main_test!() does, to allow decorations to to be used in unit tests.
// must be in root!
#[cfg(test)]
use test_rigging::*;

#[doc(hidden)]
#[cfg(test)]
mod test_rigging {

    //type ExecutionContext  = crate::runner::context::ExecutionContext<Parameters>;

    #[linkme::distributed_slice]
    pub static REGISTERED_COMPONENTS: [fn() -> crate::ComponentDecoration<Parameters>] = [..];

    #[derive(Clone)]
    pub struct TestAppParameters {
        pub max_concurrency: usize,
        pub setup_critical_threshold_seconds: u64,
        pub test_critical_threshold_seconds: u64,
        pub test_warn_threshold_seconds: u64,
        pub tear_down_critical_threshold_seconds: u64,
        pub test_concurrency: components::ConcurrencyMode,
        pub suite_concurrency: components::ConcurrencyMode
    }

    impl TestAppParameters {
        pub fn default() -> Self {
            Self {
                max_concurrency: 10,
                setup_critical_threshold_seconds: 20,
                test_critical_threshold_seconds: 30,
                test_warn_threshold_seconds: 40,
                tear_down_critical_threshold_seconds: 50,
                test_concurrency: components::ConcurrencyMode::Parallel,
                suite_concurrency: components::ConcurrencyMode::Serial,       
            }
        }
    }

    impl components::TestParameters for TestAppParameters {
        fn child_process_target(&self) -> Option<&'_ str> {
            None // not needed for tests
        }

        fn use_child_processes(&self) -> bool {
            true // not needed for tests
        }

        fn max_concurrency(&self) -> usize {
            self.max_concurrency
        }

        fn test_concurrency(&self) -> components::ConcurrencyMode {
            self.test_concurrency.clone()
        }

        fn suite_concurrency(&self) -> components::ConcurrencyMode {
            self.suite_concurrency.clone()
        }

        fn setup_critical_threshold_seconds(&self) -> u64 {
            self.setup_critical_threshold_seconds
        }
        fn tear_down_critical_threshold_seconds(&self) -> u64 {
            self.tear_down_critical_threshold_seconds
        }

        fn test_critical_threshold_seconds(&self) -> u64 {
            self.test_critical_threshold_seconds
        }

        fn test_warn_threshold_seconds(&self) -> u64 {
            self.test_warn_threshold_seconds
        }

        // Find somewhere else for this
        fn root_namespace(&self) -> &'static str {
            "integra8_decorations"
        }

        // Consider refactoring this to ::formatters::FormatterParameters
        // to de-clutter this object
        fn console_output_style(&self) -> &'_ str {
            ""
        }
        fn console_output_detail_level(&self) -> &'_ str  {
            ""
        }
        fn console_output_encoding(&self) -> &'_ str  {
            ""
        }
        fn console_output_ansi_mode(&self) -> &'_ str {
            ""
        }
    }

    pub type Parameters = TestAppParameters;

    pub mod linkme {
        pub use linkme::*;
    }

    pub mod components {
        pub use integra8_components::*;
    }

    pub mod decorations {
        pub use crate::*;
    }


}
#[cfg(test)]
mod tests {
    use super::*;
    use super::components::ConcurrencyMode;

    mod mock_app {
        
        pub use integra8_decorations_impl::*;
    
        #[integration_test]
        #[integra8(crate = crate)]
        pub fn test_a() { }

        #[integration_test]
        #[integra8(crate = crate)]
        #[name("Test A")]
        #[description("the description of this test A")]
        #[critical_threshold_seconds(2)]
        #[warn_threshold_milliseconds(1000)]
        #[sequential]
        #[ignore()]
        #[allow_fail()]
        pub fn test_a_with_decorations() { }

        #[setup]
        #[integra8(crate = crate)]
        pub fn setup_a() { }

        #[setup]
        #[integra8(crate = crate)]
        #[name("Setup A")]
        #[description("the description of this setup A")]
        #[critical_threshold_seconds(2)]
        #[ignore()]
        pub fn setup_a_with_decorations() { }
    }


    #[macro_export]
    macro_rules! assert_is_root {
        ($root:expr) => {
            assert_eq!($root.description.path().as_str(), "integra8_decorations");
            assert_eq!($root.description.relative_path(), "integra8_decorations", );
            assert_eq!($root.description.full_name(), "integra8_decorations", );
            assert_eq!($root.description.friendly_name(), "integra8_decorations", );
            assert_eq!($root.description.is_root(), true);
            assert_eq!($root.description.id().as_unique_number(), 0);
            assert_eq!($root.description.id(), $root.description.parent_id());
            assert_eq!($root.description.description(), None);
            assert_eq!($root.description.component_type(), &ComponentType::Suite);
        }
    }

    #[test]
    fn components_from_no_decorations() {

        // Act
        let root = ComponentGroup::into_components(
            vec![],
            &Parameters::default()
        );

        // Assert
        assert_eq!(root.tests.len(), 0);
        assert_eq!(root.setups.len(), 0);
        assert_eq!(root.tear_downs.len(), 0);
        assert_eq!(root.suites.len(), 0);
        assert_is_root!(root);

        // Assert attributes/description was inherited from the Parameters
        assert_eq!(root.attributes.ignore, false);
        assert_eq!(root.attributes.allow_suite_fail, false);
        assert_eq!(root.attributes.setup_critical_threshold.as_secs(), 20);
        assert_eq!(root.attributes.test_critical_threshold.as_secs(), 30);
        assert_eq!(root.attributes.test_warn_threshold.as_secs(), 40);
        assert_eq!(root.attributes.tear_down_critical_threshold.as_secs(), 50);
        assert_eq!(root.attributes.suite_concurrency_mode, ConcurrencyMode::Serial);
        assert_eq!(root.attributes.test_concurrency_mode, ConcurrencyMode::Parallel);
    }

    #[test]
    fn components_from_single_test() {

        // Act
        let root = ComponentGroup::into_components(
            vec![mock_app::test_a::test_def()],
            &Parameters::default()
        );

        // Assert
        assert_eq!(root.tests.len(), 1);
        assert_eq!(root.setups.len(), 0);
        assert_eq!(root.tear_downs.len(), 0);
        assert_eq!(root.suites.len(), 0);
        assert_is_root!(root);

        // Assert attributes/description was inherited from the Parameters
        let test1 = &root.tests[0];
        assert_eq!(test1.description.path().as_str(), "integra8_decorations::tests::mock_app::test_a", );
        assert_eq!(test1.description.relative_path(), "tests::mock_app::test_a", );
        assert_eq!(test1.description.full_name(), "integra8_decorations::tests::mock_app::test_a", );
        assert_eq!(test1.description.friendly_name(), "tests::mock_app::test_a", );
        assert_eq!(test1.description.id().as_unique_number(), 1 );
        assert_eq!(test1.description.parent_id().as_unique_number(), 0);
        assert_eq!(test1.description.description(), None);
        assert_eq!(test1.description.component_type(), &ComponentType::Test);
        assert_eq!(test1.attributes.allow_fail, false);
        assert_eq!(test1.attributes.ignore, false);
        assert_eq!(test1.attributes.critical_threshold.as_secs(), 30);
        assert_eq!(test1.attributes.warn_threshold.as_secs(), 40);
        assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Parallel);

    }

    #[test]
    fn test_decorations_should_override_parameters() {
        // Act
        let root = ComponentGroup::into_components(
            vec![mock_app::test_a_with_decorations::test_def()],
            &Parameters::default()
        );

        // Assert
        assert_eq!(root.tests.len(), 1);
        assert_eq!(root.setups.len(), 0);
        assert_eq!(root.tear_downs.len(), 0);
        assert_eq!(root.suites.len(), 0);
        assert_is_root!(root);

        // Assert attributes were inherited from the Parameters
        let test1 = &root.tests[0];

        assert_eq!(test1.description.path().as_str(), "integra8_decorations::tests::mock_app::test_a_with_decorations", );
        assert_eq!(test1.description.relative_path(), "tests::mock_app::test_a_with_decorations", );
        assert_eq!(test1.description.full_name(), "Test A", );
        assert_eq!(test1.description.friendly_name(), "Test A", );
        assert_eq!(test1.description.id().as_unique_number(), 1 );
        assert_eq!(test1.description.parent_id().as_unique_number(), 0);
        assert_eq!(test1.description.description(), Some("the description of this test A"));
        assert_eq!(test1.description.component_type(), &ComponentType::Test);
        assert_eq!(test1.attributes.allow_fail, true);
        assert_eq!(test1.attributes.ignore, true);
        assert_eq!(test1.attributes.critical_threshold.as_secs(), 2);
        assert_eq!(test1.attributes.warn_threshold.as_secs(), 1);
        assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Serial);
    }

    #[test]
    fn components_from_single_setup() {

        // Act
        let root = ComponentGroup::into_components(
            vec![mock_app::setup_a::setup_def()],
            &Parameters::default()
        );

        // Assert
        assert_eq!(root.tests.len(), 0);
        assert_eq!(root.setups.len(), 1);
        assert_eq!(root.tear_downs.len(), 0);
        assert_eq!(root.suites.len(), 0);
        assert_is_root!(root);

        // Assert attributes/description was inherited from the Parameters
        let setup1 = &root.setups[0].clone();
        assert_eq!(setup1.description.path().as_str(), "integra8_decorations::tests::mock_app::setup_a", );
        assert_eq!(setup1.description.relative_path(), "tests::mock_app::setup_a", );
        assert_eq!(setup1.description.full_name(), "integra8_decorations::tests::mock_app::setup_a", );
        assert_eq!(setup1.description.friendly_name(), "tests::mock_app::setup_a", );
        assert_eq!(setup1.description.id().as_unique_number(), 1 );
        assert_eq!(setup1.description.parent_id().as_unique_number(), 0);
        assert_eq!(setup1.description.description(), None);
        assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
        assert_eq!(setup1.attributes.ignore, false);
        assert_eq!(setup1.attributes.critical_threshold.as_secs(), 20);
    }

    #[test]
    fn setup_decorations_should_override_parameters() {

        // Act
        let root = ComponentGroup::into_components(
            vec![mock_app::setup_a_with_decorations::setup_def()],
            &Parameters::default()
        );

        // Assert
        assert_eq!(root.tests.len(), 0);
        assert_eq!(root.setups.len(), 1);
        assert_eq!(root.tear_downs.len(), 0);
        assert_eq!(root.suites.len(), 0);
        assert_is_root!(root);

        // Assert attributes/description was inherited from the Parameters
        let setup1 = &root.setups[0].clone();
        assert_eq!(setup1.description.path().as_str(), "integra8_decorations::tests::mock_app::setup_a_with_decorations", );
        assert_eq!(setup1.description.relative_path(), "tests::mock_app::setup_a_with_decorations", );
        assert_eq!(setup1.description.full_name(), "Setup A", );
        assert_eq!(setup1.description.friendly_name(), "Setup A", );
        assert_eq!(setup1.description.id().as_unique_number(), 1 );
        assert_eq!(setup1.description.parent_id().as_unique_number(), 0);
        assert_eq!(setup1.description.description(), Some("the description of this setup A"));
        assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
        assert_eq!(setup1.attributes.ignore, true);
        assert_eq!(setup1.attributes.critical_threshold.as_secs(), 2);

    }
}

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
