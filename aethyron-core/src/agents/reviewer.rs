use async_trait::async_trait;

use super::{Agent, Task};

use crate::models::{
    ollama::OllamaClient,
    tool_request::ToolRequest,
};


pub struct ReviewerAgent;


#[async_trait]
impl Agent for ReviewerAgent {

    fn name(&self) -> &str {
        "Reviewer Agent"
    }


    async fn execute(
        &self,
        task: &Task,
    ) -> Option<ToolRequest> {

        println!(
            "🔎 {} reviewing task:",
            self.name()
        );

        println!("{}", task.description);


        let client = OllamaClient::new();


        let prompt = format!(
r#"
You are a senior Rust code reviewer.

Review this task:

{}

Provide:
1. Potential problems
2. Security concerns
3. Testing recommendations

Be concise.
"#,
            task.description
        );


        match client.generate(&prompt).await {

            Ok(review) => {

                println!("\n📋 Review:\n");
                println!("{}", review);

                None
            }


            Err(error) => {

                println!(
                    "❌ Review failed: {}",
                    error
                );

                None
            }
        }
    }
}