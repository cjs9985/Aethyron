use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::models::tool_result::ToolResult;

pub struct FileSystem;

impl FileSystem {
    pub fn list(path: impl AsRef<Path>) -> Result<Vec<String>> {
        let mut files = Vec::new();

        Self::walk(path.as_ref(), &mut files)?;

        Ok(files)
    }

    fn walk(path: &Path, files: &mut Vec<String>) -> Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;

                let entry_path = entry.path();

                // Ignore build artifacts
                if entry_path.ends_with("target") {
                    continue;
                }

                if entry_path.is_dir() {
                    Self::walk(&entry_path, files)?;
                } else {
                    files.push(entry_path.display().to_string());
                }
            }
        }

        Ok(())
    }

    pub fn read(path: impl AsRef<Path>) -> Result<String> {
        Ok(fs::read_to_string(path)?)
    }

    pub fn inspect_project() -> Result<Vec<String>> {
        Self::list(".")
    }

    pub fn inspect_project_result() -> ToolResult {
        match Self::list(".") {
            Ok(files) => ToolResult {
                success: true,
                output: files.join("\n"),
            },

            Err(error) => ToolResult {
                success: false,
                output: error.to_string(),
            },
        }
    }
}
