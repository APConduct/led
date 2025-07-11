use serde::{Deserialize, Serialize};

/// Module containing buffer-related types, such as unique buffer identifiers.
pub mod buffer {
    /// Unique identifier for a buffer, backed by a UUID.
    ///
    /// This struct is used to uniquely identify buffers within the system,
    /// ensuring that each buffer can be referenced and managed independently.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct ID(uuid::Uuid);

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