use egui::ahash::{HashMap, HashMapExt};
use crate::theme::Theme;

pub struct System {
    themes: HashMap<String, Theme>,
    active_theme: String,
}

impl System {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        themes.insert("dark".to_string(), Theme {
            background: egui::Color32::from_rgb(40, 44, 52),
            foreground: egui::Color32::from_rgb(171, 178, 191),
            selection: egui::Color32::from_rgb(61, 133, 198),
            cursor: egui::Color32::WHITE,
            line_numbers: egui::Color32::from_rgb(128, 128, 128),
        });
        
        Self {
            themes,
            active_theme: "dark".to_string(),
        }
    }
    
    pub fn get_active_theme(&self) -> &Theme {
        &self.themes[&self.active_theme]
    }
}