use deboa::request::get;
use deboa_smol::Client;
use macro_rules_attribute::apply;
use smol_macros::test;
use vetis::{http::Response, virtual_host::handler_fn};
use vetis_macros::http;

use crate::common::{deboa_default_protocol, vetis_default_protocol};

async fn do_test_http() -> Result<(), Box<dyn std::error::Error>> {
    let handler = handler_fn(|_req| async move { Ok(Response::builder().text("Hello, World!")) });

    let mut server = http!(
        hostname => "localhost",
        root_directory => "src",
        protocol => vetis_default_protocol(),
        port => 8080,
        interface => "0.0.0.0",
        handler => handler
    )
    .await?;

    server
        .start()
        .await?;

    let client = Client::builder()
        .protocol(deboa_default_protocol())
        .build();

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
