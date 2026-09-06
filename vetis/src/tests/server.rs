use crate::{host::HostConfig, listener::ListenerConfig, server::ServerConfig};
use caramelo::{expect, matchers::eq};

#[test]
fn test_add_host_to_server_config() {
    let server_config = ServerConfig::builder()
        .add_listener(ListenerConfig::default())
        .add_host(HostConfig::default())
        .build()
        .unwrap();
    expect(
        server_config
            .hosts()
            .len(),
    )
    .to_be(eq(1));
}

#[test]
#[should_panic = "Server(\"No listeners configured\")"]
fn test_no_listeners_configured() {
    ServerConfig::builder()
        .add_host(HostConfig::default())
        .build()
        .unwrap();
}

#[test]
#[should_panic = "Server(\"No hosts configured\")"]
fn test_no_hosts_configured() {
    ServerConfig::builder()
        .add_listener(ListenerConfig::default())
        .build()
        .unwrap();
}

#[test]
fn test_has_hosts_configured() {
    let server_config = ServerConfig::builder()
        .add_listener(ListenerConfig::default())
        .add_host(HostConfig::default())
        .build()
        .unwrap();
    expect(
        server_config
            .hosts()
            .len(),
    )
    .to_be(eq(1));
}

#[test]
fn test_has_listsners_configured() {
    let server_config = ServerConfig::builder()
        .add_listener(ListenerConfig::default())
        .add_host(HostConfig::default())
        .build()
        .unwrap();
    expect(
        server_config
            .listeners()
            .len(),
    )
    .to_be(eq(1));
}
