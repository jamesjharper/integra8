# Say Hello world to Integra8.

## Table of Contents
### Fundamentals 
1.  [Async / Sync](#Async-/-Sync)
2.  [Human-Friendly Names and Descriptions](#Human-Friendly-Names-and-Descriptions)
3.  [Allow Failure](#Allow-Failure)
4.  [Ignore Component](#Ignore-Component)
5.  [Setup and Teardown](#Setup-and-Teardown)
6.  [Concurrency](#Concurrency)
7.  [Timing-out](#Timing-out)
8.  [Setup and Teardown](#Setup-and-Teardown)

### Suites
1. [Suite Execution Order](#Suite-Execution-Order)
2. [Nested Suites](#Nested-Suites)
3. [Cascading Suite Failure Behavior](#Cascading-Suite-Failure-Behavior)
4. [Suite Concurrency](#Suite-Concurrency)

### Settings and Context
1. [Global Settings](#Global-Settings)
2. [Component Context](#Component-Context)
4. [Custom Command Line Parameters](#Custom-Command-Line-Parameters)

### Pitfalls
1. [Stdout Capture + Child Processes](#Stdout-Capture-+-Child-Processes)
2. [Async Timeout limitations](#Async-+-Timeout-limitations)


# Fundamentals 
## Async / Sync
Integra8 has native support for both `tokio` and `async-std` runtimes.
`Tests`, `Setups` and `Tear downs` can all be declared `async` and your runtime 
of choice can be enabled via the `tokio-runtime` or `async-std-runtime` feature flag.

> Integra8 internally requires an async runtime, so even if you do not require async functionality, 
> you will still need to enable either the `tokio-runtime` or `async-std-runtime` feature flag for 
> Integra8 to compile.
>
> Using `async` for long-running blocking IO is highly recommended as Integra8 is optimized for this.

### Example with tokio 
```toml
integra8 = { version = "0.0.2-alpha", features = ["tokio-runtime"] } 
```

```rust
#[integration_test]
async fn async_test() {
    tokio::time::sleep(Duration::from_millis(10)).await;
}
```
### Example with async-std 
```toml
integra8 = { version = "0.0.2-alpha", features = ["async-std-runtime"] } 
```
```rust
#[integration_test]
async fn async_test() {
    async_std::task::sleep(Duration::from_millis(10)).await;
}
```


## Human-Friendly Names and Descriptions
Code for humans first, robots second!

`Suites`, `Tests`, `Setups` and `Tear downs` can all have a human-friendly name assigned, as well as a description for documentation.
Name and description are shown in test outputs when the test fails to help give quick feedback.

### Example 
```rust
#[integration_test]
#[name = "A concise name that tells anyone what this test is doing"]
#[description =
"A description that can be useful for adding 
exact details, assumptions or context behind 
why this test exists"
]
fn can_shutdown_hal_9000() {
    assert!(false, "I'm Afraid I Can't let you do that, Dave");
}

```
Output from `./test_basics`

```
● - test_basics
└── ■ - A concise name that tells anyone what this test is doing
          description:
            A description that can be useful for adding exact details, assumptions 
            or context behind why this test exists
          src: basic/test_basics/src/main.rs:14:1
          stderr:
            thread 'async-std/runtime' panicked at 'I'm Afraid I Can't let you do that, Dave', basic/test_basics/src/main.rs:20:3
            note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

## Allow Failure 
Use the `#[allow_fail]` decoration on `Tests` and `Suites` to indicate they are allowed to fail.

### Example 
```rust
#[integration_test]
#[allow_fail]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!");
}
```

## Ignore Component
Use the `#[ignore]` decoration on `Suites`, `Tests`, `Setups` and `Tear downs` to indicate they should be skipped.

### Example 
```rust
#[integration_test]
#[ignore]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try");
}

```

## Setup and Teardown
Use the `#[setup]` and `#[teardown]` decorator indicate a `Setup` or `Teardown`.

Different frameworks have variations in how setup's and teardown's work.

Within Integra8

- Every `Setup` will run _once_ at the start of the test group. *ie once per _suite_, not per _test_*
- Like `Setup` ever `Teardown` will run _once_ at the end of the test group. *ie once per _suite_, not per _test_*
- Every `Tear down` is _guaranteed_ to run regardless if a `test`, `setup` or `tear down` fails.

### Example 
```rust
#[setup]
fn setup() {
    println!("Setup runs first");
}

#[integration_test]
fn test_1() {
    println!("Then test 1 runs");
}

#[integration_test]
fn test_2() {
    println!("And then test 2 is run ... but fails");
    assert!(false, "Test 2 fails")
}

#[integration_test]
fn test_3() {
    println!("Test 2 failed, so test 3 is skipped");
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
Use the `#[parallel]` or `#[sequential]` decorator on `Suites`, `Tests`, `Setups` and `Tear downs` to indicate concurrency behavior.

> Integra8 has a pure `async` implementation. It does not create threads and instead leaves this to your async runtime of choice.

### Concurrency Ordering behaviour  
Integra8 always honours the component order in code (for all components _except_ suites). 
Because of this, components are only run concurrently, when they are *adjacent* to other concurrent components in the scheduling order.

This design allows ordered tests to co-exist with a concept of concurrency, while also enabling concurrency modes to combine in unique ways that may not be immediately intuitive.

Exact implementation details for scheduling can be found [here](./core/scheduling/src/components.rs)

### Example 
```
          start
            :
            │
    ┌───────┴────────┐
    │                │
 [test_1]         [test_2]
    │                │
    └───────┬────────┘
            │
         [test_3]
            │
         [test_4]
            │
    ┌───────┴────────┐
    │                │
 [test_5]         [test_6] 
    │                │
    └───────┬────────┘
            │
            :
           end
```

```rust

// 1: test_1 and test_2 are executed at the same time 
// as they are adjacent and both decorated #[parallel]
#[integration_test]
#[parallel]
fn test_1() { 
    println!("Order 1");
}

#[integration_test]
#[parallel]
fn test_2() { 
    println!("Order 1");
}

// 2: test_3 can only be executed after test 1 and test 2 completes
// as it appears lower in the source code file, and is not decorated #[parallel]

#[integration_test]
#[sequential]
fn test_3() { 
    println!("Order 2");
}

// 2: test_4 can only be executed after test 3 completes.
#[integration_test]
#[sequential]
fn test_4() { 
    println!("Order 3");
}

// 3: test_5 and test_6 can be executed at the same time
// but only after test_4 has completes
#[integration_test]
#[parallel]
fn test_5() { 
    println!("Order 4");
}

#[integration_test]
#[parallel]
fn test_6() { 
    println!("Order 6");
}

```

** *By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited. See [main.rs](./examples/3_test_main/a_global_settings/src/main.rs)*

## Timing-out

### Warning Timeout threshold

Use the `#[warning_time_limit = "x secs/mins/hours/days"]` decorator on `tests` to indicate 
the maximin duration this test can run before this test is flagged with a warning. 

This can be used to give early warnings before a test exceeds some critical threshold.
For example, an HTTP request time out, lambda time out, etc.

#### Example 
```rust
#[integration_test]
#[warning_time_limit = "1min 10 seconds"]
fn this_test_will_show_a_timeout_warning() {
    sleep(Duration::from_secs(70));
}
```

### Critical Timeout threshold
Use the `#[time_limit = "x secs /mins/ hours/ days"]` decorator 
on `Tests`, `Setups` and `Tear downs` to indicate  can all be decorated with 
the maximum duration this component can run before it is forcibly aborted.

#### Example 
```rust
#[integration_test]
#[time_limit = "10ms"]
fn this_test_will_show_a_timeout_error() {
    sleep(Duration::from_millis(100));
}
```

# Suites
Use the `#[suite]` decorator to indicate a `Suite`.
`Suites` are groupings of `tests`, `setups`, `tear downs` and other `suites`, which 
can be used to change group execution, failure, and concurrency behaviours.

## Suite Execution Order
Within Integra8, the component execution order is
1. `Setups`
2. `Tests`
3. `Suites` *(recursively with the same order)*
4. `Tear downs`

### Example 
```rust

/// `Tests`, `Setups`, `Tear downs` not belonging to a suite
/// are at part of the "root" suite, and are run first. 
#[integration_test]
fn first_test() {
    println!("This test runs before any suites");
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

/// Suites are run in the order they appear within the file.
#[suite]
mod another_suite {
 
    #[integration_test]
    fn test1() {
        println!("Then another_suite::test_1 finally 1 is called");
    }
}

```

## Nested Suites
`Suites` can be nested within each other to produce complex test behaviours
such as multi-step tests, grouping by function/scenario, or `given` `then` `when` type tests.

### Example 

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

## Cascading Suite Failure Behavior
`Suite` failures cascaded upwards to the root suite, causing execution of parent suites to abort as the failure bubbles up.
Failure bubbling can be halted with the use of `#[allow_fail]` decorator. This will cause the failure to 
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
Integra8 always honours the component order in code for all components _except_ suites. 

Instead, Integra8, favours running parallel suites over serial ones and will prioritize running as many suites at once. The intent is, 
by running as many suites upfront the scheduler will remain busy longer, and increases the chances we fail sooner, 
rather than later.

Suites use the following rules 
 - Suites are grouped by concurrent mode (`parallel` or `sequential`)
 - `parallel` grouped suites are run first
 - `sequential` suites are run in the order they appear in the scheduling order.

Exact implementation details for scheduling can be found [here](./core/scheduling/src/components.rs)


### Example 

```
                 start
                   :
                   │
         ┌─────────┴──────────┐
         │                    │
   ╔═════╧══════╗       ╔═════╧══════╗
   ║  suite_2   ║       ║   suite_3  ║
   ╟─────┬──────╢       ╟─────┬──────╢ 
   ║     │      ║       ║     │      ║
   ║  [test_1]  ║       ║  [test_1]  ║
   ║     │      ║       ║     │      ║
   ║  [test_2]  ║       ║  [test_2]  ║
   ║     │      ║       ║     │      ║
   ╚═════╪══════╝       ╚═════╪══════╝  
         │                    │
         └─────────┬──────────┘
                   │
             ╔═════╧══════╗
             ║  suite_1   ║ <-- suite 1 runs last as its `sequential`
             ╟─────┬──────╢     despite it being defined first
             ║     │      ║
             ║  [test_1]  ║
             ║     │      ║
             ║  [test_2]  ║
             ║     │      ║
             ╚═════╪══════╝
                   │
                   :
                  end
```

``` rust 

#[suite]
#[sequential]
mod suite_1 {

    #[integration_test]
    fn test_1() { 
        println!("Nothing but suite_1::test_1 will run right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Nothing but suite_1::test_2 will run right now")
    }
}

#[suite]
#[parallel]
mod suite_2 {

    #[integration_test]
    fn test_1() { 
        println!("Any thing in suite_3 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite_3 could be running right now")
    }
}

#[suite]
#[parallel]
mod suite_3 {

    #[integration_test]
    fn test_1() { 
        println!("Any thing in suite_2 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite_2 could be running right now")
    }
}

```

# Settings and Context

## Global Settings
Integra8 supports several settings that can be configured globally via `test_main` or mutated via command line parameters.

### Max Concurrency: 
 - __description:__   Limits the number of components which can run at the same time
 - __test_main:__     `max_concurrency` 
 - __Command line:__  `--framework:max-concurrency` 
 - __Default:__       `"Auto"`
 - __Possible Values:__ 
    - `Auto`    : Will limit to the number of system cores available 
    - `Max`     : Limit is determined by the test schedule (can be faster for tests with a lot async blocking calls)
    - `1`       : Forces all test to run Sequentially
    - `{usize}` : You choose your own destiny 

### Child Process 
 - __description:__   When enabled, all test run in their own process. This is required for a clean log output.
 - __test_main:__     `use_child_process` 
 - __Command line:__  `--framework:use-child-process` 
 - __Default:__       `true`
 - __Possible Values:__ 
    - `true`    : All components run in their own process 
    - `false`   : All components run internal to the test application

### Default Suite Concurrency Mode
 - __description:__   Global default concurrency mode for suites
 - __test_main:__     `suite_concurrency` 
 - __Command line:__  `--default:suite-concurrency` 
 - __Default:__       `Sequential`
 - __Possible Values:__ 
    - `Sequential` : All suites run as `Sequential` unless explicitly decorated 
    - `Parallel`   : All suites run as `Parallel` unless explicitly decorated 

### Default Test Concurrency Mode
 - __description:__   Global default concurrency mode for tests
 - __test_main:__     `test_concurrency` 
 - __Command line:__  `--default:test-concurrency` 
 - __Default:__       `Sequential`
 - __Possible Values:__ 
    - `Sequential` : All suites run as `Sequential` unless explicitly decorated 
    - `Parallel`   : All suites run as `Parallel` unless explicitly decorated 

### Default Setup Timeout
 - __description:__   Global default time out for setups
 - __test_main:__     `default_setup_time_limit` 
 - __Command line:__  `--default:setup-time-limit` 
 - __Default:__       `30s`
 - __Possible Values:__ 
    - `{usize}` : Any number of seconds

### Default Tear Down Timeout
 - __description:__   Global default time out for tear downs
 - __test_main:__     `default_tear_down_time_limit` 
 - __Command line:__  `--default:tear-down-time-limit` 
 - __Default:__       `30s`
 - __Possible Values:__ 
    - `{usize}` : Any number of seconds

### Default Test Timeout
 - __description:__   Global default time out for tests
 - __test_main:__     `default_test_time_limit` 
 - __Command line:__  `--default:test-time-limit` 
 - __Default:__       `30s`
 - __Possible Values:__ 
    - `{usize}` : Any number of seconds

### Default Test Warning Timeout
 - __description:__   Global default warning time out for tests
 - __test_main:__     `default_test_warning_time_threshold_seconds` 
 - __Command line:__  `--default:test-warn-time-threshold` 
 - __Default:__       `30s`
 - __Possible Values:__ 
    - `{usize}` : Any number of seconds


### Example 

```rust

#[macro_use]
pub extern crate integra8;

main_test! {

    // Limit the number of components which can run at the same time;
    max_concurrency: 1, 

    // When enabled, all test run in their own process.
    // This is required for a clean log output,
    use_child_process: false,

    // Global default concurrency mode for suites
    suite_concurrency: Parallel,

    // Global default concurrency mode for testes
    test_concurrency: Parallel,

    // Global default time out for setups
    default_setup_time_limit: "20 seconds",

    // Global default time out for tear downs
    tear_down_time_limit_seconds: "20 seconds",

    // Global default warning threshold for tests
    test_warning_time_threshold_seconds: "20 seconds",

    // default time out for tests
    test_time_limit_seconds: "20 seconds",
}

#[integration_test]
fn global_defaults() {

}

#[integration_test]
#[sequential]
#[warning_time_limit_milliseconds(10)]
#[time_limit_milliseconds(10)]
fn override_global_defaults() {

}
```

## Component Context 
Integra8 supports a notion of *context*, which is used for forwarding state and context data to executing components.
Context can be accessed by adding the parameter `&crate::ExecutionContext` to the test signature.

```rust

#[integration_test]
fn access_context(ctx : crate::ExecutionContext) {
    // Use context struct for generating test data, managing state and accessing command line parameters.
    // These will likely be extended in later releases.

    // The components order id.
    // this will always be a number which is unique to all other tests
    // TODO: Double check that works correctly with child processes tests (Nope, its broken)!
    println!("id: {}", ctx.description.id().as_unique_number());

    // The components parents order id.
    // this will always be a number which is unique to all other tests
    // TODO: Double check that works correctly with child processes tests (Nope, its broken)!
    println!("id: {}", ctx.description.id().as_unique_number());

    // The name assigned via #[name = "..."]
    // If no name is assigned then the components path
    println!("full_name: {}", ctx.description.full_name());

    // The name assigned via #[name = "..."]
    // If no name is assigned then the components *relative* path
    println!("friendly_name: {}", ctx.description.friendly_name());

    // The description assigned via #[description = "..."]
    // If no  description assigned is assigned, then `None`
    println!("description: {}", ctx.description.description());

    // The full path of this component 
    println!("path: {}",ctx.description.path());

    // The path of this component relative to its parent 
    println!("relative_path: {}",ctx.description.relative_path());

    // The file name this component was defined
    println!("file_name: {}", ctx.description.location().file_name);
}
```


### Generating Context Data Example 

```rust

#[suite]
mod test_some_user_actions {

    fn suite_user_name(ctx : &crate::ExecutionContext) -> String {
        // ctx.description.parent_id() will always return the same unique number when called within the same suite
        format!("user_{}", ctx.description.parent_id().as_unique_number())
    }

    fn item_name(ctx : &crate::ExecutionContext) -> String {
        // ctx.description.id() will always return a number which is unique to all other tests
        format!("item_{}", ctx.description.id().as_unique_number())
    }

    #[setup]
    fn setup(ctx : crate::ExecutionContext) {
        println!("Creating user \"{}\"", suite_user_name(&ctx));
        
        // Create user in the system under test ... 
    }

    #[integration_test]
    fn do_some_action(ctx : crate::ExecutionContext) {
        println!("User \"{}\" performs some action on item \"{}\"", suite_user_name(&ctx), item_name(&ctx));
        
        // Have user x do something ... 
    }

    #[integration_test]
    fn do_some_something_else(ctx : crate::ExecutionContext) {
        println!("User \"{}\" performs another action on item \"{}\"", suite_user_name(&ctx), item_name(&ctx));
        
        // Have user x do something else ... 
    }

    #[teardown]
    fn teardown(ctx : crate::ExecutionContext) {
        println!("Removing user \"{}\"", suite_user_name(&ctx));
        
        // Remove the user in the system under test ...
    }
}


```


## Custom Command Line Parameters
Integra8 supports a notion of *test context*, which can be used for managing state between 
tests and forwarding command line parameters within a test application.

Internally, Integra8 leverages [structopt](https://docs.rs/structopt/latest/structopt/) for managing 
command line parameters. The input parameters can be extended via `main_test{ parameters : ... }` which takes 
either an inline `struct` definition or externally defined Type which implements the `structopt` trait.

> Note, your toml file must include `structopt` for the macro to be able to find it.

### Example 

```rust
use reqwest;

#[macro_use]
pub extern crate integra8;

main_test! {
    parameters : {
        #[structopt(long = "target-url", default_value = "https://httpbin.org/ip")]
        pub url: String,
    }
}


#[integration_test]
async fn httpbin_should_reply_200_ok(ctx : crate::ExecutionContext) {

    // Note: crate::ExecutionContext was automatically generated by main_test!

    #[cfg(feature = "tokio-runtime")]
    let response = reqwest::get(&ctx.parameters.app.url).await.unwrap();

    // reqwest does not support async-std, so blocking must be used instead.
    // Its recommended to use async for these types of tests, as 
    // integra8 will run other tests while this test waits for a response 
    #[cfg(feature = "async-std-runtime")]
    let response = reqwest::blocking::get(&ctx.parameters.app.url).unwrap();

    println!("{:#?}", response);
    assert_eq!(response.status(), 200, "Expected http 200 response");
}
```

# Pitfalls

## Stdout Capture + Child Processes
Rust's inbuilt test framework makes use of `std::io::stdio::set_output_capture` to capture `stdout` outputs. 
This API is unstable, and therefore Integra8 does not make use of it. To provide comparable functionality,
Integra8 starts all `Tests`, `Setups` and `Tear downs` in their process.

While this is acceptable for most uses cases, it does undermine the async runtime's ability to 
schedule optimally and may impact any custom global state-managed added by a test author.

This behaviour can be disabled with the `use_child_process` setting or `--framework:use-child-process` command line parameter, 

This however will also disable log stdout capture, resulting in logs to console interleaving when running tests. 

When disabling `use_child_process`, consider using *artefacts writers* as described in the _Test Artifacts workaround_ segement bellow. 

```rust

#[macro_use]
pub extern crate integra8;

main_test! {
    // When disabled, all test run in this process.
    use_child_process: false,
}

#[integration_test]
fn chaos_logging() {
    println!("This will show immediately in the console output");
}
```

### Test Artifacts workaround
It can be useful to collect and collate artefacts from test runs, such as harvesting a program's state in between tests,
settings, or system metrics. Integra8 can manage artefacts via the test context.

*This feature is still under development and currently will only work when `use_child_process` is disabled.*


```rust

#[macro_use]
pub extern crate integra8;

main_test! {
    // Required for the feature to work correctly
    use_child_process: false,
}

#[integration_test]
fn no_more_chaos_logging(ctx : crate::ExecutionContext) {
    // writer's content will automatically be included in the test output
    let mut writer = ctx.artifacts.writer("log");
    write!(writer, "This will be listed in the test output")
}

#[integration_test]
fn arbitrary_test_data(ctx : crate::ExecutionContext) {
    // write arbitrary snippet of data to test output
    ctx.artifacts.include_text("test_agent", "hostname");
}

#[integration_test]
fn still_work_in_progress(ctx : crate::ExecutionContext) {
    // write the contents of this log to test output
    ctx.artifacts.include_text_file("log", "./logs.text");

    // Currently Integra8 does not manage these files lift time.
    // If this file is deleted in a teardown, the content will not be shown.
    //
    // Use this at your own risk, this implementation will likely change in the future.
}
```

## Async + Timeout limitations 
Integra8 does not create threads and instead relies on the async runtime to manage multi-tasking.
While this does offer performance benefits, it, unfortunately, has its drawbacks.

### Timeout Detection 
The current timeout implementation can only detect a timeout and abort when a task is paused.
Long-running non-async operations are not detected, and instead, execution will continue until the task is either paused or complete.

```rust

#[integration_test]
#[time_limit = "1 s"]
fn bad_test_design() {
    // `std::thread::sleep`, puts the thread to sleep, but not the task.
    // The runtime and Integra8 can not intervene here and this test will wait 
    // 10 seconds, and then fail with a timeout error
    std::thread::sleep(std::time::Duration::from_seconds(10));
}

#[integration_test]
#[time_limit = "1 s"]
async fn good_test_design() {
    // `tokio::time::sleep`, puts the task to sleep, but not the thread.
    // This thread will go do something else useful while we wait.
    // The runtime will alow Integra8 to intervene some time after 
    // 1 second has elapsed and abort this test.  
    tokio::time::sleep(std::time::Duration::from_seconds(10)).await;
}

#[integration_test]
async fn reqwest() {

    // tokio is preferable if you are also using reqwest
    #[cfg(feature = "tokio-runtime")]
    let response = reqwest::get("https://httpbin.org/ip").await.unwrap();

    // reqwest does not support async-std, so using `blocking` this recommended 
    #[cfg(feature = "async-std-runtime")]
    let response = reqwest::blocking::get("https://httpbin.org/ip").unwrap();

    assert_eq!(response.status(), 200, "Expected http 200 response");
}

#[integration_test]
#[time_limit = "1 s"]
fn running_with_a_loaded_shotgun() {
    // This test will never complete, and the process will not close.
    loop {
        println!("Hope you have access to your build server...");
    }        
}


```

### Timeout Accuracy  
The async runtime does not guarantee tasks are resumed immediately after an async operation has been completed. 
This results in a degree of variability in test run times, which in turn limits the timeout resolution.
As result, it is recommended to avoid using very short or very exact timeout durations as could lead to flaky test runs.

```rust

#[integration_test]
#[time_limit = "10ms"]
async fn fairly_bad_idea() {
    // Good chance this test will fail at random,
    // as this test will only continue execution once the async runtime 
    // has a spare thread lying around.
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
}

#[integration_test]
#[time_limit = "10s 10ms"]
async fn also_a_fairly_bad_idea() {
    // Some more niche hardware tests can require this kind of assertion.
    // For this, you might have to use `std::thread::sleep` or your own
    // assertion internal to the test.
    tokio::time::sleep(std::time::Duration::from_millis(1010)).await;
}

```