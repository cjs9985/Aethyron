// src/user_model.rs

db_user! {
    username: String,
    password_hash: String,
    email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub email: String,
}

impl User {
    pub fn new(username: String, password_hash: String, email: String) -> Self {
        User { username, password_hash, email }
    }

    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
    }
}
