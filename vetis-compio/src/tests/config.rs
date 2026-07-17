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
