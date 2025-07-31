/// The `Context` struct encapsulates the main context for the application,
/// providing access to the egui context, style system, and layout cache.
///
/// # Fields
/// - `egui_ctx`: The main egui context used for rendering and UI state.
/// - `style_system`: Manages the application's style and theming.
/// - `layout_cache`: Caches layout computations for efficient UI rendering.
#[derive(Debug, Clone)]
pub struct Context {
    /// The egui context for UI rendering and state management.
    pub egui_ctx: egui::Context,
    /// The style system for managing UI styles and themes.
    pub style_system: super::style::System,
    /// The layout cache for optimizing layout calculations.
    pub layout_cache: super::layout::Cache,
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

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Context as EguiContext;

    struct DummyUi {
        pub last_label: Option<String>,
    }

    impl DummyUi {
        fn new() -> Self {
            Self { last_label: None }
        }
        fn label(&mut self, text: &str) {
            self.last_label = Some(text.to_owned());
        }
    }

    // Adapter to allow DummyUi to be used in EguiDrawContext for testing
    struct TestEguiDrawContext<'a> {
        pub ui: &'a mut DummyUi,
    }

    impl<'a> DrawContext for TestEguiDrawContext<'a> {
        fn draw_text(&mut self, text: &str) {
            self.ui.label(text);
        }
    }

    #[test]
    fn creates_context_with_provided_egui_ctx() {
        let egui_ctx = EguiContext::default();
        let context = Context::new(egui_ctx.clone());
        assert_eq!(context.egui_ctx.memory(|_| ()), egui_ctx.memory(|_| ()));
    }

    #[test]
    fn egui_draw_context_draws_text_label() {
        let mut dummy_ui = DummyUi::new();
        let mut draw_ctx = TestEguiDrawContext { ui: &mut dummy_ui };
        draw_ctx.draw_text("Hello, world!");
        assert_eq!(draw_ctx.ui.last_label.as_deref(), Some("Hello, world!"));
    }

    #[test]
    fn egui_draw_context_draws_empty_text() {
        let mut dummy_ui = DummyUi::new();
        let mut draw_ctx = TestEguiDrawContext { ui: &mut dummy_ui };
        draw_ctx.draw_text("");
        assert_eq!(draw_ctx.ui.last_label.as_deref(), Some(""));
    }

    #[test]
    fn egui_draw_context_draws_unicode_text() {
        let mut dummy_ui = DummyUi::new();
        let mut draw_ctx = TestEguiDrawContext { ui: &mut dummy_ui };
        draw_ctx.draw_text("こんにちは世界");
        assert_eq!(draw_ctx.ui.last_label.as_deref(), Some("こんにちは世界"));
    }
}
