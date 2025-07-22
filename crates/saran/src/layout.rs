use std::collections::HashMap;
use crate::theme::Theme;
use crate::size::Size;

pub type ScreenSize = Size<f32>;

/// The `Cache` struct provides caching for text layouts and glyph font IDs,
/// optimizing repeated layout and font lookups in the UI rendering process.
///
/// # Fields
/// - `text_layouts`: Caches `egui::text::LayoutJob` objects keyed by string,
///   to avoid redundant layout computations for the same text.
/// - `glyph_cache`: Caches `egui::epaint::text::FontId` objects keyed by character,
///   to speed up font resolution for glyphs.
pub struct Cache {
    /// Stores cached text layouts for strings.
    text_layouts: HashMap<String, egui::text::LayoutJob>,
    /// Stores cached font IDs for individual glyphs.
    glyph_cache: HashMap<char, egui::epaint::text::FontId>,
}

impl Cache {
    /// Creates a new, empty `Cache` instance.
    ///
    /// # Returns
    /// A `Cache` with empty text layout and glyph caches.
    pub fn new() -> Self {
        Self {
            text_layouts: HashMap::new(),
            glyph_cache: HashMap::new(),
        }
    }
}

/// Represents the layout direction for UI elements.
///
/// - `Horizontal`: Layout elements from left to right.
/// - `Vertical`: Layout elements from top to bottom.
pub enum Direction {
    Horizontal,
    Vertical,
}

/// Provides contextual information for layout calculations.
///
/// # Fields
/// - `available_space`: The space available for layout, typically the size of the parent container.
/// - `direction`: The primary direction in which to lay out child elements.
/// - `theme`: An optional reference to the current UI theme, used for styling.
/// - `relayout_requested`: Indicates whether a relayout should be triggered (e.g., due to content or size changes).
pub struct Context<'a> {
    pub available_space: ScreenSize,
    pub direction: Direction,
    pub theme: Option<&'a Theme>,
    pub relayout_requested: bool,
}

impl<'a> Context<'a> {
    /// Creates a new `Context` with the specified available space and layout direction.
    ///
    /// # Parameters
    /// - `available_space`: The size of the area available for layout.
    /// - `direction`: The primary layout direction (horizontal or vertical).
    /// - `theme`: An optional reference to the current theme for styling.
    ///
    /// # Returns
    /// A new `Context` instance initialized with the provided parameters.
    pub fn new(available_space: ScreenSize, direction: Direction, theme: Option<&'a Theme>) -> Self {
        Self {
            available_space,
            direction,
            theme,
            relayout_requested: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::System;
    use crate::size::Size;
    #[test]
    fn creates_empty_cache() {
        let cache = Cache::new();
        assert!(cache.text_layouts.is_empty());
        assert!(cache.glyph_cache.is_empty());
    }

    #[test]
    fn context_initializes_with_given_parameters() {
        let available_space = Size::new(100.0, 50.0);
        let style_system = System::new();
        let theme = style_system.get_active_theme();
        let ctx = Context::new(available_space, Direction::Horizontal, Some(&theme));
        assert_eq!(ctx.available_space.width(), 100.0);
        assert_eq!(ctx.available_space.height(), 50.0);
        matches!(ctx.direction, Direction::Horizontal);
        assert!(ctx.theme.is_some());
        assert!(!ctx.relayout_requested);
    }

    #[test]
    fn context_allows_none_theme() {
        let available_space = Size::new(0.0, 0.0);
        let ctx = Context::new(available_space, Direction::Vertical, None);
        assert!(ctx.theme.is_none());
        matches!(ctx.direction, Direction::Vertical);
    }

    #[test]
    fn context_handles_zero_available_space() {
        let available_space = Size::new( 0.0,  0.0 );
        let ctx = Context::new(available_space, Direction::Horizontal, None);
        assert_eq!(ctx.available_space.width(), 0.0);
        assert_eq!(ctx.available_space.height(), 0.0);
    }
}