/// Unified model client factory.
///
/// Routes to Groq when GROQ_API_KEY is set, otherwise uses the local
/// Ollama instance. Every Groq call also falls back to Ollama
/// automatically on failure (bad key, model 404, rate limit, outage),
/// so chat keeps working even when the cloud backend is down.
use anyhow::Result;

use crate::models::groq::GroqClient;
use crate::models::ollama::OllamaClient;

pub enum ModelClient {
    Groq(GroqClient),
    Ollama(OllamaClient),
}

impl ModelClient {
    /// Pick the best available backend.
    /// Groq is preferred when GROQ_API_KEY is present in the environment.
    pub fn new() -> Self {
        if let Some(groq) = GroqClient::from_env() {
            println!("🌩️  Using Groq cloud backend (fallback: local Ollama)");
            Self::Groq(groq)
        } else {
            println!("🦙  Using local Ollama backend");
            Self::Ollama(OllamaClient::new())
        }
    }

    /// Structured code generation — strict PATH/CODE protocol.
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        match self {
            Self::Groq(c) => match c.generate(prompt).await {
                Ok(reply) => Ok(reply),
                Err(e) => {
                    eprintln!("⚠️  Groq failed ({}), using Ollama instead.", e);
                    OllamaClient::new().generate(prompt).await
                }
            },
            Self::Ollama(c) => c.generate(prompt).await,
        }
    }

    /// Conversational reply — plain language.
    pub async fn chat(&self, message: &str, conversation_history: &str) -> Result<String> {
        match self {
            Self::Groq(c) => match c.chat(message, conversation_history).await {
                Ok(reply) => Ok(reply),
                Err(e) => {
                    eprintln!("⚠️  Groq failed ({}), using Ollama instead.", e);
                    OllamaClient::new()
                        .chat(message, conversation_history)
                        .await
                }
            },
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
            Self::Groq(c) => match c
                .chat_with_image(message, images.clone(), conversation_history)
                .await
            {
                Ok(reply) => Ok(reply),
                Err(e) => {
                    eprintln!("⚠️  Groq vision failed ({}), using Ollama instead.", e);
                    OllamaClient::new()
                        .chat_with_image(message, images, conversation_history)
                        .await
                }
            },
            Self::Ollama(c) => {
                c.chat_with_image(message, images, conversation_history)
                    .await
            }
        }
    }
}
