
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}


#[integration_test]
#[parallelizable]
async fn parallelizable_test_1() {
    println!("parallelizable_setup_2 could run at the same time as this test");
}

#[integration_test]
#[parallelizable]
async fn parallelizable_test_2() {
    println!("parallelizable_setup_1 could run at the same time as this test");
}

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited
async fn sequential_test_1() {
    println!("This test will not run at the same time as any other test");
}

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited
async fn sequential_test_2() {
    println!("This test will only run after sequential_test_1 has completed");
}
