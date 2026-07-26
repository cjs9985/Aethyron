use anyhow::Result;
use serde::Deserialize;
use serde_json;
use serde_json::Value;
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

        let cleaned = response
        .replace("```json", "")
        .replace("```", "")
        .trim()
        .to_string();

        let json = Self::repair_json(&response);

        let repaired = Self::repair_json(&json);

        let file: GeneratedFile =
        serde_json::from_str(&json)?;

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

    let json = Self::repair_json(&response);
    
    let repaired = Self::repair_json(&response);
    let file: GeneratedFile =
        serde_json::from_str(&repaired)?;

    Ok(CodeChange {
        path: file.path,
        content: file.content,
    })
}
fn extract_json(response: &str) -> String {

    let cleaned = response
        .replace("```json", "")
        .replace("```rust", "")
        .replace("```", "")
        .trim()
        .to_string();

    if let Some(start) = cleaned.find('{') {
        if let Some(end) = cleaned.rfind('}') {
            return cleaned[start..=end].to_string();
        }
    }

    cleaned
}
fn repair_json(json: &str) -> String {
    if let Ok(_) = serde_json::from_str::<Value>(json) {
        return json.to_string();
    }

    json.replace("\"content\": \"\n", "\"content\": \"\\n")
        .replace("\n\"\n}", "\\n\"\n}")
}
}