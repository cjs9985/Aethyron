use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: &str, email: &str, password: &str) -> Result<Self, bcrypt::BcryptError> {
        let password_hash = hash(password, bcrypt::DEFAULT_COST)?;
        Ok(User {
            id: 0,
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
        })
    }

    pub fn authenticate(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}