use http::{HeaderMap, StatusCode};

use crate::http::static_response;

#[test]
fn test_static_response_basic() {
    let response = static_response(StatusCode::OK, None, "Hello, World!".to_string());

    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_static_response_with_headers() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        "text/plain"
            .parse()
            .unwrap(),
    );

    let response =
        static_response(StatusCode::OK, Some(headers.clone()), "Hello, World!".to_string());

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("content-type"),
        headers.get("content-type")
    );
}

#[test]
fn test_static_response_different_status() {
    let response = static_response(StatusCode::NOT_FOUND, None, "Not Found".to_string());

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_static_response_empty_body() {
    let response = static_response(StatusCode::NO_CONTENT, None, "".to_string());

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_static_response_with_custom_headers() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-custom-header",
        "custom-value"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "server",
        "vetis"
            .parse()
            .unwrap(),
    );

    let response = static_response(StatusCode::OK, Some(headers.clone()), "Test".to_string());

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("x-custom-header"),
        headers.get("x-custom-header")
    );
    assert_eq!(
        response
            .headers()
            .get("server"),
        headers.get("server")
    );
}

#[test]
fn test_static_response_internal_server_error() {
    let response = static_response(StatusCode::INTERNAL_SERVER_ERROR, None, "Error".to_string());

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_static_response_multiple_headers() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-header-1",
        "value1"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "x-header-2",
        "value2"
            .parse()
            .unwrap(),
    );
    headers.insert(
        "x-header-3",
        "value3"
            .parse()
            .unwrap(),
    );

    let response = static_response(StatusCode::OK, Some(headers.clone()), "Test".to_string());

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .len(),
        3
    );
}
