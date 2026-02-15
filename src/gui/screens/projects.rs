//! Projects List Screen

use egui::Ui;

use crate::gui::app::{NovelApp, Screen};

/// Show the projects list screen
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("Projects").clicked() {
            app.navigate_to(Screen::Projects);
        }

        if ui.button("New Project").clicked() {
            app.navigate_to(Screen::NewProject);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("Projects");

        ui.separator();

        if app.projects.is_empty() {
            ui.label("No projects yet. Create a new project to get started.");
            if ui.button("Create Project").clicked() {
                app.navigate_to(Screen::NewProject);
            }
        } else {
            // Show project list - clone data to avoid borrow issues
            let project_count = app.projects.len();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..project_count {
                    let project_name = app.projects[i].name.clone();
                    let project_id = app.projects[i].id;
                    ui.horizontal(|ui| {
                        if ui.button(&project_name).clicked() {
                            app.selected_project_id = Some(project_id);
                            app.navigate_to(Screen::ProjectDetail);
                        }
                    });
                }
            });
        }
    });
}
