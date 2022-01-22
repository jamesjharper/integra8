
#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_tree_formatter::TreeFormatter,
}

/*
# Graphical representation of test order in this example

                 start
                   :
                   │
         ┌─────────┴──────────┐
         │                    │
   ╔═════╧══════╗       ╔═════╧══════╗
   ║  suite_2   ║       ║   suite_3  ║
   ╟─────┬──────╢       ╟─────┬──────╢ 
   ║     │      ║       ║     │      ║
   ║  [test_1]  ║       ║  [test_1]  ║
   ║     │      ║       ║     │      ║
   ║  [test_2]  ║       ║  [test_2]  ║
   ║     │      ║       ║     │      ║
   ╚═════╪══════╝       ╚═════╪══════╝  
         │                    │
         └─────────┬──────────┘
                   │
             ╔═════╧══════╗
             ║  suite_3   ║
             ╟─────┬──────╢
             ║     │      ║
             ║  [test_1]  ║
             ║     │      ║
             ║  [test_2]  ║
             ║     │      ║
             ╚═════╪══════╝
                   │
                   :
                  end
*/

#[suite]
#[sequential]
mod suite_1 {

    #[integration_test]
    fn test_1() { 
        println!("Nothing but suite_1::test_1 will run right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Nothing but suite_1::test_2 will run right now")
    }
}

#[suite]
#[parallel]
mod suite_2 {

    #[integration_test]
    fn test_1() { 
        println!("Any thing in suite 3 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite 3 could be running right now")
    }
}

#[suite]
#[parallel]
mod suite_3 {

    #[integration_test]
    fn test_1() { 
        println!("Any thing in suite 2 could be running right now")
    }

    #[integration_test]
    fn test_2() { 
        println!("Any thing in suite 2 could be running right now")
    }
}