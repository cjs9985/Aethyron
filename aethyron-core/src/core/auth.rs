use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<bool, bcrypt::BcryptError> {
    verify(password, password_hash)
}