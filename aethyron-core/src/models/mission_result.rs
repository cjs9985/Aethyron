use serde::Serialize;


#[derive(Serialize)]
pub struct MissionResult {
    pub mission_id: String,
    pub goal: String,
    pub success: bool,
    pub files_changed: Vec<String>,
    pub notes: String,
    
}
