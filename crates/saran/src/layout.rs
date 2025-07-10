use std::collections::HashMap;

pub struct Cache {
    text_layouts: HashMap<String, egui::text::LayoutJob>,
    glyph_cache: HashMap<char, egui::epaint::text::FontId>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            text_layouts: HashMap::new(),
            glyph_cache: HashMap::new(),
        }
    }
}