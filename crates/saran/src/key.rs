bitflags::bitflags! {
    /// Modifier keys for keyboard input, represented as bitflags.
    ///
    /// This struct allows checking for the presence of modifier keys such as Shift, Control, Alt, and Super (Meta).
    /// Each modifier is represented as a single bit in a `u8`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        /// The Shift key modifier.
        const SHIFT = 0b0000_0001;
        /// The Control key modifier.
        const CTRL = 0b0000_0010;
        /// The Alt key modifier.
        const ALT = 0b0000_0100;
        /// The Super (Meta/Windows/Command) key modifier.
        const SUPER = 0b0000_1000; // Meta key (Windows/Command)
    }
}

impl Modifiers {
    /// Returns `true` if the Shift key is pressed.
    pub fn shift(self) -> bool {
        self.contains(Modifiers::SHIFT)
    }

    /// Returns `true` if the Control key is pressed.
    pub fn ctrl(self) -> bool {
        self.contains(Modifiers::CTRL)
    }

    /// Returns `true` if the Alt key is pressed.
    pub fn alt(self) -> bool {
        self.contains(Modifiers::ALT)
    }

    /// Returns `true` if the Super (Meta/Windows/Command) key is pressed.
    pub fn super_key(self) -> bool {
        self.contains(Modifiers::SUPER)
    }
}

impl Default for Modifiers {
    /// Returns an empty set of modifiers (no modifier keys pressed).
    fn default() -> Self {
        Modifiers::empty()
    }
}

/// Represents keyboard keys that can be used in keyboard events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Code {
    /// The Enter/Return key.
    Enter,
    /// The Escape key.
    Escape,
    /// The Tab key.
    Tab,
    /// The Backspace key.
    Backspace,
    /// The Up arrow key.
    ArrowUp,
    /// The Down arrow key.
    ArrowDown,
    /// The Left arrow key.
    ArrowLeft,
    /// The Right arrow key.
    ArrowRight,
    /// The Home key.
    Home,
    /// The End key.
    End,
    /// The Page Up key.
    PageUp,
    /// The Page Down key.
    PageDown,
    /// The Delete key.
    Delete,
    /// The Insert key.
    Insert,
    /// The Space bar.
    Space,
    /// The 'A' key.
    A,
    /// The 'B' key.
    B,
    /// The 'C' key.
    C,
    /// The 'D' key.
    D,
    /// The 'E' key.
    E,
    /// The 'F' key.
    F,
    /// The 'G' key.
    G,
    /// The 'H' key.
    H,
    /// The 'I' key.
    I,
    /// The 'J' key.
    J,
    /// The 'K' key.
    K,
    /// The 'L' key.
    L,
    /// The 'M' key.
    M,
    /// The 'N' key.
    N,
    /// The 'O' key.
    O,
    /// The 'P' key.
    P,
    /// The 'Q' key.
    Q,
    /// The 'R' key.
    R,
    /// The 'S' key.
    S,
    /// The 'T' key.
    T,
    /// The 'U' key.
    U,
    /// The 'V' key.
    V,
    /// The 'W' key.
    W,
    /// The 'X' key.
    X,
    /// The 'Y' key.
    Y,
    /// The 'Z' key.
    Z,
    /// The '0' key.
    Zero,
    /// The '1' key.
    One,
    /// The '2' key.
    Two,
    /// The '3' key.
    Three,
    /// The '4' key.
    Four,
    /// The '5' key.
    Five,
    /// The '6' key.
    Six,
    /// The '7' key.
    Seven,
    /// The '8' key.
    Eight,
    /// The '9' key.
    Nine,
    /// The Caps Lock key.
    CapsLock,
    /// The Num Lock key.
    NumLock,
    /// The Scroll Lock key.
    ScrollLock,
    /// The F1 function key.
    F1,
    /// The F2 function key.
    F2,
    /// The F3 function key.
    F3,
    /// The F4 function key.
    F4,
    /// The F5 function key.
    F5,
    /// The F6 function key.
    F6,
    /// The F7 function key.
    F7,
    /// The F8 function key.
    F8,
    /// The F9 function key.
    F9,
    /// The F10 function key.
    F10,
    /// The F11 function key.
    F11,
    /// The F12 function key.
    F12,
    /// The left Shift key.
    LeftShift,
    /// The right Shift key.
    RightShift,
    /// The left Control key.
    LeftControl,
    /// The right Control key.
    RightControl,
    /// The left Alt key.
    LeftAlt,
    /// The right Alt key.
    RightAlt,
    /// The left Meta key (Windows/Command).
    LeftMeta,
    /// The right Meta key (Windows/Command).
    RightMeta,
    /// The Print Screen key.
    PrintScreen,
    /// Any other key, specified by its numeric code.
    Other(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifiers_shift_ctrl_alt_super_detection() {
        let mods = Modifiers::SHIFT | Modifiers::CTRL | Modifiers::ALT | Modifiers::SUPER;
        assert!(mods.shift());
        assert!(mods.ctrl());
        assert!(mods.alt());
        assert!(mods.super_key());
    }

    #[test]
    fn modifiers_individual_keys() {
        let shift = Modifiers::SHIFT;
        let ctrl = Modifiers::CTRL;
        let alt = Modifiers::ALT;
        let super_key = Modifiers::SUPER;

        assert!(shift.shift());
        assert!(!shift.ctrl());
        assert!(!shift.alt());
        assert!(!shift.super_key());

        assert!(!ctrl.shift());
        assert!(ctrl.ctrl());
        assert!(!ctrl.alt());
        assert!(!ctrl.super_key());

        assert!(!alt.shift());
        assert!(!alt.ctrl());
        assert!(alt.alt());
        assert!(!alt.super_key());

        assert!(!super_key.shift());
        assert!(!super_key.ctrl());
        assert!(!super_key.alt());
        assert!(super_key.super_key());
    }

    #[test]
    fn modifiers_default_is_empty() {
        let mods = Modifiers::default();
        assert!(!mods.shift());
        assert!(!mods.ctrl());
        assert!(!mods.alt());
        assert!(!mods.super_key());
        assert_eq!(mods, Modifiers::empty());
    }

    #[test]
    fn code_enum_variants_are_distinct() {
        assert_ne!(Code::Enter, Code::Escape);
        assert_ne!(Code::A, Code::B);
        assert_ne!(Code::F1, Code::F2);
        assert_ne!(Code::LeftShift, Code::RightShift);
    }

    #[test]
    fn code_other_variant_accepts_any_u32() {
        let code1 = Code::Other(42);
        let code2 = Code::Other(9999);
        assert_eq!(code1, Code::Other(42));
        assert_ne!(code1, code2);
    }
}