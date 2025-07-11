
use crate::point::Point;
use crate::key::Code as KeyCode;
use crate::widget;
use crate::key;
use std::collections::HashSet;
use crate::theme::Theme;

type ScreenPoint = Point<f32>;

pub enum Event {

    MouseDown {
        button: MouseButton,
        position: ScreenPoint,
    },
    MouseUp {
        button: MouseButton,
        position: ScreenPoint,
    },
    MouseMove {
        position: ScreenPoint,
        delta: ScreenPoint,
    },
    MouseWheel {
        delta: ScreenPoint,
        position: ScreenPoint,
    },
    KeyDown {
        key: KeyCode,
        // modifiers: u32, // Bitmask for modifier keys (Shift, Ctrl, Alt, etc.)
        repeat: bool,
    },
    KeyUp {
        key: KeyCode,
        // modifiers: u32, // Bitmask for modifier keys (Shift, Ctrl, Alt, etc.)
    },
    TextInput {
        text: String,
    },
    FocusGained,
    FocusLost,
}

pub struct Context<'a> {
    pub focused_widget: Option<widget::ID>,
    pub hovered_widget: Option<widget::ID>,
    pub active_widget: Option<widget::ID>,
    pub mouse_pos: ScreenPoint,
    pub mouse_buttons: HashSet<MouseButton>,
    pub keyboard_modifiers: key::Modifiers,
    pub theme: &'a Theme,
    pub handled: bool,

    pub clipboard: Option<&'a mut dyn Clipboard>,
    // pub ui_tree: &'a mut WidgetTree,
    
    
}

pub trait Clipboard {
    fn get(&self) -> Option<String>;
    fn set(&mut self, contents: &str);
}


/// Represents mouse buttons that can be used in mouse events.
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button (usually the scroll wheel).
    Middle,
    /// Any other mouse button, specified by its numeric code.
    Other(u8),
}

