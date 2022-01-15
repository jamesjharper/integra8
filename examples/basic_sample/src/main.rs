mod tests;

#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,

    test_critical_threshold_seconds: 2,
    test_warn_threshold_seconds: 10,
    suite_concurrency: Serial,
    test_concurrency: Serial,
    settings : {
        #[structopt(long = "target-url", default_value = "https://httpbin.org/ip")]
        pub url: String,
    }
}
