use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

pub struct EditorTool;

impl EditorTool {

    fn resolve_path(path: impl AsRef<Path>) -> PathBuf {
        let workspace = Path::new("workspace/missions");

        workspace.join(path)
    }


    pub fn write(
        path: impl AsRef<Path>,
        content: &str
    ) -> Result<()> {

        let full_path = Self::resolve_path(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(full_path, content)?;

        Ok(())
    }


    pub fn append(
        path: impl AsRef<Path>,
        content: &str
    ) -> Result<()> {

        let full_path = Self::resolve_path(path);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(full_path)?
            .write_all(content.as_bytes())?;

        Ok(())
    }
}