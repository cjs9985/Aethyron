use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct ConversationHistory {
    pub turns: Vec<ConversationTurn>,
}

impl ConversationHistory {
    pub fn new() -> Self {
        Self { turns: Vec::new() }
    }

    pub fn add(&mut self, role: impl Into<String>, content: impl Into<String>) {
        self.turns.push(ConversationTurn {
            role: role.into(),
            content: content.into(),
        });
    }

    pub fn is_empty(&self) -> bool {
        self.turns.is_empty()
    }

    pub fn format_for_prompt(&self) -> String {
        if self.turns.is_empty() {
            return String::new();
        }

        self.turns
            .iter()
            .map(|turn| format!("[{}]: {}", turn.role.to_uppercase(), turn.content))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
