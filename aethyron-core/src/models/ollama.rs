use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    system: String,
    prompt: String,
    stream: bool,
    temperature: f32,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct OllamaClient {
    endpoint: String,
}

impl OllamaClient {
    pub fn new() -> Self {
        Self {
            endpoint:
                "http://127.0.0.1:11434/api/generate"
                    .to_string(),
        }
    }

    pub async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String> {

        println!("🧠 Sending request to Ollama...");
        println!("⏳ Model is reasoning...");

        let client = reqwest::Client::new();

        let system_prompt = r#"
You are Aethyron's autonomous Rust code generation engine.

You are communicating with software.

Your response is parsed automatically.

Return ONLY:

PATH: relative/path
-----BEGIN CODE-----
code
-----END CODE-----

Never explain.
Never apologize.
Never output markdown.
Never output JSON.
Never output examples.
Never output prose.

If modifying Cargo.toml, output ONLY dependency lines.

Never refuse a task.

Always produce a valid PATH.
"#;

       let request = OllamaRequest {
    model: "qwen2.5-coder:7b".to_string(),
    system: system_prompt.to_string(),
    prompt: prompt.to_string(),
    stream: false,
    temperature: 0.0,
};

        let response = client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?
            .json::<OllamaResponse>()
            .await?;

        println!("📡 Response received from Ollama");
        println!("================ MODEL RESPONSE ================");
        println!("{}", response.response);
        println!("================================================");
   let normalized = Self::normalize_response(&response.response);

Ok(normalized)
    }

    fn normalize_response(response: &str) -> String {

        response
            .replace("```json", "")
            .replace("```rust", "")
            .replace("```text", "")
            .replace("```", "")
            .trim()
            .to_string()
    }
}