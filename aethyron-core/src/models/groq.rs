/// Groq cloud API client — OpenAI-compatible endpoint.
///
/// Uses `llama-3.1-70b-versatile` for chat/review and
/// `llama-3.3-70b-versatile` for code generation.
/// Falls back gracefully when the API returns an error.
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const GROQ_CHAT_MODEL: &str = "llama-3.1-70b-versatile";
const GROQ_CODE_MODEL: &str = "llama-3.3-70b-versatile";
/// Groq does not support image inputs on most models.
/// Use the text model with a description request instead.
const GROQ_VISION_MODEL: &str = "llama-3.1-70b-versatile";

// ---------------------------------------------------------------------------
// Wire types
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<GroqMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Clone)]
struct GroqMessage {
    role: String,
    content: String,
}

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
    /// Returns `None` when `GROQ_API_KEY` is not set.
    pub fn from_env() -> Option<Self> {
        std::env::var("GROQ_API_KEY").ok().map(|key| Self { api_key: key })
    }

    fn http_client() -> reqwest::Client {
        reqwest::Client::new()
    }

    async fn complete(
        &self,
        model: &str,
        system: &str,
        user: &str,
        temperature: f32,
        max_tokens: u32,
    ) -> Result<String> {
        let messages = vec![
            GroqMessage { role: "system".to_string(), content: system.to_string() },
            GroqMessage { role: "user".to_string(),   content: user.to_string()   },
        ];

        let request = GroqRequest {
            model: model.to_string(),
            messages,
            temperature,
            max_tokens,
        };

        let response = Self::http_client()
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

    // -----------------------------------------------------------------------
    // Public API — mirrors OllamaClient signatures
    // -----------------------------------------------------------------------

    /// Code generation — strict structured output.
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

        let result = self.complete(GROQ_CODE_MODEL, system, prompt, 0.0, 4096).await?;

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

        let result = self.complete(GROQ_CHAT_MODEL, system, &user, 0.7, 2048).await?;
        println!("📡 Conversational response received from Groq");
        Ok(result)
    }

    /// Vision / image analysis — Groq text models cannot process raw images,
    /// so we ask the model to analyse based on the user's description.
    pub async fn chat_with_image(
        &self,
        message: &str,
        _images: Vec<String>,
        conversation_history: &str,
    ) -> Result<String> {
        println!("🖼️  Groq vision: analysing via text model ({})...", GROQ_VISION_MODEL);

        let system = r#"You are Aethyron, an autonomous AI coding assistant.
The user has attached an image and described it. Respond based on their description.
If the description mentions code, errors, diagrams, or UI, provide actionable insights.
Be concise and direct."#;

        let user_msg = if conversation_history.is_empty() {
            format!("[User attached an image]\n{}", message)
        } else {
            format!(
                "Conversation so far:\n{}\n\nUser [attached an image]: {}",
                conversation_history, message
            )
        };

        let result = self.complete(GROQ_VISION_MODEL, system, &user_msg, 0.7, 2048).await?;
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
