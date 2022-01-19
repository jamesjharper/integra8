use integra8::{integration_suite, integration_test, setup, teardown};

use std::{thread, time};

#[setup]
#[name("custom named for setup")]
#[description("the setup description")]
fn setup() {
    println!("setting up!");
}

#[integration_test]
#[name("custom named for test")]
#[description("the test description")]
fn test1() {
    println!("Running basic test 2");
}

#[integration_test]
#[allow_fail]
fn test2() {
    assert_eq!(true, true);
}

#[integration_test]
fn test3() {
    println!("Running basic test 3");
}

#[teardown]
#[name("custom named for tear down")]
#[description("the tear down description")]
fn tear_down(_ctx: crate::ExecutionContext<'_>) {
    assert_eq!(true, true);
}

#[suite]
#[name("custom named suite")]
#[description("the suite description")]
mod nested_test_suite {
    use super::*;
    #[integration_test]
    fn nested_test_1() {
        //assert_eq!(true, false);
    }

    #[integration_test]
    //  #[allow_fail]
    fn nested_test_2() {
        //assert_eq!(true, false);
    }

    #[suite]
    mod another_nested_test_suite {
        use super::*;
        #[integration_test]
        #[allow_fail]
        fn first() {
            assert_eq!(false, false);
        }

        #[integration_test]
        //  #[allow_fail]
        fn second() {
            //assert_eq!(true, false);
        }

        #[suite]
        mod another_nested_test_suite {
            use super::*;
            #[integration_test]
            fn first() {
                //assert_eq!(false, true);
            }

            #[integration_test]
            fn second() {}

            #[integration_suite]
            //#[allow_fail]
            mod suite_which_is_aloud_to_fail {
                use super::*;
                #[integration_test]
                //#[allow_fail]
                fn first() {}

                #[integration_test]
                #[description("the test description")]
                //  #[allow_fail]
                fn second() {
                    assert_eq!(true, true);
                }
            }

            #[suite]
            mod suite_which_should_warning {
                use super::*;
                #[integration_test]
                //#[allow_fail]
                fn first() {}

                #[integration_test]
                
                fn should_have_time_warning() {
                   thread::sleep(time::Duration::from_millis(1100));
                }
            }

            #[integration_suite]
            #[ignore]
            mod suite_should_be_ignored {
                use super::*;
                #[integration_test]
                //#[allow_fail]
                fn first() {}

                #[integration_test]
                #[critical_threshold_seconds(1)]
                fn should_have_time_error() {
                    //thread::sleep(time::Duration::from_millis(1100));
                }
            }

            #[suite]
            #[allow_fail]
            mod suite_should_fail {
                use super::*;

                #[setup]
                fn setup() {
                    println!("setting up!");
                }

                #[integration_test]
                #[description(indoc::indoc! 
                {"
                    the test description
                    on more then
                    one
                    line
                "})]
                fn second() {
                    assert_eq!(true, true);
                }

                #[integration_test]
                fn first() {}


                

                #[teardown]
                fn tear_down_which_should_run() {
                    println!("tearing down !");
                }
            }

            #[suite]
            mod suite_should_also_skipped {
                use super::*;
                #[integration_test]
                #[ignore]
                fn first() {}

                #[integration_test]
                fn second() {
                    assert_eq!(true, true);
                }
            }
        }
    }
}
