use crate::{
    rt::Vetis,
    tests::vetis_default_protocol,
    virtual_host::{path::HandlerPath, VirtualHostImpl},
};
use http::StatusCode;
use std::error::Error;
use vetis::{
    listener::ListenerConfig,
    server::ServerConfig,
    virtual_host::{handler_fn, VirtualHostConfig},
    Response, Vetis as _,
};

fn create_listener() -> ListenerConfig {
    ListenerConfig::builder()
        .port(8080)
        .protocol(vetis_default_protocol())
        .interface("0.0.0.0")
        .build()
        .unwrap()
}

#[test]
fn test_vetis_new() {
    let config = ServerConfig::builder()
        .add_listener(create_listener())
        .build()
        .unwrap();
    let server = Vetis::new(config);

    assert_eq!(
        server
            .config()
            .listeners()
            .len(),
        1
    );
}

#[test]
fn test_vetis_config() {
    let config = ServerConfig::builder()
        .add_listener(create_listener())
        .build()
        .unwrap();

    let server = Vetis::new(config);

    assert_eq!(
        server
            .config()
            .listeners()
            .len(),
        1
    );
    assert_eq!(
        server
            .config()
            .listeners()[0]
            .port(),
        8080
    );
}

#[tokio::test]
async fn test_vetis_add_virtual_host() -> Result<(), Box<dyn Error>> {
    let config = ServerConfig::builder()
        .add_listener(create_listener())
        .build()
        .unwrap();
    let mut server = Vetis::new(config);

    let vhost_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(8080)
        .root_directory("src/tests")
        .build()?;

    let mut vhost = VirtualHostImpl::new(vhost_config);

    let handler_path = HandlerPath::builder()
        .uri("/")
        .handler(handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello, World!"))
        }))
        .build()?;

    vhost.add_path(handler_path);

    server
        .add_virtual_host(vhost)
        .await;

    assert_eq!(
        server
            .virtual_hosts()
            .read()
            .await
            .len(),
        1
    );

    Ok(())
}

#[tokio::test]
async fn test_vetis_start_no_virtual_hosts() -> Result<(), Box<dyn Error>> {
    let config = ServerConfig::builder()
        .add_listener(create_listener())
        .build()
        .unwrap();
    let mut server = Vetis::new(config);

    let result = server.start().await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_vetis_stop_no_instance() -> Result<(), Box<dyn Error>> {
    let config = ServerConfig::builder()
        .add_listener(create_listener())
        .build()
        .unwrap();
    let mut server = Vetis::new(config);

    let result = server.stop().await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_vetis_virtual_hosts() -> Result<(), Box<dyn Error>> {
    let config = ServerConfig::builder()
        .add_listener(create_listener())
        .build()
        .unwrap();
    let mut server = Vetis::new(config);

    let vhost_config = VirtualHostConfig::builder()
        .hostname("localhost")
        .port(8080)
        .root_directory("src/tests")
        .build()?;

    let mut vhost = VirtualHostImpl::new(vhost_config);

    let handler_path = HandlerPath::builder()
        .uri("/")
        .handler(handler_fn(|_request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .text("Hello, World!"))
        }))
        .build()?;

    vhost.add_path(handler_path);

    server
        .add_virtual_host(vhost)
        .await;

    let virtual_hosts = server
        .virtual_hosts()
        .read()
        .await;
    assert_eq!(virtual_hosts.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_vetis_add_multiple_virtual_hosts() -> Result<(), Box<dyn Error>> {
    let config = ServerConfig::builder()
        .add_listener(create_listener())
        .build()
        .unwrap();
    let mut server = Vetis::new(config);

    for i in 0..3 {
        let vhost_config = VirtualHostConfig::builder()
            .hostname(&format!("host{}", i))
            .port(8080 + i)
            .root_directory("src/tests")
            .build()?;

        let mut vhost = VirtualHostImpl::new(vhost_config);

        let handler_path = HandlerPath::builder()
            .uri("/")
            .handler(handler_fn(|_request| async move {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .text("Hello, World!"))
            }))
            .build()?;

        vhost.add_path(handler_path);

        server
            .add_virtual_host(vhost)
            .await;
    }

    assert_eq!(
        server
            .virtual_hosts()
            .read()
            .await
            .len(),
        3
    );

    Ok(())
}
