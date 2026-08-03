use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CoderResult {
    pub code_changes: Vec<CodeChange>,
    pub compilation_status: CompilationStatus,
}

#[derive(Serialize, Deserialize)]
pub enum CompilationStatus {
    Success,
    Failure(String),
}