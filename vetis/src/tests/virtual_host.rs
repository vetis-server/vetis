use crate::{
    errors::{ConfigError, VetisError},
    security::SecurityConfig,
    virtual_host::VirtualHostConfig,
};
use std::{collections::HashMap, fs};

#[test]
fn test_virtual_host_config_build_success() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_build_success");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.port(), 80);
    assert_eq!(
        config.root_directory(),
        root_dir
            .to_str()
            .unwrap()
    );
    assert!(config
        .default_headers()
        .is_none());
    assert!(config
        .security()
        .is_none());
    assert!(config
        .status_pages()
        .is_none());
    assert!(config.enable_logging());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_with_port() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_port");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .port(443)
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.port(), 443);

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_with_header() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_header");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
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

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_with_multiple_headers() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_multiple_headers");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
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

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_with_security() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_security");
    fs::create_dir_all(&root_dir).unwrap();

    let security = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .build()
        .unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .security(security)
        .build()
        .unwrap();

    assert!(config
        .security()
        .is_some());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_with_status_pages() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_status_pages");
    fs::create_dir_all(&root_dir).unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, String::from("404.html"));
    status_pages.insert(500, String::from("500.html"));

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
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
fn test_virtual_host_config_build_with_logging_disabled() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_logging_disabled");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .enable_logging(false)
        .build()
        .unwrap();

    assert!(!config.enable_logging());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_full() {
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

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .port(8443)
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .header("X-Custom", "value")
        .security(security)
        .status_pages(status_pages)
        .enable_logging(false)
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.port(), 8443);
    assert_eq!(
        config.root_directory(),
        root_dir
            .to_str()
            .unwrap()
    );
    assert!(config
        .default_headers()
        .is_some());
    assert!(config
        .security()
        .is_some());
    assert!(config
        .status_pages()
        .is_some());
    assert!(!config.enable_logging());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_missing_hostname() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_missing_hostname");
    fs::create_dir_all(&root_dir).unwrap();

    let result = VirtualHostConfig::builder()
        .hostname("")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build();

    assert!(result.is_err());
    match result {
        Err(VetisError::Config(ConfigError::VirtualHost(msg))) => {
            assert_eq!(msg, "Missing hostname");
        }
        _ => panic!("Expected ConfigError::VirtualHost"),
    }

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_build_missing_root_directory() {
    let result = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory("")
        .build();

    assert!(result.is_err());
    match result {
        Err(VetisError::Config(ConfigError::VirtualHost(msg))) => {
            assert_eq!(msg, "Missing root directory");
        }
        _ => panic!("Expected ConfigError::VirtualHost"),
    }
}

#[test]
fn test_virtual_host_config_build_nonexistent_root_directory() {
    let result = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory("/nonexistent/path/to/root")
        .build();

    assert!(result.is_err());
    match result {
        Err(VetisError::Config(ConfigError::VirtualHost(msg))) => {
            assert!(msg.contains("root_directory does not exist"));
        }
        _ => panic!("Expected ConfigError::VirtualHost"),
    }
}

#[test]
fn test_virtual_host_config_hostname_getter() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_hostname_getter");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("api.example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "api.example.com");

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_port_getter() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_port_getter");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .port(8080)
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.port(), 8080);

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_root_directory_getter() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_dir_getter");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(
        config.root_directory(),
        root_dir
            .to_str()
            .unwrap()
    );

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_default_headers_getter_none() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_headers_none");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert!(config
        .default_headers()
        .is_none());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_default_headers_getter_some() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_headers_some");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .header("X-Test", "test-value")
        .build()
        .unwrap();

    assert!(config
        .default_headers()
        .is_some());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_security_getter_none() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_security_none");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert!(config
        .security()
        .is_none());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_security_getter_some() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_security_some");
    fs::create_dir_all(&root_dir).unwrap();

    let security = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .build()
        .unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .security(security)
        .build()
        .unwrap();

    assert!(config
        .security()
        .is_some());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_status_pages_getter_none() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_status_pages_none");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert!(config
        .status_pages()
        .is_none());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_status_pages_getter_some() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_status_pages_some");
    fs::create_dir_all(&root_dir).unwrap();

    let mut status_pages = HashMap::new();
    status_pages.insert(404, String::from("404.html"));

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .status_pages(status_pages)
        .build()
        .unwrap();

    assert!(config
        .status_pages()
        .is_some());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_enable_logging_getter_true() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_logging_true");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .enable_logging(true)
        .build()
        .unwrap();

    assert!(config.enable_logging());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_enable_logging_getter_false() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_logging_false");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .enable_logging(false)
        .build()
        .unwrap();

    assert!(!config.enable_logging());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_paths_getter_none() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_paths_none");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert!(config
        .paths()
        .is_none());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_builder_default_hostname() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_default_hostname");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "localhost");

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_builder_default_port() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_default_port");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.port(), 80);

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_builder_default_logging() {
    let temp_dir = std::env::temp_dir();
    let root_dir = temp_dir.join("test_vetis_root_default_logging");
    fs::create_dir_all(&root_dir).unwrap();

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert!(config.enable_logging());

    fs::remove_dir_all(&root_dir).unwrap();
}

#[test]
fn test_virtual_host_config_builder_chain() {
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

    let config = VirtualHostConfig::builder()
        .hostname("example.com")
        .port(8443)
        .root_directory(
            root_dir
                .to_str()
                .unwrap(),
        )
        .header("X-Custom-1", "value1")
        .header("X-Custom-2", "value2")
        .security(security)
        .status_pages(status_pages)
        .enable_logging(false)
        .build()
        .unwrap();

    assert_eq!(config.hostname(), "example.com");
    assert_eq!(config.port(), 8443);
    assert_eq!(
        config.root_directory(),
        root_dir
            .to_str()
            .unwrap()
    );
    assert!(config
        .default_headers()
        .is_some());
    assert!(config
        .security()
        .is_some());
    assert!(config
        .status_pages()
        .is_some());
    assert!(!config.enable_logging());

    fs::remove_dir_all(&root_dir).unwrap();
}
