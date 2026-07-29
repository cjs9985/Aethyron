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

                let exists =
                    project_index
                        .lines()
                        .any(|line| line.trim() == path);

                if !exists  {

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
- Inspect current structure before creating files.
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
- Reuse existing Rust files when possible.
- Do not initialize Cargo.
- Do not create duplicate modules.
- Reference existing Rust files in tasks.
- Tasks must describe concrete code modifications.
- Every task must include a Rust file path.

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