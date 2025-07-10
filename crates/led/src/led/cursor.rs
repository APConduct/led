use crate::led::types::{Position, Range};

/// Represents the state of a cursor in the editor, including its position,
/// optional selection range, and the buffer it belongs to.
pub struct State {
    /// The current position of the cursor.
    pub(crate) position: Position,
    /// The currently selected range, if any.
    pub(crate) selection: Option<Range>,
    /// The identifier of the buffer the cursor is associated with.
    pub(crate) buffer_id: super::buffer::ID,
}