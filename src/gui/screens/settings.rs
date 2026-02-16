//! 设置页面 - LLM模型配置

use egui::{ComboBox, Ui};

use crate::gui::app::{NovelApp, Screen};

/// 全局LLM配置结构
#[derive(Debug, Clone, Default)]
pub struct LlmConfigForm {
    pub provider: String,
    pub api_key: String,
    pub model: String,
    pub temperature: String,
    pub max_tokens: String,
}

/// 显示设置页面
pub fn show(ui: &mut Ui, app: &mut NovelApp) {
    // 静态配置表单（实际应从config.toml加载）
    static CONFIG: std::sync::LazyLock<std::sync::Mutex<LlmConfigForm>> =
        std::sync::LazyLock::new(|| {
            std::sync::Mutex::new(LlmConfigForm {
                provider: "minimax".to_string(),
                api_key: "".to_string(),
                model: "abab6.5s-chat".to_string(),
                temperature: "0.7".to_string(),
                max_tokens: "4096".to_string(),
            })
        });

    let mut config = CONFIG.lock().unwrap();

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

        // LLM配置区域
        egui::CollapsingHeader::new("LLM 模型配置")
            .default_open(true)
            .show(ui, |ui| {
                ui.add_space(5.0);

                // 提供商选择
                ui.label("模型提供商:");
                ComboBox::from_id_salt("provider_selector")
                    .selected_text(&config.provider)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut config.provider, "minimax".to_string(), "MiniMax");
                        ui.selectable_value(&mut config.provider, "qwen".to_string(), "Qwen (阿里)");
                        ui.selectable_value(&mut config.provider, "openai".to_string(), "OpenAI");
                        ui.selectable_value(&mut config.provider, "anthropic".to_string(), "Anthropic");
                    });

                ui.add_space(10.0);

                // API Key
                ui.label("API Key:");
                ui.text_edit_singleline(&mut config.api_key);

                ui.add_space(10.0);

                // 模型选择
                ui.label("模型名称:");
                ui.text_edit_singleline(&mut config.model);

                ui.add_space(10.0);

                // Temperature
                ui.label("Temperature (0.0-1.0):");
                ui.text_edit_singleline(&mut config.temperature);

                ui.add_space(10.0);

                // Max Tokens
                ui.label("Max Tokens:");
                ui.text_edit_singleline(&mut config.max_tokens);

                ui.add_space(15.0);

                // 保存按钮
                if ui.button("保存配置").clicked() {
                    // TODO: 保存到 config.toml
                    app.set_error("配置已保存 (模拟)".to_string());
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
