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
/// use vetis::{Request, Response, errors::VetisError};
///
/// // In a request handler:
/// async fn handler(request: Request) -> Result<Response, VetisError> {
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
    pub(crate) inner: Option<http::Request<HttpBody>>,
}

impl Request {
    /// Creates a `Request` from an HTTP/1 or HTTP/2 request.
    ///
    /// This is used internally by the server to wrap incoming HTTP requests.
    pub fn from_parts(parts: http::request::Parts, body: HttpBody) -> Self {
        Self { inner: Some(http::Request::from_parts(parts, body)) }
    }

    /// Returns the request URI.
    pub fn uri(&self) -> &http::Uri {
        match &self.inner {
            Some(req) => req.uri(),
            None => panic!("No request"),
        }
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
        match &self.inner {
            Some(req) => req.headers(),
            None => panic!("No request"),
        }
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
        match &mut self.inner {
            Some(req) => req.headers_mut(),
            None => panic!("No request"),
        }
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
        match &self.inner {
            Some(req) => req.method(),
            None => panic!("No request"),
        }
    }

    /// Convert the request into parts.
    pub fn into_parts(self) -> (http::request::Parts, HttpBody) {
        match self.inner {
            Some(req) => {
                let (parts, body) = req.into_parts();
                (parts, body)
            }
            None => {
                panic!("No request");
            }
        }
    }
}
