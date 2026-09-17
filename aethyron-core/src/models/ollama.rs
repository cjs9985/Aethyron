use anyhow::Result;
use serde::{Deserialize, Serialize};

const DEFAULT_OLLAMA_URL: &str = "http://127.0.0.1:11434";
const DEFAULT_MODEL: &str = "qwen2.5-coder:7b";
/// Vision-capable model used when the user attaches an image.
const VISION_MODEL: &str = "llava:7b";

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    system: String,
    prompt: String,
    stream: bool,
    temperature: f32,
    /// Optional list of base64-encoded images (multimodal models only).
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Vec<String>>,
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

    /// Check whether the vision model is available locally.
    pub async fn check_vision_model(&self) -> Result<bool> {
        let endpoint = format!("{}/api/tags", DEFAULT_OLLAMA_URL);

        let response = reqwest::Client::new()
            .get(endpoint)
            .send()
            .await?
            .error_for_status()?
            .json::<OllamaTagsResponse>()
            .await?;

        Ok(response
            .models
            .iter()
            .any(|m| m.name.starts_with("llava")))
    }

    /// Send a conversational message that includes one or more images.
    /// `images` must be raw base64-encoded image bytes (no data-URL prefix).
    pub async fn chat_with_image(
        &self,
        message: &str,
        images: Vec<String>,
        conversation_history: &str,
    ) -> Result<String> {
        println!("🖼️  Sending multimodal message to Ollama ({})...", VISION_MODEL);

        let client = reqwest::Client::new();

        let system_prompt = r#"You are Aethyron, an autonomous AI coding assistant with vision capabilities.
When the user sends an image, describe what you see and relate it to any engineering context provided.
If the image shows code, a diagram, an error, or a UI screenshot, analyse it carefully and provide
actionable insights or implement what is requested.
Be concise and direct. Do not output markdown code fences unless the user asks for code."#;

        let prompt = if conversation_history.is_empty() {
            message.to_string()
        } else {
            format!(
                "Conversation so far:\n{}\n\nUser: {}",
                conversation_history, message
            )
        };

        let request = OllamaRequest {
            model: VISION_MODEL.to_string(),
            system: system_prompt.to_string(),
            prompt,
            stream: false,
            temperature: 0.7,
            images: Some(images),
        };

        let response = client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<OllamaResponse>()
            .await?;

        println!("📡 Vision response received from Ollama");

        Ok(response.response.trim().to_string())
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
            images: None,
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

    /// Send a plain conversational message and get a plain-text reply.
    /// Used when the user asks a question rather than issuing a coding mission.
    pub async fn chat(&self, message: &str, conversation_history: &str) -> Result<String> {
        println!("💬 Sending conversational message to Ollama...");

        let client = reqwest::Client::new();

        let system_prompt = r#"You are Aethyron, an autonomous AI coding assistant.
You are friendly, knowledgeable, and concise.
You can plan, implement, review, and repair Rust code on behalf of the user.
When the user asks a question, answer it clearly and helpfully in plain language.
When the user describes a coding goal or mission, confirm you understand and will proceed.
Do not output JSON. Do not output code blocks unless the user specifically asks for code.
"#;

        let prompt = if conversation_history.is_empty() {
            message.to_string()
        } else {
            format!(
                "Conversation so far:\n{}\n\nUser: {}",
                conversation_history, message
            )
        };

        let request = OllamaRequest {
            model: self.model.clone(),
            system: system_prompt.to_string(),
            prompt,
            stream: false,
            temperature: 0.7,
            images: None,
        };

        let response = client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<OllamaResponse>()
            .await?;

        println!("📡 Conversational response received from Ollama");

        Ok(response.response.trim().to_string())
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
