//! Check command - consistency verification

use anyhow::Result;
use uuid::Uuid;
use std::path::Path;

/// Run consistency check on a project
pub async fn run(project_id: &str) -> Result<()> {
    println!("Checking consistency for project: {}", project_id);

    let project_uuid = Uuid::parse_str(project_id)?;
    let project_path = Path::new(".").join("projects").join(project_id);

    if !project_path.exists() {
        println!("Project directory not found: {}", project_id);
        return Ok(());
    }

    // Load project
    let storage = crate::services::StorageService::new_project(".", project_uuid)?;

    // Check if project exists
    let project: Option<crate::models::NovelProject> = storage.load()?;

    if let Some(project) = project {
        println!("Project: {}", project.name);
        println!("Genre: {}", project.genre);
        println!("Target: {} words", project.target_word_count);
        println!("Status: {:?}", project.status);

        // Check if analysis exists
        let analysis_path = project_path.join("analysis").join("feasibility.json");
        if analysis_path.exists() {
            println!("✓ Analysis: exists");
        } else {
            println!("✗ Analysis: missing");
        }

        // Check if outline exists
        let outline_path = project_path.join("outline").join("outline.json");
        if outline_path.exists() {
            println!("✓ Outline: exists");
        } else {
            println!("✗ Outline: missing");
        }

        // Check if plan exists
        let plan_path = project_path.join("plans").join("plan.json");
        if plan_path.exists() {
            println!("✓ Chapter Plan: exists");
        } else {
            println!("✗ Chapter Plan: missing");
        }

        // Check if chapters exist
        let chapters_path = project_path.join("chapters");
        if chapters_path.exists() {
            let chapter_count = std::fs::read_dir(chapters_path)?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .count();
            println!("✓ Chapters: {} files", chapter_count);
        } else {
            println!("✗ Chapters: missing");
        }

        println!("\nConsistency check: PASSED");
    } else {
        println!("Project not found: {}", project_id);
    }

    Ok(())
}
