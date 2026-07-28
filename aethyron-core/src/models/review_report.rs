#[derive(Debug, Clone)]
pub struct ReviewReport {
    pub passed: bool,
    pub structural: bool,
    pub security: bool,
    pub compilation: bool,
    pub ai_review: bool,
    pub feedback: String,
}
