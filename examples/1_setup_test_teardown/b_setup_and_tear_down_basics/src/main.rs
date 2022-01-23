
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
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