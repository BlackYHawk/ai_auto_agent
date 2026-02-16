//! 小说导入页面

use egui::{ScrollArea, Ui, RichText};

use crate::gui::app::{NovelApp, Screen};

/// 显示小说导入页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::Projects);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("导入小说");
        ui.add_space(10.0);

        // 说明
        ui.label(RichText::new("支持格式").strong());
        ui.label("• TXT 文本文件");
        ui.label("• 按\"第X章\"自动分割章节");
        ui.label("• UTF-8 编码");

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // 文件选择区域
        ui.label(RichText::new("步骤1: 选择文件").strong());
        ui.add_space(10.0);

        // 模拟文件选择按钮
        ui.horizontal(|ui| {
            if ui.button("选择TXT文件").clicked() {
                // TODO: 实现文件选择对话框
                // 使用 rfd crate 实现原生文件选择
                app.set_error("请在控制台使用命令行导入，或将TXT文件放入项目目录".to_string());
            }
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // 项目信息
        ui.label(RichText::new("步骤2: 设置项目信息").strong());
        ui.add_space(10.0);

        ui.label("项目名称:");
        ui.text_edit_singleline(&mut app.new_project_form.name);

        ui.add_space(10.0);

        // 导入预览
        ui.label(RichText::new("导入预览").strong());
        ui.add_space(10.0);

        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            ui.label("暂无文件预览");
            ui.label("请选择要导入的TXT文件");
            ui.label("");
            ui.label("提示:");
            ui.label("- 文件名将被用作项目名称");
            ui.label("- 系统会自动检测章节标题");
            ui.label("- 支持中英文章节标题格式");
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // 导入按钮
        if ui.button("开始导入").clicked() {
            if app.new_project_form.name.is_empty() {
                app.set_error("请先选择要导入的文件".to_string());
            } else {
                // 模拟导入流程
                app.set_error("导入功能开发中，请使用CLI工具导入".to_string());
            }
        }

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // 使用说明
        ui.label(RichText::new("使用方法").strong());
        ui.add_space(5.0);
        ui.label("1. 将已有的TXT小说文件准备好");
        ui.label("2. 点击\"选择TXT文件\"按钮");
        ui.label("3. 系统会自动解析章节结构");
        ui.label("4. 设置项目信息并导入");
        ui.label("5. 导入后可继续在GUI中编辑生成");
    });
}
