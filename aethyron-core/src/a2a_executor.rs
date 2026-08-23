use a2a::{A2AError,StreamResponse, Task, TaskState, TaskStatus};
use a2a_server::executor::{AgentExecutor, ExecutorContext};
    
use futures::stream::{self, BoxStream};
use serde_json::Value;

use crate::core::orchestrator::{Mission, Orchestrator};

pub struct AethyronA2AExecutor;

impl AethyronA2AExecutor {
    pub fn new() -> Self {
        Self
    }

    fn extract_text(message: &a2a::Message) -> String {
        let value = match serde_json::to_value(message) {
            Ok(value) => value,
            Err(_) => return String::new(),
        };

        fn find_text(value: &Value) -> Option<String> {
            match value {
                Value::Object(map) => {
                    if let Some(Value::String(text)) = map.get("text") {
                        return Some(text.clone());
                    }

                    for child in map.values() {
                        if let Some(text) = find_text(child) {
                            return Some(text);
                        }
                    }

                    None
                }

                Value::Array(values) => {
                    for value in values {
                        if let Some(text) = find_text(value) {
                            return Some(text);
                        }
                    }

                    None
                }

                _ => None,
            }
        }

        find_text(&value).unwrap_or_default()
    }
}

impl Default for AethyronA2AExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentExecutor for AethyronA2AExecutor {
    fn execute(
        &self,
        ctx: ExecutorContext,
    ) -> BoxStream<'static, Result<StreamResponse, A2AError>> {
        let task_id = ctx.task_id;
        let context_id = ctx.context_id;

        let goal = match ctx.message.as_ref() {
            Some(message) => Self::extract_text(message),
            None => String::new(),
        };

        let mission = Mission::new(&goal);

        Box::pin(stream::once(async move {
            let result = Orchestrator::new().execute(mission).await;

            let state = if result.success {
                TaskState::Completed
            } else {
                TaskState::Failed
            };

            let task = Task {
                id: task_id,
                context_id,
                status: TaskStatus {
                    state,
                    message: None,
                    timestamp: None,
                },
                artifacts: None,
                history: None,
                metadata: None,
            };

            Ok(StreamResponse::Task(task))
        }))
    }

    fn cancel(
        &self,
        ctx: ExecutorContext,
    ) -> BoxStream<'static, Result<StreamResponse, A2AError>> {
        let task = Task {
            id: ctx.task_id,
            context_id: ctx.context_id,
            status: TaskStatus {
                state: TaskState::Canceled,
                message: None,
                timestamp: None,
            },
            artifacts: None,
            history: None,
            metadata: None,
        };

        Box::pin(stream::once(async move {
            Ok(StreamResponse::Task(task))
        }))
    }
}