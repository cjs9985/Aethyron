#[derive(Debug, Clone)]
pub struct CoderResult {
    pub files_changed: Vec<String>,
    pub generated_code: String,
}