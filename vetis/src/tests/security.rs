use crate::security::Tls;

#[test]
fn test_tls_config_build_success() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];

    let config = Tls::from_cert_and_key(&cert, &key);

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert_eq!(config.ca(), &None);
    assert!(!config.client_auth());
}

#[test]
fn test_tls_with_ca_cert() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];
    let ca_cert = vec![7, 8, 9];

    let config = Tls::from_cert_and_key(&cert, &key).with_ca(&ca_cert);

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert_eq!(config.ca(), &Some(ca_cert));
    assert!(!config.client_auth());
}

#[test]
fn test_tls_with_client_auth() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];

    let config = Tls::from_cert_and_key(&cert, &key).with_client_auth(true);

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert!(config.client_auth());
}

#[test]
fn test_tls_config_build_full() {
    let cert = vec![1, 2, 3];
    let key = vec![4, 5, 6];
    let ca_cert = vec![7, 8, 9];

    let config = Tls::from_cert_and_key(&cert, &key)
        .with_ca(&ca_cert)
        .with_client_auth(true);

    assert_eq!(config.cert(), &cert);
    assert_eq!(config.key(), &key);
    assert_eq!(config.ca(), &Some(ca_cert));
    assert!(config.client_auth());
}

#[test]
fn test_tls_config_cert_getter() {
    let cert = vec![1, 2, 3, 4, 5];
    let config = Tls::from_cert_and_key(&cert, &vec![6, 7, 8]);
    assert_eq!(config.cert(), &cert);
}

#[test]
fn test_tls_config_key_getter() {
    let key = vec![10, 20, 30, 40];
    let config = Tls::from_cert_and_key(&vec![1, 2, 3], &key);
    assert_eq!(config.key(), &key);
}

#[test]
fn test_tls_config_ca_cert_getter_none() {
    let config = Tls::from_cert_and_key(&vec![1, 2, 3], &vec![4, 5, 6]);
    assert_eq!(config.ca(), &None);
}

#[test]
fn test_tls_config_ca_cert_getter_some() {
    let ca_cert = vec![100, 200, 255];
    let config = Tls::from_cert_and_key(&vec![1, 2, 3], &vec![4, 5, 6]).with_ca(&ca_cert);

    assert_eq!(config.ca(), &Some(ca_cert));
}

#[test]
fn test_tls_config_client_auth_getter_true() {
    let config = Tls::from_cert_and_key(&vec![1, 2, 3], &vec![4, 5, 6]).with_client_auth(true);
    assert!(config.client_auth());
}

#[test]
fn test_tls_config_client_auth_getter_false() {
    let config = Tls::from_cert_and_key(&vec![1, 2, 3], &vec![4, 5, 6]).with_client_auth(false);
    assert!(!config.client_auth());
}
