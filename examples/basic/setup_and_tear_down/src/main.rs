
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

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