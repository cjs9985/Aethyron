use async_trait::async_trait;

use super::{Agent, Task};

use crate::core::repair_engine::RepairEngine;

use crate::models::{
    code_generator::CodeGenerator,
    compiler::Compiler,
    file_operation::FileOperation,
    project_context::ProjectContext,
    tool_request::ToolRequest,
};

use crate::tools::editor::EditorTool;

pub struct CoderAgent;

#[async_trait]
impl Agent for CoderAgent {
    fn name(&self) -> &str {
        "Coder Agent"
    }

    async fn execute(
        &self,
        task: &Task,
    ) -> Option<ToolRequest> {

        println!("💻 {} received task:", self.name());
        println!("{}", task.description);

        None
    }
}

impl CoderAgent {

    const MAX_RETRIES: usize = 3;

    pub async fn execute_with_context(
        &self,
        task: &Task,
        context: &ProjectContext,
    ) -> Vec<String> {

        let mut files_changed = Vec::new();

        println!("💻 {} working on task:", self.name());
        println!("{}", task.description);

        println!("📦 Project Context");
        println!("Cargo.toml: {} bytes", context.cargo_toml.len());

        for file in &context.files {
            println!("  {}", file);
        }

        let mut attempts = 0;

        loop {

            attempts += 1;

            println!(
                "🧠 Generating code (attempt {}/{})...",
                attempts,
                Self::MAX_RETRIES
            );

            let change = match CodeGenerator::generate(
                &task.description,
            )
            .await
            {
                Ok(change) => change,

                Err(error) => {

                    println!(
                        "❌ Code generation failed: {}",
                        error
                    );

                    if attempts >= Self::MAX_RETRIES {
                        return files_changed;
                    }

                    continue;
                }
            };

            let previous_code = change.content.clone();

            let operation = FileOperation {
                path: change.path,
                content: change.content,
            };

            println!("📝 Writing {}", operation.path);

            if let Err(error) = EditorTool::write(
                &operation.path,
                &operation.content,
            ) {

                println!("❌ Write error: {}", error);
                return files_changed;
            }

            files_changed.push(operation.path.clone());

            println!("⚙ Running cargo check...");

            match Compiler::check() {

                Ok(report) => {

                    println!("{}", report);

                    println!(
                        "✅ Task completed successfully."
                    );

                    return files_changed;
                }

                Err(error) => {

                    println!(
                        "❌ Cargo check failed:\n{}",
                        error
                    );

                    if attempts >= Self::MAX_RETRIES {

                        println!(
                            "❌ Maximum repair attempts reached."
                        );

                        return files_changed;
                    }

                    println!("🔧 Attempting automatic repair...");

                    match RepairEngine::repair(
                        error.to_string(),
                        previous_code,
                    )
                    .await
                    {

                        Ok(_) => {

                            println!(
                                "🔄 Repair complete. Retrying..."
                            );

                            continue;
                        }

                        Err(error) => {

                            println!(
                                "❌ Repair failed: {}",
                                error
                            );

                            return files_changed;
                        }
                    }
                }
            }
        }
    }
}