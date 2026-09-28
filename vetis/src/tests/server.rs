use crate::{host::HostConfig, server::ServerConfig};
use caramelo::{expect, matchers::eq};

#[test]
fn test_add_host_to_server_config() {
    let server_config = ServerConfig::builder()
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
fn test_has_hosts_configured() {
    let server_config = ServerConfig::builder()
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
