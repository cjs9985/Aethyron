mod core;
mod agents;
mod tools;
mod memory;
mod models;

use core::orchestrator::{Mission, Orchestrator};
use tools::filesystem::FileSystem;
use agents::tool_agent::ToolAgent;


#[tokio::main]
async fn main() {

    println!("🌌 Aethyron Core Online");

    println!("📂 Workspace inspection:");

    let files = FileSystem::list(".")
        .unwrap();

    for file in files {
        println!(" - {}", file);
    }

    let mission = Mission::new(
        "Build a Rust authentication service"
    );

    let tool_agent = ToolAgent;

    tool_agent.inspect_project();


    let orchestrator = Orchestrator::new();

    orchestrator.execute(mission).await;
}
