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

        // 显示当前选择的文件
        if !app.import_form.file_name.is_empty() {
            ui.horizontal(|ui| {
                ui.label("已选择: ");
                ui.label(&app.import_form.file_name);
            });
            ui.add_space(5.0);
        }

        // 文件选择按钮
        ui.horizontal(|ui| {
            if ui.button("选择TXT文件").clicked() {
                // 使用 rfd 打开文件对话框
                let picker = rfd::FileDialog::new()
                    .add_filter("Text files", &["txt"])
                    .set_title("选择小说文件");

                if let Some(path_buf) = picker.pick_file() {
                    let path_buf: std::path::PathBuf = path_buf;
                    let path_str = path_buf.to_string_lossy().to_string();
                    app.import_form.file_path = path_str.clone();

                    // 获取文件名
                    if let Some(name) = path_buf.file_name() {
                        let name: &std::ffi::OsStr = name;
                        app.import_form.file_name = name.to_string_lossy().to_string();
                    }

                    // 读取文件内容预览
                    if let Ok(content) = std::fs::read_to_string(&path_str) {
                        // 预览前1000个字符
                        let preview = if content.len() > 1000 {
                            format!("{}...", &content[..1000])
                        } else {
                            content.clone()
                        };
                        app.import_form.content_preview = preview;

                        // 统计章节数 (简化版)
                        let chapter_count = content
                            .lines()
                            .filter(|line| line.trim().starts_with("第") && line.contains("章"))
                            .count()
                            .max(1);
                        app.import_form.chapter_count = chapter_count;
                    }
                }
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

        // 自动填充项目名称
        if app.new_project_form.name.is_empty() && !app.import_form.file_name.is_empty() {
            // 去除文件扩展名
            let name = app.import_form.file_name
                .trim_end_matches(".txt")
                .trim_end_matches(".TXT");
            app.new_project_form.name = name.to_string();
        }

        ui.add_space(10.0);

        // 导入预览
        ui.label(RichText::new("导入预览").strong());
        ui.add_space(10.0);

        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            if app.import_form.file_name.is_empty() {
                ui.label("暂无文件预览");
                ui.label("请选择要导入的TXT文件");
            } else {
                ui.label(format!("文件: {}", app.import_form.file_name));
                ui.label(format!("预估章节数: {}", app.import_form.chapter_count));
                ui.add_space(10.0);
                ui.label("内容预览:");
                ui.label(&app.import_form.content_preview);
            }
        });

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // 导入按钮
        if ui.button("开始导入").clicked() {
            if app.import_form.file_path.is_empty() {
                app.set_error("请先选择要导入的文件".to_string());
            } else if app.new_project_form.name.is_empty() {
                app.set_error("请输入项目名称".to_string());
            } else {
                // 执行导入
                match app.import_novel() {
                    Ok(_) => {
                        // 重置表单
                        app.import_form = Default::default();
                        app.new_project_form = Default::default();
                        app.navigate_to(Screen::Projects);
                    }
                    Err(e) => {
                        app.set_error(format!("导入失败: {}", e));
                    }
                }
            }
        }

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // 使用说明
        ui.label(RichText::new("使用方法").strong());
        ui.add_space(5.0);
        ui.label("1. 点击\"选择TXT文件\"按钮");
        ui.label("2. 选择要导入的小说文件");
        ui.label("3. 系统会自动解析章节结构");
        ui.label("4. 设置项目名称并导入");
        ui.label("5. 导入后可继续在GUI中编辑生成");
    });
}
