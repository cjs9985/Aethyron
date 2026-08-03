use std::collections::HashMap;

pub struct AuthToken {
    token: String,
}

impl AuthToken {
    pub fn new(token: &str) -> Self {
        AuthToken { token: token.to_string() }
    }

    pub fn is_valid(&self, valid_tokens: &HashSet<String>) -> bool {
        valid_tokens.contains(&self.token)
    }
}

pub struct AuthService {
    valid_tokens: HashSet<String>,
}

impl AuthService {
    pub fn new(valid_tokens: Vec<&str>) -> Self {
        AuthService { valid_tokens: valid_tokens.into_iter().map(|t| t.to_string()).collect() }
    }

    pub fn authenticate(&self, token_str: &str) -> Option<AuthToken> {
        if self.valid_tokens.contains(token_str) {
            Some(AuthToken::new(token_str))
        } else {
            None
        }
    }
}