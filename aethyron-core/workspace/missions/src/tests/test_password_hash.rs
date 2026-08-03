use crate::models::password_hash::{hash_password, verify_password};
use std::str;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "secure_password123";
        let hashed_password = hash_password(password).unwrap();
        assert!(verify_password(hashed_password.as_str(), password).is_ok());
    }

    #[test]
    fn test_wrong_password_verification() {
        let password = "secure_password123";
        let hashed_password = hash_password(password).unwrap();
        assert!(!verify_password(hashed_password.as_str(), "wrong_password").is_ok());
    }
}