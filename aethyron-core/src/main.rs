mod agents;
mod core;
mod memory;
mod models;
mod tools;

use core::orchestrator::{Mission, Orchestrator};

#[tokio::main]
async fn main() {
    let mission = Mission::new("Build a Rust authentication service");

    let orchestrator = Orchestrator::new();

    orchestrator.execute(mission).await;
}