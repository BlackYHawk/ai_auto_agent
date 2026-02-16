//! 发布页面

use egui::{ComboBox, ScrollArea, Ui};

use crate::gui::app::{NovelApp, PublishAction, Screen, TaskState};

/// 显示发布页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::ProjectDetail);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("发布到番茄小说");

        ui.separator();

        // 操作选择
        ui.label("发布操作:");
        ComboBox::from_id_salt("publish_action")
            .selected_text(app.publish_form.action.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.publish_form.action, PublishAction::Create, "创建小说");
                ui.selectable_value(&mut app.publish_form.action, PublishAction::Upload, "上传章节");
                ui.selectable_value(&mut app.publish_form.action, PublishAction::Submit, "提交审核");
            });

        ui.separator();

        // 章节范围
        ui.label("章节范围 (如: 1-10):");
        ui.text_edit_singleline(&mut app.publish_form.chapter_range);

        ui.separator();

        // 发布按钮
        if ui.button("发布").clicked() {
            let project_id = app.selected_project_id.unwrap_or_default();
            let action = app.publish_form.action.clone();
            let chapter_range = app.publish_form.chapter_range.clone();

            if chapter_range.is_empty() {
                app.set_error("请输入章节范围".to_string());
            } else {
                match app.publish_to_fanqie(project_id, action, chapter_range) {
                    Ok(result) => {
                        app.publish_result = Some(result);
                        app.running_tasks.insert(
                            "publish".to_string(),
                            TaskState::Completed,
                        );
                    }
                    Err(e) => {
                        let err_msg = e.clone();
                        app.set_error(e);
                        app.running_tasks.insert(
                            "publish".to_string(),
                            TaskState::Failed { error: err_msg },
                        );
                    }
                }
            }
        }

        // 显示进度/状态
        if let Some(task_state) = app.running_tasks.get("publish") {
            match task_state {
                TaskState::Running { progress, message } => {
                    ui.separator();
                    ui.label(message);
                    ui.add(egui::ProgressBar::new(*progress));
                }
                TaskState::Completed => {
                    ui.separator();
                    ui.label("发布完成!");
                }
                TaskState::Failed { error } => {
                    ui.separator();
                    ui.label(format!("错误: {}", error));
                }
                _ => {}
            }
        }

        ui.separator();

        // 显示发布结果
        ui.label("发布状态:");
        if let Some(ref result) = app.publish_result {
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.label(result);
            });
        } else {
            ScrollArea::vertical().show(ui, |ui| {
                ui.label("输入章节范围并点击发布...\n\n注意: 请在 config.toml 中配置番茄小说cookies以完成完整集成。");
            });
        }
    });
}
