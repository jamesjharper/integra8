


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
use integra8::results::{ComponentResult, WarningReason, PassReason, DidNotRunReason};


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
macro_rules! assert_test {
    (        
        report => $report:expr,
        path => $path:expr,
        result => $result:expr,
        id => $id:expr,
        parent_id => $parent_id:expr,
        $($key:expr => $value:expr),* 
    ) => {
        assert_eq!($report[$id].description.path().as_str(), $path);
        assert_eq!($report[$id].result, $result);
        assert_eq!($report[$id].description.id().as_unique_number(), $id);
        assert_eq!($report[$id].description.parent_id().as_unique_number(), $parent_id);
        assert_eq!($report[$id].description.component_type(), &ComponentType::Test);

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

        assert_test!(
            report => r,
            path => "test_basics::hello_world_test",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 1,
            parent_id => 0,
            stdout => "Hello world!\n"
        );

        assert_test!(
            report => r,
            path => "test_basics::async_test",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 2,
            parent_id => 0,
        );

        assert_test!(
            report => r,
            path => "test_basics::can_shutdown_hal_9000",
            result => ComponentResult::Pass(PassReason::Accepted),
            id => 3,
            parent_id => 0,
        );

        assert_description!(
            report => r,
            id => 3,
            name => "A concise name that tells anyone what this test is doing",
            description => "A description that can be useful for adding \nexact details, assumptions or context behind \nwhy this test exists",
        );
        
        assert_test!(
            report => r,
            path => "test_basics::this_test_is_sus",
            result => ComponentResult::Warning(WarningReason::FailureAllowed),
            id => 4,
            parent_id => 0,
            stderr => "thread 'tokio-runtime-worker' panicked at 'You shall not pass!', 1_setup_test_teardown/a_test_basics/src/main.rs:69:5\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n"
        );

        assert_test!(
            report => r,
            path => "test_basics::this_test_wont_even_run",
            result => ComponentResult::DidNotRun(DidNotRunReason::Ignored),
            id => 5,
            parent_id => 0,
        );
    }

    #[integration_test]
    async fn timeout_behavior(ctx : crate::ExecutionContext) {
        assert_test_fails!("./timeout_behavior", ctx);
    }

    #[integration_test]
    async fn setup_and_tear_down_basics(ctx : crate::ExecutionContext) {
        assert_test_passes!("./setup_and_tear_down_basics", ctx);
    }
    
    #[integration_test]
    async fn setup_and_tear_down_failure_behavior(ctx : crate::ExecutionContext) {
        assert_test_fails!("./setup_and_tear_down_failure_behavior", ctx);
    }

    #[integration_test]
    async fn parallel_test_behavior(ctx : crate::ExecutionContext) {
        assert_test_passes!("./parallel_test_behavior", ctx);
    }
    
}

#[suite]
mod execution_context {

    #[integration_test]
    async fn custom_parameters(ctx : crate::ExecutionContext) {
         assert_test_passes!("./custom_parameters", ctx);
    }

    #[integration_test]
    async fn generate_test_data(ctx : crate::ExecutionContext) {
        assert_test_passes!("./generate_test_data", ctx);
    }
}

#[suite]
mod suites {

    #[integration_test]
    async fn suites_basics(ctx : crate::ExecutionContext) {
        assert_test_passes!("./suites_basics", ctx);
    }

    #[integration_test]
    async fn parallel_suite_behavior(ctx : crate::ExecutionContext) {
        assert_test_passes!("./parallel_suite_behavior", ctx);
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

    /*#[integration_test]
    async fn use_child_process(ctx : crate::ExecutionContext) {
       assert_test_passes!("./use_child_process", ctx);
    }*/
}


#[suite]
mod test_main {
    #[integration_test]
    async fn global_settings(ctx : crate::ExecutionContext) {
        assert_test_passes!("./global_settings", ctx);
    }
}