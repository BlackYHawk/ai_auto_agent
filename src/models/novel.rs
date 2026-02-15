//! Novel Project Model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Genre of the novel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NovelGenre {
    Fantasy,
    Urban,
    Xianxia,
    Historical,
    Romance,
    Scifi,
    Game,
    Horror,
    Other,
}

impl std::fmt::Display for NovelGenre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NovelGenre::Fantasy => write!(f, "fantasy"),
            NovelGenre::Urban => write!(f, "urban"),
            NovelGenre::Xianxia => write!(f, "xianxia"),
            NovelGenre::Historical => write!(f, "historical"),
            NovelGenre::Romance => write!(f, "romance"),
            NovelGenre::Scifi => write!(f, "scifi"),
            NovelGenre::Game => write!(f, "game"),
            NovelGenre::Horror => write!(f, "horror"),
            NovelGenre::Other => write!(f, "other"),
        }
    }
}

/// Project status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Draft,
    Feasibility,
    Outline,
    Planning,
    Generating,
    Publishing,
    Completed,
}

/// Publication status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PublicationStatus {
    NotPublished,
    Created,
    Publishing,
    Published,
}

/// Novel project entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelProject {
    /// Unique identifier
    pub id: Uuid,

    /// Novel title
    pub name: String,

    /// Genre
    pub genre: NovelGenre,

    /// Current status
    pub status: ProjectStatus,

    /// Target word count
    pub target_word_count: u64,

    /// Fanqie novel ID (if published)
    pub fanqie_novel_id: Option<String>,

    /// Publication status
    pub publication_status: PublicationStatus,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl NovelProject {
    /// Create a new project
    pub fn new(name: String, genre: NovelGenre, target_word_count: u64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            genre,
            status: ProjectStatus::Draft,
            target_word_count,
            fanqie_novel_id: None,
            publication_status: PublicationStatus::NotPublished,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update status
    pub fn set_status(&mut self, status: ProjectStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_project() {
        let project = NovelProject::new(
            "Test Novel".to_string(),
            NovelGenre::Fantasy,
            1_000_000,
        );

        assert_eq!(project.name, "Test Novel");
        assert_eq!(project.genre, NovelGenre::Fantasy);
        assert_eq!(project.status, ProjectStatus::Draft);
    }
}
