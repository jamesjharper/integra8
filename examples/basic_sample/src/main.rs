mod tests;

#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
    console_output_style: Symbols,
    //console_output_ansi_mode: Auto,
    console_output_encoding: Ascii,
    suite_concurrency: Serial,
    test_concurrency: Serial,
    settings : {
        #[structopt(long = "target-url", default_value = "https://httpbin.org/ip")]
        pub url: String,
    }
}
