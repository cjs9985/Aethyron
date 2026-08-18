use serde::Serialize;

#[derive(Serialize)]
pub struct MissionResult {
    pub mission_id: String,
    pub goal: String,
    pub success: bool,
    pub files_changed: Vec<String>,
    pub tasks_completed: usize,
    pub repairs: usize,
    pub duration_ms: u128,
    pub notes: String,
}
