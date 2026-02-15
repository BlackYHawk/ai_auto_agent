//! Publish Command

use anyhow::Result;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;
use crate::models::NovelProject;
use crate::services::StorageService;

pub async fn run(project_id: &str, action: &str) -> Result<()> {
    tracing::info!("Publishing {} to Fanqie with action: {}", project_id, action);

    let project_uuid = Uuid::parse_str(project_id)?;

    // Load project data
    let storage = StorageService::new_project(".", project_uuid)?;
    let project: NovelProject = storage.load()?.ok_or_else(|| anyhow::anyhow!("Project not found"))?;

    // Check if browser automation is requested
    let use_browser = std::env::var("FANQIE_BROWSER").unwrap_or_default() == "true";

    if use_browser {
        run_browser_automation(action, project_id, &project.name, &project.genre.to_string()).await?;
    } else {
        run_demo_mode(project_id, action, &project).await;
    }

    Ok(())
}

async fn run_browser_automation(action: &str, project_id: &str, title: &str, genre: &str) -> Result<()> {
    println!("\n=== Running Browser Automation ===");

    // Get credentials - check config first, then env vars
    let (username, password) = get_credentials();

    if username.is_empty() || password.is_empty() {
        println!("⚠️ No credentials found, using demo mode");
        println!("   Set FANQIE_USERNAME and FANQIE_PASSWORD env vars");
        return Ok(());
    }

    // Find script path
    let script_path = Path::new("scripts/fanqie-auto.js");
    if !script_path.exists() {
        println!("⚠️ Browser automation script not found");
        return Ok(());
    }

    // Run node script
    let output = Command::new("node")
        .arg(script_path)
        .arg(action)
        .arg(project_id)
        .arg(title)
        .arg(genre)
        .arg("")
        .env("FANQIE_USERNAME", &username)
        .env("FANQIE_PASSWORD", &password)
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("{}", stdout);

    if !output.status.success() {
        println!("⚠️ Browser automation warning: {}", stderr);
    }

    Ok(())
}

fn get_credentials() -> (String, String) {
    // First check environment variables
    let username = std::env::var("FANQIE_USERNAME").unwrap_or_default();
    let password = std::env::var("FANQIE_PASSWORD").unwrap_or_default();

    if !username.is_empty() && !password.is_empty() {
        return (username, password);
    }

    // Then check config file
    if let Ok(config) = std::fs::read_to_string("config.toml") {
        if let Ok(parsed) = config.parse::<toml::Value>() {
            let u = parsed.get("fanqie")
                .and_then(|f| f.get("username"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let p = parsed.get("fanqie")
                .and_then(|f| f.get("password"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if !u.is_empty() && !p.is_empty() {
                return (u.to_string(), p.to_string());
            }
        }
    }

    (username, password)
}

async fn run_demo_mode(_project_id: &str, action: &str, project: &NovelProject) {
    println!("\n=== Fanqie Publishing (Demo Mode) ===");
    println!("Project: {}", project.name);

    match action {
        "create" => {
            println!("✅ Novel created on Fanqie (demo)");
            println!("   To use real automation, set: FANQIE_BROWSER=true");
        }
        "upload" => {
            println!("✅ Chapters uploaded to Fanqie (demo)");
        }
        "submit" => {
            println!("✅ Chapters submitted for review (demo)");
        }
        _ => {
            println!("Unknown action: {}", action);
        }
    }
}
