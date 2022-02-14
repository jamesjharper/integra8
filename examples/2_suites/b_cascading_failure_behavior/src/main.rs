
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
}

#[suite]
mod suite_which_will_fail {

    // Suites with #[allow_fail] that fail, 
    // will not propagate their failure to their parent suite.
    // However internally to this suite,
    // all nested components will be aborted
    #[suite]
    #[allow_fail]
    mod allow_fail_suite {
    
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
            assert!(false, "Real Fail")
        }

        // This test will not run and will be 
        // indicated as `ComponentResult::DidNotRun(DidNotRunReason::ParentFailure)`
        #[integration_test]
        fn test_3() {
            assert!(false, "Cant fail if you never try")
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
    mod not_allow_fail_suite {
    
        #[integration_test]
        fn test_1() {
            assert!(false, "Fail")
        }

        #[integration_test]
        fn test_2() {
            println!("Is never called");
        }
    }

    // Tear downs are always run!
    #[teardown]
    fn teardown() {
        println!("Teardown is called");
    }
}


