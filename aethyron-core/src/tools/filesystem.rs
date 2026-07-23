use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::models::tool_result::ToolResult;
pub struct FileSystem;

impl FileSystem {

    pub fn list(path: impl AsRef<Path>) -> Result<Vec<String>> {
        let mut files = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            files.push(
                entry.path()
                    .display()
                    .to_string()
            );
        }

        Ok(files)
    }

    pub fn read(path: impl AsRef<Path>) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }
    pub fn inspect_project() -> Result<Vec<String>> {
    Self::list(".")
}
pub fn inspect_project_result() -> ToolResult {

    match Self::list(".") {

        Ok(files) => {

            ToolResult {
                success: true,
                output: files.join("\n"),
            }

        }

        Err(error) => {

            ToolResult {
                success: false,
                output: error.to_string(),
            }

        }
    }
}
}