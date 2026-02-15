//! Chapter Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Chapter status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChapterStatus {
    Draft,
    Review,
    Approved,
    Published,
}

/// A generated chapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedChapter {
    /// Unique identifier
    pub id: Uuid,

    /// Project ID
    pub project_id: Uuid,

    /// Chapter number
    pub chapter_number: u32,

    /// Title
    pub title: String,

    /// Content
    pub content: String,

    /// Word count
    pub word_count: u32,

    /// Generation parameters
    pub generation_params: GenerationParams,

    /// Status
    pub status: ChapterStatus,

    /// Fanqie chapter ID (if published)
    pub fanqie_chapter_id: Option<String>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParams {
    /// Model used
    pub model: String,

    /// Temperature setting
    pub temperature: f32,

    /// Max tokens
    pub max_tokens: u32,
}

impl GeneratedChapter {
    /// Create a new chapter
    pub fn new(
        project_id: Uuid,
        chapter_number: u32,
        title: String,
        content: String,
        params: GenerationParams,
    ) -> Self {
        let word_count = content.chars().count() as u32;
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            project_id,
            chapter_number,
            title,
            content,
            word_count,
            generation_params: params,
            status: ChapterStatus::Draft,
            fanqie_chapter_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Approve chapter
    pub fn approve(&mut self) {
        self.status = ChapterStatus::Approved;
        self.updated_at = Utc::now();
    }
}

/// Chapter plan summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterSummary {
    /// Chapter number
    pub number: u32,

    /// Title
    pub title: String,

    /// Summary
    pub summary: String,

    /// Key events
    pub key_events: Vec<String>,

    /// Protagonist development
    pub protagonist_development: String,

    /// Estimated word count
    pub word_count_estimate: u32,

    /// Is plot twist chapter (every 10th)
    pub is_plot_twist_chapter: bool,

    /// Plot twist description (if applicable)
    pub plot_twist_description: Option<String>,
}

/// Chapter plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterPlan {
    /// Unique identifier
    pub id: Uuid,

    /// Project ID
    pub project_id: Uuid,

    /// Total chapters
    pub total_chapters: u32,

    /// Chapter summaries
    pub chapters: Vec<ChapterSummary>,

    /// Plot twist positions
    pub plot_twist_positions: Vec<u32>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl ChapterPlan {
    /// Create a new chapter plan
    pub fn new(project_id: Uuid, total_chapters: u32) -> Self {
        let now = Utc::now();

        // Calculate plot twist positions (every 10th chapter)
        let plot_twist_positions: Vec<u32> = (10..=total_chapters)
            .step_by(10)
            .collect();

        Self {
            id: Uuid::new_v4(),
            project_id,
            total_chapters,
            chapters: Vec::new(),
            plot_twist_positions,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter_creation() {
        let project_id = Uuid::new_v4();
        let params = GenerationParams {
            model: "qwen2.5".to_string(),
            temperature: 0.8,
            max_tokens: 4096,
        };

        let chapter = GeneratedChapter::new(
            project_id,
            1,
            "第一章".to_string(),
            "这是章节内容".to_string(),
            params,
        );

        assert_eq!(chapter.chapter_number, 1);
        assert_eq!(chapter.status, ChapterStatus::Draft);
    }

    #[test]
    fn test_plot_twist_positions() {
        let plan = ChapterPlan::new(Uuid::new_v4(), 50);

        assert_eq!(plan.plot_twist_positions, vec![10, 20, 30, 40, 50]);
    }
}

/// Consistency issue type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyIssue {
    pub issue_type: ConsistencyIssueType,
    pub description: String,
    pub chapter_reference: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConsistencyIssueType {
    Character,
    Plot,
    Setting,
    Timeline,
}
