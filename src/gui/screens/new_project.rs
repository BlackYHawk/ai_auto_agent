//! New Project Screen

use egui::{ComboBox, Ui};

use crate::gui::app::{NovelApp, Screen};
use crate::models::NovelGenre;

/// Show the new project screen
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← Back").clicked() {
            app.navigate_to(Screen::Projects);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("Create New Project");

        ui.separator();

        // Project name
        ui.label("Project Name:");
        ui.text_edit_singleline(&mut app.new_project_form.name);

        ui.separator();

        // Genre selection
        ui.label("Genre:");
        ComboBox::from_id_salt("genre_selector")
            .selected_text(format!("{:?}", app.new_project_form.genre.unwrap_or(NovelGenre::Fantasy)))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Fantasy), "Fantasy");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Urban), "Urban");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Xianxia), "Xianxia");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Historical), "Historical");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Romance), "Romance");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Scifi), "Scifi");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Game), "Game");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Horror), "Horror");
            });

        ui.separator();

        // Target word count
        ui.label("Target Word Count:");
        ui.text_edit_singleline(&mut app.new_project_form.target_word_count);

        ui.separator();

        // Create button
        if ui.button("Create Project").clicked() {
            // Validate input
            if app.new_project_form.name.is_empty() {
                app.set_error("Please enter a project name".to_string());
            } else if app.new_project_form.genre.is_none() {
                app.set_error("Please select a genre".to_string());
            } else if app.new_project_form.target_word_count.is_empty() {
                app.set_error("Please enter target word count".to_string());
            } else {
                // Parse target word count
                let target: u64 = match app.new_project_form.target_word_count.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        app.set_error("Please enter a valid number for target word count".to_string());
                        return;
                    }
                };

                // Create project via StorageService
                let genre = app.new_project_form.genre.unwrap();
                let name = app.new_project_form.name.clone();

                match app.create_project(name, genre, target) {
                    Ok(_project) => {
                        // Reset form
                        app.new_project_form = Default::default();
                        app.navigate_to(Screen::Projects);
                    }
                    Err(e) => {
                        app.set_error(format!("Failed to create project: {}", e));
                    }
                }
            }
        }
    });
}
