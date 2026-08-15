use bcrypt::{hash, verify};
use std::collections::HashMap;

pub struct AuthStore {
    passwords: HashMap<String, String>,
}

impl AuthStore {
    pub fn new() -> Self {
        AuthStore {
            passwords: HashMap::new(),
        }
    }

    pub fn store_password(&mut self, username: &str, password: &str) -> Result<(), bcrypt::BcryptError> {
        let hashed = hash(password, bcrypt::DEFAULT_COST)?;
        self.passwords.insert(username.to_string(), hashed);
        Ok(())
    }

    pub fn retrieve_stored_hashed_password(&self, username: &str) -> Option<&str> {
        self.passwords.get(username)
    }
}

pub struct EditorTool {
    auth_store: AuthStore,
}

impl EditorTool {
    pub fn new() -> Self {
        EditorTool {
            auth_store: AuthStore::new(),
        }
    }

    pub fn login_user(&self, username: &str, password: &str) -> Result<(), String> {
        match self.auth_store.retrieve_stored_hashed_password(username) {
            Some(stored_hash) => match verify(password, stored_hash) {
                Ok(true) => Ok(()),
                _ => Err("Invalid credentials".to_string()),
            },
            None => Err("User not found".to_string()),
        }
    }

    pub fn register_user(&self, username: &str, password: &str) -> Result<(), String> {
        match self.auth_store.retrieve_stored_hashed_password(username) {
            Some(_) => Err("User already exists".to_string()),
            None => self.auth_store.store_password(username, password).map_err(|e| e.to_string()),
        }
    }
}use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}use bcrypt;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}