use argon2::{Argon2, PasswordHash, PasswordVerifier};
use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, bcrypt::gen_salt())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}