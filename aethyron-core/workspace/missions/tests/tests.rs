#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "password123";
        let hashed = hash_password(password).unwrap();
        assert!(hashed.len() > 0);
    }

    #[test]
    fn test_verify_password() {
        let password = "password123";
        let hashed = hash_password(password).unwrap();
        assert!(verify_password(password, &hashed).unwrap());
    }

    #[test]
    fn test_verify_password_wrong_password() {
        let password = "password123";
        let hashed = hash_password(password).unwrap();
        assert!(!verify_password("wrong_password", &hashed).unwrap());
    }
}