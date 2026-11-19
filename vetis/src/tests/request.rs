use crate::Request;
use caramelo::{expect, matchers::eq};
use http::{Method, Uri, Version};
use hyper_body_utils::HttpBody;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};

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
        .body(HttpBody::text("test body"))
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
fn test_request_version() {
    let http_request = http::Request::builder()
        .method(Method::GET)
        .version(Version::HTTP_11)
        .body(HttpBody::empty())
        .unwrap();

    let http_request = Request::new(http_request);
    expect(http_request.version()).to_be(eq(Version::HTTP_11));
}

#[test]
fn test_request_client_ip() {
    let http_request = http::Request::builder()
        .method(Method::GET)
        .version(Version::HTTP_11)
        .body(HttpBody::empty())
        .unwrap();

    let http_request =
        Request::new(http_request).with_client_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)));
    expect(http_request.client_ip()).to_be(eq(Some(&IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)))));
}

#[test]
fn test_request_root_directory() {
    let http_request = http::Request::builder()
        .method(Method::GET)
        .version(Version::HTTP_11)
        .body(HttpBody::empty())
        .unwrap();

    let path: PathBuf = "/tmp".into();
    let http_request = Request::new(http_request).with_root_directory(Some(path.clone()));
    expect(http_request.root_directory()).to_be(eq(Some(&path)));
}

#[test]
fn test_mutate_uri() {
    let mut request = create_test_request(Method::POST, "/api/users");
    *request.uri_mut() = "/other/api"
        .parse()
        .unwrap();
    expect(request.uri().path()).to_be(eq("/other/api"));
}

#[test]
fn test_request_as_ref() {
    let http_request = http::Request::builder()
        .uri("https://localhost:8080/api/users")
        .method(Method::GET)
        .version(Version::HTTP_11)
        .header("Accept", "text/html")
        .body(HttpBody::empty())
        .unwrap();

    let http_request = Request::new(http_request)
        .with_client_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)))
        .with_root_directory(Some("/tmp".into()));

    let req_ref = http_request.as_ref();
    let req_path = "https://localhost:8080/api/users"
        .parse::<Uri>()
        .unwrap();
    let header = http::header::ACCEPT;
    expect(req_ref.uri).to_be(eq(&req_path));
    expect(req_ref.method).to_be(eq(&Method::GET));
    expect(req_ref.version).to_be(eq(Version::HTTP_11));
    expect(req_ref.host).to_be(eq(Some("localhost")));
    expect(req_ref.path).to_be(eq("/api/users"));
    expect(
        req_ref
            .headers
            .get(header)
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .to_be(eq("text/html"));
}
