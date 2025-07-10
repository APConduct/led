// Re-exports all items from the buffer module in the types module.
use super::types::buffer::*;

/// Module containing editor-related commands and their responses.
pub mod editor {
    use crate::led::types::{Position, Range};
    use serde::{Deserialize, Serialize};

    /// Represents an editor command, such as inserting or deleting text,
    /// moving the cursor, setting a selection, creating a new buffer, or saving a buffer.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Command {
        /// Command to insert text into a buffer at a specific offset.
        InsertText {
            /// The ID of the buffer to insert text into.
            buffer_id: super::ID,
            /// The offset at which to insert the text.
            offset: usize,
            /// The text to insert.
            text: String,
        },

        /// Command to delete a range of text from a buffer.
        DeleteText {
            /// The ID of the buffer to delete text from.
            buffer_id: super::ID,
            /// The start offset of the text to delete.
            start: usize,
            /// The length of the text to delete.
            length: usize,
        },

        /// Command to move the cursor to a new position in a buffer.
        MoveCursor {
            /// The ID of the buffer whose cursor should be moved.
            buffer_id: super::ID,
            /// The new position for the cursor.
            position: Position,
        },

        /// Command to set a selection range in a buffer.
        SetSelection {
            /// The ID of the buffer to set the selection in.
            buffer_id: super::ID,
            /// The range to select.
            range: Range,
        },

        /// Command to create a new buffer with the given content.
        NewBuffer {
            /// The initial content of the new buffer.
            content: String,
        },

        /// Command to save a buffer to a file.
        SaveBuffer {
            /// The ID of the buffer to save.
            buffer_id: super::ID,
            /// The file path to save the buffer to.
            file_path: String,
        },
    }

    /// Represents the response to an editor command, including any resulting commands,
    /// and flags indicating if the cursor moved or the text changed.
    #[derive(Debug, Clone)]
    pub struct Response {
        /// A list of commands generated as a result of the original command.
        pub commands: Vec<Command>,
        /// Indicates whether the cursor was moved as a result of the command.
        pub cursor_moved: bool,
        /// Indicates whether the text was changed as a result of the command.
        pub text_changed: bool,
    }
}