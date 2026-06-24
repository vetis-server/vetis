use crate::{
    errors::{ConfigError, VetisError},
    security::SecurityConfig,
};
use std::fs;

#[test]
fn test_security_config_build_success() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];

    let config = SecurityConfig::builder()
        .cert_from_bytes(cert.clone())
        .key_from_bytes(key.clone())
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert_eq!(config.ca_cert(), &None);
    assert!(!config.client_auth());
}

#[test]
fn test_security_config_build_with_ca_cert() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];
    let ca_cert = vec![7, 8, 9];

    let config = SecurityConfig::builder()
        .cert_from_bytes(cert.clone())
        .key_from_bytes(key.clone())
        .ca_cert_from_bytes(ca_cert.clone())
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert_eq!(config.ca_cert(), &Some(ca_cert));
    assert!(!config.client_auth());
}

#[test]
fn test_security_config_build_with_client_auth() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];

    let config = SecurityConfig::builder()
        .cert_from_bytes(cert.clone())
        .key_from_bytes(key.clone())
        .client_auth(true)
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert!(config.client_auth());
}

#[test]
fn test_security_config_build_full() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];
    let ca_cert = vec![7, 8, 9];

    let config = SecurityConfig::builder()
        .cert_from_bytes(cert.clone())
        .key_from_bytes(key.clone())
        .ca_cert_from_bytes(ca_cert.clone())
        .client_auth(true)
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert_eq!(config.ca_cert(), &Some(ca_cert));
    assert!(config.client_auth());
}

#[test]
fn test_security_config_build_missing_cert() {
    let result = SecurityConfig::builder()
        .key_from_bytes(vec![1, 2, 3])
        .build();

    assert!(result.is_err());
    match result {
        Err(VetisError::Config(ConfigError::Security(msg))) => {
            assert_eq!(msg, "Missing certificate");
        }
        _ => panic!("Expected ConfigError::Security"),
    }
}

#[test]
fn test_security_config_build_missing_key() {
    let result = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .build();

    assert!(result.is_err());
    match result {
        Err(VetisError::Config(ConfigError::Security(msg))) => {
            assert_eq!(msg, "Missing key");
        }
        _ => panic!("Expected ConfigError::Security"),
    }
}

#[test]
fn test_security_config_build_empty_cert() {
    let result = SecurityConfig::builder()
        .cert_from_bytes(vec![])
        .key_from_bytes(vec![1, 2, 3])
        .build();

    assert!(result.is_err());
}

#[test]
fn test_security_config_build_empty_key() {
    let result = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![])
        .build();

    assert!(result.is_err());
}

#[test]
fn test_security_config_cert_getter() {
    let cert = vec![1, 2, 3, 4, 5];
    let config = SecurityConfig::builder()
        .cert_from_bytes(cert.clone())
        .key_from_bytes(vec![6, 7, 8])
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert);
}

#[test]
fn test_security_config_key_getter() {
    let key = vec![10, 20, 30, 40];
    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(key.clone())
        .build()
        .unwrap();

    assert_eq!(config.key(), &key);
}

#[test]
fn test_security_config_ca_cert_getter_none() {
    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .build()
        .unwrap();

    assert_eq!(config.ca_cert(), &None);
}

#[test]
fn test_security_config_ca_cert_getter_some() {
    let ca_cert = vec![100, 200, 255];
    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .ca_cert_from_bytes(ca_cert.clone())
        .build()
        .unwrap();

    assert_eq!(config.ca_cert(), &Some(ca_cert));
}

#[test]
fn test_security_config_client_auth_getter_true() {
    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .client_auth(true)
        .build()
        .unwrap();

    assert!(config.client_auth());
}

#[test]
fn test_security_config_client_auth_getter_false() {
    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .client_auth(false)
        .build()
        .unwrap();

    assert!(!config.client_auth());
}

#[test]
fn test_security_config_builder_chain() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];
    let ca_cert = vec![7, 8, 9];

    let config = SecurityConfig::builder()
        .cert_from_bytes(cert.clone())
        .key_from_bytes(key.clone())
        .ca_cert_from_bytes(ca_cert.clone())
        .client_auth(true)
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert_eq!(config.ca_cert(), &Some(ca_cert));
    assert!(config.client_auth());
}

#[test]
fn test_security_config_clone() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];

    let config = SecurityConfig::builder()
        .cert_from_bytes(cert.clone())
        .key_from_bytes(key.clone())
        .build()
        .unwrap();

    let cloned = config.clone();

    assert_eq!(cloned.cert(), config.cert());
    assert_eq!(cloned.key(), config.key());
    assert_eq!(cloned.ca_cert(), config.ca_cert());
    assert_eq!(cloned.client_auth(), config.client_auth());
}

#[test]
fn test_security_config_cert_from_file_with_temp_file() {
    let temp_dir = std::env::temp_dir();
    let cert_path = temp_dir.join("test_cert.der");
    let cert_data = vec![1, 2, 3, 4, 5];

    fs::write(&cert_path, &cert_data).unwrap();

    let config = SecurityConfig::builder()
        .cert_from_file(
            cert_path
                .to_str()
                .unwrap(),
        )
        .key_from_bytes(vec![6, 7, 8])
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert_data);

    fs::remove_file(&cert_path).unwrap();
}

#[test]
fn test_security_config_key_from_file_with_temp_file() {
    let temp_dir = std::env::temp_dir();
    let key_path = temp_dir.join("test_key.der");
    let key_data = vec![10, 20, 30, 40, 50];

    fs::write(&key_path, &key_data).unwrap();

    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_file(
            key_path
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.key(), &key_data);

    fs::remove_file(&key_path).unwrap();
}

#[test]
fn test_security_config_ca_cert_from_file_with_temp_file() {
    let temp_dir = std::env::temp_dir();
    let ca_cert_path = temp_dir.join("test_ca.der");
    let ca_cert_data = vec![100, 200, 255];

    fs::write(&ca_cert_path, &ca_cert_data).unwrap();

    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .ca_cert_from_file(
            ca_cert_path
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.ca_cert(), &Some(ca_cert_data));

    fs::remove_file(&ca_cert_path).unwrap();
}

#[test]
fn test_security_config_build_from_temp_files() {
    let temp_dir = std::env::temp_dir();
    let cert_path = temp_dir.join("test_cert.der");
    let key_path = temp_dir.join("test_key.der");

    let cert_data = vec![1, 2, 3];
    let key_data = vec![4, 5, 6];

    fs::write(&cert_path, &cert_data).unwrap();
    fs::write(&key_path, &key_data).unwrap();

    let config = SecurityConfig::builder()
        .cert_from_file(
            cert_path
                .to_str()
                .unwrap(),
        )
        .key_from_file(
            key_path
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert_data);
    assert_eq!(config.key(), &key_data);

    fs::remove_file(&cert_path).unwrap();
    fs::remove_file(&key_path).unwrap();
}

#[test]
fn test_security_config_cert_from_file_nonexistent_build_fails() {
    let result = SecurityConfig::builder()
        .cert_from_file("/nonexistent/path/to/cert.der")
        .key_from_bytes(vec![1, 2, 3])
        .build();

    assert!(result.is_err());
}

#[test]
fn test_security_config_key_from_file_nonexistent_build_fails() {
    let result = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_file("/nonexistent/path/to/key.der")
        .build();

    assert!(result.is_err());
}

#[test]
fn test_security_config_ca_cert_from_file_nonexistent() {
    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .ca_cert_from_file("/nonexistent/path/to/ca.der")
        .build()
        .unwrap();

    assert_eq!(config.ca_cert(), &None);
}
