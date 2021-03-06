mod test;
pub use test::{TestAttributesDecoration, TestDecoration};

mod bookends;
pub use bookends::{BookEndAttributesDecoration, BookEndDecoration};

mod suite;
pub use suite::SuiteAttributesDecoration;

mod hierarchy;
pub use hierarchy::{ComponentGroup, ComponentHierarchy};

use crate::components::{ComponentLocation, ComponentType, Delegate};

#[derive(Debug)]
pub enum ComponentDecoration<TParameters> {
    IntegrationTest(TestDecoration<TParameters>),
    Suite(SuiteAttributesDecoration),
    TearDown(BookEndDecoration<TParameters>),
    Setup(BookEndDecoration<TParameters>),
}

impl<TParameters> ComponentDecoration<TParameters> {
    pub fn name(&self) -> Option<&'static str> {
        match self {
            ComponentDecoration::IntegrationTest(c) => c.desc.name.clone(),
            ComponentDecoration::Suite(c) => c.name.clone(),
            ComponentDecoration::TearDown(c) => c.desc.name.clone(),
            ComponentDecoration::Setup(c) => c.desc.name.clone(),
        }
    }

    pub fn description(&self) -> Option<&'static str> {
        match self {
            ComponentDecoration::IntegrationTest(c) => c.desc.description.clone(),
            ComponentDecoration::Suite(c) => c.description.clone(),
            ComponentDecoration::TearDown(c) => c.desc.description.clone(),
            ComponentDecoration::Setup(c) => c.desc.description.clone(),
        }
    }

    pub fn location(&self) -> &'_ ComponentLocation {
        match self {
            ComponentDecoration::IntegrationTest(c) => &c.desc.location,
            ComponentDecoration::Suite(c) => &c.location,
            ComponentDecoration::TearDown(c) => &c.desc.location,
            ComponentDecoration::Setup(c) => &c.desc.location,
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

    pub fn into_delegate(self) -> Option<Delegate<TParameters>> {
        match self {
            ComponentDecoration::IntegrationTest(c) => Some(c.test_fn),
            ComponentDecoration::Suite(_) => None,
            ComponentDecoration::TearDown(c) => Some(c.bookend_fn),
            ComponentDecoration::Setup(c) => Some(c.bookend_fn),
        }
    }
}



#[doc(hidden)]
#[cfg(test)]
pub mod test_rigging {
    use std::time::Duration;

    #[linkme::distributed_slice]
    pub static REGISTERED_COMPONENTS: [fn() -> crate::decorations::ComponentDecoration<Parameters>] = [..];

    #[derive(Clone)]
    pub struct TestAppParameters {
        pub max_concurrency: usize,
        pub default_setup_time_limit: u64,
        pub test_time_limit_seconds: u64,
        pub test_warning_time_threshold_seconds: u64,
        pub tear_down_time_limit_seconds: u64,
        pub test_concurrency: crate::components::ConcurrencyMode,
        pub suite_concurrency: crate::components::ConcurrencyMode,
    }

    impl TestAppParameters {
        pub fn default() -> Self {
            Self {
                max_concurrency: 10,
                default_setup_time_limit: 20,
                test_time_limit_seconds: 30,
                test_warning_time_threshold_seconds: 40,
                tear_down_time_limit_seconds: 50,
                test_concurrency: crate::components::ConcurrencyMode::Parallel,
                suite_concurrency: crate::components::ConcurrencyMode::Sequential,
            }
        }
    }

    impl crate::components::TestParameters for TestAppParameters {
        fn child_process_target(&self) -> Option<&'_ crate::components::ChildProcessComponentArgs> {
            None // not needed for tests
        }

        fn use_child_processes(&self) -> bool {
            true // not needed for tests
        }

        fn max_concurrency(&self) -> usize {
            self.max_concurrency
        }

        fn test_concurrency(&self) -> crate::components::ConcurrencyMode {
            self.test_concurrency.clone()
        }

        fn suite_concurrency(&self) -> crate::components::ConcurrencyMode {
            self.suite_concurrency.clone()
        }

        fn setup_time_limit_duration(&self) -> Duration {
            Duration::from_secs(self.default_setup_time_limit)
        }

        fn tear_down_time_limit_duration(&self) -> Duration {
            Duration::from_secs(self.tear_down_time_limit_seconds)
        }

        fn test_time_limit_duration(&self) -> Duration {
            Duration::from_secs(self.test_time_limit_seconds)
        }

        fn test_warning_time_limit_duration(&self) -> Duration {
            Duration::from_secs(self.test_warning_time_threshold_seconds)
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
        fn console_output_detail_level(&self) -> &'_ str {
            ""
        }
        fn console_output_encoding(&self) -> &'_ str {
            ""
        }
        fn console_output_ansi_mode(&self) -> &'_ str {
            ""
        }
    }

    pub type Parameters = TestAppParameters;
}

#[cfg(test)]
mod tests {
    use crate::components::ConcurrencyMode;
    use super::*;

    mod mock_app {

        pub use integra8_decorations_impl::*;

        // Setups

        #[setup]
        // redirect integra8 namespace to decorations_impl so code gen works correctly
        #[integra8(crate = crate)]
        pub fn setup_a() {}

        #[setup]
        #[integra8(crate = crate)]
        #[name = "Setup A"]
        #[description = "the description of this setup A"]
        #[time_limit = "2s"]
        #[ignore]
        #[parallel]
        pub fn setup_a_with_decorations() {}

        #[setup]
        #[integra8(crate = crate)]
        pub fn setup_b() {}

        #[setup]
        #[integra8(crate = crate)]
        pub fn setup_c() {}

        // Tests

        #[integration_test]
        #[integra8(crate = crate)]
        pub fn test_a() {}

        #[integration_test]
        #[integra8(crate = crate)]
        #[name = "Test A"]
        #[description = "the description of this test A"]
        #[time_limit = "2s"]
        #[warning_time_limit = "1s"]
        #[sequential]
        #[ignore]
        #[allow_fail]
        pub fn test_a_with_decorations() {}

        #[integration_test]
        #[integra8(crate = crate)]
        pub fn test_b() {}

        #[integration_test]
        #[integra8(crate = crate)]
        pub fn test_c() {}

        // Tear downs

        #[teardown]
        #[integra8(crate = crate)]
        pub fn teardown_a() {}

        #[teardown]
        #[integra8(crate = crate)]
        #[name = "Teardown A"]
        #[description = "the description of this teardown A"]
        #[time_limit = "2s"]
        #[ignore]
        #[parallel]
        pub fn teardown_a_with_decorations() {}

        #[teardown]
        #[integra8(crate = crate)]
        pub fn teardown_b() {}

        #[teardown]
        #[integra8(crate = crate)]
        pub fn teardown_c() {}

        pub mod nested_namespace {

            pub use integra8_decorations_impl::*;

            #[integration_test]
            #[integra8(crate = crate)]
            #[name = "Test D"]
            #[description = "the description of this test D"]
            #[time_limit = "2s"]
            #[warning_time_limit = "1s"]
            #[sequential]
            #[ignore]
            #[allow_fail]
            pub fn test_d_nested_with_decorations() {}

            #[setup]
            #[integra8(crate = crate)]
            #[name = "Setup D"]
            #[description = "the description of this setup D"]
            #[time_limit = "2s"]
            #[ignore]
            #[parallel]
            pub fn setup_d_nested_with_decorations() {}

            #[teardown]
            #[integra8(crate = crate)]
            #[name = "Teardown D"]
            #[description = "the description of this teardown D"]
            #[time_limit = "2s"]
            #[ignore]
            #[parallel]
            pub fn teardown_d_nested_with_decorations() {}
        }

        #[suite]
        #[integra8(crate = crate)]
        pub mod nested_suite_z {

            pub use integra8_decorations_impl::*;

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn test_az() {}

            #[integration_test]
            #[integra8(crate = crate)]
            #[name = "Test Az"]
            #[description = "the description of this test Az"]
            #[time_limit = "2s"]
            #[warning_time_limit = "1000ms"]
            #[sequential]
            #[ignore]
            #[allow_fail]
            pub fn test_az_with_decorations() {}

            #[setup]
            #[integra8(crate = crate)]
            pub fn setup_az() {}

            #[setup]
            #[integra8(crate = crate)]
            #[name = "Setup Az"]
            #[description = "the description of this setup Az"]
            #[time_limit = "2s"]
            #[ignore]
            #[parallel]
            pub fn setup_az_with_decorations() {}

            #[teardown]
            #[integra8(crate = crate)]
            pub fn teardown_az() {}

            #[teardown]
            #[integra8(crate = crate)]
            #[name = "Teardown Az"]
            #[description = "the description of this teardown Az"]
            #[time_limit = "2s"]
            #[ignore]
            #[parallel]
            pub fn teardown_az_with_decorations() {}
        }

        #[suite]
        #[integra8(crate = crate)]
        #[test_parallel]
        #[parallel]
        #[allow_fail]
        #[ignore]
        #[name = "Nested Suite A"]
        #[description = "the description of this nested suite A"]
        #[tear_down_time_limit = "11s"]
        #[setup_time_limit = "12s"]
        #[test_time_limit = "13s"]
        #[test_warning_time_limit = "14s"]
        pub mod nested_suite_y {

            pub use integra8_decorations_impl::*;

            #[integration_test]
            #[integra8(crate = crate)]
            pub fn test_ay() {}

            #[integration_test]
            #[integra8(crate = crate)]
            #[name = "Test Ay"]
            #[description = "the description of this test Ay"]
            #[time_limit = "2s"]
            #[warning_time_limit = "1000ms"]
            #[sequential]
            #[ignore]
            #[allow_fail]
            pub fn test_ay_with_decorations() {}

            #[setup]
            #[integra8(crate = crate)]
            pub fn setup_ay() {}

            #[setup]
            #[integra8(crate = crate)]
            #[name = "Setup Ay"]
            #[description = "the description of this setup Ay"]
            #[time_limit = "2s"]
            #[ignore]
            #[parallel]
            pub fn setup_ay_with_decorations() {}

            #[teardown]
            #[integra8(crate = crate)]
            pub fn teardown_ay() {}

            #[teardown]
            #[integra8(crate = crate)]
            #[name = "Teardown Ay"]
            #[description = "the description of this teardown Ay"]
            #[time_limit = "2s"]
            #[ignore]
            #[parallel]
            pub fn teardown_ay_with_decorations() {}

            #[suite]
            #[integra8(crate = crate)]
            pub mod nested_suite_x {

                pub use integra8_decorations_impl::*;

                #[integration_test]
                #[integra8(crate = crate)]
                pub fn test_ax() {}

                #[setup]
                #[integra8(crate = crate)]
                pub fn setup_ax() {}

                #[teardown]
                #[integra8(crate = crate)]
                pub fn teardown_ax() {}
            }
        }
    }

    #[macro_export]
    macro_rules! assert_is_root {
        ($root:expr) => {
            assert_eq!($root.description.path().as_str(), "integra8_decorations");
            assert_eq!($root.description.relative_path(), "integra8_decorations",);
            assert_eq!($root.description.full_name(), "integra8_decorations",);
            assert_eq!($root.description.friendly_name(), "integra8_decorations",);
            assert_eq!($root.description.is_root(), true);
            assert_eq!($root.description.id().as_unique_number(), 0);
            assert_eq!($root.description.id(), $root.description.parent_id());
            assert_eq!($root.description.description(), None);
            assert_eq!($root.description.component_type(), &ComponentType::Suite);
        };
    }

    #[test]
    fn should_initialize_from_no_components() {
        // Act
        let root = ComponentGroup::into_root_component(vec![], &crate::Parameters::default());

        // Assert
        assert_eq!(root.tests.len(), 0);
        assert_eq!(root.setups.len(), 0);
        assert_eq!(root.tear_downs.len(), 0);
        assert_eq!(root.suites.len(), 0);
        assert_is_root!(root);

        // Assert attributes/description was inherited from the Parameters
        assert_eq!(root.attributes.ignore, false);
        assert_eq!(root.attributes.allow_suite_fail, false);
        assert_eq!(root.attributes.setup_time_limit.as_secs(), 20);
        assert_eq!(root.attributes.test_time_limit.as_secs(), 30);
        assert_eq!(root.attributes.test_warning_time_limit.as_secs(), 40);
        assert_eq!(root.attributes.tear_down_time_limit.as_secs(), 50);
        assert_eq!(
            root.attributes.suite_concurrency_mode,
            ConcurrencyMode::Sequential
        );
        assert_eq!(
            root.attributes.test_concurrency_mode,
            ConcurrencyMode::Parallel
        );
    }

    mod should_initialize_with_a_single_component {
        use super::*;
        use super::test_rigging::Parameters;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::test_a::test_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 1);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let test1 = &root.tests[0];
            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::test_a",
            );
            assert_eq!(test1.description.relative_path(), "integra8::decorations::tests::mock_app::test_a",);
            assert_eq!(
                test1.description.full_name(),
                "integra8::decorations::tests::mock_app::test_a",
            );
            assert_eq!(test1.description.friendly_name(), "integra8::decorations::tests::mock_app::test_a",);
            assert_eq!(test1.description.id().as_unique_number(), 1);
            assert_eq!(test1.description.parent_id().as_unique_number(), 0);
            assert_eq!(test1.description.description(), None);
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, false);
            assert_eq!(test1.attributes.ignore, false);
            assert_eq!(test1.attributes.time_limit.as_secs(), 30);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 40);
            assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::setup_a::setup_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 1);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.setups[0].clone();
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::setup_a",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "integra8::decorations::tests::mock_app::setup_a",
            );
            assert_eq!(
                setup1.description.full_name(),
                "integra8::decorations::tests::mock_app::setup_a",
            );
            assert_eq!(
                setup1.description.friendly_name(),
                "integra8::decorations::tests::mock_app::setup_a",
            );
            assert_eq!(setup1.description.id().as_unique_number(), 1);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 0);
            assert_eq!(setup1.description.description(), None);
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, false);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 20);
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::teardown_a::teardown_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 1);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.tear_downs[0].clone();
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::teardown_a",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "integra8::decorations::tests::mock_app::teardown_a",
            );
            assert_eq!(
                teardown1.description.full_name(),
                "integra8::decorations::tests::mock_app::teardown_a",
            );
            assert_eq!(
                teardown1.description.friendly_name(),
                "integra8::decorations::tests::mock_app::teardown_a",
            );
            assert_eq!(teardown1.description.id().as_unique_number(), 1);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 0);
            assert_eq!(teardown1.description.description(), None);
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, false);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 50);
        }

        #[test]
        fn for_suite() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::nested_suite_z::__suite_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let suite1 = &root.suites[0];
            assert_eq!(
                suite1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_z",
            );
            assert_eq!(
                suite1.description.relative_path(),
                "integra8::decorations::tests::mock_app::nested_suite_z",
            );
            assert_eq!(
                suite1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_z",
            );
            assert_eq!(
                suite1.description.friendly_name(),
                "integra8::decorations::tests::mock_app::nested_suite_z",
            );
            assert_eq!(suite1.description.id().as_unique_number(), 1);
            assert_eq!(suite1.description.parent_id().as_unique_number(), 0);
            assert_eq!(suite1.description.description(), None);
            assert_eq!(suite1.description.component_type(), &ComponentType::Suite);
            assert_eq!(suite1.attributes.ignore, false);
            assert_eq!(suite1.attributes.allow_suite_fail, false);
            assert_eq!(suite1.attributes.test_time_limit.as_secs(), 30);
            assert_eq!(suite1.attributes.test_warning_time_limit.as_secs(), 40);
            assert_eq!(suite1.attributes.setup_time_limit.as_secs(), 20);
            assert_eq!(suite1.attributes.tear_down_time_limit.as_secs(), 50);
            assert_eq!(
                suite1.attributes.suite_concurrency_mode,
                ConcurrencyMode::Sequential
            );
            assert_eq!(
                suite1.attributes.test_concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }
    }

    mod should_initialize_with_a_single_component_in_nested_suite {

        use super::*;
        use super::test_rigging::Parameters;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_z::__suite_def(),
                    mock_app::nested_suite_z::test_az::test_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let test1 = &root.suites[0].tests[0];
            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_z::test_az",
            );
            assert_eq!(test1.description.relative_path(), "test_az",);
            assert_eq!(
                test1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_z::test_az",
            );
            assert_eq!(test1.description.friendly_name(), "test_az",);
            assert_eq!(test1.description.id().as_unique_number(), 2);
            assert_eq!(test1.description.parent_id().as_unique_number(), 1);
            assert_eq!(test1.description.description(), None);
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, false);
            assert_eq!(test1.attributes.ignore, false);
            assert_eq!(test1.attributes.time_limit.as_secs(), 30);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 40);
            assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_z::__suite_def(),
                    mock_app::nested_suite_z::setup_az::setup_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.suites[0].setups[0];
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_z::setup_az",
            );
            assert_eq!(setup1.description.relative_path(), "setup_az",);
            assert_eq!(
                setup1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_z::setup_az",
            );
            assert_eq!(setup1.description.friendly_name(), "setup_az",);
            assert_eq!(setup1.description.id().as_unique_number(), 2);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 1);
            assert_eq!(setup1.description.description(), None);
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, false);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 20);
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_z::__suite_def(),
                    mock_app::nested_suite_z::teardown_az::teardown_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.suites[0].tear_downs[0];
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_z::teardown_az",
            );
            assert_eq!(teardown1.description.relative_path(), "teardown_az",);
            assert_eq!(
                teardown1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_z::teardown_az",
            );
            assert_eq!(teardown1.description.friendly_name(), "teardown_az",);
            assert_eq!(teardown1.description.id().as_unique_number(), 2);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 1);
            assert_eq!(teardown1.description.description(), None);
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, false);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 50);
        }
    }

    mod should_override_parameters {
        use super::*;
        use super::test_rigging::Parameters;
        
        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::test_a_with_decorations::test_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 1);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes were inherited from the Parameters
            let test1 = &root.tests[0];

            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::test_a_with_decorations",
            );
            assert_eq!(
                test1.description.relative_path(),
                "integra8::decorations::tests::mock_app::test_a_with_decorations",
            );
            assert_eq!(test1.description.full_name(), "Test A",);
            assert_eq!(test1.description.friendly_name(), "Test A",);
            assert_eq!(test1.description.id().as_unique_number(), 1);
            assert_eq!(test1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                test1.description.description(),
                Some("the description of this test A")
            );
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, true);
            assert_eq!(test1.attributes.ignore, true);
            assert_eq!(test1.attributes.time_limit.as_secs(), 2);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 1);
            assert_eq!(
                test1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::setup_a_with_decorations::setup_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 1);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.setups[0].clone();
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::setup_a_with_decorations",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "integra8::decorations::tests::mock_app::setup_a_with_decorations",
            );
            assert_eq!(setup1.description.full_name(), "Setup A",);
            assert_eq!(setup1.description.friendly_name(), "Setup A",);
            assert_eq!(setup1.description.id().as_unique_number(), 1);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                setup1.description.description(),
                Some("the description of this setup A")
            );
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, true);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                setup1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::teardown_a_with_decorations::teardown_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 1);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.tear_downs[0].clone();
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::teardown_a_with_decorations",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "integra8::decorations::tests::mock_app::teardown_a_with_decorations",
            );
            assert_eq!(teardown1.description.full_name(), "Teardown A",);
            assert_eq!(teardown1.description.friendly_name(), "Teardown A",);
            assert_eq!(teardown1.description.id().as_unique_number(), 1);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                teardown1.description.description(),
                Some("the description of this teardown A")
            );
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                teardown1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }

        #[test]
        fn for_suite() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::nested_suite_y::__suite_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let suite1 = &root.suites[0];
            assert_eq!(
                suite1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y",
            );
            assert_eq!(
                suite1.description.relative_path(),
                "integra8::decorations::tests::mock_app::nested_suite_y",
            );
            assert_eq!(suite1.description.full_name(), "Nested Suite A",);
            assert_eq!(suite1.description.friendly_name(), "Nested Suite A",);
            assert_eq!(suite1.description.id().as_unique_number(), 1);
            assert_eq!(suite1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                suite1.description.description(),
                Some("the description of this nested suite A")
            );
            assert_eq!(suite1.description.component_type(), &ComponentType::Suite);
            assert_eq!(suite1.attributes.ignore, true);
            assert_eq!(suite1.attributes.allow_suite_fail, true);
            assert_eq!(suite1.attributes.test_time_limit.as_secs(), 13);
            assert_eq!(suite1.attributes.test_warning_time_limit.as_secs(), 14);
            assert_eq!(suite1.attributes.setup_time_limit.as_secs(), 12);
            assert_eq!(suite1.attributes.tear_down_time_limit.as_secs(), 11);
            assert_eq!(
                suite1.attributes.suite_concurrency_mode,
                ConcurrencyMode::Parallel
            );
            assert_eq!(
                suite1.attributes.test_concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }
    }

    mod should_override_parameters_when_nested {
        use super::*;
        use super::test_rigging::Parameters;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::nested_namespace::test_d_nested_with_decorations::test_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 1);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes were inherited from the Parameters
            let test1 = &root.tests[0];

            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_namespace::test_d_nested_with_decorations",
            );
            assert_eq!(
                test1.description.relative_path(),
                "integra8::decorations::tests::mock_app::nested_namespace::test_d_nested_with_decorations",
            );
            assert_eq!(test1.description.full_name(), "Test D",);
            assert_eq!(test1.description.friendly_name(), "Test D",);
            assert_eq!(test1.description.id().as_unique_number(), 1);
            assert_eq!(test1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                test1.description.description(),
                Some("the description of this test D")
            );
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, true);
            assert_eq!(test1.attributes.ignore, true);
            assert_eq!(test1.attributes.time_limit.as_secs(), 2);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 1);
            assert_eq!(
                test1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![mock_app::nested_namespace::setup_d_nested_with_decorations::setup_def()],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 1);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.setups[0].clone();
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_namespace::setup_d_nested_with_decorations",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "integra8::decorations::tests::mock_app::nested_namespace::setup_d_nested_with_decorations",
            );
            assert_eq!(setup1.description.full_name(), "Setup D",);
            assert_eq!(setup1.description.friendly_name(), "Setup D",);
            assert_eq!(setup1.description.id().as_unique_number(), 1);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                setup1.description.description(),
                Some("the description of this setup D")
            );
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, true);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                setup1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_namespace::teardown_d_nested_with_decorations::teardown_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 1);
            assert_eq!(root.suites.len(), 0);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.tear_downs[0].clone();
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_namespace::teardown_d_nested_with_decorations",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "integra8::decorations::tests::mock_app::nested_namespace::teardown_d_nested_with_decorations",
            );
            assert_eq!(teardown1.description.full_name(), "Teardown D",);
            assert_eq!(teardown1.description.friendly_name(), "Teardown D",);
            assert_eq!(teardown1.description.id().as_unique_number(), 1);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                teardown1.description.description(),
                Some("the description of this teardown D")
            );
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                teardown1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }
    }

    mod should_override_parameters_when_nested_in_another_suite {
        use super::*;
        use super::test_rigging::Parameters;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_z::__suite_def(),
                    mock_app::nested_suite_z::test_az_with_decorations::test_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes were inherited from the Parameters
            let test1 = &root.suites[0].tests[0];

            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_z::test_az_with_decorations",
            );
            assert_eq!(
                test1.description.relative_path(),
                "test_az_with_decorations",
            );
            assert_eq!(test1.description.full_name(), "Test Az",);
            assert_eq!(test1.description.friendly_name(), "Test Az",);
            assert_eq!(test1.description.id().as_unique_number(), 2);
            assert_eq!(test1.description.parent_id().as_unique_number(), 1);
            assert_eq!(
                test1.description.description(),
                Some("the description of this test Az")
            );
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, true);
            assert_eq!(test1.attributes.ignore, true);
            assert_eq!(test1.attributes.time_limit.as_secs(), 2);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 1);
            assert_eq!(
                test1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_z::__suite_def(),
                    mock_app::nested_suite_z::setup_az_with_decorations::setup_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.suites[0].setups[0];
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_z::setup_az_with_decorations",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "setup_az_with_decorations",
            );
            assert_eq!(setup1.description.full_name(), "Setup Az",);
            assert_eq!(setup1.description.friendly_name(), "Setup Az",);
            assert_eq!(setup1.description.id().as_unique_number(), 2);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 1);

            assert_eq!(
                setup1.description.description(),
                Some("the description of this setup Az")
            );
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, true);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                setup1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_z::__suite_def(),
                    mock_app::nested_suite_z::teardown_az_with_decorations::teardown_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.suites[0].tear_downs[0];
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_z::teardown_az_with_decorations",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "teardown_az_with_decorations",
            );
            assert_eq!(teardown1.description.full_name(), "Teardown Az",);
            assert_eq!(teardown1.description.friendly_name(), "Teardown Az",);
            assert_eq!(teardown1.description.id().as_unique_number(), 2);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 1);
            assert_eq!(
                teardown1.description.description(),
                Some("the description of this teardown Az")
            );
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                teardown1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }
    }

    mod should_override_parameter_defaults_set_on_nested_suite {
        use super::*;
        use super::test_rigging::Parameters;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::test_ay_with_decorations::test_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes were inherited from the Parameters
            let test1 = &root.suites[0].tests[0];

            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::test_ay_with_decorations",
            );
            assert_eq!(
                test1.description.relative_path(),
                "test_ay_with_decorations",
            );
            assert_eq!(test1.description.full_name(), "Test Ay",);
            assert_eq!(test1.description.friendly_name(), "Test Ay",);
            assert_eq!(test1.description.id().as_unique_number(), 2);
            assert_eq!(test1.description.parent_id().as_unique_number(), 1);
            assert_eq!(
                test1.description.description(),
                Some("the description of this test Ay")
            );
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, true);
            assert_eq!(test1.attributes.ignore, true);
            assert_eq!(test1.attributes.time_limit.as_secs(), 2);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 1);
            assert_eq!(
                test1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::setup_ay_with_decorations::setup_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.suites[0].setups[0];
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::setup_ay_with_decorations",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "setup_ay_with_decorations",
            );
            assert_eq!(setup1.description.full_name(), "Setup Ay",);
            assert_eq!(setup1.description.friendly_name(), "Setup Ay",);
            assert_eq!(setup1.description.id().as_unique_number(), 2);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 1);

            assert_eq!(
                setup1.description.description(),
                Some("the description of this setup Ay")
            );
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, true);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                setup1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::teardown_ay_with_decorations::teardown_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.suites[0].tear_downs[0];
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::teardown_ay_with_decorations",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "teardown_ay_with_decorations",
            );
            assert_eq!(teardown1.description.full_name(), "Teardown Ay",);
            assert_eq!(teardown1.description.friendly_name(), "Teardown Ay",);
            assert_eq!(teardown1.description.id().as_unique_number(), 2);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 1);
            assert_eq!(
                teardown1.description.description(),
                Some("the description of this teardown Ay")
            );
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 2);
            assert_eq!(
                teardown1.attributes.concurrency_mode,
                ConcurrencyMode::Parallel
            );
        }
    }

    mod should_inherit_parameter_defaults_set_on_nested_suite {
        use super::*;
        use super::test_rigging::Parameters;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::test_ay::test_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes were inherited from the Parameters
            let test1 = &root.suites[0].tests[0];

            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::test_ay",
            );
            assert_eq!(test1.description.relative_path(), "test_ay");
            assert_eq!(
                test1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_y::test_ay",
            );
            assert_eq!(test1.description.friendly_name(), "test_ay",);
            assert_eq!(test1.description.id().as_unique_number(), 2);
            assert_eq!(test1.description.parent_id().as_unique_number(), 1);
            assert_eq!(test1.description.description(), None);
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, false);
            assert_eq!(test1.attributes.ignore, true);
            assert_eq!(test1.attributes.time_limit.as_secs(), 13);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 14);
            assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::setup_ay::setup_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.suites[0].setups[0];
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::setup_ay",
            );
            assert_eq!(setup1.description.relative_path(), "setup_ay",);
            assert_eq!(
                setup1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_y::setup_ay",
            );
            assert_eq!(setup1.description.friendly_name(), "setup_ay",);
            assert_eq!(setup1.description.id().as_unique_number(), 2);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 1);

            assert_eq!(setup1.description.description(), None);
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, true);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 12);
            assert_eq!(
                setup1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::teardown_ay::teardown_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.suites[0].tear_downs[0];
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::teardown_ay",
            );
            assert_eq!(teardown1.description.relative_path(), "teardown_ay",);
            assert_eq!(
                teardown1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_y::teardown_ay"
            );
            assert_eq!(teardown1.description.friendly_name(), "teardown_ay",);
            assert_eq!(teardown1.description.id().as_unique_number(), 2);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 1);
            assert_eq!(teardown1.description.description(), None);
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 11);
            assert_eq!(
                teardown1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }
    }

    mod should_inherit_parameter_defaults_set_on_nested_nested_suite {
        use super::*;
        use super::test_rigging::Parameters;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::nested_suite_x::__suite_def(),
                    mock_app::nested_suite_y::nested_suite_x::test_ax::test_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes were inherited from the Parameters
            let test1 = &root.suites[0].suites[0].tests[0];

            assert_eq!(
                test1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::nested_suite_x::test_ax",
            );
            assert_eq!(test1.description.relative_path(), "test_ax");
            assert_eq!(
                test1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_y::nested_suite_x::test_ax",
            );
            assert_eq!(test1.description.friendly_name(), "test_ax",);
            assert_eq!(test1.description.id().as_unique_number(), 3);
            assert_eq!(test1.description.parent_id().as_unique_number(), 2);
            assert_eq!(test1.description.description(), None);
            assert_eq!(test1.description.component_type(), &ComponentType::Test);
            assert_eq!(test1.attributes.allow_fail, false);
            assert_eq!(test1.attributes.ignore, true);
            assert_eq!(test1.attributes.time_limit.as_secs(), 13);
            assert_eq!(test1.attributes.warning_time_limit.as_secs(), 14);
            assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::nested_suite_x::__suite_def(),
                    mock_app::nested_suite_y::nested_suite_x::setup_ax::setup_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let setup1 = &root.suites[0].suites[0].setups[0];
            assert_eq!(
                setup1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::nested_suite_x::setup_ax",
            );
            assert_eq!(setup1.description.relative_path(), "setup_ax",);
            assert_eq!(
                setup1.description.full_name(),
                "integra8::decorations::tests::mock_app::nested_suite_y::nested_suite_x::setup_ax",
            );
            assert_eq!(setup1.description.friendly_name(), "setup_ax",);
            assert_eq!(setup1.description.id().as_unique_number(), 3);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 2);

            assert_eq!(setup1.description.description(), None);
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, true);
            assert_eq!(setup1.attributes.time_limit.as_secs(), 12);
            assert_eq!(
                setup1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_root_component(
                vec![
                    mock_app::nested_suite_y::__suite_def(),
                    mock_app::nested_suite_y::nested_suite_x::__suite_def(),
                    mock_app::nested_suite_y::nested_suite_x::teardown_ax::teardown_def(),
                ],
                &Parameters::default(),
            );

            // Assert
            assert_eq!(root.tests.len(), 0);
            assert_eq!(root.setups.len(), 0);
            assert_eq!(root.tear_downs.len(), 0);
            assert_eq!(root.suites.len(), 1);
            assert_is_root!(root);

            // Assert attributes/description was inherited from the Parameters
            let teardown1 = &root.suites[0].suites[0].tear_downs[0];
            assert_eq!(
                teardown1.description.path().as_str(),
                "integra8::decorations::tests::mock_app::nested_suite_y::nested_suite_x::teardown_ax"
            );
            assert_eq!(teardown1.description.relative_path(), "teardown_ax",);
            assert_eq!(teardown1.description.full_name(),  "integra8::decorations::tests::mock_app::nested_suite_y::nested_suite_x::teardown_ax");
            assert_eq!(teardown1.description.friendly_name(), "teardown_ax",);
            assert_eq!(teardown1.description.id().as_unique_number(), 3);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 2);
            assert_eq!(teardown1.description.description(), None);
            assert_eq!(
                teardown1.description.component_type(),
                &ComponentType::TearDown
            );
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.time_limit.as_secs(), 11);
            assert_eq!(
                teardown1.attributes.concurrency_mode,
                ConcurrencyMode::Sequential
            );
        }
    }

    #[test]
    fn should_return_components_in_the_order_they_are_defined() {
        // Act
        let root = ComponentGroup::into_root_component(
            vec![
                // linkme does not order components in the
                // order they appear in the file.
                // we have internally order by line count,
                // and lexicographically order between files
                mock_app::setup_c::setup_def(),
                mock_app::setup_b::setup_def(),
                mock_app::setup_a::setup_def(),
                mock_app::test_c::test_def(),
                mock_app::test_b::test_def(),
                mock_app::test_a::test_def(),
                mock_app::nested_suite_z::__suite_def(),
                mock_app::nested_suite_z::setup_az::setup_def(),
                mock_app::nested_suite_z::test_az::test_def(),
                mock_app::nested_suite_z::teardown_az::teardown_def(),
                mock_app::nested_suite_y::__suite_def(),
                mock_app::teardown_c::teardown_def(),
                mock_app::teardown_b::teardown_def(),
                mock_app::teardown_a::teardown_def(),
            ],
            &crate::Parameters::default(),
        );

        // Assert
        assert_eq!(root.tests.len(), 3);
        assert_eq!(root.setups.len(), 3);
        assert_eq!(root.tear_downs.len(), 3);
        assert_eq!(root.suites.len(), 2);

        // Setups

        let setup1 = &root.setups[0];
        assert_eq!(setup1.description.id().as_unique_number(), 1);
        assert_eq!(
            setup1.description.path().as_str(),
            "integra8::decorations::tests::mock_app::setup_a"
        );

        let setup2 = &root.setups[1];
        assert_eq!(setup2.description.id().as_unique_number(), 2);
        assert_eq!(
            setup2.description.path().as_str(),
            "integra8::decorations::tests::mock_app::setup_b"
        );

        let setup3 = &root.setups[2];
        assert_eq!(setup3.description.id().as_unique_number(), 3);
        assert_eq!(
            setup3.description.path().as_str(),
            "integra8::decorations::tests::mock_app::setup_c"
        );

        // Tests

        let test1 = &root.tests[0];
        assert_eq!(test1.description.id().as_unique_number(), 4);
        assert_eq!(
            test1.description.path().as_str(),
            "integra8::decorations::tests::mock_app::test_a"
        );

        let test2 = &root.tests[1];
        assert_eq!(test2.description.id().as_unique_number(), 5);
        assert_eq!(
            test2.description.path().as_str(),
            "integra8::decorations::tests::mock_app::test_b"
        );

        let test3 = &root.tests[2];
        assert_eq!(test3.description.id().as_unique_number(), 6);
        assert_eq!(
            test3.description.path().as_str(),
            "integra8::decorations::tests::mock_app::test_c"
        );

        // Nested Suite 1
        let suite1 = &root.suites[0];
        assert_eq!(suite1.description.id().as_unique_number(), 7);
        assert_eq!(
            suite1.description.path().as_str(),
            "integra8::decorations::tests::mock_app::nested_suite_z"
        );

        let setup11 = &suite1.setups[0];
        assert_eq!(setup11.description.id().as_unique_number(), 8);
        assert_eq!(
            setup11.description.path().as_str(),
            "integra8::decorations::tests::mock_app::nested_suite_z::setup_az"
        );

        let test11 = &suite1.tests[0];
        assert_eq!(test11.description.id().as_unique_number(), 9);
        assert_eq!(
            test11.description.path().as_str(),
            "integra8::decorations::tests::mock_app::nested_suite_z::test_az"
        );

        let teardown11 = &suite1.tear_downs[0];
        assert_eq!(teardown11.description.id().as_unique_number(), 10);
        assert_eq!(
            teardown11.description.path().as_str(),
            "integra8::decorations::tests::mock_app::nested_suite_z::teardown_az"
        );

        // Nested Suite 2
        let suite2 = &root.suites[1];
        assert_eq!(suite2.description.id().as_unique_number(), 11);
        assert_eq!(
            suite2.description.path().as_str(),
            "integra8::decorations::tests::mock_app::nested_suite_y"
        );

        // Tear downs

        let tear_down1 = &root.tear_downs[0];
        assert_eq!(tear_down1.description.id().as_unique_number(), 12);
        assert_eq!(
            tear_down1.description.path().as_str(),
            "integra8::decorations::tests::mock_app::teardown_a"
        );

        let tear_down2 = &root.tear_downs[1];
        assert_eq!(tear_down2.description.id().as_unique_number(), 13);
        assert_eq!(
            tear_down2.description.path().as_str(),
            "integra8::decorations::tests::mock_app::teardown_b"
        );

        let tear_down3 = &root.tear_downs[2];
        assert_eq!(tear_down3.description.id().as_unique_number(), 14);
        assert_eq!(
            tear_down3.description.path().as_str(),
            "integra8::decorations::tests::mock_app::teardown_c"
        );
    }
}
