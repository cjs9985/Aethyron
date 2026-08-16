use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Plan {
    pub tasks: Vec<String>,
}
