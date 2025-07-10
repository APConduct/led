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