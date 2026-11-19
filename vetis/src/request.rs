use http::request::Parts;
use hyper_body_utils::HttpBody;
use serde::Serialize;
use std::{net::IpAddr, path::PathBuf};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// Serializable version of Request
pub struct RequestRef<'a> {
    /// The request's root directory
    pub root_directory: Option<&'a PathBuf>,

    /// The request's client ip
    pub client_ip: Option<&'a IpAddr>,

    /// The request's method
    #[serde(with = "http_serde_ext::method")]
    pub method: &'a http::Method,

    /// The request's URI
    #[serde(with = "http_serde_ext::uri")]
    pub uri: &'a http::Uri,

    /// The request's path
    pub path: &'a str,

    /// The request's query
    pub query: Option<&'a str>,

    /// The request's URI schem
    pub scheme: Option<&'a str>,

    /// The request's URI schem
    pub host: Option<&'a str>,

    /// The request's version
    #[serde(with = "http_serde_ext::version")]
    pub version: http::Version,

    /// The request's headers
    #[serde(with = "http_serde_ext::header_map")]
    pub headers: &'a http::HeaderMap,
}

#[derive(Default)]
/// HTTP request wrapper supporting multiple protocols.
///
/// The `Request` struct provides a unified interface for handling HTTP requests
/// from different protocols (HTTP/1, HTTP/2, HTTP/3). It abstracts away the protocol-specific
/// details while providing access to common request properties.
///
/// # Examples
///
/// ```rust,no_run
/// use vetis::{Request, Response, VetisResult};
///
/// // In a request handler:
/// async fn handler(request: Request) -> VetisResult<Response> {
///     let method = request.method();
///     let uri = request.uri();
///     let user_agent = request.headers().get("user-agent");
///
///     // Process request...
///
///     Ok(Response::builder()
///         .status(http::StatusCode::OK)
///         .text("Hello"))
/// }
/// ```
pub struct Request {
    pub(crate) inner: http::Request<HttpBody>,
    pub(crate) root_directory: Option<PathBuf>,
    pub(crate) client_ip: Option<IpAddr>,
}

impl Request {
    /// Create a `Request` out of http::Request
    pub fn new(inner: http::Request<HttpBody>) -> Self {
        Self { inner, root_directory: None, client_ip: None }
    }

    /// Creates a `Request` from an HTTP/1 or HTTP/2 request.
    ///
    /// This is used internally by the server to wrap incoming HTTP requests.
    pub fn from_parts(parts: Parts, body: HttpBody) -> Self {
        Self {
            inner: http::Request::from_parts(parts, body),
            root_directory: None,
            client_ip: None,
        }
    }

    /// Allow set root directory
    pub fn with_root_directory(mut self, path: Option<PathBuf>) -> Self {
        self.root_directory = path;
        self
    }

    /// Allow set client ip
    pub fn with_client_ip(mut self, client_ip: impl Into<IpAddr>) -> Self {
        self.client_ip = Some(client_ip.into());
        self
    }

    /// Returns inner request
    pub fn inner(&self) -> &http::Request<HttpBody> {
        &self.inner
    }

    /// Returns mutable version of inner request
    pub fn inner_mut(&mut self) -> &mut http::Request<HttpBody> {
        &mut self.inner
    }

    /// Returns root directory.
    pub fn root_directory(&self) -> Option<&PathBuf> {
        self.root_directory
            .as_ref()
    }

    /// Returns client ip.
    pub fn client_ip(&self) -> Option<&IpAddr> {
        self.client_ip
            .as_ref()
    }

    /// Returns request URI.
    pub fn uri(&self) -> &http::Uri {
        self.inner.uri()
    }

    /// Returns request URI.
    pub fn uri_mut(&mut self) -> &mut http::Uri {
        self.inner.uri_mut()
    }

    /// Returns request headers.
    pub fn headers(&self) -> &http::HeaderMap {
        self.inner.headers()
    }

    /// Returns request headers (mutable).
    pub fn headers_mut(&mut self) -> &mut http::HeaderMap {
        self.inner
            .headers_mut()
    }

    /// Returns HTTP method.
    pub fn method(&self) -> &http::Method {
        self.inner.method()
    }

    /// Returns HTTP version.
    pub fn version(&self) -> http::Version {
        self.inner.version()
    }

    /// Returns request body.
    pub fn body(&self) -> &HttpBody {
        self.inner.body()
    }

    /// Convert the request into parts.
    pub fn into_parts(self) -> (http::request::Parts, HttpBody) {
        let (parts, body) = self
            .inner
            .into_parts();
        (parts, body)
    }

    /// Returns a serializable version of inner request
    pub fn as_ref<'a>(&'a self) -> RequestRef<'a> {
        RequestRef {
            root_directory: self
                .root_directory
                .as_ref(),
            client_ip: self
                .client_ip
                .as_ref(),
            method: self.method(),
            uri: self.uri(),
            scheme: self
                .uri()
                .scheme_str(),
            host: self.uri().host(),
            path: self.uri().path(),
            query: self.uri().query(),
            version: self.version(),
            headers: self.headers(),
        }
    }
}
