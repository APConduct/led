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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID(pub u64);

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

pub struct Column {
    children: Vec<Box<dyn Widget>>,
}

impl Column {
    /// Creates a new empty Column.
    pub fn new() -> Self {
        Self { children: Vec::new() }
    }

    /// Adds a child widget to the column.
    pub fn add_child(&mut self, child: Box<dyn Widget>) {
        self.children.push(child);
    }
}

impl Widget for Column {
    fn layout(&mut self /*, layout params */) {
        // Simple vertical stacking: each child is laid out below the previous one.
        for child in &mut self.children {
            child.layout(/* layout params */);
            // You would update the child's position here based on your layout system.
        }
    }

    fn handle_event(&mut self /*, event params */) {
        for child in &mut self.children {
            child.handle_event(/* event params */);
        }
    }
}

impl Draw for Column {
    fn draw(&mut self, ctx: &mut crate::context::Context) {
        for child in &mut self.children {
            child.draw(ctx);
        }
    }
}

// Example usage (in your UI code):
// let mut col = Column::new();
// col.add_child(Box::new(Label::new("First label")));
// col.add_child(Box::new(Button::new("Click me!")));
// ...
