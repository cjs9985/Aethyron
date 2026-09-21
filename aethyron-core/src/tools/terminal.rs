use anyhow::Result;
use std::process::Command;

pub struct TerminalTool;

impl TerminalTool {
    pub fn run(command: &str, args: &[&str]) -> Result<String> {
        let output = Command::new(command).args(args).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Combine stdout + stderr so callers see the full output.
        let combined = match (stdout.trim().is_empty(), stderr.trim().is_empty()) {
            (false, false) => format!("{}\n{}", stdout, stderr),
            (true, false)  => stderr.to_string(),
            _              => stdout.to_string(),
        };

        if output.status.success() {
            Ok(combined)
        } else {
            Err(anyhow::anyhow!("{}", combined))
        }
    }
}
