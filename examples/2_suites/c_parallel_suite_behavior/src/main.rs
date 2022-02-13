
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

/*
# Graphical representation of test order in this example

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
             ║  suite_3   ║
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
*/

#[suite]
#[sequential]
mod suite_1 {
    use super::*;

    #[integration_test]
    async fn test_1() { 
        assert_completion_order_was!( 5/*th*/);
    }

    #[integration_test]
    async fn test_2() { 
        assert_completion_order_was!( 6/*th*/);
    }
}

#[suite]
#[parallel]
mod suite_2 {
    use super::*;

    #[integration_test]
    async fn test_1() { 
       sleep(Duration::from_millis(100)).await;
       assert_completion_order_was!( 2/*nd*/);
    }

    #[integration_test]
    fn test_2() { 
        assert_completion_order_was!( 3/*rd*/);
    }
}

#[suite]
#[parallel]
mod suite_3 {
    use super::*;

    #[integration_test]
    fn test_1() { 
        assert_completion_order_was!( 1/*st*/);
    }

    #[integration_test]
    async fn test_2() { 
        sleep(Duration::from_millis(200)).await;
        assert_completion_order_was!( 4/*th*/);
    }
}