
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

    #[allow_fail]
    #[integration_test]
    fn test1() {
        assert!(false, "Fail")
    }

    #[suite]
    #[allow_fail]
    mod another_suite {
    
        #[integration_test]
        fn test1() {
            assert!(false, "Fail")
        }

        #[integration_test]
        fn test2() {
            println!("Is never run");
        }
    }

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

    #[teardown]
    fn teardown() {
        println!("And teardown is called");
    }
}



