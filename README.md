# integra8
Integra8 rust integration test framework Rust with a focus on productivity, extensibility, and speed.

| This repo is in a "work in progress" state, and is not yet available as a crate via crates.io.

Work remaining before release,
[ ] Finalize formatter design
[ ] Reinstate Unit tests
[ ] Compete documentation
[ ] Write examples
[ ] Validate against Mac, Windows and Linux

```rust
#[macro_use]
pub extern crate integra8;
use integra8::{integration_suite, integration_test, setup, teardown};
use integra8::formatters::tree::TreeFormatter;

main_test! {
    console_output: TreeFormatter,
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

```

```
○ - simple_test_with_tokio - 10.700192ms 
└── ○ - sample_test_suite
    ├── ▲ - setup
    ├── ▧ - green_test
    ├── ▼ - teardown
    └── ○ - sample_nested_suite
        └── ▧ - nested_failing_test - (allowed)
```


# Special Notes:
Mac Build for 1.56 and above, seem seems to broken dues to open issue with linkme crate, used to auto detect tests
https://github.com/dtolnay/linkme/issues/41
https://github.com/CodeChain-io/intertrait/issues/6
