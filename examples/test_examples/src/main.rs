use async_process::{Command, Stdio};

#[macro_use]
pub extern crate integra8;

main_test! {
    // TODO: this should be automatically detected as default
    console_output: integra8_tree_formatter::TreeFormatter,
}


#[integration_test]
#[name("custom named for test")]
#[description("the test description")]
async fn test1() {
    
    Command::new("./basic_sample")
        .kill_on_drop(true)
      //  .stdout(Stdio::piped())
      //  .stderr(Stdio::piped())
      .output()
      .await
       .expect("failed");
}


#[integration_test]
#[name("custom named for test")]
#[description("the test description")]
async fn test2(ctx: crate::ExecutionContext) {

//async fn test2(&self) {
    
    Command::new("./basic_sample")
        .kill_on_drop(true)
      //  .stdout(Stdio::piped())
      //  .stderr(Stdio::piped())
      .output()
      .await
       .expect("failed");
}
