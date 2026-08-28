use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

use crate::core::project_index::ProjectIndex;
use crate::core::rust_parser::RustParser;

pub struct ProjectIndexer;

impl ProjectIndexer {
    pub fn build(workspace: impl AsRef<Path>) -> Result<ProjectIndex> {
        let workspace = workspace.as_ref();
        let mut index = ProjectIndex::default();

        for entry in WalkDir::new(workspace.join("src")) {
            let entry = entry?;

            if !entry.file_type().is_file() {
                continue;
            }

            if entry.path().extension().and_then(|s| s.to_str()) != Some("rs") {
                continue;
            }

            let path = entry.path().display().to_string();

            index.files.push(path.clone());

            let source = std::fs::read_to_string(entry.path())?;

            for line in source.lines() {
                let line = line.trim();

                if line.starts_with("pub struct ") || line.starts_with("struct ") {
                    index.structs.push(line.to_string());
                }

                if line.starts_with("pub enum ") || line.starts_with("enum ") {
                    index.enums.push(line.to_string());
                }

                if line.starts_with("pub trait ") || line.starts_with("trait ") {
                    index.traits.push(line.to_string());
                }

                if line.starts_with("pub fn ") || line.starts_with("fn ") {
                    index.functions.push(line.to_string());
                }

                if line.starts_with("mod ") || line.starts_with("pub mod ") {
                    index.modules.push(line.to_string());
                }
            }

            // Extract detailed symbols (impl blocks, trait impls, associated fns)
            // using RustParser and merge any not already captured.
            let symbols = RustParser::extract_symbols(&source);
            for symbol in symbols {
                if !index.functions.contains(&symbol) && !index.structs.contains(&symbol) {
                    index.functions.push(symbol);
                }
            }
        }

        Ok(index)
    }
}
