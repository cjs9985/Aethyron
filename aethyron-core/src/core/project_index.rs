#[derive(Debug, Default, Clone)]
pub struct ProjectIndex {
    pub modules: Vec<String>,
    pub structs: Vec<String>,
    pub enums: Vec<String>,
    pub traits: Vec<String>,
    pub functions: Vec<String>,
    pub files: Vec<String>,
}

impl ProjectIndex {
    pub fn summary(&self) -> String {
        format!(
            "Files:\n{}\n\nModules:\n{}\n\nStructs:\n{}\n\nTraits:\n{}\n\nFunctions:\n{}",
            self.files.join("\n"),
            self.modules.join("\n"),
            self.structs.join("\n"),
            self.traits.join("\n"),
            self.functions.join("\n"),
        )
    }
}
