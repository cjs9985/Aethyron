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

    pub async fn create_plan_with_context(
        &self,
        task: &Task,
        context: Option<&ProjectContext>,
    ) -> Option<Plan> {

        let client = OllamaClient::new();

        let memory = context
            .map(|ctx| ctx.memory.as_str())
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
Mission:

{}

Previous project memory:

{}

Return ONLY valid JSON in this format:

{{
  "tasks": [
    "task one",
    "task two",
    "task three"
  ]
}}

Rules:

- No markdown.
- No explanations.
- No extra text.
"#,
            task.description,
            memory
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

        match serde_json::from_str::<Plan>(&response) {

            Ok(plan) => {

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