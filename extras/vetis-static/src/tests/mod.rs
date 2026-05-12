use std::error::Error;

use vetis::errors::{ConfigError, VetisError};

use vetis::{
    listener::ListenerConfig, security::SecurityConfig, server::ServerConfig,
    virtual_host::VirtualHostConfig,
};

use crate::tests::vetis_default_protocol;

#[cfg(feature = "static-files")]
mod static_files_tests {
    use vetis::virtual_host::path::static_files::StaticPathConfig;

    #[test]
    fn test_static_files_config() -> Result<(), Box<dyn std::error::Error>> {
        let static_files_config = StaticPathConfig::builder()
            .uri("/static")
            .extensions("html,css,js")
            .directory("/var/vetis/www")
            .index_files(vec!["index.html".to_string(), "index.htm".to_string()])
            .build()?;
        assert_eq!(static_files_config.uri(), "/static");
        assert_eq!(static_files_config.extensions(), "html,css,js");
        assert_eq!(static_files_config.directory(), "/var/vetis/www");
        assert_eq!(
            static_files_config.index_files(),
            &Some(vec!["index.html".to_string(), "index.htm".to_string()])
        );
        Ok(())
    }
}
