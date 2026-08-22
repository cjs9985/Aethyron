mod agents;
mod core;
mod mcp;
mod memory;
mod models;
mod tools;

use crate::core::orchestrator::{Mission, Orchestrator};
use crate::core::project_indexer::ProjectIndexer;
use crate::mcp::AethyronMcp;
use crate::models::ollama::OllamaClient;
use axum::{Router, routing::get};
use rmcp::ServiceExt;
use std::env;
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

async fn run_doctor() -> bool {
    println!("==============================");
    println!("AETHYRON DOCTOR");
    println!("==============================");

    let mut healthy = true;

    let cargo_ok = std::path::Path::new("Cargo.toml").is_file();
    println!(
        "{} Cargo.toml exists",
        if cargo_ok { "PASS" } else { "FAIL" }
    );

    if !cargo_ok {
        healthy = false;
    }

    let src_ok = std::path::Path::new("src").is_dir();
    println!(
        "{} src directory exists",
        if src_ok { "PASS" } else { "FAIL" }
    );

    if !src_ok {
        healthy = false;
    }

    match ProjectIndexer::build(std::path::Path::new(".")) {
        Ok(index) => {
            println!("PASS Project workspace can be indexed");
            println!("     Files indexed: {}", index.files.len());
        }

        Err(error) => {
            println!("FAIL Project workspace indexing: {}", error);
            healthy = false;
        }
    }

    match OllamaClient::new().check().await {
        Ok(true) => println!("PASS Ollama and qwen2.5-coder:7b available"),

        Ok(false) => {
            println!("FAIL Ollama reachable, but qwen2.5-coder:7b not found");
            healthy = false;
        }

        Err(error) => {
            println!("FAIL Ollama check: {}", error);
            healthy = false;
        }
    }

    println!("==============================");

    healthy
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.first().map(String::as_str) == Some("mcp") {
        let server = AethyronMcp::new();

        server
            .serve(rmcp::transport::stdio())
            .await
            .expect("Aethyron MCP server failed");

        return;
    }

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
        let healthy = run_doctor().await;

        if !healthy {
            std::process::exit(1);
        }

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
