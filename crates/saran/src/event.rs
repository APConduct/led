
use crate::point::Point;
use crate::key::Code as KeyCode;
use crate::widget;
use std::collections::HashSet;
use crate::theme::Theme;
use crate::key::Modifiers;

/// A type alias for a 2D point with `f32` coordinates, representing a position on the screen.
type ScreenPoint = Point<f32>;

/// Represents all possible user input events that can be handled by the UI system.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Mouse button pressed.
    MouseDown {
        /// The mouse button that was pressed.
        button: MouseButton,
        /// The position of the mouse when the button was pressed.
        position: ScreenPoint,
    },
    /// Mouse button released.
    MouseUp {
        /// The mouse button that was released.
        button: MouseButton,
        /// The position of the mouse when the button was released.
        position: ScreenPoint,
    },
    /// Mouse moved.
    MouseMove {
        /// The new position of the mouse.
        position: ScreenPoint,
        /// The change in position since the last event.
        delta: ScreenPoint,
    },
    /// Mouse wheel scrolled.
    MouseWheel {
        /// The amount and direction of the scroll.
        delta: ScreenPoint,
        /// The position of the mouse during the scroll.
        position: ScreenPoint,
    },
    /// Keyboard key pressed.
    KeyDown {
        /// The key that was pressed.
        key: KeyCode,
        /// The active keyboard modifiers (e.g., Shift, Ctrl).
        modifiers: Modifiers,
        /// Whether this is a repeated key press (from holding the key down).
        repeat: bool,
    },
    /// Keyboard key released.
    KeyUp {
        /// The key that was released.
        key: KeyCode,
        /// The active keyboard modifiers at the time of release.
        modifiers: Modifiers,
    },
    /// Text input event (for character input).
    TextInput {
        /// The text that was input.
        text: String,
    },
    /// The UI has gained focus.
    FocusGained,
    /// The UI has lost focus.
    FocusLost,
}

/// Context for handling events, containing state about the UI and input devices.
pub struct Context<'a> {
    /// The currently focused widget, if any.
    pub focused_widget: Option<widget::ID>,
    /// The widget currently under the mouse cursor, if any.
    pub hovered_widget: Option<widget::ID>,
    /// The widget currently being interacted with (e.g., pressed), if any.
    pub active_widget: Option<widget::ID>,
    /// The current position of the mouse cursor.
    pub mouse_pos: ScreenPoint,
    /// The set of mouse buttons currently pressed.
    pub mouse_buttons: HashSet<MouseButton>,
    /// The current state of keyboard modifier keys.
    pub keyboard_modifiers: Modifiers,
    /// The current theme used for rendering.
    pub theme: &'a Theme,
    /// Whether the current event has been handled.
    pub handled: bool,
    /// Clipboard access for copy/paste operations, if available.
    pub clipboard: Option<&'a mut dyn Clipboard>,
    // pub ui_tree: &'a mut WidgetTree,
}

/// Trait for clipboard operations, allowing getting and setting clipboard contents.
pub trait Clipboard {
    /// Gets the current contents of the clipboard, if any.
    fn get(&self) -> Option<String>;
    /// Sets the contents of the clipboard.
    fn set(&mut self, contents: &str);
}

/// Represents mouse buttons that can be used in mouse events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::key::Code as KeyCode;
    use crate::key::Modifiers;

    fn dummy_point() -> Point<f32> {
        Point::new( 1.0, 2.0 )
    }

    #[test]
    fn test_mouse_down_event() {
        let event = Event::MouseDown {
            button: MouseButton::Left,
            position: dummy_point(),
        };
        if let Event::MouseDown { button, position } = event {
            assert_eq!(button, MouseButton::Left);
            assert_eq!(position, dummy_point());
        } else {
            panic!("Expected MouseDown event");
        }
    }

    #[test]
    fn test_mouse_up_event() {
        let event = Event::MouseUp {
            button: MouseButton::Right,
            position: dummy_point(),
        };
        if let Event::MouseUp { button, position } = event {
            assert_eq!(button, MouseButton::Right);
            assert_eq!(position, dummy_point());
        } else {
            panic!("Expected MouseUp event");
        }
    }

    #[test]
    fn test_mouse_move_event() {
        let event = Event::MouseMove {
            position: dummy_point(),
            delta: Point::new(0.5, 0.5),
        };
        if let Event::MouseMove { position, delta } = event {
            assert_eq!(position, dummy_point());
            assert_eq!(delta, Point::new(0.5, 0.5));
        } else {
            panic!("Expected MouseMove event");
        }
    }
    #[test]
    fn test_mouse_wheel_event() {
        let event = Event::MouseWheel {
            delta: Point::new(0.0, 1.0),
            position: dummy_point(),
        };
        if let Event::MouseWheel { delta, position } = event {
            assert_eq!(delta, Point::new(0.0, 1.0));
            assert_eq!(position, dummy_point());
        } else {
            panic!("Expected MouseWheel event");
        }
    }

    #[test]
    fn test_key_down_event() {
        let event = Event::KeyDown {
            key: KeyCode::A,
            modifiers: Modifiers::default(),
            repeat: true,
        };
        if let Event::KeyDown { key, modifiers, repeat } = event {
            assert_eq!(key, KeyCode::A);
            assert_eq!(modifiers, Modifiers::default());
            assert!(repeat);
        } else {
            panic!("Expected KeyDown event");
        }
    }

    #[test]
    fn test_key_up_event() {
        let event = Event::KeyUp {
            key: KeyCode::B,
            modifiers: Modifiers::default(),
        };
        if let Event::KeyUp { key, modifiers } = event {
            assert_eq!(key, KeyCode::B);
            assert_eq!(modifiers, Modifiers::default());
        } else {
            panic!("Expected KeyUp event");
        }
    }

    #[test]
    fn test_text_input_event() {
        let event = Event::TextInput {
            text: "hello".to_string(),
        };
        if let Event::TextInput { text } = event {
            assert_eq!(text, "hello");
        } else {
            panic!("Expected TextInput event");
        }
    }

    #[test]
    fn test_focus_gained_event() {
        let event = Event::FocusGained;
        if let Event::FocusGained = event {
        } else {
            panic!("Expected FocusGained event");
        }
    }

    #[test]
    fn test_focus_lost_event() {
        let event = Event::FocusLost;
        if let Event::FocusLost = event {
        } else {
            panic!("Expected FocusLost event");
        }
    }
}
