


#[macro_use]
pub extern crate integra8;


main_test! {
    max_concurrency: Auto, // [Auto, 1, any]

    // TODO: this should be automatically detected as default
    console_output: integra8_tree_formatter::TreeFormatter,
    //console_output_ansi_mode: Auto,
   // console_output_level: Verbose,
    use_child_process: false,
    default_suite_concurrency: Parallel,
    default_test_concurrency: Parallel,
}

macro_rules! run_tests {
    ($exe_name:expr, $ctx:expr) => {
        {
            use async_process::{Command, Stdio};
            match Command::new($exe_name)
                .kill_on_drop(true)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await {
                Err(e) => panic!("Failed to run test binary {}, {}", $exe_name, e.to_string()),
                Ok(r) => {
                    let result = r.status.code();
                    let stdout_string = String::from_utf8(r.stdout).unwrap();
                    let stderr_string = String::from_utf8(r.stderr).unwrap();

                    use integra8::results::report::ComponentRunReport;
                    use serde_yaml::Error;

                    let test_report : Result<Vec<ComponentRunReport>, Error>  = serde_yaml::from_str(&stdout_string);
                    $ctx.artifacts.include_text("stdout", stdout_string);
                    $ctx.artifacts.include_text("stderr", stderr_string);

                    (result, test_report)
                }
            }
        }
    }
}

macro_rules! assert_test_passes {
    ($exe_name:expr, $ctx:expr) => {
        match run_tests!($exe_name, $ctx) {
            (Some(0), Ok(report)) => {
                // success!
                report
            },
            (Some(0), Err(e)) => {
                panic!("Failed to parse formatted output {}", e)
            },
            (Some(other), _ ) => {
                panic!("Expected status code 0, but received {}", other)
            },
            (None, _ ) => {
                panic!("Expected status code 0, but received none")
            },
        }
    }
}

macro_rules! assert_test_fails {
    ($exe_name:expr, $ctx:expr) => {
        match run_tests!($exe_name, $ctx) {
            (Some(1), Ok(report)) => {
                // Failure ... which is a success!
                report
            },
            (Some(1), Err(e)) => {
                panic!("Failed to parse formatted output {}", e)
            },
            (Some(other), _ ) => {
                panic!("Expected status code 1, but received {}", other)
            },
            (None, _ ) => {
                panic!("Expected status code 1, but received none")
            },
        }
    }
}

use integra8::components::ComponentType;
use integra8::results::{ComponentResult, WarningReason, PassReason, FailureReason, DidNotRunReason};


#[macro_export]
macro_rules! assert_root_suite {
    (
        report => $report:expr,
        path => $path:expr,
        result => $result:expr,
    ) => {
        assert_eq!($report[0].description.path().as_str(), $path);
        assert_eq!($report[0].result, $result);
        assert_eq!($report[0].description.id().as_unique_number(), 0);
        assert_eq!($report[0].description.parent_id().as_unique_number(), 0);
        assert_eq!($report[0].description.component_type(), &ComponentType::Suite);
    };
}



#[macro_export]
macro_rules! assert_component {
    (        
        report => $report:expr,
        path => $path:expr,
        result => $result:expr,
        id => $id:expr,
        parent_id => $parent_id:expr,
        component_type => $component_type:expr,
        $($key:expr => $value:expr),* 
    ) => {
        assert_eq!($report[$id].description.path().as_str(), $path);
        assert_eq!($report[$id].result, $result);
        assert_eq!($report[$id].description.id().as_unique_number(), $id);
        assert_eq!($report[$id].description.parent_id().as_unique_number(), $parent_id);
        assert_eq!($report[$id].description.component_type(), &$component_type);

        $(
            assert_eq!($report[$id].artifacts.map[stringify!($key)].as_string().unwrap(), $value);
        )*
    };
}

#[macro_export]
macro_rules! assert_description {
    (        
        report => $report:expr,
        id => $id:expr,
        name => $name:expr,
        description => $description:expr,
    ) => {
        assert_eq!($report[$id].description.friendly_name().as_str(), $name);
        assert_eq!($report[$id].description.description(), Some($description));
    };
}


#[suite]
mod basic_examples {

    use super::*;

    #[integration_test]
    async fn test_basics(ctx : crate::ExecutionContext) {
        // Act
        let r = assert_test_passes!("./test_basics", ctx);

        // Assert 
        assert_root_suite!(
            report => r,
            path => "test_basics",
            result => ComponentResult::Warning(WarningReason::ChildWarning),
        );

        assert_component!(
            report => r,
            path => "test_basics::hello_world_test",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "Hello world!\n"
        );

        assert_component!(
            report => r,
            path => "test_basics::async_test",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "test_basics::can_shutdown_hal_9000",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 3,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_description!(
            report => r,
            id => 3,
            name => "A concise name that tells anyone what this test is doing",
            description => "A description that can be useful for adding \nexact details, assumptions or context behind \nwhy this test exists",
        );
        
        assert_component!(
            report => r,
            path => "test_basics::this_test_is_sus",
            result => ComponentResult::Warning(WarningReason::FailureAllowed),
            id => 4,
            parent_id => 0,
            component_type => ComponentType::Test,
            stderr => "thread 'tokio-runtime-worker' panicked at 'You shall not pass!', 1_setup_test_teardown/a_test_basics/src/main.rs:69:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"
        );

        assert_component!(
            report => r,
            path => "test_basics::this_test_wont_even_run",
            result => ComponentResult::DidNotRun(DidNotRunReason::Ignored),
            id => 5,
            parent_id => 0,
            component_type => ComponentType::Test,
        );
    }

    #[integration_test]
    async fn setup_and_tear_down_basics(ctx : crate::ExecutionContext) {
        let r = assert_test_passes!("./setup_and_tear_down_basics", ctx);

        // Assert 
        assert_root_suite!(
            report => r,
            path => "setup_and_tear_down_basics",
            result => ComponentResult::Pass(PassReason::Accepted),
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_basics::setup",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Setup,
            stdout => "Setup is called first\n"
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_basics::test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "Then test 1 is called\n"
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_basics::test_2",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 3,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "And then test 2 is called\n"
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_basics::teardown",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 4,
            parent_id => 0,
            component_type => ComponentType::TearDown,
            stdout => "And finally teardown is called\n"
        );
    }
    
    #[integration_test]
    async fn setup_and_tear_down_failure_behavior(ctx : crate::ExecutionContext) {
        // Act
        let r = assert_test_fails!("./setup_and_tear_down_failure_behavior", ctx);

         // Assert 
         assert_root_suite!(
            report => r,
            path => "setup_and_tear_down_failure_behavior",
            result => ComponentResult::Fail(FailureReason::ChildFailure),
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_failure_behavior::setup",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Setup,
            stdout => "Setup is called first\n"
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_failure_behavior::test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "Then test 1 is called\n"
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_failure_behavior::test_2",
            result => ComponentResult::Fail(FailureReason::Rejected),
            id => 3,
            parent_id => 0,
            component_type => ComponentType::Test,
            stderr => "thread 'tokio-runtime-worker' panicked at 'Test 2 fails', 1_setup_test_teardown/c_setup_and_tear_down_failure_behavior/src/main.rs:31:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_failure_behavior::test_3",
            result => ComponentResult::DidNotRun(DidNotRunReason::ParentFailure),
            id => 4,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_failure_behavior::teardown_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 5,
            parent_id => 0,
            component_type => ComponentType::TearDown,
            stdout => "However teardown 1 is run regardless of the failure\n"
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_failure_behavior::teardown_2",
            result => ComponentResult::Fail(FailureReason::Rejected),
            id => 6,
            parent_id => 0,
            component_type => ComponentType::TearDown,
        );

        assert_component!(
            report => r,
            path => "setup_and_tear_down_failure_behavior::teardown_3",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 7,
            parent_id => 0,
            component_type => ComponentType::TearDown,
            stdout => "And also teardown 3 is run regardless of all other failures\n"
        );
    }

    #[integration_test]
    async fn parallel_test_behavior(ctx : crate::ExecutionContext) {
        // Act 
        let r = assert_test_passes!("./parallel_test_behavior", ctx);

        // Assert 
        assert_root_suite!(
            report => r,
            path => "parallel_test_behavior",
            result => ComponentResult::Pass(PassReason::Accepted),
        );

        assert_component!(
            report => r,
            path => "parallel_test_behavior::test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "parallel_test_behavior::test_2",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "parallel_test_behavior::test_3",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 3,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "parallel_test_behavior::test_4",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 4,
            parent_id => 0,
            component_type => ComponentType::Test,
        );
        
        assert_component!(
            report => r,
            path => "parallel_test_behavior::test_5",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 5,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "parallel_test_behavior::test_6",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 6,
            parent_id => 0,
            component_type => ComponentType::Test,
        );
    }
    
    #[integration_test]
    async fn timeout_behavior(ctx : crate::ExecutionContext) {
        // Act 
        let r = assert_test_fails!("./timeout_behavior", ctx);

        // Assert 
        assert_root_suite!(
            report => r,
            path => "timeout_behavior",
            result => ComponentResult::Fail(FailureReason::ChildFailure),
        );

        assert_component!(
            report => r,
            path => "timeout_behavior::this_test_will_show_a_timeout_warning",
            result => ComponentResult::Warning(WarningReason::OvertimeWarning),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "timeout_behavior::this_test_will_show_a_timeout_error",
            result => ComponentResult::Fail(FailureReason::Rejected),
            id => 2,
            parent_id => 0,
            component_type => ComponentType::Test,
        );
    }

    #[integration_test]
    async fn multi_file_test_order(ctx : crate::ExecutionContext) {
        let r = assert_test_passes!("./multi_file_test_order", ctx);

        // Assert 
        assert_root_suite!(
            report => r,
            path => "multi_file_test_order",
            result => ComponentResult::Pass(PassReason::Accepted),
        );

        assert_component!(
            report => r,
            path => "multi_file_test_order::test_c",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "Test C was called first\n"
        );

        assert_component!(
            report => r,
            path => "multi_file_test_order::a_test_mod::test_a",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "Test A was called second\n"
        );

        assert_component!(
            report => r,
            path => "multi_file_test_order::a_test_mod::aa_test_mod::test_zz",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 3,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "Test ZZ was called third\n"
        );

        assert_component!(
            report => r,
            path => "multi_file_test_order::b_test_mod::test_b",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 4,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "Test B was called last\n"
        );
    }
}

#[suite]
mod execution_context {
    use super::*;

    #[integration_test]
    async fn custom_parameters(ctx : crate::ExecutionContext) {
         let r = assert_test_passes!("./custom_parameters", ctx);

        // Assert 
        assert_root_suite!(
            report => r,
            path => "custom_parameters",
            result => ComponentResult::Pass(PassReason::Accepted),
        );

        assert_component!(
            report => r,
            path => "custom_parameters::httpbin_should_reply_200_ok",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Test,
        );
    }

    #[integration_test]
    async fn generate_test_data(ctx : crate::ExecutionContext) {
        let r = assert_test_passes!("./generate_test_data", ctx);


        // Assert 
        assert_root_suite!(
            report => r,
            path => "generate_test_data",
            result => ComponentResult::Pass(PassReason::Accepted),
        );

        // TODO: fix!
    }
}

#[suite]
mod suites {
    use super::*;

    #[integration_test]
    async fn suites_basics(ctx : crate::ExecutionContext) {
        let r = assert_test_passes!("./suites_basics", ctx);


        // Assert 
        assert_root_suite!(
            report => r,
            path => "suites_basics",
            result => ComponentResult::Pass(PassReason::Accepted),
        );

        assert_component!(
            report => r,
            path => "suites_basics::first_test",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Test,
            stdout => "This test before any suites\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::first_suite",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 0,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "suites_basics::first_suite::setup",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 3,
            parent_id => 2,
            component_type => ComponentType::Setup,
            stdout => "first_suite::setup is called first\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::first_suite::test",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 4,
            parent_id => 2,
            component_type => ComponentType::Test,
            stdout => "Then first_suite::test is called\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::first_suite::teardown",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 5,
            parent_id => 2,
            component_type => ComponentType::TearDown,
            stdout => "And first_suite::teardown is called\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::another_suite",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 6,
            parent_id => 0,
            component_type => ComponentType::Suite,
        );


        assert_component!(
            report => r,
            path => "suites_basics::another_suite::test1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 7,
            parent_id => 6,
            component_type => ComponentType::Test,
            stdout => "Then another_suite::test_1 finally 1 is called\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 8,
            parent_id => 0,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::test1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 9,
            parent_id => 8,
            component_type => ComponentType::Test,
            stdout => "Call order 1\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::inner_matryoshka_suite",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 10,
            parent_id => 8,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::inner_matryoshka_suite::inner_test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 11,
            parent_id => 10,
            component_type => ComponentType::Test,
            stdout => "Call order 2\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::inner_matryoshka_suite::inner_most_matryoshka_suite",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 12,
            parent_id => 10,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::inner_matryoshka_suite::inner_most_matryoshka_suite::inner_most_test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 13,
            parent_id => 12,
            component_type => ComponentType::Test,
            stdout => "Call order 3\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::inner_matryoshka_suite::inner_most_matryoshka_suite::inner_most_teardown",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 14,
            parent_id => 12,
            component_type => ComponentType::TearDown,
            stdout => "Call order 4\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::inner_matryoshka_suite::inner_teardown",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 15,
            parent_id => 10,
            component_type => ComponentType::TearDown,
            stdout => "Call order 5\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::matryoshka_suite::teardown",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 16,
            parent_id => 8,
            component_type => ComponentType::TearDown,
            stdout => "Call order 6\n"
        );

        assert_component!(
            report => r,
            path => "suites_basics::suite_with_internal_namespaces",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 17,
            parent_id => 0,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "suites_basics::suite_with_internal_namespaces::test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 18,
            parent_id => 17,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "suites_basics::suite_with_internal_namespaces::internal_namespace::test_2",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 19,
            parent_id => 17,
            component_type => ComponentType::Test,
        );
    }

    #[integration_test]
    async fn parallel_suite_behavior(ctx : crate::ExecutionContext) {
        let r = assert_test_passes!("./parallel_suite_behavior", ctx);

        // Assert 
        assert_root_suite!(
            report => r,
            path => "parallel_suite_behavior",
            result => ComponentResult::Pass(PassReason::Accepted),
        );

        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_1::test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 1,
            component_type => ComponentType::Test,
        );
      
        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_1::test_2",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 3,
            parent_id => 1,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_2",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 4,
            parent_id => 0,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_2::test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 5,
            parent_id => 4,
            component_type => ComponentType::Test,
        );
      
        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_2::test_2",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 6,
            parent_id => 4,
            component_type => ComponentType::Test,
        );

        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_3",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 7,
            parent_id => 0,
            component_type => ComponentType::Suite,
        );

        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_3::test_1",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 8,
            parent_id => 7,
            component_type => ComponentType::Test,
        );
      
        assert_component!(
            report => r,
            path => "parallel_suite_behavior::suite_3::test_2",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 9,
            parent_id => 7,
            component_type => ComponentType::Test,
        );
    }

    #[integration_test]
    async fn cascading_failure_behavior(ctx : crate::ExecutionContext) {
        assert_test_fails!("./cascading_failure_behavior", ctx);
    }
}

#[suite]
mod pitfalls {

    #[integration_test]
    async fn nested_sequential_behavior(ctx : crate::ExecutionContext) {
        assert_test_passes!("./nested_sequential_behavior", ctx);
    }

    #[integration_test]
    async fn timeout_limitations(ctx : crate::ExecutionContext) {
        assert_test_fails!("./timeout_limitations", ctx);
    }

    #[integration_test]
    async fn use_child_process(ctx : crate::ExecutionContext) {
       assert_test_passes!("./use_child_process", ctx);
    }
}


#[suite]
mod test_main {
    #[integration_test]
    async fn global_settings(ctx : crate::ExecutionContext) {
        assert_test_passes!("./global_settings", ctx);
    }
}