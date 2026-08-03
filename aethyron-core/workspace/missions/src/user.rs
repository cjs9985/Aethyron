use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

pub fn login(request: LoginRequest) -> LoginResponse {
    // Here you would implement your login logic
    if request.username == "admin" && request.password == "password" {
        LoginResponse { success: true, message: "Login successful!" }
    } else {
        LoginResponse { success: false, message: "Invalid credentials." }
    }
}
