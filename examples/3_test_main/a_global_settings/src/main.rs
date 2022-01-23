#[macro_use]
pub extern crate integra8;

main_test! {

    // Limit the number of components which can run at the same time;
    // Auto:    Will limit to the number of system cores available 
    // Max:     Limit is determined by the test schedule (can be faster for tests with a lot async blocking calls)
    // 1:       Forces all test to run Sequentially
    // {usize}: You choose your own destiny 
    //
    // Default values = Auto
    max_concurrency: 1, 

    // When enabled, all test run in their own process.
    // This is required for a clean log output,
    // Default values = true
    use_child_process: false,

    // Global default concurrency mode for suites
    // Default values = Sequential
    default_suite_concurrency: "Parallel",

    // Global default concurrency mode for testes
    // Default values = Sequential
    default_test_concurrency: "Parallel",

    // Global default time out for setups
    // Default values = "30 seconds"
    default_setup_time_limit: "20 seconds",

    // Global default time out for tear downs
    // Default values = "30 seconds"
    default_tear_down_time_limit: "20 seconds",

    // Global default warning threshold for tests
    // Default values = "30 seconds"
    default_test_warning_time_limit: "30 seconds",

    // default time out for tests
    // Default values = "30 seconds"
    default_test_time_limit: "30 seconds",

    // TODO: this should be automatically detected as default
    console_output: integra8_tree_formatter::TreeFormatter

    // Console output parameters will be documented once 
    // the design is finalized 
    //console_output_ansi_mode: Auto,
    //console_output_level: Error,

    // Console output style. 
    //console_output_style: Symbols,

    // Console output encoding
    // console_output_encoding: Ascii,

}

#[integration_test]
fn global_defaults() {
}

#[integration_test]
#[sequential]
#[warning_time_limit = "10 ms"]
#[time_limit = "10 ms"]
fn override_global_defaults() {
}