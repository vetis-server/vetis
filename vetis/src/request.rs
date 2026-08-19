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
    /// Creates a `Request` from an HTTP/1 or HTTP/2 request.
    ///
    /// This is used internally by the server to wrap incoming HTTP requests.
    pub fn from_parts(parts: http::request::Parts, body: HttpBody) -> Self {
        Self { inner: http::Request::from_parts(parts, body) }
    }

    /// Returns the request URI.
    pub fn uri(&self) -> &http::Uri {
        self.inner.uri()
    }

    /// Returns the request headers.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{Request, Response, VetisResult};
    /// use http::{Method, StatusCode};
    ///
    /// async fn handler(request: Request) -> VetisResult<Response> {
    ///     let content_type = request.headers().get("content-type");
    ///     let user_agent = request.headers().get("user-agent");
    ///     Ok(Response::builder().status(StatusCode::OK).text("Hello"))
    /// }
    /// ```
    pub fn headers(&self) -> &http::HeaderMap {
        self.inner.headers()
    }

    /// Returns the request headers (mutable).
    ///
    /// # Examples
    ///
    /// ```
    /// use vetis::{Request, Response, VetisResult};
    /// use http::{Method, StatusCode};
    ///
    /// async fn handler(mut request: Request) -> VetisResult<Response> {
    ///     request.headers_mut().insert("x-custom-header", "value".parse().unwrap());
    ///     Ok(Response::builder().status(StatusCode::OK).text("Hello"))
    /// }
    /// ```
    pub fn headers_mut(&mut self) -> &mut http::HeaderMap {
        self.inner
            .headers_mut()
    }

    /// Returns the HTTP method.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{Request, Response, VetisResult};
    /// use http::{Method, StatusCode};
    ///
    /// async fn handler(request: Request) -> VetisResult<Response> {
    ///     match request.method() {
    ///         &Method::GET => { /* handle GET */ }
    ///         &Method::POST => { /* handle POST */ }
    ///         _ => { /* handle other methods */ }
    ///     }
    ///     Ok(Response::builder().status(StatusCode::OK).text("Hello"))
    /// }
    /// ```
    pub fn method(&self) -> &http::Method {
        self.inner.method()
    }

    /// Returns the HTTP version.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::{Request, Response, VetisResult};
    /// use http::{Method, StatusCode};
    ///
    /// async fn handler(request: Request) -> VetisResult<Response> {
    ///     match request.version() {
    ///         http::Version::HTTP_11 => { /* handle HTTP/1.1 */ }
    ///         http::Version::HTTP_2 => { /* handle HTTP/2 */ }
    ///         _ => { /* handle other versions */ }
    ///     }
    ///     Ok(Response::builder().status(StatusCode::OK).text("Hello"))
    /// }
    /// ```
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
