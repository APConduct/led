/// The `Theme` struct defines a set of colors used for UI theming,
/// including background, foreground, selection, cursor, and line numbers.
///
/// # Fields
/// - `background`: The background color of the UI.
/// - `foreground`: The primary text or foreground color.
/// - `selection`: The color used for selected text or elements.
/// - `cursor`: The color of the text cursor.
/// - `line_numbers`: The color used for line numbers in the UI.
pub struct Theme {
    /// The background color of the UI.
    pub background: egui::Color32,
    /// The primary text or foreground color.
    pub foreground: egui::Color32,
    /// The color used for selected text or elements.
    pub selection: egui::Color32,
    /// The color of the text cursor.
    pub cursor: egui::Color32,
    /// The color used for line numbers in the UI.
    pub line_numbers: egui::Color32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Color32;

    #[test]
    fn theme_fields_are_set_correctly() {
        let theme = Theme {
            background: Color32::from_rgb(10, 20, 30),
            foreground: Color32::from_rgb(40, 50, 60),
            selection: Color32::from_rgb(70, 80, 90),
            cursor: Color32::from_rgb(100, 110, 120),
            line_numbers: Color32::from_rgb(130, 140, 150),
        };
        assert_eq!(theme.background, Color32::from_rgb(10, 20, 30));
        assert_eq!(theme.foreground, Color32::from_rgb(40, 50, 60));
        assert_eq!(theme.selection, Color32::from_rgb(70, 80, 90));
        assert_eq!(theme.cursor, Color32::from_rgb(100, 110, 120));
        assert_eq!(theme.line_numbers, Color32::from_rgb(130, 140, 150));
    }

    #[test]
    fn theme_fields_can_be_updated() {
        let mut theme = Theme {
            background: Color32::BLACK,
            foreground: Color32::WHITE,
            selection: Color32::GRAY,
            cursor: Color32::RED,
            line_numbers: Color32::BLUE,
        };
        theme.background = Color32::from_rgb(1, 2, 3);
        theme.foreground = Color32::from_rgb(4, 5, 6);
        theme.selection = Color32::from_rgb(7, 8, 9);
        theme.cursor = Color32::from_rgb(10, 11, 12);
        theme.line_numbers = Color32::from_rgb(13, 14, 15);

        assert_eq!(theme.background, Color32::from_rgb(1, 2, 3));
        assert_eq!(theme.foreground, Color32::from_rgb(4, 5, 6));
        assert_eq!(theme.selection, Color32::from_rgb(7, 8, 9));
        assert_eq!(theme.cursor, Color32::from_rgb(10, 11, 12));
        assert_eq!(theme.line_numbers, Color32::from_rgb(13, 14, 15));
    }
}