//! LLM Client Module

use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// LLM client trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate(&self, context: &str, prompt: &str) -> Result<String>;
    fn name(&self) -> &str;
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

    pub fn name(&self) -> &str {
        self.provider.name()
    }
}

// ============ Qwen Provider ============

#[derive(Serialize)]
struct QwenRequest {
    model: String,
    input: QwenInput,
    parameters: QwenParameters,
}

#[derive(Serialize)]
struct QwenInput {
    messages: Vec<QwenMessage>,
}

#[derive(Serialize, Deserialize)]
struct QwenMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct QwenParameters {
    result_format: String,
}

#[derive(Deserialize)]
struct QwenResponse {
    output: QwenOutput,
}

#[derive(Deserialize)]
struct QwenOutput {
    choices: Vec<QwenChoice>,
}

#[derive(Deserialize)]
struct QwenChoice {
    message: QwenMessage,
}

/// Qwen provider
pub struct QwenProvider {
    client: Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl QwenProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "qwen-turbo".to_string()),
            base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for QwenProvider {
    async fn generate(&self, context: &str, prompt: &str) -> Result<String> {
        tracing::info!("Calling Qwen API with model: {}", self.model);

        let url = format!("{}/services/aigc/text-generation/generation", self.base_url);

        let messages = vec![
            QwenMessage {
                role: "system".to_string(),
                content: format!("上下文背景:\n{}\n\n请根据以上上下文生成内容。", context),
            },
            QwenMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = QwenRequest {
            model: self.model.clone(),
            input: QwenInput { messages },
            parameters: QwenParameters {
                result_format: "message".to_string(),
            },
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to call Qwen API")?;

        let api_response: QwenResponse = response.json().await?;

        Ok(api_response.output.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "生成失败".to_string()))
    }

    fn name(&self) -> &str {
        "qwen"
    }
}

// ============ MiniMax Provider ============

#[derive(Serialize)]
struct MiniMaxRequest {
    model: String,
    messages: Vec<MiniMaxMessage>,
    #[serde(rename = "max_tokens")]
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct MiniMaxMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct MiniMaxResponse {
    base_resp: MiniMaxBaseResp,
    choices: Option<Vec<MiniMaxChoice>>,
}

#[derive(Deserialize)]
struct MiniMaxBaseResp {
    status_code: i32,
    status_msg: String,
}

#[derive(Deserialize)]
struct MiniMaxChoice {
    message: MiniMaxMessage,
    #[serde(rename = "finish_reason")]
    #[allow(dead_code)]
    finish_reason: String,
}

/// MiniMax provider
pub struct MiniMaxProvider {
    client: Client,
    api_key: String,
    model: String,
    group_id: String,
}

impl MiniMaxProvider {
    pub fn new(api_key: String, model: Option<String>, group_id: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "abab6.5s-chat".to_string()), // MiniMax 2.5
            group_id: group_id.unwrap_or_default(),
        }
    }
}

#[async_trait]
impl LlmProvider for MiniMaxProvider {
    async fn generate(&self, context: &str, prompt: &str) -> Result<String> {
        tracing::info!("Calling MiniMax API with model: {}", self.model);

        let url = format!(
            "https://api.minimax.chat/v1/text/chatcompletion_v2?GroupId={}",
            self.group_id
        );

        let system_prompt = format!("你是小说作家，根据以下上下文背景创作小说内容。\n\n上下文背景:\n{}", context);

        let messages = vec![
            MiniMaxMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            MiniMaxMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = MiniMaxRequest {
            model: self.model.clone(),
            messages,
            max_tokens: Some(4096),
            temperature: Some(0.7),
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to call MiniMax API")?;

        let api_response: MiniMaxResponse = response.json().await
            .context("Failed to parse MiniMax response")?;

        if api_response.base_resp.status_code != 0 {
            anyhow::bail!("MiniMax API error: {}", api_response.base_resp.status_msg);
        }

        Ok(api_response.choices
            .and_then(|c| c.first().map(|choice| choice.message.content.clone()))
            .unwrap_or_else(|| "生成失败".to_string()))
    }

    fn name(&self) -> &str {
        "minimax"
    }
}

// ============ OpenAI Provider ============

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

/// OpenAI provider
pub struct OpenAiProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: model.unwrap_or_else(|| "gpt-4o".to_string()),
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    async fn generate(&self, context: &str, prompt: &str) -> Result<String> {
        tracing::info!("Calling OpenAI API with model: {}", self.model);

        let url = "https://api.openai.com/v1/chat/completions";

        let messages = vec![
            OpenAIMessage {
                role: "system".to_string(),
                content: format!("上下文背景:\n{}\n\n请根据以上上下文生成内容。", context),
            },
            OpenAIMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ];

        let request = OpenAIRequest {
            model: self.model.clone(),
            messages,
            temperature: 0.7,
        };

        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to call OpenAI API")?;

        let api_response: OpenAIResponse = response.json().await?;

        Ok(api_response.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "生成失败".to_string()))
    }

    fn name(&self) -> &str {
        "openai"
    }
}

/// Available LLM providers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlmProviderType {
    Qwen,
    MiniMax,
    OpenAI,
}

impl LlmProviderType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "qwen" | "tongyi" | "aliyun" => LlmProviderType::Qwen,
            "minimax" => LlmProviderType::MiniMax,
            "openai" | "gpt" => LlmProviderType::OpenAI,
            _ => LlmProviderType::Qwen,
        }
    }
}

/// Create LLM client from config
pub fn create_client(provider: &str, api_key: &str) -> LlmClient {
    match LlmProviderType::from_str(provider) {
        LlmProviderType::MiniMax => {
            LlmClient::new(Box::new(MiniMaxProvider::new(api_key.to_string(), None, None)))
        }
        LlmProviderType::OpenAI => {
            LlmClient::new(Box::new(OpenAiProvider::new(api_key.to_string(), None)))
        }
        LlmProviderType::Qwen => {
            LlmClient::new(Box::new(QwenProvider::new(api_key.to_string(), None)))
        }
    }
}

/// Create LLM client with full config
pub fn create_client_with_config(
    provider: &str,
    api_key: &str,
    model: Option<String>,
    group_id: Option<String>,
) -> LlmClient {
    match LlmProviderType::from_str(provider) {
        LlmProviderType::MiniMax => {
            LlmClient::new(Box::new(MiniMaxProvider::new(api_key.to_string(), model, group_id)))
        }
        LlmProviderType::OpenAI => {
            LlmClient::new(Box::new(OpenAiProvider::new(api_key.to_string(), model)))
        }
        LlmProviderType::Qwen => {
            LlmClient::new(Box::new(QwenProvider::new(api_key.to_string(), model)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_from_str() {
        assert_eq!(LlmProviderType::from_str("minimax"), LlmProviderType::MiniMax);
        assert_eq!(LlmProviderType::from_str("MiniMax"), LlmProviderType::MiniMax);
        assert_eq!(LlmProviderType::from_str("qwen"), LlmProviderType::Qwen);
        assert_eq!(LlmProviderType::from_str("openai"), LlmProviderType::OpenAI);
        assert_eq!(LlmProviderType::from_str("unknown"), LlmProviderType::Qwen);
    }
}
