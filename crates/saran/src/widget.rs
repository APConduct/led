/// The `Draw` trait defines a common interface for drawable UI widgets.
///
/// Types implementing this trait must provide a `draw` method, which
/// takes a mutable reference to a drawing context and renders the widget.
///
/// # Arguments
/// * `ctx` - A mutable reference to the drawing context used for rendering.
pub trait Draw {
    /// Draws the widget using the provided context.
    ///
    /// # Parameters
    /// - `ctx`: The mutable drawing context to render into.
    fn draw(&mut self, ctx: &mut crate::context::Context);
}

/// A simple label widget that displays static text.
///
/// The `Label` struct holds a string of text to be rendered by the UI.
/// It is typically used for displaying non-interactive, read-only text
/// within a user interface.
pub struct Label {
    /// The text content of the label.
    text: String,
}

impl Label {
    /// Creates a new `Label` with the given text.
    ///
    /// # Parameters
    /// - `text`: The text to display in the label. Can be any type that
    ///   implements `Into<String>`.
    ///
    /// # Returns
    /// A new `Label` instance containing the provided text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

pub trait Widget: Draw {
    fn layout(&mut self, /* layout params */);
    fn handle_event(&mut self, /* event params */);
    // Add more widget-specific methods as needed
}

pub struct Button {
    label: String,
    on_click: Option<Box<dyn FnMut()>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), on_click: None }
    }
    pub fn on_click<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}