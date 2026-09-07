use caramelo::{expect, matchers::eq};

use crate::{
    errors::{ConfigError, VetisError},
    security::SecurityConfig,
    tests::TestResult,
};
use std::io::Write;

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
fn test_security_config_cert_from_file_with_temp_file() -> TestResult<()> {
    let mut cert_path = tempfile::NamedTempFile::new()?;
    let cert_data = vec![1, 2, 3, 4, 5];
    cert_path.write(&cert_data)?;

    let config = SecurityConfig::builder()
        .cert_from_file(
            cert_path
                .path()
                .to_str()
                .unwrap(),
        )
        .key_from_bytes(vec![6, 7, 8])
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert_data);

    cert_path.close()?;

    Ok(())
}

#[test]
fn test_security_config_key_from_file_with_temp_file() -> TestResult<()> {
    let mut key_path = tempfile::NamedTempFile::new()?;
    let key_data = vec![10, 20, 30, 40, 50];
    key_path.write(&key_data)?;

    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_file(
            key_path
                .path()
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.key(), &key_data);
    key_path.close()?;
    Ok(())
}

#[test]
fn test_security_config_ca_cert_from_file_with_temp_file() -> TestResult<()> {
    let mut ca_cert_path = tempfile::NamedTempFile::new()?;
    let ca_cert_data = vec![100, 200, 255];
    ca_cert_path
        .write(&ca_cert_data)
        .unwrap();

    let config = SecurityConfig::builder()
        .cert_from_bytes(vec![1, 2, 3])
        .key_from_bytes(vec![4, 5, 6])
        .ca_cert_from_file(
            ca_cert_path
                .path()
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.ca_cert(), &Some(ca_cert_data));
    ca_cert_path.close()?;
    Ok(())
}

#[test]
fn test_security_config_build_from_temp_files() -> TestResult<()> {
    let mut cert_path = tempfile::NamedTempFile::new()?;
    let mut key_path = tempfile::NamedTempFile::new()?;

    let cert_data = vec![1, 2, 3];
    let key_data = vec![4, 5, 6];

    cert_path
        .write(&cert_data)
        .unwrap();
    key_path
        .write(&key_data)
        .unwrap();

    let config = SecurityConfig::builder()
        .cert_from_file(
            cert_path
                .path()
                .to_str()
                .unwrap(),
        )
        .key_from_file(
            key_path
                .path()
                .to_str()
                .unwrap(),
        )
        .build()
        .unwrap();

    assert_eq!(config.cert(), &cert_data);
    assert_eq!(config.key(), &key_data);

    cert_path.close()?;
    key_path.close()?;

    Ok(())
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

#[test]
fn test_security_from_pair() {
    let config: SecurityConfig =
        (vec![1u8, 2u8, 4u8], vec![4u8, 5u8, 6u8], Some(vec![7u8, 8u8, 9u8])).into();
    expect(config.cert()).to_be(eq(&vec![1u8, 2u8, 4u8]));
}
