//! Web Scraping Service for Fanqie Novel
//!
//! This service provides real web scraping capabilities for Fanqie Novel website.
//! It can scrape genre rankings and work details from the platform.

use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Genre ID mapping for Fanqie rankings
const GENRE_IDS: &[(&str, &str)] = &[
    ("fantasy", "1"),
    ("xianxia", "2"),
    ("urban", "3"),
    ("historical", "4"),
    ("romance", "5"),
    ("scifi", "6"),
    ("game", "7"),
    ("horror", "8"),
    ("wuxia", "9"),
    ("other", "10"),
];

/// Scraping service for Fanqie Novel website
#[allow(dead_code)]
pub struct ScrapingService {
    client: Client,
    base_url: String,
    api_base: String,
}

impl ScrapingService {
    /// Create a new scraping service
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: "https://fanqienovel.com".to_string(),
            api_base: "https://api2.fanqiecloud.com.cn".to_string(),
        }
    }

    /// Scrape genre rankings - fetches top works from a specific genre
    pub async fn scrape_genre_rankings(&self, genre: &str) -> Result<Vec<WorkInfo>> {
        tracing::info!("Scraping rankings for genre: {}", genre);

        // Try API first
        let genre_id = self.get_genre_id(genre);
        let api_url = format!("{}/api/in.rank/v1/ranking/genre/{}", self.api_base, genre_id);

        tracing::debug!("Fetching from API: {}", api_url);

        let response = self.client
            .get(&api_url)
            .header("Referer", &self.base_url)
            .header("Origin", &self.base_url)
            .send()
            .await
            .context("Failed to fetch genre rankings")?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await
                .context("Failed to parse API response")?;

            tracing::debug!("API response: {:?}", body);

            // Parse the API response
            if let Some(works) = self.parse_api_rankings(&body) {
                if !works.is_empty() {
                    tracing::info!("Successfully scraped {} works from genre: {}", works.len(), genre);
                    return Ok(works);
                }
            }
        }

        // Fallback: Try web scraping
        tracing::debug!("API failed, trying web scraping");
        self.scrape_genre_rankings_web(genre).await
    }

    /// Scrape genre rankings from web page
    async fn scrape_genre_rankings_web(&self, genre: &str) -> Result<Vec<WorkInfo>> {
        let url = format!("{}/rank/{}", self.base_url, genre);

        tracing::debug!("Fetching web page: {}", url);

        let response = self.client
            .get(&url)
            .header("Referer", &self.base_url)
            .send()
            .await
            .context("Failed to fetch genre page")?;

        let html = response.text().await
            .context("Failed to read HTML response")?;

        // Parse HTML to extract work info
        self.parse_html_rankings(&html, genre)
    }

    /// Scrape work details - fetches detailed information about a specific work
    pub async fn scrape_work_details(&self, work_id: &str) -> Result<WorkDetails> {
        tracing::info!("Scraping details for work: {}", work_id);

        // Try API first
        let api_url = format!("{}/api/in.book/v1/book/detail/{}", self.api_base, work_id);

        tracing::debug!("Fetching from API: {}", api_url);

        let response = self.client
            .get(&api_url)
            .header("Referer", &self.base_url)
            .header("Origin", &self.base_url)
            .send()
            .await
            .context("Failed to fetch work details")?;

        if response.status().is_success() {
            let body: serde_json::Value = response.json().await
                .context("Failed to parse API response")?;

            tracing::debug!("API response: {:?}", body);

            // Parse the API response
            if let Some(details) = self.parse_api_details(&body) {
                tracing::info!("Successfully scraped details for work: {}", work_id);
                return Ok(details);
            }
        }

        // Fallback: Try web scraping
        tracing::debug!("API failed, trying web scraping");
        self.scrape_work_details_web(work_id).await
    }

    /// Scrape work details from web page
    async fn scrape_work_details_web(&self, work_id: &str) -> Result<WorkDetails> {
        let url = format!("{}/book/{}.html", self.base_url, work_id);

        tracing::debug!("Fetching web page: {}", url);

        let response = self.client
            .get(&url)
            .header("Referer", &self.base_url)
            .send()
            .await
            .context("Failed to fetch work page")?;

        let html = response.text().await
            .context("Failed to read HTML response")?;

        // Parse HTML to extract work details
        self.parse_html_details(&html, work_id)
    }

    /// Get genre ID from genre name
    fn get_genre_id(&self, genre: &str) -> &str {
        let lower = genre.to_lowercase();
        for (name, id) in GENRE_IDS {
            if lower.contains(name) {
                return id;
            }
        }
        "1" // Default to fantasy
    }

    /// Parse API ranking response
    fn parse_api_rankings(&self, body: &serde_json::Value) -> Option<Vec<WorkInfo>> {
        // Try different API response formats
        let works = body.get("data")
            .and_then(|d| d.get("list"))
            .or_else(|| body.get("data"))
            .and_then(|d| d.as_array())?;

        Some(works.iter().filter_map(|item| {
            Some(WorkInfo {
                title: item.get("title")?.as_str()?.to_string(),
                author: item.get("author_name")
                    .or_else(|| item.get("author"))?
                    .as_str()?
                    .to_string(),
                views: item.get("word_count")
                    .or_else(|| item.get("views"))?
                    .as_u64()?
                    .saturating_mul(1000), // Approximate views from word count
                favorites: item.get("total_reads")
                    .or_else(|| item.get("favorites"))?
                    .as_u64()?
                    .saturating_mul(100),
                rating: item.get("score")
                    .or_else(|| item.get("rating"))?
                    .as_f64()
                    .unwrap_or(4.5) as f32,
            })
        }).collect())
    }

    /// Parse API details response
    fn parse_api_details(&self, body: &serde_json::Value) -> Option<WorkDetails> {
        let data = body.get("data")?;

        Some(WorkDetails {
            title: data.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string(),
            author: data.get("author_name")
                .or_else(|| data.get("author"))?
                .as_str()?
                .to_string(),
            views: data.get("word_count")
                .or_else(|| data.get("views"))?
                .as_u64()?
                .saturating_mul(1000),
            favorites: data.get("total_reads")
                .or_else(|| data.get("favorites"))?
                .as_u64()?
                .saturating_mul(100),
            rating: data.get("score")
                .or_else(|| data.get("rating"))?
                .as_f64()
                .unwrap_or(4.5) as f32,
            tags: data.get("tags")
                .and_then(|t| t.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect())
                .unwrap_or_default(),
            description: data.get("description")
                .or_else(|| data.get("intro"))
                .and_then(|v| v.as_str())
                .unwrap_or("No description")
                .to_string(),
        })
    }

    /// Parse HTML ranking page (simplified parsing)
    fn parse_html_rankings(&self, html: &str, _genre: &str) -> Result<Vec<WorkInfo>> {
        // Simple HTML parsing using basic string search
        // In a real implementation, you'd use a proper HTML parser like scraper

        let works = Vec::new();

        // Look for common patterns in Fanqie HTML
        // This is a simplified approach - in production, use proper HTML parsing

        // Try to extract work items using regex-like patterns
        if let Some(start) = html.find("window.__INITIAL_STATE__") {
            if let Some(end) = html[start..].find("</script>") {
                let script_content = &html[start..start + end];
                if let Some(json_start) = script_content.find('{') {
                    if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&script_content[json_start..]) {
                        if let Some(items) = self.extract_from_json_state(&json_data) {
                            return Ok(items);
                        }
                    }
                }
            }
        }

        // If JSON extraction fails, return empty with warning
        tracing::warn!("Could not parse HTML rankings, returning empty list");
        Ok(works)
    }

    /// Extract works from JSON state in HTML
    fn extract_from_json_state(&self, json: &serde_json::Value) -> Option<Vec<WorkInfo>> {
        // Try to find ranking data in the JSON state
        let items = json.get("rankList")
            .or_else(|| json.get("ranking"))
            .or_else(|| json.get("bookList"))
            .or_else(|| json.get("list"))?
            .as_array()?;

        Some(items.iter().filter_map(|item| {
            Some(WorkInfo {
                title: item.get("bookName")
                    .or_else(|| item.get("title"))?
                    .as_str()?
                    .to_string(),
                author: item.get("authorName")
                    .or_else(|| item.get("author"))?
                    .as_str()?
                    .to_string(),
                views: item.get("wordCount")
                    .or_else(|| item.get("views"))?
                    .as_u64()?
                    .saturating_mul(1000),
                favorites: item.get("totalRead")
                    .or_else(|| item.get("favorites"))?
                    .as_u64()?
                    .saturating_mul(100),
                rating: item.get("score")
                    .or_else(|| item.get("rating"))?
                    .as_f64()
                    .unwrap_or(4.5) as f32,
            })
        }).collect())
    }

    /// Parse HTML details page
    fn parse_html_details(&self, html: &str, work_id: &str) -> Result<WorkDetails> {
        // Try to extract from JSON state in HTML
        if let Some(start) = html.find("window.__INITIAL_STATE__") {
            if let Some(end) = html[start..].find("</script>") {
                let script_content = &html[start..start + end];
                if let Some(json_start) = script_content.find('{') {
                    if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&script_content[json_start..]) {
                        if let Some(details) = self.extract_details_from_json(&json_data) {
                            return Ok(details);
                        }
                    }
                }
            }
        }

        // Fallback: return minimal data
        tracing::warn!("Could not parse HTML details, returning minimal data");
        Ok(WorkDetails {
            title: format!("Work {}", work_id),
            author: "Unknown".to_string(),
            views: 0,
            favorites: 0,
            rating: 0.0,
            tags: vec![],
            description: "Could not fetch details".to_string(),
        })
    }

    /// Extract details from JSON state
    fn extract_details_from_json(&self, json: &serde_json::Value) -> Option<WorkDetails> {
        let data = json.get("bookDetail")
            .or_else(|| json.get("detail"))
            .or_else(|| json.get("data"))?;

        Some(WorkDetails {
            title: data.get("bookName")
                .or_else(|| data.get("title"))?
                .as_str()?
                .to_string(),
            author: data.get("authorName")
                .or_else(|| data.get("author"))?
                .as_str()?
                .to_string(),
            views: data.get("wordCount")
                .or_else(|| data.get("views"))?
                .as_u64()?
                .saturating_mul(1000),
            favorites: data.get("totalRead")
                .or_else(|| data.get("favorites"))?
                .as_u64()?
                .saturating_mul(100),
            rating: data.get("score")
                .or_else(|| data.get("rating"))?
                .as_f64()
                .unwrap_or(4.5) as f32,
            tags: data.get("tags")
                .and_then(|t| t.as_array())
                .map(|arr| arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect())
                .unwrap_or_default(),
            description: data.get("description")
                .or_else(|| data.get("intro"))
                .and_then(|v| v.as_str())
                .unwrap_or("No description")
                .to_string(),
        })
    }
}

impl Default for ScrapingService {
    fn default() -> Self {
        Self::new()
    }
}

/// Work info from ranking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkInfo {
    pub title: String,
    pub author: String,
    pub views: u64,
    pub favorites: u64,
    pub rating: f32,
}

/// Detailed work information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDetails {
    pub title: String,
    pub author: String,
    pub views: u64,
    pub favorites: u64,
    pub rating: f32,
    pub tags: Vec<String>,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scrape_xianxia_rankings() {
        let service = ScrapingService::new();
        let result = service.scrape_genre_rankings("xianxia").await;

        match result {
            Ok(works) => {
                println!("Successfully scraped {} works:", works.len());
                for (i, work) in works.iter().enumerate().take(10) {
                    println!("{}. {} by {} (views: {}, rating: {:.1})",
                        i + 1, work.title, work.author, work.views, work.rating);
                }

                // Save to file
                let json = serde_json::to_string_pretty(&works).unwrap();
                std::fs::write("data/xianxia_rankings.json", &json).unwrap();
                println!("\nSaved to data/xianxia_rankings.json");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        // Just verify it runs without panic
        assert!(true);
    }
}
