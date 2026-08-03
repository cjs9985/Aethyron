use argon2::{Argon2, PasswordHash, PasswordVerifier};
use bcrypt::{hash, verify};

// Add this function to handle password hashing and verification
fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, bcrypt::gen_salt())
}

fn verify_password(hashed_password: &str, provided_password: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(provided_password, hashed_password)
}

// Modify the registration function to use password hashing
pub fn register_user(username: &str, password: &str) -> Result<(), String> {
    let hashed_password = hash_password(password)?;
    // Store `hashed_password` instead of `password`
    Ok(())
}

// Modify the login function to use password verification
pub fn login_user(username: &str, password: &str) -> Result<bool, String> {
    match get_stored_password_for_user(username) {
        Some(stored_hashed_password) => verify_password(&stored_hashed_password, password).map_err(|e| e.to_string()),
        None => Err("User not found".to_string()),
    }
}

// Dummy function to simulate retrieving a stored hashed password
fn get_stored_password_for_user(_username: &str) -> Option<String> {
    Some("$argon2id$v=19$m=65536,t=3,p=8$abcdefgh$abcdefgh".to_string())
}