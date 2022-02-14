
#[macro_use]
pub extern crate integra8;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
    max_concurrency: 2,
    // Needed so we can manage global state
    use_child_process: false, 

}

static TEST_COMPLETE_ORDER: AtomicUsize = AtomicUsize::new(1);

macro_rules! assert_completion_order_was {
    ($order:expr) => {
        assert_eq!($order, TEST_COMPLETE_ORDER.fetch_add(1, Ordering::SeqCst));
    }
}

async fn sleep(duration : Duration) {
    #[cfg(feature = "tokio-runtime")]
    tokio::time::sleep(duration).await;
    
    #[cfg(feature = "async-std-runtime")]
    async_std::task::sleep(duration).await;
}


// exact implementation details can be found at ./core/scheduling/src/component.rs

/*
# Graphical representation of test order in this example

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
    [test_5]          [test_6] 
        │                │
        └───────┬────────┘
                │
                :
               end
*/

// 1: test_1 and test_2 are executed at the same time
#[integration_test]
#[parallel]
async fn test_1() { 
    sleep(Duration::from_millis(100)).await;
    assert_completion_order_was!( 2/*nd*/);
}

#[integration_test]
#[parallel]
fn test_2() { 
    assert_completion_order_was!( 1/*st*/);
}

// 2: test_3 can only be executed after test 1 and test 2 completes

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited
async fn test_3() { 
    sleep(Duration::from_millis(100)).await;
    assert_completion_order_was!( 3/*rd*/);
}

// 2: test_4 can only be executed after test 3 completes

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited
fn test_4() { 
    assert_completion_order_was!( 4/*th*/);
}

// 3: test_5 and test_6 can be executed at the same time
// but only after test_4 has completes
#[integration_test]
#[parallel]
fn test_5() { 
    assert_completion_order_was!( 5/*th*/);
}

#[integration_test]
#[parallel]
async fn test_6() { 
    sleep(Duration::from_millis(100)).await;
    assert_completion_order_was!( 6/*th*/);
}