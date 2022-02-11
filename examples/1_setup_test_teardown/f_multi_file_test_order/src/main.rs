
#[macro_use]
pub extern crate integra8;

// regardless of the order the mod are defined in file
// integra8 will run these components defined in other files 
// in lexicographical order 

mod b_test_mod;
mod a_test_mod;

// # Test main
// Test main is required to setup the application entrypoint and bootstrap the test framework
main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
}

#[integration_test]
fn test_c() {
    println!("Test C was called first");
}
