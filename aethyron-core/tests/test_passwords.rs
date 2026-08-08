use argon2::{Argon2, Config};
use bcrypt::{hash, verify};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "password123";
        let config = Config::default();
        let argon2 = Argon2::new(&config);
        let hash = argon2.hash_encoded(password.as_bytes(), b"salt").unwrap();
        assert_ne!(password, &hash);
    }

    #[test]
    fn test_password_verification() {
        let password = "password123";
        let config = Config::default();
        let argon2 = Argon2::new(&config);
        let hash = argon2.hash_encoded(password.as_bytes(), b"salt").unwrap();
        assert!(verify(password, &hash).unwrap());
        assert!(!verify("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_edge_cases() {
        assert!(argon2::hash_encoded(b"", b"salt").is_err());
        assert!(argon2::hash_encoded(&b"a".repeat(72)[..], b"salt").is_err());
    }
}