//! Fanqie Platform Credentials

use serde::{Deserialize, Serialize};

/// Credentials for Fanqie platform authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanqieCredentials {
    /// Username or phone number
    pub username: String,
    /// Password
    pub password: String,
    /// Optional: cookies for session persistence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<String>,
}

impl FanqieCredentials {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password,
            cookies: None,
        }
    }
}

/// Fanqie novel metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanqieNovel {
    /// Fanqie novel ID
    pub novel_id: String,
    /// Novel title
    pub title: String,
    /// Genre/category
    pub genre: String,
    /// Description
    pub description: String,
    /// Status (draft, published, etc.)
    pub status: String,
}

/// Fanqie chapter metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanqieChapter {
    /// Fanqie chapter ID
    pub chapter_id: String,
    /// Chapter number
    pub chapter_number: u32,
    /// Chapter title
    pub title: String,
    /// Word count
    pub word_count: u32,
    /// Status (draft, published, etc.)
    pub status: String,
}
