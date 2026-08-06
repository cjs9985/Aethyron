use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

pub struct EditorTool;

impl EditorTool {

   fn resolve_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    // Already an absolute path
    if path.is_absolute() {
        return path.to_path_buf();
    }

    // Files that belong at the project root
    if path == Path::new("Cargo.toml")
        || path == Path::new("Cargo.lock")
        || path.starts_with(".github")
        || path.starts_with("README")
    {
        return path.to_path_buf();
    }

    // Everything else lives relative to the project root
    path.to_path_buf()
}

    pub fn write(
    path: impl AsRef<Path>,
    content: &str,
) -> Result<()> {

    let path = path.as_ref();

    // Cargo.toml should be patched, not replaced.
    if path == Path::new("Cargo.toml") {
        return Self::patch_cargo_toml(content);
    }

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
   fn patch_cargo_toml(content: &str) -> Result<()> {
    let manifest = Path::new("Cargo.toml");
    let mut existing = fs::read_to_string(manifest)?;

    // Extract only dependency lines from the model output.
    let dependencies: Vec<String> = content
        .lines()
        .map(str::trim)
        .filter(|line| {
            !line.is_empty()
                && !line.starts_with('[')
                && line.contains('=')
        })
        .map(String::from)
        .collect();

    if dependencies.is_empty() {
        return Ok(());
    }

    let marker = "[dependencies]";

    if let Some(pos) = existing.find(marker) {
        let insert_pos = pos + marker.len();

        let mut to_insert = String::new();

        for dep in dependencies {
            if !existing.contains(&dep) {
                to_insert.push('\n');
                to_insert.push_str(&dep);
            }
        }

        if !to_insert.is_empty() {
            existing.insert_str(insert_pos, &to_insert);
            fs::write(manifest, existing)?;
        }
    }

    Ok(())
}
}