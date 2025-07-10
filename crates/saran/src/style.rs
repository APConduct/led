use std::collections::HashMap;
use crate::theme::Theme;

/// The `System` struct manages multiple UI themes and tracks the currently active theme.
///
/// # Fields
/// - `themes`: A map of theme names to their corresponding `Theme` objects.
/// - `active_theme`: The name of the currently active theme.
pub struct System {
    /// Stores available themes, keyed by their names.
    themes: HashMap<String, Theme>,
    /// The name of the currently active theme.
    active_theme: String,
}

impl System {
    /// Creates a new `System` with a default "dark" theme.
    ///
    /// # Returns
    /// A `System` instance with the "dark" theme set as active.
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

    /// Returns a reference to the currently active `Theme`.
    ///
    /// # Returns
    /// A reference to the `Theme` corresponding to the active theme name.
    pub fn get_active_theme(&self) -> &Theme {
        &self.themes[&self.active_theme]
    }
}