mod tests;

#[macro_use]
pub extern crate integra8;
main_test! {
    max_concurrency: 1,
    use_child_process: true,
    critical_threshold_seconds: 2,
    warn_threshold_seconds: 10,
    console_output: integra8_tree_formatter::TreeFormatter,
    //console_output: integra8::formaters::PrettyFormatter,
    settings : {
        #[structopt(long = "target-url", default_value = "https://httpbin.org/ip")]
        pub url: String,
    }
}
