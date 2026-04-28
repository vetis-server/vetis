use std::time::Duration;

use serde::{Deserialize, Deserializer};

#[cfg(feature = "auth")]
use crate::auth::BasicAuthConfig;
use crate::errors::{ConfigError, VetisError};

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const DEFAULT_TTL: Duration = Duration::from_secs(60);
const DEFAULT_TTI: Duration = Duration::from_secs(10);
const DEFAULT_CAPACITY: u64 = 1000;

/// Builder for creating `StaticPathCache` instances.
#[derive(Debug, Clone)]
pub struct StaticPathCacheBuilder {
    max_file_size: usize,
    ttl: Duration,
    tti: Duration,
    capacity: u64,
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
