use super::types::*;
use anyhow::Result as AnyResult;

/// Module containing line-related structures.
pub mod line {
    /// Stores information about a line in the piece table.
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

    use crate::led::util::count_line_breaks;
    use crate::led::types::source::ID;
    use std::cmp::PartialEq;
    use std::collections::BTreeMap;

    /// Piece table data structure for efficient text editing.
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
            self.mark_caches_dirty_from(start);
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
            if offset >= self.total_length {
                return super::Position { line: 0, column: 0 }; // or handle error
            }
            let mut current_line = 0;
            let mut current_offset = 0;

            for piece in &self.pieces {
                if current_offset + piece.length > offset {
                    let src_txt = match piece.source {
                        ID::Original => &self.original,
                        ID::Add => &self.add_buffer,
                    };

                    let piece_txt = &src_txt[piece.start..piece.start + piece.length];
                    let mut line_in_piece = current_line;

                    for (i, ch) in piece_txt.char_indices() {
                        if current_offset + i >= offset {
                            return super::Position {
                                line: line_in_piece,
                                column: offset - current_offset - i,
                            };
                        }
                        if ch == '\n' {
                            line_in_piece += 1;
                        }
                    }
                    return super::Position {
                        line: line_in_piece,
                        column: piece.length - (offset - current_offset),
                    };
                }
                current_offset += piece.length;
                current_line += piece.line_breaks as usize;
            }
            super::Position { line: 0, column: 0 } // Fallback, should not happen
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
            let mut current_offset = 0;

            for piece in &self.pieces {
                let src_txt = match piece.source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                };
                let piece_txt = &src_txt[piece.start..piece.start + piece.length];
                let mut line_start_offset = current_offset;
                let mut chars = piece_txt.char_indices().peekable();

                while let Some((i, ch)) = chars.next() {
                    if current_line == pos.line {
                        // We're on the correct line, so offset is at line_start_offset + column
                        // But don't go past the end of the piece
                        let line_end_offset = if let Some((next_i, _)) = chars.peek() {
                            current_offset + *next_i
                        } else {
                            current_offset + piece.length
                        };
                        let line_len = line_end_offset - line_start_offset;
                        return line_start_offset + pos.column.min(line_len);
                    }
                    if ch == '\n' {
                        current_line += 1;
                        line_start_offset = current_offset + i + 1;
                    }
                }
                current_offset += piece.length;
            }
            // If the position is past the end, return the total length
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

            let piece = &mut self.pieces[piece_idx];

            if offset_in_piece_start >= piece.length || offset_in_piece_end > piece.length {
                return Err(anyhow::anyhow!("Delete range out of bounds for the piece"));
            }

            let deleted_length = offset_in_piece_end - offset_in_piece_start;
            let deleted_text = {
                let source_text = match piece.source {
                    ID::Original => &self.original,
                    ID::Add => &self.add_buffer,
                };
                &source_text[piece.start + offset_in_piece_start..piece.start + offset_in_piece_end]
            };
            let deleted_line_breaks = count_line_breaks(&deleted_text.to_string());

            if deleted_length == piece.length {
                // Remove the whole piece
                self.pieces.remove(piece_idx);
            } else if offset_in_piece_start == 0 {
                // Delete from the start
                piece.start += deleted_length;
                piece.length -= deleted_length;
                piece.line_breaks -= deleted_line_breaks;
            } else if offset_in_piece_end == piece.length {
                // Delete from the end
                piece.length -= deleted_length;
                piece.line_breaks -= deleted_line_breaks;
            } else {
                // Delete in the middle: split into two pieces
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
            end_piece_idx: usize,
            start: usize,
            end: usize,
        ) -> super::AnyResult<()> {
            if start_piece_idx >= self.pieces.len() || end_piece_idx >= self.pieces.len() {
                return Err(anyhow::anyhow!("Piece index out of bounds"));
            }

            // Gather info for first piece
            let first_piece_start_offset = self.get_piece_start_offset(start_piece_idx);
            let offset_in_first_piece = start - first_piece_start_offset;
            let first_piece = &self.pieces[start_piece_idx];
            let _first_piece_delete_len = first_piece.length - offset_in_first_piece;
            let first_piece_source_text = match first_piece.source {
                ID::Original => &self.original,
                ID::Add => &self.add_buffer,
            };
            let first_piece_deleted_text = &first_piece_source_text
                [first_piece.start + offset_in_first_piece..first_piece.start + first_piece.length];
            let _first_piece_deleted_line_breaks =
                count_line_breaks(&first_piece_deleted_text.to_string());

            // Gather info for last piece
            let last_piece_start_offset = self.get_piece_start_offset(end_piece_idx);
            let offset_in_last_piece = end - last_piece_start_offset;
            let last_piece = &self.pieces[end_piece_idx];
            let last_piece_source_text = match last_piece.source {
                ID::Original => &self.original,
                ID::Add => &self.add_buffer,
            };
            let last_piece_deleted_text =
                &last_piece_source_text[last_piece.start..last_piece.start + offset_in_last_piece];
            let _last_piece_deleted_line_breaks =
                count_line_breaks(&last_piece_deleted_text.to_string());

            // Mutate first piece: keep only the left part
            {
                let piece = &mut self.pieces[start_piece_idx];
                piece.length = offset_in_first_piece;
                piece.line_breaks = count_line_breaks(
                    &match piece.source {
                        ID::Original => &self.original,
                        ID::Add => &self.add_buffer,
                    }[piece.start..piece.start + offset_in_first_piece]
                        .to_string(),
                );
            }

            // Mutate last piece: keep only the right part
            {
                let piece = &mut self.pieces[end_piece_idx];
                piece.start += offset_in_last_piece;
                piece.length -= offset_in_last_piece;
                piece.line_breaks = count_line_breaks(
                    &match piece.source {
                        ID::Original => &self.original,
                        ID::Add => &self.add_buffer,
                    }[piece.start..piece.start + piece.length]
                        .to_string(),
                );
            }

            // Remove all pieces between first and last (exclusive)
            if end_piece_idx > start_piece_idx + 1 {
                self.pieces.drain(start_piece_idx + 1..end_piece_idx);
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
            if piece_idx > 0 && piece_idx < self.pieces.len() {
                // Copy fields needed for comparison
                let can_merge = {
                    let prev = &self.pieces[piece_idx - 1];
                    let curr = &self.pieces[piece_idx];
                    prev.source == curr.source && prev.start + prev.length == curr.start
                };
                if can_merge {
                    // Mutate after checks
                    let (prev, curr) = self.pieces.split_at_mut(piece_idx);
                    prev[piece_idx - 1].length += curr[0].length;
                    prev[piece_idx - 1].line_breaks += curr[0].line_breaks;
                    self.pieces.remove(piece_idx);
                }
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