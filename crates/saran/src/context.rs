pub struct Context {
    pub egui_ctx: egui::Context,
    style_system: super::style::System,
    layout_cache: super::layout::Cache,
}

impl Context {
    pub fn new(egui_ctx: egui::Context) -> Self {
        Self {
            egui_ctx,
            style_system: super::style::System::new(),
            layout_cache: super::layout::Cache::new(),
        }
    }
}
