//! File-based storage service

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// Storage service for persisting data
#[allow(dead_code)]
pub struct StorageService {
    base_path: PathBuf,
    project_id: Option<Uuid>,
}

impl StorageService {
    /// Create a new storage service
    pub fn new(base_path: impl Into<PathBuf>) -> Result<Self> {
        let base_path = base_path.into();

        // Create directory if it doesn't exist
        if !base_path.exists() {
            fs::create_dir_all(&base_path)
                .context("Failed to create storage directory")?;
        }

        Ok(Self { base_path, project_id: None })
    }

    /// List all projects in the projects directory
    pub fn list_projects(base_path: impl Into<PathBuf>) -> Result<Vec<NovelProject>> {
        let base_path = base_path.into();
        let projects_dir = base_path.join("projects");

        if !projects_dir.exists() {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();

        for entry in fs::read_dir(&projects_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let project_file = path.join("project.json");
                if project_file.exists() {
                    match fs::read_to_string(&project_file) {
                        Ok(json) => {
                            match serde_json::from_str::<NovelProject>(&json) {
                                Ok(project) => projects.push(project),
                                Err(e) => {
                                    tracing::warn!("Failed to parse project at {:?}: {}", path, e);
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to read project file {:?}: {}", project_file, e);
                        }
                    }
                }
            }
        }

        // Sort by creation date (newest first)
        projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(projects)
    }

    /// Create a new storage service for a specific project
    pub fn new_project(base_path: impl Into<PathBuf>, project_id: Uuid) -> Result<Self> {
        let base_path = base_path.into();
        let project_path = base_path.join("projects").join(project_id.to_string());

        // Create project directory structure
        Self::create_project_dirs(&project_path)?;

        Ok(Self {
            base_path: project_path,
            project_id: Some(project_id),
        })
    }

    /// Create project directory structure
    pub fn create_project_dirs(project_path: &PathBuf) -> Result<()> {
        let dirs = ["analysis", "outline", "plans", "chapters"];

        for dir in dirs {
            let path = project_path.join(dir);
            if !path.exists() {
                fs::create_dir_all(&path)
                    .context(format!("Failed to create directory: {:?}", path))?;
            }
        }

        tracing::info!("Created project directories at {:?}", project_path);
        Ok(())
    }

    /// Get base path
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Get path for an entity
    fn entity_path<T: StorageKey>(&self) -> PathBuf {
        let folder = T::storage_folder();
        let filename = T::storage_filename();
        // Add .json extension
        let filename_with_ext = format!("{}.json", filename);
        if folder.is_empty() {
            // Save directly in base_path (for project.json)
            self.base_path.join(filename_with_ext)
        } else {
            self.base_path.join(folder).join(filename_with_ext)
        }
    }

    /// Save an entity
    pub fn save<T: StorageKey + Serialize>(&self, entity: &T) -> Result<()> {
        let path = self.entity_path::<T>();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let json = serde_json::to_string_pretty(entity)
            .context("Failed to serialize entity")?;

        fs::write(&path, json)
            .context("Failed to write entity")?;

        tracing::debug!("Saved entity to {:?}", path);

        Ok(())
    }

    /// Load an entity
    pub fn load<T: StorageKey + DeserializeOwned>(&self) -> Result<Option<T>> {
        let path = self.entity_path::<T>();

        if !path.exists() {
            return Ok(None);
        }

        let json = fs::read_to_string(&path)
            .context("Failed to read entity")?;

        let entity = serde_json::from_str(&json)
            .context("Failed to deserialize entity")?;

        tracing::debug!("Loaded entity from {:?}", path);

        Ok(Some(entity))
    }

    /// Delete an entity
    pub fn delete<T: StorageKey>(&self) -> Result<()> {
        let path = self.entity_path::<T>();

        if path.exists() {
            fs::remove_file(&path)
                .context("Failed to delete entity")?;

            tracing::debug!("Deleted entity from {:?}", path);
        }

        Ok(())
    }
}

/// Trait for storage key
pub trait StorageKey {
    /// Storage folder name
    fn storage_folder() -> &'static str;

    /// Storage filename (without extension)
    fn storage_filename() -> &'static str;
}

// Import models for storage key implementations
use crate::models::{NovelProject, NovelOutline, ChapterPlan, GeneratedChapter, FeasibilityReport};

impl StorageKey for NovelProject {
    fn storage_folder() -> &'static str {
        ""  // Save directly in project folder
    }

    fn storage_filename() -> &'static str {
        "project"
    }
}

impl StorageKey for NovelOutline {
    fn storage_folder() -> &'static str {
        "outline"
    }

    fn storage_filename() -> &'static str {
        "outline"
    }
}

impl StorageKey for ChapterPlan {
    fn storage_folder() -> &'static str {
        "plans"
    }

    fn storage_filename() -> &'static str {
        "plan"
    }
}

impl StorageKey for GeneratedChapter {
    fn storage_folder() -> &'static str {
        "chapters"
    }

    fn storage_filename() -> &'static str {
        "chapter"
    }
}

impl StorageKey for FeasibilityReport {
    fn storage_folder() -> &'static str {
        "analysis"
    }

    fn storage_filename() -> &'static str {
        "feasibility"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::models::NovelGenre;

    #[test]
    fn test_save_load_project() {
        let dir = tempdir().unwrap();
        let storage = StorageService::new(dir.path()).unwrap();

        let project = NovelProject::new(
            "Test Novel".to_string(),
            NovelGenre::Fantasy,
            1_000_000,
        );

        storage.save(&project).unwrap();

        let loaded: Option<NovelProject> = storage.load().unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "Test Novel");
    }
}
