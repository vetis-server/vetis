use deboa::request::get;

use macro_rules_attribute::apply;
use smol_macros::test;

use deboa_smol::Client;
use vetis::http::Response;

use crate::{http, server::virtual_host::handler_fn};

async fn do_test_http() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        hostname => "localhost",
        root_directory => "src",
        protocol => vetis::Protocol::Http2,
        port => 8080,
        interface => "0.0.0.0",
        handler => handler
    )
    .await?;

    server
        .start()
        .await?;

    let client = Client::default();

    let response = get("http://localhost:8080")?
        .send_with(&client)
        .await?;

    assert_eq!(response.status(), 200);
    assert_eq!(
        response
            .text()
            .await?,
        "Hello, World!"
    );

    server
        .stop()
        .await?;

    Ok(())
}

#[apply(test!)]
async fn test_http() -> Result<(), Box<dyn std::error::Error>> {
    do_test_http().await
}
