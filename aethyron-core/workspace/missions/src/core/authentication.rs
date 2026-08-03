use argon2::{Argon2, PasswordHasher};
use bcrypt::Bcrypt;
use std::error::Error;

pub trait Authentication {
    fn register(username: &str, password: &str) -> Result<(), Box<dyn Error>>;
    fn login(username: &str, password: &str) -> Result<bool, Box<dyn Error>>;
}

impl Authentication for Argon2 {
    fn register(username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let argon2 = Argon2::default();
        let hash = argon2.hash_password(password.as_bytes(), b"").unwrap();
        // Store the username and hash in your database
        Ok(())
    }

    fn login(username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        let argon2 = Argon2::default();
        let hash = // Retrieve the hash from your database for the given username
        argon2.verify_password(password.as_bytes(), &hash).is_ok()
    }
}

impl Authentication for Bcrypt {
    fn register(username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let bcrypt = Bcrypt::new();
        let hash = bcrypt.hash(password)?;
        // Store the username and hash in your database
        Ok(())
    }

    fn login(username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        let bcrypt = Bcrypt::new();
        let hash = // Retrieve the hash from your database for the given username
        bcrypt.verify(password, &hash)
    }
}