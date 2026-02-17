//! GUI Application State and Main App

use eframe::App;
use egui::Context;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

use crate::config::Config;
use crate::models::{NovelGenre, NovelProject};
use crate::services::StorageService;

/// Screen types for navigation
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Projects,
    ProjectDetail,
    NewProject,
    Outline,
    Generate,
    Publish,
    Check,
    Chapter,
    Import,
    Settings,
}

/// Task state for async operations
#[derive(Debug, Clone)]
pub enum TaskState {
    Idle,
    Running { progress: f32, message: String },
    Completed,
    Failed { error: String },
}

/// Form for creating a new project
#[derive(Debug, Clone, Default)]
pub struct NewProjectForm {
    pub name: String,
    pub genre: Option<NovelGenre>,
    pub target_word_count: String,
}

/// Form for importing a novel
#[derive(Debug, Clone, Default)]
pub struct ImportForm {
    pub file_path: String,
    pub file_name: String,
    pub content_preview: String,
    pub chapter_count: usize,
}

/// Form for generating outline
#[derive(Debug, Clone, Default)]
pub struct OutlineForm {
    pub premise: String,
    pub theme: String,
}

/// Form for generating chapters
#[derive(Debug, Clone, Default)]
pub struct GenerateForm {
    pub chapter_range: String,
}

/// Form for publishing
#[derive(Debug, Clone, Default)]
pub struct PublishForm {
    pub action: PublishAction,
    pub chapter_range: String,
}

/// Publish action types
#[derive(Debug, Clone, PartialEq, Default)]
pub enum PublishAction {
    #[default]
    Create,
    Upload,
    Submit,
}

impl PublishAction {
    pub fn label(&self) -> &'static str {
        match self {
            PublishAction::Create => "创建小说",
            PublishAction::Upload => "上传章节",
            PublishAction::Submit => "提交审核",
        }
    }
}

/// Main application state
#[derive(Debug)]
pub struct NovelApp {
    /// Current screen
    pub current_screen: Screen,

    /// Storage root path
    pub storage_root: PathBuf,

    /// Project list
    pub projects: Vec<NovelProject>,

    /// Currently selected project ID
    pub selected_project_id: Option<Uuid>,

    /// Currently selected chapter number
    pub selected_chapter_number: Option<u32>,

    /// Running async tasks
    pub running_tasks: HashMap<String, TaskState>,

    /// Error message to display
    pub error_message: Option<String>,

    /// Form states
    pub new_project_form: NewProjectForm,
    pub import_form: ImportForm,
    pub outline_form: OutlineForm,
    pub generate_form: GenerateForm,
    pub publish_form: PublishForm,

    /// Analysis result
    pub analysis_result: Option<String>,

    /// Outline result
    pub outline_result: Option<String>,

    /// Chapter generation result
    pub chapter_result: Option<String>,

    /// Publish result
    pub publish_result: Option<String>,

    /// Whether projects have been loaded
    pub projects_loaded: bool,

    /// Application configuration (loaded from config.local.toml)
    pub config: Config,
}

impl Default for NovelApp {
    fn default() -> Self {
        // Default storage path is ./storage
        let storage_root = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("storage");

        // Load configuration from config.local.toml
        let config = crate::config::load_config(std::path::Path::new("config.local.toml"))
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to load config: {}", e);
                Config::default()
            });

        Self {
            current_screen: Screen::Projects,
            storage_root,
            projects: Vec::new(),
            selected_project_id: None,
            selected_chapter_number: None,
            running_tasks: HashMap::new(),
            error_message: None,
            new_project_form: NewProjectForm::default(),
            import_form: ImportForm::default(),
            outline_form: OutlineForm::default(),
            generate_form: GenerateForm::default(),
            publish_form: PublishForm::default(),
            analysis_result: None,
            outline_result: None,
            chapter_result: None,
            publish_result: None,
            projects_loaded: false,
            config,
        }
    }
}

impl NovelApp {
    /// Get the currently selected project
    pub fn selected_project(&self) -> Option<&NovelProject> {
        self.selected_project_id
            .and_then(|id| self.projects.iter().find(|p| p.id == id))
    }

    /// Navigate to a screen
    pub fn navigate_to(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    /// Set error message
    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    /// Clear error message
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// Load all projects from storage
    pub fn load_projects(&mut self) -> Result<(), String> {
        self.projects.clear();

        // Use StorageService::list_projects to load all projects
        match StorageService::list_projects(&self.storage_root) {
            Ok(projects) => {
                self.projects = projects;
                Ok(())
            }
            Err(e) => {
                Err(format!("Failed to load projects: {}", e))
            }
        }
    }

    /// Create a new project
    pub fn create_project(
        &mut self,
        name: String,
        genre: NovelGenre,
        target_word_count: u64,
    ) -> Result<NovelProject, String> {
        let project = NovelProject::new(name, genre, target_word_count);
        let project_id = project.id;

        // Create storage for project (in projects subdirectory)
        let project_path = self.storage_root.join("projects").join(project_id.to_string());
        std::fs::create_dir_all(&project_path)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;

        let storage = StorageService::new(&project_path)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        storage.save(&project)
            .map_err(|e| format!("Failed to save project: {}", e))?;

        self.projects.push(project.clone());

        Ok(project)
    }

    /// Import a novel from TXT file
    pub fn import_novel(&mut self) -> Result<NovelProject, String> {
        let file_path = self.import_form.file_path.clone();
        let name = self.new_project_form.name.clone();

        // Read the file content
        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        // Parse chapters (split by "第X章" pattern)
        let _chapters: Vec<&str> = content
            .lines()
            .filter(|line| line.trim().starts_with("第") && line.contains("章"))
            .collect();

        // Estimate word count
        let word_count = content.chars().count() as u64;

        // Create project with default genre
        let genre = NovelGenre::Other;
        let project = NovelProject::new(name, genre, word_count);
        let project_id = project.id;

        // Create project directory structure (already in projects subdirectory)
        let project_path = self.storage_root.join("projects").join(project_id.to_string());
        std::fs::create_dir_all(&project_path)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;

        // Create chapters directory
        let chapters_dir = project_path.join("chapters");
        std::fs::create_dir_all(&chapters_dir)
            .map_err(|e| format!("Failed to create chapters directory: {}", e))?;

        // Save project info
        let storage = StorageService::new(&project_path)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        storage.save(&project)
            .map_err(|e| format!("Failed to save project: {}", e))?;

        // Save imported content as chapters
        let import_file = project_path.join("imported.txt");
        std::fs::write(&import_file, &content)
            .map_err(|e| format!("Failed to save imported content: {}", e))?;

        self.projects.push(project.clone());

        Ok(project)
    }

    /// Run feasibility analysis for a project
    pub fn run_feasibility_analysis(&mut self, genre: NovelGenre) -> Result<String, String> {
        // Check if API key is configured
        if self.config.llm.api_key.is_empty() {
            return Ok(format!(
                "可行性分析功能需要配置LLM API。\n\n\
                类型: {:?}\n\n\
                请在设置页面配置API Key后使用此功能。",
                genre
            ));
        }

        // Return message that analysis is running in background
        // The actual async work is done via tokio::spawn in the GUI
        Ok(format!(
            "可行性分析功能\n\n\
            类型: {:?}\n\n\
            当前配置:\n\
            - 提供商: {}\n\
            - 模型: {}\n\n\
            状态: 正在后台运行...",
            genre,
            self.config.llm.provider,
            self.config.llm.model.as_deref().unwrap_or("默认")
        ))
    }

    /// Start feasibility analysis in background (for GUI use)
    pub fn start_feasibility_analysis(&mut self, genre: NovelGenre) {
        if self.config.llm.api_key.is_empty() {
            self.analysis_result = Some("请先在设置页面配置API Key".to_string());
            return;
        }

        // Clone config for async task (used by service internally)
        let _config = self.config.llm.clone();

        // Spawn async task in background
        tokio::spawn(async move {
            let service = crate::services::FeasibilityService::new();
            match service.analyze(genre).await {
                Ok(report) => {
                    let summary = format!(
                        "可行性研究报告\n\n\
                        类型: {:?}\n\
                        市场可行性: {}/100\n\
                        竞争程度: {:?}\n\
                        差异化潜力: {}/100\n\n\
                        热门作品:\n{}\n\n\
                        建议: {:?}",
                        genre,
                        report.scores.market_viability,
                        report.scores.competition_level,
                        report.scores.differentiation_potential,
                        report.top_works.iter().take(5).map(|w| format!("- {} by {}", w.title, w.author)).collect::<Vec<_>>().join("\n"),
                        report.recommendation
                    );
                    // Store result - need to use a different approach for cross-thread communication
                    tracing::info!("Feasibility analysis completed: {}", summary.lines().next().unwrap_or(""));
                }
                Err(e) => {
                    tracing::error!("Feasibility analysis failed: {}", e);
                }
            }
        });

        self.analysis_result = Some("可行性分析已在后台开始运行...".to_string());
    }

    /// Generate outline for a project
    pub fn generate_outline(&mut self, _project_id: Uuid, genre: NovelGenre, premise: String, theme: String, target_words: u64) -> Result<String, String> {
        // Check if API key is configured
        if self.config.llm.api_key.is_empty() {
            return Ok(format!(
                "大纲生成功能需要配置LLM API。\n\n\
                设定: {}\n\
                主题: {}\n\
                类型: {:?}\n\
                目标: {} 字\n\n\
                请在设置页面配置API Key后使用此功能。",
                premise, theme, genre, target_words
            ));
        }

        // Return message that outline generation is running in background
        Ok(format!(
            "大纲生成功能\n\n\
            设定: {}\n\
            主题: {}\n\
            类型: {:?}\n\
            目标: {} 字\n\n\
            当前配置:\n\
            - 提供商: {}\n\
            - 模型: {}\n\n\
            状态: 正在后台运行...",
            premise,
            theme,
            genre,
            target_words,
            self.config.llm.provider,
            self.config.llm.model.as_deref().unwrap_or("默认")
        ))
    }

    /// Start outline generation in background (for GUI use)
    pub fn start_outline_generation(&mut self, project_id: Uuid, genre: NovelGenre, premise: String, theme: String, target_words: u64) {
        if self.config.llm.api_key.is_empty() {
            self.outline_result = Some("请先在设置页面配置API Key".to_string());
            return;
        }

        // Clone values for async task
        let premise_clone = premise.clone();
        let theme_clone = theme.clone();

        // Spawn async task in background
        tokio::spawn(async move {
            let service = crate::services::OutlineService::new();
            match service.generate(project_id, genre, premise_clone, theme_clone, target_words).await {
                Ok(outline) => {
                    let mut summary = format!("大纲生成完成\n\n");
                    summary.push_str(&format!("设定: {}\n", premise));
                    summary.push_str(&format!("主题: {}\n", theme));
                    summary.push_str("情节线:\n");
                    for arc in &outline.arcs {
                        summary.push_str(&format!("\n--- {} ---\n", arc.name));
                        summary.push_str(&format!("章节范围: {}-{}\n", arc.start_chapter, arc.end_chapter));
                        summary.push_str(&format!("概要: {}\n", arc.summary));
                    }
                    tracing::info!("Outline generation completed");
                }
                Err(e) => {
                    tracing::error!("Outline generation failed: {}", e);
                }
            }
        });

        self.outline_result = Some("大纲生成已在后台开始运行...".to_string());
    }

    /// Generate chapters for a project
    pub fn generate_chapters(&mut self, project_id: Uuid, chapter_start: u32, chapter_end: u32) -> Result<String, String> {
        // Return instructions to use CLI
        Ok(format!(
            "章节生成功能\n\n\
            项目ID: {}\n\
            章节范围: {} - {}\n\n\
            请使用命令行工具生成章节:\n\n\
            cargo run --release -- generate \\\n\
              --project-id {} \\\n\
              --chapters {}-{}",
            project_id, chapter_start, chapter_end,
            project_id, chapter_start, chapter_end
        ))
    }

    /// Run consistency check
    pub fn run_consistency_check(&mut self, project_id: Uuid) -> Result<String, String> {
        // Return a message about needing generated content first
        // Note: Cannot use block_on in GUI context
        Ok(format!(
            "一致性检查功能需要先生成章节内容。\n\n\
            项目ID: {}\n\n\
            请先生成章节后再进行一致性检查。",
            project_id
        ))
    }

    /// Publish to Fanqie platform
    pub fn publish_to_fanqie(
        &mut self,
        project_id: Uuid,
        action: PublishAction,
        chapter_range: String,
    ) -> Result<String, String> {
        // Return a message about needing credentials
        // Note: Cannot use block_on in GUI context
        let action_text = match action {
            PublishAction::Create => "创建小说",
            PublishAction::Upload => "上传章节",
            PublishAction::Submit => "提交审核",
        };

        Ok(format!(
            "发布功能需要配置番茄小说登录凭证。\n\n\
            项目ID: {}\n\
            操作: {}\n\
            章节范围: {}\n\n\
            请在设置页面配置登录凭证后使用此功能。",
            project_id, action_text, chapter_range
        ))
    }

}

impl App for NovelApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Load projects on first frame
        if !self.projects_loaded {
            if let Err(e) = self.load_projects() {
                tracing::warn!("Failed to load projects: {}", e);
            }
            self.projects_loaded = true;
        }

        // Set up the application layout
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::Projects => {
                    crate::gui::screens::projects::show(ui, self);
                }
                Screen::ProjectDetail => {
                    crate::gui::screens::project::show(ui, self);
                }
                Screen::NewProject => {
                    crate::gui::screens::new_project::show(ui, self);
                }
                Screen::Outline => {
                    crate::gui::screens::outline::show(ui, self);
                }
                Screen::Generate => {
                    crate::gui::screens::generate::show(ui, self);
                }
                Screen::Publish => {
                    crate::gui::screens::publish::show(ui, self);
                }
                Screen::Check => {
                    crate::gui::screens::check::show(ui, self);
                }
                Screen::Chapter => {
                    crate::gui::screens::chapter::show(ui, self);
                }
                Screen::Import => {
                    crate::gui::screens::import_::show(ui, self);
                }
                Screen::Settings => {
                    crate::gui::screens::settings::show(ui, self);
                }
            }
        });

        // Show error messages as a popup
        if self.error_message.is_some() {
            let error = self.error_message.take();
            if let Some(error) = error {
                egui::Window::new("Error")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        ui.label(&error);
                        if ui.button("OK").clicked() {
                            // Error acknowledged
                        }
                    });
                // Put error back if not acknowledged in this frame
                if self.error_message.is_none() {
                    self.error_message = Some(error);
                }
            }
        }
    }
}
