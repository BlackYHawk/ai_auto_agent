//! Outline Command

use anyhow::Result;
use uuid::Uuid;
use crate::models::NovelGenre;
use crate::services::{OutlineService, StorageService};

pub async fn run(project_id: &str, premise: &str, theme: Option<&str>, target: u64, genre: &str) -> Result<()> {
    tracing::info!("Generating outline for: {}", project_id);

    let theme = theme.unwrap_or("成长与挑战");
    let project_uuid = Uuid::parse_str(project_id)?;

    // Parse genre
    let novel_genre = match genre.to_lowercase().as_str() {
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

    let service = OutlineService::new();
    let outline = service.generate(
        project_uuid,
        novel_genre,
        premise.to_string(),
        theme.to_string(),
        target,
    ).await?;

    // Save outline to project directory
    let storage = StorageService::new_project(".", project_uuid)?;
    storage.save(&outline)?;

    println!("\n=== Novel Outline ===");
    println!("Premise: {}", outline.premise);
    println!("Theme: {}", outline.theme);
    println!("\n=== Plot Arcs ===");
    for arc in &outline.arcs {
        println!("\n--- {} ---", arc.name);
        println!("Chapters {}-: {}", arc.start_chapter, arc.summary);
        println!("Climax: {}", arc.climax);
    }
    println!("\n=== Protagonist ===");
    println!("Name: {}", outline.protagonist.name);
    println!("Traits: {:?}", outline.protagonist.personality_traits);
    println!("Arc: {}", outline.protagonist.arc_description);
    println!("\n=== World Settings ===");
    println!("World: {}", outline.world_settings.name);
    println!("Type: {:?}", outline.world_settings.world_type);
    println!("Rules: {:?}", outline.world_settings.rules);

    println!("\nSaved to: projects/{}/outline/outline.json", project_id);

    Ok(())
}
