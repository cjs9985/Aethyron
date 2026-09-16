mod agents;
mod core;
mod memory;
mod models;
mod tools;

use agents::tool_agent::ToolAgent;
use core::orchestrator::{Mission, Orchestrator};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("🌌 Aethyron Core Online");
    println!("Type an action order and press Enter. Type exit or quit to stop.");

    let orchestrator = Orchestrator::new();
    let tool_agent = ToolAgent;

    let startup_order = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if !startup_order.trim().is_empty() {
        run_order(&orchestrator, &tool_agent, startup_order.trim()).await;
        return;
    }

    loop {
        print!("Aethyron> ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("Failed to flush prompt: {}", error);
            break;
        }

        let mut order = String::new();
        match io::stdin().read_line(&mut order) {
            Ok(0) => {
                println!("Input closed. Aethyron shutting down.");
                break;
            }
            Ok(_) => {
                let order = order.trim();

                if order.is_empty() {
                    continue;
                }

                if matches!(order.to_ascii_lowercase().as_str(), "exit" | "quit") {
                    println!("Aethyron standing by.");
                    break;
                }

                run_order(&orchestrator, &tool_agent, order).await;
            }
            Err(error) => {
                eprintln!("Failed to read action order: {}", error);
                break;
            }
        }
    }
}

async fn run_order(orchestrator: &Orchestrator, tool_agent: &ToolAgent, order: &str) {
    println!("📂 Workspace inspection:");
    tool_agent.inspect_project();

    let mission = Mission::new(order);
    orchestrator.execute(mission).await;
    println!("✅ Ready for the next action order.");
}
