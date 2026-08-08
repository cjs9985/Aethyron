use async_trait::async_trait;
use serde_json;

use super::{Agent, Task};

use crate::models::{
    ollama::OllamaClient,
    plan::Plan,
    project_context::ProjectContext,
    tool_request::ToolRequest,
};

pub struct PlannerAgent;

impl PlannerAgent {
    pub async fn create_plan(
        &self,
        task: &Task,
    ) -> Option<Plan> {

        self.create_plan_with_context(
            task,
            None,
        )
        .await
    }
    fn validate_plan(
    &self,
    plan: &mut Plan,
    project_index: &str,
) {
    plan.tasks.retain(|task| {

        let words = task.split_whitespace();

        for word in words {

            if word.starts_with("src/")
                || word.starts_with("src\\")
                || word.starts_with("tests/")
                || word.starts_with("tests\\")
            {
                let path = word
                    .trim_matches('`')
                    .trim_matches(',')
                    .trim_matches('.');

            let normalized_path = path.replace('\\', "/");

            let exists = project_index.lines().any(|line| {
    line.trim().replace('\\', "/") == normalized_path
});

if !exists {
    println!(
        "⚠ Removing invalid planner task:\n{}",
        task
    );

    return false;
}                
 }
        }
        let lower = task.to_lowercase();

        if lower.contains("create a new project")
    || lower.contains("initialize cargo")
    || lower.contains("cargo new")
{
    println!(
        "⚠ Planner proposed creating a new project. Rejecting task."
    );

    return false;
    
}
        true
    });
}

    pub async fn create_plan_with_context(
        &self,
        task: &Task,
        context: Option<&ProjectContext>,
    ) -> Option<Plan> {

        let client = OllamaClient::new();
        let memory = context
           .map(|ctx| ctx.memory.as_str())
           .unwrap_or("");

        let project_index = context
           .map(|ctx| ctx.project_index.as_str())
           .unwrap_or("");
        

        let prompt = format!(
r#"You are the planning intelligence of Aethyron.

Convert the mission into executable engineering tasks.

Important:
- This is an existing Rust project.
- Never recreate the project.
- Never initialize Cargo again.
- Modify existing files only.
- Inspect the supplied project structure before selecting any file.
- Prefer existing files over creating new files.
- A file may only be created when the mission explicitly requires a new file and no appropriate existing file can perform the required role.
Output format:
Return ONLY valid JSON matching this structure:

{{
  "tasks": [
    "Concrete Rust modification task"
  ]
        }}

No markdown.
No explanations.
No tutorials.
No code blocks.
Mission:

{}

Previous project memory:

{}

Existing project structure:

{}

Analyze the existing project before creating tasks.

Rules:

- This project is Rust only.
- NEVER generate Python, JavaScript, Java, or other language instructions.
- NEVER provide tutorials or explanations.
- ONLY output executable Rust engineering tasks.
- Reuse existing Rust files whenever possible.
- Do not initialize Cargo.
- Do not create a new Cargo project.
- Do not create duplicate modules.
- Reference ONLY paths that exist in the Existing project structure.
- Select files according to their existing responsibility, not merely because the path exists.
- Authentication, password hashing, password verification, registration authentication, and login authentication MUST use the existing authentication module src/core/auth.rs.
- NEVER assign authentication or password-related modifications to src/tools/editor.rs.
- src/tools/editor.rs is an editing/file-operation utility and must not contain authentication logic.
- Every task must describe a concrete modification to the existing project.
- Every task must include a valid Rust project file path.

Architecture rules:

- Preserve the existing project architecture.
- Do not move functionality between modules unless the mission explicitly requires it.
- Do not use utility modules as substitutes for domain modules.
- src/tools/editor.rs is the file-editing utility. NEVER place authentication, password hashing, registration, login, or user-management logic in src/tools/editor.rs.
- Authentication and password hashing logic belongs in the existing authentication module src/core/auth.rs when that file exists.
- If src/core/auth.rs exists, reuse it rather than creating another authentication module.
- Tests must use the existing tests/ directory and existing test structure.
- Do not invent test directories or filenames.
- Do not create a new file when an appropriate existing file is already present.
- Cargo.toml may be modified when a dependency is required.
- Do not replace the contents of an existing file merely to add functionality.
- Tasks modifying existing files must request a minimal modification, preserving all existing functionality.
- Authentication architecture is fixed: all password hashing, password verification, registration authentication logic, and login authentication logic MUST be implemented through src/core/auth.rs.
- NEVER assign authentication, registration, login, password hashing, or password verification modifications to src/tools/editor.rs.
- src/tools/editor.rs is strictly an editing/file-operation utility and must not contain user authentication logic.
- If a mission requires authentication changes, create tasks targeting src/core/auth.rs or another existing authentication-specific source file identified in the project structure.

- For an authentication mission, the plan MUST include tasks for:
  1. Adding the required password-hashing dependency if it is missing.
  2. Implementing password hashing and password verification in the existing authentication module src/core/auth.rs.
  3. Integrating the authentication functions into the existing authentication flow, using the appropriate existing authentication-related source file identified in the project structure.
  4. Adding or updating tests using an existing test location when one exists.
- NEVER assign authentication logic to src/tools/editor.rs.
- Do not treat adding a dependency as completing an authentication mission.

Return ONLY valid JSON in this format:
  {{
     "tasks": [
        "task one",
        "task two",
        "task three"
     ]
  }}
     No markdown.
     No explanations.
     No extra text.
     "#,
            task.description,
            memory,
            project_index
        );

        let response = match client.generate(&prompt).await {
            Ok(response) => response,

            Err(error) => {
                println!("❌ Planner model error: {}", error);
                return None;
            }
        };

        println!("\n📜 Planner Response:\n");
        println!("{}", response);
        
        let _clean_response = response
            .replace("\\", "\\\\");
        match serde_json::from_str::<Plan>(&response) {

            Ok(mut plan) => {
                self.validate_plan(
                &mut plan,
                project_index,
         );
                println!(
                    "✅ Plan created with {} tasks.",
                    plan.tasks.len()
                );

                Some(plan)
            }

            Err(error) => {

                println!(
                    "❌ Failed to parse planner response: {}",
                    error
                );

                None
            }
        }
    }
}

#[async_trait]
impl Agent for PlannerAgent {

    fn name(&self) -> &str {
        "Planner Agent"
    }

    async fn execute(
        &self,
        _task: &Task,
    ) -> Option<ToolRequest> {

        println!(
            "🧭 {} analyzing mission...",
            self.name()
        );

        let tool_request = ToolRequest::InspectProject;

        println!(
            "🧭 Planner requested tool: {:?}",
            tool_request
        );

        Some(tool_request)
    }
    

}