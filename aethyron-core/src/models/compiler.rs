use anyhow::Result;

use crate::tools::terminal::TerminalTool;

pub struct Compiler;

impl Compiler {
    pub fn check() -> Result<String> {
        TerminalTool::run("cargo", &["check"])
    }
}
