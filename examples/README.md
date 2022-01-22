# Guide

## Say Hello world to Integra8.

```rust
#[macro_use]
pub extern crate integra8;

// Test main is required to setup the application entrypoint and bootstrap the test framework
main_test! {
}

/// # Hello World
/// a test can be declared with the the `#[integration_test]` decoration.
#[integration_test]
fn hello_world_test() {
    println!("Hello integra8!");
}
```

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

## Setup and Teardown

A `Setup` or `Teardown` can be declared with the `#[setup]` and `#[teardown]` decoration and also can be `async`.
Different test frameworks can have variations in how setup's and teardown's work.

Within Integra8

- Every `Setup` will run _once_ at the start of the test run, (ie once per _suite_, not once per _test_)
- Every `Tear down` is _guaranteed_ to run regardless if a `test`, `setup` or `tear down` fails.
    *Except if they belong to a suite which was never run*

```rust
#[setup]
fn setup() {
    println!("Setup is run first");
}

#[integration_test]
fn test_1() {
    println!("Then test 1 is run");
}

#[integration_test]
fn test_2() {
    println!("And then test 2 is run, but fails");
    assert!(false, "Test 2 fails")
}

#[integration_test]
fn test_3() {
    println!("As test 2 failed, test 3 is never called ");
}

#[teardown]
fn teardown_1() {
    println!("However teardown 1 is run regardless of the failure");
}

#[teardown]
fn teardown_2() {
    assert!(false, "teardown 2 fails")
}

#[teardown]
async fn teardown_3() {
    println!("And also teardown 3 is run regardless of all other failures");
}

```


## Concurrency

Using the `#[parallel]` or `#[sequential]` decoration on `Tests` `Setups` `Tear downs` and `Suites` can influence concurrency behavior. 

Integra8 always honors the component order in code, which allows the Concurrency modes to mixed in unique ways.
Exact implementation details can be found [here](./../core/scheduling/src/component.rs)

> By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited. See [main.rs](./3_test_main/a_global_settings/src/main.rs) 

```rust

// 1: test_1 and test_2 are executed at the same time
#[integration_test]
#[parallel]
fn test_1() { 

}

#[integration_test]
#[parallel]
fn test_2() { 

}

// 2: test_3 can only be executed after test 1 and test 2 completes

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites`
// are assumed to be `sequential` unless overridden using parameters or inherited
fn test_3() { 

}

// 2: test_4 can only be executed after test 3 completes

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed 
// to be `sequential` unless overridden using parameters or inherited
fn test_4() { 

}

// 3: test_5 and test_6 can be executed at the same time
// but only after test_4 has completes
//
// This is because Integra8 is honoring the order
// which each component is defined in the test fie
#[integration_test]
#[parallel]
fn test_5() { 

}

#[integration_test]
#[parallel]
fn test_6() { 

}

```

## Timeout Behavior

#### Duration warning threshold
`Tests` can be decorated with `#[warn_threshold_milliseconds( )]`
or `#[warn_threshold_seconds( )]` to indicate the duration threshold 
for warning result.

```rust
#[integration_test]
#[warn_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_warning() {
    sleep(Duration::from_millis(100));
}
```

#### Critical duration threshold
`Tests`, `Setups` and `Tear downs` can all be decorated with `#[critical_threshold_milliseconds( )]`
or `#[critical_threshold_seconds( )]` to indicate the max duration 
before it is aborted.

```rust
#[integration_test]
#[critical_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_error() {
    sleep(Duration::from_millis(100));
}
```













## Suites
A `Suites` can be declared with the `#[Suite]` decoration.
`Suites` are a groupings of `tests`, `setups`, `tear downs` and other `suites`, which 
can be used to change execution, failure, and concurrency behavior.

### Execution Order
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

/// Suites at the root level, are run after 
/// root tests have completed
#[suite]
mod first_suite {

    /// Same execution order applies inside a suite 
    /// Setup ➞ Test ➞ Suites  ➞ Teardown
    #[setup]
    fn setup() {
        println!("first_suite::setup is called first");
    }

    #[integration_test]
    fn test() {
        println!("Then first_suite::test is called");
    }

    #[teardown]
    fn teardown() {
        println!("And first_suite::teardown is called");
    }
}

/// Suites are run in the order they appear within file.
#[suite]
mod another_suite {
 
    #[integration_test]
    fn test1() {
        println!("Then another_suite::test_1 finally 1 is called");
    }
}

```

## Nested Suites
`Suites` can be nested within each other to produce complex test behaviors
such as multi step tests, grouping by function or scenario, or given then when type tests.

```rust

#[suite]
mod matryoshka_suite {
 
    #[integration_test]
    fn test1() {
        println!("Called first");
    }

    #[suite]
    mod inner_matryoshka_suite {
    
        #[integration_test]
        fn inner_test_1() {
            println!("Called second");
        }

        #[suite]
        mod inner_most_matryoshka_suite {
        
            #[integration_test]
            fn inner_most_test_1() {
                println!("Called last");
            }
        }
    }
}
```

## Cascading Suites Failure Behavior
`Suite` failures cascaded up to the root suite, causing execution of parent suites to abort as the failure bubbles up.
Failure bubbling can be stopped with the use of `#[allow_fail]` decoration. This will cause the failure to 
bubble as a warning and prevent further abortion to parent suites.

```rust

#[suite]
mod suite_which_will_fail {
    // Suites with #[allow_fail], will not effect their parent suite
    // However internally all nested components will be aborted
    #[suite]
    #[allow_fail]
    mod another_suite {
    
        // Tests with #[allow_fail], will not effect their parent suite
        #[allow_fail]
        #[integration_test]
        fn test_1() {
            assert!(false, "Failing hard, hardly failing")
        }

        // However failing tests without #[allow_fail], 
        // immediately aborts execution of their parent suite.
        #[integration_test]
        fn test_2() {
            assert!(false, "Really Fail")
        }
        
        // This test will not run and will be 
        // indicated as `ComponentResult::DidNotRun(DidNotRunReason::ParentFailure)`
        #[integration_test]
        fn test_2() {
            println!("Test 2 Is never run");
        }

        // However Tear downs are run           
        #[teardown]
        fn teardown() {
            println!("Teardown is run regardless of all other failures");
        }

    }

    // Failing Suites without #[allow_fail], will cascade this failures
    // to their parent suite
    #[suite]
    mod failing_suite {
    
        #[integration_test]
        fn test1() {
            assert!(false, "Fail")
        }

        #[integration_test]
        fn test2() {
            println!("Is never run");
        }
    }
    // Tear downs are always run!
    #[teardown]
    fn teardown() {
        println!("Teardown is called");
    }
}


```




## Suite Concurrency

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
    reqwest::get(&ctx.parameters.app_parameters.url)).await.unwrap()
}

```








# Special Notes:
Mac Build for 1.56 and above, seem seems to broken dues to open issue with linkme crate, used to auto detect tests
https://github.com/dtolnay/linkme/issues/41
https://github.com/CodeChain-io/intertrait/issues/6
