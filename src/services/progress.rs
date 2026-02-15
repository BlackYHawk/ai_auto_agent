//! Progress Tracking Service

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// Progress tracker for long-running operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressTracker {
    pub total: u32,
    pub completed: u32,
    pub failed: u32,
    pub status: ProgressStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProgressStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl ProgressTracker {
    pub fn new(total: u32) -> Self {
        Self {
            total,
            completed: 0,
            failed: 0,
            status: ProgressStatus::Pending,
        }
    }

    pub fn start(&mut self) {
        self.status = ProgressStatus::Running;
    }

    pub fn increment_completed(&mut self) {
        self.completed += 1;
        if self.completed >= self.total {
            self.status = ProgressStatus::Completed;
        }
    }

    pub fn increment_failed(&mut self) {
        self.failed += 1;
    }

    pub fn progress_percentage(&self) -> f32 {
        if self.total == 0 {
            return 100.0;
        }
        (self.completed as f32 / self.total as f32) * 100.0
    }
}
