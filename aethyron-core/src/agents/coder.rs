use async_trait::async_trait;

use super::{Agent, Task};

use crate::models::{
    code_generator::CodeGenerator,
    compiler::Compiler,
    file_operation::FileOperation,
    project_context::ProjectContext,
    tool_request::ToolRequest,
    coder_result::CoderResult,
};

use crate::core::repair_engine::RepairEngine;
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

        println!(
            "💻 {} received task:",
            self.name()
        );

        println!("{}", task.description);

        Some(ToolRequest::InspectProject)
    }
}
impl CoderAgent {
     pub async fn execute_with_context(
    &self,
    task: &Task,
    context: &ProjectContext,
) -> CoderResult {

    let mut files_changed = Vec::new();

    println!(
        "💻 {} working on task:",
        self.name()
    );

    println!("{}", task.description);

    // Safety guard:
    // Aethyron modifies existing projects.
    // It must not recreate itself.
    if task
        .description
        .to_lowercase()
        .contains("initialize a new rust project")
    {
        println!("⚠️ Skipping unsafe initialization task.");
        return CoderResult {
            files_changed,
            generated_code: String::new(),
        };
    }

    println!("📦 Project Context");
    println!(
        "Cargo.toml: {} bytes",
        context.cargo_toml.len()
    );

    for file in &context.files {
        println!("  {}", file);
    }
     println!();
     println!("📚 Project Index:");
     println!("{}", context.project_index);
     println!();
     
    const MAX_RETRIES: usize = 3;
    let mut generated_code = String::new();
    let mut attempts = 0;

    loop {

        attempts += 1;

        println!(
            "🧠 Generating code (attempt {}/{})...",
            attempts,
            MAX_RETRIES
        );

        let change = match CodeGenerator::generate(
            &task.description,
            &context.project_index,
        )
        .await
        {
            Ok(change) => change,

            Err(error) => {

                println!(
                    "❌ Code generation failed: {}",
                    error
                );

                if attempts >= MAX_RETRIES {
                    break;
                }

                continue;
            }
        };
        if let Err(error) =
    self.validate_generated_path(
        &change.path,
        context,
    )
{
    println!(
        "❌ {}",
        error
    );

    if attempts >= MAX_RETRIES {
        break;
    }

    continue;
}
        
        generated_code = change.content.clone();

        let previous_code = generated_code.clone();

        let operation = FileOperation {
            path: change.path,
            content: generated_code.clone(),
        };

        println!("📝 Writing {}", operation.path);

       if std::path::Path::new(&operation.path).exists() {
    println!(
        "❌ Refusing to overwrite existing file: {}",
        operation.path
    );
    println!(
        "⚠️ Existing-file patching is required before this change can be applied."
    );
    break;
}

if let Err(error) = EditorTool::write(
    &operation.path,
    &operation.content,
) {
    println!("❌ Write error: {}", error);
    break;
}

        println!("✅ Generated {}", operation.path);

        files_changed.push(operation.path.clone());

        println!("⚙ Running cargo check...");

        match Compiler::check() {

            Ok(report) => {

                println!("✅ {}", report);

                break;
            }

            Err(error) => {

                println!(
                    "❌ Cargo error:\n{}",
                    error
                );

                if attempts >= MAX_RETRIES {

                    println!(
                        "❌ Maximum repair attempts reached."
                    );

                    break;
                }

                match RepairEngine::repair(
                    error.to_string(),
                    previous_code.clone(),
                )
                .await
                {

                    Ok(_) => {

                        println!(
                            "🔄 Repair completed. Retrying..."
                        );

                        continue;
                    }

                    Err(repair_error) => {

                        println!(
                            "❌ Repair failed: {}",
                            repair_error
                        );

                        break;
                    }
                }
            }
        }
    }

    CoderResult {
        files_changed,
        generated_code,
    }
}
fn validate_generated_path(
    &self,
    path: &str,
    context: &ProjectContext,
) -> Result<(), String> {

    // Never allow placeholders.
    if path == "existing/file.rs" {
        return Err("Placeholder path generated.".into());
    }

    if path.ends_with("example.rs") {
        return Err("Refusing to modify example.rs".into());
    }

    let allowed = path == "Cargo.toml"
    || path == "README.md"
    || path.starts_with("src/")
    || path.starts_with("tests/")
    || path.starts_with("examples/")
    || path.starts_with("benches/")
    || path.starts_with("workspace/");

    if !allowed {
        return Err(format!(
           "Invalid generated path: {}",
        path
    ));
}

    let exists = context.files.iter().any(|file| {
        file.replace("\\", "/").ends_with(path)
    });

    if !exists {

        return Err(format!(
            "Target file does not exist: {}",
            path
        ));
    }

    Ok(())
}
}