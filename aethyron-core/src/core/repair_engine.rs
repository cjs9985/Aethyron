use anyhow::Result;

use crate::models::{
    code_change::CodeChange,
    code_generator::CodeGenerator,
    fix_request::FixRequest,
    tool_request::ToolRequest,
};

use crate::tools::{dispatcher::ToolDispatcher, editor::EditorTool};

pub struct RepairEngine;

impl RepairEngine {
    pub async fn repair(
        compiler_output: String,
        previous_code: String,
    ) -> Result<CodeChange> {
        println!("🔧 Repair engine activated...");

        println!("📋 Compiler errors:");
        println!("{}", compiler_output);

        let request = FixRequest {
            compiler_output,
            previous_code,
        };

        let fix = CodeGenerator::fix(&request).await?;

        println!("📝 Applying repair to {}", fix.path);

        EditorTool::write(&fix.path, &fix.content)?;

        println!("⚙️ Verifying repair...");

        match ToolDispatcher::execute(ToolRequest::CargoCheck) {
            Ok(_) => {
                println!("✅ Repair compiled successfully.");
            }

            Err(error) => {
                println!("❌ Repair still has compilation errors.");
                return Err(error);
            }
        }

        Ok(fix)
    }
}