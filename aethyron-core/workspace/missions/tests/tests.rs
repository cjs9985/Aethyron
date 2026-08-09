#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() -> Result<(), Box<dyn std::error::Error>> {
        let password = "password123";
        let hash_result = hash_password(password)?;
        assert!(hash_result.is_ok());
        let hash = hash_result?;
        assert_ne!(hash, password);

        // Verify the same password hashes to the same value
        let verify_result = verify_password(password, &hash)?;
        assert!(verify_result.is_ok());
        assert!(verify_result?);
        Ok(())
    }

    #[test]
    fn test_verify_password() -> Result<(), Box<dyn std::error::Error>> {
        let password = "password123";
        let hash = hash_password(password)?;
        let verify_result = verify_password("wrongpassword", &hash)?;
        assert!(verify_result.is_err());

        // Verify with the correct password
        let verify_result = verify_password(password, &hash)?;
        assert!(verify_result.is_ok());
        assert!(verify_result?);
        Ok(())
    }
}