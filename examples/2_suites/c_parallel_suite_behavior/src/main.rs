
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}


#[suite]
#[parallel]
mod suite_with_mixed_concurrently_modes {

    // 1: test_1 and test_2 can be executed at the same time
    #[integration_test]
    #[parallel]
    fn test_1() { 

    }

    #[integration_test]
    #[parallel]
    fn test_2() { 

    }

    // 2: test_3 can only be executed after test 1 and test 2

    #[integration_test]
    #[sequential]
    fn test_3() { 

    }

    // 2: test_4 can only be executed after test 3

    #[integration_test]
    #[sequential]
    fn test_4() { 

    }

    // 3: test_5 and test_6 can be executed at the same time
    // but only after test_4 has finished
    #[integration_test]
    #[parallel]
    fn test_5() { 

    }

    #[integration_test]
    #[parallel]
    fn test_6() { 

    }
}