/// Unified model client factory.
///
/// Returns a `ModelClient` that routes to Groq when `GROQ_API_KEY` is set,
/// otherwise falls back to the local Ollama instance.
use anyhow::Result;

use crate::models::groq::GroqClient;
use crate::models::ollama::OllamaClient;

pub enum ModelClient {
    Groq(GroqClient),
    Ollama(OllamaClient),
}

impl ModelClient {
    /// Pick the best available backend.
    /// Groq is preferred when `GROQ_API_KEY` is present in the environment.
    pub fn new() -> Self {
        if let Some(groq) = GroqClient::from_env() {
            println!("🌩️  Using Groq cloud backend");
            Self::Groq(groq)
        } else {
            println!("🦙  Using local Ollama backend");
            Self::Ollama(OllamaClient::new())
        }
    }

    /// Structured code generation — strict PATH/CODE protocol.
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        match self {
            Self::Groq(c)   => c.generate(prompt).await,
            Self::Ollama(c) => c.generate(prompt).await,
        }
    }

    /// Conversational reply — plain language.
    pub async fn chat(&self, message: &str, conversation_history: &str) -> Result<String> {
        match self {
            Self::Groq(c)   => c.chat(message, conversation_history).await,
            Self::Ollama(c) => c.chat(message, conversation_history).await,
        }
    }

    /// Vision / image analysis.
    pub async fn chat_with_image(
        &self,
        message: &str,
        images: Vec<String>,
        conversation_history: &str,
    ) -> Result<String> {
        match self {
            Self::Groq(c)   => c.chat_with_image(message, images, conversation_history).await,
            Self::Ollama(c) => c.chat_with_image(message, images, conversation_history).await,
        }
    }
}
