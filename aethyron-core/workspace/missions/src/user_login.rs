use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use argon2;
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login_user(
    data: web::Json<LoginRequest>,
) -> impl Responder {
    let user = get_user_by_username(&data.username).await;
    if let Some(user) = user {
        match verify_password(&user.password_hash, &data.password) {
            Ok(true) => HttpResponse::Ok().body("Login successful"),
            _ => HttpResponse::Unauthorized().body("Invalid credentials"),
        }
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

async fn get_user_by_username(username: &str) -> Option<User> {
    // Replace with actual user retrieval logic
    None
}

fn verify_password(password_hash: &str, password: &str) -> argon2::Result<bool> {
    let config = argon2::Config::default();
    argon2::verify_encoded(password_hash, password.as_bytes(), &config)
}

#[derive(Deserialize)]
struct User {
    username: String,
    password_hash: String,
}

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(login_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}