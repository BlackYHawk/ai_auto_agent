//! Configuration module

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// LLM provider configuration
    pub llm: LlmConfig,

    /// Fanqie platform configuration
    pub fanqie: Option<FanqieConfig>,

    /// Storage configuration
    pub storage: StorageConfig,

    /// Generation settings
    pub generation: GenerationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Provider: "qwen", "openai", "claude"
    pub provider: String,

    /// API key
    pub api_key: String,

    /// Base URL for API (optional)
    pub base_url: Option<String>,

    /// Default model
    pub model: Option<String>,

    /// Temperature for generation
    #[serde(default = "default_temperature")]
    pub temperature: f32,

    /// Max tokens per request
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_temperature() -> f32 {
    0.8
}

fn default_max_tokens() -> u32 {
    4096
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanqieConfig {
    /// Enable Fanqie integration
    pub enabled: bool,

    /// Session cookie
    pub cookie: Option<String>,

    /// Username
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage path
    #[serde(default = "default_storage_path")]
    pub path: String,
}

fn default_storage_path() -> String {
    "./data".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    /// Default word count target
    #[serde(default = "default_word_count")]
    pub default_word_count: u64,

    /// Target words per chapter
    #[serde(default = "default_words_per_chapter")]
    pub words_per_chapter: u32,

    /// Chapters per batch generation
    #[serde(default = "default_batch_size")]
    pub batch_size: u32,
}

fn default_word_count() -> u64 {
    1_000_000
}

fn default_words_per_chapter() -> u32 {
    10_000
}

fn default_batch_size() -> u32 {
    10
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm: LlmConfig {
                provider: "qwen".to_string(),
                api_key: std::env::var("LLM_API_KEY").unwrap_or_default(),
                base_url: None,
                model: None,
                temperature: 0.8,
                max_tokens: 4096,
            },
            fanqie: None,
            storage: StorageConfig {
                path: "./data".to_string(),
            },
            generation: GenerationConfig {
                default_word_count: 1_000_000,
                words_per_chapter: 10_000,
                batch_size: 10,
            },
        }
    }
}

/// Load configuration from file
pub fn load_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        tracing::warn!("Config file not found, using defaults: {:?}", path);
        return Ok(Config::default());
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {:?}", path))?;

    let config: Config = toml::from_str(&content)
        .with_context(|| "Failed to parse config")?;

    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config, path: &Path) -> Result<()> {
    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;

    fs::write(path, content)
        .with_context(|| format!("Failed to write config to {:?}", path))?;

    Ok(())
}
