# integra8
Integra8 a test framework for Rust with a focus on productivity, extensibility, and speed.

| This repo is in a "work in progress" state, and is not yet available as a crate via crates.io.


# Say Hello world to Integra8.

```rust
#[macro_use]
pub extern crate integra8;

// Test main is required to setup the application entrypoint and bootstrap the test framework
main_test! {
}

#[integration_test]
fn hello_world_test() {
    println!("Hello integra8!");
}
```

## Why Integra8?
Thanks to its thriving community, Rust is increasingly finding more and more uses across the tech stack. With this growth comes the need for new tools to meet its new demands.

Rust has great inbuilt support for Continuos Integration Testing, Integra8's goal is to bring that same experience to the Continuos Deployment side of testing.

You should consider Integra8 for these types of use
- Web service testing
- Web frontend testing
- Blue/Green Cloud deployments
- Certifications for multiple for environments 
- Running many tests at the same time
- Anything with long running blocking IO

## Why not Integra8?
Integra8 does not aim to replace Rusts existing inbuilt libtest framework. libtest is great, and many of Integra8's features can be replicated with whats already available in the community. 

> TLDR: Integra8 is kind of like what Robot is for python but with without gherkin style syntax (for now ...) 

# Quick Guide:

## Async / Sync
Integra8 has native support both `tokio` and `async-std` runtimes.
Tests can be declared `async` and your runtime of choice can be enabled 
via the `tokio-runtime` or `async-std-runtime` feature flag.

> Integra8 internally requires an async runtime, so even if you do not need async functionality, 
> you will still need to enable ether the `tokio-runtime` or `async-std-runtime` feature flag for 
> Integra8 to compile.
>
> Using `async` for long running blocking IO is highly recommended as Integra8 is optimized for this

```rust
#[integration_test]
async fn async_test() {
    #[cfg(feature = "integra8/tokio-runtime")]
    tokio::time::sleep(Duration::from_millis(10)).await;

    #[cfg(feature = "integra8/async-std-runtime")]
    async_std::task::sleep(Duration::from_millis(10)).await;
}
```

## Suites
A `Suites` can be declared with the `#[Suite]` decoration.
`Suites` are a groupings of `tests`, `setups`, `tear downs` and other `suites`, which 
can be used to change execution, failure, and concurrency behavior.

Within Integra8, the component execution order is
1. `Setups`
2. `Tests`
3. `Suites` *(recursively with the same order)*
4. `Tear downs`

```rust

/// `Tests`, `Setups`, `Tear downs` not belonging to a suite
/// are at part of the "root" suite, and are run first. 
#[integration_test]
fn first_test() {
    println!("This test before any suites");
}

#[suite]
mod first_suite {
    #[setup]
    fn setup() {
        println!("Setup is called first in this suite");
    }

    #[integration_test]
    fn test1() {
        println!("Then test 1 is called");
    }

    #[teardown]
    fn teardown() {
        println!("And teardown is called (but after nested_suite)");
    }

    #[suite]
    mod nested_suite {
        #[integration_test]
        fn nested_test() {
            println!("Suites can be nested indefinitely!");
            println!("and will be run before any parents tear down");
        }
    }
}

#[suite]
mod second_suite {
    #[integration_test]
    fn test1() {
        println!("Then finally 1 is called");
    }
}

```

## Setup and Teardown
A `Setup` or `Teardown` can be declared with the `#[setup]` and `#[teardown]` decoration and also can be `async`.
Different test frameworks can have variations in how setup's and teardown's work.

Within Integra8

- Every `Setup` will run _once_ at the start of the test run, (ie once per _suite_, not once per _test_)
- Every `Tear down` is _guaranteed_ to run regardless if a `test`, `setup` or `tear down` fails.
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

## Concurrency
Using the `#[parallel]` or `#[sequential]` decoration on `Tests` `Setups` `Tear downs` and `Suites` can influence concurrency behavior. 

Any component will be scheduled to run at the same time if it is,
1. Of the same type (`Test` `Setup` `Tear down` or `Suite`) 
2. Decorated `#[parallel]`
3. Sharing the same parent Suite.

Within Integra8, the concurrent modes can be mixed. The execution order is
1. `parallel` components
2. `Sequential` components

> By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited. See TODO: add link to documentation here

```rust
#[integration_test]
#[parallel]
fn test_1() {
    println!("Test 2 could be running now");
}

#[integration_test]
#[parallel]
fn test_2() {
    println!("Test 1 could be running now");
}

#[suite]
#[parallel]
mod first_suite {
    #[integration_test]
    fn test1() {
        println!("Anything in second_suite could be running now, but test");
    }

    #[integration_test]
    fn test1() {
        println!("Anything in second_suite could be running now");
    }
}

#[suite]
#[parallel]
mod second_suite {
    #[integration_test]
    fn test1() {
        println!("Anything in first_suite could be running now");
    }
    
}

```

## Test Context
Integra8 supports a concept of context which can be used for managing state between tests and forwarding command line parameters within a test applications.

```rust
use reqwest;

#[macro_use]
pub extern crate integra8;

main_test! {
    settings : {
        #[structopt(long = "target-url", default_value = "https://httpbin.org/ip")]
        pub url: String,
    }
}


#[integration_test]
async fn make_async_request_test(ctx : crate::TestContext) {
    reqwest::get(&ctx.parameters.app.url)).await.unwrap()
}

```

## Custom names and descriptions
`Suites`, `Tests`, `Setups` and `Tear downs` can all have a human friendly name assigned, as well as description for documentation.
Name and description are shown in test outputs when the test fails to help give quick feedback.

```rust
#[integration_test]
#[name("A concise name that tells anyone what this test is doing")]
#[description("
A description which can be useful for adding 
exact details, assumptions or context behind 
why this test exists
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

## Allow Failure 
Using the `#[allow_fail]` decoration, `Tests` and `Suites` can be allowed to fail.


```rust
#[integration_test]
#[allow_fail]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!")
}
```

## Ignore Component
Using the `#[ignore]` decoration, `Suites`, `Tests`, `Setups` and `Tear downs` can skipped altogether.

```rust
#[integration_test]
#[ignore]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try")
}

```

## Duration warning threshold
`Tests` can be decorated with `#[warning_time_limit_milliseconds( )]`
or `#[warning_time_limit_seconds( )]` to indicate the duration threshold 
for warning result.

```rust
#[integration_test]
#[warning_time_limit_milliseconds(10)]
fn this_test_will_show_a_timeout_warning() {
    sleep(Duration::from_millis(100));
}
```

## Critical duration threshold
`Tests`, `Setups` and `Tear downs` can all be decorated with `#[time_limit_milliseconds( )]`
or `#[time_limit_seconds( )]` to indicate the max duration 
before it is aborted.

```rust
#[integration_test]
#[time_limit_milliseconds(10)]
fn this_test_will_show_a_timeout_error() {
    sleep(Duration::from_millis(100));
}
```
