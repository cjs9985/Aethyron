use async_trait::async_trait;

use super::{Agent, Task};

use crate::models::{
    code_generator::CodeGenerator,
    compiler::Compiler,
    file_operation::FileOperation,
    project_context::ProjectContext,
    tool_request::ToolRequest,
    fix_request::FixRequest,
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

        const MAX_RETRIES: usize = 3;
        let mut attempts = 0;


        loop {

            attempts += 1;


            let change = match CodeGenerator::generate(
                &task.description
            ).await {

                Ok(change) => change,

                Err(error) => {

                    println!(
                        "❌ Code generation failed: {}",
                        error
                    );

                    return;
                }
            };


            let previous_code = change.content.clone();


            println!("Generated code:\n");
            println!("{}", change.content);


            let operation = FileOperation {
                path: change.path,
                content: change.content,
            };


            match EditorTool::write(
                &operation.path,
                &operation.content,
            ) {

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

                    return;
                }
            }


            println!("⚙ Running cargo check...");


            match Compiler::check() {

                Ok(report) => {

                    println!("{}", report);

                    println!("✅ Build successful.");

                    break;
                }


                Err(error) => {

                    println!(
                        "❌ Cargo error: {}",
                        error
                    );


                    if attempts >= MAX_RETRIES {

                        println!(
                            "❌ Maximum repair attempts reached."
                        );

                        break;
                    }


                    let request = FixRequest {

                        compiler_output: error.to_string(),

                        previous_code: previous_code.clone(),
                    };


                    println!(
                        "🔄 Asking Ollama to repair the code..."
                    );


                    match CodeGenerator::fix(&request).await {

                        Ok(fix) => {

                            match EditorTool::write(
                                &fix.path,
                                &fix.content,
                            ) {

                                Ok(_) => {

                                    println!(
                                        "✅ Applied automatic fix."
                                    );
                                }


                                Err(e) => {

                                    println!(
                                        "❌ Failed to apply fix: {}",
                                        e
                                    );

                                    break;
                                }
                            }
                        }


                        Err(e) => {

                            println!(
                                "❌ Repair failed: {}",
                                e
                            );

                            break;
                        }
                    }
                }
            }
        }
    }
}