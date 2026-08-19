use crate::host::{path::HandlerPath, HostImpl};
use http::StatusCode;
use http_body_util::BodyExt;
use hyper_body_utils::HttpBody;
use vetis::host::{handler_fn, Host, HostConfig};

#[tokio::test]
async fn test_add_host() -> Result<(), Box<dyn std::error::Error>> {
    let config = HostConfig::builder()
        .hostname("localhost")
        .root_directory("src/tests".into())
        .build()
        .unwrap();

    let mut host = HostImpl::new(config);
    host.add_path(
        HandlerPath::builder()
            .uri("/")
            .handler(handler_fn(|_request| async move {
                Ok(vetis::Response::builder()
                    .status(StatusCode::OK)
                    .text("Hello, world!"))
            }))
            .build()
            .unwrap(),
    );

    assert_eq!(
        host.config()
            .hostname(),
        "localhost"
    );

    Ok(())
}

#[tokio::test]
async fn test_handle_request() -> Result<(), Box<dyn std::error::Error>> {
    let config = HostConfig::builder()
        .hostname("localhost")
        .root_directory("src/tests".into())
        .build()
        .unwrap();

    let mut host = HostImpl::new(config);
    host.add_path(
        HandlerPath::builder()
            .uri("/")
            .handler(handler_fn(|_request| async move {
                Ok(vetis::Response::builder()
                    .status(StatusCode::OK)
                    .text("Hello, world!"))
            }))
            .build()
            .unwrap(),
    );

    assert_eq!(
        host.config()
            .hostname(),
        "localhost"
    );

    let body = HttpBody::from_text("Hello, world!");

    let request = http::Request::builder()
        .uri("/")
        .body(body)
        .unwrap();

    let (parts, body) = request.into_parts();

    let request = vetis::Request::from_parts(parts, body);

    let response = host
        .route(request)
        .await?;

    let (parts, body) = response
        .into_inner()
        .into_parts();
    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(
        body.collect()
            .await
            .unwrap()
            .to_bytes()
            .as_ref(),
        b"Hello, world!"
    );

    Ok(())
}
