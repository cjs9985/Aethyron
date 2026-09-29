use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "password123";
        let hashed_password = hash_password(password)?;
        assert_ne!(password, hashed_password);
    }

    #[test]
    fn test_verify_password() {
        let password = "password123";
        let hashed_password = hash_password(password)?;
        assert!(verify_password(password, &hashed_password)?);
        assert!(!verify_password("wrong_password", &hashed_password)?);
    }
}