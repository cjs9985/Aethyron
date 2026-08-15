mod agents;
mod core;
mod memory;
mod models;
mod tools;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

async fn health() -> &'static str {
    "Aethyron API is running"
}
async fn agents() -> axum::Json<Vec<serde_json::Value>> {
    axum::Json(vec![
        serde_json::json!({
            "name": "PLANNER",
            "role": "Plans and coordinates missions"
        }),
        serde_json::json!({
            "name": "TOOLS",
            "role": "Executes tools and project operations"
        }),
        serde_json::json!({
            "name": "MEMORY",
            "role": "Stores and retrieves mission context"
        }),
    ])
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/agents", get(agents))
        .layer(CorsLayer::very_permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind Aethyron API");

    println!("Aethyron API running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Aethyron API server failed");
}