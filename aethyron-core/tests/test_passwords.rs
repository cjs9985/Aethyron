use bcrypt::{DEFAULT_COST, hash, verify};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "password123";
        let password_hash = hash(password, DEFAULT_COST).unwrap();

        assert_ne!(password, password_hash);
    }

    #[test]
    fn test_password_verification() {
        let password = "password123";
        let password_hash = hash(password, DEFAULT_COST).unwrap();

        assert!(verify(password, &password_hash).unwrap());
        assert!(!verify("wrong_password", &password_hash).unwrap());
    }

    #[test]
    fn test_edge_cases() {
        let empty_password = "";
        let empty_hash = hash(empty_password, DEFAULT_COST).unwrap();

        assert!(verify(empty_password, &empty_hash).unwrap());
        assert!(!verify("not_empty", &empty_hash).unwrap());

        let long_password = "a".repeat(72);
        let long_hash = hash(&long_password, DEFAULT_COST).unwrap();

        assert!(verify(&long_password, &long_hash).unwrap());
    }
}
