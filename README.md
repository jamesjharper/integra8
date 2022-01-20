# integra8
Integra8 rust integration test framework Rust with a focus on productivity, extensibility, and speed.

| This repo is in a "work in progress" state, and is not yet available as a crate via crates.io.

Work remaining before release,
- [ ] Finalize formatter design
- [ ] Reinstate Unit tests
- [ ] Compete documentation
- [ ] Write examples
- [ ] Validate against Mac, Windows and Linux

# Say Hello world to Integra8.

```rust
#[macro_use]
pub extern crate integra8;

// Test main is required to setup the application entrypoint and bootstrap the test framework
main_test! {
}

#[integration_test]
fn hello_world_test() {
    println!("Hello integr8!");
}
```

# Why Integra8?
Thanks to its thriving community, Rust is increasingly finding more and more uses across the tech stack. With this growth comes the need for new tools to meet its new demands.

Rust has great inbuilt support for Continuos Integration Testing, Inter8's goal is to bring that same experience to the Continuos Deployment side of testing.

You should consider Inter8 for these types of use
- Web service testing
- Web frontend testing
- Blue/Green Cloud deployments
- Certifications for multiple for environments 
- Running many tests at the same time
- Anything with long running blocking IO


# Why not Integra8?
Inter8 does not aim to replace Rusts existing inbuilt libtest framework. libtest is great, and many of Inter8's features can be replicated with whats already available in the community. 

> TLDR: Integra8 is kind of like what Robot is for python but with without gherkin style syntax (for now ...) 

### How to guide:

## Async / Sync
Integra8 has native support both `tokio` and `async-std` runtimes.
Tests can be declared `async` and your runtime of choice can be enabled 
via the `tokio-runtime` or `async-std-runtime` feature flag.

> Integra8 internally requires an async runtime, so even if you do not need async functionality, 
> you will still need to enable ether the `tokio-runtime` or `async-std-runtime` feature flag for 
> Integra8 to compile.
>
> Using async for anything with long running blocking IO is highly recommended as Integra8 is optimized to take advantage of futures.

```rust
#[integration_test]
async fn async_test() {
    #[cfg(feature = "integra8/tokio-runtime")]
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;

    #[cfg(feature = "integra8/async-std-runtime")]
    async_std::task::sleep(std::time::Duration::from_millis(10)).await;
}
```

## Setup and Teardown
A `Setup` or `Teardown` can be declared with the `#[setup]` and `#[teardown]` decoration and also can be async.
Different test frameworks can have variations in how setup's and teardown's work.

In Integra8

- Every Setups will run _once_ at the start of the test run, (ie not once _suite_, not once per _test_)
- Every Tear down is _guaranteed_ to run regardless if a test, setup or tear down fails.
    *Except if they belong to a suite which was never run*

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
fn teardown() {
    println!("However this teardown is run regardless of the failure");
}

```

## Custom names and descriptions
`Suites`, `Tests`, `Setups` and `Tear downs` can all have a human friendly name assigned, as well as description for documentation.
Name and description are shown in test outputs when the test fails to help give quick feedback.

```rust
#[integration_test]
#[name("A concise name that tells anyone what this test is doing")]
#[description("
A description which can be useful for adding exact details, assumptions or context behind why this test exists
")]
fn a_test_with_a_name() {

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

## Allow Fail Tests 
Using the `#[allow_fail]` decoration, `Tests` and `Suites` can be allowed to fail.

```rust
#[integration_test]
#[allow_fail]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!")
}
```

## Ignore Tests
Using the `#[ignore]` decoration, `Suites`, `Tests`, `Setups` and `Tear downs` can skipped altogether.

```rust
#[integration_test]
#[ignore]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try")
}

```

## Duration warning threshold
`Tests` can be decorated with `#[warn_threshold_milliseconds( )]`
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
`Tests`, `Setups` and `Tear downs` can all be decorated with `#[critical_threshold_milliseconds( )]`
or `#[critical_threshold_seconds( )]` to indicate the max duration 
before it is aborted.

```rust
#[integration_test]
#[critical_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_error() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```



# Whats on Offer

- Test Suites, Tests, Setup and Tear downs
- Granular control over parallel / series suite and test execution 
- Baked in support for `tokio` and `async-std` async runtimes
- Customizable and extendable test output
- Test timeout and warning thresholds 
- `libtest` like capture of `stdout` and `stderr`

# Special Notes:
Mac Build for 1.56 and above, seem seems to broken dues to open issue with linkme crate, used to auto detect tests
https://github.com/dtolnay/linkme/issues/41
https://github.com/CodeChain-io/intertrait/issues/6
