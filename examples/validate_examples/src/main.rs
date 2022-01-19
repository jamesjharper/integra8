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

//#[macro_export]
macro_rules! assert_test_passes {
    ($exe_name:expr) => {
        Command::new($exe_name)
            .kill_on_drop(true)
            .output()
            .await
            .expect("failed");
    }
}


#[integration_test]
#[name("custom named for test")]
#[description("the test description")]
async fn test1() {
    assert_test_passes!("./basic_sample_tests");
}
