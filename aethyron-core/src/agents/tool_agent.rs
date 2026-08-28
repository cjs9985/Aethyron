use crate::tools::filesystem::FileSystem;
use crate::tools::dispatcher::ToolDispatcher;
use crate::models::tool_request::ToolRequest;

pub struct ToolAgent;

impl ToolAgent {
    pub fn inspect_project(&self) -> String {
        println!("🔍 Tool Agent inspecting project...");

        match FileSystem::inspect_project() {
            Ok(files) => {
                for file in &files {
                    println!("📄 {}", file);
                }
                files.join("\n")
            }
            Err(error) => {
                println!("❌ File inspection failed: {}", error);
                error.to_string()
            }
        }
    }

    pub fn cargo_check(&self) -> bool {
        let result = ToolDispatcher::execute(ToolRequest::CargoCheck);
        println!(
            "⚙️ Cargo check: {}",
            if result.success { "passed" } else { "failed" }
        );
        if !result.output.is_empty() {
            println!("{}", result.output);
        }
        result.success
    }
}
