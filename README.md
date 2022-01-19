# integra8
Integra8 rust integration test framework Rust with a focus on productivity, extensibility, and speed.

| This repo is in a "work in progress" state, and is not yet available as a crate via crates.io.

Work remaining before release,
- [ ] Finalize formatter design
- [ ] Reinstate Unit tests
- [ ] Compete documentation
- [ ] Write examples
- [ ] Validate against Mac, Windows and Linux

# How to guide:

## Hello world test for integr8.

```rust
#[macro_use]
pub extern crate integra8;

/// Test main is required to setup the application entrypoint and bootstrap the test framework
main_test! {
}

#[integration_test]
fn hello_world_test() {
    println!("Hello world!");
}
```

## Tests with custom names and descriptions
Tests can have a human friendly name assigned, as well as description for documentation of the test.
Name and description are shown in test outputs when the test fails to help give quick feedback.

```rust
#[integration_test]
#[name("A concise name that tells anyone what this test is doing")]
#[description("
A description which can be useful for adding exact details, assumptions or context behind why this test exists
")]
fn a_test_with_a_name() {
  assert!(false, "You shall not pass!")
}

```
Output from `./test_basics`

```
● - test_basics
└── ■ - A concise name that tells anyone what this test is doing
          description:
            A description which can be useful for adding exact details, assumptions 
            or context behind why this test exists
          src: basic/test_basics/src/main.rs:14:1
          stderr:
            thread 'async-std/runtime' panicked at 'You shall not pass!', basic/test_basics/src/main.rs:20:3
            note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

## Async / Sync tests
Integra8 has native support both `tokio` and `async-std` runtimes.
Tests can be declared `async` and your runtime of choice can be enabled 
via the \"tokio-runtime\" or \"async-std-runtime\" feature flag.

Integra8 internally requires an async runtime, so if you do not need to use async functionality, 
you will still need to enable ether the "tokio-runtime" or "async-std-runtime" feature flag for 
Integra8 to compile.

```rust
#[integration_test]
async fn async_test() {
    #[cfg(feature = "integra8/tokio-runtime")]
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    #[cfg(feature = "integra8/async-std-runtime")]
    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
}
```

## Allow Fail Tests
Using the `#[allow_fail]` decoration, tests can be allowed to fail.

```rust
#[integration_test]
#[allow_fail]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!")
}
```

## Ignore Tests
Using the `#[ignore]` decoration, tests can skipped altogether.

```rust
#[integration_test]
#[ignore]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try")
}

```

## Duration warning threshold
A test can be decorated with `#[warn_threshold_milliseconds( )]`
or `#[warn_threshold_seconds( )]` to indicate the duration threshold 
for warning result.

```rust
#[integration_test]
#[warn_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_warning() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

## Critical duration threshold
A test can be decorated with `#[critical_threshold_milliseconds( )]`
or `#[critical_threshold_seconds( )]` to indicate the max duration 
before a test is aborted.

```rust
#[integration_test]
#[critical_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_error() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

## Setup and Teardown
A setup or teardown can be declared with the  `#[setup]` and `#[teardown]` decoration. 
- Every Setups will run _once_ at the start of the test run.
- Every Tear down is _guaranteed_ to run regardless if a test fails or another tear down or setup fails.

```rust
#[setup]
async fn setup() {
    println!("Setup is run first");
}

#[integration_test]
async fn test_1() {
    println!("And then test 1 runs, but fails");
    assert!(false, "Test 1 fails")
}

#[integration_test]
async fn test_2() {
    println!("As test 1 failed, test 2 is never called ");
}

#[teardown]
fn teardown_1() {
    println!("However teardown 1 is run regardless of the failure");
}

#[teardown]
async fn teardown_2() {
    println!("And also teardown 2 is run regardless of the failure");
}

```

# Special Notes:
Mac Build for 1.56 and above, seem seems to broken dues to open issue with linkme crate, used to auto detect tests
https://github.com/dtolnay/linkme/issues/41
https://github.com/CodeChain-io/intertrait/issues/6
