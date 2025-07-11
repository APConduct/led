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

impl State {
    /// Creates a new `State` with the specified position, selection, and buffer ID.
    ///
    /// # Arguments
    ///
    /// * `position` - The initial position of the cursor.
    /// * `selection` - An optional selection range.
    /// * `buffer_id` - The ID of the buffer the cursor is in.
    pub fn new(position: Position, selection: Option<Range>, buffer_id: super::buffer::ID) -> Self {
        Self {
            position,
            selection,
            buffer_id,
        }
    }

    /// Returns the current position of the cursor.
    pub fn position(&self) -> Position {
        self.position
    }

    /// Returns the current selection range, if any.
    pub fn selection(&self) -> Option<Range> {
        self.selection
    }

    /// Returns the ID of the buffer associated with this cursor state.
    pub fn buffer_id(&self) -> super::buffer::ID {
        self.buffer_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::led::types::{Position, Range};
    use crate::led::buffer;
    use uuid::Uuid;

    #[test]
    fn new_state_sets_fields_correctly() {
        let pos = Position { line: 3, column: 7 };
        let range = Some(Range {
            start: Position { line: 2, column: 1 },
            end: Position { line: 3, column: 7 },
        });
        let buffer_id = buffer::ID(Uuid::new_v4());
        let state = State::new(pos, range, buffer_id);
        assert_eq!(state.position, pos);
        assert_eq!(state.selection, range);
        assert_eq!(state.buffer_id, buffer_id);
    }

    #[test]
    fn position_returns_current_position() {
        let pos = Position { line: 0, column: 0 };
        let state = State::new(pos, None, buffer::ID(Uuid::new_v4()));
        assert_eq!(state.position(), pos);
    }

    #[test]
    fn selection_returns_none_when_no_selection() {
        let state = State::new(
            Position { line: 1, column: 1 },
            None,
            buffer::ID(Uuid::new_v4()),
        );
        assert_eq!(state.selection(), None);
    }

    #[test]
    fn selection_returns_some_when_selection_exists() {
        let range = Range {
            start: Position { line: 0, column: 0 },
            end: Position { line: 1, column: 2 },
        };
        let state = State::new(
            Position { line: 0, column: 0 },
            Some(range),
            buffer::ID(Uuid::new_v4()),
        );
        assert_eq!(state.selection(), Some(range));
    }

    #[test]
    fn buffer_id_returns_correct_id() {
        let buffer_id = buffer::ID(Uuid::new_v4());
        let state = State::new(
            Position { line: 2, column: 2 },
            None,
            buffer_id,
        );
        assert_eq!(state.buffer_id(), buffer_id);
    }
}