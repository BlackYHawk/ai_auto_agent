//! Feasibility Command

use anyhow::Result;
use uuid::Uuid;
use crate::models::NovelGenre;
use crate::services::{FeasibilityService, StorageService};

/// Run feasibility analysis for a project
pub async fn run(project_id: &str, genre_str: &str) -> Result<()> {
    tracing::info!("Running feasibility analysis for project: {}", project_id);

    // Parse genre
    let genre = match genre_str.to_lowercase().as_str() {
        "fantasy" | "玄幻" => NovelGenre::Fantasy,
        "urban" | "都市" => NovelGenre::Urban,
        "xianxia" | "仙侠" => NovelGenre::Xianxia,
        "historical" | "历史" => NovelGenre::Historical,
        "romance" | "言情" => NovelGenre::Romance,
        "scifi" | "科幻" => NovelGenre::Scifi,
        "game" | "游戏" => NovelGenre::Game,
        "horror" | "悬疑" => NovelGenre::Horror,
        _ => NovelGenre::Other,
    };

    // Run analysis
    let service = FeasibilityService::new();
    let mut report = service.analyze(genre).await?;

    // Set project ID
    let project_uuid = Uuid::parse_str(project_id)?;
    report.project_id = project_uuid;

    // Save report to project directory
    let storage = StorageService::new_project(".", project_uuid)?;
    storage.save(&report)?;

    // Display results
    println!("\n=== Feasibility Analysis Report ===");
    println!("Genre: {:?}", report.genre);
    println!("Total Works: {}", report.total_works_in_genre);
    println!("Average Views (Top 100): {}", report.average_views_top100);
    println!("Average Favorites: {}", report.average_favorites_top100);
    println!("\n=== Scores ===");
    println!("Market Viability: {}/100", report.scores.market_viability);
    println!("Competition Level: {:?}", report.scores.competition_level);
    println!("Differentiation Potential: {}/100", report.scores.differentiation_potential);
    println!("\n=== Recommendation ===");
    println!("{:?}", report.recommendation);

    if !report.suggested_angles.is_empty() {
        println!("\n=== Suggested Angles ===");
        for angle in &report.suggested_angles {
            println!("- {}", angle);
        }
    }

    println!("\nSaved to: projects/{}/analysis/feasibility.json", project_id);

    Ok(())
}
