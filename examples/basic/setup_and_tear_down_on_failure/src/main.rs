
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

#[setup]
async fn setup() {
    println!("Setup is run first");
}

#[integration_test]
async fn test_1() {
    println!("Then test 1 is run");
}

#[integration_test]
async fn test_2() {
    println!("And then test 2 is run, but fails");
    assert!(false, "Test 2 fails")
}

#[integration_test]
async fn test_3() {
    println!("As test 2 failed, test 3 is never called ");
}

#[teardown]
async fn teardown_1() {
    println!("However teardown 1 is run regardless of the failure");
}

#[teardown]
async fn teardown_2() {
    println!("And also teardown 2 is run regardless of the failure");
}