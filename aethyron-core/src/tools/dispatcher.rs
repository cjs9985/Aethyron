use anyhow::Result;

use crate::models::{compiler::Compiler, tool_request::ToolRequest};

use crate::tools::{editor::EditorTool, filesystem::FileSystem};

pub struct ToolDispatcher;

impl ToolDispatcher {
    pub fn execute(request: ToolRequest) -> Result<()> {
        match request {
            ToolRequest::InspectProject => {
                FileSystem::inspect_project()?;
            }

            ToolRequest::ReadFile(path) => {
                println!("{}", FileSystem::read(path)?);
            }

            ToolRequest::WriteFile { path, content } => {
                EditorTool::write(path, &content)?;
            }

            ToolRequest::AppendFile { path, content } => {
                EditorTool::append(path, &content)?;
            }

            ToolRequest::CreateDirectory(path) => {
                std::fs::create_dir_all(path)?;
            }

            ToolRequest::CargoCheck => {
                Compiler::check()?;
            }

            ToolRequest::CargoFmt => {
                std::process::Command::new("cargo").arg("fmt").status()?;
            }

            ToolRequest::GitStatus => {
                std::process::Command::new("git").arg("status").status()?;
            }

            ToolRequest::GitAdd => {
                std::process::Command::new("git")
                    .args(["add", "."])
                    .status()?;
            }

            ToolRequest::GitCommit(message) => {
                std::process::Command::new("git")
                    .args(["commit", "-m", &message])
                    .status()?;
            }
        }

        Ok(())
    }
}
