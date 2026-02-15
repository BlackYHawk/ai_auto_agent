//! Plan Command

use anyhow::Result;
use crate::models::NovelOutline;
use crate::services::ChapterPlanningService;

pub async fn run(project_id: &str) -> Result<()> {
    tracing::info!("Generating chapter plan for: {}", project_id);

    // Create a mock outline for demonstration
    let outline = NovelOutline::new(
        uuid::Uuid::new_v4(),
        "一个少年踏上修仙之路".to_string(),
        "坚持与成长".to_string(),
        1_000_000,
    );

    let service = ChapterPlanningService::new();
    let plan = service.generate_plan(uuid::Uuid::new_v4(), &outline).await?;

    println!("\n=== Chapter Plan ===");
    println!("Total Chapters: {}", plan.total_chapters);
    println!("Plot Twist Chapters: {:?}", plan.plot_twist_positions);
    println!("\n=== First 15 Chapters ===");

    for chapter in plan.chapters.iter().take(15) {
        println!("\n--- Chapter {} ---", chapter.number);
        println!("Title: {}", chapter.title);
        println!("Summary: {}", chapter.summary);
        println!("Key Event: {}", chapter.key_events.first().unwrap_or(&"N/A".to_string()));

        if chapter.is_plot_twist_chapter {
            println!("🔀 PLOT TWIST: {}", chapter.plot_twist_description.as_ref().unwrap_or(&"".to_string()));
        }
    }

    if plan.total_chapters > 15 {
        println!("\n... and {} more chapters", plan.total_chapters - 15);
    }

    Ok(())
}
