use crate::context::Context as DrawContext;
use crate::event::Event;
use crate::layout::Context as LayoutContext;
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for widgets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u64);

impl WidgetId {
    fn next() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        WidgetId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// Common trait for all widgets.
pub trait Widget {
    fn id(&self) -> WidgetId;
    fn draw(&mut self, ctx: &mut DrawContext);
    fn layout(&mut self, ctx: &mut LayoutContext);
    /// Returns true if the event was handled.
    fn handle_event(&mut self, event: &Event) -> bool;
    /// Called when the widget receives focus.
    fn on_focus(&mut self) {}
    /// Called when the widget loses focus.
    fn on_blur(&mut self) {}
}

/// Label widget.
pub struct Label {
    id: WidgetId,
    text: String,
    color: Option<egui::Color32>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: WidgetId::next(),
            text: text.into(),
            color: None,
        }
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
    pub fn color(mut self, color: egui::Color32) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for Label {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn draw(&mut self, _ctx: &mut DrawContext) {
        println!("Drawing Label: {}", self.text);
    }
    fn layout(&mut self, _ctx: &mut LayoutContext) {}
    fn handle_event(&mut self, _event: &Event) -> bool {
        false
    }
}

/// Button widget.
///
/// # Examples
///
/// Basic focus handling:
/// ```
/// use saran::widget::{Button, Widget};
/// use saran::event::Event;
///
/// let mut btn = Button::new("Click Me");
///
/// // Initially not focused
/// assert!(!btn.is_focused());
///
/// // Simulate focus gained
/// btn.handle_event(&Event::FocusGained);
/// assert!(btn.is_focused());
///
/// // Simulate focus lost
/// btn.handle_event(&Event::FocusLost);
/// assert!(!btn.is_focused());
///
/// // Simulate mouse click (should set focus)
/// btn.handle_event(&Event::MouseDown {
///     button: saran::event::MouseButton::Left,
///     position: saran::point::Point::new(0.0, 0.0),
/// });
/// assert!(btn.is_focused());
/// ```
pub struct Button {
    id: WidgetId,
    label: String,
    on_click: Option<Box<dyn FnMut()>>,
    enabled: bool,
    focused: bool,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: WidgetId::next(),
            label: label.into(),
            on_click: None,
            enabled: true,
            focused: false,
        }
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }
    pub fn on_click<F: FnMut() + 'static>(mut self, f: F) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    /// Returns whether the button is focused.
    pub fn is_focused(&self) -> bool {
        self.focused
    }
}

impl Widget for Button {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn draw(&mut self, _ctx: &mut DrawContext) {
        println!(
            "Drawing Button: {}{}",
            self.label,
            if self.focused { " [focused]" } else { "" }
        );
    }
    fn layout(&mut self, _ctx: &mut LayoutContext) {}
    /// Handles mouse down (click) and focus events.
    /// Returns true if the event was handled.
    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::MouseDown { .. } => {
                if self.enabled {
                    if let Some(cb) = &mut self.on_click {
                        cb();
                    }
                    self.focused = true;
                    true
                } else {
                    false
                }
            }
            Event::FocusGained => {
                self.focused = true;
                true
            }
            Event::FocusLost => {
                self.focused = false;
                true
            }
            _ => false,
        }
    }
    fn on_focus(&mut self) {
        self.focused = true;
    }
    fn on_blur(&mut self) {
        self.focused = false;
    }
}

/// TextInput widget.
///
/// # Examples
///
/// Focus and text input handling:
/// ```
/// use saran::widget::{TextInput, Widget};
/// use saran::event::Event;
///
/// let mut input = TextInput::new();
///
/// // Initially not focused
/// assert!(!input.is_focused());
///
/// // Simulate mouse click (should set focus)
/// input.handle_event(&Event::MouseDown {
///     button: saran::event::MouseButton::Left,
///     position: saran::point::Point::new(0.0, 0.0),
/// });
/// assert!(input.is_focused());
///
/// // Simulate text input
/// input.handle_event(&Event::TextInput { text: "abc".to_string() });
/// assert_eq!(input.value(), "abc");
///
/// // Simulate focus lost
/// input.handle_event(&Event::FocusLost);
/// assert!(!input.is_focused());
/// ```
pub struct TextInput {
    id: WidgetId,
    value: String,
    on_change: Option<Box<dyn FnMut(String)>>,
    placeholder: Option<String>,
    focused: bool,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            id: WidgetId::next(),
            value: String::new(),
            on_change: None,
            placeholder: None,
            focused: false,
        }
    }
    pub fn set_value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
    pub fn on_change<F: FnMut(String) + 'static>(mut self, f: F) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }
    /// Returns whether the text input is focused.
    pub fn is_focused(&self) -> bool {
        self.focused
    }
    /// Returns the current value of the text input.
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Widget for TextInput {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn draw(&mut self, _ctx: &mut DrawContext) {
        println!(
            "Drawing TextInput: {}{}",
            self.value,
            if self.focused { " [focused]" } else { "" }
        );
    }
    fn layout(&mut self, _ctx: &mut LayoutContext) {}

    /// Handles focus and text input events.
    /// Returns true if the event was handled.
    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::TextInput { text } => {
                self.value = text.clone();
                if let Some(cb) = &mut self.on_change {
                    cb(text.clone());
                }
                true
            }
            Event::MouseDown { .. } => {
                // Request focus on mouse click
                self.focused = true;
                true
            }
            Event::FocusGained => {
                self.focused = true;
                true
            }
            Event::FocusLost => {
                self.focused = false;
                true
            }
            _ => false,
        }
    }

    fn on_focus(&mut self) {
        self.focused = true;
    }
    fn on_blur(&mut self) {
        self.focused = false;
    }
}

/// Column widget.
///
/// # Examples
///
/// Event propagation and focus handling:
/// ```
/// use saran::widget::{Column, Button, TextInput, Widget, WidgetId};
/// use saran::event::Event;
///
/// // Create widgets
/// let mut btn = Button::new("Click Me");
/// let mut input = TextInput::new();
///
/// // Compose into a column
/// let mut col = Column::new()
///     .add_child(Box::new(btn))
///     .add_child(Box::new(input));
///
/// // Set focus to the first child (Button)
/// let first_id = col.child_id(0).unwrap();
/// col.set_focus(first_id);
///
/// // Send a MouseDown event to the column
/// let mouse_event = Event::MouseDown {
///     button: saran::event::MouseButton::Left,
///     position: saran::point::Point::new(10.0, 10.0),
/// };
/// let handled = col.handle_event(&mouse_event);
/// assert!(handled, "Button should handle MouseDown and request focus");
///
/// // Send a TextInput event to the column (should go to focused child)
/// let text_event = Event::TextInput { text: "Hello".to_string() };
/// let handled = col.handle_event(&text_event);
/// assert!(handled, "TextInput should handle TextInput event");
///
/// // Set focus to the second child (TextInput)
/// let second_id = col.child_id(1).unwrap();
/// col.set_focus(second_id);
///
/// // Send a TextInput event to the column (should go to TextInput)
/// let text_event = Event::TextInput { text: "World".to_string() };
/// let handled = col.handle_event(&text_event);
/// assert!(handled, "TextInput should handle TextInput event");
/// ```
pub struct Column {
    id: WidgetId,
    children: Vec<Box<dyn Widget>>,
    focused_child: Option<WidgetId>,
}

impl Column {
    pub fn new() -> Self {
        Self {
            id: WidgetId::next(),
            children: Vec::new(),
            focused_child: None,
        }
    }
    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
    /// Sets focus to the child with the given WidgetId.
    pub fn set_focus(&mut self, id: WidgetId) {
        self.focused_child = Some(id);
    }
    /// Returns the WidgetId of the child at the given index, if it exists.
    pub fn child_id(&self, idx: usize) -> Option<WidgetId> {
        self.children.get(idx).map(|c| c.id())
    }
}

impl Widget for Column {
    fn id(&self) -> WidgetId {
        self.id
    }
    fn draw(&mut self, ctx: &mut DrawContext) {
        for child in &mut self.children {
            child.draw(ctx);
        }
    }
    fn layout(&mut self, ctx: &mut LayoutContext) {
        for child in &mut self.children {
            child.layout(ctx);
        }
    }
    fn handle_event(&mut self, event: &Event) -> bool {
        for child in &mut self.children {
            if child.handle_event(event) {
                return true;
            }
        }
        false
    }
}

// Example usage: building a UI tree
pub fn example_ui_tree() -> Box<dyn Widget> {
    Box::new(
        Column::new()
            .add_child(Box::new(
                Label::new("Welcome to LED!").color(egui::Color32::WHITE),
            ))
            .add_child(Box::new(
                TextInput::new()
                    .placeholder("Type here...")
                    .on_change(|val| println!("Text changed: {}", val)),
            ))
            .add_child(Box::new(
                Button::new("Save")
                    .enabled(true)
                    .on_click(|| println!("Save clicked!")),
            )),
    )
}

// Example: traversing the UI tree
pub fn run_example() {
    let mut ui = example_ui_tree();
    let mut draw_ctx = crate::context::Context::new(egui::Context::default());
    let mut layout_ctx = crate::layout::Context::new(
        crate::size::Size::new(800.0, 600.0),
        crate::layout::Direction::Vertical,
        None,
    );
    let event = crate::event::Event::TextInput {
        text: "Hello, world!".to_string(),
    };

    ui.layout(&mut layout_ctx);
    ui.handle_event(&event);
    ui.draw(&mut draw_ctx);
}
