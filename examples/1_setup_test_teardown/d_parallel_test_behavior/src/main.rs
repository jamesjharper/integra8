
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

// exact implementation details can be found at ./core/scheduling/src/component.rs

/*
# Graphical representation of test order in this example

              start
                :
                │
        ┌───────┴────────┐
        │                │
    [test_1]         [test_2]
        │                │
        └───────┬────────┘
                │
             [test_3]
                │
             [test_4]
                │
        ┌───────┴────────┐
        │                │
    [test_5]          [test_6] 
        │                │
        └───────┬────────┘
                │
                :
               end
*/

// 1: test_1 and test_2 are executed at the same time
#[integration_test]
#[parallel]
fn test_1() { 

}

#[integration_test]
#[parallel]
fn test_2() { 

}

// 2: test_3 can only be executed after test 1 and test 2 completes

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited
fn test_3() { 

}

// 2: test_4 can only be executed after test 3 completes

#[integration_test]
// #[sequential] By default all `Tests` `Setups` `Tear downs` and `Suites` are assumed to be `sequential` unless overridden using parameters or inherited
fn test_4() { 

}

// 3: test_5 and test_6 can be executed at the same time
// but only after test_4 has completes
#[integration_test]
#[parallel]
fn test_5() { 

}

#[integration_test]
#[parallel]
fn test_6() { 

}