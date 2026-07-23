#[derive(Clone, Debug)]
pub struct ProjectContext {
    pub cargo_toml: String,
    pub files: Vec<String>,
}