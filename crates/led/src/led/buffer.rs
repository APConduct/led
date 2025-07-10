pub use super::types::buffer::ID;
use super::commands::editor::Command;
pub mod meta {
    #[derive(Debug, Clone)]
    pub struct Data {
        pub file_path: Option<String>,
        pub language: Option<String>,
        pub modified: bool,
        pub created_at: std::time::SystemTime,
    }
}

pub mod editor {
    use std::collections::HashMap;
    use crate::led::buffer::meta;

    pub struct State {
        buffers: HashMap<super::ID, super::super::piece::Table>,
        buffer_metadata: HashMap<super::ID, super::meta::Data>,
        cursors: HashMap<super::ID, super::super::cursor::State>,
        active_buffer: Option<super::ID>,

        undo_stack: HashMap<super::ID, Vec<super::Command>>,
        redo_stack: HashMap<super::ID, Vec<super::Command>>,
    }

    impl State {
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
        
        fn mark_buffer_modified(&mut self, buffer_id: super::ID) {
            if let Some(meta) = self.buffer_metadata.get_mut(&buffer_id) { 
                meta.modified = true;
            }
        }
        
        fn get_buffer_text(&self, buffer_id: super::ID) -> Option<String> {
            self.buffers.get(&buffer_id).map(|buffer| buffer.get_text(0, buffer.len()))
        }
        
        pub fn get_active_biffer(&self) -> Option<super::ID> {
            self.active_buffer
        }
        
        pub fn get_cursor_state(&self, buffer_id: super::ID) -> Option<&super::super::cursor::State> {
            self.cursors.get(&buffer_id)
        }
    }
}

