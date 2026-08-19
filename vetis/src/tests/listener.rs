use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use http::Version;

use crate::listener::ListenerConfig;

#[test]
fn test_listener_config_builder_default() {
    let builder = ListenerConfig::builder();
    let config = builder
        .build()
        .unwrap();

    assert_eq!(config.port(), 80);
    assert_eq!(config.protos(), &vec![Version::HTTP_11]);
    assert_eq!(
        config.interface(),
        &"0.0.0.0"
            .parse::<Ipv4Addr>()
            .unwrap()
    );
}

#[test]
fn test_listener_config_builder_with_port() {
    let config = ListenerConfig::builder()
        .port(8080)
        .build()
        .unwrap();

    assert_eq!(config.port(), 8080);
}

#[test]
fn test_listener_config_builder_with_interface() {
    let config = ListenerConfig::builder()
        .interface(Ipv4Addr::LOCALHOST.into())
        .build()
        .unwrap();

    assert_eq!(config.interface(), &IpAddr::V4(Ipv4Addr::LOCALHOST));
}

#[test]
fn test_listener_config_builder_with_protocol_version() {
    let config = ListenerConfig::builder()
        .protos(vec![Version::HTTP_2])
        .build()
        .unwrap();

    assert_eq!(config.protos(), &vec![Version::HTTP_2]);
}

#[test]
fn test_listener_config_builder_chain() {
    let config = ListenerConfig::builder()
        .port(8443)
        .interface(Ipv4Addr::LOCALHOST.into())
        .protos(vec![Version::HTTP_2])
        .build()
        .unwrap();

    assert_eq!(config.port(), 8443);
    assert_eq!(config.interface(), &IpAddr::V4(Ipv4Addr::LOCALHOST));
    assert_eq!(config.protos(), &vec![Version::HTTP_2]);
}

#[test]
fn test_listener_config_builder_port_zero_error() {
    let result = ListenerConfig::builder()
        .port(0)
        .build();

    assert!(result.is_err());
}

#[test]
fn test_listener_config_port_getter() {
    let config = ListenerConfig::builder()
        .port(9090)
        .build()
        .unwrap();

    assert_eq!(config.port(), 9090);
}

#[test]
fn test_listener_config_protocol_getter() {
    let config = ListenerConfig::builder()
        .protos(vec![Version::HTTP_11])
        .build()
        .unwrap();

    assert_eq!(config.protos(), &vec![Version::HTTP_11]);

    let config = ListenerConfig::builder()
        .protos(vec![Version::HTTP_2])
        .build()
        .unwrap();

    assert_eq!(config.protos(), &vec![Version::HTTP_2]);
}

#[test]
fn test_listener_config_interface_getter() {
    let config = ListenerConfig::builder()
        .interface(Ipv6Addr::LOCALHOST.into())
        .build()
        .unwrap();

    assert_eq!(config.interface(), &IpAddr::V6(Ipv6Addr::LOCALHOST));
}

#[test]
fn test_listener_config_clone() {
    let config = ListenerConfig::builder()
        .port(8080)
        .interface(Ipv4Addr::LOCALHOST.into())
        .protos(vec![Version::HTTP_2])
        .build()
        .unwrap();

    let cloned_config = config.clone();

    assert_eq!(cloned_config.port(), config.port());
    assert_eq!(cloned_config.interface(), config.interface());
    assert_eq!(cloned_config.protos(), config.protos());
}

#[test]
fn test_listener_config_multiple_ports() {
    let ports = [80, 8080, 8443, 3000, 5000];

    for port in ports {
        let config = ListenerConfig::builder()
            .port(port)
            .build()
            .unwrap();

        assert_eq!(config.port(), port);
    }
}

#[test]
fn test_listener_config_various_interfaces() {
    let interfaces = [
        IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
    ];

    for interface in interfaces {
        let config = ListenerConfig::builder()
            .interface(interface)
            .build()
            .unwrap();

        assert_eq!(config.interface(), &interface);
    }
}

#[test]
fn test_listener_config_builder_preserves_settings() {
    let config = ListenerConfig::builder()
        .port(3000)
        .interface(Ipv4Addr::LOCALHOST.into())
        .protos(vec![Version::HTTP_11])
        .build()
        .unwrap();

    assert_eq!(config.port(), 3000);
    assert_eq!(config.interface(), &IpAddr::V4(Ipv4Addr::LOCALHOST));
    assert_eq!(config.protos(), &vec![Version::HTTP_11]);
}

#[test]
fn test_listener_config_max_port() {
    let config = ListenerConfig::builder()
        .port(65535)
        .build()
        .unwrap();

    assert_eq!(config.port(), 65535);
}

#[test]
fn test_listener_config_min_port() {
    let config = ListenerConfig::builder()
        .port(1)
        .build()
        .unwrap();

    assert_eq!(config.port(), 1);
}

#[test]
fn test_port_into_listener() {
    let config: ListenerConfig = 80.into();
    assert_eq!(config.port(), 80)
}

#[test]
fn test_version_into_listener() {
    let config: ListenerConfig = Version::HTTP_11.into();
    assert_eq!(config.protos()[0], Version::HTTP_11);
}

#[test]
fn test_port_version_into_listener() {
    let config: ListenerConfig = (80, Version::HTTP_11).into();
    assert_eq!(config.port(), 80);
    assert_eq!(config.protos()[0], Version::HTTP_11);
}
