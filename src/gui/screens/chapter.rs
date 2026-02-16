//! 章节查看页面

use egui::{ScrollArea, Ui, RichText};

use crate::gui::app::{NovelApp, Screen};

/// 显示章节查看页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    // 获取选中的项目ID和章节号
    let _project_id = app.selected_project_id;
    let chapter_number = app.selected_chapter_number;

    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::ProjectDetail);
        }

        ui.separator();

        // 章节列表
        ui.label(RichText::new("章节列表").strong());

        // 生成一些示例章节（实际应该从存储加载）
        let total_chapters = 40; // 默认40章
        for i in 1..=total_chapters {
            let is_selected = chapter_number == Some(i);
            if ui.selectable_label(is_selected, format!("第{}章", i)).clicked() {
                app.selected_chapter_number = Some(i);
            }
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(chapter_num) = chapter_number {
            // 显示章节标题
            ui.heading(format!("第{}章", chapter_num));
            ui.add_space(10.0);

            // 章节元信息
            egui::CollapsingHeader::new("章节信息")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("字数: ");
                        ui.label("约 3000 字");
                    });
                    ui.horizontal(|ui| {
                        ui.label("状态: ");
                        ui.label("已完成");
                    });
                    ui.horizontal(|ui| {
                        ui.label("创建时间: ");
                        ui.label("2024-01-01 10:00");
                    });
                });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // 章节内容
            ui.label(RichText::new("章节内容").strong());
            ui.add_space(5.0);

            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                // 示例章节内容
                let content = format!(
                    "第{}章内容预览\n\n这是第{}章的正文内容。在实际系统中，章节内容会从项目目录中的chapters/文件夹加载。\n\n每个章节以JSON格式存储，包含：\n- 标题 (title)\n- 正文 (content)\n- 字数 (word_count)\n- 状态 (status)\n- 创建时间 (created_at)\n\n您可以在项目详情页面点击生成章节来创建新章节。",
                    chapter_num, chapter_num
                );
                ui.label(content);
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // 编辑按钮
            ui.horizontal(|ui| {
                if ui.button("编辑章节").clicked() {
                    // TODO: 实现编辑功能
                }
                if ui.button("删除章节").clicked() {
                    // TODO: 实现删除功能
                }
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(RichText::new("请选择章节").size(20.0));
                ui.add_space(20.0);
                ui.label("从左侧列表选择要查看的章节");
            });
        }
    });
}
