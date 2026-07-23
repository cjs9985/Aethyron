use anyhow::Result;

use crate::tools::filesystem::FileSystem;
use crate::models::project_context::ProjectContext;


pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build() -> Result<ProjectContext> {
        Ok(ProjectContext {
            cargo_toml: FileSystem::read("Cargo.toml")?,
            files: FileSystem::list(".")?,
        })
    }
}