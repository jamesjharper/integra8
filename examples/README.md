# Say Hello world to Integra8.

```rust
#[macro_use]
pub extern crate integra8;

main_test! {}

#[suite]
mod introducing {
    #[setup]
    fn setup() {
        println!("Get Ready ...");
    }

    #[integration_test]
    fn hello_world_test() {
        println!("Hello integra8!");
    }

    #[teardown]
    fn teardown() {
        println!("Enjoy!");
    }
}

```

## Table of Contents
1.  [Async / Sync](#Async-/-Sync)
2.  [Human Friendly Names and Descriptions](#Human-Friendly-Names-and-Descriptions)
3.  [Allow Failure](#Allow-Failure)
4.  [Ignore Component](#Ignore-Component)
5.  [Setup and Teardown](#Setup-and-Teardown)
6.  [Concurrency](#Concurrency)
7.  [Timing-out](#Timing-out)
8.  [Setup and Teardown](#Setup-and-Teardown)
9.  [Suites](#Suites)
10. [Nested Suites](#Nested-Suites)
11. [Cascading Suite Failure Behavior](#Cascading-Suite-Failure-Behavior)
12. [Suite Concurrency](#Suite-Concurrency)
13. [Global Settings](#Global-Settings)
14. [Component Context ](#Component-Context)
15. [Generating Context Data](#Generating-Test-Data)
16. [Custom Command Line Parameters](#Custom-Command-Line-Parameters)


# Async / Sync
Integra8 has native support both `tokio` and `async-std` runtimes.
`Tests`, `Setups` and `Tear downs` can all be declared `async` and your runtime 
of choice can be enabled via the `tokio-runtime` or `async-std-runtime` feature flag.

> Integra8 internally requires an async runtime, so even if you do not need async functionality, 
> you will still need to enable ether the `tokio-runtime` or `async-std-runtime` feature flag for 
> Integra8 to compile.
>
> Using `async` for long running blocking IO is highly recommended as Integra8 is optimized for this

## Example 

```rust
#[integration_test]
async fn async_test() {
    #[cfg(feature = "integra8/tokio-runtime")]
    tokio::time::sleep(Duration::from_millis(10)).await;

    #[cfg(feature = "integra8/async-std-runtime")]
    async_std::task::sleep(Duration::from_millis(10)).await;
}
```

# Human Friendly Names and Descriptions
Code for humans first and robots second!
`Suites`, `Tests`, `Setups` and `Tear downs` can all have a human friendly name assigned, as well as description for documentation.
Name and description are shown in test outputs when the test fails to help give quick feedback.

## Example 
```rust
#[integration_test]
#[name("A concise name that tells anyone what this test is doing")]
#[description("
A description which can be useful for adding 
exact details, assumptions or context behind 
why this test exists
")]
fn can_shutdown_hal_9000() {
    assert!(false, "I'm Afraid I Can't Do That, Dave");
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
            thread 'async-std/runtime' panicked at 'I'm Afraid I Can't Do That, Dave', basic/test_basics/src/main.rs:20:3
            note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

# Allow Failure 
Use the `#[allow_fail]` decoration on `Tests` and `Suites` to indicate they are allowed to fail.

## Example 
```rust
#[integration_test]
#[allow_fail]
fn this_test_is_sus() {
    assert!(false, "You shall not pass!");
}
```

# Ignore Component
Use the `#[ignore]` decoration on `Suites`, `Tests`, `Setups` and `Tear downs` to indicate they should should be skipped.

## Example 
```rust
#[integration_test]
#[ignore]
fn this_test_wont_even_run() {
    assert!(false, "you will never fail if you don't try");
}

```

# Setup and Teardown
A `Setup` or `Teardown` can be declared with the `#[setup]` and `#[teardown]` decorator and also can be `async`.
Different frameworks have variations in how setup's and teardown's work.

Within Integra8

- Every `Setup` will run _once_ at the start of the test run, (ie once per _suite_, not once per _test_)
- Every `Tear down` is _guaranteed_ to run regardless if a `test`, `setup` or `tear down` fails.

## Example 
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


# Concurrency
Use the `#[parallel]` or `#[sequential]` decorator on `Suites`, `Tests`, `Setups` and `Tear downs` to indicate there concurrency behavior.

> Integra8 has a pure `async`  implementation. It does not create threads, and instead leaves this to you async runtime of choice.
> TODO: add link to section which explains the pros vs cons on this design choice

## Concurrency Ordering behavior  
Integra8 always honors the component order in code. 
Due to this, components are only run concurrently, when they are *adjacent* to other concurrent components in the schedule order.

This design allows ordered tests to co-exist with a concept of concurrency, while also enabling concurrency modes to combine in unique ways that may not be immediately intuitive.
Exact implementation details for scheduling can be found [here](./../core/scheduling/src/components.rs)

## Example 
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
// as it is appears lower in the source code file, and is not decorated #[parallel]

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

** *By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited. See [main.rs](./3_test_main/a_global_settings/src/main.rs)*

# Timing-out

## Duration warning threshold
`Tests` can be decorated with `#[warn_threshold_milliseconds( )]`
or `#[warn_threshold_seconds( )]` to indicate the duration threshold 
for warning result.

### Example 
```rust
#[integration_test]
#[warn_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_warning() {
    sleep(Duration::from_millis(100));
}
```

## Critical duration threshold
`Tests`, `Setups` and `Tear downs` can all be decorated with `#[critical_threshold_milliseconds( )]`
or `#[critical_threshold_seconds( )]` to indicate the max duration 
before it is aborted.

### Example 
```rust
#[integration_test]
#[critical_threshold_milliseconds(10)]
fn this_test_will_show_a_timeout_error() {
    sleep(Duration::from_millis(100));
}
```

# Suites
A `Suites` can be declared with the `#[Suite]` decorator.
`Suites` are a groupings of `tests`, `setups`, `tear downs` and other `suites`, which 
can be used to change execution, failure, and concurrency behavior.

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

# Nested Suites
`Suites` can be nested within each other to produce complex test behaviors
such as multi step tests, grouping by function or scenario, or given then when type tests.

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

# Cascading Suite Failure Behavior
`Suite` failures cascaded up to the root suite, causing execution of parent suites to abort as the failure bubbles up.
Failure bubbling can be stopped with the use of `#[allow_fail]` decorator. This will cause the failure to 
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


# Suite Concurrency
Using the `#[parallel]` or `#[sequential]` decorator on `Tests` `Setups` `Tear downs` and `Suites` can influence concurrency behavior. 

Integra8 always honors the component order in code for all components _except_ suites. 

Instead Integra8, favors running parallel suites over serial onces, and will prioritizes running as many suites at once. The intent is, 
by running as many suites upfront the scheduler will remain busy longer, and increases the chances we fail sooner, 
rather then later.

Suites follow the following rules 
 - Suites are are group by concurrent mode (`parallel` or `sequential`)
 - `parallel` grouped suites are run first
 - `sequential` suites are run in the order they appear in the schedule order.

Exact implementation details for scheduling can be found [here](./../core/scheduling/src/components.rs)


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
             ║  suite_1   ║
             ╟─────┬──────╢
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
        println!("Any thing in suite 3 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite 3 could be running right now")
    }
}

#[suite]
#[parallel]
mod suite_2 {

    #[integration_test]
    fn test_1() { 
        println!("Any thing in suite 2 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite 2 could be running right now")
    }
}

```


# Global Settings
Integra8 supports a number of settings which can be configured globally via `test_main` or mutated via command line parameters.

## Max Concurrency: 
 - __description:__   Limits the number of components which can run at the same time
 - __test_main:__     `max_concurrency` 
 - __Command line:__  `--framework:max-concurrency` 
 - __Default:__       `"Auto"`
 - __Possible Values:__ 
    - `Auto`    : Will limit to the number of system cores available 
    - `Max`     : Limit is determined by the test schedule (can be faster for tests with a lot async blocking calls)
    - `1`       : Forces all test to run Sequentially
    - `{usize}` : You choose your own destiny 

## Child Process 
 - __description:__   When enabled, all test run in their own process. This is required for a clean log output.
 - __test_main:__     `use_child_process` 
 - __Command line:__  `--framework:use-child-process` 
 - __Default:__       `true`
 - __Possible Values:__ 
    - `true`    : All components run in their own process 
    - `false`   : All components run internal to the test application

## Default Suite Concurrency Mode
 - __description:__   Global default concurrency mode for suites
 - __test_main:__     `suite_concurrency` 
 - __Command line:__  `--default:suite-concurrency` 
 - __Default:__       `Sequential`
 - __Possible Values:__ 
    - `Sequential` : All suites run as `Sequential` unless explicitly decorated 
    - `Parallel`   : All suites run as `Parallel` unless explicitly decorated 

## Default Test Concurrency Mode
 - __description:__   Global default concurrency mode for tests
 - __test_main:__     `test_concurrency` 
 - __Command line:__  `--default:test-concurrency` 
 - __Default:__       `Sequential`
 - __Possible Values:__ 
    - `Sequential` : All suites run as `Sequential` unless explicitly decorated 
    - `Parallel`   : All suites run as `Parallel` unless explicitly decorated 

## Default Setup Timeout
 - __description:__   Global default time out for setups
 - __test_main:__     `default_setup_time_limit` 
 - __Command line:__  `--default:setup-time-limit` 
 - __Default:__       `30`
 - __Possible Values:__ 
    - `{usize}` : Any number of seconds

## Default Tear Down Timeout
 - __description:__   Global default time out for tear downs
 - __test_main:__     `default_tear_down_time_limit` 
 - __Command line:__  `--default:tear-down-time-limit` 
 - __Default:__       `30`
 - __Possible Values:__ 
    - `{usize}` : Any number of seconds

## Default Test Timeout
 - __description:__   Global default time out for tests
 - __test_main:__     `default_test_time_limit` 
 - __Command line:__  `--default:test-time-limit` 
 - __Default:__       `30`
 - __Possible Values:__ 
    - `{usize}` : Any number of seconds


## Default Test Warning Timeout
 - __description:__   Global default warning time out for tests
 - __test_main:__     `default_test_warning_time_threshold_seconds` 
 - __Command line:__  `--default:test-warn-time-threshold` 
 - __Default:__       `30`
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
    default_setup_time_limit: 20,

    // Global default time out for tear downs
    tear_down_time_limit_seconds: 20,

    // Global default warning threshold for tests
    test_warning_time_threshold_seconds: 30,

    // default time out for tests
    test_time_limit_seconds: 30,
}

#[integration_test]
fn global_defaults() {

}

#[integration_test]
#[sequential]
#[warn_threshold_milliseconds(10)]
#[critical_threshold_milliseconds(10)]
fn override_global_defaults() {

}
```

# Component Context 
Integra8 supports a concept of *test context*, which is used for forward state and context data to executing components.
The context can be accessed by by adding a parameter `ctx : &crate::ExecutionContext` to the test signature.

```rust

#[integration_test]
fn access_context(ctx : crate::ExecutionContext) {
    // Use context struct for generating test data, managing state and accessing command line parameters.

    // The name assigned via #[name(..)]. 
    // If no name is assigned then the components path
    println!(ctx.description.full_name());

    // The name assigned via #[name(..)]. 
    // If no name is assigned then the components relative path
    println!(ctx.description.friendly_name());

    // The description assigned via #[description(..)].
    //  If no  description assigned is assigned, then `None`
    println!(ctx.description.description());



    // The full path of this component 
    println!(ctx.description.path());

    // The path of this component relative to its parent 
    println!(ctx.description.relative_path());
}
```

The maybe extended in the future, at this time, the key element's
```json
{
    "description": {
        // Paths
        "path": "The components path",
        "relative_path": "The components path relative to its parent  suite",

        // Naming
        "full_name": "The name assigned via #[name(..)]. If no name is assigned then the components path",
        "friendly_name": "The name assigned via #[name(..)]. If no name is assigned then the components relative path",

        // description
        "description": "The description assigned via #[description(..)]. If no  description assigned is assigned, then `None`",

        // Identifiers 
        "id": "The order number assigned to component. This will be unique to all other components",
        "parent_id": "The order number of the parent of this component.",
        
        // code meta data  
        "location": {
            "file_name": "the file this component is defined",
            "line": "the line this component is defined",
            "column": "the colum this component is defined"
        }
    },
    "parameters" {
        "framework": {
            // Settings used by the test framework
        },
        "app" : {
            // Users defined parameters 
        },
    }
}
```

# Generating Context Data

### Example 

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
        
        // Create user in system under test ... 
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
        
        // Remove the user in system under test ...
    }
}


```


# Custom Command Line Parameters
Integra8 supports a concept of *test context*, which can be used for managing state between 
tests and forwarding command line parameters within a test applications.

Internally, Integra8 leverages [structopt](https://docs.rs/structopt/latest/structopt/) for managing 
command line parameters. The input parameters can be extended via `main_test{ parameters : ... }` which takes 
either an inline `struct` definition or externally defined Type which implements the `structopt` trait.

> Note, your toml file must include `structopt` in order for the marco to be able to find it.

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


# Special Notes:
Mac Build for 1.56 and above, seem seems to broken dues to open issue with linkme crate, used to auto detect tests
https://github.com/dtolnay/linkme/issues/41
https://github.com/CodeChain-io/intertrait/issues/6
