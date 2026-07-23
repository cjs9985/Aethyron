use async_trait::async_trait;

use super::{Agent, Task};

use crate::models::ollama::OllamaClient;


pub struct PlannerAgent;


#[async_trait]
impl Agent for PlannerAgent {


    fn name(&self) -> &str {
        "Planner Agent"
    }


    async fn execute(
        &self,
        task: Task
    ) {


        println!(
            "🧭 {} analyzing mission...",
            self.name()
        );


        let client =
            OllamaClient::new();


        let prompt =
format!(
"
You are the planning intelligence of Aethyron.

Create a detailed engineering plan.

Mission:
{}

Return:
1. Architecture
2. Implementation steps
3. Required files
4. Testing strategy
",
task.description
);


        match client.generate(&prompt).await {

            Ok(plan) => {

                println!("\n📜 Generated Plan:\n");
                println!("{}", plan);

            }


            Err(error) => {

                println!(
                    "Model error: {}",
                    error
                );

            }
        }
    }
}