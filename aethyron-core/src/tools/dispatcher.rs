use crate::models::{compiler::Compiler, tool_request::ToolRequest, tool_result::ToolResult};

use crate::tools::{editor::EditorTool, filesystem::FileSystem};

pub struct ToolDispatcher;

impl ToolDispatcher {
    pub fn execute(request: ToolRequest) -> ToolResult {
        match request {
            ToolRequest::InspectProject => FileSystem::inspect_project_result(),

            ToolRequest::ReadFile(path) => match FileSystem::read(&path) {
                Ok(content) => ToolResult {
                    success: true,
                    output: content,
                },
                Err(error) => ToolResult {
                    success: false,
                    output: error.to_string(),
                },
            },

            ToolRequest::WriteFile { path, content } => match EditorTool::write(path, &content) {
                Ok(()) => ToolResult {
                    success: true,
                    output: String::new(),
                },
                Err(error) => ToolResult {
                    success: false,
                    output: error.to_string(),
                },
            },

            ToolRequest::AppendFile { path, content } => {
                match EditorTool::append(path, &content) {
                    Ok(()) => ToolResult {
                        success: true,
                        output: String::new(),
                    },
                    Err(error) => ToolResult {
                        success: false,
                        output: error.to_string(),
                    },
                }
            }

            ToolRequest::CreateDirectory(path) => {
                match std::fs::create_dir_all(&path) {
                    Ok(()) => ToolResult {
                        success: true,
                        output: format!("Directory created: {}", path),
                    },
                    Err(error) => ToolResult {
                        success: false,
                        output: error.to_string(),
                    },
                }
            }

            ToolRequest::CargoCheck => match Compiler::check() {
                Ok(output) => ToolResult {
                    success: true,
                    output,
                },
                Err(error) => ToolResult {
                    success: false,
                    output: error.to_string(),
                },
            },

            ToolRequest::CargoFmt => {
                match std::process::Command::new("cargo").arg("fmt").status() {
                    Ok(status) => ToolResult {
                        success: status.success(),
                        output: String::new(),
                    },
                    Err(error) => ToolResult {
                        success: false,
                        output: error.to_string(),
                    },
                }
            }

            ToolRequest::GitStatus => {
                match std::process::Command::new("git")
                    .arg("status")
                    .output()
                {
                    Ok(output) => ToolResult {
                        success: output.status.success(),
                        output: String::from_utf8_lossy(&output.stdout).to_string(),
                    },
                    Err(error) => ToolResult {
                        success: false,
                        output: error.to_string(),
                    },
                }
            }

            ToolRequest::GitAdd => {
                match std::process::Command::new("git")
                    .args(["add", "."])
                    .status()
                {
                    Ok(status) => ToolResult {
                        success: status.success(),
                        output: String::new(),
                    },
                    Err(error) => ToolResult {
                        success: false,
                        output: error.to_string(),
                    },
                }
            }

            ToolRequest::GitCommit(message) => {
                match std::process::Command::new("git")
                    .args(["commit", "-m", &message])
                    .output()
                {
                    Ok(output) => ToolResult {
                        success: output.status.success(),
                        output: String::from_utf8_lossy(&output.stdout).to_string(),
                    },
                    Err(error) => ToolResult {
                        success: false,
                        output: error.to_string(),
                    },
                }
            }
        }
    }
}
