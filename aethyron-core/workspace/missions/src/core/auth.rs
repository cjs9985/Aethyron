use bcrypt::{hash, verify, DEFAULT_COST};
use std::error::Error;

pub struct User {
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, password: &str) -> Result<Self, Box<dyn Error>> {
        let password_hash = hash(password, DEFAULT_COST)?;
        Ok(User { username, password_hash })
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, Box<dyn Error>> {
        verify(password, &self.password_hash)
    }
}

pub struct Auth {
    users: Vec<User>,
}

impl Auth {
    pub fn new() -> Self {
        Auth { users: Vec::new() }
    }

    pub fn register(&mut self, username: String, password: &str) -> Result<(), Box<dyn Error>> {
        if self.users.iter().any(|user| user.username == username) {
            return Err("Username already exists".into());
        }
        let user = User::new(username, password)?;
        self.users.push(user);
        Ok(())
    }

    pub fn login(&self, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        if let Some(user) = self.users.iter().find(|user| user.username == username) {
            return user.verify_password(password);
        }
        Err("Invalid username or password".into())
    }
}