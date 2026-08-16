#[derive(Debug, Clone)]
pub enum ToolRequest {
    InspectProject,
    ReadFile(String),
    WriteFile { path: String, content: String },
    AppendFile { path: String, content: String },
    CreateDirectory(String),
    CargoCheck,
    CargoFmt,
    GitStatus,
    GitAdd,
    GitCommit(String),
}
