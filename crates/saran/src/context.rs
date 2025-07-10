/// The `Context` struct encapsulates the main context for the application,
/// providing access to the egui context, style system, and layout cache.
///
/// # Fields
/// - `egui_ctx`: The main egui context used for rendering and UI state.
/// - `style_system`: Manages the application's style and theming.
/// - `layout_cache`: Caches layout computations for efficient UI rendering.
pub struct Context {
    /// The egui context for UI rendering and state management.
    pub egui_ctx: egui::Context,
    /// The style system for managing UI styles and themes.
    style_system: super::style::System,
    /// The layout cache for optimizing layout calculations.
    layout_cache: super::layout::Cache,
}

impl Context {
    /// Creates a new `Context` instance with the provided egui context.
    ///
    /// # Arguments
    ///
    /// * `egui_ctx` - The egui context to be used for UI rendering.
    ///
    /// # Returns
    ///
    /// A new `Context` with initialized style system and layout cache.
    pub fn new(egui_ctx: egui::Context) -> Self {
        Self {
            egui_ctx,
            style_system: super::style::System::new(),
            layout_cache: super::layout::Cache::new(),
        }
    }
}