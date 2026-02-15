//! AI Novel Agent - CLI Entry Point

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ai-novel-agent")]
#[command(about = "AI-powered novel generation system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new novel project
    New {
        /// Novel name
        name: String,

        /// Genre (fantasy, urban, xianxia, etc.)
        #[arg(short, long)]
        genre: String,

        /// Target word count
        #[arg(long, default_value = "1000000")]
        target: u64,
    },

    /// Run feasibility analysis for a genre
    Feasibility {
        /// Project ID
        project_id: String,

        /// Genre to analyze
        #[arg(short, long)]
        genre: String,
    },

    /// Generate novel outline
    Outline {
        /// Project ID
        project_id: String,

        /// Premise/pitch
        #[arg(short, long)]
        premise: String,

        /// Theme
        #[arg(short, long)]
        theme: Option<String>,
    },

    /// Generate chapter plan
    Plan {
        /// Project ID
        project_id: String,
    },

    /// Generate chapter(s)
    Generate {
        /// Project ID
        project_id: String,

        /// Chapter number (or range like "1-10")
        #[arg(short, long)]
        chapters: String,
    },

    /// Publish to Fanqie platform
    Publish {
        /// Project ID
        project_id: String,

        /// Subcommand
        #[command(subcommand)]
        action: PublishAction,
    },

    /// Check consistency
    Check {
        /// Project ID
        project_id: String,
    },
}

#[derive(Subcommand)]
enum PublishAction {
    /// Create novel on Fanqie
    Create,

    /// Upload chapters
    Upload {
        /// Chapter range
        chapters: String,
    },

    /// Submit for review
    Submit {
        /// Chapter range
        chapters: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    ai_novel_agent::init_logging();

    // Parse CLI
    let cli = Cli::parse();

    // Set log level based on verbosity
    let _log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing::info!("Starting AI Novel Agent");

    // Load config
    let _config = ai_novel_agent::config::load_config(&cli.config)?;

    // Execute command
    match cli.command {
        Commands::New { name, genre, target } => {
            tracing::info!("Creating new project: {} ({})", name, genre);

            // Parse genre
            let novel_genre = match genre.to_lowercase().as_str() {
                "fantasy" => ai_novel_agent::models::NovelGenre::Fantasy,
                "urban" => ai_novel_agent::models::NovelGenre::Urban,
                "xianxia" => ai_novel_agent::models::NovelGenre::Xianxia,
                "historical" => ai_novel_agent::models::NovelGenre::Historical,
                "romance" => ai_novel_agent::models::NovelGenre::Romance,
                "scifi" => ai_novel_agent::models::NovelGenre::Scifi,
                "game" => ai_novel_agent::models::NovelGenre::Game,
                "horror" => ai_novel_agent::models::NovelGenre::Horror,
                _ => {
                    println!("Unknown genre: {}, using Fantasy", genre);
                    ai_novel_agent::models::NovelGenre::Fantasy
                }
            };

            // Create project
            let project = ai_novel_agent::models::NovelProject::new(name, novel_genre, target);

            // Save project
            let storage = ai_novel_agent::services::StorageService::new(".")?;
            storage.save(&project)?;

            println!("Project created: {} (target: {} words)", project.name, project.target_word_count);
            println!("Project ID: {}", project.id);
        }
        Commands::Feasibility { project_id, genre } => {
            tracing::info!("Running feasibility analysis for: {}", project_id);
            ai_novel_agent::cli::commands::feasibility::run(&project_id, &genre).await?;
        }
        Commands::Outline { project_id, premise, theme } => {
            tracing::info!("Generating outline for: {}", project_id);
            ai_novel_agent::cli::commands::outline::run(&project_id, &premise, theme.as_deref()).await?;
        }
        Commands::Plan { project_id } => {
            tracing::info!("Generating chapter plan for: {}", project_id);
            ai_novel_agent::cli::commands::plan::run(&project_id).await?;
        }
        Commands::Generate { project_id, chapters } => {
            tracing::info!("Generating chapters {} for: {}", chapters, project_id);
            ai_novel_agent::cli::commands::generate::run(&project_id, &chapters).await?;
        }
        Commands::Publish { project_id, action } => {
            tracing::info!("Publishing {} to Fanqie", project_id);
            let action_str = match action {
                PublishAction::Create => "create",
                PublishAction::Upload { .. } => "upload",
                PublishAction::Submit { .. } => "submit",
            };
            ai_novel_agent::cli::commands::publish::run(&project_id, action_str).await?;
        }
        Commands::Check { project_id } => {
            tracing::info!("Checking consistency for: {}", project_id);
            ai_novel_agent::cli::commands::check::run(&project_id).await?;
        }
    }

    Ok(())
}
