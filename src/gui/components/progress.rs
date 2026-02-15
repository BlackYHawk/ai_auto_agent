//! Progress Component

use egui::{ProgressBar, Ui};

/// Show a progress bar with message
pub fn show(ui: &mut Ui, progress: f32, message: &str) {
    ui.label(message);
    ui.add(ProgressBar::new(progress));
}
