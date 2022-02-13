use reqwest;

#[macro_use]
pub extern crate integra8;

main_test! {
    console_output: integra8_serde_formatter::SerdeFormatter,
    parameters : {
        #[structopt(long = "target-url", default_value = "https://httpbin.org/ip")]
        pub url: String,
    }
}


#[integration_test]
async fn httpbin_should_reply_200_ok(ctx : crate::ExecutionContext) {

    #[cfg(feature = "tokio-runtime")]
    let response = reqwest::get(&ctx.parameters.app.url).await.unwrap();

    // reqwest does not support async-std, so blocking must be used instead.
    // Its recommended to use async for these types of tests, as 
    // integra8 will run other tests while this test waits for a response 
    #[cfg(feature = "async-std-runtime")]
    let response = reqwest::blocking::get(&ctx.parameters.app.url).unwrap();
    assert_eq!(response.status(), 200, "Expected http 200 response");
}
