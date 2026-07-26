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
r#"
You are the coding agent inside Aethyron.

Task:
{}

You are modifying an EXISTING Rust project.

Requirements:

1. Inspect existing structure mentally.
2. Do not create unrelated examples.
3. Do not create main functions unless requested.
4. Return ONLY a file modification.

Required format:

PATH: relative/path/file.rs
-----BEGIN CODE-----
Rust code only
-----END CODE-----

Rules:
- No markdown.
- No ```rust fences.
- No explanations.
- No comments outside code.
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
r#"
You are repairing Rust code.

Compiler error:

{}

Previous code:

{}

Return only the corrected file.

Format:

PATH: relative/path/file.rs
-----BEGIN CODE-----
Rust code only
-----END CODE-----

Rules:
- No markdown.
- No explanations.
"#,
            request.compiler_output,
            request.previous_code
        );


        let response =
            client.generate(&prompt).await?;


        Self::parse_generated_file(&response)
    }



    fn parse_generated_file(
        response: &str,
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


        if content.is_empty() {
            return Err(anyhow!(
                "Generated code empty"
            ));
        }



        Ok(CodeChange {
            path,
            content,
        })
    }
}