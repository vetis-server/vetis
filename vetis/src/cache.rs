use crate::{Str, VetisResult, matcher::MatcherConfig};
use compact_str::ToCompactString;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

/// Builder for creating `CacheHookConfig` instances.
pub struct CacheConfigBuilder {
    matcher: Option<Box<dyn MatcherConfig>>,
    ttl: Duration,
    max_body_size: usize,
    status_ttl: HashMap<Str, Duration>,
    l1_backend: Option<Box<dyn CacheBackendConfig>>,
    l2_backend: Option<Box<dyn CacheBackendConfig>>,
}

impl CacheConfigBuilder {
    /// Allow set a matcher
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn matcher<M>(mut self, matcher: M) -> Self
    where
        M: MatcherConfig + 'static,
    {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Allow set a global ttl
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    /// Allow set a max max body size
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn max_body_size(mut self, max_size: usize) -> Self {
        self.max_body_size = max_size;
        self
    }

    /// Allow add per status ttl
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_status_tll(mut self, code: &str, duration: Duration) -> Self {
        self.status_ttl
            .insert(code.to_compact_string(), duration);
        self
    }

    /// Allow set a l1 cache backend
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_l1_backend<B>(mut self, config: B) -> Self
    where
        B: CacheBackendConfig + 'static,
    {
        self.l1_backend = Some(Box::new(config));
        self
    }

    /// Allow set a l2 cache backend
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn add_l2_backend<B>(mut self, config: B) -> Self
    where
        B: CacheBackendConfig + 'static,
    {
        self.l2_backend = Some(Box::new(config));
        self
    }

    /// Build the `CacheHookConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<CacheHookConfig>` - The `CacheHookConfig` with the configured settings.
    pub fn build(self) -> VetisResult<CacheConfig> {
        Ok(CacheConfig {
            matcher: self
                .matcher
                .map_or(None, |val| Some(val)),
            ttl: self.ttl,
            max_body_size: self.max_body_size,
            status_ttl: self.status_ttl,
            l1_backend: self
                .l1_backend
                .map_or(None, |val| Some(val)),
            l2_backend: self
                .l2_backend
                .map_or(None, |val| Some(val)),
        })
    }
}

/// Configuration for webserver cache.
#[derive(Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    matcher: Option<Box<dyn MatcherConfig>>,
    ttl: Duration,
    max_body_size: usize,
    status_ttl: HashMap<Str, Duration>,
    l1_backend: Option<Box<dyn CacheBackendConfig>>,
    l2_backend: Option<Box<dyn CacheBackendConfig>>,
}

impl CacheConfig {
    /// Return cache matcher
    pub fn matcher(&self) -> Option<&Box<dyn MatcherConfig>> {
        self.matcher
            .as_ref()
    }

    /// Return cache defaul ttl
    pub fn ttl(&self) -> Duration {
        self.ttl
    }

    /// Return max body size
    pub fn max_body_size(&self) -> usize {
        self.max_body_size
    }

    /// Return ttl overide by status
    pub fn status_ttl(&self) -> &HashMap<Str, Duration> {
        &self.status_ttl
    }

    /// Returns l1 cache backend configuration
    pub fn l1_backend(&self) -> Option<&Box<dyn CacheBackendConfig>> {
        self.l1_backend
            .as_ref()
    }

    /// Returns l2 cache backend configuration
    pub fn l2_backend(&self) -> Option<&Box<dyn CacheBackendConfig>> {
        self.l2_backend
            .as_ref()
    }

    /// Allow create a new `CacheConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `CacheConfigBuilder` - The builder.
    pub fn builder() -> CacheConfigBuilder {
        CacheConfigBuilder {
            matcher: None,
            ttl: Duration::from_mins(5),
            max_body_size: 50 * (1024 ^ 2),
            status_ttl: HashMap::new(),
            l1_backend: None,
            l2_backend: None,
        }
    }
}

#[typetag::serde(tag = "engine")]
/// A trait which describe Hook configuration
pub trait CacheBackendConfig {
    /// Allow clone any type which implements CacheBackendConfig
    ///
    /// # Returns
    ///
    /// * `Box<dyn CacheBackendConfig>` - Clone of boxed cache backend config
    fn boxed_clone(&self) -> Box<dyn CacheBackendConfig>;
}

impl Clone for Box<dyn CacheBackendConfig> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}
