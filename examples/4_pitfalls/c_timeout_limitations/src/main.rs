#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

/// # Duration warning threshold
/// A test can be decorated with `#[warn_threshold_milliseconds( )]`
/// or `#[warn_threshold_seconds(1)]` to indicate the duration threshold 
/// for warning result.
#[integration_test]
#[warn_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_warning() {
    std::thread::sleep(std::time::Duration::from_millis(2000));
}

/// # Critical duration threshold
/// A test can be decorated with `#[critical_threshold_milliseconds( )]`
/// or `#[critical_threshold_seconds(1)]` to indicate the max duration 
/// before a test is aborted.
#[integration_test]
#[critical_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_error() {
    std::thread::sleep(std::time::Duration::from_millis(2000));
}

