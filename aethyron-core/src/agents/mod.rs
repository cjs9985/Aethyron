pub mod planner;
pub mod tool_agent;
pub mod coder;
pub mod reviewer;

use async_trait::async_trait;
use crate::models::tool_request::ToolRequest;

#[derive(Debug)]
pub struct Task {
    pub description: String,
}

#[async_trait]
pub trait Agent {
    fn name(&self) -> &str;

    async fn execute(
        &self,
        task: &Task
    ) -> Option<ToolRequest>;
}