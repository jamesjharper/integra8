


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
            (Some(0), Ok(_report)) => {
                // success!
               // report
            },
            (Some(0), Err(_e)) => {
                //panic!("Failed to parse formatted output {}", e)
            },
            (Some(other), _ ) => {
                panic!("Expected status code 0, but received {}", other)
            },
            (None, _ ) => {
                panic!("Expected status code 0, but received none")
            },
        };
    }
}

macro_rules! assert_test_fails {
    ($exe_name:expr, $ctx:expr) => {
        match run_tests!($exe_name, $ctx) {
            (Some(1), Ok(_report)) => {
                // Failure ... which is a success!
                //report
            },
            (Some(1), Err(_e)) => {
                //panic!("Failed to parse formatted output {}", e)
            },
            (Some(other), _ ) => {
                panic!("Expected status code 1, but received {}", other)
            },
            (None, _ ) => {
                panic!("Expected status code 1, but received none")
            },
        };
    }
}


#[suite]
mod basic_examples {

    #[integration_test]
    async fn test_basics(ctx : crate::ExecutionContext) {
        assert_test_passes!("./test_basics", ctx);
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