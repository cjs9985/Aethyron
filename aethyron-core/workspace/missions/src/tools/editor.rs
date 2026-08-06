use std::path::PathBuf;
use crate::core::auth::{self, User};

#[derive(Debug)]
pub struct EditorTool {
    user: Option<User>,
}

impl EditorTool {
    pub fn new() -> Self {
        EditorTool { user: None }
    }

    pub async fn register(&mut self, username: &str, password: &str) -> Result<(), String> {
        let hashed_password = auth::hash_password(password)?;
        self.user = Some(User::new(username, hashed_password));
        Ok(())
    }

    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), String> {
        match &self.user {
            Some(user) if user.verify_password(password).await => Ok(()),
            _ => Err("Invalid credentials".to_string()),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }
}