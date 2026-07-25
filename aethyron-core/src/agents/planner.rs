use async_trait::async_trait;
use serde_json;

use super::{Agent, Task};

use crate::models::{
    ollama::OllamaClient,
    tool_request::ToolRequest,
    plan::Plan,
    project_context::ProjectContext,
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
        ).await
    }


    pub async fn create_plan_with_context(
        &self,
        task: &Task,
        context: Option<&ProjectContext>,
    ) -> Option<Plan> {

        let client = OllamaClient::new();


        let memory = match context {
            Some(ctx) => &ctx.memory,
            None => "",
        };


        let prompt = format!(
r#"
You are the planning intelligence of Aethyron.

Convert the mission into executable engineering tasks.

Mission:

{}

Previous project memory:

{}

Return ONLY valid JSON.

Format:

{{
  "tasks": [
    "task one",
    "task two",
    "task three"
  ]
}}

Do not include markdown.
Do not include explanations.
"#,
            task.description,
            memory
        );


        match client.generate(&prompt).await {

            Ok(response) => {

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
                            "❌ Failed to parse plan JSON: {}",
                            error
                        );

                        None
                    }
                }
            }


            Err(error) => {

                println!(
                    "❌ Planner model error: {}",
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
        task: &Task,
    ) -> Option<ToolRequest> {


        println!(
            "🧭 {} analyzing mission...",
            self.name()
        );


        let tool_request =
            ToolRequest::InspectProject;


        println!(
            "🧭 Planner requested tool: {:?}",
            tool_request
        );


        let _ = self.create_plan(task).await;


        Some(tool_request)
    }
}