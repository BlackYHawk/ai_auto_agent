//! Novel Outline Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Outline status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutlineStatus {
    Draft,
    Approved,
    Locked,
}

/// World type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorldType {
    Modern,
    Fantasy,
    Scifi,
    Historical,
    Xianxia,
}

/// A location in the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    /// Name
    pub name: String,

    /// Description
    pub description: String,

    /// Importance
    pub importance: LocationImportance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationImportance {
    Major,
    Minor,
}

/// World settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSettings {
    /// World name
    pub name: String,

    /// World type
    pub world_type: WorldType,

    /// Description
    pub description: String,

    /// Rules (magic system, technology level, etc.)
    pub rules: Vec<String>,

    /// Locations
    pub locations: Vec<Location>,
}

/// A character moment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterMoment {
    /// Chapter number
    pub chapter: u32,

    /// Description
    pub description: String,

    /// Development
    pub development: String,
}

/// Character arc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArc {
    /// Unique identifier
    pub id: Uuid,

    /// Character name
    pub name: String,

    /// Role
    pub role: CharacterRole,

    /// Description
    pub description: String,

    /// Personality traits
    pub personality_traits: Vec<String>,

    /// Arc description
    pub arc_description: String,

    /// Key moments
    pub key_moments: Vec<CharacterMoment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CharacterRole {
    Protagonist,
    Supporting,
    Antagonist,
}

/// Plot arc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotArc {
    /// Unique identifier
    pub id: Uuid,

    /// Arc name
    pub name: String,

    /// Start chapter
    pub start_chapter: u32,

    /// End chapter
    pub end_chapter: u32,

    /// Summary
    pub summary: String,

    /// Key events
    pub key_events: Vec<String>,

    /// Climax
    pub climax: String,
}

/// Sensitive content issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveContentIssue {
    /// Category
    pub category: SensitiveCategory,

    /// Description
    pub description: String,

    /// Severity
    pub severity: SensitiveSeverity,

    /// Suggestion
    pub suggestion: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SensitiveCategory {
    Violence,
    Explicit,
    Political,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SensitiveSeverity {
    Low,
    Medium,
    High,
}

/// Sensitive content check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveContentResult {
    /// Passed check
    pub passed: bool,

    /// Issues found
    pub issues: Vec<SensitiveContentIssue>,
}

/// Novel outline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelOutline {
    /// Unique identifier
    pub id: Uuid,

    /// Project ID
    pub project_id: Uuid,

    /// Premise (1-2 sentence hook)
    pub premise: String,

    /// Theme
    pub theme: String,

    /// Target word count
    pub target_word_count: u64,

    /// Plot arcs
    pub arcs: Vec<PlotArc>,

    /// Protagonist
    pub protagonist: CharacterArc,

    /// Supporting characters
    pub supporting_characters: Vec<CharacterArc>,

    /// World settings
    pub world_settings: WorldSettings,

    /// Sensitive content check
    pub sensitive_content_check: SensitiveContentResult,

    /// Status
    pub status: OutlineStatus,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

impl NovelOutline {
    /// Create a new outline
    pub fn new(
        project_id: Uuid,
        premise: String,
        theme: String,
        target_word_count: u64,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            project_id,
            premise,
            theme,
            target_word_count,
            arcs: Vec::new(),
            protagonist: CharacterArc {
                id: Uuid::new_v4(),
                name: String::new(),
                role: CharacterRole::Protagonist,
                description: String::new(),
                personality_traits: Vec::new(),
                arc_description: String::new(),
                key_moments: Vec::new(),
            },
            supporting_characters: Vec::new(),
            world_settings: WorldSettings {
                name: String::new(),
                world_type: WorldType::Modern,
                description: String::new(),
                rules: Vec::new(),
                locations: Vec::new(),
            },
            sensitive_content_check: SensitiveContentResult {
                passed: true,
                issues: Vec::new(),
            },
            status: OutlineStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }

    /// Approve outline
    pub fn approve(&mut self) {
        self.status = OutlineStatus::Approved;
        self.updated_at = Utc::now();
    }

    /// Lock outline
    pub fn lock(&mut self) {
        self.status = OutlineStatus::Locked;
        self.updated_at = Utc::now();
    }
}
