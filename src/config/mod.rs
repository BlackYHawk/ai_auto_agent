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
    /// Provider: "qwen", "openai", "minimax"
    pub provider: String,

    /// API key
    pub api_key: String,

    /// Base URL for API (optional)
    pub base_url: Option<String>,

    /// Model name (optional, provider-specific)
    /// - qwen: qwen-turbo, qwen-plus, qwen-max
    /// - minimax: abab6.5s-chat, abab6.5g-chat
    /// - openai: gpt-4o, gpt-4o-mini, gpt-3.5-turbo
    pub model: Option<String>,

    /// Group ID (required for MiniMax)
    pub group_id: Option<String>,

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
                group_id: None,
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
/// Priority: local config > default config file > defaults
pub fn load_config(path: &Path) -> Result<Config> {
    // Try to load from local config first (config.local.toml)
    let local_config_path = Path::new("config.local.toml");

    let mut final_config = if local_config_path.exists() {
        tracing::info!("Loading config from local file: {:?}", local_config_path);
        let content = fs::read_to_string(local_config_path)
            .with_context(|| format!("Failed to read local config from {:?}", local_config_path))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse local config")?;
        Some(config)
    } else {
        None
    };

    // If no local config, try the specified path
    if final_config.is_none() && path.exists() {
        tracing::info!("Loading config from file: {:?}", path);
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config from {:?}", path))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse config")?;
        final_config = Some(config);
    }

    // If still no config, use defaults
    if final_config.is_none() {
        tracing::warn!("Config file not found, using defaults");
        return Ok(Config::default());
    }

    // Merge with environment variables (for sensitive data)
    let mut config = final_config.unwrap();

    // Override sensitive values from environment variables if set
    if let Ok(api_key) = std::env::var("LLM_API_KEY") {
        if !api_key.is_empty() {
            config.llm.api_key = api_key;
        }
    }

    if let Ok(fanqie_cookie) = std::env::var("FANQIE_COOKIE") {
        if !fanqie_cookie.is_empty() {
            if config.fanqie.is_none() {
                config.fanqie = Some(FanqieConfig {
                    enabled: true,
                    cookie: Some(fanqie_cookie),
                    username: None,
                });
            } else if let Some(ref mut fc) = config.fanqie {
                fc.cookie = Some(fanqie_cookie);
            }
        }
    }

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
