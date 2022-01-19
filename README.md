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
Hello world test for integr8.

Each test application requires a `main_test! {}` to setup the application entrypoint and bootstrap the test framework,
and tests are declared using the `#[integration_test]` decoration.

```rust

// Test main is required setup the application entrypoint and bootstrap the test framework
main_test! {
}

// a test can be declared with the the `#[integration_test]` decoration.
#[integration_test]
fn hello_world_test() {
    println!("Hello world!");
}
```

Tests can have custom names assigned using the `#[name( )]` decorator.
and can also have customer descriptions, which are displayed in the output when a test fails.

```rust
#[integration_test]
#[name("Tests can have custom names")]
#[description("
And can have customer descriptions, which are displayed in the output when a test fails.
")]
fn a_test_with_a_name() {
  
}

```


# Special Notes:
Mac Build for 1.56 and above, seem seems to broken dues to open issue with linkme crate, used to auto detect tests
https://github.com/dtolnay/linkme/issues/41
https://github.com/CodeChain-io/intertrait/issues/6
