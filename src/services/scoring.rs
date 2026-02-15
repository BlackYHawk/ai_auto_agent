//! Scoring Service

use crate::models::{NovelScore, NovelGenre};

/// Scoring service for evaluating novel concepts
pub struct ScoringService;

impl ScoringService {
    /// Calculate market score
    pub fn calculate_market_score(&self, genre: NovelGenre, _trend_data: &impl TrendData) -> u32 {
        // Mock implementation
        match genre {
            NovelGenre::Fantasy => 80,
            NovelGenre::Urban => 85,
            NovelGenre::Xianxia => 75,
            NovelGenre::Romance => 90,
            _ => 70,
        }
    }

    /// Calculate content score
    pub fn calculate_content_score(&self, premise: &str, originality: u32) -> u32 {
        // Mock implementation
        let base = 60;
        let premise_bonus = if premise.len() > 20 { 10 } else { 0 };
        (base + originality + premise_bonus).min(100)
    }

    /// Calculate feasibility score
    pub fn calculate_feasibility_score(&self, word_count: u64) -> u32 {
        // Feasibility decreases for extremely long novels
        if word_count > 2_000_000 {
            50
        } else if word_count > 1_000_000 {
            70
        } else {
            85
        }
    }

    /// Score a novel concept
    pub fn score_concept(
        &self,
        genre: NovelGenre,
        premise: &str,
        word_count: u64,
        trend_data: &impl TrendData,
    ) -> NovelScore {
        let market = self.calculate_market_score(genre, trend_data);
        let content = self.calculate_content_score(premise, 70); // Default originality
        let feasibility = self.calculate_feasibility_score(word_count);

        NovelScore::calculate(market, content, feasibility)
    }
}

/// Trait for trend data
pub trait TrendData {
    fn trend_score(&self) -> f32;
}

impl TrendData for () {
    fn trend_score(&self) -> f32 {
        0.0
    }
}

impl Default for ScoringService {
    fn default() -> Self {
        Self::new()
    }
}

impl ScoringService {
    pub fn new() -> Self {
        Self
    }
}
