

bitflags::bitflags! {
    pub struct Modifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CTRL = 0b0000_0010;
        const ALT = 0b0000_0100;
        const SUPER = 0b0000_1000; // Meta key (Windows/Command)
    }
}

/// Represents keyboard keys that can be used in keyboard events.
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