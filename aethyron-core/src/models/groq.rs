/// Groq cloud API client — OpenAI-compatible endpoint.
///
/// Uses `llama-3.3-70b-versatile` for code generation,
/// `llama-3.1-70b-versatile` for chat/review, and
/// `meta-llama/llama-4-scout-17b-16e-instruct` for vision (image input).
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const GROQ_CHAT_MODEL: &str = "llama-3.1-70b-versatile";
const GROQ_CODE_MODEL: &str = "llama-3.3-70b-versatile";
/// Vision-capable model on Groq — supports image_url content parts.
const GROQ_VISION_MODEL: &str = "meta-llama/llama-4-scout-17b-16e-instruct";

// ---------------------------------------------------------------------------
// Wire types — text-only path
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct GroqTextRequest {
    model: String,
    messages: Vec<GroqTextMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct GroqTextMessage {
    role: String,
    content: String,
}

// ---------------------------------------------------------------------------
// Wire types — vision path (content is a JSON array of parts)
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct GroqVisionRequest {
    model: String,
    messages: Vec<GroqVisionMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize)]
struct GroqVisionMessage {
    role: String,
    /// Array of content parts — text and/or image_url.
    content: Vec<Value>,
}

// ---------------------------------------------------------------------------
// Shared response type
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct GroqResponse {
    choices: Vec<GroqChoice>,
}

#[derive(Deserialize)]
struct GroqChoice {
    message: GroqChoiceMessage,
}

#[derive(Deserialize)]
struct GroqChoiceMessage {
    content: String,
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

pub struct GroqClient {
    api_key: String,
}

impl GroqClient {
    /// Returns `None` when `GROQ_API_KEY` is not set in the environment.
    pub fn from_env() -> Option<Self> {
        std::env::var("GROQ_API_KEY")
            .ok()
            .filter(|k| !k.trim().is_empty())
            .map(|key| Self { api_key: key })
    }

    fn http() -> reqwest::Client {
        reqwest::Client::new()
    }

    /// Send a text-only completion request.
    async fn complete_text(
        &self,
        model: &str,
        system: &str,
        user: &str,
        temperature: f32,
        max_tokens: u32,
    ) -> Result<String> {
        let request = GroqTextRequest {
            model: model.to_string(),
            messages: vec![
                GroqTextMessage { role: "system".to_string(), content: system.to_string() },
                GroqTextMessage { role: "user".to_string(),   content: user.to_string()   },
            ],
            temperature,
            max_tokens,
        };

        let response = Self::http()
            .post(GROQ_API_URL)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<GroqResponse>()
            .await?;

        response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content.trim().to_string())
            .ok_or_else(|| anyhow!("Groq returned no choices"))
    }

    /// Send a multimodal (text + image) completion request.
    /// `image_b64` is raw base64 — no data-URL prefix.
    /// `mime` should be e.g. "image/png" or "image/jpeg".
    async fn complete_vision(
        &self,
        system: &str,
        text: &str,
        image_b64: &str,
    ) -> Result<String> {
        // Groq vision expects content as an array of parts:
        // [{ type: "text", text: "..." }, { type: "image_url", image_url: { url: "data:..." } }]
        let content_parts: Vec<Value> = vec![
            serde_json::json!({ "type": "text", "text": text }),
            serde_json::json!({
                "type": "image_url",
                "image_url": {
                    "url": format!("data:image/png;base64,{}", image_b64)
                }
            }),
        ];

        let request = GroqVisionRequest {
            model: GROQ_VISION_MODEL.to_string(),
            messages: vec![
                GroqVisionMessage {
                    role: "system".to_string(),
                    content: vec![serde_json::json!({ "type": "text", "text": system })],
                },
                GroqVisionMessage {
                    role: "user".to_string(),
                    content: content_parts,
                },
            ],
            temperature: 0.7,
            max_tokens: 2048,
        };

        let response = Self::http()
            .post(GROQ_API_URL)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<GroqResponse>()
            .await?;

        response
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content.trim().to_string())
            .ok_or_else(|| anyhow!("Groq vision returned no choices"))
    }

    // -----------------------------------------------------------------------
    // Public API — mirrors OllamaClient signatures
    // -----------------------------------------------------------------------

    /// Structured code generation — strict PATH/CODE protocol.
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        println!("🧠 Sending code generation request to Groq ({})...", GROQ_CODE_MODEL);

        let system = r#"You are Aethyron's autonomous Rust code generation engine.
You are communicating with software. Your response is parsed automatically.
Return ONLY:

PATH: relative/path
-----BEGIN CODE-----
code
-----END CODE-----

Never explain. Never apologize. Never output markdown. Never output JSON.
Never output examples. Never output prose.
If modifying Cargo.toml, output ONLY dependency lines.
Never refuse a task. Always produce a valid PATH."#;

        let result = self.complete_text(GROQ_CODE_MODEL, system, prompt, 0.0, 4096).await?;

        println!("📡 Code generation response received from Groq");
        println!("================ MODEL RESPONSE ================");
        println!("{}", result);
        println!("================================================");

        Ok(Self::normalize_response(&result))
    }

    /// Conversational reply — plain language.
    pub async fn chat(&self, message: &str, conversation_history: &str) -> Result<String> {
        println!("💬 Sending conversational message to Groq ({})...", GROQ_CHAT_MODEL);

        let system = r#"You are Aethyron, an autonomous AI coding assistant.
You are friendly, knowledgeable, and concise.
You can plan, implement, review, and repair Rust code on behalf of the user.
When the user asks a question, answer it clearly and helpfully in plain language.
Do not output JSON. Do not output code blocks unless the user specifically asks for code."#;

        let user = if conversation_history.is_empty() {
            message.to_string()
        } else {
            format!("Conversation so far:\n{}\n\nUser: {}", conversation_history, message)
        };

        let result = self.complete_text(GROQ_CHAT_MODEL, system, &user, 0.7, 2048).await?;
        println!("📡 Conversational response received from Groq");
        Ok(result)
    }

    /// Vision — sends the image to Groq's multimodal model.
    /// `images` contains raw base64 strings (no data-URL prefix).
    /// Only the first image is used (Groq currently supports one per request).
    pub async fn chat_with_image(
        &self,
        message: &str,
        images: Vec<String>,
        conversation_history: &str,
    ) -> Result<String> {
        println!("🖼️  Sending image to Groq vision model ({})...", GROQ_VISION_MODEL);

        let system = r#"You are Aethyron, an autonomous AI coding assistant with vision capabilities.
When the user sends an image, analyse it carefully.
If the image shows code, errors, a diagram, or a UI screenshot, provide actionable insights.
Be concise and direct."#;

        let image_b64 = images
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No image provided"))?;

        let text = if conversation_history.is_empty() {
            message.to_string()
        } else {
            format!("Conversation so far:\n{}\n\nUser: {}", conversation_history, message)
        };

        let result = self.complete_vision(system, &text, &image_b64).await?;
        println!("📡 Vision response received from Groq");
        Ok(result)
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
