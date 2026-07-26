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
    ) -> Result<CodeChange> {

        let client = OllamaClient::new();

        let prompt = format!(
r#"You are a senior Rust engineer.
Task:
{}
Return EXACTLY in this format.
PATH: src/example.rs
-----BEGIN CODE-----
Rust source code only
-----END CODE-----

Rules:
- No markdown.
- No JSON.
- No explanations.
- No comments before or after.
"#,
            instruction
        );

        let response =
            client.generate(&prompt).await?;

        Self::parse_generated_file(&response)
    }

    pub async fn fix(
        request: &FixRequest,
    ) -> Result<CodeChange> {

        let client = OllamaClient::new();

        let prompt = format!(
r#"You generated Rust code that failed to compile.
Compiler output:
{}
Previous code:
{}
Return EXACTLY in this format.
PATH: same file
-----BEGIN CODE-----
Corrected Rust source code
-----END CODE-----
Rules:
- No markdown.
- No JSON.
- No explanations.
"#,
            request.compiler_output,
            request.previous_code,
        );

        let response =
            client.generate(&prompt).await?;

        Self::parse_generated_file(&response)
    }

    fn parse_generated_file(
        response: &str,
    ) -> Result<CodeChange> {

        let path_prefix = "PATH:";
        let begin = "-----BEGIN CODE-----";
        let end = "-----END CODE-----";

        let path_start =
            response
                .find(path_prefix)
                .ok_or_else(|| anyhow!("Missing PATH"))?;

        let begin_start =
            response
                .find(begin)
                .ok_or_else(|| anyhow!("Missing BEGIN CODE marker"))?;

        let end_start =
            response
                .find(end)
                .ok_or_else(|| anyhow!("Missing END CODE marker"))?;

        let path =
            response[path_start + path_prefix.len()..begin_start]
                .trim()
                .to_string();

        let content = response
        .get(begin_start + begin.len()..end_start)
        .ok_or_else(|| anyhow!("Invalid code boundaries"))?
        .trim()
        .to_string();

        if path.is_empty() {
            return Err(anyhow!("Generated file path is empty"));
        }

        if content.is_empty() {
            return Err(anyhow!("Generated file content is empty"));
        }

        Ok(CodeChange {
            path,
            content,
        })
    }
}