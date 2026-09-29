use a2a::{A2AError, Artifact, Message, Part, Role, StreamResponse, Task, TaskState, TaskStatus};
use a2a_server::executor::{AgentExecutor, ExecutorContext};

use futures::stream::{self, BoxStream};
use serde_json::Value;

use crate::core::context_builder::ContextBuilder;
use crate::core::orchestrator::{Mission, Orchestrator};
use crate::memory::store::MemoryStore;
use crate::models::conversation::{ConversationHistory, ConversationTurn};

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

        // Load conversation history so the mission has prior context.
        let prior_turns = MemoryStore::load_conversation(20).unwrap_or_default();
        let mut history = ConversationHistory::new();
        for turn in prior_turns {
            history.add(turn.role, turn.content);
        }
        if !goal.is_empty() {
            history.add("user", &goal);
            let _ = MemoryStore::save_turn(&ConversationTurn {
                role: "user".to_string(),
                content: goal.clone(),
            });
        }

        // Build project context with conversation so the planner has full info.
        let context = ContextBuilder::build_with_conversation(".", history.format_for_prompt())
            .unwrap_or_else(|_| crate::models::project_context::ProjectContext {
                cargo_toml: String::new(),
                files: Vec::new(),
                memory: String::new(),
                project_index: String::new(),
                conversation: history.format_for_prompt(),
            });

        let mission = Mission::new_with_context(&goal, context);

        Box::pin(stream::once(async move {
            let result = Orchestrator::new().execute(mission).await;

            let summary = format!(
                "Mission complete. Tasks: {} | Files changed: {} | Repairs: {} | Success: {}",
                result.tasks_completed,
                result.files_changed.len(),
                result.repairs,
                result.success,
            );

            // Persist the assistant turn so follow-up missions have context.
            let _ = MemoryStore::save_turn(&ConversationTurn {
                role: "aethyron".to_string(),
                content: summary.clone(),
            });

            let state = if result.success {
                TaskState::Completed
            } else {
                TaskState::Failed
            };

            // Attach the mission summary as a text artifact so the calling
            // agent can read the outcome.
            let artifact = Artifact {
                artifact_id: uuid::Uuid::new_v4().to_string(),
                name: Some("mission-result".to_string()),
                description: Some("Aethyron mission execution summary".to_string()),
                parts: vec![Part::text(summary.clone())],
                metadata: None,
                extensions: None,
            };

            // Build a reply Message for TaskStatus so callers see the outcome.
            let reply_message = Message {
                message_id: uuid::Uuid::new_v4().to_string(),
                context_id: None,
                task_id: None,
                role: Role::Agent,
                parts: vec![Part::text(summary.clone())],
                metadata: None,
                extensions: None,
                reference_task_ids: None,
            };

            let task = Task {
                id: task_id,
                context_id,
                status: TaskStatus {
                    state,
                    message: Some(reply_message),
                    timestamp: None,
                },
                artifacts: Some(vec![artifact]),
                history: None,
                metadata: None,
            };

            Ok(StreamResponse::Task(task))
        }))
    }

    fn cancel(&self, ctx: ExecutorContext) -> BoxStream<'static, Result<StreamResponse, A2AError>> {
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

        Box::pin(stream::once(async move { Ok(StreamResponse::Task(task)) }))
    }
}
