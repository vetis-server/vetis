use crate::Response;
use http::StatusCode;
use hyper_body_utils::HttpBody;

#[test]
fn test_response_builder_default() {
    let builder = Response::builder();
    let response = builder.text("Hello");

    let inner = response.into_inner();
    assert_eq!(inner.status(), StatusCode::OK);
    assert_eq!(inner.version(), http::Version::HTTP_11);
}

#[test]
fn test_response_builder_status() {
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .text("Not found");

    let inner = response.into_inner();
    assert_eq!(inner.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_response_builder_version() {
    let response = Response::builder()
        .version(http::Version::HTTP_2)
        .text("Response");

    let inner = response.into_inner();
    assert_eq!(inner.version(), http::Version::HTTP_2);
}

#[test]
fn test_response_builder_header() {
    let response = Response::builder()
        .header(
            "content-type",
            "application/json"
                .parse()
                .unwrap(),
        )
        .header(
            "x-custom",
            "custom-value"
                .parse()
                .unwrap(),
        )
        .text("{}");

    let inner = response.into_inner();
    assert_eq!(
        inner
            .headers()
            .get("content-type"),
        Some(
            &"application/json"
                .parse()
                .unwrap()
        )
    );
    assert_eq!(
        inner
            .headers()
            .get("x-custom"),
        Some(
            &"custom-value"
                .parse()
                .unwrap()
        )
    );
}

#[test]
fn test_response_builder_headers() {
    let mut headers = http::HeaderMap::new();
    headers.insert(
        "content-type",
        "text/html"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "cache-control",
        "no-cache"
            .parse()
            .unwrap(),
    );

    let response = Response::builder()
        .headers(headers)
        .text("<html></html>");

    let inner = response.into_inner();
    assert_eq!(
        inner
            .headers()
            .get("content-type"),
        Some(
            &"text/html"
                .parse()
                .unwrap()
        )
    );
    assert_eq!(
        inner
            .headers()
            .get("cache-control"),
        Some(
            &"no-cache"
                .parse()
                .unwrap()
        )
    );
}

#[test]
fn test_response_builder_empty() {
    let response = Response::builder().empty();

    let inner = response.into_inner();
    assert_eq!(inner.status(), StatusCode::OK);
}

#[test]
fn test_response_builder_text() {
    let response = Response::builder().text("Hello, World!");

    let inner = response.into_inner();
    assert_eq!(inner.status(), StatusCode::OK);
}

#[test]
fn test_response_builder_bytes() {
    let response = Response::builder().bytes(b"binary data");

    let inner = response.into_inner();
    assert_eq!(inner.status(), StatusCode::OK);
}

#[test]
fn test_response_builder_body() {
    let body = HttpBody::from_text("custom body");
    let response = Response::builder().body(body);

    let inner = response.into_inner();
    assert_eq!(inner.status(), StatusCode::OK);
}

#[test]
fn test_response_builder_chain() {
    let response = Response::builder()
        .status(StatusCode::CREATED)
        .version(http::Version::HTTP_2)
        .header(
            "location",
            "/resource/123"
                .parse()
                .unwrap(),
        )
        .header(
            "content-type",
            "application/json"
                .parse()
                .unwrap(),
        )
        .text(r#"{"id": 123}"#);

    let inner = response.into_inner();
    assert_eq!(inner.status(), StatusCode::CREATED);
    assert_eq!(inner.version(), http::Version::HTTP_2);
    assert_eq!(
        inner
            .headers()
            .get("location"),
        Some(
            &"/resource/123"
                .parse()
                .unwrap()
        )
    );
    assert_eq!(
        inner
            .headers()
            .get("content-type"),
        Some(
            &"application/json"
                .parse()
                .unwrap()
        )
    );
}

#[test]
fn test_response_into_inner() {
    let response = Response::builder().text("test");
    let inner = response.into_inner();

    assert_eq!(inner.status(), StatusCode::OK);
    assert_eq!(inner.version(), http::Version::HTTP_11);
}

#[test]
fn test_response_headers_replace() {
    let response = Response::builder()
        .header(
            "x-old",
            "old-value"
                .parse()
                .unwrap(),
        )
        .headers({
            let mut headers = http::HeaderMap::new();
            headers.insert(
                "x-new",
                "new-value"
                    .parse()
                    .unwrap(),
            );
            headers
        })
        .text("test");

    let inner = response.into_inner();
    assert!(inner
        .headers()
        .get("x-old")
        .is_none());
    assert_eq!(
        inner
            .headers()
            .get("x-new"),
        Some(
            &"new-value"
                .parse()
                .unwrap()
        )
    );
}

#[test]
fn test_response_multiple_status_codes() {
    let status_codes = [
        StatusCode::OK,
        StatusCode::CREATED,
        StatusCode::ACCEPTED,
        StatusCode::NO_CONTENT,
        StatusCode::BAD_REQUEST,
        StatusCode::UNAUTHORIZED,
        StatusCode::FORBIDDEN,
        StatusCode::NOT_FOUND,
        StatusCode::INTERNAL_SERVER_ERROR,
    ];

    for status in status_codes {
        let response = Response::builder()
            .status(status)
            .text("test");
        let inner = response.into_inner();
        assert_eq!(inner.status(), status);
    }
}
