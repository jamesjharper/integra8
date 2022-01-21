
#[macro_use]
pub extern crate integra8;

main_test! {
    // Limit the number of components which can run at the same time;
    max_concurrency: Auto, 

    // When enabled, all test run in their own process.
    use_child_process: true,

    // Global default concurrency mode for suites
    suite_concurrency: Sequential,

    // Global default concurrency mode for testes
    test_concurrency: Sequential,

    // Global default time out for setups
    setup_critical_threshold_seconds: 20,

    // Global default time out for tear downs
    tear_down_critical_threshold_seconds: 20,

    // Global default warning threshold for tests
    test_warn_threshold_seconds: 30,

    // default time out for tests
    test_critical_threshold_seconds: 30

    // TODO: this should be automatically detected as default
    console_output: integra8_tree_formatter::TreeFormatter,
}

#[suite]
#[parallel]

#[test_warn_threshold_seconds(50)]
#[test_critical_threshold_seconds(60)]
#[setup_critical_threshold_seconds(60)]
#[name("A concise name that tells anyone what this suite is doing")]
#[description(
"A description which can be useful for adding exact details, assumptions or context behind why this suite exists"
)]
#[tear_down_critical_threshold_seconds(60)]
mod suite1 {

    #[setup]
    #[parallel]
    fn setup1() {

    }

    #[setup]
    #[parallel]
    fn setup2() {
        
    }

    #[integration_test]
    #[parallel]
    fn test1() { 

    }

    #[integration_test]
    #[parallel]
    fn test2() { 

    }

    #[integration_test]
    #[sequential]
    fn test3() { 

    }

    mod inner_mod {
        #[integration_test]
        fn test_within_a_nested_mod() {
    
        }
    }




    #[teardown]
    #[parallel]
    fn teardown() {

    }

    #[teardown]
    #[parallel]
    fn teardown() {

    }
}

#[suite]
#[parallel]
mod suite_with_mixed_concurrently_modes {

    // 1: test1 and test2 can be executed at the same time

    #[integration_test]
    #[parallel]
    fn test1() { 

    }

    #[integration_test]
    #[parallel]
    fn test2() { 

    }

    // 2: test3 can only be executed after test 1 and test 2

    #[integration_test]
    #[sequential]
    fn test3() { 

    }

    #[integration_test]
    #[sequential]
    fn test3() { 

    }

    #[integration_test]
    #[parallel]
    fn test4() { 

    }

    #[integration_test]
    #[parallel]
    fn test5() { 

    }
}

#[suite]
#[parallel_tests]
mod another_suite {
 
    #[integration_test]
    fn test1() {
        println!("Then finally 1 is called");
    }

    mod another_suite {
 
        #[integration_test]
        fn test1() {
            println!("Then finally 1 is called");
        }
    }
}


#[suite]
mod matryoshka_suite {
 
    #[integration_test]
    fn test2() {
        
    }

    #[suite]
    mod inner_matryoshka_suite {
    
        #[integration_test]
        fn test2() {
            
        }

        #[suite]
        mod inner_most_matryoshka_suite {
        
            #[integration_test]
            fn test3() {
                
            }
        }
    }
}


#[suite]
mod suite_with_internal_namespaces {
    #[integration_test]
    fn test2() {
        
    }

    mod internal_namespace {
        #[integration_test]
        fn another_test() {
            
        }
    }
}