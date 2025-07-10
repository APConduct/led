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

/// The `DrawContext` trait defines an interface for drawing operations
/// that can be implemented by various backend adapters. This abstraction
/// allows widgets to perform drawing actions (such as rendering text)
/// without depending on a specific GUI backend.
///
/// Implementors of this trait should provide concrete behavior for
/// drawing primitives, enabling backend-agnostic widget rendering.
pub trait DrawContext {
    /// Draws the given text using the current drawing context.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice to be rendered.
    fn draw_text(&mut self, text: &str);
    // More drawing primitives may be added here
}

/// `EguiDrawContext` is a concrete implementation of the `DrawContext` trait
/// for the `egui` backend. It provides access to an `egui::Ui` instance,
/// enabling widgets to perform drawing operations using egui's API.
///
/// # Type Parameters
/// - `'a`: The lifetime of the underlying `egui::Ui` reference.
pub struct EguiDrawContext<'a> {
    /// A mutable reference to the egui UI, used for rendering widgets.
    pub ui: &'a mut egui::Ui,
}

/// Implements the `DrawContext` trait for `EguiDrawContext`, allowing
/// widgets to render text using egui's `label` method.
impl<'a> DrawContext for EguiDrawContext<'a> {
    /// Draws the given text as a label in the egui UI.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice to be rendered as a label.
    fn draw_text(&mut self, text: &str) {
        self.ui.label(text);
    }
    // More drawing methods can be implemented as needed
}