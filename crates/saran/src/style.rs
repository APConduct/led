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

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Color32;

    #[test]
    fn new_creates_system_with_dark_theme_active() {
        let system = System::new();
        assert_eq!(system.active_theme, "dark");
        let theme = system.get_active_theme();
        assert_eq!(theme.background, Color32::from_rgb(40, 44, 52));
        assert_eq!(theme.foreground, Color32::from_rgb(171, 178, 191));
        assert_eq!(theme.selection, Color32::from_rgb(61, 133, 198));
        assert_eq!(theme.cursor, Color32::WHITE);
        assert_eq!(theme.line_numbers, Color32::from_rgb(128, 128, 128));
    }

    #[test]
    fn get_active_theme_returns_correct_theme_when_multiple_themes_present() {
        let mut system = System::new();
        system.themes.insert("light".to_string(), Theme {
            background: Color32::from_rgb(255, 255, 255),
            foreground: Color32::from_rgb(0, 0, 0),
            selection: Color32::from_rgb(200, 200, 200),
            cursor: Color32::BLACK,
            line_numbers: Color32::from_rgb(100, 100, 100),
        });
        // Still returns dark theme since active_theme is "dark"
        let theme = system.get_active_theme();
        assert_eq!(theme.background, Color32::from_rgb(40, 44, 52));
    }

    #[test]
    fn get_active_theme_panics_if_active_theme_missing() {
        let mut system = System::new();
        system.active_theme = "nonexistent".to_string();
        let result = std::panic::catch_unwind(|| {
            system.get_active_theme();
        });
        assert!(result.is_err());
    }
}