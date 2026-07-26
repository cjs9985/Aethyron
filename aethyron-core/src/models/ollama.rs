use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
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

        let request = OllamaRequest {
            model: "qwen2.5-coder:7b".to_string(),
            prompt: prompt.to_string(),
            stream: false,
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

        Ok(Self::normalize_response(&response.response))
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