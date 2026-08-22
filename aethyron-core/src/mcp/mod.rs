use rmcp::{ServerHandler, handler::server::wrapper::Parameters, schemars, tool, tool_router};

use crate::{models::tool_request::ToolRequest, tools::dispatcher::ToolDispatcher};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ReadFileRequest {
    pub path: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct WriteFileRequest {
    pub path: String,
    pub content: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct AppendFileRequest {
    pub path: String,
    pub content: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GitCommitRequest {
    pub message: String,
}

#[derive(Clone)]
pub struct AethyronMcp {
    tool_router: rmcp::handler::server::router::tool::ToolRouter<Self>,
}

#[tool_router]
impl AethyronMcp {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Inspect the current Aethyron project")]
    async fn inspect_project(&self) -> String {
        match ToolDispatcher::execute(ToolRequest::InspectProject) {
            Ok(()) => "Project inspection completed.".to_string(),
            Err(error) => format!("Project inspection failed: {error}"),
        }
    }

    #[tool(description = "Read a file from the current project")]
    async fn read_file(
        &self,
        Parameters(ReadFileRequest { path }): Parameters<ReadFileRequest>,
    ) -> String {
        match ToolDispatcher::execute(ToolRequest::ReadFile(path)) {
            Ok(()) => "File read successfully.".to_string(),
            Err(error) => format!("File read failed: {error}"),
        }
    }

    #[tool(description = "Write content to a project file")]
    async fn write_file(
        &self,
        Parameters(WriteFileRequest { path, content }): Parameters<WriteFileRequest>,
    ) -> String {
        match ToolDispatcher::execute(ToolRequest::WriteFile { path, content }) {
            Ok(()) => "File written successfully.".to_string(),
            Err(error) => format!("File write failed: {error}"),
        }
    }

    #[tool(description = "Append content to a project file")]
    async fn append_file(
        &self,
        Parameters(AppendFileRequest { path, content }): Parameters<AppendFileRequest>,
    ) -> String {
        match ToolDispatcher::execute(ToolRequest::AppendFile { path, content }) {
            Ok(()) => "File appended successfully.".to_string(),
            Err(error) => format!("File append failed: {error}"),
        }
    }

    #[tool(description = "Run cargo check on the project")]
    async fn cargo_check(&self) -> String {
        match ToolDispatcher::execute(ToolRequest::CargoCheck) {
            Ok(()) => "Cargo check completed successfully.".to_string(),
            Err(error) => format!("Cargo check failed: {error}"),
        }
    }

    #[tool(description = "Run cargo fmt on the project")]
    async fn cargo_fmt(&self) -> String {
        match ToolDispatcher::execute(ToolRequest::CargoFmt) {
            Ok(()) => "Cargo format completed.".to_string(),
            Err(error) => format!("Cargo format failed: {error}"),
        }
    }

    #[tool(description = "Return the current Git status")]
    async fn git_status(&self) -> String {
        match ToolDispatcher::execute(ToolRequest::GitStatus) {
            Ok(()) => "Git status completed.".to_string(),
            Err(error) => format!("Git status failed: {error}"),
        }
    }

    #[tool(description = "Commit current project changes")]
    async fn git_commit(
        &self,
        Parameters(GitCommitRequest { message }): Parameters<GitCommitRequest>,
    ) -> String {
        match ToolDispatcher::execute(ToolRequest::GitCommit(message)) {
            Ok(()) => "Git commit completed.".to_string(),
            Err(error) => format!("Git commit failed: {error}"),
        }
    }
}

#[rmcp::tool_handler]
impl ServerHandler for AethyronMcp {}
