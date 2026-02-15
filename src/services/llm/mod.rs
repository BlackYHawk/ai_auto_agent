//! LLM Client Module

use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;

/// LLM client trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, context: &str, prompt: &str) -> Result<String>;
}

/// LLM client wrapper
pub struct LlmClient {
    provider: Box<dyn LlmProvider>,
}

impl LlmClient {
    pub fn new(provider: Box<dyn LlmProvider>) -> Self {
        Self { provider }
    }

    pub async fn generate(&self, context: &str, prompt: &str) -> Result<String> {
        self.provider.generate(context, prompt).await
    }
}

/// Qwen provider
#[allow(dead_code)]
pub struct QwenProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl QwenProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: "qwen-turbo".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for QwenProvider {
    async fn generate(&self, _context: &str, _prompt: &str) -> Result<String> {
        // TODO: Implement actual Qwen API call
        tracing::info!("Calling Qwen API with model: {}", self.model);

        // Mock response
        Ok("这是生成的章节内容。".to_string())
    }
}

/// OpenAI provider (fallback)
#[allow(dead_code)]
pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: "gpt-4o".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn generate(&self, _context: &str, _prompt: &str) -> Result<String> {
        // TODO: Implement actual OpenAI API call
        tracing::info!("Calling OpenAI API with model: {}", self.model);

        // Mock response
        Ok("This is generated content.".to_string())
    }
}

/// Create LLM client from config
pub fn create_client(provider: &str, api_key: &str) -> LlmClient {
    match provider {
        "qwen" => LlmClient::new(Box::new(QwenProvider::new(api_key.to_string()))),
        "openai" => LlmClient::new(Box::new(OpenAiProvider::new(api_key.to_string()))),
        _ => LlmClient::new(Box::new(QwenProvider::new(api_key.to_string()))),
    }
}
