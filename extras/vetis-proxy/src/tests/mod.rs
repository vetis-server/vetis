use std::error::Error;

use vetis::errors::{ConfigError, VetisError};

use vetis::{
    listener::ListenerConfig, security::SecurityConfig, server::ServerConfig,
    virtual_host::VirtualHostConfig,
};

use crate::tests::vetis_default_protocol;

#[cfg(feature = "reverse-proxy")]
mod reverse_proxy_tests {
    use vetis::virtual_host::path::proxy::ProxyPathConfig;

    #[test]
    fn test_reverse_proxy_config() -> Result<(), Box<dyn std::error::Error>> {
        let reverse_proxy_config = ProxyPathConfig::builder()
            .uri("/")
            .target("http://localhost:8081")
            .build()?;
        assert_eq!(reverse_proxy_config.uri(), "/");
        assert_eq!(reverse_proxy_config.target(), "http://localhost:8081");
        Ok(())
    }
}
