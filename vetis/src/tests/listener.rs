use crate::{listener::ListenerConfig, security::Alpn};
use caramelo::{expect, matchers::eq};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[test]
fn test_listener_config_builder_default() {
    let builder = ListenerConfig::builder();
    let config = builder
        .build()
        .unwrap();

    assert_eq!(config.port(), 80);
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
        .interface(Ipv4Addr::LOCALHOST)
        .build()
        .unwrap();

    assert_eq!(config.interface(), &IpAddr::V4(Ipv4Addr::LOCALHOST));
}

#[test]
fn test_listener_config_builder_chain() {
    let config = ListenerConfig::builder()
        .port(8443)
        .interface(Ipv4Addr::LOCALHOST)
        .build()
        .unwrap();

    assert_eq!(config.port(), 8443);
    assert_eq!(config.interface(), &IpAddr::V4(Ipv4Addr::LOCALHOST));
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
fn test_listener_config_interface_getter() {
    let config = ListenerConfig::builder()
        .interface(Ipv6Addr::LOCALHOST)
        .build()
        .unwrap();

    assert_eq!(config.interface(), &IpAddr::V6(Ipv6Addr::LOCALHOST));
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
        .interface(Ipv4Addr::LOCALHOST)
        .build()
        .unwrap();

    assert_eq!(config.port(), 3000);
    assert_eq!(config.interface(), &IpAddr::V4(Ipv4Addr::LOCALHOST));
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
    expect(config.port()).to_be(eq(80));
}

#[test]
fn test_from_str_to_alpn() {
    expect("http/1.1".into()).to_be(eq(Alpn::Http11));
    expect("h2".into()).to_be(eq(Alpn::H2));
    expect("h2c".into()).to_be(eq(Alpn::H2c));
    expect("h3".into()).to_be(eq(Alpn::H3));
    expect("dot".into()).to_be(eq(Alpn::Dot));
    expect("doq".into()).to_be(eq(Alpn::Doq));
    expect("doh".into()).to_be(eq(Alpn::Doh));
    expect("acme-tls/1".into()).to_be(eq(Alpn::AcmeTls1));
}

#[test]
fn test_from_alpn_to_bytes() {
    expect(
        b"http/1.1"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::Http11));
    expect(
        b"h2"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::H2));
    expect(
        b"h2c"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::H2c));
    expect(
        b"h3"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::H3));
    expect(
        b"dot"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::Dot));
    expect(
        b"doq"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::Doq));
    expect(
        b"doh"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::Doh));
    expect(
        b"acme-tls/1"
            .to_vec()
            .into(),
    )
    .to_be(eq(Alpn::AcmeTls1));
}
