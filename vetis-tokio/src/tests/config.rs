use crate::tests::default_protocol_version;
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use vetis::errors::{ConfigError, VetisError};
use vetis::{
    host::HostConfig, listener::ListenerConfig, security::SecurityConfig, server::ServerConfig,
};

#[test]
fn test_listener_config() -> Result<(), Box<dyn Error>> {
    let protos = vec![default_protocol_version()];

    let listener_config = ListenerConfig::builder()
        .port(8080)
        .protos(protos.clone())
        .interface(
            "127.0.0.1"
                .parse()
                .unwrap(),
        )
        .build()?;
    assert_eq!(listener_config.port(), 8080);
    assert_eq!(listener_config.protos(), &protos);
    assert_eq!(listener_config.interface(), &IpAddr::V4(Ipv4Addr::LOCALHOST));

    Ok(())
}

#[test]
fn test_server_config() -> Result<(), Box<dyn Error>> {
    let server_config = ServerConfig::builder()
        .add_host(
            HostConfig::builder()
                .hostname("localhost")
                .root_directory("src/tests".into())
                .build()?,
        )
        .build()?;
    assert_eq!(
        server_config
            .hosts()
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
fn test_host_config() -> Result<(), Box<dyn std::error::Error>> {
    let host_config = HostConfig::builder()
        .hostname("localhost")
        .root_directory("src/tests".into())
        .build()?;
    assert_eq!(host_config.hostname(), "localhost");

    Ok(())
}

#[test]
fn test_default_host_config() -> Result<(), Box<dyn std::error::Error>> {
    let host_config = HostConfig::builder().build();
    assert_eq!(
        host_config.err(),
        Some(VetisError::Config(ConfigError::Host(
            "root_directory does not exist: /var/vetis/www".to_string()
        )))
    );
    Ok(())
}

#[test]
fn test_invalid_host_config() -> Result<(), Box<dyn std::error::Error>> {
    let host_config = HostConfig::builder()
        .hostname("")
        .root_directory("src/tests".into())
        .build();

    assert_eq!(
        host_config.err(),
        Some(VetisError::Config(ConfigError::Host("Missing hostname".to_string())))
    );
    Ok(())
}
