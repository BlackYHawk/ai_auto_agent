//! File-based storage service

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;

/// Storage service for persisting data
pub struct StorageService {
    base_path: PathBuf,
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

        Ok(Self { base_path })
    }

    /// Get path for an entity
    fn entity_path<T: StorageKey>(&self) -> PathBuf {
        self.base_path.join(T::storage_folder()).join(T::storage_filename())
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
        "projects"
    }

    fn storage_filename() -> &'static str {
        "project"
    }
}

impl StorageKey for NovelOutline {
    fn storage_folder() -> &'static str {
        "outlines"
    }

    fn storage_filename() -> &'static str {
        "outline"
    }
}

impl StorageKey for ChapterPlan {
    fn storage_folder() -> &'static str {
        "chapter_plans"
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
        "feasibility"
    }

    fn storage_filename() -> &'static str {
        "report"
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
