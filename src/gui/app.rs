//! GUI Application State and Main App

use eframe::App;
use egui::Context;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

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
            PublishAction::Create => "Create Book",
            PublishAction::Upload => "Upload Chapters",
            PublishAction::Submit => "Submit for Review",
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

    /// Running async tasks
    pub running_tasks: HashMap<String, TaskState>,

    /// Error message to display
    pub error_message: Option<String>,

    /// Form states
    pub new_project_form: NewProjectForm,
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
}

impl Default for NovelApp {
    fn default() -> Self {
        // Default storage path is ./storage
        let storage_root = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("storage");

        Self {
            current_screen: Screen::Projects,
            storage_root,
            projects: Vec::new(),
            selected_project_id: None,
            running_tasks: HashMap::new(),
            error_message: None,
            new_project_form: NewProjectForm::default(),
            outline_form: OutlineForm::default(),
            generate_form: GenerateForm::default(),
            publish_form: PublishForm::default(),
            analysis_result: None,
            outline_result: None,
            chapter_result: None,
            publish_result: None,
            projects_loaded: false,
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

        let storage_path = &self.storage_root;
        if !storage_path.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(storage_path)
            .map_err(|e| format!("Failed to read storage directory: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Try to load project from this folder
                if let Ok(storage) = StorageService::new(&path) {
                    if let Ok(Some(project)) = storage.load::<NovelProject>() {
                        self.projects.push(project);
                    }
                }
            }
        }

        Ok(())
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

        // Create storage for project
        let project_path = self.storage_root.join(project_id.to_string());
        std::fs::create_dir_all(&project_path)
            .map_err(|e| format!("Failed to create project directory: {}", e))?;

        let storage = StorageService::new(&project_path)
            .map_err(|e| format!("Failed to create storage: {}", e))?;

        storage.save(&project)
            .map_err(|e| format!("Failed to save project: {}", e))?;

        self.projects.push(project.clone());

        Ok(project)
    }

    /// Run feasibility analysis for a project
    pub fn run_feasibility_analysis(&mut self, genre: NovelGenre) -> Result<String, String> {
        // Demo implementation - in production, call the actual service
        Ok(format!(
            "Feasibility Analysis for {:?}\n\n\
            Market Viability: 75/100\n\
            Competition Level: High\n\
            Differentiation Potential: 80/100\n\n\
            Note: Full analysis requires LLM API configuration.",
            genre
        ))
    }

    /// Generate outline for a project
    pub fn generate_outline(&mut self, _project_id: Uuid, genre: NovelGenre, premise: String, theme: String, target_words: u64) -> Result<String, String> {
        // Demo implementation - in production, call the actual service
        let mut result = format!(
            "Outline Generated\n\n\
            Premise: {}\n\
            Theme: {}\n\
            Genre: {:?}\n\
            Target: {} words\n\n",
            premise, theme, genre, target_words
        );

        result.push_str("Plot Arcs:\n");
        result.push_str("- Act 1: Introduction (Chapters 1-10)\n");
        result.push_str("  Summary: The protagonist begins their journey...\n");
        result.push_str("- Act 2: Rising Action (Chapters 11-30)\n");
        result.push_str("  Summary: Challenges and growth emerge...\n");
        result.push_str("- Act 3: Climax & Resolution (Chapters 31-40)\n");
        result.push_str("  Summary: Final confrontation and conclusion...\n\n");

        result.push_str("Note: Full outline generation requires LLM API configuration.");

        Ok(result)
    }

    /// Generate chapters for a project
    pub fn generate_chapters(&mut self, _project_id: Uuid, chapter_start: u32, chapter_end: u32) -> Result<String, String> {
        // Demo implementation - in production, call the actual service
        let mut results = String::new();

        for chapter_num in chapter_start..=chapter_end {
            let chapter_text = format!(
                "\n=== Chapter {} ===\n\nThis is a placeholder for chapter {}. In production, this would contain AI-generated content based on the novel's outline and previous chapters. (Full chapter generation requires LLM API configuration)\n\n",
                chapter_num, chapter_num
            );
            results.push_str(&chapter_text);
        }

        Ok(results)
    }

    /// Run consistency check
    pub fn run_consistency_check(&mut self, project_id: Uuid) -> Result<String, String> {
        // Demo implementation - in production, call the actual service
        let result = format!(
            "Consistency Check Report\n\nProject ID: {}\n\nChecks performed:\n- Character consistency: OK\n- Plot consistency: OK\n- Timeline consistency: OK\n- World-building consistency: OK\n\nNote: Full consistency check requires generated chapters.\n",
            project_id
        );

        Ok(result)
    }

    /// Publish to Fanqie platform
    pub fn publish_to_fanqie(
        &mut self,
        project_id: Uuid,
        action: PublishAction,
        chapter_range: String,
    ) -> Result<String, String> {
        // Demo implementation - in production, use FanqieClient
        let action_label = match action {
            PublishAction::Create => "Create Novel",
            PublishAction::Upload => "Upload Chapters",
            PublishAction::Submit => "Submit for Review",
        };

        let result = format!(
            "Publish Action: {}\nProject ID: {}\nChapter Range: {}\n\nStatus: Ready to publish\n\nNote: Full Fanqie integration requires:\n1. Configure cookies in config.toml\n2. Login to Fanqie account\n3. Select target book or create new\n\nFor production use, run CLI: ai-novel-agent publish --project-id {} --action {:?}",
            action_label, project_id, chapter_range, project_id, action
        );

        Ok(result)
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
