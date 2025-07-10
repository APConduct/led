use super::types::buffer::*;
pub mod editor {
    use crate::led::types::{Position, Range};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Command {
        InsertText {
            buffer_id: super::ID,
            offset: usize,
            text: String,
        },

        DeleteText {
            buffer_id: super::ID,
            start: usize,
            length: usize,
        },

        MoveCursor {
            buffer_id: super::ID,
            position: Position,
        },

        SetSelection {
            buffer_id: super::ID,
            range: Range,
        },

        NewBuffer {
            content: String,
        },

        SaveBuffer {
            buffer_id: super::ID,
            file_path: String,
        },
    }

    #[derive(Debug, Clone)]
    pub struct Response {
        pub commands: Vec<Command>,
        pub cursor_moved: bool,
        pub text_changed: bool,
    }
}
