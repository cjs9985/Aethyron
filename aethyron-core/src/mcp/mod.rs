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
pub struct CreateDirectoryRequest {
    pub path: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GitCommitRequest {
    pub message: String,
}

#[derive(Clone)]
pub struct AethyronMcp {
    // Populated by #[tool_router] and read by the macro-generated ServerHandler impl.
    // Rust's dead-code lint cannot see inside proc macro expansions.
    #[allow(dead_code)]
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
        let result = ToolDispatcher::execute(ToolRequest::InspectProject);
        if result.success {
            result.output
        } else {
            format!("Project inspection failed: {}", result.output)
        }
    }

    #[tool(description = "Read a file from the current project")]
    async fn read_file(
        &self,
        Parameters(ReadFileRequest { path }): Parameters<ReadFileRequest>,
    ) -> String {
        let result = ToolDispatcher::execute(ToolRequest::ReadFile(path));
        if result.success {
            result.output
        } else {
            format!("File read failed: {}", result.output)
        }
    }

    #[tool(description = "Write content to a project file")]
    async fn write_file(
        &self,
        Parameters(WriteFileRequest { path, content }): Parameters<WriteFileRequest>,
    ) -> String {
        let result = ToolDispatcher::execute(ToolRequest::WriteFile { path, content });
        if result.success {
            "File written successfully.".to_string()
        } else {
            format!("File write failed: {}", result.output)
        }
    }

    #[tool(description = "Append content to a project file")]
    async fn append_file(
        &self,
        Parameters(AppendFileRequest { path, content }): Parameters<AppendFileRequest>,
    ) -> String {
        let result = ToolDispatcher::execute(ToolRequest::AppendFile { path, content });
        if result.success {
            "File appended successfully.".to_string()
        } else {
            format!("File append failed: {}", result.output)
        }
    }

    #[tool(description = "Create a directory in the project workspace")]
    async fn create_directory(
        &self,
        Parameters(CreateDirectoryRequest { path }): Parameters<CreateDirectoryRequest>,
    ) -> String {
        let result = ToolDispatcher::execute(ToolRequest::CreateDirectory(path));
        if result.success {
            result.output
        } else {
            format!("Directory creation failed: {}", result.output)
        }
    }

    #[tool(description = "Run cargo check on the project")]
    async fn cargo_check(&self) -> String {
        let result = ToolDispatcher::execute(ToolRequest::CargoCheck);
        if result.success {
            format!("Cargo check passed.\n{}", result.output)
        } else {
            format!("Cargo check failed:\n{}", result.output)
        }
    }

    #[tool(description = "Run cargo fmt on the project")]
    async fn cargo_fmt(&self) -> String {
        let result = ToolDispatcher::execute(ToolRequest::CargoFmt);
        if result.success {
            "Cargo format completed.".to_string()
        } else {
            format!("Cargo format failed: {}", result.output)
        }
    }

    #[tool(description = "Return the current Git status")]
    async fn git_status(&self) -> String {
        let result = ToolDispatcher::execute(ToolRequest::GitStatus);
        if result.success {
            result.output
        } else {
            format!("Git status failed: {}", result.output)
        }
    }

    #[tool(description = "Stage all current project changes")]
    async fn git_add(&self) -> String {
        let result = ToolDispatcher::execute(ToolRequest::GitAdd);
        if result.success {
            "Git add completed.".to_string()
        } else {
            format!("Git add failed: {}", result.output)
        }
    }

    #[tool(description = "Commit current project changes")]
    async fn git_commit(
        &self,
        Parameters(GitCommitRequest { message }): Parameters<GitCommitRequest>,
    ) -> String {
        let result = ToolDispatcher::execute(ToolRequest::GitCommit(message));
        if result.success {
            format!("Git commit completed.\n{}", result.output)
        } else {
            format!("Git commit failed: {}", result.output)
        }
    }
}

#[rmcp::tool_handler]
impl ServerHandler for AethyronMcp {}
