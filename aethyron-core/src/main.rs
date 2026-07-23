mod core;
mod agents;
mod tools;
mod memory;
mod models;

use core::orchestrator::{Mission, Orchestrator};


#[tokio::main]
async fn main() {

    println!("🌌 Aethyron Core Online");


    let mission = Mission::new(
        "Build a Rust authentication service"
    );


    let orchestrator = Orchestrator::new();


    orchestrator.execute(mission).await;

}
