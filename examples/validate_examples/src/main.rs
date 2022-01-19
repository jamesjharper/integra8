

#[macro_use]
pub extern crate integra8;

main_test! {
    // TODO: this should be automatically detected as default
    console_output: integra8_tree_formatter::TreeFormatter,
    //console_output_ansi_mode: Auto,
    console_output_level: Verbose,
    //use_child_process: false,
    suite_concurrency: Parallel,
    test_concurrency: Parallel,
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

    #[integration_test]
    async fn simple_test() {
        assert_test_passes!("./test_basics");
    }
    #[integration_test]
    async fn test_timing() {
        assert_test_fails!("./test_timing");
    }

    #[integration_test]
    async fn setup_and_tear_down() {
        assert_test_passes!("./setup_and_tear_down");
    }
    
    #[integration_test]
    async fn setup_and_tear_down_on_failure() {
        assert_test_fails!("./setup_and_tear_down_on_failure");
    }
}



