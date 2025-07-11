use serde::{Deserialize, Serialize};

/// Module containing buffer-related types, such as unique buffer identifiers.
pub mod buffer {
    /// Unique identifier for a buffer, backed by a UUID.
    ///
    /// This struct is used to uniquely identify buffers within the system,
    /// ensuring that each buffer can be referenced and managed independently.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct ID(pub uuid::Uuid);

    impl ID {
        /// Creates a new unique buffer ID using a randomly generated UUID (v4).
        ///
        /// # Returns
        /// A new `ID` instance with a unique UUID.
        pub fn new() -> Self {
            Self(uuid::Uuid::new_v4())
        }
    }
}

/// Module containing source buffer identifiers.
pub mod source {
    /// Identifies the source of a piece: either the original buffer or the add buffer.
    ///
    /// This enum is used to distinguish between the original buffer and the add buffer,
    /// which is typically used for inserted text.
    #[derive(Debug, Clone, Copy)]
    pub enum ID {
        /// The original buffer.
        Original,
        /// The add buffer (for inserted text).
        Add,
    }
}

/// Represents a position in the document (line and column).
///
/// The `Position` struct is used to specify a location within a document,
/// using zero-based line and column numbers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(PartialEq)]
pub struct Position {
    /// Line number (zero-based).
    pub line: usize,
    /// Column number (zero-based).
    pub column: usize,
}

/// Represents a range in the document, defined by start and end positions.
///
/// The `Range` struct is used to specify a span within a document,
/// using two `Position` values to indicate the start and end.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(PartialEq)]
pub struct Range {
    /// Start position of the range.
    pub start: Position,
    /// End position of the range.
    pub end: Position,
}

/// Module containing cursor-related types.
pub mod cursor {
    use crate::led::types::{Position, Range};

    /// Represents the state of a cursor, including its position, selection, and buffer ID.
    ///
    /// The `State` struct tracks the current position of the cursor, any active selection,
    /// and the identifier of the buffer the cursor is currently in.
    #[derive(Debug, Clone, Copy)]
    pub struct State {
        /// Current cursor position.
        pub pos: Position,
        /// Optional selection range.
        pub selection: Option<Range>,
        /// Identifier of the buffer the cursor is in.
        pub buffer_id: super::buffer::ID,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn creates_unique_buffer_ids() {
        let id1 = buffer::ID::new();
        let id2 = buffer::ID::new();
        assert_ne!(id1, id2, "Buffer IDs should be unique");
    }

    #[test]
    fn buffer_id_equality_and_hash() {
        let uuid = Uuid::new_v4();
        let id1 = buffer::ID(uuid);
        let id2 = buffer::ID(uuid);
        assert_eq!(id1, id2, "Buffer IDs with the same UUID should be equal");
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(id1);
        assert!(set.contains(&id2), "HashSet should recognize equal buffer IDs");
    }

    #[test]
    fn source_id_variants_are_distinct() {
        assert_ne!(source::ID::Original, source::ID::Add, "Source ID variants should be distinct");
    }

    #[test]
    fn position_equality_and_values() {
        let pos1 = Position { line: 0, column: 0 };
        let pos2 = Position { line: 0, column: 0 };
        let pos3 = Position { line: 1, column: 2 };
        assert_eq!(pos1, pos2, "Positions with same values should be equal");
        assert_ne!(pos1, pos3, "Positions with different values should not be equal");
    }

    #[test]
    fn range_equality_and_values() {
        let start = Position { line: 1, column: 2 };
        let end = Position { line: 3, column: 4 };
        let range1 = Range { start, end };
        let range2 = Range { start, end };
        let range3 = Range { start: end, end: start };
        assert_eq!(range1, range2, "Ranges with same positions should be equal");
        assert_ne!(range1, range3, "Ranges with different positions should not be equal");
    }

    #[test]
    fn cursor_state_with_and_without_selection() {
        let pos = Position { line: 2, column: 5 };
        let range = Range {
            start: Position { line: 1, column: 1 },
            end: Position { line: 2, column: 5 },
        };
        let buffer_id = buffer::ID::new();
        let state_with_selection = cursor::State {
            pos,
            selection: Some(range),
            buffer_id,
        };
        let state_without_selection = cursor::State {
            pos,
            selection: None,
            buffer_id,
        };
        assert_eq!(state_with_selection.pos, pos);
        assert_eq!(state_with_selection.selection, Some(range));
        assert_eq!(state_with_selection.buffer_id, buffer_id);
        assert_eq!(state_without_selection.selection, None);
    }

    #[test]
    fn position_and_range_serde_roundtrip() {
        let pos = Position { line: 10, column: 20 };
        let range = Range {
            start: Position { line: 1, column: 2 },
            end: Position { line: 3, column: 4 },
        };
        let pos_json = serde_json::to_string(&pos).unwrap();
        let pos_back: Position = serde_json::from_str(&pos_json).unwrap();
        assert_eq!(pos, pos_back);

        let range_json = serde_json::to_string(&range).unwrap();
        let range_back: Range = serde_json::from_str(&range_json).unwrap();
        assert_eq!(range, range_back);
    }

    #[test]
    fn buffer_id_serde_roundtrip() {
        let id = buffer::ID::new();
        let json = serde_json::to_string(&id).unwrap();
        let id_back: buffer::ID = serde_json::from_str(&json).unwrap();
        assert_eq!(id, id_back);
    }
}