use std::fs::{OpenOptions, read_to_string};
use std::io::Write;

use anyhow::Result;
use serde_json;

use crate::models::conversation::ConversationTurn;
use crate::models::mission_result::MissionResult;

pub struct MemoryStore;

impl MemoryStore {
    const FILE: &'static str = "aethyron_memory.json";
    const CONVERSATION_FILE: &'static str = "aethyron_conversation.json";

    pub fn save(content: &str) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Self::FILE)?;

        writeln!(file, "{}", content)?;

        Ok(())
    }

    pub fn save_result(result: &MissionResult) -> Result<()> {
        let json = serde_json::to_string(result)?;

        Self::save(&json)
    }

    pub fn load() -> Result<String> {
        match read_to_string(Self::FILE) {
            Ok(memory) => Ok(memory),

            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    Ok(String::new())
                } else {
                    Err(error.into())
                }
            }
        }
    }

    pub fn load_recent(limit: usize) -> Result<String> {
        let memory = Self::load()?;

        if memory.is_empty() {
            return Ok(String::new());
        }

        let lines: Vec<&str> = memory.lines().collect();

        let recent = lines
            .iter()
            .rev()
            .take(limit)
            .rev()
            .cloned()
            .collect::<Vec<&str>>();

        Ok(recent.join("\n"))
    }

    pub fn save_turn(turn: &ConversationTurn) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Self::CONVERSATION_FILE)?;

        writeln!(file, "{}", serde_json::to_string(turn)?)?;

        Ok(())
    }

    pub fn load_conversation(limit: usize) -> Result<Vec<ConversationTurn>> {
        let content = match read_to_string(Self::CONVERSATION_FILE) {
            Ok(c) => c,
            Err(error) => {
                if error.kind() == std::io::ErrorKind::NotFound {
                    return Ok(Vec::new());
                } else {
                    return Err(error.into());
                }
            }
        };

        let turns: Vec<ConversationTurn> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        let recent = if turns.len() > limit {
            turns[turns.len() - limit..].to_vec()
        } else {
            turns
        };

        Ok(recent)
    }
}
