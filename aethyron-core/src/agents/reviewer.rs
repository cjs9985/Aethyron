use async_trait::async_trait;

use super::{Agent, Task};

use crate::models::{
    ollama::OllamaClient,
    tool_request::ToolRequest,
    review_report::ReviewReport,
    project_context::ProjectContext,
};

pub struct ReviewerAgent;

impl ReviewerAgent {

    pub async fn review(
        &self,
        task: &Task,
        context: &ProjectContext,
    ) -> ReviewReport {

        let client = OllamaClient::new();
        println!("Project files: {}",
                  context.files.len());
        let prompt = format!(
r#"
You are a senior Rust code reviewer.

Review the work completed for this engineering task.

Task:
{}

Return a concise review.
"#,
            task.description
        );

        match client.generate(&prompt).await {

            Ok(response) => ReviewReport {
                passed: true,
                notes: response,
            },

            Err(error) => ReviewReport {
                passed: false,
                notes: format!(
                    "Review failed: {}",
                    error
                ),
            },
        }
    }
}

#[async_trait]
impl Agent for ReviewerAgent {

    fn name(&self) -> &str {
        "Reviewer Agent"
    }

    async fn execute(
        &self,
        task: &Task,) -> Option<ToolRequest> {

        println!("🔍 Review:");
        println!("{}", task.description);
        None
    }
}