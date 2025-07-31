use super::types::*;
use anyhow::Result as AnyResult;

/// Module containing line-related structures.
pub mod line {
    /// Stores information about a line in the piece table.
    #[allow(unused)]
    #[derive(Debug, Clone)]
    pub struct Info {
        /// Index of the piece containing this line.
        pub(crate) piece_idx: usize,
        /// Offset within the piece where the line starts.
        pub(crate) offset_in_piece: usize,
        /// Absolute offset in the document.
        pub(crate) abs_offset: usize,
        /// Line number in the document.
        pub(crate) line_number: usize,
    }
}

/// Module containing the piece table implementation.
#[allow(unused)]
pub mod piece {
    /// Represents a piece in the piece table.
    #[derive(Debug, Clone, Copy)]
    pub struct Piece {
        /// Source buffer (original or add).
        pub source: ID,
        /// Start offset in the source buffer.
        pub start: usize,
        /// Length of the piece.
        pub length: usize,
        /// Number of line breaks in the piece.
        pub line_breaks: u32,
    }

    use crate::led::types::source::ID;
    use crate::led::util::count_line_breaks;
    use std::cmp::PartialEq;
    use std::collections::BTreeMap;

    /// Piece table data structure for efficient text editing.
    #[derive(Debug, Clone)]
    pub struct Table {
        /// The original buffer (read-only).
        original: String,
        /// The add buffer (for inserted text).
        add_buffer: String,

        /// List of pieces representing the current document.
        pieces: Vec<Piece>,

        /// Cache for line information.
        line_cache: Vec<super::line::Info>,
        /// Cache mapping character offsets to piece indices.
        char_to_piece_cache: BTreeMap<usize, usize>,

        /// Total length of the document.
        total_length: usize,
        /// Total number of lines in the document.
        total_lines: usize,

        /// Indicates if the line cache is dirty.
        line_cache_dirty: bool,
        /// Offset from which the char cache is dirty.
        char_cache_dirty_from: usize,
    }

    /// Implements equality for the ID type.
    impl PartialEq for ID {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (ID::Original, ID::Original) => true,
                (ID::Add, ID::Add) => true,
                _ => false,
            }
        }
    }

    impl Table {
        /// Creates a new piece table with the given initial text.
        ///
        /// # Arguments
        ///
        /// * `initial` - The initial text for the document.
        pub fn new(initial: String) -> Self {
            let line_breaks = count_line_breaks(&initial);
            let length = initial.len();

            let mut table = Self {
                original: initial,
                add_buffer: String::new(),
                pieces: vec![Piece {
                    source: ID::Original,
                    start: 0,
                    length,
                    line_breaks,
                }],
                line_cache: Vec::new(),
                char_to_piece_cache: BTreeMap::new(),
                total_length: length,
                total_lines: line_breaks as usize + 1,
                line_cache_dirty: true,
                char_cache_dirty_from: 0,
            };
            table.rebuild_caches();
            table
        }

        /// Returns the total length of the document.
        pub fn len(&self) -> usize {
            self.total_length
        }

        /// Returns the total number of lines in the document.
        pub fn lines(&self) -> usize {
            self.total_lines
        }

        /// Inserts text at the specified offset.
        ///
        /// # Arguments
        ///
        /// * `offset` - The position to insert the text.
        /// * `text` - The text to insert.
        ///
        /// # Errors
        ///
        /// Returns an error if the offset is out of bounds.
        pub fn insert(&mut self, offset: usize, text: &str) -> super::AnyResult<()> {
            if offset > self.total_length {
                return Err(anyhow::anyhow!("Insert offset out of bounds"));
            }
            let piece_idx = self.find_piece_containing_offset(offset);
            let add_start = self.add_buffer.len();
            self.add_buffer.push_str(text);
            let new_piece = Piece {
                source: ID::Add,
                start: add_start,
                length: text.len(),
                line_breaks: count_line_breaks(&text.to_string()),
            };
            if let Some(split_result) = self.split_piece_at(piece_idx, offset) {
                self.pieces.insert(split_result.insert_idx, new_piece);
            } else {
                self.pieces.push(new_piece);
            }
            self.total_length += text.len();
            self.total_lines += new_piece.line_breaks as usize;
            self.mark_caches_dirty_from(offset);
            self.coalesce_pieces_around(piece_idx);
            Ok(())
        }

        /// Deletes a range of text from the document.
        ///
        /// # Arguments
        ///
        /// * `start` - The start offset of the range to delete.
        /// * `length` - The length of the range to delete.
        ///
        /// # Errors
        ///
        /// Returns an error if the range is out of bounds.
        pub fn delete(&mut self, start: usize, length: usize) -> super::AnyResult<()> {
            if start + length > self.total_length {
                return Err(anyhow::anyhow!("Delete range out of bounds"));
            }
            let end = start + length;
            let start_piece_idx = self.find_piece_containing_offset(start);
            let end_piece_idx = self.find_piece_containing_offset(end);
            let deleted_lines = self.count_line_breaks_in_range(start, end);
            if start_piece_idx == end_piece_idx {
                self.delete_within_piece(start_piece_idx, start, end)?;
            } else {
                self.delete_across_pieces(start_piece_idx, end_piece_idx, start, end)?;
            }
            self.total_length -= length;
            self.total_lines -= deleted_lines;
            if self.pieces.is_empty() {
                self.total_lines = 1;
            }
            self.mark_caches_dirty_from(start);

            // Early return if table is now empty
            if self.pieces.is_empty() {
                return Ok(());
            }

            self.coalesce_pieces_around(start_piece_idx);
            Ok(())
        }

        /// Returns the text in the specified range.
        ///
        /// # Arguments
        ///
        /// * `start` - The start offset.
        /// * `length` - The length of the text to retrieve.
        pub fn get_text(&self, start: usize, length: usize) -> String {
            if start + length > self.total_length {
                return String::new(); // or handle error
            }
            let mut result = String::with_capacity(length);
            let mut current_offset = start;
            let end_offset = start + length;
            while current_offset < end_offset {
                let piece_idx = self.find_piece_containing_offset(current_offset);
                if piece_idx >= self.pieces.len() {
                    break; // Out of bounds, should not happen if logic is correct
                }
                let piece = &self.pieces[piece_idx];
                let piece_start_offset = self.get_piece_start_offset(piece_idx);
                let offset_in_piece = current_offset - piece_start_offset;
                let source_text = match piece.source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                };
                let piece_text = &source_text[piece.start..piece.start + piece.length];
                let available_in_piece = piece.length - offset_in_piece;
                let to_copy = (end_offset - current_offset).min(available_in_piece);
                result.push_str(&piece_text[offset_in_piece..offset_in_piece + to_copy]);
                current_offset += to_copy;
            }
            result
        }

        /// Converts an offset to a line and column position.
        ///
        /// # Arguments
        ///
        /// * `offset` - The character offset in the document.
        ///
        /// # Returns
        ///
        /// The corresponding `Position` (line and column).
        pub fn offset_to_position(&self, offset: usize) -> super::Position {
            if offset > self.total_length {
                return super::Position { line: 0, column: 0 };
            }
            let mut current_line = 0;
            let mut current_offset = 0;
            let mut last_line_start = 0;

            for piece in &self.pieces {
                let src_txt = match piece.source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                };
                let piece_txt = &src_txt[piece.start..piece.start + piece.length];
                let mut line_start = current_offset;
                for (i, ch) in piece_txt.char_indices() {
                    if current_offset + i == offset {
                        return super::Position {
                            line: current_line,
                            column: offset - line_start,
                        };
                    }
                    if ch == '\n' {
                        current_line += 1;
                        line_start = current_offset + i + 1;
                        last_line_start = line_start;
                    }
                }
                current_offset += piece.length;
            }
            // If offset is at the end of the document, return last line and column
            super::Position {
                line: current_line,
                column: offset - last_line_start,
            }
        }

        /// Converts a line and column position to an offset.
        ///
        /// # Arguments
        ///
        /// * `pos` - The position (line and column).
        ///
        /// # Returns
        ///
        /// The corresponding character offset.
        pub fn position_to_offset(&self, pos: super::Position) -> usize {
            let mut current_line = 0;
            let mut current_column = 0;
            let mut offset = 0;

            for piece in &self.pieces {
                let src_txt = match piece.source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                };
                let piece_txt = &src_txt[piece.start..piece.start + piece.length];
                for ch in piece_txt.chars() {
                    if current_line == pos.line && current_column == pos.column {
                        return offset;
                    }
                    if ch == '\n' {
                        current_line += 1;
                        current_column = 0;
                    } else {
                        current_column += 1;
                    }
                    offset += ch.len_utf8();
                }
            }
            // If position is past the end, return total length
            self.total_length
        }

        /// Finds the index of the piece containing the given offset.
        ///
        /// # Arguments
        ///
        /// * `offset` - The character offset.
        ///
        /// # Returns
        ///
        /// The index of the piece.
        fn find_piece_containing_offset(&self, offset: usize) -> usize {
            if offset > 0 && offset == self.total_length {
                return self.pieces.len() - 1;
            }
            let mut current_offset = 0;
            for (i, piece) in self.pieces.iter().enumerate() {
                if current_offset + piece.length > offset {
                    return i;
                }
                current_offset += piece.length;
            }
            self.pieces.len()
        }

        /// Returns the absolute start offset of the specified piece.
        ///
        /// # Arguments
        ///
        /// * `piece_idx` - The index of the piece.
        ///
        /// # Returns
        ///
        /// The absolute offset.
        fn get_piece_start_offset(&self, piece_idx: usize) -> usize {
            self.pieces[..piece_idx].iter().map(|p| p.length).sum()
        }

        /// Splits a piece at the given offset.
        ///
        /// # Arguments
        ///
        /// * `piece_idx` - The index of the piece to split.
        /// * `offset` - The offset at which to split.
        ///
        /// # Returns
        ///
        /// An option containing the split result.
        fn split_piece_at(
            &mut self,
            piece_idx: usize,
            offset: usize,
        ) -> Option<super::split::Result> {
            if piece_idx >= self.pieces.len() {
                return None; // Invalid piece index
            }
            let piece = self.pieces[piece_idx];
            let piece_start_offset = self.get_piece_start_offset(piece_idx);
            let offset_in_piece = offset - piece_start_offset;

            if offset_in_piece == 0 {
                return Some(super::split::Result {
                    insert_idx: piece_idx,
                });
            }

            if offset_in_piece >= piece.length {
                return Some(super::split::Result {
                    insert_idx: piece_idx + 1,
                });
            }

            let source_text = match piece.source {
                ID::Original => &self.original,
                ID::Add => &self.add_buffer,
            };
            let piece_text = &source_text[piece.start..piece.start + piece.length];
            let left_text = &piece_text[..offset_in_piece];
            let right_text = &piece_text[offset_in_piece..];

            let left_piece = Piece {
                source: piece.source,
                start: piece.start,
                length: offset_in_piece,
                line_breaks: count_line_breaks(&left_text.to_string()),
            };

            let right_piece = Piece {
                source: piece.source,
                start: piece.start + offset_in_piece,
                length: piece.length - offset_in_piece,
                line_breaks: count_line_breaks(&right_text.to_string()),
            };

            self.pieces[piece_idx] = left_piece;
            self.pieces.insert(piece_idx + 1, right_piece);

            Some(super::split::Result {
                insert_idx: piece_idx + 1,
            })
        }

        //noinspection ALL
        /// Deletes text within a single piece.
        ///
        /// # Arguments
        ///
        /// * `piece_idx` - The index of the piece.
        /// * `start` - The start offset.
        /// * `end` - The end offset.
        ///
        /// # Errors
        ///
        /// Returns an error if the range is out of bounds.
        // In `delete_within_piece`
        fn delete_within_piece(
            &mut self,
            piece_idx: usize,
            start: usize,
            end: usize,
        ) -> super::AnyResult<()> {
            if piece_idx >= self.pieces.len() {
                return Err(anyhow::anyhow!("Piece index out of bounds"));
            }

            let piece_start_offset = self.get_piece_start_offset(piece_idx);
            let offset_in_piece_start = start - piece_start_offset;
            let offset_in_piece_end = end - piece_start_offset;

            if offset_in_piece_start >= self.pieces[piece_idx].length
                || offset_in_piece_end > self.pieces[piece_idx].length
            {
                return Err(anyhow::anyhow!("Delete range out of bounds for the piece"));
            }

            let deleted_length = offset_in_piece_end - offset_in_piece_start;
            let piece = &mut self.pieces[piece_idx];
            let deleted_text = {
                let source_text = match piece.source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                };
                &source_text[piece.start + offset_in_piece_start..piece.start + offset_in_piece_end]
            };
            let deleted_line_breaks = count_line_breaks(&deleted_text.to_string());

            if deleted_length == piece.length {
                self.pieces.remove(piece_idx);
                return Ok(());
            }

            // Only mutate if not removing the whole piece
            let piece = &mut self.pieces[piece_idx];
            if offset_in_piece_start == 0 {
                piece.start += deleted_length;
                piece.length -= deleted_length;
                piece.line_breaks -= deleted_line_breaks;
            } else if offset_in_piece_end == piece.length {
                piece.length -= deleted_length;
                piece.line_breaks -= deleted_line_breaks;
            } else {
                let right_piece = Piece {
                    source: piece.source,
                    start: piece.start + offset_in_piece_end,
                    length: piece.length - offset_in_piece_end,
                    line_breaks: count_line_breaks(
                        &match piece.source {
                            ID::Original => &self.original,
                            ID::Add => &self.add_buffer,
                        }[piece.start + offset_in_piece_end..piece.start + piece.length]
                            .to_string(),
                    ),
                };
                piece.length = offset_in_piece_start;
                piece.line_breaks = count_line_breaks(
                    &match piece.source {
                        ID::Original => &self.original,
                        ID::Add => &self.add_buffer,
                    }[piece.start..piece.start + offset_in_piece_start]
                        .to_string(),
                );
                self.pieces.insert(piece_idx + 1, right_piece);
            }

            Ok(())
        }

        //noinspection ALL
        /// Deletes text across multiple pieces.
        ///
        /// # Arguments
        ///
        /// * `start_piece_idx` - The index of the first piece.
        /// * `end_piece_idx` - The index of the last piece.
        /// * `start` - The start offset.
        /// * `end` - The end offset.
        ///
        /// # Errors
        ///
        /// Returns an error if the indices are out of bounds.
        fn delete_across_pieces(
            &mut self,
            start_piece_idx: usize,
            mut end_piece_idx: usize,
            start: usize,
            end: usize,
        ) -> super::AnyResult<()> {
            if start_piece_idx >= self.pieces.len() || end_piece_idx >= self.pieces.len() {
                return Err(anyhow::anyhow!("Piece index out of bounds"));
            }

            let first_piece_start_offset = self.get_piece_start_offset(start_piece_idx);
            let offset_in_first_piece = start - first_piece_start_offset;
            let last_piece_start_offset = self.get_piece_start_offset(end_piece_idx);
            let offset_in_last_piece = end - last_piece_start_offset;

            // Mutate first piece: keep only the left part
            self.pieces[start_piece_idx].length = offset_in_first_piece;
            self.pieces[start_piece_idx].line_breaks = count_line_breaks(
                &match self.pieces[start_piece_idx].source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                }[self.pieces[start_piece_idx].start
                    ..self.pieces[start_piece_idx].start + offset_in_first_piece]
                    .to_string(),
            );

            // Mutate last piece: keep only the right part
            self.pieces[end_piece_idx].start += offset_in_last_piece;
            self.pieces[end_piece_idx].length -= offset_in_last_piece;
            self.pieces[end_piece_idx].line_breaks = count_line_breaks(
                &match self.pieces[end_piece_idx].source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                }[self.pieces[end_piece_idx].start
                    ..self.pieces[end_piece_idx].start + self.pieces[end_piece_idx].length]
                    .to_string(),
            );

            // Remove all pieces between first and last (exclusive)
            if end_piece_idx > start_piece_idx + 1 {
                let drain_start = start_piece_idx + 1;
                let drain_end = end_piece_idx;
                self.pieces.drain(drain_start..drain_end);
                end_piece_idx = drain_start;
            }

            // Remove empty pieces, highest index first
            let mut to_remove = Vec::new();
            if self
                .pieces
                .get(end_piece_idx)
                .map_or(false, |p| p.length == 0)
            {
                to_remove.push(end_piece_idx);
            }
            if self
                .pieces
                .get(start_piece_idx)
                .map_or(false, |p| p.length == 0)
                && start_piece_idx != end_piece_idx
            {
                to_remove.push(start_piece_idx);
            }
            to_remove.sort_unstable_by(|a, b| b.cmp(a));
            for idx in to_remove {
                if idx < self.pieces.len() {
                    self.pieces.remove(idx);
                }
            }

            if self.pieces.is_empty() {
                return Ok(());
            }

            Ok(())
        }

        /// Counts the number of line breaks in a given range.
        ///
        /// # Arguments
        ///
        /// * `start` - The start offset.
        /// * `end` - The end offset.
        ///
        /// # Returns
        ///
        /// The number of line breaks.
        fn count_line_breaks_in_range(&self, start: usize, end: usize) -> usize {
            let text = self.get_text(start, end - start);
            count_line_breaks(&text) as usize
        }

        /// Attempts to merge adjacent pieces if possible.
        ///
        /// # Arguments
        ///
        /// * `piece_idx` - The index around which to coalesce.
        fn coalesce_pieces_around(&mut self, piece_idx: usize) {
            if self.pieces.len() < 2 || piece_idx == 0 || piece_idx >= self.pieces.len() {
                return;
            }
            let can_merge = {
                let prev = &self.pieces[piece_idx - 1];
                let curr = &self.pieces[piece_idx];
                prev.source == curr.source && prev.start + prev.length == curr.start
            };
            if can_merge {
                let (prev, curr) = self.pieces.split_at_mut(piece_idx);
                prev[piece_idx - 1].length += curr[0].length;
                prev[piece_idx - 1].line_breaks += curr[0].line_breaks;
                self.pieces.remove(piece_idx);
            }
        }

        /// Marks caches as dirty from a given offset.
        ///
        /// # Arguments
        ///
        /// * `offset` - The offset from which caches are dirty.
        fn mark_caches_dirty_from(&mut self, offset: usize) {
            self.line_cache_dirty = true;
            self.char_cache_dirty_from = offset;
        }

        /// Rebuilds the line and character caches.
        fn rebuild_caches(&mut self) {
            self.line_cache.clear();
            let mut current_offset = 0;
            let mut current_line = 0;

            for (piece_idx, piece) in self.pieces.iter().enumerate() {
                if current_line % 64 == 0 {
                    self.line_cache.push(super::line::Info {
                        piece_idx,
                        offset_in_piece: 0,
                        abs_offset: current_offset,
                        line_number: current_line,
                    })
                }

                current_offset += piece.length;
                current_line += piece.line_breaks as usize;
            }

            self.line_cache_dirty = false;
            self.char_cache_dirty_from = usize::MAX;
        }
    }
}

/// Module for split operation results.
mod split {
    /// Result of splitting a piece.
    pub(crate) struct Result {
        /// Index at which to insert the new piece.
        pub(crate) insert_idx: usize,
    }
}

#[cfg(test)]
mod tests {
    use super::piece::Table;

    #[test]
    fn new_table_has_correct_length_and_lines() {
        let text = String::from("Hello\nWorld\n");
        let table = Table::new(text.clone());
        assert_eq!(table.len(), text.len());
        assert_eq!(table.lines(), 3);
    }

    #[test]
    fn insert_text_at_start() {
        let mut table = Table::new("World".to_string());
        table.insert(0, "Hello ").unwrap();
        assert_eq!(table.get_text(0, table.len()), "Hello World");
    }

    #[test]
    fn insert_text_at_end() {
        let mut table = Table::new("Hello".to_string());
        table.insert(5, " World").unwrap();
        assert_eq!(table.get_text(0, table.len()), "Hello World");
    }

    #[test]
    fn insert_text_in_middle() {
        let mut table = Table::new("Helo World".to_string());
        table.insert(2, "l").unwrap();
        assert_eq!(table.get_text(0, table.len()), "Hello World");
    }

    #[test]
    fn insert_text_with_newlines_updates_lines() {
        let mut table = Table::new("Hello".to_string());
        table.insert(5, "\nWorld\n!").unwrap();
        assert_eq!(table.lines(), 3);
        assert_eq!(table.get_text(0, table.len()), "Hello\nWorld\n!");
    }

    #[test]
    fn delete_text_at_start() {
        let mut table = Table::new("Hello World".to_string());
        table.delete(0, 6).unwrap();
        assert_eq!(table.get_text(0, table.len()), "World");
    }

    #[test]
    fn delete_text_at_end() {
        let mut table = Table::new("Hello World".to_string());
        table.delete(5, 6).unwrap();
        assert_eq!(table.get_text(0, table.len()), "Hello");
    }

    #[test]
    fn delete_text_in_middle() {
        let mut table = Table::new("Hello cruel World".to_string());
        table.delete(6, 6).unwrap();
        assert_eq!(table.get_text(0, table.len()), "Hello World");
    }

    #[test]
    fn delete_entire_content_results_in_empty() {
        let mut table = Table::new("Hello".to_string());
        table.delete(0, 5).unwrap();
        assert_eq!(table.get_text(0, table.len()), "");
        assert_eq!(table.len(), 0);
        assert_eq!(table.lines(), 1);
    }

    #[test]
    fn get_text_out_of_bounds_returns_empty() {
        let table = Table::new("Hello".to_string());
        assert_eq!(table.get_text(10, 5), "");
    }

    #[test]
    fn offset_to_position_and_back() {
        let table = Table::new("Hello\nWorld\n!".to_string());
        let pos = table.offset_to_position(7);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);
        let offset = table.position_to_offset(pos);
        assert_eq!(offset, 7);
    }

    #[test]
    fn offset_to_position_at_end() {
        let table = Table::new("abc\ndef".to_string());
        let pos = table.offset_to_position(7);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 3);
    }

    #[test]
    fn position_to_offset_past_end_returns_total_length() {
        let table = Table::new("abc\ndef".to_string());
        let offset = table.position_to_offset(super::super::types::Position {
            line: 10,
            column: 10,
        });
        assert_eq!(offset, table.len());
    }

    #[test]
    fn insert_offset_out_of_bounds_returns_error() {
        let mut table = Table::new("abc".to_string());
        assert!(table.insert(10, "x").is_err());
    }

    #[test]
    fn delete_range_out_of_bounds_returns_error() {
        let mut table = Table::new("abc".to_string());
        assert!(table.delete(2, 5).is_err());
    }
}
