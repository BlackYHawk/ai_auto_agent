//! AI Novel Agent - CLI Entry Point

use anyhow::{Result, Context};
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
        /// Novel name/title
        name: String,

        /// Novel summary/description (required, 10-2000 characters)
        #[arg(short, long = "summary")]
        summary: String,

        /// Genre (fantasy, urban, xianxia, historical, romance, scifi, game, horror)
        #[arg(short, long = "genre")]
        genre: String,

        /// Target word count
        #[arg(short, long = "target", default_value = "1000000")]
        target: u64,
    },

    /// Run feasibility analysis for a genre
    Feasibility {
        /// Project ID (optional, will create temp project if not provided)
        #[arg(short = 'i', long = "project-id")]
        project_id: Option<String>,

        /// Genre to analyze (fantasy, urban, xianxia, etc.)
        #[arg(short, long = "genre")]
        genre: String,
    },

    /// Generate novel outline
    Outline {
        /// Project ID
        #[arg(short = 'i', long = "project-id")]
        project_id: String,

        /// Premise/pitch of the story (optional, uses project summary if not provided)
        #[arg(short = 'm', long = "premise")]
        premise: Option<String>,

        /// Theme of the story
        #[arg(short = 't', long = "theme")]
        theme: Option<String>,

        /// Target word count
        #[arg(short = 'w', long = "target", default_value = "1000000")]
        target: u64,

        /// Genre (optional, will use project genre if not provided)
        #[arg(short, long = "genre")]
        genre: Option<String>,
    },

    /// Generate chapter plan
    Plan {
        /// Project ID
        #[arg(short = 'i', long = "project-id")]
        project_id: String,
    },

    /// Generate chapter(s)
    Generate {
        /// Project ID
        #[arg(short = 'i', long = "project-id")]
        project_id: String,

        /// Chapter number (or range like "1-10")
        #[arg(short = 'c', long = "chapters")]
        chapters: String,
    },

    /// Publish to Fanqie platform
    Publish {
        /// Project ID
        #[arg(short = 'i', long = "project-id")]
        project_id: String,

        /// Subcommand
        #[command(subcommand)]
        action: PublishAction,
    },

    /// Check consistency
    Check {
        /// Project ID
        #[arg(short = 'i', long = "project-id")]
        project_id: String,
    },

    /// Launch GUI
    Gui,
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
        Commands::New { name, summary, genre, target } => {
            tracing::info!("Creating new project: {} ({})", name, genre);

            // Validate project fields
            use ai_novel_agent::services::validation::ProjectValidator;
            let validation = ProjectValidator::validate(&name, &summary, &genre, target);

            if !validation.valid {
                println!("Validation failed:");
                for error in &validation.errors {
                    println!("  - {}: {}", error.field, error.message);
                }
                anyhow::bail!("Project validation failed");
            }

            // Show warnings if any
            for warning in &validation.warnings {
                println!("Warning: {} - {}", warning.field, warning.message);
            }

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

            // Create project with summary
            let project = ai_novel_agent::models::NovelProject::new_with_summary(
                name,
                summary,
                novel_genre,
                target,
            );

            // Save project with directory structure
            let storage = ai_novel_agent::services::StorageService::new_project(".", project.id)?;
            storage.save(&project)?;

            println!("Project created: {} (target: {} words)", project.name, project.target_word_count);
            println!("Project ID: {}", project.id);
            println!("Project directory: projects/{}/", project.id);
        }
        Commands::Feasibility { project_id, genre } => {
            // If no project_id provided, generate a temp one
            let proj_id = project_id.unwrap_or_else(|| {
                let temp_id = uuid::Uuid::new_v4();
                println!("No project-id provided, using temporary ID: {}", temp_id);
                temp_id.to_string()
            });
            tracing::info!("Running feasibility analysis for: {}", proj_id);
            ai_novel_agent::cli::commands::feasibility::run(&proj_id, &genre).await?;
        }
        Commands::Outline { project_id, premise, theme, target, genre } => {
            tracing::info!("Generating outline for: {}", project_id);

            // Parse project ID
            let project_uuid = uuid::Uuid::parse_str(&project_id)
                .context("Invalid project ID format")?;

            // Load project to get summary
            let storage = ai_novel_agent::services::StorageService::new_project(".", project_uuid)?;
            let project = storage.load::<ai_novel_agent::models::NovelProject>()?
                .context("Project not found")?;

            // Use provided premise or fall back to project summary
            let final_premise = premise.unwrap_or_else(|| {
                if project.summary.is_empty() {
                    tracing::warn!("No premise provided and project summary is empty, using default");
                    "".to_string()
                } else {
                    project.summary.clone()
                }
            });

            // Parse genre if provided, otherwise use project genre
            let genre_str = genre.unwrap_or_else(|| project.genre.to_string());
            ai_novel_agent::cli::commands::outline::run(&project_id, &final_premise, theme.as_deref(), target, &genre_str).await?;
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
        Commands::Gui => {
            tracing::info!("Launching GUI");
            if let Err(e) = run_gui() {
                eprintln!("GUI error: {}", e);
            }
            return Ok(());
        }
    }

    Ok(())
}

fn run_gui() -> std::result::Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("AI Novel Agent"),
        ..Default::default()
    };

    eframe::run_native(
        "AI Novel Agent",
        options,
        Box::new(|cc| {
            // 配置中文字体
            let mut fonts = egui::FontDefinitions::default();

            // 使用 font-kit 加载系统字体
            let source = font_kit::source::SystemSource::new();

            // 尝试查找中文字体
            let font_names = [
                "Hiragino Sans GB",
                "PingFang SC",
                "Microsoft YaHei",
                "SimHei",
                "Noto Sans CJK SC",
            ];

            let mut font_loaded = false;
            for font_name in &font_names {
                if let Ok(family) = source.select_family_by_name(font_name) {
                    for handle in family.fonts() {
                        match handle {
                            font_kit::handle::Handle::Path { path, font_index: _ } => {
                                if let Ok(font_data) = std::fs::read(&path) {
                                    fonts.font_data.insert(
                                        "chinese".to_string(),
                                        egui::FontData::from_owned(font_data),
                                    );
                                    font_loaded = true;
                                    break;
                                }
                            }
                            font_kit::handle::Handle::Memory { bytes, font_index: _ } => {
                                let font_data: Vec<u8> = bytes.as_ref().clone();
                                fonts.font_data.insert(
                                    "chinese".to_string(),
                                    egui::FontData::from_owned(font_data),
                                );
                                font_loaded = true;
                                break;
                            }
                        }
                    }
                    if font_loaded {
                        break;
                    }
                }
            }

            // 设置为默认字体
            if fonts.font_data.contains_key("chinese") {
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "chinese".to_string());

                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, "chinese".to_string());
            }

            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(ai_novel_agent::gui::app::NovelApp::default()))
        }),
    )
}
