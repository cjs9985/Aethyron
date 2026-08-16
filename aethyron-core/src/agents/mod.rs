pub mod coder;
pub mod planner;
pub mod reviewer;
pub mod tool_agent;

use crate::models::tool_request::ToolRequest;
use async_trait::async_trait;

#[derive(Debug)]
pub struct Task {
    pub description: String,
}

#[async_trait]
pub trait Agent {
    fn name(&self) -> &str;

    async fn execute(&self, task: &Task) -> Option<ToolRequest>;
}
