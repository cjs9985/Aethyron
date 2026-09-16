use anyhow::Result;
use std::process::Command;

pub struct TerminalTool;

impl TerminalTool {
    pub fn run(command: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(command).args(args).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            Ok(format!("STDERR:\n{}", stderr))
        } else {
            Ok(stdout.to_string())
        }
    }
}
