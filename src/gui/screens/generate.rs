//! 章节生成页面

use egui::{ProgressBar, ScrollArea, Ui};

use crate::gui::app::{NovelApp, Screen, TaskState};

/// 显示章节生成页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::ProjectDetail);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("生成章节");

        ui.separator();

        // 章节范围输入
        ui.label("章节范围 (如: 1-10):");
        ui.text_edit_singleline(&mut app.generate_form.chapter_range);

        ui.separator();

        // 生成按钮
        if ui.button("生成").clicked() {
            if app.generate_form.chapter_range.is_empty() {
                app.set_error("请输入章节范围".to_string());
            } else {
                // 解析章节范围
                let parts: Vec<&str> = app.generate_form.chapter_range.split('-').collect();
                let (start, end) = if parts.len() == 2 {
                    (parts[0].trim().parse::<u32>(), parts[1].trim().parse::<u32>())
                } else if parts.len() == 1 {
                    let n = parts[0].trim().parse::<u32>();
                    let m = parts[0].trim().parse::<u32>();
                    (n, m)
                } else {
                    app.set_error("格式无效，请使用 '1-10' 或 '5'".to_string());
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
                        app.set_error("无效的章节范围".to_string());
                    }
                }
            }
        }

        // 显示进度
        if let Some(task_state) = app.running_tasks.get("generate") {
            match task_state {
                TaskState::Running { progress, message } => {
                    ui.separator();
                    ui.label(message);
                    ui.add(ProgressBar::new(*progress));
                }
                TaskState::Failed { error } => {
                    ui.separator();
                    ui.label(format!("错误: {}", error));
                }
                TaskState::Completed => {
                    ui.separator();
                    ui.label("生成完成!");
                }
                _ => {}
            }
        }

        // 显示生成的内容
        ui.separator();
        ui.label("生成内容:");
        if let Some(ref result) = app.chapter_result {
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.label(result);
            });
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("输入章节范围并点击生成...");
            });
        }
    });
}
