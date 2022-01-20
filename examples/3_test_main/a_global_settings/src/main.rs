#[macro_use]
pub extern crate integra8;

main_test! {
    max_concurrency: 10, // [Auto, 1, any]
    // TODO: this should be automatically detected as default
    console_output: integra8_tree_formatter::TreeFormatter,
    console_output_ansi_mode: Auto, // [Auto, Enabled, Disabled]
    console_output_level: Error,
    console_output_style: Symbols,
    console_output_encoding: Ascii,

    use_child_process: false,
    suite_concurrency: Parallel,
    test_concurrency: Parallel,

    setup_critical_threshold_seconds: 20,
    tear_down_critical_threshold_seconds: 20,

    test_warn_threshold_seconds: 30,
    test_critical_threshold_seconds: 30

}

/// # Hello World
/// a test can be declared with the the `#[integration_test]` decoration.
#[integration_test]
fn hello_world_test() {
    println!("Hello world!");
}
