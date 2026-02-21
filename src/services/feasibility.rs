//! Feasibility Analysis Service

use anyhow::Result;
use crate::models::{CompetitionLevel, DataSource, FeasibilityReport, NovelGenre, CompetitiveWork};
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

    /// Analyze a genre with real data from Fanqie
    pub async fn analyze(&self, genre: NovelGenre) -> Result<FeasibilityReport> {
        tracing::info!("Analyzing genre: {:?}", genre);

        let genre_str = genre.to_string();

        // Get market data with caching (real data from Fanqie)
        let (market_data, data_source) = self.scraping.get_market_data(&genre_str).await;

        let mut report = FeasibilityReport::new(
            uuid::Uuid::new_v4(),
            genre,
        );

        // Set data source
        report.data_source = data_source;

        // Process market data if available
        if let Some(data) = market_data {
            report.market_data = Some(data.clone());
            report.total_works_in_genre = data.total_books;

            // Calculate averages
            let total_views: u64 = data.hot_books.iter().map(|b| b.word_count).sum();
            let total_likes: u64 = data.hot_books.iter().map(|b| b.likes).sum();
            report.average_views_top100 = total_views / data.hot_books.len().max(1) as u64;
            report.average_favorites_top100 = total_likes / data.hot_books.len().max(1) as u64;

            // Convert to competitive works
            report.top_works = data.hot_books.iter().map(|b| CompetitiveWork {
                title: b.title.clone(),
                author: b.author.clone(),
                views: b.word_count,
                favorites: b.likes,
                rating: b.rating.unwrap_or(4.5),
                unique_elements: vec![],
                tags: data.tags.clone(),
            }).collect();

            // Calculate market score
            let market_score = self.calculate_market_score(report.average_views_top100, report.average_favorites_top100);
            report.scores.market_viability = market_score;

            // Competition level based on average views
            report.scores.competition_level = if report.average_views_top100 > 1_500_000 {
                CompetitionLevel::High
            } else if report.average_views_top100 > 800_000 {
                CompetitionLevel::Medium
            } else {
                CompetitionLevel::Low
            };

            report.scores.differentiation_potential = self.calculate_differentiation(&report.top_works);

            // Generate market gaps and suggestions based on tags
            report.market_gaps = self.generate_market_gaps(&data.tags);
            report.suggested_angles = self.generate_suggestions(&genre_str, &data.tags);
        } else {
            // Fallback when no data available
            report.total_works_in_genre = 10000; // Estimate
            report.scores.market_viability = 50;
            report.scores.competition_level = CompetitionLevel::Medium;
            report.scores.differentiation_potential = 50;
            report.market_gaps = vec!["数据获取失败，使用估算值".to_string()];
            report.suggested_angles = vec![
                format!("建议稍后重新分析{}类型", genre_str),
            ];
        }

        report.trend_score = 0.3;
        report.calculate_recommendation();

        Ok(report)
    }

    /// Generate market gaps based on popular tags
    fn generate_market_gaps(&self, tags: &[String]) -> Vec<String> {
        let saturated_tags = ["穿越", "重生", "系统", "都市", "修仙", "玄幻"];
        let gaps: Vec<String> = tags.iter()
            .filter(|t| !saturated_tags.contains(&t.as_str()))
            .take(3)
            .cloned()
            .collect();

        if gaps.is_empty() {
            vec!["创新元素".to_string(), "细分题材".to_string()]
        } else {
            gaps
        }
    }

    /// Generate genre-specific suggestions
    fn generate_suggestions(&self, genre: &str, tags: &[String]) -> Vec<String> {
        let mut suggestions = vec![
            format!("创新{}流派", genre),
            format!("融合{}与热门元素", genre),
        ];

        // Add tag-based suggestions
        if tags.contains(&"系统".to_string()) {
            suggestions.push(format!("在{}中加入系统元素", genre));
        }
        if tags.contains(&"穿越".to_string()) {
            suggestions.push(format!("尝试{}魂穿设定", genre));
        }

        suggestions
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
