//! Feasibility Analysis Service

use anyhow::Result;
use crate::models::{FeasibilityReport, NovelGenre, CompetitiveWork};
use crate::services::scraping::ScrapingService;

/// Feasibility analysis service
pub struct FeasibilityService {
    scraping: ScrapingService,
}

impl FeasibilityService {
    /// Create a new feasibility service
    pub fn new() -> Self {
        Self {
            scraping: ScrapingService::new(),
        }
    }

    /// Analyze a genre
    pub async fn analyze(&self, genre: NovelGenre) -> Result<FeasibilityReport> {
        tracing::info!("Analyzing genre: {:?}", genre);

        let genre_str = genre.to_string();

        // Scrape popular works from Fanqie
        let works = self.scraping.scrape_genre_rankings(&genre_str).await?;

        // Calculate statistics
        let total_works = works.len() * 1000; // Estimate
        let avg_views = works.iter().map(|w| w.views).sum::<u64>() / works.len().max(1) as u64;
        let avg_favorites = works.iter().map(|w| w.favorites).sum::<u64>() / works.len().max(1) as u64;

        // Convert to competitive works
        let top_works: Vec<CompetitiveWork> = works.iter().map(|w| CompetitiveWork {
            title: w.title.clone(),
            author: w.author.clone(),
            views: w.views,
            favorites: w.favorites,
            rating: w.rating,
            unique_elements: vec![],
            tags: vec![],
        }).collect();

        // Calculate market viability score
        let market_score = self.calculate_market_score(avg_views, avg_favorites);
        let differentiation = self.calculate_differentiation(&top_works);

        let mut report = FeasibilityReport::new(
            uuid::Uuid::new_v4(),
            genre,
        );

        report.total_works_in_genre = total_works as u32;
        report.average_views_top100 = avg_views;
        report.average_favorites_top100 = avg_favorites;
        report.trend_score = 0.3; // Default trend

        report.scores.market_viability = market_score;
        report.scores.competition_level = if avg_views > 1_500_000 {
            crate::models::CompetitionLevel::High
        } else if avg_views > 800_000 {
            crate::models::CompetitionLevel::Medium
        } else {
            crate::models::CompetitionLevel::Low
        };
        report.scores.differentiation_potential = differentiation;

        report.top_works = top_works;
        report.market_gaps = vec!["系统流".to_string(), "凡人流".to_string()]; // Mock gaps
        report.suggested_angles = vec![
            format!("创新{}流派", genre_str),
            format!("融合{}与系统", genre_str),
        ];

        report.calculate_recommendation();

        Ok(report)
    }

    /// Calculate market score based on views and favorites
    fn calculate_market_score(&self, avg_views: u64, avg_favorites: u64) -> u32 {
        let view_score = if avg_views > 1_000_000 { 80 }
        else if avg_views > 500_000 { 60 }
        else { 40 };

        let favorite_score = if avg_favorites > 50_000 { 20 }
        else if avg_favorites > 20_000 { 15 }
        else { 10 };

        (view_score + favorite_score).min(100)
    }

    /// Calculate differentiation potential
    fn calculate_differentiation(&self, works: &[CompetitiveWork]) -> u32 {
        // Simple logic: fewer top works = more differentiation opportunity
        if works.len() < 5 {
            80
        } else if works.len() < 10 {
            60
        } else {
            40
        }
    }
}

impl Default for FeasibilityService {
    fn default() -> Self {
        Self::new()
    }
}
