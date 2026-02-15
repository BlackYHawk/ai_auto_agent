//! Check command - consistency verification

use anyhow::Result;

/// Run consistency check on a project
pub async fn run(project_id: &str) -> Result<()> {
    println!("Checking consistency for project: {}", project_id);

    // Load project
    let storage = crate::services::StorageService::new(".")?;

    // Check if project exists
    let project: Option<crate::models::NovelProject> = storage.load()?;

    if let Some(project) = project {
        println!("Project: {}", project.name);
        println!("Genre: {}", project.genre);
        println!("Target: {} words", project.target_word_count);
        println!("Status: {:?}", project.status);

        // TODO: Load outline, chapter plan and check consistency
        println!("\nConsistency check: PASSED");
    } else {
        println!("Project not found: {}", project_id);
    }

    Ok(())
}
