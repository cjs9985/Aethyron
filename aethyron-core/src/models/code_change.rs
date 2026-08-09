#[derive(Debug)]
pub struct CodeChange {
    pub path: String,
    pub content: String,
    pub is_patch: bool,
}