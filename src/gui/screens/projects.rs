//! Projects List Screen

use egui::{Ui, RichText};

use crate::gui::app::{NovelApp, Screen};
use crate::models::{NovelGenre, ProjectStatus};

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

/// Show the projects list screen
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    // Left sidebar
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("项目列表").clicked() {
            app.navigate_to(Screen::Projects);
        }

        if ui.button("新建项目").clicked() {
            app.navigate_to(Screen::NewProject);
        }

        ui.separator();

        if ui.button("导入小说").clicked() {
            app.navigate_to(Screen::Import);
        }

        ui.separator();

        // Settings button
        if ui.button("设置").clicked() {
            app.navigate_to(Screen::Settings);
        }

        ui.separator();

        // Exit button
        if ui.button("退出").clicked() {
            std::process::exit(0);
        }
    });

    // Main content area
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("项目列表");
        ui.add_space(10.0);

        // Refresh button
        if ui.button("刷新").clicked() {
            if let Err(e) = app.load_projects() {
                app.set_error(e);
            }
        }

        ui.separator();
        ui.add_space(10.0);

        // Load projects if not loaded
        if app.projects.is_empty() {
            if let Err(e) = app.load_projects() {
                app.set_error(e);
            }
        }

        if app.projects.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(RichText::new("暂无项目").size(20.0));
                ui.add_space(20.0);
                if ui.button("创建新项目").clicked() {
                    app.navigate_to(Screen::NewProject);
                }
            });
        } else {
            // Show project count
            ui.horizontal(|ui| {
                ui.label(format!("共 {} 个项目", app.projects.len()));
            });

            ui.add_space(10.0);

            // Project cards grid
            egui::ScrollArea::vertical().show(ui, |ui| {
                let item_width = 280.0;
                let spacing = 10.0;

                // Calculate columns based on available width
                let available_width = ui.available_width();
                let columns = ((available_width / (item_width + spacing)).floor() as usize).max(1);

                egui::Grid::new("projects_grid")
                    .num_columns(columns)
                    .spacing([spacing, spacing])
                    .show(ui, |ui| {
                        // Collect project data to avoid borrow issues
                        let project_data: Vec<_> = app.projects.iter().enumerate().map(|(i, p)| {
                            (i, p.id, p.name.clone(), p.genre, p.target_word_count, p.status, p.created_at)
                        }).collect();

                        for (i, project_id, name, genre, target, status, created_at) in project_data {
                            // Create a card for each project using collapsing header
                            let card_response = ui.collapsing(
                                name,
                                |ui| {
                                    ui.vertical(|ui| {
                                        // Genre
                                        ui.horizontal(|ui| {
                                            ui.label("类型: ");
                                            ui.label(genre_to_chinese(genre));
                                        });

                                        // Target word count
                                        ui.horizontal(|ui| {
                                            ui.label("目标: ");
                                            ui.label(format!("{:?} 字", target));
                                        });

                                        // Status
                                        ui.horizontal(|ui| {
                                            ui.label("状态: ");
                                            ui.label(status_to_chinese(status));
                                        });

                                        // Created date
                                        ui.horizontal(|ui| {
                                            ui.label("创建: ");
                                            let date = created_at.format("%Y-%m-%d");
                                            ui.label(date.to_string());
                                        });
                                    });
                                }
                            );

                            if card_response.header_response.clicked() {
                                app.selected_project_id = Some(project_id);
                                app.navigate_to(Screen::ProjectDetail);
                            }

                            // Add to grid properly
                            if (i + 1) % columns == 0 {
                                ui.end_row();
                            }
                        }
                    });
            });
        }
    });
}
