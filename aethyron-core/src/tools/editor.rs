use anyhow::Result;
use std::fs;
use std::path::Path;
use std::io::Write;

pub struct EditorTool;

impl EditorTool {

    pub fn write(
        path: impl AsRef<Path>,
        content: &str
    ) -> Result<()> {

        fs::write(path, content)?;

        Ok(())
    }


    pub fn append(
        path: impl AsRef<Path>,
        content: &str
    ) -> Result<()> {

        fs::OpenOptions::new()
            .append(true)
            .open(path)?
            .write_all(content.as_bytes())?;

        Ok(())
    }
}