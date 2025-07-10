use serde::{Deserialize, Serialize};

/// Module containing buffer-related types.
pub mod buffer {
    /// Unique identifier for a buffer, backed by a UUID.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct ID(uuid::Uuid);

    impl ID {
        /// Creates a new unique buffer ID.
        pub fn new() -> Self {
            Self(uuid::Uuid::new_v4())
        }
    }
}

/// Module containing source buffer identifiers.
pub mod source {
    /// Identifies the source of a piece: either the original buffer or the add buffer.
    #[derive(Debug, Clone, Copy)]
    pub enum ID {
        /// The original buffer.
        Original,
        /// The add buffer (for inserted text).
        Add,
    }
}

/// Represents a position in the document (line and column).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    /// Line number (zero-based).
    pub line: usize,
    /// Column number (zero-based).
    pub column: usize,
}

/// Represents a range in the document, defined by start and end positions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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