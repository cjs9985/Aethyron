use async_trait::async_trait;

use super::{Agent, Task};

use crate::tools::{
    terminal::TerminalTool,
    editor::EditorTool,
};
use crate::models::{project_context::ProjectContext,
 tool_request::ToolRequest,
 code_generator::CodeGenerator,
 file_operation::FileOperation,
};

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
   pub async fn execute_with_context(
    &self,
    task: &Task,
    context: &ProjectContext,
) {

    println!("💻 {} working on task:", self.name());
    println!("{}", task.description);

    println!("📦 Project Context");
    println!("Cargo.toml: {} bytes", context.cargo_toml.len());

    for file in &context.files {
        println!("  {}", file);
    }


    println!("🧠 Generating code...");

    match CodeGenerator::generate(&task.description).await {

        Ok(change) => {

            println!("Generated code:");

            println!("{}", change.content);

        let operation = FileOperation{
                path: change.path,
                content: change.content,
            };

            match EditorTool::write(&operation.path,
             &operation.content,) {

                Ok(_) => {

                    println!(
                        "✅ Generated {}",
                        operation.path
                    );

                }

                Err(error) => {

                    println!(
                        "❌ Write error: {}",
                        error
                    );

                }
            }

            println!("⚙ Running cargo check...");

            match TerminalTool::run(
                "cargo",
                &["check"],
            ) {

                Ok(result) => {

                    println!("{}", result);

                }

                Err(error) => {

                    println!(
                        "❌ Cargo error: {}",
                        error
                    );

                }
            }
        }


        Err(error) => {

            println!(
                "❌ Code generation failed: {}",
                error
            );

        }
    }
}
}