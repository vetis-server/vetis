use crate::{
    VetisTestResult,
    errors::{ConfigError, VetisError},
    host::{AltService, HostConfig},
    listener::{self, ListenerConfig},
    security::TlsConfig,
};
use caramelo::{
    expect,
    matchers::{eq, truthy},
};
use http::{Version, uri::Authority};
use std::{collections::HashMap, fs, net::Ipv4Addr, time::Duration};

#[test]
fn test_host_config_build_success() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_build_success");
    fs::create_dir_all(&root_dir).unwrap();

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.root_directory(), &Some(root_dir.clone()));
    assert!(
        config
            .default_headers()
            .is_none()
    );
    assert!(
        config
            .status_pages()
            .is_none()
    );

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_host_config_build_with_port() {
    let listener = listener::ListenerConfig::builder()
        .port(8080)
        .build()
        .unwrap();

    let config = HostConfig::builder()
        .hostname("example.com")
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(listener.port(), 8080);
}

#[test]
fn test_host_config_build_with_header() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .header("X-Custom", "value")
        .build()
        .unwrap();

    assert!(
        config
            .default_headers()
            .is_some()
    );
    let headers = config
        .default_headers()
        .unwrap();
    assert_eq!(headers.len(), 1);
    assert_eq!(headers[0], ("X-Custom".into(), "value".into()));
}

#[test]
fn test_host_config_build_with_multiple_headers() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .header("X-Custom-1", "value1")
        .header("X-Custom-2", "value2")
        .build()
        .unwrap();

    assert!(
        config
            .default_headers()
            .is_some()
    );
    let headers = config
        .default_headers()
        .unwrap();
    assert_eq!(headers.len(), 2);
    assert_eq!(headers[0], ("X-Custom-1".into(), "value1".into()));
    assert_eq!(headers[1], ("X-Custom-2".into(), "value2".into()));
}

#[test]
fn test_host_config_build_with_security() -> VetisTestResult<()> {
    let security = TlsConfig::builder()
        .cert_file("../certs/server.der")
        .key_file("../certs/server.key.der")
        .build()
        .unwrap();

    let host = HostConfig::builder()
        .tls(security.clone())
        .build()
        .unwrap();

    let host_security = host
        .tls()
        .as_ref()
        .unwrap();

    assert_eq!(host_security.cert_file(), security.cert_file());
    assert_eq!(host_security.key_file(), security.key_file());

    Ok(())
}

#[test]
fn test_host_config_build_with_status_pages() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_status_pages");
    fs::create_dir_all(&root_dir).unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, "404.html".into());
    status_pages.insert(500, "500.html".into());

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .status_pages(status_pages)
        .build()
        .unwrap();

    assert!(
        config
            .status_pages()
            .is_some()
    );
    let pages = config
        .status_pages()
        .unwrap();
    assert_eq!(pages.len(), 2);
    assert_eq!(pages.get(&404), Some(&"404.html".into()));
    assert_eq!(pages.get(&500), Some(&"500.html".into()));

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_host_config_build_full() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_full");
    fs::create_dir_all(&root_dir).unwrap();

    let tls = TlsConfig::builder()
        .cert_file("../certs/server.der")
        .key_file("../certs/server.key.der")
        .build()
        .unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, "404.html".into());

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .header("X-Custom", "value")
        .tls(tls)
        .status_pages(status_pages)
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.root_directory(), &Some(root_dir.clone()));
    assert!(
        config
            .default_headers()
            .is_some()
    );
    assert!(
        config
            .status_pages()
            .is_some()
    );

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_host_config_build_missing_hostname() {
    let result = HostConfig::builder()
        .hostname("")
        .build();

    assert!(result.is_err());
    match result {
        Err(VetisError::Config(ConfigError::Host(msg))) => {
            assert_eq!(msg, "Missing hostname");
        }
        _ => panic!("Expected ConfigError::Host"),
    }
}

#[test]
fn test_host_config_build_missing_root_directory() {
    let result = HostConfig::builder()
        .hostname("example.com")
        .build();

    assert!(result.is_ok());
}

#[test]
fn test_host_config_build_nonexistent_root_directory() {
    let result = HostConfig::builder()
        .hostname("example.com")
        .root_directory("/nonexistent/path/to/root")
        .build();

    assert!(result.is_err());
    match result {
        Err(VetisError::Config(ConfigError::Host(msg))) => {
            assert!(msg.contains("root_directory does not exist"));
        }
        _ => panic!("Expected ConfigError::Host"),
    }
}

#[test]
fn test_host_config_hostname_getter() {
    let config = HostConfig::builder()
        .hostname("api.example.com")
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "api.example.com");
}

#[test]
fn test_host_config_root_directory_getter() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_dir_getter");
    fs::create_dir_all(&root_dir).unwrap();

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .build()
        .unwrap();

    assert_eq!(config.root_directory(), &Some(root_dir));
}

#[test]
fn test_host_config_default_headers_getter_none() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .build()
        .unwrap();

    assert!(
        config
            .default_headers()
            .is_none()
    );
}

#[test]
fn test_host_config_default_headers_getter_some() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .header("X-Test", "test-value")
        .build()
        .unwrap();

    assert!(
        config
            .default_headers()
            .is_some()
    );
}

#[test]
fn test_host_config_status_pages_getter_none() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .build()
        .unwrap();

    assert!(
        config
            .status_pages()
            .is_none()
    );
}

#[test]
fn test_host_config_status_pages_getter_some() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_status_pages_some");
    fs::create_dir_all(&root_dir).unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, "404.html".into());

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .status_pages(status_pages)
        .build()
        .unwrap();

    assert!(
        config
            .status_pages()
            .is_some()
    );

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_host_config_paths_getter_none() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .build()
        .unwrap();

    assert!(
        config
            .paths()
            .is_empty()
    );
}

#[test]
fn test_host_config_builder_default_hostname() {
    let config = HostConfig::builder()
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "localhost");
}

#[test]
fn test_listener_config_builder_default_port() {
    let config = ListenerConfig::default();
    assert_eq!(config.port(), 80);
}

#[test]
fn test_host_config_builder_chain() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_chain");
    fs::create_dir_all(&root_dir).unwrap();

    let tls = TlsConfig::builder()
        .cert_file("../certs/server.der")
        .key_file("../certs/server.key.der")
        .build()
        .unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, "404.html".into());

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .header("X-Custom-1", "value1")
        .header("X-Custom-2", "value2")
        .tls(tls)
        .status_pages(status_pages)
        .build()
        .unwrap();

    expect(config.hostname()).to_be(eq("example.com"));
    assert_eq!(config.root_directory(), &Some(root_dir));
    assert!(
        config
            .default_headers()
            .is_some()
    );
    assert!(
        config
            .status_pages()
            .is_some()
    );
}

#[test]
fn test_hostname_into_hostconfig() {
    let config: HostConfig = "example.com".into();
    assert_eq!(config.hostname(), "example.com")
}

#[test]
fn test_hostname_rootdir_into_hostconfig() {
    let config: HostConfig = ("example.com", "/var/vetis").into();
    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.root_directory(), &Some("/var/vetis".into()))
}

#[test]
fn test_host_config_enable_flags() {
    let config = HostConfig::builder()
        .enable_hsts(true)
        .build()
        .unwrap();

    expect(config.enable_hsts()).to_be(truthy());
}

#[test]
fn test_bind_addresses() {
    let config = HostConfig::builder()
        .bind_addresses(&[(Ipv4Addr::UNSPECIFIED.into(), 80)])
        .build()
        .unwrap();
    expect(config.bind_addresses()).to_be(eq(config.bind_addresses()));
}

#[test]
fn test_alt_service_builder() {
    let versions = vec![Version::HTTP_11, Version::HTTP_2, Version::HTTP_3];
    for version in versions {
        let alt_service = AltService::builder()
            .protocol(version)
            .autority(Authority::from_static("example.com"))
            .ma(Duration::from_mins(7200))
            .port(443)
            .persist(true)
            .build();
        let content: String = alt_service.into();
        let alpn = match version {
            Version::HTTP_11 => "http/1.1",
            Version::HTTP_2 => "h2",
            Version::HTTP_3 => "h3",
            _ => "",
        };
        expect(format!("{}", content))
            .to_be(eq(format!("{alpn}=example.com:443,ma=432000,persist=1")));
    }
}
