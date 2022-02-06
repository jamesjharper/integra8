#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
}

#[suite]
mod test_some_user_actions {

    fn suite_user_name(ctx : &crate::ExecutionContext) -> String {

        // ctx.description.parent_id() will always return the same unique number when called within the same suite
        format!("user_{}", ctx.description.parent_id().as_unique_number())
    }

    fn item_name(ctx : &crate::ExecutionContext) -> String {
        // ctx.description.id() will always return a number which is unique to all other tests
        format!("item_{}", ctx.description.id().as_unique_number())
    }

    #[setup]
    fn setup(ctx : crate::ExecutionContext) {
        println!("Creating user \"{}\"", suite_user_name(&ctx));
        
        // Create user in system under test ... 
    }

    #[integration_test]
    fn do_some_action(ctx : crate::ExecutionContext) {
        println!("User \"{}\" performs some action on item \"{}\"", suite_user_name(&ctx), item_name(&ctx));
        
        // Have user x do something ... 
    }

    #[integration_test]
    fn do_some_something_else(ctx : crate::ExecutionContext) {
        println!("User \"{}\" performs another action on item \"{}\"", suite_user_name(&ctx), item_name(&ctx));
        
        // Have user x do something else ... 
    }

    #[teardown]
    fn teardown(ctx : crate::ExecutionContext) {
        println!("Removing user \"{}\"", suite_user_name(&ctx));
        
        // Remove the user in system under test ...
    }
}
