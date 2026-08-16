use anyhow::Result;
use std::path::Path;

use crate::core::project_indexer::ProjectIndexer;
use crate::memory::store::MemoryStore;
use crate::models::project_context::ProjectContext;
use crate::tools::filesystem::FileSystem;

pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build(workspace: impl AsRef<Path>) -> Result<ProjectContext> {
        let workspace = workspace.as_ref();
        let index = ProjectIndexer::build(workspace)?;

        Ok(ProjectContext {
            cargo_toml: FileSystem::read(workspace.join("Cargo.toml"))?,
            files: FileSystem::list(workspace)?,
            memory: MemoryStore::load()?,
            project_index: index.summary(),
        })
    }
}
