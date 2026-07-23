pub mod planner;

use async_trait::async_trait;

#[derive(Debug)]
pub struct Task {
    pub description: String,
}

#[async_trait]
pub trait Agent: Send + Sync {

    fn name(&self) -> &str;


    async fn execute(
        &self,
        task: Task
    );

}