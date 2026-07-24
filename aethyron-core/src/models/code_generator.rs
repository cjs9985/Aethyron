use anyhow::Result;
use serde::Deserialize;
use crate::models::
     {ollama::OllamaClient,
      code_change::CodeChange,
      fix_request::FixRequest,
};


#[derive(Deserialize)]
struct GeneratedFile {
    path: String,
    content: String,
}
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

Return JSON only:

{{
  "path": "src/example.rs",
  "content": "rust code here"
}}

Do not include markdown.
"#,
            instruction
        );


        let response =
            client.generate(&prompt).await?;


        let file: GeneratedFile =
            serde_json::from_str(&response)?;


        Ok(CodeChange {
            path: file.path,
            content: file.content,
        })
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

Return JSON only:

{{
  "path": "same file",
  "content": "corrected rust code"
}}

Do not include markdown."#,
        request.compiler_output,
        request.previous_code,
    );

    let response = client.generate(&prompt).await?;

    let file: GeneratedFile =
        serde_json::from_str(&response)?;

    Ok(CodeChange {
        path: file.path,
        content: file.content,
    })
}
}