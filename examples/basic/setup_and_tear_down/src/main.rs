
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}


/// # Setup and Teardown
/// a setup and teardown can be declared with the  `#[setup]` and `#[teardown]` decoration. 
/// - Setups will run _once_ before tests are run,
/// - Every Tear down is _guaranteed_ to run regardless if a test fails or another tear down or setup fails.
#[setup]
async fn setup() {
    println!("Setup is called first");
}

#[integration_test]
async fn test1() {
    println!("Then test 1 is called");
}

#[integration_test]
async fn test2() {
    println!("And then test 2 is called");
}

#[teardown]
async fn teardown() {
    println!("And finally teardown is called");
}