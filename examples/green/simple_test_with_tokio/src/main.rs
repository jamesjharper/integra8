#[macro_use]
pub extern crate integra8;
use integra8::{integration_suite, integration_test, setup, teardown};

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
    settings : {
        #[structopt(long = "target-url", default_value = "https://httpbin.org/ip")]
        pub url: String,
    }
}

#[integration_suite]
mod sample_test_suite {
    use super::*;

    #[setup]
    fn setup() {
        println!("setting up!");
    }

    #[integration_test]
    fn green_test() {
        assert_eq!(true, true);
    }

    #[integration_suite]
    mod sample_nested_suite {
        use super::*;

        #[integration_test]
        #[allow_fail]
        fn nested_failing_test() {
            assert_eq!(true, false);
        }
    }

    #[teardown]
    fn teardown() {
        println!("Tear downs, always runs");
    }
}
