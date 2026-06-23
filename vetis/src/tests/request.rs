use crate::Request;
use http::Method;
use hyper_body_utils::HttpBody;

fn create_test_request(method: Method, uri: &str) -> Request {
    let http_request = http::Request::builder()
        .method(method.clone())
        .uri(uri)
        .body(HttpBody::empty())
        .unwrap();
    let (parts, body) = http_request.into_parts();
    Request::from_parts(parts, body)
}

#[test]
fn test_request_from_parts() {
    let request = create_test_request(Method::GET, "/test");

    assert_eq!(request.method(), &Method::GET);
    assert_eq!(
        request
            .uri()
            .to_string(),
        "/test"
    );
}

#[test]
fn test_request_uri() {
    let request = create_test_request(Method::POST, "/api/users");

    assert_eq!(
        request
            .uri()
            .to_string(),
        "/api/users"
    );
}

#[test]
fn test_request_headers() {
    let http_request = http::Request::builder()
        .method(Method::GET)
        .uri("/")
        .header("content-type", "application/json")
        .header("user-agent", "test-agent")
        .body(HttpBody::empty())
        .unwrap();
    let (parts, body) = http_request.into_parts();
    let request = Request::from_parts(parts, body);

    let headers = request.headers();
    assert_eq!(
        headers.get("content-type"),
        Some(
            &"application/json"
                .parse()
                .unwrap()
        )
    );
    assert_eq!(
        headers.get("user-agent"),
        Some(
            &"test-agent"
                .parse()
                .unwrap()
        )
    );
}

#[test]
fn test_request_headers_mut() {
    let mut request = create_test_request(Method::GET, "/");

    request
        .headers_mut()
        .insert(
            "x-custom-header",
            "custom-value"
                .parse()
                .unwrap(),
        );

    assert_eq!(
        request
            .headers()
            .get("x-custom-header"),
        Some(
            &"custom-value"
                .parse()
                .unwrap()
        )
    );
}

#[test]
fn test_request_method() {
    let methods = [Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH];

    for method in methods {
        let request = create_test_request(method.clone(), "/");
        assert_eq!(request.method(), &method);
    }
}

#[test]
fn test_request_into_parts() {
    let http_request = http::Request::builder()
        .method(Method::GET)
        .uri("/test")
        .header("x-test", "value")
        .body(HttpBody::from_text("test body"))
        .unwrap();
    let (parts, body) = http_request.into_parts();
    let request = Request::from_parts(parts, body);

    let (returned_parts, _returned_body) = request.into_parts();

    assert_eq!(returned_parts.method, Method::GET);
    assert_eq!(
        returned_parts
            .uri
            .to_string(),
        "/test"
    );
    assert_eq!(
        returned_parts
            .headers
            .get("x-test"),
        Some(
            &"value"
                .parse()
                .unwrap()
        )
    );
}

#[test]
#[should_panic(expected = "No request")]
fn test_request_panic_on_no_inner() {
    let request = Request { inner: None };
    request.method();
}
