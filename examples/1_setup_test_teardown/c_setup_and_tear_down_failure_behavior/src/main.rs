
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
}

// A `Setup` or `Teardown` can be declared with the `#[setup]` and `#[teardown]` decoration and also can be `async`.
// Different test frameworks can have variations in how setup's and teardown's work.
// 
// Within Integra8
// 
// - Every `Setup` will run _once_ at the start of the test run, (ie once per _suite_, not once per _test_)
// - Every `Tear down` is _guaranteed_ to run regardless if a `test`, `setup` or `tear down` fails.
//    *Except if they belong to a suite which was never run*
//
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
    assert!(false, "teardown 2 fails")
}

#[teardown]
async fn teardown_3() {
    println!("And also teardown 3 is run regardless of all other failures");
}