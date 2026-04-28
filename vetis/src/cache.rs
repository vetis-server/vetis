use std::time::Duration;

use log::error;
use moka::future::Cache;
use serde::Deserialize;

use crate::{errors::{FileError, VetisError, VirtualHostError}, fs::FileSource};

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const DEFAULT_TTL: Duration = Duration::from_secs(60);
const DEFAULT_TTI: Duration = Duration::from_secs(10);
const DEFAULT_CAPACITY: u64 = 1000;

/// Builder for creating `ResourceCache` instances.
#[derive(Debug, Clone)]
pub struct CacheConfigBuilder {
    max_file_size: usize,
    ttl: Duration,
    tti: Duration,
    capacity: u64,
}

impl CacheConfigBuilder {
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

    /// Build the `Cache`
    pub fn build(self) -> CacheConfig {
        CacheConfig {
            max_file_size: self.max_file_size,
            ttl: self.ttl,
            tti: self.tti,
            capacity: self.capacity,
        }
    }
}

/// Configuration for resource caching.
#[derive(Debug, Clone, Deserialize)]
pub struct CacheConfig {
    max_file_size: usize,
    #[serde(deserialize_with = "crate::utils::serde::deserialize_duration")]
    ttl: Duration,
    #[serde(deserialize_with = "crate::utils::serde::deserialize_duration")]
    tti: Duration,
    capacity: u64,
}

impl Default for CacheConfig {
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
impl CacheConfig {
    /// Create a new builder for `StaticPathCache`.
    pub fn builder() -> CacheConfigBuilder {
        CacheConfigBuilder {
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

/// Cache for files
#[derive(Clone)]
pub struct FileCache {
    cache: Cache<String, FileSource>,
}

impl FileCache {
    /// Create a new FileCache
    pub fn new(cache: Cache<String, FileSource>) -> Self {
        Self { cache }
    }

    /// Cache a file
    pub async fn cache_file(&self, file_path: &std::path::Path) -> Result<FileSource, VetisError> {
        let path = file_path
            .display()
            .to_string();

        let file = if let Some(file) = self
            .cache
            .get(&path)
            .await
        {
            Ok(file)
        } else {
            let file = self
                .resolver
                .load_file(file_path)
                .await;
            match file {
                Ok(file) => {
                    self.cache
                        .insert(path.clone(), file)
                        .await;
                    let file = self
                        .cache
                        .get(&path)
                        .await
                        .unwrap();
                    Ok(file)
                }
                Err(e) => {
                    error!("Error resolving file {}: {}", path, e);
                    Err(VetisError::VirtualHost(VirtualHostError::File(FileError::NotFound)))
                }
            }
        };

        file
    }
}
