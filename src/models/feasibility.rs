//! Feasibility Report Models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::NovelGenre;

/// Data source for feasibility analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataSource {
    FanqieLive,    // Real-time data from Fanqie website
    FanqieCache,  // Cached data from previous scrape
    Fallback,     // Fallback/mock data when no data available
}

/// Competition level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompetitionLevel {
    Low,
    Medium,
    High,
}

/// Recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Recommendation {
    Proceed,
    Revise,
    Reject,
}

/// Market data from scraping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    /// Total books in genre
    pub total_books: u32,
    /// Hot books in genre
    pub hot_books: Vec<HotBook>,
    /// Average word count
    pub average_word_count: u64,
    /// Popular tags
    pub tags: Vec<String>,
}

/// Hot book entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotBook {
    /// Book title
    pub title: String,
    /// Author
    pub author: String,
    /// Word count
    pub word_count: u64,
    /// Likes/favorites
    pub likes: u64,
    /// Rating
    pub rating: Option<f32>,
}

/// A competitive work
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitiveWork {
    /// Title
    pub title: String,

    /// Author
    pub author: String,

    /// Views
    pub views: u64,

    /// Favorites
    pub favorites: u64,

    /// Rating
    pub rating: f32,

    /// Unique elements
    pub unique_elements: Vec<String>,

    /// Tags
    pub tags: Vec<String>,
}

/// Feasibility scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeasibilityScores {
    /// Market viability (0-100)
    pub market_viability: u32,

    /// Competition level
    pub competition_level: CompetitionLevel,

    /// Differentiation potential (0-100)
    pub differentiation_potential: u32,
}

/// Feasibility report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeasibilityReport {
    /// Unique identifier
    pub id: Uuid,

    /// Project ID
    pub project_id: Uuid,

    /// Genre analyzed
    pub genre: NovelGenre,

    /// Market data from scraping
    pub market_data: Option<MarketData>,

    /// Data source indicator
    pub data_source: DataSource,

    /// Total works in genre
    pub total_works_in_genre: u32,

    /// Average views top 100
    pub average_views_top100: u64,

    /// Average favorites top 100
    pub average_favorites_top100: u64,

    /// Trend score (-1 to 1)
    pub trend_score: f32,

    /// Top works
    pub top_works: Vec<CompetitiveWork>,

    /// Market gaps
    pub market_gaps: Vec<String>,

    /// Scores
    pub scores: FeasibilityScores,

    /// Recommendation
    pub recommendation: Recommendation,

    /// Suggested angles
    pub suggested_angles: Vec<String>,

    /// Generated timestamp
    pub generated_at: DateTime<Utc>,
}

impl FeasibilityReport {
    /// Create a new feasibility report
    pub fn new(project_id: Uuid, genre: NovelGenre) -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id,
            genre,
            market_data: None,
            data_source: DataSource::Fallback,
            total_works_in_genre: 0,
            average_views_top100: 0,
            average_favorites_top100: 0,
            trend_score: 0.0,
            top_works: Vec::new(),
            market_gaps: Vec::new(),
            scores: FeasibilityScores {
                market_viability: 0,
                competition_level: CompetitionLevel::Medium,
                differentiation_potential: 0,
            },
            recommendation: Recommendation::Revise,
            suggested_angles: Vec::new(),
            generated_at: Utc::now(),
        }
    }

    /// Calculate recommendation based on scores
    pub fn calculate_recommendation(&mut self) {
        let score = self.scores.market_viability;

        if score >= 70 {
            self.recommendation = Recommendation::Proceed;
        } else if score >= 50 {
            self.recommendation = Recommendation::Revise;
        } else {
            self.recommendation = Recommendation::Reject;
        }
    }
}

/// Novel score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelScore {
    /// Total score (0-100)
    pub total_score: u32,

    /// Market score
    pub market_score: u32,

    /// Content score
    pub content_score: u32,

    /// Feasibility score
    pub feasibility_score: u32,

    /// Recommendation
    pub recommendation: Recommendation,

    /// Suggested improvements
    pub suggested_improvements: Vec<String>,
}

impl NovelScore {
    /// Create and calculate score
    pub fn calculate(
        market: u32,
        content: u32,
        feasibility: u32,
    ) -> Self {
        // Weighted calculation: Market 40%, Content 35%, Feasibility 25%
        let total = ((market as f32 * 0.4) + (content as f32 * 0.35) + (feasibility as f32 * 0.25)) as u32;

        let recommendation = if total >= 70 {
            Recommendation::Proceed
        } else if total >= 50 {
            Recommendation::Revise
        } else {
            Recommendation::Reject
        };

        Self {
            total_score: total,
            market_score: market,
            content_score: content,
            feasibility_score: feasibility,
            recommendation,
            suggested_improvements: Vec::new(),
        }
    }
}
