//noinspection ALL
/// Re-exports the buffer ID type from the types' module.
pub use super::types::buffer::ID;
use super::commands::editor::Command;

/// Module containing metadata related to buffers, such as file path, language, and timestamps.
pub mod meta {
    /// Metadata associated with a buffer, including file path, language, modification status, and creation time.
    #[derive(Debug, Clone)]
    pub struct Data {
        /// Optional file path associated with the buffer.
        pub file_path: Option<String>,
        /// Optional programming language of the buffer.
        pub language: Option<String>,
        /// Indicates whether the buffer has been modified.
        pub modified: bool,
        /// Timestamp of when the buffer was created.
        pub created_at: std::time::SystemTime,
    }
}

/// Module containing the editor state and buffer management logic.
pub mod editor {
    use std::collections::HashMap;
    use crate::led::buffer::meta;

    /// Represents the state of the editor, including buffers, metadata, cursors, and undo/redo stacks.
    pub struct State {
        /// Maps buffer IDs to their corresponding piece tables.
        buffers: HashMap<super::ID, super::super::piece::Table>,
        /// Maps buffer IDs to their metadata.
        buffer_metadata: HashMap<super::ID, meta::Data>,
        /// Maps buffer IDs to their cursor states.
        cursors: HashMap<super::ID, super::super::cursor::State>,
        /// The currently active buffer, if any.
        active_buffer: Option<super::ID>,

        /// Undo stack for each buffer.
        undo_stack: HashMap<super::ID, Vec<super::Command>>,
        /// Redo stack for each buffer.
        redo_stack: HashMap<super::ID, Vec<super::Command>>,
    }

    impl State {
        /// Creates a new editor state with no buffers.
        pub fn new() -> Self {
            Self {
                buffers: HashMap::new(),
                buffer_metadata: HashMap::new(),
                cursors: HashMap::new(),
                active_buffer: None,
                undo_stack: HashMap::new(),
                redo_stack: HashMap::new(),
            }
        }

        /// Creates a new buffer with the given content and initializes its metadata, cursor, and undo/redo stacks.
        ///
        /// # Arguments
        ///
        /// * `content` - The initial content of the buffer.
        ///
        /// # Returns
        ///
        /// The unique ID of the newly created buffer.
        pub fn create_buffer(&mut self, content: String) -> super::ID {
            let buffer_id = super::ID::new();
            let piece_table = super::super::piece::Table::new(content);
            self.buffers.insert(buffer_id, piece_table);

            self.buffer_metadata.insert(buffer_id, meta::Data {
                file_path: None,
                language: None,
                modified: false,
                created_at: std::time::SystemTime::now(),
            });
            self.cursors.insert(buffer_id, super::super::cursor::State{
                position: super::super::types::Position { line: 0, column: 0 },
                selection: None,
                buffer_id,
            });
            self.undo_stack.insert(buffer_id, Vec::new());
            self.redo_stack.insert(buffer_id, Vec::new());
            if self.active_buffer.is_none() { self.active_buffer = Some(buffer_id); }
            buffer_id
        }

        /// Executes an editor command, such as inserting or deleting text, moving the cursor, or saving a buffer.
        ///
        /// # Arguments
        ///
        /// * `command` - The command to execute.
        ///
        /// # Errors
        ///
        /// Returns an error if the command cannot be executed.
        pub fn execute_command(&mut self, command: super::Command) -> anyhow::Result<()> {
            match command {
                super::Command::InsertText {buffer_id, offset, text} => {
                    if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                        buffer.insert(offset, &text)?;
                        self.mark_buffer_modified(buffer_id);
                    }
                }
                super::Command::DeleteText { buffer_id, start, length } => {
                    if let Some(buffer) = self.buffers.get_mut(&buffer_id) {
                        buffer.delete(start, length)?;
                        self.mark_buffer_modified(buffer_id);
                    }
                }
                super::Command::MoveCursor {buffer_id, position} => {
                    if let Some(cursor) = self.cursors.get_mut(&buffer_id) {
                        cursor.position = position;
                        cursor.selection = None;
                    }
                }
                super::Command::SetSelection {buffer_id, range} => {
                    if let Some(cursor) = self.cursors.get_mut(&buffer_id) {
                        cursor.selection = Some(range);
                    }
                }

                super::Command::NewBuffer {content} => {
                    self.create_buffer(content);
                }

                super::Command::SaveBuffer {buffer_id, file_path} => {
                    if let Some(meta) = self.buffer_metadata.get_mut(&buffer_id) {
                        meta.file_path = Some(file_path);
                        meta.modified = false;
                    }
                }
            }
            Ok(())
        }

        /// Marks the specified buffer as modified in its metadata.
        ///
        /// # Arguments
        ///
        /// * `buffer_id` - The ID of the buffer to mark as modified.
        fn mark_buffer_modified(&mut self, buffer_id: super::ID) {
            if let Some(meta) = self.buffer_metadata.get_mut(&buffer_id) {
                meta.modified = true;
            }
        }

        /// Retrieves the full text of the specified buffer, if it exists.
        ///
        /// # Arguments
        ///
        /// * `buffer_id` - The ID of the buffer.
        ///
        /// # Returns
        ///
        /// An `Option` containing the buffer's text, or `None` if the buffer does not exist.
        pub fn get_buffer_text(&self, buffer_id: super::ID) -> Option<String> {
            self.buffers.get(&buffer_id).map(|buffer| buffer.get_text(0, buffer.len()))
        }

        /// Returns the ID of the currently active buffer, if any.
        pub fn get_active_biffer(&self) -> Option<super::ID> {
            self.active_buffer
        }

        /// Retrieves the cursor state for the specified buffer, if it exists.
        ///
        /// # Arguments
        ///
        /// * `buffer_id` - The ID of the buffer.
        ///
        /// # Returns
        ///
        /// An `Option` containing a reference to the cursor state, or `None` if not found.
        pub fn get_cursor_state(&self, buffer_id: super::ID) -> Option<&super::super::cursor::State> {
            self.cursors.get(&buffer_id)
        }
    }
}