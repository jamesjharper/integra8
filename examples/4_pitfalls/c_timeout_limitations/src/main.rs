#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
}

// # Warning time limit
// Decorate tests with `#[warning_time_limit( )]] to 
// indicate the max duration for warning is issued.
#[integration_test]
#[warning_time_limit = "10 ms"]
fn this_test_will_show_a_timeout_warning() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

// # Timeout limit 
// Decorate tests with `#[time_limit( )]` to indicate 
// the max duration before a test is aborted.
#[integration_test]
#[time_limit = "10 ms"]
fn this_test_will_show_a_timeout_error() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

