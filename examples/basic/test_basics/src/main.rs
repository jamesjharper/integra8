
#[macro_use]
pub extern crate integra8;

// # Test main
// Test main is required to setup the application entrypoint and bootstrap the test framework
main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

/// # Hello World
/// a test can be declared with the the `#[integration_test]` decoration.
#[integration_test]
fn hello_world_test() {
    println!("Hello world!");
}

/// # Tests with custom names and descriptions
/// Tests can have a human friendly name assigned, as well as description for documentation of the test.
/// Name and description are shown in test outputs when the test fails to help give quick feedback.
/// 
/// Output from `./test_basics`
///
/// ```
/// ● - test_basics
/// └── ■ - A concise name that tells anyone what this test is doing
///           description:
///             A description which can be useful for adding exact details, assumptions 
///             or context behind why this test exists
///           src: basic/test_basics/src/main.rs:14:1
///           stderr:
///             thread 'async-std/runtime' panicked at 'You shall not pass!', basic/test_basics/src/main.rs:20:3
///             note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
///
/// ```
/// 
#[integration_test]
#[name("A concise name that tells anyone what this test is doing")]
#[description(
"A description which can be useful for adding exact details, assumptions or context behind why this test exists"
)]
fn a_test_with_a_name() {

}

/// # Sync / Async 
/// Integra8 has native support both tokio and async-std runtimes.
/// Test can be declared `async` and your runtime of choice
/// can be enabled via the \"tokio-runtime\" or \"async-std-runtime\" feature flag.
/// Integra8 internally requires an async runtime, so if you do not need to use async functionality, 
/// you will still need to enable ether the \"tokio-runtime\" or \"async-std-runtime\" feature flag for 
/// Integra8 to compile
#[integration_test]
async fn async_test() {
    #[cfg(feature = "integra8/tokio-runtime")]
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    #[cfg(feature = "integra8/async-std-runtime")]
    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
}


/// # Allow Fail Tests
/// Using the `#[allow_fail]` decoration, tests can be allowed to fail.
#[integration_test]
#[allow_fail]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!")
}

/// # Ignore Tests
/// Using the `#[ignore]` decoration, tests can skipped altogether.
#[integration_test]
#[ignore]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try")
}
