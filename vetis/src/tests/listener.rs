use crate::{listener::ListenerConfig, server::Protocol};

#[test]
fn test_listener_config_builder_default() {
    let builder = ListenerConfig::builder();
    let config = builder
        .build()
        .unwrap();

    assert_eq!(config.port(), 80);
    assert_eq!(config.protocol(), &Protocol::Http1);
    assert_eq!(config.interface(), "0.0.0.0");
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
        .interface("127.0.0.1")
        .build()
        .unwrap();

    assert_eq!(config.interface(), "127.0.0.1");
}

#[test]
fn test_listener_config_builder_with_protocol() {
    let config = ListenerConfig::builder()
        .protocol(Protocol::Http2)
        .build()
        .unwrap();

    assert_eq!(config.protocol(), &Protocol::Http2);
}

#[test]
fn test_listener_config_builder_chain() {
    let config = ListenerConfig::builder()
        .port(8443)
        .interface("127.0.0.1")
        .protocol(Protocol::Http2)
        .build()
        .unwrap();

    assert_eq!(config.port(), 8443);
    assert_eq!(config.interface(), "127.0.0.1");
    assert_eq!(config.protocol(), &Protocol::Http2);
}

#[test]
fn test_listener_config_builder_port_zero_error() {
    let result = ListenerConfig::builder()
        .port(0)
        .build();

    assert!(result.is_err());
}

#[test]
fn test_listener_config_builder_empty_interface_error() {
    let result = ListenerConfig::builder()
        .interface("")
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
        .protocol(Protocol::Http1)
        .build()
        .unwrap();

    assert_eq!(config.protocol(), &Protocol::Http1);

    let config = ListenerConfig::builder()
        .protocol(Protocol::Http2)
        .build()
        .unwrap();

    assert_eq!(config.protocol(), &Protocol::Http2);
}

#[test]
fn test_listener_config_interface_getter() {
    let config = ListenerConfig::builder()
        .interface("::1")
        .build()
        .unwrap();

    assert_eq!(config.interface(), "::1");
}

#[test]
fn test_listener_config_clone() {
    let config = ListenerConfig::builder()
        .port(8080)
        .interface("127.0.0.1")
        .protocol(Protocol::Http2)
        .build()
        .unwrap();

    let cloned_config = config.clone();

    assert_eq!(cloned_config.port(), config.port());
    assert_eq!(cloned_config.interface(), config.interface());
    assert_eq!(cloned_config.protocol(), config.protocol());
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
    let interfaces = ["0.0.0.0", "127.0.0.1", "::1", "192.168.1.1", "10.0.0.1"];

    for interface in interfaces {
        let config = ListenerConfig::builder()
            .interface(interface)
            .build()
            .unwrap();

        assert_eq!(config.interface(), interface);
    }
}

#[test]
fn test_listener_config_builder_preserves_settings() {
    let config = ListenerConfig::builder()
        .port(3000)
        .interface("localhost")
        .protocol(Protocol::Http1)
        .build()
        .unwrap();

    assert_eq!(config.port(), 3000);
    assert_eq!(config.interface(), "localhost");
    assert_eq!(config.protocol(), &Protocol::Http1);
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
