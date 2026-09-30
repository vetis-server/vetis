use hyper_body_utils::HttpBody;

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
}

impl Request {
    /// Create a `Request` out of http::Request
    pub fn new(inner: http::Request<HttpBody>) -> Self {
        Self { inner }
    }

    /// Creates a `Request` from an HTTP/1 or HTTP/2 request.
    ///
    /// This is used internally by the server to wrap incoming HTTP requests.
    pub fn from_parts(parts: http::request::Parts, body: HttpBody) -> Self {
        Self { inner: http::Request::from_parts(parts, body) }
    }

    /// Returns inner request
    pub fn inner(&self) -> &http::Request<HttpBody> {
        &self.inner
    }

    /// Returns mutable version of inner request
    pub fn inner_mut(&mut self) -> &mut http::Request<HttpBody> {
        &mut self.inner
    }

    /// Returns the request URI.
    pub fn uri(&self) -> &http::Uri {
        self.inner.uri()
    }

    /// Returns the request headers.
    pub fn headers(&self) -> &http::HeaderMap {
        self.inner.headers()
    }

    /// Returns the request headers (mutable).
    pub fn headers_mut(&mut self) -> &mut http::HeaderMap {
        self.inner
            .headers_mut()
    }

    /// Returns the HTTP method.
    pub fn method(&self) -> &http::Method {
        self.inner.method()
    }

    /// Returns the HTTP version.
    pub fn version(&self) -> http::Version {
        self.inner.version()
    }

    /// Convert the request into parts.
    pub fn into_parts(self) -> (http::request::Parts, HttpBody) {
        let (parts, body) = self
            .inner
            .into_parts();
        (parts, body)
    }
}
