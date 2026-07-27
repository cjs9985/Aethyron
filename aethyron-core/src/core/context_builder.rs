use anyhow::Result;

use crate::core::project_indexer::ProjectIndexer;
use crate::memory::store::MemoryStore;
use crate::models::project_context::ProjectContext;
use crate::tools::filesystem::FileSystem;

pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build() -> Result<ProjectContext> {
        let index = ProjectIndexer::build()?;

        Ok(ProjectContext {
            cargo_toml: FileSystem::read("Cargo.toml")?,
            files: FileSystem::list(".")?,
            memory: MemoryStore::load()?,
            project_index: index.summary(),
        })
    }
}