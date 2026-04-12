#[cfg(feature = "static-files")]
mod static_files_tests {

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

#[cfg(feature = "reverse-proxy")]
mod reverse_proxy_tests {
    use crate::config::server::virtual_host::path::proxy::ProxyPathConfig;

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

#[cfg(feature = "auth")]
mod auth_tests {
    use crate::config::server::virtual_host::path::auth::{Algorithm, BasicAuthConfig};

    #[test]
    fn test_auth_config() -> Result<(), Box<dyn std::error::Error>> {
        let auth_config = BasicAuthConfig::builder()
            .algorithm(Algorithm::BCrypt)
            .htpasswd(Some("src/tests/files/.htpasswd".to_string()))
            .build()?;
        assert_eq!(auth_config.algorithm(), &Algorithm::BCrypt);
        assert_eq!(auth_config.htpasswd(), &Some("src/tests/files/.htpasswd".to_string()));
        Ok(())
    }
}
