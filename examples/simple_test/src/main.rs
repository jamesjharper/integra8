
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}


#[integration_test]
#[name("custom named for test")]
#[description("the test description")]
async fn test1() {
    assert_eq!(false, true)
}