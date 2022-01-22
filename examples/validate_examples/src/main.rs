

#[macro_use]
pub extern crate integra8;

main_test! {
    // TODO: this should be automatically detected as default
    max_concurrency: 1, //Auto, // [Auto, 1, any]
    console_output: integra8_tree_formatter::TreeFormatter,
    //console_output_ansi_mode: Auto,
    //console_output_level: Error,
    //use_child_process: false,
    default_suite_concurrency: Parallel,
    default_test_concurrency: Parallel,
}

macro_rules! run_tests {
    ($exe_name:expr) => {
        {
            use async_process::{Command, Stdio};
            // Running the bin this way makes 
            // the exe think there is no TTY attached.
            // We can pass --console:ansi-mode to force ANSI 
            // to be Enabled/Disabled
            let ansi_mode = match atty::is(atty::Stream::Stdout) {
                true => "Enabled",
                false => "Disabled"
            };

            println!("running {} --console:ansi-mode {}", $exe_name, ansi_mode);
            match Command::new($exe_name)
                .kill_on_drop(true)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .arg("--console:ansi-mode").arg(ansi_mode)
                .output()
                .await {
                Err(e) => panic!("Failed to run test binary {}, {}", $exe_name, e.to_string()),
                Ok(r) => r
            }
        }
    }
}

macro_rules! assert_test_passes {
    ($exe_name:expr) => {
        match run_tests!($exe_name).status.code() {
            Some(0) => {
                // success!
            },
            Some(other) => assert!(false, "Expected status code 0, but received {}", other),
            None => assert!(false, "Expected status code 0, but received none"),
        };
    }
}

macro_rules! assert_test_fails {
    ($exe_name:expr) => {
        match run_tests!($exe_name).status.code() {
            Some(1) => {
                // Failure ... which is a success!
            },
            Some(other) => assert!(false, "Expected status code 1, but received {}", other),
            None => assert!(false, "Expected status code 1, but received none"),
        };
    }
}


#[suite]
mod basic_examples {

    // Somehow this got very broken
    // TODO: fix this 
    #[allow_fail] 
    #[integration_test]
    async fn test_basics() {
        assert_test_passes!("./test_basics");
    }

    #[integration_test]
    async fn timeout_behavior() {
        assert_test_fails!("./timeout_behavior");
    }

    #[integration_test]
    async fn setup_and_tear_down_basics() {
        assert_test_passes!("./setup_and_tear_down_basics");
    }
    
    #[integration_test]
    async fn setup_and_tear_down_failure_behavior() {
        assert_test_fails!("./setup_and_tear_down_failure_behavior");
    }

    #[integration_test]
    async fn parallel_test_behavior() {
        assert_test_passes!("./parallel_test_behavior");
    }
    
}


#[suite]
mod execution_context {

    #[integration_test]
    async fn custom_parameters() {
        assert_test_passes!("./custom_parameters");
    }

    #[integration_test]
    async fn generate_test_data() {
        assert_test_passes!("./generate_test_data");
    }
}

#[suite]
mod suites {

    #[integration_test]
    async fn suites_basics() {
        assert_test_passes!("./suites_basics");
    }

    #[integration_test]
    async fn parallel_suite_behavior() {
        assert_test_passes!("./parallel_suite_behavior");
    }

    #[integration_test]
    async fn cascading_failure_behavior() {
        assert_test_fails!("./cascading_failure_behavior");
    }
}

#[suite]
mod pitfalls {

    #[integration_test]
    async fn nested_sequential_behavior() {
        assert_test_passes!("./nested_sequential_behavior");
    }

    #[integration_test]
    async fn timeout_limitations() {
        assert_test_fails!("./timeout_limitations");
    }

    #[integration_test]
    async fn use_child_process() {
        assert_test_passes!("./use_child_process");
    }
}



#[suite]
mod test_main {

    #[integration_test]
    async fn global_settings() {
        assert_test_passes!("./global_settings");
    }
}