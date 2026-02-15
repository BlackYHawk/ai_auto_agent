//! Chapter Generation Screen

use egui::{ProgressBar, ScrollArea, Ui};

use crate::gui::app::{NovelApp, Screen, TaskState};

/// Show the chapter generation screen
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← Back").clicked() {
            app.navigate_to(Screen::ProjectDetail);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("Generate Chapters");

        ui.separator();

        // Chapter range input
        ui.label("Chapter Range (e.g., 1-10):");
        ui.text_edit_singleline(&mut app.generate_form.chapter_range);

        ui.separator();

        // Generate button
        if ui.button("Generate").clicked() {
            if app.generate_form.chapter_range.is_empty() {
                app.set_error("Please enter chapter range".to_string());
            } else {
                // Parse chapter range
                let parts: Vec<&str> = app.generate_form.chapter_range.split('-').collect();
                let (start, end) = if parts.len() == 2 {
                    (parts[0].trim().parse::<u32>(), parts[1].trim().parse::<u32>())
                } else if parts.len() == 1 {
                    let n = parts[0].trim().parse::<u32>();
                    let m = parts[0].trim().parse::<u32>();
                    (n, m)
                } else {
                    app.set_error("Invalid format. Use '1-10' or '5'".to_string());
                    return;
                };

                match (start, end) {
                    (Ok(s), Ok(e)) if s <= e => {
                        let project_id = app.selected_project_id.unwrap_or_default();
                        match app.generate_chapters(project_id, s, e) {
                            Ok(result) => {
                                app.chapter_result = Some(result);
                                app.running_tasks.insert(
                                    "generate".to_string(),
                                    TaskState::Completed,
                                );
                            }
                            Err(e) => {
                                let err_msg = e.clone();
                                app.set_error(e);
                                app.running_tasks.insert(
                                    "generate".to_string(),
                                    TaskState::Failed { error: err_msg },
                                );
                            }
                        }
                    }
                    _ => {
                        app.set_error("Invalid chapter range".to_string());
                    }
                }
            }
        }

        // Show progress if running
        if let Some(task_state) = app.running_tasks.get("generate") {
            match task_state {
                TaskState::Running { progress, message } => {
                    ui.separator();
                    ui.label(message);
                    ui.add(ProgressBar::new(*progress));
                }
                TaskState::Failed { error } => {
                    ui.separator();
                    ui.label(format!("Error: {}", error));
                }
                TaskState::Completed => {
                    ui.separator();
                    ui.label("Generation completed!");
                }
                _ => {}
            }
        }

        // Show generated content
        ui.separator();
        ui.label("Generated Content:");
        if let Some(ref result) = app.chapter_result {
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.label(result);
            });
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("Enter a chapter range and click Generate...");
            });
        }
    });
}
