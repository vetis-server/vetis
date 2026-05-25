use std::{path::PathBuf, time::Duration};

use serde::{Deserialize, Deserializer, Serialize};

#[cfg(feature = "auth")]
use crate::auth::BasicAuthConfig;
use vetis::{
    Response, errors::{ConfigError, FileError, VetisError, VirtualHostError}, virtual_host::path::PathConfig
};

pub(crate) type VetisFileCache = Cache<String, StaticFile>;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const DEFAULT_TTL: Duration = Duration::from_secs(60);
const DEFAULT_TTI: Duration = Duration::from_secs(10);
const DEFAULT_CAPACITY: u64 = 1000;

/// Builder for creating `StaticPathCache` instances.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StaticPathCacheBuilder {
    uri: String,
    max_file_size: usize,
    ttl: Duration,
    tti: Duration,
    capacity: u64,
}

#[typetag::serde]
impl PathConfig for StaticPathCacheBuilder {
    fn uri(&mut self, value: &str) {
        self.uri = value.to_string();
    }
}

impl StaticPathCacheBuilder {
    /// Set max file size
    pub fn max_file_size(mut self, max_file_size: usize) -> Self {
        self.max_file_size = max_file_size;
        self
    }

    /// Set time to live
    pub fn ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    /// Set time to idle
    pub fn tti(mut self, tti: Duration) -> Self {
        self.tti = tti;
        self
    }

    /// Set capacity
    pub fn capacity(mut self, capacity: u64) -> Self {
        self.capacity = capacity;
        self
    }

    /// Build the `StaticPathCache`
    pub fn build(self) -> StaticPathCache {
        StaticPathCache {
            max_file_size: self.max_file_size,
            ttl: self.ttl,
            tti: self.tti,
            capacity: self.capacity,
        }
    }
}

/// Configuration for static file caching.
#[derive(Debug, Clone, Deserialize)]
pub struct StaticPathCache {
    max_file_size: usize,
    #[serde(deserialize_with = "deserialize_duration")]
    ttl: Duration,
    #[serde(deserialize_with = "deserialize_duration")]
    tti: Duration,
    capacity: u64,
}

impl Default for StaticPathCache {
    fn default() -> Self {
        Self {
            max_file_size: MAX_FILE_SIZE,
            ttl: DEFAULT_TTL,
            tti: DEFAULT_TTI,
            capacity: DEFAULT_CAPACITY,
        }
    }
}

/// Configuration for static file caching.
impl StaticPathCache {
    /// Create a new builder for `StaticPathCache`.
    pub fn builder() -> StaticPathCacheBuilder {
        StaticPathCacheBuilder {
            uri: String::new(),
            max_file_size: MAX_FILE_SIZE,
            ttl: DEFAULT_TTL,
            tti: DEFAULT_TTI,
            capacity: DEFAULT_CAPACITY,
        }
    }

    /// Return max file size
    pub fn max_file_size(&self) -> usize {
        self.max_file_size
    }

    /// Return time to live
    pub fn ttl(&self) -> Duration {
        self.ttl
    }

    /// Return time to idle
    pub fn tti(&self) -> Duration {
        self.tti
    }

    /// Return capacity
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}

/// Builder for creating `StaticPathConfig` instances.
pub struct StaticPathConfigBuilder {
    uri: String,
    extensions: String,
    directory: String,
    index_files: Option<Vec<String>>,
    #[cfg(feature = "auth")]
    basic_auth: Option<BasicAuthConfig>,
    cache: Option<StaticPathCache>,
}

impl StaticPathConfigBuilder {
    /// Allow set the URI of the static path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn uri(mut self, uri: &str) -> Self {
        self.uri = uri.to_string();
        self
    }

    /// Allow set the extensions of the static path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn extensions(mut self, extensions: &str) -> Self {
        self.extensions = extensions.to_string();
        self
    }

    /// Allow set the directory of the static path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn directory(mut self, directory: &str) -> Self {
        self.directory = directory.to_string();
        self
    }

    /// Allow set the index files of the static path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn index_files(mut self, index_files: Vec<String>) -> Self {
        self.index_files = Some(index_files);
        self
    }

    #[cfg(feature = "auth")]
    /// Allow set the authentication of the static path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn basic_auth(mut self, basic_auth: BasicAuthConfig) -> Self {
        self.basic_auth = Some(basic_auth);
        self
    }

    /// Allow set the cache of the static path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn cache(mut self, cache: StaticPathCache) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Build the `StaticPathConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `Result<StaticPathConfig, VetisError>` - The `StaticPathConfig` with the configured settings.
    pub fn build(self) -> Result<StaticPathConfig, VetisError> {
        if self.uri.is_empty() {
            return Err(VetisError::Config(ConfigError::Path("URI cannot be empty".to_string())));
        }
        if self
            .extensions
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Path(
                "Extensions cannot be empty".to_string(),
            )));
        }
        if self
            .directory
            .is_empty()
        {
            return Err(VetisError::Config(ConfigError::Path(
                "Directory cannot be empty".to_string(),
            )));
        }

        Ok(StaticPathConfig {
            uri: self.uri,
            extensions: self.extensions,
            directory: self.directory,
            index_files: self.index_files,
            #[cfg(feature = "auth")]
            basic_auth: self.basic_auth,
            cache: self.cache,
        })
    }
}

/// Configuration for static file serving.
#[derive(Clone, Deserialize)]
pub struct StaticPathConfig {
    uri: String,
    extensions: String,
    directory: String,
    index_files: Option<Vec<String>>,
    #[cfg(feature = "auth")]
    basic_auth: Option<BasicAuthConfig>,
    cache: Option<StaticPathCache>,
}

impl StaticPathConfig {
    /// Allow create a new `StaticPathConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `StaticPathConfigBuilder` - The builder.
    pub fn builder() -> StaticPathConfigBuilder {
        StaticPathConfigBuilder {
            uri: "/".to_string(),
            extensions: ".html".to_string(),
            directory: ".".to_string(),
            index_files: None,
            #[cfg(feature = "auth")]
            basic_auth: None,
            cache: Some(StaticPathCache::default()),
        }
    }

    /// Returns uri
    ///
    /// # Returns
    ///
    /// * `&str` - The uri.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns extensions
    ///
    /// # Returns
    ///
    /// * `&str` - The extensions.
    pub fn extensions(&self) -> &str {
        &self.extensions
    }

    /// Returns directory
    ///
    /// # Returns
    ///
    /// * `&str` - The directory.
    pub fn directory(&self) -> &str {
        &self.directory
    }

    /// Returns index_files
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<String>>` - The index_files.
    pub fn index_files(&self) -> &Option<Vec<String>> {
        &self.index_files
    }

    #[cfg(feature = "auth")]
    /// Returns basic_auth
    ///
    /// # Returns
    ///
    /// * `&Option<BasicAuthConfig>` - The basic_auth.
    pub fn basic_auth(&self) -> &Option<BasicAuthConfig> {
        &self.basic_auth
    }

    /// Returns cache
    ///
    /// # Returns
    ///
    /// * `&Option<StaticPathCache>` - The cache.
    pub fn cache(&self) -> &Option<StaticPathCache> {
        &self.cache
    }
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_duration::parse(&s).map_err(serde::de::Error::custom)
}

pub struct StaticFileMetadataBuilder {
    mime: Option<String>,
    size: u64,
    modified: std::time::SystemTime,
    etag: Option<String>,
}

impl StaticFileMetadataBuilder {
    pub fn mime(mut self, mime: String) -> Self {
        self.mime = Some(mime);
        self
    }

    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn modified(mut self, modified: std::time::SystemTime) -> Self {
        self.modified = modified;
        self
    }

    pub fn etag(mut self, etag: String) -> Self {
        self.etag = Some(etag);
        self
    }

    pub fn build(self) -> StaticFileMetadata {
        StaticFileMetadata {
            mime: self.mime,
            size: self.size,
            modified: self.modified,
            etag: self.etag,
        }
    }
}

#[derive(Clone)]
pub struct StaticFileMetadata {
    mime: Option<String>,
    size: u64,
    modified: std::time::SystemTime,
    etag: Option<String>,
}

impl StaticFileMetadata {
    pub fn builder() -> StaticFileMetadataBuilder {
        StaticFileMetadataBuilder {
            mime: None,
            size: 0,
            modified: std::time::SystemTime::now(),
            etag: None,
        }
    }

    pub fn mime(&self) -> Option<&String> {
        self.mime.as_ref()
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn modified(&self) -> std::time::SystemTime {
        self.modified
    }

    pub fn etag(&self) -> Option<&String> {
        self.etag.as_ref()
    }
}

#[derive(Clone)]
pub enum StaticFile {
    Data { data: Vec<u8>, metadata: StaticFileMetadata },
    File { path: PathBuf, metadata: StaticFileMetadata },
}

impl StaticFile {
    pub fn metadata(&self) -> &StaticFileMetadata {
        match self {
            StaticFile::Data { metadata, .. } => metadata,
            StaticFile::File { metadata, .. } => metadata,
        }
    }

    pub fn data(&self) -> Option<&Vec<u8>> {
        match self {
            StaticFile::Data { data, .. } => Some(data),
            StaticFile::File { .. } => None,
        }
    }

    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            StaticFile::Data { .. } => None,
            StaticFile::File { path, .. } => Some(path),
        }
    }
}

/// Static path
pub struct StaticPath {
    config: StaticPathConfig,
    index_file: Option<String>,
    file_cache: VetisFileCache,
}

impl StaticPath {
    /// Create a new static path with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the static path
    ///
    /// # Returns
    ///
    /// * `StaticPath` - The static path
    pub fn new(config: StaticPathConfig) -> StaticPath {
        let file_cache = if let Some(cache) = config.cache() {
            CacheBuilder::new(cache.capacity())
                .time_to_idle(cache.tti())
                .time_to_live(cache.ttl())
        } else {
            CacheBuilder::new(1000)
                .time_to_idle(Duration::from_secs(60))
                .time_to_live(Duration::from_secs(60))
        }
        .build();

        if let Some(index_files) = config.index_files() {
            let directory = PathBuf::from(config.directory());
            if let Some(index_file) = index_files
                .iter()
                .find(|index_file| {
                    directory
                        .join(index_file)
                        .exists()
                })
            {
                return StaticPath {
                    config: config.clone(),
                    index_file: Some(index_file.to_string()),
                    file_cache,
                };
            }
        }
        StaticPath { config, index_file: None, file_cache }
    }

    async fn cache_file(&self, file_path: &std::path::Path) -> Result<StaticFile, VetisError> {
        let path = file_path
            .display()
            .to_string();

        let file = if let Some(file) = self
            .file_cache
            .get(&path)
            .await
        {
            Ok(file)
        } else {
            let file = VetisFile::open(path.clone()).await;
            match file {
                Ok(file) => {
                    let metadata = match file
                        .metadata()
                        .await
                    {
                        Ok(metadata) => metadata,
                        Err(e) => {
                            error!("Error getting metadata for file {:?}: {}", file_path, e);
                            return Err(VetisError::VirtualHost(VirtualHostError::File(
                                FileError::NotFound,
                            )));
                        }
                    };

                    let modified = metadata
                        .modified()
                        .unwrap_or(std::time::SystemTime::now());

                    let file_name = file_path.file_name();

                    let mime_type = match file_name {
                        Some(file_name) => match file_name.to_str() {
                            Some(file_name) => match minimime::lookup_by_filename(file_name) {
                                Some(mime) => Some(mime.content_type),
                                None => None,
                            },
                            None => None,
                        },
                        None => None,
                    };

                    let metadata = StaticFileMetadata {
                        mime: mime_type,
                        #[cfg(unix)]
                        size: metadata.size(),
                        #[cfg(windows)]
                        size: metadata.file_size(),
                        modified,
                        etag: None,
                    };

                    let max_file_size = if let Some(cache) = self.config.cache() {
                        cache.max_file_size() as u64
                    } else {
                        1024 * 1024 * 10 // 10MB default
                    };

                    let static_file = if metadata.size() < max_file_size {
                        #[cfg(feature = "tokio-rt")]
                        let data = tokio::fs::read(file_path).await;
                        #[cfg(not(feature = "tokio-rt"))]
                        let data = smol::fs::read(file_path).await;
                        if let Ok(data) = data {
                            StaticFile::Data { data, metadata }
                        } else {
                            return Err(VetisError::VirtualHost(VirtualHostError::File(
                                FileError::NotFound,
                            )));
                        }
                    } else {
                        StaticFile::File { path: file_path.to_path_buf(), metadata }
                    };

                    self.file_cache
                        .insert(path.clone(), static_file)
                        .await;
                    let file = self
                        .file_cache
                        .get(&path)
                        .await
                        .unwrap();
                    Ok(file)
                }
                Err(e) => {
                    error!("Error opening file {}: {}", path, e);
                    Err(VetisError::VirtualHost(VirtualHostError::File(FileError::NotFound)))
                }
            }
        };

        file
    }

    async fn serve_file(
        &self,
        file_path: &std::path::Path,
        range: Option<&str>,
    ) -> Result<Response, VetisError> {
        let file = self
            .cache_file(file_path)
            .await?;

        let filesize = file
            .metadata()
            .size();

        if let Some(range) = range {
            let range_info = match range
                .split_once("=")
                .ok_or(VetisError::VirtualHost(VirtualHostError::File(FileError::InvalidRange)))
            {
                Ok(info) => info,
                Err(e) => return Err(e),
            };

            let (unit, range) = range_info;
            if unit != "bytes" {
                return Err(VetisError::VirtualHost(VirtualHostError::File(
                    FileError::InvalidRange,
                )));
            }

            let (start, end) = range
                .split_once("-")
                .ok_or(VetisError::VirtualHost(VirtualHostError::File(FileError::InvalidRange)))?;
            let start = start
                .parse::<u64>()
                .map_err(|_| {
                    VetisError::VirtualHost(VirtualHostError::File(FileError::InvalidRange))
                })?;
            let end = end
                .parse::<u64>()
                .map_err(|_| {
                    VetisError::VirtualHost(VirtualHostError::File(FileError::InvalidRange))
                })?;
            if start > end || start >= filesize {
                return Ok(Response::builder()
                    .status(http::StatusCode::RANGE_NOT_SATISFIABLE)
                    .body(HttpBody::from_text("")));
            } else if start < end && end < filesize {
                return Ok(Response::builder()
                    .status(http::StatusCode::PARTIAL_CONTENT)
                    .body(HttpBody::from_bytes(file.data().unwrap())));
            }
        }

        Ok(Response::builder()
            .status(http::StatusCode::OK)
            .header(
                http::header::ACCEPT_RANGES,
                "bytes"
                    .parse()
                    .unwrap(),
            )
            .header(http::header::CONTENT_LENGTH, HeaderValue::from(filesize))
            .body(HttpBody::from_bytes(file.data().unwrap())))
    }

    async fn serve_metadata(&self, file_path: PathBuf) -> Result<Response, VetisError> {
        let file = self
            .cache_file(&file_path)
            .await?;

        let len = file
            .metadata()
            .size();
        let mut headers = HeaderMap::new();
        match len
            .to_string()
            .parse()
        {
            Ok(len) => {
                headers.insert(http::header::CONTENT_LENGTH, len);
            }
            Err(_) => todo!(),
        }
        let last_modified = file
            .metadata()
            .modified();
        let date = crate::utils::date::format_date(last_modified);
        headers.insert(
            http::header::LAST_MODIFIED,
            date.parse()
                .map_err(|_| {
                    VetisError::VirtualHost(VirtualHostError::File(FileError::InvalidMetadata))
                })?,
        );

        let mime_type = file
            .metadata()
            .mime();
        if let Some(mime_type) = mime_type {
            headers.insert(
                http::header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type).map_err(|_| {
                    VetisError::VirtualHost(VirtualHostError::File(FileError::InvalidMetadata))
                })?,
            );
        }

        let response = Response::builder()
            .status(http::StatusCode::OK)
            .headers(headers)
            .text("");

        Ok(response)
    }

    async fn serve_index_file(&self, directory: &std::path::Path) -> Result<Response, VetisError> {
        match &self.index_file {
            Some(index_file) => {
                let full_path = directory.join(index_file);
                self.serve_file(&full_path, None)
                    .await
            }
            None => {
                println!("No index file configured");
                Err(VetisError::VirtualHost(VirtualHostError::File(FileError::NotFound)))
            }
        }
    }
}

impl From<StaticPath> for HostPath {
    /// Convert static path to host path
    ///
    /// # Arguments
    ///
    /// * `value` - The static path to convert
    ///
    /// # Returns
    ///
    /// * `HostPath` - The host path
    fn from(value: StaticPath) -> Self {
        HostPath::Static(value)
    }
}

impl Path for StaticPath {
    /// Returns the uri of the static path
    ///
    /// # Returns
    ///
    /// * `&str` - The uri of the static path
    fn uri(&self) -> &str {
        self.config.uri()
    }

    /// Handles the request for the static path
    ///
    /// # Returns
    ///
    /// * `Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>>` - The response to the request
    fn handle(
        &self,
        request: Request,
        uri: Arc<String>,
    ) -> Pin<Box<dyn Future<Output = Result<Response, VetisError>> + Send + '_>> {
        Box::pin(async move {
            let ext_regex = regex::Regex::new(
                self.config
                    .extensions(),
            );

            let directory = PathBuf::from(
                self.config
                    .directory(),
            );

            #[cfg(feature = "auth")]
            if let Some(auth) = self.config.auth() {
                if !auth
                    .authenticate(request.headers())
                    .await
                    .unwrap_or(false)
                {
                    return Err(VetisError::VirtualHost(VirtualHostError::Auth(
                        "Unauthorized".to_string(),
                    )));
                }
            }

            let uri = uri
                .strip_prefix("/")
                .unwrap_or(&uri);
            let file = directory.join(uri);

            if self
                .config
                .index_files()
                .is_some()
            {
                if !file.exists() {
                    if let Ok(ext_regex) = ext_regex {
                        if !ext_regex.is_match(uri.as_ref()) {
                            return Err(VetisError::VirtualHost(VirtualHostError::File(
                                FileError::NotFound,
                            )));
                        }
                    }
                } else if file.is_dir() {
                    return self
                        .serve_index_file(&file)
                        .await;
                }
            } else if !file.exists() {
                return Err(VetisError::VirtualHost(VirtualHostError::File(FileError::NotFound)));
            }

            if request.method() == http::Method::HEAD {
                return self
                    .serve_metadata(file)
                    .await;
            }

            let range = if request
                .headers()
                .contains_key(http::header::RANGE)
            {
                let value = request
                    .headers()
                    .get(http::header::RANGE);
                Some(
                    value
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
            } else {
                None
            };

            self.serve_file(&file, range)
                .await
        })
    }
}