// Re-exports all items from the buffer module in the types module.
use super::types::buffer::*;

/// Module containing editor-related commands and their responses.
pub mod editor {
    use crate::led::types::{Position, Range};
    use serde::{Deserialize, Serialize};

    /// Represents an editor command, such as inserting or deleting text,
    /// moving the cursor, setting a selection, creating a new buffer, or saving a buffer.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::editor::*;
    use super::*;
    use crate::led::types::{Position, Range};
    use uuid::Uuid;

    #[test]
    fn command_insert_text_fields_are_set_correctly() {
        let buffer_id = ID(Uuid::new_v4());
        let offset = 5;
        let text = "hello".to_string();
        let cmd = Command::InsertText { buffer_id, offset, text: text.clone() };
        if let Command::InsertText { buffer_id: bid, offset: off, text: t } = cmd {
            assert_eq!(bid, buffer_id);
            assert_eq!(off, offset);
            assert_eq!(t, text);
        } else {
            panic!("Expected InsertText variant");
        }
    }

    #[test]
    fn command_delete_text_fields_are_set_correctly() {
        let buffer_id = ID(Uuid::new_v4());
        let start = 2;
        let length = 3;
        let cmd = Command::DeleteText { buffer_id, start, length };
        if let Command::DeleteText { buffer_id: bid, start: s, length: l } = cmd {
            assert_eq!(bid, buffer_id);
            assert_eq!(s, start);
            assert_eq!(l, length);
        } else {
            panic!("Expected DeleteText variant");
        }
    }

    #[test]
    fn command_move_cursor_fields_are_set_correctly() {
        let buffer_id = ID(Uuid::new_v4());
        let position = Position { line: 1, column: 2 };
        let cmd = Command::MoveCursor { buffer_id, position };
        if let Command::MoveCursor { buffer_id: bid, position: pos } = cmd {
            assert_eq!(bid, buffer_id);
            assert_eq!(pos, position);
        } else {
            panic!("Expected MoveCursor variant");
        }
    }

    #[test]
    fn command_set_selection_fields_are_set_correctly() {
        let buffer_id = ID(Uuid::new_v4());
        let range = Range {
            start: Position { line: 0, column: 0 },
            end: Position { line: 1, column: 1 },
        };
        let cmd = Command::SetSelection { buffer_id, range };
        if let Command::SetSelection { buffer_id: bid, range: r } = cmd {
            assert_eq!(bid, buffer_id);
            assert_eq!(r, range);
        } else {
            panic!("Expected SetSelection variant");
        }
    }

    #[test]
    fn command_new_buffer_content_is_set() {
        let content = "initial content".to_string();
        let cmd = Command::NewBuffer { content: content.clone() };
        if let Command::NewBuffer { content: c } = cmd {
            assert_eq!(c, content);
        } else {
            panic!("Expected NewBuffer variant");
        }
    }

    #[test]
    fn command_save_buffer_fields_are_set_correctly() {
        let buffer_id = ID(Uuid::new_v4());
        let file_path = "file.txt".to_string();
        let cmd = Command::SaveBuffer { buffer_id, file_path: file_path.clone() };
        if let Command::SaveBuffer { buffer_id: bid, file_path: fp } = cmd {
            assert_eq!(bid, buffer_id);
            assert_eq!(fp, file_path);
        } else {
            panic!("Expected SaveBuffer variant");
        }
    }

    #[test]
    fn response_fields_are_set_correctly() {
        let commands = vec![
            Command::NewBuffer { content: "abc".to_string() },
            Command::SaveBuffer { buffer_id: ID(Uuid::new_v4()), file_path: "a.txt".to_string() },
        ];
        let response = Response {
            commands: commands.clone(),
            cursor_moved: true,
            text_changed: false,
        };
        assert_eq!(response.commands, commands);
        assert!(response.cursor_moved);
        assert!(!response.text_changed);
    }

    #[test]
    fn command_serde_roundtrip() {
        let buffer_id = ID(Uuid::new_v4());
        let cmd = Command::InsertText {
            buffer_id,
            offset: 10,
            text: "serde".to_string(),
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let cmd_back: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{:?}", cmd), format!("{:?}", cmd_back));
    }
}