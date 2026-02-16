//! 设置页面 - LLM模型配置

use egui::{ComboBox, Ui};

use crate::gui::app::{NovelApp, Screen};

/// 显示设置页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    egui::SidePanel::left("left_panel").min_width(200.0).show_inside(ui, |ui| {
        ui.heading("AI Novel Agent");

        ui.separator();

        if ui.button("← 返回").clicked() {
            app.navigate_to(Screen::Projects);
        }
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("设置");
        ui.add_space(10.0);

        // 显示当前配置状态
        let has_api_key = !app.config.llm.api_key.is_empty();
        let config_status = if has_api_key {
            "✓ 已配置"
        } else {
            "✗ 未配置"
        };
        let status_color = if has_api_key { egui::Color32::GREEN } else { egui::Color32::RED };
        ui.label(egui::RichText::new(format!("LLM配置状态: {}", config_status)).color(status_color));
        ui.add_space(5.0);
        ui.label(format!("当前提供商: {}", app.config.llm.provider));
        if let Some(ref model) = app.config.llm.model {
            ui.label(format!("当前模型: {}", model));
        }

        ui.add_space(15.0);

        // LLM配置区域
        egui::CollapsingHeader::new("LLM 模型配置")
            .default_open(true)
            .show(ui, |ui| {
                ui.add_space(5.0);

                // 获取可变引用
                let provider = app.config.llm.provider.clone();
                let mut provider_selected = provider.clone();

                // 提供商选择
                ui.label("模型提供商:");
                ComboBox::from_id_salt("provider_selector")
                    .selected_text(&provider_selected)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut provider_selected, "minimax".to_string(), "MiniMax");
                        ui.selectable_value(&mut provider_selected, "qwen".to_string(), "Qwen (阿里)");
                        ui.selectable_value(&mut provider_selected, "openai".to_string(), "OpenAI");
                        ui.selectable_value(&mut provider_selected, "anthropic".to_string(), "Anthropic");
                    });

                // 如果提供商改变了，更新配置
                if provider_selected != provider {
                    app.config.llm.provider = provider_selected;
                }

                ui.add_space(10.0);

                // API Key
                let mut api_key = app.config.llm.api_key.clone();
                ui.label("API Key:");
                ui.text_edit_singleline(&mut api_key);
                app.config.llm.api_key = api_key;

                ui.add_space(10.0);

                // 模型选择
                let mut model = app.config.llm.model.clone().unwrap_or_default();
                ui.label("模型名称:");
                ui.text_edit_singleline(&mut model);
                app.config.llm.model = if model.is_empty() { None } else { Some(model) };

                ui.add_space(10.0);

                // Temperature
                let mut temperature = app.config.llm.temperature.to_string();
                ui.label("Temperature (0.0-1.0):");
                ui.text_edit_singleline(&mut temperature);
                if let Ok(t) = temperature.parse::<f32>() {
                    app.config.llm.temperature = t;
                }

                ui.add_space(10.0);

                // Max Tokens
                let mut max_tokens = app.config.llm.max_tokens.to_string();
                ui.label("Max Tokens:");
                ui.text_edit_singleline(&mut max_tokens);
                if let Ok(mt) = max_tokens.parse::<u32>() {
                    app.config.llm.max_tokens = mt;
                }

                ui.add_space(15.0);

                // 保存按钮
                if ui.button("保存配置").clicked() {
                    match crate::config::save_config(&app.config, std::path::Path::new("config.local.toml")) {
                        Ok(_) => app.set_error("配置已保存到 config.local.toml".to_string()),
                        Err(e) => app.set_error(format!("保存配置失败: {}", e)),
                    }
                }
            });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(10.0);

        // 项目级模型配置说明
        egui::CollapsingHeader::new("项目级模型配置")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("在项目详情页面，您可以为每个项目单独设置不同的LLM模型。");
                ui.label("项目级配置会覆盖全局配置，优先使用。");
                ui.add_space(10.0);
                ui.label("使用方法:");
                ui.label("1. 进入项目详情页面");
                ui.label("2. 在左侧找到\"模型配置\"选项");
                ui.label("3. 启用项目级配置并选择模型");
                ui.label("4. 保存后该项目的生成将使用指定模型");
            });

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(10.0);

        // 关于区域
        egui::CollapsingHeader::new("关于")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("AI Novel Agent v0.1.0");
                ui.label("基于 Rust + egui 构建的 AI 小说生成工具");
                ui.add_space(5.0);
                ui.label("支持的LLM提供商:");
                ui.label("• MiniMax (abab6.5s-chat)");
                ui.label("• Qwen (阿里通义千问)");
                ui.label("• OpenAI (GPT系列)");
                ui.label("• Anthropic (Claude系列)");
            });
    });
}
