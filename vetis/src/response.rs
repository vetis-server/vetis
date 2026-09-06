use http::{HeaderMap, StatusCode};
use hyper_body_utils::HttpBody;

/// Builder for creating HTTP responses.
///
/// `ResponseBuilder` provides a fluent interface for constructing HTTP responses
/// with custom status codes, headers, and body content.
///
/// # Examples
///
/// ```rust,no_run
/// use bytes::Bytes;
/// use http::StatusCode;
/// use vetis::Response;
///
/// // Simple response
/// let response = Response::builder()
///     .status(StatusCode::OK)
///     .text("Hello, World!");
///
/// // Response with custom headers
/// let mut headers = http::HeaderMap::new();
/// headers.insert("content-type", "application/json".parse().unwrap());
/// let response = Response::builder()
///     .status(StatusCode::CREATED)
///     .headers(headers)
///     .text(r#"{"status": "success"}"#);
/// ```
pub struct ResponseBuilder {
    inner: http::Response<HttpBody>,
}

impl ResponseBuilder {
    /// Sets the HTTP status code for the response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    /// use http::StatusCode;
    ///
    /// let response = Response::builder()
    ///     .status(StatusCode::NOT_FOUND)
    ///     .text("Not found");
    /// ```
    pub fn status(mut self, status: http::StatusCode) -> Self {
        *self
            .inner
            .status_mut() = status;
        self
    }

    /// Sets the HTTP version for the response.
    ///
    /// By default, responses use HTTP/1.1.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    /// use http::Version;
    ///
    /// let response = Response::builder()
    ///     .version(http::Version::HTTP_2)
    ///     .text("Response");
    /// ```
    pub fn version(mut self, version: http::Version) -> Self {
        *self
            .inner
            .version_mut() = version;
        self
    }

    /// Adds a header to the response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let response = Response::builder()
    ///     .header("content-type", "text/plain".parse().unwrap())
    ///     .text("Plain text");
    /// ```
    pub fn header<K>(mut self, key: K, value: http::header::HeaderValue) -> Self
    where
        K: http::header::IntoHeaderName,
    {
        self.inner
            .headers_mut()
            .append(key, value);
        self
    }

    /// Sets the headers for the response.
    ///
    /// This replaces all existing headers.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.insert("content-type", "text/plain".parse().unwrap());
    ///
    /// let response = Response::builder()
    ///     .headers(headers)
    ///     .text("Plain text");
    /// ```
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        *self
            .inner
            .headers_mut() = headers;
        self
    }

    /// Sets an empty body and creates the final `Response`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let response = Response::builder()
    ///     .empty();
    /// ```
    pub fn empty(mut self) -> Response {
        *self
            .inner
            .body_mut() = HttpBody::empty();
        self.build()
    }

    /// Sets the body from a text string and creates the final `Response`.
    ///
    /// # Arguments
    ///
    /// * `text` - The response body as a text slice
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let response = Response::builder()
    ///     .text("Hello, World!");
    /// ```
    pub fn text(mut self, text: &str) -> Response {
        *self
            .inner
            .body_mut() = HttpBody::from_text(text);
        self.build()
    }

    /// Sets the body with bytes and creates the final `Response`.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The response body as a `Bytes`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let response = Response::builder()
    ///     .bytes(b"Hello, World!");
    /// ```
    pub fn bytes(mut self, bytes: &[u8]) -> Response {
        *self
            .inner
            .body_mut() = HttpBody::from_bytes(bytes);
        self.build()
    }

    /// Sets the body and creates the final `Response`.
    ///
    /// # Arguments
    ///
    /// * `body` - The response body as a `VetisBody`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let response = Response::builder()
    ///     .bytes(b"Hello, World!");
    /// ```
    pub fn body(mut self, body: HttpBody) -> Response {
        *self
            .inner
            .body_mut() = body;
        self.build()
    }

    #[inline]
    /// Build a new `Response`
    fn build(self) -> Response {
        Response { inner: self.inner }
    }
}

/// HTTP response containing status, headers, and body.
///
/// The `Response` struct represents an HTTP response that can be sent back to clients.
/// It's created using the `Response::builder()` method and contains the response body.
///
/// # Examples
///
/// ```rust,no_run
/// use bytes::Bytes;
/// use http_body_util::Full;
/// use http::StatusCode;
/// use vetis::Response;
///
/// // Create a simple response
/// let response = Response::builder()
///     .status(StatusCode::OK)
///     .text("Hello, World!");
///
/// // Convert to inner http::Response if needed
/// let inner_response = response.into_inner();
/// ```
pub struct Response {
    pub(crate) inner: http::Response<HttpBody>,
}

impl Response {
    /// Creates a new `ResponseBuilder` with default settings.
    ///
    /// The builder starts with:
    /// - Status: 200 OK
    /// - Version: HTTP/1.1
    /// - No headers
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let builder = Response::builder();
    /// let response = builder.text("Hello");
    /// ```
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder { inner: http::Response::new(HttpBody::empty()) }
    }

    /// Converts the response into the underlying `http::Response`.
    ///
    /// This is useful when you need to work with the standard library HTTP types
    /// or pass the response to other libraries that expect `http::Response`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use vetis::Response;
    ///
    /// let response = Response::builder()
    ///     .text("Hello");
    /// let inner = response.into_inner();
    /// ```
    pub fn into_inner(self) -> http::Response<HttpBody> {
        self.inner
    }
}

impl From<StatusCode> for Response {
    fn from(value: StatusCode) -> Self {
        Response::builder()
            .status(value)
            .empty()
    }
}
