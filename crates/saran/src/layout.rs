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

pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Context<'a> {
    pub available_space: ScreenSize,
    pub direction: Direction,
    pub theme: Option<&'a Theme>,
    pub relayout_requested: bool,
}