use std::hash::{Hash, Hasher};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        let password_hash = hash(password, DEFAULT_COST).expect("Failed to hash password");
        User {
            id: 0,
            username: username.to_string(),
            password_hash,
        }
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).expect("Failed to verify password")
    }
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.username.hash(state);
        self.id.hash(state);
    }
}