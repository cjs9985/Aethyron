mod agents;
mod core;
mod memory;
mod models;
mod tools;

use axum::{Router, routing::get};
use std::env;
use tower_http::cors::CorsLayer;

use crate::core::orchestrator::{Mission, Orchestrator};
use crate::core::project_indexer::ProjectIndexer;
use crate::models::ollama::OllamaClient;

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

async fn run_doctor() {
    println!("==============================");
    println!("AETHYRON DOCTOR");
    println!("==============================");

    let cargo_ok = std::path::Path::new("Cargo.toml").is_file();
    println!(
        "{} Cargo.toml exists",
        if cargo_ok { "PASS" } else { "FAIL" }
    );

    let src_ok = std::path::Path::new("src").is_dir();
    println!(
        "{} src directory exists",
        if src_ok { "PASS" } else { "FAIL" }
    );

    match ProjectIndexer::build(std::path::Path::new(".")) {
        Ok(index) => {
            println!("PASS Project workspace can be indexed");
            println!("     Files indexed: {}", index.files.len());
        }
        Err(error) => {
            println!("FAIL Project workspace indexing: {}", error);
        }
    }

    match OllamaClient::new().check().await {
        Ok(true) => println!("PASS Ollama and qwen2.5-coder:7b available"),
        Ok(false) => println!("FAIL Ollama reachable, but qwen2.5-coder:7b not found"),
        Err(error) => println!("FAIL Ollama check: {}", error),
    }

    println!("==============================");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.first().map(String::as_str) == Some("run") {
        let goal = args.get(1..).unwrap_or(&[]).join(" ");

        if goal.trim().is_empty() {
            eprintln!("Usage: cargo run -- run \"<mission>\"");
            std::process::exit(1);
        }

        let mission = Mission::new(&goal);
        Orchestrator::new().execute(mission).await;
        return;
    }

    if args.first().map(String::as_str) == Some("inspect") {
        match ProjectIndexer::build(std::path::Path::new(".")) {
            Ok(index) => println!("{}", index.summary()),
            Err(error) => {
                eprintln!("Inspect failed: {}", error);
                std::process::exit(1);
            }
        }

        return;
    }

    if args.first().map(String::as_str) == Some("doctor") {
        run_doctor().await;
        return;
    }

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