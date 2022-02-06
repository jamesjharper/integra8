
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
}

#[suite]
#[parallel]
mod suite1 {
    
    #[setup]
    #[sequential]
    fn setup() {
        println!("Will only be #[sequential] in the context its own suite");
    }

    #[integration_test]
    #[sequential]
    fn test1() {
        println!("Then test 1 is called");
    }


    #[teardown]
    #[sequential]
    fn teardown() {
        println!("And teardown is called");
    }
}


#[suite]
#[parallel]
mod another_suite {

    #[integration_test]
    #[sequential]
    fn test1() {
        println!("Then finally 1 is called");
    }
}