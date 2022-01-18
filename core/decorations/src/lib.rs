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
        pub suite_concurrency: components::ConcurrencyMode,
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
    use super::components::ConcurrencyMode;
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
        #[name("Setup A")]
        #[description("the description of this setup A")]
        #[critical_threshold_seconds(2)]
        #[ignore()]
        #[parallelizable]
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
        #[name("Test A")]
        #[description("the description of this test A")]
        #[critical_threshold_seconds(2)]
        #[warn_threshold_milliseconds(1000)]
        #[sequential]
        #[ignore()]
        #[allow_fail()]
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
        #[name("Teardown A")]
        #[description("the description of this teardown A")]
        #[critical_threshold_seconds(2)]
        #[ignore()]
        #[parallelizable]
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
            #[name("Test D")]
            #[description("the description of this test D")]
            #[critical_threshold_seconds(2)]
            #[warn_threshold_milliseconds(1000)]
            #[sequential]
            #[ignore()]
            #[allow_fail()]
            pub fn test_d_nested_with_decorations() {}
    
            #[setup]
            #[integra8(crate = crate)]
            #[name("Setup D")]
            #[description("the description of this setup D")]
            #[critical_threshold_seconds(2)]
            #[ignore()]
            #[parallelizable]
            pub fn setup_d_nested_with_decorations() {}
    
            #[teardown]
            #[integra8(crate = crate)]
            #[name("Teardown D")]
            #[description("the description of this teardown D")]
            #[critical_threshold_seconds(2)]
            #[ignore()]
            #[parallelizable]
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
            #[name("Test Az")]
            #[description("the description of this test Az")]
            #[critical_threshold_seconds(2)]
            #[warn_threshold_milliseconds(1000)]
            #[sequential]
            #[ignore()]
            #[allow_fail()]
            pub fn test_az_with_decorations() {}
    
            #[setup]
            #[integra8(crate = crate)]
            pub fn setup_az() {}

            #[setup]
            #[integra8(crate = crate)]
            #[name("Setup Az")]
            #[description("the description of this setup Az")]
            #[critical_threshold_seconds(2)]
            #[ignore()]
            #[parallelizable]
            pub fn setup_az_with_decorations() {}
    
            #[teardown]
            #[integra8(crate = crate)]
            pub fn teardown_az() {}

            #[teardown]
            #[integra8(crate = crate)]
            #[name("Teardown Az")]
            #[description("the description of this teardown Az")]
            #[critical_threshold_seconds(2)]
            #[ignore()]
            #[parallelizable]
            pub fn teardown_az_with_decorations() {}
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
        let root = ComponentGroup::into_components(vec![], &Parameters::default());

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
        assert_eq!(
            root.attributes.suite_concurrency_mode,
            ConcurrencyMode::Serial
        );
        assert_eq!(
            root.attributes.test_concurrency_mode,
            ConcurrencyMode::Parallel
        );
    }

    mod should_initialize_with_a_single_component {
        use super::*;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::test_a",
            );
            assert_eq!(test1.description.relative_path(), "tests::mock_app::test_a",);
            assert_eq!(
                test1.description.full_name(),
                "integra8_decorations::tests::mock_app::test_a",
            );
            assert_eq!(test1.description.friendly_name(), "tests::mock_app::test_a",);
            assert_eq!(test1.description.id().as_unique_number(), 1);
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
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::setup_a",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "tests::mock_app::setup_a",
            );
            assert_eq!(
                setup1.description.full_name(),
                "integra8_decorations::tests::mock_app::setup_a",
            );
            assert_eq!(
                setup1.description.friendly_name(),
                "tests::mock_app::setup_a",
            );
            assert_eq!(setup1.description.id().as_unique_number(), 1);
            assert_eq!(setup1.description.parent_id().as_unique_number(), 0);
            assert_eq!(setup1.description.description(), None);
            assert_eq!(setup1.description.component_type(), &ComponentType::Setup);
            assert_eq!(setup1.attributes.ignore, false);
            assert_eq!(setup1.attributes.critical_threshold.as_secs(), 20);
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::teardown_a",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "tests::mock_app::teardown_a",
            );
            assert_eq!(
                teardown1.description.full_name(),
                "integra8_decorations::tests::mock_app::teardown_a",
            );
            assert_eq!(
                teardown1.description.friendly_name(),
                "tests::mock_app::teardown_a",
            );
            assert_eq!(teardown1.description.id().as_unique_number(), 1);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 0);
            assert_eq!(teardown1.description.description(), None);
            assert_eq!(teardown1.description.component_type(), &ComponentType::TearDown);
            assert_eq!(teardown1.attributes.ignore, false);
            assert_eq!(teardown1.attributes.critical_threshold.as_secs(), 50);
        }
    }

    mod should_override_parameters {
        use super::*;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::test_a_with_decorations",
            );
            assert_eq!(
                test1.description.relative_path(),
                "tests::mock_app::test_a_with_decorations",
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
            assert_eq!(test1.attributes.critical_threshold.as_secs(), 2);
            assert_eq!(test1.attributes.warn_threshold.as_secs(), 1);
            assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Serial);
        }
    
        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::setup_a_with_decorations",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "tests::mock_app::setup_a_with_decorations",
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
            assert_eq!(setup1.attributes.critical_threshold.as_secs(), 2);
            assert_eq!(setup1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }
    
        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::teardown_a_with_decorations",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "tests::mock_app::teardown_a_with_decorations",
            );
            assert_eq!(teardown1.description.full_name(), "Teardown A",);
            assert_eq!(teardown1.description.friendly_name(), "Teardown A",);
            assert_eq!(teardown1.description.id().as_unique_number(), 1);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                teardown1.description.description(),
                Some("the description of this teardown A")
            );
            assert_eq!(teardown1.description.component_type(), &ComponentType::TearDown);
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.critical_threshold.as_secs(), 2);
            assert_eq!(teardown1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }
    }


    mod should_override_parameters_even_when_nested {
        use super::*;

        #[test]
        fn for_test() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::nested_namespace::test_d_nested_with_decorations",
            );
            assert_eq!(
                test1.description.relative_path(),
                "tests::mock_app::nested_namespace::test_d_nested_with_decorations",
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
            assert_eq!(test1.attributes.critical_threshold.as_secs(), 2);
            assert_eq!(test1.attributes.warn_threshold.as_secs(), 1);
            assert_eq!(test1.attributes.concurrency_mode, ConcurrencyMode::Serial);
        }

        #[test]
        fn for_setup() {
            // Act
            let root = ComponentGroup::into_components(
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
                "integra8_decorations::tests::mock_app::nested_namespace::setup_d_nested_with_decorations",
            );
            assert_eq!(
                setup1.description.relative_path(),
                "tests::mock_app::nested_namespace::setup_d_nested_with_decorations",
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
            assert_eq!(setup1.attributes.critical_threshold.as_secs(), 2);
            assert_eq!(setup1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }

        #[test]
        fn for_tear_down() {
            // Act
            let root = ComponentGroup::into_components(
                vec![mock_app::nested_namespace::teardown_d_nested_with_decorations::teardown_def()],
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
                "integra8_decorations::tests::mock_app::nested_namespace::teardown_d_nested_with_decorations",
            );
            assert_eq!(
                teardown1.description.relative_path(),
                "tests::mock_app::nested_namespace::teardown_d_nested_with_decorations",
            );
            assert_eq!(teardown1.description.full_name(), "Teardown D",);
            assert_eq!(teardown1.description.friendly_name(), "Teardown D",);
            assert_eq!(teardown1.description.id().as_unique_number(), 1);
            assert_eq!(teardown1.description.parent_id().as_unique_number(), 0);
            assert_eq!(
                teardown1.description.description(),
                Some("the description of this teardown D")
            );
            assert_eq!(teardown1.description.component_type(), &ComponentType::TearDown);
            assert_eq!(teardown1.attributes.ignore, true);
            assert_eq!(teardown1.attributes.critical_threshold.as_secs(), 2);
            assert_eq!(teardown1.attributes.concurrency_mode, ConcurrencyMode::Parallel);
        }
    }

   /* mod should_override_parameters_even_when_nested_in_anther_suite {
        // POPULATE
    }*/

}