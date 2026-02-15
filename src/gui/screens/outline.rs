//! Outline Generation Screen

use egui::{ScrollArea, Ui};

use crate::gui::app::{NovelApp, Screen};

/// Show the outline generation screen
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← Back").clicked() {
            app.navigate_to(Screen::ProjectDetail);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("Generate Outline");

        ui.separator();

        // Premise input
        ui.label("Premise:");
        ui.text_edit_multiline(&mut app.outline_form.premise);

        ui.separator();

        // Theme input
        ui.label("Theme (optional):");
        ui.text_edit_singleline(&mut app.outline_form.theme);

        ui.separator();

        // Generate button
        if ui.button("Generate Outline").clicked() {
            if app.outline_form.premise.is_empty() {
                app.set_error("Please enter a premise".to_string());
            } else {
                // Get selected project for genre
                let genre = app.selected_project()
                    .map(|p| p.genre)
                    .unwrap_or(crate::models::NovelGenre::Fantasy);

                let theme = if app.outline_form.theme.is_empty() {
                    "成长与挑战".to_string()
                } else {
                    app.outline_form.theme.clone()
                };

                let project_id = app.selected_project_id.unwrap_or_default();

                match app.generate_outline(project_id, genre, app.outline_form.premise.clone(), theme, 1_000_000) {
                    Ok(result) => {
                        app.outline_result = Some(result);
                    }
                    Err(e) => {
                        app.set_error(e);
                    }
                }
            }
        }

        // Show result if available
        if let Some(ref result) = app.outline_result {
            ui.separator();
            ui.label("Generated Outline:");
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.label(result);
            });
        }
    });
}
