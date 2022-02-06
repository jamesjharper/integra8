
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
}

// `Tests`, `Setups`, `Tear downs` not belonging to a suite
// are at part of the "root" suite, and are run first. 
#[integration_test]
fn first_test() {
    println!("This test before any suites");
}

// Suites at the root level, are run after 
// root tests have completed
#[suite]
mod first_suite {

    // Same execution order applies inside a suite 
    // Setup ➞ Test ➞ Suites  ➞ Teardown
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

// Suites are run in the order they appear within file.
#[suite]
mod another_suite {
 
    #[integration_test]
    fn test1() {
        println!("Then another_suite::test_1 finally 1 is called");
    }
}

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


#[suite]
mod suite_with_internal_namespaces {
    #[integration_test]
    fn test_1() {
        
    }

    mod internal_namespace {
        #[integration_test]
        fn test_2() {
            println!("This test still belongs to \"suite_with_internal_namespaces\"");
        }
    }
}