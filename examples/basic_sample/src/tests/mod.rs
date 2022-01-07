use integra8::{integration_suite, integration_test, setup, teardown};

use std::{thread, time};

#[setup]
fn setup() {
    println!("setting up!");
}

#[integration_test]
fn test1() {
    println!("Running basic test 2");
}

#[integration_test]
#[allow_fail]
fn test2() {
    assert_eq!(true, false);
}

#[integration_test]
fn test3() {
    println!("Running basic test 3");
}

#[teardown]
fn tear_down() {
    println!("tearing down !");
}

#[integration_suite]
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

    #[integration_suite]
    mod another_nested_test_suite {
        use super::*;
        #[integration_test]
        //#[allow_fail]
        fn first() {
            //assert_eq!(false, true);
        }

        #[integration_test]
        //  #[allow_fail]
        fn second() {
            //assert_eq!(true, false);
        }

        #[integration_suite]
        mod another_nested_test_suite {
            use super::*;
            #[integration_test]
            fn first() {
                //assert_eq!(false, true);
            }

            #[integration_test]
            fn second() {}

            #[integration_suite]
            #[allow_fail]
            mod suite_which_is_aloud_to_fail {
                use super::*;
                #[integration_test]
                //#[allow_fail]
                fn first() {}

                #[integration_test]
                //  #[allow_fail]
                fn second() {
                    assert_eq!(true, false);
                }
            }

            #[integration_suite]
            mod suite_which_should_warning {
                use super::*;
                #[integration_test]
                //#[allow_fail]
                fn first() {}

                #[integration_test]
                #[warn_threshold_seconds(1)]
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

            #[integration_suite]
            #[allow_fail]
            mod suite_should_fail {
                use super::*;

                #[setup]
                fn setup() {
                    println!("setting up!");
                }

                #[integration_test]
                fn first() {}

                #[integration_test]
                fn second() {
                    assert_eq!(true, false);
                }

                #[teardown]
                fn tear_down_which_should_run() {
                    println!("tearing down !");
                }
            }

            #[integration_suite]
            mod suite_should_also_skipped {
                use super::*;
                #[integration_test]
                #[ignore]
                fn first() {}

                #[integration_test]
                #[ignore]
                fn second() {
                    assert_eq!(true, false);
                }
            }
        }
    }
}
