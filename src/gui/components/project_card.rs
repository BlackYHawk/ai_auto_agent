//! Project Card Component

use egui::{Frame, Ui, Vec2};

use crate::models::NovelProject;

/// Show a project card
pub fn show(ui: &mut Ui, project: &NovelProject) {
    Frame::default()
        .inner_margin(Vec2::new(8.0, 8.0))
        .show(ui, |ui| {
            ui.label(&project.name);
            ui.label(format!("Genre: {:?}", project.genre));
            ui.label(format!(
                "Target: {} words",
                project.target_word_count
            ));
            ui.label(format!("Status: {:?}", project.status));
        });
}
