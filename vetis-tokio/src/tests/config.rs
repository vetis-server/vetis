use crate::tests::default_protocol_version;
use std::error::Error;
use vetis::errors::{ConfigError, VetisError};
use vetis::{
    listener::ListenerConfig, security::SecurityConfig, server::ServerConfig,
    virtual_host::VirtualHostConfig,
};

#[test]
fn test_listener_config() -> Result<(), Box<dyn Error>> {
    let protocol = default_protocol_version();

    let listener_config = ListenerConfig::builder()
        .port(8080)
        .protocol_version(protocol.clone())
        .interface("127.0.0.1")
        .build()?;
    assert_eq!(listener_config.port(), 8080);
    assert_eq!(listener_config.protocol_version(), &protocol);
    assert_eq!(listener_config.interface(), "127.0.0.1");

    Ok(())
}

#[test]
fn test_server_config() -> Result<(), Box<dyn Error>> {
    let server_config = ServerConfig::builder()
        .add_listener(
            ListenerConfig::builder()
                .port(8080)
                .build()?,
        )
        .build()?;
    assert_eq!(
        server_config
            .listeners()
            .len(),
        1
    );

    Ok(())
}

#[test]
fn test_security_config() -> Result<(), Box<dyn Error>> {
    let security_config = SecurityConfig::builder()
        .ca_cert_from_bytes(vec![])
        .cert_from_bytes(vec![])
        .key_from_bytes(vec![])
        .build();

    assert_eq!(
        security_config.err(),
        Some(VetisError::Config(ConfigError::Security("Missing certificate".to_string())))
    );

    Ok(())
}

#[test]
fn test_virtual_host_config() -> Result<(), Box<dyn std::error::Error>> {
    let virtual_host_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(8080)
        .root_directory("src/tests")
        .build()?;
    assert_eq!(virtual_host_config.hostname(), "localhost");
    assert_eq!(virtual_host_config.port(), 8080);

    Ok(())
}

#[test]
fn test_default_virtual_host_config() -> Result<(), Box<dyn std::error::Error>> {
    let virtual_host_config = VirtualHostConfig::builder().build();
    assert_eq!(
        virtual_host_config.err(),
        Some(VetisError::Config(ConfigError::VirtualHost(
            "root_directory does not exist: /var/vetis/www".to_string()
        )))
    );
    Ok(())
}

#[test]
fn test_invalid_virtual_host_config() -> Result<(), Box<dyn std::error::Error>> {
    let virtual_host_config = VirtualHostConfig::builder()
        .hostname("")
        .root_directory("src/tests")
        .build();

    assert_eq!(
        virtual_host_config.err(),
        Some(VetisError::Config(ConfigError::VirtualHost("Missing hostname".to_string())))
    );
    Ok(())
}
