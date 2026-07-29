use anyhow::{anyhow, Result};

use crate::models::{
    code_change::CodeChange,
    fix_request::FixRequest,
    ollama::OllamaClient,
};
pub struct CodeGenerator;
impl CodeGenerator {
    pub async fn generate(
        instruction: &str,
        project_index: &str,
    ) -> Result<CodeChange> {

        let client = OllamaClient::new();

        let prompt = format!(
r#"You are the senior Rust engineer responsible for maintaining an existing project.

Project Index:

{}

Task:

{}

Requirements:

- Modify the EXISTING project.
- Reuse existing modules whenever possible.
- Do NOT invent a new architecture.
- Do NOT generate unrelated examples.
- Only create new files when the task explicitly requires them.
- Preserve existing coding style.

Return EXACTLY in this format.

PATH: src/appropriate_module.rs
-----BEGIN CODE-----
Rust source code only
-----END CODE-----
The PATH value is an example format only. Never use src/example.rs unless the task explicitly names it.
Rules:

Security requirements:
- Never store plaintext passwords.
- Never compare plaintext passwords.
- Use bcrypt or Argon2 for password hashing.
- Password fields must store only password hashes.
- Authentication must verify hashes.

Code requirements:
- Reuse existing project structure.
- Do not create duplicate modules.
- Do not initialize new projects.
- Use existing dependencies from Cargo.toml.
IMPORTANT:
- Do not modify src/example.rs unless the task explicitly requires it.
- Do not create demo files.
- Use existing project modules.
- Before selecting a file, inspect the project index.
-Generated PATH MUST be an existing file from the Project Index.

Do NOT invent filenames.

Do NOT use:
- existing/file.rs
- src/example.rs
- main.rs unless explicitly requested.

If no existing file is appropriate, return:

NO_VALID_PATH

Output requirements:
- No markdown.
- No JSON.
- No explanations.
- No comments before or after.
"#,
    project_index,
    instruction,
);
        let response =
            client.generate(&prompt).await?;
        Self::parse_generated_file(&response, project_index,)
    }

    pub async fn fix(
        request: &FixRequest,
    ) -> Result<CodeChange> {

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
            request.compiler_output,
            request.previous_code
        );


        let response =
            client.generate(&prompt).await?;


        Self::parse_generated_file(&response, "",)
    }

    fn parse_generated_file(
        response: &str,
        project_index: &str,
    ) -> Result<CodeChange> {

        let cleaned = response
            .replace("```rust", "")
            .replace("```", "")
            .trim()
            .to_string();

        let path_marker = "PATH:";
        let begin_marker = "-----BEGIN CODE-----";
        let end_marker = "-----END CODE-----";

        let path_start =
            cleaned
                .find(path_marker)
                .ok_or_else(|| anyhow!("Missing PATH"))?;
        let begin_start =
            cleaned
                .find(begin_marker)
                .ok_or_else(|| anyhow!("Missing BEGIN CODE marker"))?;
        let end_start =
            cleaned
                .find(end_marker)
                .ok_or_else(|| anyhow!("Missing END CODE marker"))?;

        let path =
            cleaned
                [
                    path_start + path_marker.len()
                    ..
                    begin_start
                ]
                .trim()
                .to_string();

        let forbidden = [
    "existing/file.rs",
    "appropriate_module.rs",
    "file.rs",
    "your_file.rs",
    "src/example.rs",
];

       if forbidden.contains(&path.as_str()) {
          return Err(anyhow!(
        "Placeholder path returned by model."
    ));
}

        let content =
            cleaned
                [
                    begin_start + begin_marker.len()
                    ..
                    end_start
                ]
                .trim()
                .to_string();

        if path.is_empty() {
            return Err(anyhow!(
                "Generated path empty"
            ));
        }
        if !project_index.is_empty() {

    let exists =
        project_index
            .lines()
            .any(|line| line.trim() == path);

    if !exists {

        let allowed_new_module =
            path.starts_with("src/")
            && path.ends_with(".rs");

        if !allowed_new_module {

            return Err(anyhow!(
                "Model selected invalid project path: {}",
                path
            ));
        }
    }
}
        if content.is_empty() {
            return Err(anyhow!(
                "Generated code empty"
            ));
        }
        if !project_index.contains(&path) && !path.starts_with("src/") {
            return Err(anyhow!("Generated invalid project path"));
            }
        if path.contains('<')
    || path.contains('>')
{
    return Err(anyhow!(
        "Placeholder PATH returned."
    ));
}
        Ok(CodeChange {
            path,
            content,
        })
    }
}