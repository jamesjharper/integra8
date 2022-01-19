# integra8
Integra8 rust integration test framework Rust with a focus on productivity, extensibility, and speed.

| This repo is in a "work in progress" state, and is not yet available as a crate via crates.io.

Work remaining before release,
- [ ] Finalize formatter design
- [ ] Reinstate Unit tests
- [ ] Compete documentation
- [ ] Write examples
- [ ] Validate against Mac, Windows and Linux

```rust
#[macro_use]
pub extern crate integra8;

main_test! {
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
    #[parallelizable]
    #[critical_threshold_seconds(10)]
    async fn response_test(ctx: crate::ExecutionContext) {
        reqwest::get(&p.parameters.app_parameters.url)
            .await.unwrap()
            .json::<std::collections::HashMap<String, String>>()
            .await.unwrap();
    }

    #[integration_test]
    #[parallelizable]
    fn another_test() {

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

# How to guide:

## Test Basics

### Hello world test for integr8.

```rust

// Test main is required to setup the application entrypoint and bootstrap the test framework
main_test! {
}

// a test can be declared with the the `#[integration_test]` decoration.
#[integration_test]
fn hello_world_test() {
    println!("Hello world!");
}
```

### Tests with custom names and descriptions

```rust
#[integration_test]
#[name("Tests can have custom names")]
#[description("
And can have customer descriptions, which are displayed in the output when a test fails.
")]
fn a_test_with_a_name() {
  
}

```

### Async / Sync tests

```rust
#[integration_test]
#[description("
Integra8 has native support both tokio and async-std runtimes.
So test can be declared `async` and your runtime of choice
can be enabled via the \"tokio-runtime\" or \"async-std-runtime\" feature flag.

Integra8 internally requires an async runtime, so if you do not need to use async functionality, 
you will still need to enable ether the \"tokio-runtime\" or \"async-std-runtime\" feature flag for 
Integra8 to compile.
")]
async fn async_test() {
    #[cfg(feature = "integra8/tokio-runtime")]
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    #[cfg(feature = "integra8/async-std-runtime")]
    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
}
```

### Allow Fail Tests

```rust
#[integration_test]
#[allow_fail]
#[description("
Using the `#[allow_fail]` decoration, tests can be allowed to fail.
")]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!")
}
```

### Ignore Tests

```rust
#[integration_test]
#[ignore]
#[description("
Using the `#[ignore]` decoration, tests can skipped altogether.
")]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try")
}

```

# Special Notes:
Mac Build for 1.56 and above, seem seems to broken dues to open issue with linkme crate, used to auto detect tests
https://github.com/dtolnay/linkme/issues/41
https://github.com/CodeChain-io/intertrait/issues/6
