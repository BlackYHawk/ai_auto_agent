//! Project Detail Screen

use egui::Ui;

use crate::gui::app::{NovelApp, Screen};

/// Show the project detail screen
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    // Get the selected project ID first
    let project_id = app.selected_project_id;

    // Get project data if available
    let project_data = project_id.and_then(|id| {
        app.projects.iter().find(|p| p.id == id).cloned()
    });

    // Get the selected project
    let project = match project_data {
        Some(p) => p,
        None => {
            ui.label("No project selected");
            if ui.button("Back to Projects").clicked() {
                app.navigate_to(Screen::Projects);
            }
            return;
        }
    };

    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("Project");

        ui.separator();

        if ui.button("← Back").clicked() {
            app.navigate_to(Screen::Projects);
        }

        ui.separator();

        // Show project info
        ui.label(format!("Name: {}", project.name));
        ui.label(format!("Genre: {:?}", project.genre));
        ui.label(format!("Target: {} words", project.target_word_count));
        ui.label(format!("Status: {:?}", project.status));
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading(&project.name);

        ui.separator();

        // Show action buttons
        ui.label("Actions:");

        ui.horizontal(|ui| {
            if ui.button("Run Analysis").clicked() {
                // Run feasibility analysis
                match app.run_feasibility_analysis(project.genre) {
                    Ok(result) => {
                        app.analysis_result = Some(result);
                    }
                    Err(e) => {
                        app.set_error(e);
                    }
                }
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Generate Outline").clicked() {
                app.navigate_to(Screen::Outline);
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Generate Chapters").clicked() {
                app.navigate_to(Screen::Generate);
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Publish").clicked() {
                app.navigate_to(Screen::Publish);
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Check Consistency").clicked() {
                app.navigate_to(Screen::Check);
            }
        });

        // Show analysis result if available
        if let Some(ref result) = app.analysis_result {
            ui.separator();
            ui.label("Analysis Result:");
            ui.label(result);
        }
    });
}
