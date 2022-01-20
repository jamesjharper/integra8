
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

#[suite]
mod suite1 {
    #[setup]
    fn setup() {
        println!("Setup is called first");
    }

    #[integration_test]
    fn test1() {
        println!("Then test 1 is called");
    }

    #[teardown]
    fn teardown() {
        println!("And teardown is called");
    }
}

#[suite]
mod another_suite {
 
    #[integration_test]
    fn test1() {
        println!("Then finally 1 is called");
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