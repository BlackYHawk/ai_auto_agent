//! 新建项目页面

use egui::{ComboBox, Ui};

use crate::gui::app::{NovelApp, Screen};
use crate::models::NovelGenre;

/// 将类型转换为中文
fn genre_to_chinese(genre: NovelGenre) -> &'static str {
    match genre {
        NovelGenre::Fantasy => "奇幻",
        NovelGenre::Urban => "都市",
        NovelGenre::Xianxia => "仙侠",
        NovelGenre::Historical => "历史",
        NovelGenre::Romance => "言情",
        NovelGenre::Scifi => "科幻",
        NovelGenre::Game => "游戏",
        NovelGenre::Horror => "悬疑",
        NovelGenre::Other => "其他",
    }
}

/// 显示新建项目页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::Projects);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("新建项目");

        ui.separator();

        // 项目名称
        ui.label("项目名称:");
        ui.text_edit_singleline(&mut app.new_project_form.name);

        ui.separator();

        // 类型选择
        ui.label("小说类型:");
        ComboBox::from_id_salt("genre_selector")
            .selected_text(genre_to_chinese(app.new_project_form.genre.unwrap_or(NovelGenre::Fantasy)))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Fantasy), "奇幻");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Urban), "都市");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Xianxia), "仙侠");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Historical), "历史");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Romance), "言情");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Scifi), "科幻");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Game), "游戏");
                ui.selectable_value(&mut app.new_project_form.genre, Some(NovelGenre::Horror), "悬疑");
            });

        ui.separator();

        // 目标字数
        ui.label("目标字数:");
        ui.text_edit_singleline(&mut app.new_project_form.target_word_count);

        ui.separator();

        // 创建按钮
        if ui.button("创建项目").clicked() {
            // 验证输入
            if app.new_project_form.name.is_empty() {
                app.set_error("请输入项目名称".to_string());
            } else if app.new_project_form.genre.is_none() {
                app.set_error("请选择小说类型".to_string());
            } else if app.new_project_form.target_word_count.is_empty() {
                app.set_error("请输入目标字数".to_string());
            } else {
                // 解析目标字数
                let target: u64 = match app.new_project_form.target_word_count.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        app.set_error("请输入有效的数字".to_string());
                        return;
                    }
                };

                // 通过 StorageService 创建项目
                let genre = app.new_project_form.genre.unwrap();
                let name = app.new_project_form.name.clone();

                match app.create_project(name, genre, target) {
                    Ok(_project) => {
                        // 重置表单
                        app.new_project_form = Default::default();
                        app.navigate_to(Screen::Projects);
                    }
                    Err(e) => {
                        app.set_error(format!("创建项目失败: {}", e));
                    }
                }
            }
        }
    });
}
