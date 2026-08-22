use anyhow::{Result, anyhow};

use crate::models::{code_change::CodeChange, fix_request::FixRequest, ollama::OllamaClient};

pub struct CodeGenerator;

impl CodeGenerator {
    pub async fn generate(instruction: &str, project_index: &str) -> Result<CodeChange> {
        let client = OllamaClient::new();

        let prompt = format!(
            r#"You are Aethyron's autonomous Rust code generation engine.

You are communicating with software, not a human.

Your response is parsed automatically.

If you output anything except the required protocol, the response will be rejected.

========================
OUTPUT FORMAT
========================

Your entire response MUST be exactly:

PATH: relative/project/path
-----BEGIN CODE-----
code only
-----END CODE-----

No Markdown.
No explanations.
No JSON.
No code fences.
No comments before PATH.
No text after END CODE.

========================
PROJECT
========================

Project Index:

{}

========================
TASK
========================

{}

========================
RULES
========================

1. Before selecting a file, inspect the Project Index.

2. If the task specifies a destination path, use that exact path.

3. Otherwise modify the closest existing file.

4. Never invent filenames.

5. Never return NO_VALID_PATH.

6. Never refuse the task.

7. If the task requires a new file, create it.

========================
Cargo.toml
========================

When modifying Cargo.toml:

Return ONLY the dependency lines to add.

Example:

PATH: Cargo.toml
-----BEGIN CODE-----
bcrypt = "0.12"
argon2 = "0.5"
-----END CODE-----

Never output:

[package]
[workspace]
[dependencies]

========================
Rust files
========================

Modify only the requested portion.

Do not rewrite an entire file.

Preserve the existing style.

Reuse existing modules whenever possible.

========================
Security
========================

Never store plaintext passwords.

Never compare plaintext passwords.

Use Argon2 or bcrypt.

Store only password hashes.

Verify hashes during authentication.
"#,
            project_index, instruction,
        );

        let response = client.generate(&prompt).await?;

        Self::parse_generated_file(&response, project_index)
    }

    pub async fn fix(request: &FixRequest) -> Result<CodeChange> {
        let client = OllamaClient::new();

        let prompt = format!(
            r#"
You are repairing Rust code.

Compiler error:

{}

Previous code:

{}

Return only the corrected file.

Format:

PATH: <relative Rust file path>
-----BEGIN CODE-----
Rust source code only
-----END CODE-----

PATH rules:
- The PATH must be the actual file to modify.
- Choose an existing Rust source file whenever possible.
- If a new module is required, use a valid path under src/.
- Never output placeholders like:
  - existing/file.rs
  - appropriate_module.rs
  - file.rs
  - your_file.rs

Rules:
- No markdown.
- No explanations.
"#,
            request.compiler_output, request.previous_code
        );

        let response = client.generate(&prompt).await?;

        Self::parse_generated_file(&response, "")
    }

    fn parse_generated_file(response: &str, project_index: &str) -> Result<CodeChange> {
        let cleaned = response.trim();

        let begin_marker = "-----BEGIN CODE-----";
        let end_marker = "-----END CODE-----";

        if cleaned.is_empty() {
            return Err(anyhow!("Model returned an empty response."));
        }

        if cleaned.contains("```") {
            return Err(anyhow!("Generated response contains Markdown code fences."));
        }

        if !cleaned.starts_with("PATH:") {
            return Err(anyhow!(
                "Model did not return a valid code patch: missing PATH."
            ));
        }

        let path_and_code = cleaned
            .strip_prefix("PATH:")
            .ok_or_else(|| anyhow!("Missing PATH marker."))?;

        let begin_start = path_and_code
            .find(begin_marker)
            .ok_or_else(|| anyhow!("Missing BEGIN CODE marker."))?;

        let path_section = path_and_code[..begin_start].trim();

        if path_section.contains('\n') || path_section.contains('\r') {
            return Err(anyhow!("PATH must be a single line."));
        }

        if path_section.contains(end_marker) {
            return Err(anyhow!("Malformed PATH: END CODE marker found in PATH."));
        }

        if path_section.contains(begin_marker) {
            return Err(anyhow!("Malformed PATH: duplicate BEGIN CODE marker."));
        }

        let path = path_section.to_string();

        if path.is_empty() {
            return Err(anyhow!("Generated path empty."));
        }

        let forbidden = [
            "existing/file.rs",
            "appropriate_module.rs",
            "file.rs",
            "your_file.rs",
            "src/example.rs",
            "relative/project/path",
        ];

        if forbidden.contains(&path.as_str()) {
            return Err(anyhow!("Placeholder path returned by model."));
        }

        if path.contains('<') || path.contains('>') {
            return Err(anyhow!("Placeholder PATH returned."));
        }

        if path.contains("-----") {
            return Err(anyhow!("Malformed generated path."));
        }

        let code_start = begin_start + begin_marker.len();

        let remaining = &path_and_code[code_start..];

        let end_start = remaining
            .find(end_marker)
            .ok_or_else(|| anyhow!("Missing END CODE marker."))?;

        let content = remaining[..end_start].trim().to_string();

        let trailing = remaining[end_start + end_marker.len()..].trim();

        if !trailing.is_empty() {
            return Err(anyhow!("Generated response contains text after END CODE."));
        }

        if content.is_empty() {
            return Err(anyhow!("Generated code empty."));
        }

        if content.contains(end_marker) {
            return Err(anyhow!("Multiple END CODE markers found."));
        }

        if !project_index.is_empty() {
            let normalized_path = path.replace('\\', "/");

            let exists = project_index.lines().any(|line| {
                let candidate = line.trim().replace('\\', "/");

                candidate == normalized_path
                    || candidate.ends_with(&normalized_path)
                    || normalized_path.ends_with(&candidate)
            });

            if !exists {
                let allowed_new_module =
                    normalized_path.starts_with("src/") && normalized_path.ends_with(".rs");

                if !allowed_new_module {
                    return Err(anyhow!("Model selected invalid project path: {}", path));
                }
            }
        }

        Ok(CodeChange {
            path,
            content,
            is_patch: true,
        })
    }
}
