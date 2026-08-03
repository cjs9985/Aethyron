use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct AuthService {
    password_hasher: Arc<bcrypt::BcryptHasher>,
}

impl AuthService {
    pub fn new() -> Self {
        AuthService {
            password_hasher: Arc::new(bcrypt::BcryptHasher),
        }
    }

    pub async fn hash_password(&self, password: &str) -> Result<String, bcrypt::Error> {
        self.password_hasher.hash(password, DEFAULT_COST)
    }

    pub async fn verify_password(&self, plain_password: &str, hashed_password: &str) -> Result<bool, bcrypt::Error> {
        verify(plain_password, hashed_password)
    }
}