use crate::auth::{Algorithm, BasicAuthConfig};
use std::{collections::HashMap, fs};

fn create_temp_htpasswd(content: &str) -> String {
    let path = format!("test_htpasswd_{}.tmp", std::process::id());
    fs::write(&path, content).unwrap();
    path
}

fn cleanup_temp_file(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_algorithm_bcrypt_variant() {
    let bcrypt = Algorithm::BCrypt;
    assert_eq!(bcrypt, Algorithm::BCrypt);
}

#[test]
fn test_algorithm_argon2_variant() {
    let argon2 = Algorithm::Argon2;
    assert_eq!(argon2, Algorithm::Argon2);
}

#[test]
fn test_algorithm_partial_eq() {
    assert_eq!(Algorithm::BCrypt, Algorithm::BCrypt);
    assert_eq!(Algorithm::Argon2, Algorithm::Argon2);
    assert_ne!(Algorithm::BCrypt, Algorithm::Argon2);
}

#[test]
fn test_algorithm_debug() {
    let bcrypt = format!("{:?}", Algorithm::BCrypt);
    assert_eq!(bcrypt, "BCrypt");

    let argon2 = format!("{:?}", Algorithm::Argon2);
    assert_eq!(argon2, "Argon2");
}

#[test]
fn test_basic_auth_config_builder_build_success() {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), "hashed_password".to_string());

    let config = BasicAuthConfig::builder()
        .users(users.clone())
        .algorithm(Algorithm::Argon2)
        .build()
        .unwrap();

    assert_eq!(config.users(), &users);
    assert_eq!(config.algorithm(), &Algorithm::Argon2);
    assert_eq!(config.htpasswd(), &None);
}

#[test]
fn test_basic_auth_config_builder_build_with_htpasswd_file() {
    let path = create_temp_htpasswd("user1:$2y$10$hash1\n");
    let path_for_cleanup = path.clone();
    let config = BasicAuthConfig::builder()
        .htpasswd(&path)
        .build()
        .unwrap();

    assert_eq!(config.htpasswd(), &Some(path_for_cleanup.clone()));
    cleanup_temp_file(&path_for_cleanup);
}

#[test]
fn test_basic_auth_config_builder_build_htpasswd_not_found() {
    let result = BasicAuthConfig::builder()
        .htpasswd("/nonexistent/.htpasswd")
        .build();

    assert!(result.is_err());
}

#[test]
fn test_basic_auth_config_builder_cache_users_valid_file() {
    let content = "user1:$2y$10$hash1\nuser2:$2y$10$hash2\n\nuser3:$2y$10$hash3\n";
    let path = create_temp_htpasswd(content);

    let config = BasicAuthConfig::builder()
        .htpasswd(&path)
        .cache_users()
        .build()
        .unwrap();

    assert_eq!(config.users().len(), 3);
    assert_eq!(
        config
            .users()
            .get("user1"),
        Some(&"$2y$10$hash1".to_string())
    );
    assert_eq!(
        config
            .users()
            .get("user2"),
        Some(&"$2y$10$hash2".to_string())
    );
    assert_eq!(
        config
            .users()
            .get("user3"),
        Some(&"$2y$10$hash3".to_string())
    );
    cleanup_temp_file(&path);
}

#[test]
fn test_basic_auth_config_builder_cache_users_malformed_lines() {
    let content =
        "valid_user:$2y$10$hash\ninvalid_line_without_colon\nanother_valid:$2y$10$hash2\n";
    let path = create_temp_htpasswd(content);

    let config = BasicAuthConfig::builder()
        .htpasswd(&path)
        .cache_users()
        .build()
        .unwrap();

    assert_eq!(config.users().len(), 2);
    assert_eq!(
        config
            .users()
            .get("valid_user"),
        Some(&"$2y$10$hash".to_string())
    );
    assert_eq!(
        config
            .users()
            .get("another_valid"),
        Some(&"$2y$10$hash2".to_string())
    );
    assert!(config
        .users()
        .get("invalid_line_without_colon")
        .is_none());
    cleanup_temp_file(&path);
}

#[test]
fn test_basic_auth_config_builder_cache_users_nonexistent_file() {
    let config = BasicAuthConfig::builder()
        .htpasswd("/nonexistent/.htpasswd")
        .cache_users()
        .build();

    assert!(config.is_err());
}

#[test]
fn test_basic_auth_config_builder_cache_users_no_htpasswd() {
    let config = BasicAuthConfig::builder()
        .cache_users()
        .build()
        .unwrap();

    assert!(config
        .users()
        .is_empty());
}

#[test]
fn test_basic_auth_config_builder_chain() {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), "hashed_password".to_string());

    let config = BasicAuthConfig::builder()
        .users(users.clone())
        .algorithm(Algorithm::Argon2)
        .build()
        .unwrap();

    assert_eq!(config.users(), &users);
    assert_eq!(config.algorithm(), &Algorithm::Argon2);
}

#[test]
fn test_basic_auth_config_users_getter() {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), "hashed_password".to_string());
    users.insert("user".to_string(), "another_hash".to_string());

    let config = BasicAuthConfig::builder()
        .users(users.clone())
        .build()
        .unwrap();

    assert_eq!(config.users(), &users);
}

#[test]
fn test_basic_auth_config_algorithm_getter() {
    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::BCrypt)
        .build()
        .unwrap();

    assert_eq!(config.algorithm(), &Algorithm::BCrypt);

    let config = BasicAuthConfig::builder()
        .algorithm(Algorithm::Argon2)
        .build()
        .unwrap();

    assert_eq!(config.algorithm(), &Algorithm::Argon2);
}

#[test]
fn test_basic_auth_config_htpasswd_getter() {
    let config = BasicAuthConfig::builder()
        .build()
        .unwrap();

    assert_eq!(config.htpasswd(), &None);

    let path = create_temp_htpasswd("user1:$2y$10$hash1\n");
    let path_for_cleanup = path.clone();
    let path_for_assertion = path.clone();
    let config = BasicAuthConfig::builder()
        .htpasswd(&path)
        .build()
        .unwrap();

    assert_eq!(config.htpasswd(), &Some(path_for_assertion));
    cleanup_temp_file(&path_for_cleanup);
}

#[test]
fn test_basic_auth_config_default_algorithm() {
    let config = BasicAuthConfig::builder()
        .build()
        .unwrap();
    assert_eq!(config.algorithm(), &Algorithm::BCrypt);
}

#[test]
fn test_basic_auth_config_empty_users() {
    let config = BasicAuthConfig::builder()
        .build()
        .unwrap();
    assert!(config
        .users()
        .is_empty());
}

#[test]
fn test_basic_auth_config_multiple_users() {
    let mut users = HashMap::new();
    users.insert("user1".to_string(), "hash1".to_string());
    users.insert("user2".to_string(), "hash2".to_string());
    users.insert("user3".to_string(), "hash3".to_string());

    let config = BasicAuthConfig::builder()
        .users(users.clone())
        .build()
        .unwrap();

    assert_eq!(config.users().len(), 3);
    assert_eq!(config.users(), &users);
}

#[test]
fn test_basic_auth_config_clone() {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), "hashed_password".to_string());

    let config = BasicAuthConfig::builder()
        .users(users.clone())
        .build()
        .unwrap();

    let cloned_config = config.clone();
    assert_eq!(cloned_config.users(), config.users());
    assert_eq!(cloned_config.algorithm(), config.algorithm());
    assert_eq!(cloned_config.htpasswd(), config.htpasswd());
}
