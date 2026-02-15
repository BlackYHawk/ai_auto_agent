//! Consistency Check Screen

use egui::{ProgressBar, ScrollArea, Ui};

use crate::gui::app::{NovelApp, Screen, TaskState};

/// Show the consistency check screen
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← Back").clicked() {
            app.navigate_to(Screen::ProjectDetail);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("Consistency Check");

        ui.separator();

        // Check button
        if ui.button("Run Consistency Check").clicked() {
            let project_id = app.selected_project_id.unwrap_or_default();
            match app.run_consistency_check(project_id) {
                Ok(result) => {
                    app.outline_result = Some(result);
                    app.running_tasks.insert(
                        "check".to_string(),
                        TaskState::Completed,
                    );
                }
                Err(e) => {
                    app.set_error(e.clone());
                    app.running_tasks.insert(
                        "check".to_string(),
                        TaskState::Failed { error: e },
                    );
                }
            }
        }

        // Show progress if running
        if let Some(task_state) = app.running_tasks.get("check") {
            match task_state {
                TaskState::Running { progress, message } => {
                    ui.separator();
                    ui.label(message);
                    ui.add(ProgressBar::new(*progress));
                }
                TaskState::Completed => {
                    ui.separator();
                    ui.label("Check completed!");
                }
                TaskState::Failed { error } => {
                    ui.separator();
                    ui.label(format!("Error: {}", error));
                }
                _ => {}
            }
        }

        // Show results
        ui.separator();
        ui.label("Consistency Report:");
        if let Some(ref result) = app.outline_result {
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.label(result);
            });
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("Click 'Run Consistency Check' to analyze your project...");
            });
        }
    });
}
