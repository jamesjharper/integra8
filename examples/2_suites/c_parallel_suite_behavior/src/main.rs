
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

#[suite]
#[parallel]
mod suite_1 {

    #[integration_test]
    fn test_1() { 
        println!("Any thing in suite 2 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite 2 could be running right now")
    }
}

#[suite]
#[parallel]
mod suite_2 {

    #[integration_test]
    fn test_1() { 
        println!("Any thing in suite 1 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite 1 could be running right now")
    }
}