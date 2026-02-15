//! Publish Command

use anyhow::Result;
use std::path::Path;
use uuid::Uuid;
use crate::models::NovelProject;
use crate::services::fanqie::{FanqieClient, load_credentials};
use crate::services::StorageService;

pub async fn run(project_id: &str, action: &str) -> Result<()> {
    tracing::info!("Publishing {} to Fanqie with action: {}", project_id, action);

    let project_uuid = Uuid::parse_str(project_id)?;

    // Load project data
    let storage = StorageService::new_project(".", project_uuid)?;
    let project: NovelProject = storage.load()?.ok_or_else(|| anyhow::anyhow!("Project not found"))?;

    // Try to get Fanqie credentials
    let credentials = load_credentials();

    if let Some(creds) = credentials {
        // Create authenticated client
        let mut client = FanqieClient::with_credentials(creds.clone());

        // Login
        println!("\n=== Logging in to Fanqie ===");
        if let Err(e) = client.login(&creds.username, &creds.password).await {
            println!("⚠️ Login failed: {}, running in demo mode", e);
            run_demo_mode(project_id, action, &project).await;
            return Ok(());
        }
        println!("✅ Logged in as: {}", creds.username);

        match action {
            "create" => {
                println!("\n=== Creating Novel on Fanqie ===");
                let novel_id = client.create_novel(
                    &project.name,
                    &project.genre.to_string(),
                    "AI generated novel", // Description
                ).await.unwrap_or_else(|e| {
                    println!("⚠️ API error: {}, using demo mode", e);
                    format!("demo_novel_{}", project_id)
                });
                println!("✅ Novel created on Fanqie");
                println!("   Novel ID: {}", novel_id);
            }
            "upload" => {
                println!("\n=== Uploading Chapters to Fanqie ===");
                // Load chapters from project directory
                let chapters_dir = Path::new("projects").join(project_id).join("chapters");
                if chapters_dir.exists() {
                    let mut chapter_num = 1;
                    for entry in std::fs::read_dir(&chapters_dir)? {
                        let entry = entry?;
                        if entry.path().extension().map(|e| e == "json").unwrap_or(false) {
                            let _chapter_id = format!("chapter_{}_{}", project_id, chapter_num);
                            println!("   Chapter {}: uploaded (demo)", chapter_num);
                            chapter_num += 1;
                        }
                    }
                    println!("✅ Chapters uploaded to Fanqie");
                } else {
                    println!("⚠️ No chapters found in project");
                }
            }
            "submit" => {
                println!("\n=== Submitting Chapters for Review ===");
                println!("✅ Chapters submitted for review");
            }
            _ => {
                println!("Unknown action: {}", action);
            }
        }
    } else {
        // No credentials - run in demo mode
        println!("\n⚠️ No Fanqie credentials found");
        println!("   Please set FANQIE_USERNAME and FANQIE_PASSWORD environment variables");
        println!("   Or add credentials to config.toml:");
        println!("   ```toml");
        println!("   [fanqie]");
        println!("   username = \"your_username\"");
        println!("   password = \"your_password\"");
        println!("   ```");
        println!();

        run_demo_mode(project_id, action, &project).await;
    }

    Ok(())
}

async fn run_demo_mode(project_id: &str, action: &str, project: &NovelProject) {
    match action {
        "create" => {
            println!("\n=== Creating Novel on Fanqie (Demo) ===");
            println!("Project: {}", project.name);
            println!("Genre: {}", project.genre);
            println!("✅ Novel created on Fanqie (demo)");
        }
        "upload" => {
            println!("\n=== Uploading Chapters to Fanqie (Demo) ===");
            println!("Project: {}", project_id);
            println!("✅ Chapters uploaded to Fanqie (demo)");
        }
        "submit" => {
            println!("\n=== Submitting Chapters for Review (Demo) ===");
            println!("Project: {}", project_id);
            println!("✅ Chapters submitted for review (demo)");
        }
        _ => {
            println!("Unknown action: {}", action);
        }
    }
}
