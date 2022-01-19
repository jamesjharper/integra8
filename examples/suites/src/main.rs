
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

#[suite]
#[name("Suites can have custom names")]
#[description("
Suites can contain tests, setups, tear downs and other suites within them.

")]
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
#[description("
There can also be multiple suites in one source file, and are run in order they appear.
")]
mod another_suite {
 
    #[integration_test]
    fn test1() {
        println!("Then finally 1 is called");
    }
}

