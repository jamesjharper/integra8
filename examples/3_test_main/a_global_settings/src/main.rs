#[macro_use]
pub extern crate integra8;
use std::time::Duration;

main_test! {

    // Limit the number of components which can run at the same time;
    // Auto:    Will limit to the number of system cores available 
    // Max:     Limit is determined by the test schedule (can be faster for tests with a lot async blocking calls)
    // 1:       Forces all test to run Sequentially
    // {usize}: You choose your own destiny 
    //
    // Default value = Auto
    max_concurrency: 4, 

    // When enabled, all test run in their own process.
    // This is required for a clean log output,
    // Default value = true
    use_child_process: false,

    // Global default concurrency mode for suites
    // Default value = Sequential
    default_suite_concurrency: "Parallel",

    // Global default concurrency mode for testes
    // Default value = Sequential
    default_test_concurrency: "Parallel",

    // Global default time out for setups
    // Default value = "30 seconds"
    default_setup_time_limit: "200 millis",

    // Global default time out for tear downs
    // Default value = "30 seconds"
    default_tear_down_time_limit: "200 millis",

    // Global default time out for tests
    // Default value = "30 seconds"
    default_test_time_limit: "200 millis",

    // Global default warning threshold for tests
    // Default value = "30 seconds"
    default_test_warning_time_limit: "10 millis",

    console_output: integra8_serde_formatter::SerdeFormatter,

    // Console output parameters will be documented once 
    // the design is finalized 
    //console_output_ansi_mode: Auto,
    //console_output_level: Error,

    // Console output style. 
    //console_output_style: Symbols,

    // Console output encoding
    // console_output_encoding: Ascii,

}


#[suite]
#[allow_fail]
mod setup_should_time_out {
    use super::*;

    #[setup]
    async fn setup_default_timeout(ctx : crate::ExecutionContext) {
        // Each component should be running under the same process
        ctx.artifacts.include_value("process_id", std::process::id());
        sleep!(Duration::from_millis(210))
    }
}

#[suite]
mod test_should_time_out_warning {
    use super::*;

    #[integration_test]
    async fn test_warning_default_timeout(ctx : crate::ExecutionContext) {
        // Each component should be running under the same process
        ctx.artifacts.include_value("process_id", std::process::id());
        sleep!(Duration::from_millis(20))
    }
}

#[suite]
#[allow_fail]
mod test_should_time_out {
    use super::*;

    #[integration_test]
    async fn test_default_timeout(ctx : crate::ExecutionContext) {
        // Each component should be running under the same process
        ctx.artifacts.include_value("process_id", std::process::id());
        sleep!(Duration::from_millis(210))
    }
}

#[suite]
#[allow_fail]
mod tear_down_should_time_out {
    use super::*;

    #[teardown]
    async fn tear_down_default_timeout(ctx : crate::ExecutionContext) {
        // Each component should be running under the same process
        ctx.artifacts.include_value("process_id", std::process::id());
        sleep!(Duration::from_millis(210))
    }
}