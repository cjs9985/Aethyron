use anyhow::Result;

use crate::models::{
    code_generator::CodeGenerator,
    fix_request::FixRequest,
};

use crate::tools::editor::EditorTool;

pub struct RepairEngine;

impl RepairEngine {

    pub async fn repair(
        compiler_output: String,
        previous_code: String,
    ) -> Result<()> {

        println!("🔧 Repair engine activated...");

        println!("📋 Compiler errors:");
        println!("{}", compiler_output);

        let request = FixRequest {
            compiler_output,
            previous_code,
        };

        let fix = CodeGenerator::fix(&request).await?;

        println!("📝 Applying repair to {}", fix.path);

        EditorTool::write(
            &fix.path,
            &fix.content,
        )?;

        println!("✅ Repair applied.");

        Ok(())
    }
}