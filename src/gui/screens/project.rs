//! 项目详情页面

use egui::{ComboBox, ScrollArea, Ui, RichText};

use crate::gui::app::{NovelApp, Screen};
use crate::models::{NovelGenre, ProjectStatus, PublicationStatus};

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

/// 将状态转换为中文
fn status_to_chinese(status: ProjectStatus) -> &'static str {
    match status {
        ProjectStatus::Draft => "草稿",
        ProjectStatus::Feasibility => "可行性分析",
        ProjectStatus::Outline => "大纲",
        ProjectStatus::Planning => "规划中",
        ProjectStatus::Generating => "生成中",
        ProjectStatus::Publishing => "发布中",
        ProjectStatus::Completed => "已完成",
    }
}

/// 将发布状态转换为中文
fn pub_status_to_chinese(status: PublicationStatus) -> &'static str {
    match status {
        PublicationStatus::NotPublished => "未发布",
        PublicationStatus::Created => "已创建",
        PublicationStatus::Publishing => "发布中",
        PublicationStatus::Published => "已发布",
    }
}

/// 显示项目详情页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    // 获取选中的项目ID
    let project_id = app.selected_project_id;

    // 获取项目数据
    let project_data = project_id.and_then(|id| {
        app.projects.iter().find(|p| p.id == id).cloned()
    });

    // 获取选中的项目
    let project = match project_data {
        Some(p) => p,
        None => {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(RichText::new("未选择项目").size(20.0));
                ui.add_space(20.0);
                if ui.button("返回项目列表").clicked() {
                    app.navigate_to(Screen::Projects);
                }
            });
            return;
        }
    };

    // 左侧导航栏
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::Projects);
        }

        if ui.button("项目详情").clicked() {
            // 当前页面
        }

        ui.separator();

        // 项目基本信息
        ui.label(RichText::new("项目信息").strong());
        ui.label(format!("名称: {}", project.name));
        ui.label(format!("类型: {:?}", project.genre));
        ui.label(format!("目标: {} 字", project.target_word_count));
        ui.label(format!("状态: {:?}", project.status));

        ui.separator();

        // 操作菜单
        ui.label(RichText::new("操作").strong());

        if ui.button("可行性分析").clicked() {
            match app.run_feasibility_analysis(project.genre) {
                Ok(result) => {
                    app.analysis_result = Some(result);
                }
                Err(e) => {
                    app.set_error(e);
                }
            }
        }

        if ui.button("生成大纲").clicked() {
            app.navigate_to(Screen::Outline);
        }

        if ui.button("生成章节").clicked() {
            app.navigate_to(Screen::Generate);
        }

        if ui.button("发布").clicked() {
            app.navigate_to(Screen::Publish);
        }

        if ui.button("一致性检查").clicked() {
            app.navigate_to(Screen::Check);
        }
    });

    // 主内容区域
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading(&project.name);
        ui.add_space(10.0);

        // 项目基本信息卡片
        egui::CollapsingHeader::new("项目信息")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("类型: ");
                    ui.label(genre_to_chinese(project.genre));
                });
                ui.horizontal(|ui| {
                    ui.label("目标字数: ");
                    ui.label(format!("{} 字", project.target_word_count));
                });
                ui.horizontal(|ui| {
                    ui.label("当前状态: ");
                    ui.label(status_to_chinese(project.status));
                });
                ui.horizontal(|ui| {
                    ui.label("创建时间: ");
                    ui.label(project.created_at.format("%Y-%m-%d %H:%M").to_string());
                });
                ui.horizontal(|ui| {
                    ui.label("更新时间: ");
                    ui.label(project.updated_at.format("%Y-%m-%d %H:%M").to_string());
                });
            });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // 可行性研究报告
        egui::CollapsingHeader::new("可行性研究报告")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(ref result) = app.analysis_result {
                    ui.label(result);
                } else {
                    ui.label("点击左侧\"可行性分析\"按钮生成报告");
                }
            });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // 小说大纲
        egui::CollapsingHeader::new("小说大纲")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(ref result) = app.outline_result {
                    ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        ui.label(result);
                    });
                } else {
                    ui.label("点击左侧\"生成大纲\"按钮创建大纲");
                }
            });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // 章节列表
        egui::CollapsingHeader::new("章节列表")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(ref result) = app.chapter_result {
                    ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                        ui.label(result);
                    });
                } else {
                    ui.label("暂无章节内容");
                }
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if ui.button("查看章节").clicked() {
                        app.navigate_to(Screen::Chapter);
                    }
                    if ui.button("生成章节").clicked() {
                        app.navigate_to(Screen::Generate);
                    }
                });
            });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // 发布状态
        egui::CollapsingHeader::new("发布状态")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("发布状态: ");
                    ui.label(pub_status_to_chinese(project.publication_status));
                });
                if let Some(ref novel_id) = project.fanqie_novel_id {
                    ui.horizontal(|ui| {
                        ui.label("番茄小说ID: ");
                        ui.label(novel_id);
                    });
                }
            });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // 项目级模型配置
        egui::CollapsingHeader::new("模型配置")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("启用项目级配置: ");
                    ui.label(if project.model_config.enabled { "是" } else { "否" });
                });

                if project.model_config.enabled {
                    ui.horizontal(|ui| {
                        ui.label("当前模型: ");
                        ui.label(project.model_config.model.as_deref().unwrap_or("默认"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("提供商: ");
                        ui.label(project.model_config.provider.as_deref().unwrap_or("默认"));
                    });
                }

                ui.add_space(10.0);
                ui.label("切换模型:");

                // 模型选择下拉框
                let mut selected_model = project.model_config.model.clone().unwrap_or_else(|| "minimax".to_string());
                ComboBox::from_id_salt("project_model_selector")
                    .selected_text(&selected_model)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected_model, "minimax".to_string(), "MiniMax (abab6.5s)");
                        ui.selectable_value(&mut selected_model, "qwen".to_string(), "Qwen (通义千问)");
                        ui.selectable_value(&mut selected_model, "gpt4".to_string(), "OpenAI (GPT-4)");
                        ui.selectable_value(&mut selected_model, "claude".to_string(), "Anthropic (Claude)");
                    });

                ui.add_space(10.0);
                if ui.button("应用模型配置").clicked() {
                    // TODO: 保存项目级模型配置
                    app.set_error("项目级模型配置已更新".to_string());
                }
            });
    });
}
