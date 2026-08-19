use crate::{
    errors::{ConfigError, VetisError},
    host::HostConfig,
    listener::{self, ListenerConfig},
    security::SecurityConfig,
};
use std::{collections::HashMap, error::Error, fs};

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
    assert!(config
        .default_headers()
        .is_none());
    assert!(config
        .status_pages()
        .is_none());
    assert!(config.enable_logging());

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

    assert!(config
        .default_headers()
        .is_some());
    let headers = config
        .default_headers()
        .as_ref()
        .unwrap();
    assert_eq!(headers.len(), 1);
    assert_eq!(headers[0], (String::from("X-Custom"), String::from("value")));
}

#[test]
fn test_host_config_build_with_multiple_headers() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .header("X-Custom-1", "value1")
        .header("X-Custom-2", "value2")
        .build()
        .unwrap();

    assert!(config
        .default_headers()
        .is_some());
    let headers = config
        .default_headers()
        .as_ref()
        .unwrap();
    assert_eq!(headers.len(), 2);
    assert_eq!(headers[0], (String::from("X-Custom-1"), String::from("value1")));
    assert_eq!(headers[1], (String::from("X-Custom-2"), String::from("value2")));
}

#[test]
fn test_host_config_build_with_security() -> Result<(), Box<dyn Error>> {
    let security = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .build()
        .unwrap();

    let host = HostConfig::builder()
        .security(security.clone())
        .build()
        .unwrap();

    let host_security = host
        .security()
        .as_ref()
        .unwrap();

    assert_eq!(host_security.cert(), security.cert());
    assert_eq!(host_security.key(), security.key());

    Ok(())
}

#[test]
fn test_host_config_build_with_status_pages() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_status_pages");
    fs::create_dir_all(&root_dir).unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, String::from("404.html"));
    status_pages.insert(500, String::from("500.html"));

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .status_pages(status_pages)
        .build()
        .unwrap();

    assert!(config
        .status_pages()
        .is_some());
    let pages = config
        .status_pages()
        .as_ref()
        .unwrap();
    assert_eq!(pages.len(), 2);
    assert_eq!(pages.get(&404), Some(&String::from("404.html")));
    assert_eq!(pages.get(&500), Some(&String::from("500.html")));

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_host_config_build_with_logging_disabled() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .enable_logging(false)
        .build()
        .unwrap();

    assert!(!config.enable_logging());
}

#[test]
fn test_host_config_build_full() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_full");
    fs::create_dir_all(&root_dir).unwrap();

    let security = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .build()
        .unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, String::from("404.html"));

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .header("X-Custom", "value")
        .security(security)
        .status_pages(status_pages)
        .enable_logging(false)
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.root_directory(), &Some(root_dir.clone()));
    assert!(config
        .default_headers()
        .is_some());
    assert!(config
        .status_pages()
        .is_some());
    assert!(!config.enable_logging());

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
        .root_directory("/nonexistent/path/to/root".into())
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

    assert!(config
        .default_headers()
        .is_none());
}

#[test]
fn test_host_config_default_headers_getter_some() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .header("X-Test", "test-value")
        .build()
        .unwrap();

    assert!(config
        .default_headers()
        .is_some());
}

#[test]
fn test_host_config_status_pages_getter_none() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .build()
        .unwrap();

    assert!(config
        .status_pages()
        .is_none());
}

#[test]
fn test_host_config_status_pages_getter_some() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_status_pages_some");
    fs::create_dir_all(&root_dir).unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, String::from("404.html"));

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .status_pages(status_pages)
        .build()
        .unwrap();

    assert!(config
        .status_pages()
        .is_some());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_host_config_enable_logging_getter_true() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .enable_logging(true)
        .build()
        .unwrap();

    assert!(config.enable_logging());
}

#[test]
fn test_host_config_enable_logging_getter_false() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .enable_logging(false)
        .build()
        .unwrap();

    assert!(!config.enable_logging());
}

#[test]
fn test_host_config_paths_getter_none() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .build()
        .unwrap();

    assert!(config
        .paths()
        .is_none());
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
fn test_host_config_builder_default_logging() {
    let config = HostConfig::builder()
        .hostname("example.com")
        .build()
        .unwrap();

    assert!(config.enable_logging());
}

#[test]
fn test_host_config_builder_chain() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_chain");
    fs::create_dir_all(&root_dir).unwrap();

    let security = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .build()
        .unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, String::from("404.html"));

    let config = HostConfig::builder()
        .hostname("example.com")
        .root_directory(root_dir.clone())
        .header("X-Custom-1", "value1")
        .header("X-Custom-2", "value2")
        .security(security)
        .status_pages(status_pages)
        .enable_logging(false)
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.root_directory(), &Some(root_dir));
    assert!(config
        .default_headers()
        .is_some());
    assert!(config
        .status_pages()
        .is_some());
    assert!(!config.enable_logging());
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
