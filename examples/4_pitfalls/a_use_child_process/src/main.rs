#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
    use_child_process: false,
}

#[integration_test]
fn hello_world_test() {
    eprintln!("Hello world!");
}
