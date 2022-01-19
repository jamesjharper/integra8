
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

#[integration_test]
fn hello_world_test() {
    println!("Hello world!");
}

#[integration_test]
#[name("A concise name that tells anyone what this test is doing")]
#[description(
"A description which can be useful for adding exact details, assumptions or context behind why this test exists"
)]
fn a_test_with_a_name() {
  assert!(false, "You shall not pass!")
}


#[integration_test]
#[description(
"Integra8 has native support both tokio and async-std runtimes.
So test can be declared `async` and your runtime of choice
can be enabled via the \"tokio-runtime\" or \"async-std-runtime\" feature flag.

Integra8 internally requires an async runtime, so if you do not need to use async functionality, 
you will still need to enable ether the \"tokio-runtime\" or \"async-std-runtime\" feature flag for 
Integra8 to compile."
)]
async fn async_test() {
    #[cfg(feature = "integra8/tokio-runtime")]
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    #[cfg(feature = "integra8/async-std-runtime")]
    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
}

#[integration_test]
#[allow_fail]
#[description("Using the `#[allow_fail]` decoration, tests can be allowed to fail.")]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!")
}

#[integration_test]
#[ignore]
#[description(
    "Using the `#[ignore]` decoration, tests can skipped altogether."
)]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try")
}
