use async_process::{Command, Stdio};

#[macro_use]
pub extern crate integra8;

main_test! {
    // TODO: this should be automatically detected as default
    console_output: integra8_tree_formatter::TreeFormatter,
    console_output_level: Verbose,
    //use_child_process: false,
    suite_concurrency: Parallel,
    test_concurrency: Parallel,
}

macro_rules! assert_test_passes {
    ($exe_name:expr) => {
        let out = Command::new($exe_name)
            .kill_on_drop(true)
            .output()
            .await
            .expect("failed");

        println!("{}", std::str::from_utf8(&out.stdout).unwrap());
        println!("{}", std::str::from_utf8(&out.stderr).unwrap());

        match out.status.code() {
            Some(0) => {
                // Failure! ... which is a success!
            },
            Some(other) => assert!(false, "Expected status code 0, but received {}", other),
            None => assert!(false, "Expected status code 0, but received none"),
        };
    }
}

macro_rules! assert_test_failed {
    ($exe_name:expr) => {
        let out = Command::new($exe_name)
            .kill_on_drop(true)
            .output()
            .await
            .expect("failed");

        println!("{}", std::str::from_utf8(&out.stdout).unwrap());
        println!("{}", std::str::from_utf8(&out.stderr).unwrap());

        match out.status.code() {
            Some(1) => {
                // Failure! ... which is a success!
            },
            Some(other) => assert!(false, "Expected status code 1, but received {}", other),
            None => assert!(false, "Expected status code 1, but received none"),
        };
    }
}



#[integration_test]
#[name("custom named for test")]
#[description("the test description")]
async fn test1() {
    assert_test_failed!("./simple_test");
}
