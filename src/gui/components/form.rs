//! Form Components

use egui::{ComboBox, Ui};

use crate::models::NovelGenre;

/// Genre selector dropdown
pub fn genre_selector(ui: &mut Ui, label: &str, selected: &mut Option<NovelGenre>) {
    ui.label(label);
    ComboBox::from_id_salt("genre")
        .selected_text(format!("{:?}", selected.unwrap_or(NovelGenre::Fantasy)))
        .show_ui(ui, |ui| {
            ui.selectable_value(selected, Some(NovelGenre::Fantasy), "Fantasy");
            ui.selectable_value(selected, Some(NovelGenre::Urban), "Urban");
            ui.selectable_value(selected, Some(NovelGenre::Xianxia), "Xianxia");
            ui.selectable_value(selected, Some(NovelGenre::Historical), "Historical");
            ui.selectable_value(selected, Some(NovelGenre::Romance), "Romance");
            ui.selectable_value(selected, Some(NovelGenre::Scifi), "Scifi");
            ui.selectable_value(selected, Some(NovelGenre::Game), "Game");
            ui.selectable_value(selected, Some(NovelGenre::Horror), "Horror");
        });
}
