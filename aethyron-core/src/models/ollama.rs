use anyhow::Result;
use serde::{Deserialize, Serialize};

const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";
const DEFAULT_MODEL: &str = "qwen2.5-coder:7b";

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

#[derive(Deserialize)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Deserialize)]
struct OllamaModel {
    name: String,
}

pub struct OllamaClient {
    endpoint: String,
    model: String,
}

impl OllamaClient {
    pub fn new() -> Self {
        Self {
            endpoint: format!("{}/api/generate", DEFAULT_OLLAMA_URL),
            model: DEFAULT_MODEL.to_string(),
        }
    }

    pub async fn check(&self) -> Result<bool> {
        let endpoint = format!("{}/api/tags", DEFAULT_OLLAMA_URL);

        let response = reqwest::Client::new()
            .get(endpoint)
            .send()
            .await?
            .error_for_status()?
            .json::<OllamaTagsResponse>()
            .await?;

        Ok(response.models.iter().any(|model| model.name == self.model))
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
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
            model: self.model.clone(),
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
            .error_for_status()?
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
