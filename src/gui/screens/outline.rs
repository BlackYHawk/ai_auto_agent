//! 大纲生成页面

use egui::{ScrollArea, Ui};

use crate::gui::app::{NovelApp, Screen};

/// 显示大纲生成页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::ProjectDetail);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("生成大纲");

        ui.separator();

        // 小说设定输入
        ui.label("小说设定:");
        ui.text_edit_multiline(&mut app.outline_form.premise);

        ui.separator();

        // 主题输入
        ui.label("主题 (可选):");
        ui.text_edit_singleline(&mut app.outline_form.theme);

        ui.separator();

        // 生成按钮
        if ui.button("生成大纲").clicked() {
            if app.outline_form.premise.is_empty() {
                app.set_error("请输入小说设定".to_string());
            } else {
                // 获取选中项目的类型
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

        // 显示生成结果
        if let Some(ref result) = app.outline_result {
            ui.separator();
            ui.label("生成的大纲:");
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                ui.label(result);
            });
        }
    });
}
