//! Rope-based text buffer for efficient large document handling
//!
//! This module provides a `TextBuffer` wrapper around ropey::Rope that offers:
//! - O(log n) insertions and deletions
//! - Efficient slicing for large documents
//! - Line-based operations
//! - UTF-8 correctness
//! - Conversion to/from String

use crate::semantic::ast::{Position, SourceRange};
use ropey::{Rope, RopeSlice};

/// A rope-based text buffer for efficient large document handling.
///
/// Wraps ropey::Rope to provide a cleaner API for the editor while
/// maintaining O(log n) performance for insertions and deletions.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
}

impl TextBuffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    /// Create a buffer from a String
    pub fn from_string(content: String) -> Self {
        Self {
            rope: Rope::from_str(&content),
        }
    }

    /// Create a buffer from existing content with known length (for performance)
    pub fn from_content(content: &str) -> Self {
        Self {
            rope: Rope::from_str(content),
        }
    }

    /// Get the content as a String
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Get the raw Rope reference for advanced operations
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Get the length in bytes
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Get the length in characters
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the number of lines
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0
    }

    /// Get a slice of the buffer from start to end byte offsets
    pub fn slice(&self, start: usize, end: usize) -> RopeSlice<'_> {
        self.rope.slice(start..end)
    }

    /// Insert text at the given byte offset
    ///
    /// # Arguments
    /// * `offset` - Byte offset where to insert
    /// * `text` - Text to insert
    ///
    /// # Performance
    /// O(log n) where n is the document size
    pub fn insert(&mut self, offset: usize, text: &str) {
        let offset = offset.min(self.rope.len_bytes());
        self.rope.insert(offset, text);
    }

    /// Insert text at the given character offset
    pub fn insert_char(&mut self, char_offset: usize, text: &str) {
        let byte_offset = if char_offset >= self.rope.len_chars() {
            self.rope.len_bytes()
        } else {
            self.rope.char_to_byte(char_offset)
        };
        self.rope.insert(byte_offset, text);
    }

    /// Delete text in the given byte offset range
    ///
    /// # Arguments
    /// * `start` - Start byte offset (inclusive)
    /// * `end` - End byte offset (exclusive)
    ///
    /// # Performance
    /// O(log n) where n is the document size
    pub fn delete(&mut self, start: usize, end: usize) {
        let start = start.min(self.rope.len_bytes());
        let end = end.min(self.rope.len_bytes());
        if start < end {
            self.rope.remove(start..end);
        }
    }

    /// Delete text in the given character offset range
    pub fn delete_char_range(&mut self, start_char: usize, end_char: usize) {
        let start = self
            .rope
            .char_to_byte(start_char.min(self.rope.len_chars()));
        let end = self.rope.char_to_byte(end_char.min(self.rope.len_chars()));
        if start < end {
            self.rope.remove(start..end);
        }
    }

    /// Replace text in the given range with new text
    ///
    /// # Arguments
    /// * `range` - SourceRange specifying the range to replace
    /// * `new_text` - Text to insert at the replacement location
    pub fn replace(&mut self, range: &SourceRange, new_text: &str) {
        let start = range.start.offset.min(self.rope.len_bytes());
        let end = range.end.offset.min(self.rope.len_bytes());
        if start < end {
            self.rope.remove(start..end);
        }
        self.rope.insert(start, new_text);
    }

    /// Get the character at the given byte offset
    pub fn char_at(&self, byte_offset: usize) -> Option<char> {
        if byte_offset >= self.rope.len_bytes() {
            return None;
        }
        self.rope.chars_at(byte_offset).next()
    }

    /// Get the line at the given line index (0-indexed)
    pub fn line(&self, line_index: usize) -> RopeSlice<'_> {
        self.rope.line(line_index)
    }

    /// Get line number from byte offset
    ///
    /// Returns (line, column) where both are 0-indexed
    pub fn offset_to_line_col(&self, byte_offset: usize) -> (usize, usize) {
        let byte_offset = byte_offset.min(self.rope.len_bytes());
        let mut current_line = 0;
        let mut offset = 0;

        for line_slice in self.rope.lines() {
            let line_len = line_slice.len_bytes();
            if offset + line_len > byte_offset {
                return (current_line, byte_offset - offset);
            }
            offset += line_len;
            current_line += 1;
        }
        (current_line, byte_offset.saturating_sub(offset))
    }

    /// Get byte offset from line and column
    pub fn line_col_to_offset(&self, line: usize, col: usize) -> usize {
        let mut current_line = 0;
        let mut offset = 0;

        for line_slice in self.rope.lines() {
            if current_line == line {
                return offset + col.min(line_slice.len_bytes());
            }
            offset += line_slice.len_bytes() + 1; // +1 for newline
            current_line += 1;
        }
        self.rope.len_bytes()
    }

    /// Get the Position from a byte offset
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let (line, column) = self.offset_to_line_col(offset);
        Position::new(offset, line as u32, column as u32)
    }

    /// Get the byte offset from a Position
    pub fn position_to_offset(&self, pos: Position) -> usize {
        self.line_col_to_offset(pos.line as usize, pos.column as usize)
    }

    /// Find a pattern and return all match positions
    pub fn find(&self, pattern: &str, case_sensitive: bool) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let text = self.rope.to_string();

        let search_text = if case_sensitive {
            text.as_str()
        } else {
            // Case-insensitive search
            unimplemented!("Use ropey's built-in case-insensitive search")
        };

        let mut start = 0;
        while let Some(pos) = search_text[start..].find(pattern) {
            let abs_pos = start + pos;
            matches.push((abs_pos, abs_pos + pattern.len()));
            start = abs_pos + 1;
        }
        matches
    }

    /// Append text to the end of the buffer
    pub fn append(&mut self, text: &str) {
        self.rope.insert(self.rope.len_bytes(), text);
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.rope = Rope::new();
    }

    /// Get the line_breaks (newlines) in the document
    pub fn line_breaks(&self) -> Vec<usize> {
        let mut breaks = Vec::new();
        let mut offset = 0;
        for line_slice in self.rope.lines() {
            offset += line_slice.len_bytes();
            breaks.push(offset);
            offset += 1; // for newline
        }
        breaks
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl From<String> for TextBuffer {
    fn from(s: String) -> Self {
        Self::from_string(s)
    }
}

impl From<&str> for TextBuffer {
    fn from(s: &str) -> Self {
        Self::from_content(s)
    }
}

impl From<TextBuffer> for String {
    fn from(buffer: TextBuffer) -> Self {
        buffer.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = TextBuffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len_bytes(), 0);
    }

    #[test]
    fn test_offset_to_position() {
        let buffer = TextBuffer::from_string("Line 1\nLine 2\nLine 3".to_string());
        let pos7 = buffer.offset_to_position(7);
        assert_eq!(pos7.line, 1, "byte 7 is start of Line 2");
        assert_eq!(pos7.column, 0);
        let pos0 = buffer.offset_to_position(0);
        assert_eq!(pos0.line, 0);
        assert_eq!(pos0.column, 0);
        let pos6 = buffer.offset_to_position(6);
        assert_eq!(pos6.line, 0, "byte 6 is last char of Line 1");
        assert_eq!(pos6.column, 6);
    }

    #[test]
    fn test_insert() {
        let mut buffer = TextBuffer::from_string("Hello, World!".to_string());
        buffer.insert(5, " Beautiful");
        assert_eq!(buffer.to_string(), "Hello Beautiful, World!");
    }

    #[test]
    fn test_delete() {
        let mut buffer = TextBuffer::from_string("Hello, World!".to_string());
        buffer.delete(5, 6);
        assert_eq!(buffer.to_string(), "Hello World!");
    }

    #[test]
    fn test_replace() {
        let mut buffer = TextBuffer::from_string("Hello, World!".to_string());
        let range = SourceRange::new(Position::new(0, 0, 0), Position::new(5, 0, 5));
        buffer.replace(&range, "Hi");
        assert_eq!(buffer.to_string(), "Hi, World!");
    }

    #[test]
    fn test_lines() {
        let buffer = TextBuffer::from_string("Line 1\nLine 2\nLine 3".to_string());
        assert_eq!(buffer.len_lines(), 3);
        assert_eq!(buffer.line(0).to_string(), "Line 1\n");
        assert_eq!(buffer.line(1).to_string(), "Line 2\n");
        assert_eq!(buffer.line(2).to_string(), "Line 3");
    }

    #[test]
    fn test_large_document() {
        // Create a 5MB+ document
        let large_content = "x".repeat(5 * 1024 * 1024);
        let buffer = TextBuffer::from_string(large_content.clone());
        assert_eq!(buffer.len_bytes(), 5 * 1024 * 1024);
        assert_eq!(buffer.to_string(), large_content);
    }

    #[test]
    fn test_insert_at_end() {
        let mut buffer = TextBuffer::from_string("Hello".to_string());
        buffer.insert(buffer.len_bytes(), " World!");
        assert_eq!(buffer.to_string(), "Hello World!");
    }

    #[test]
    fn test_insert_at_beginning() {
        let mut buffer = TextBuffer::from_string("World!".to_string());
        buffer.insert(0, "Hello ");
        assert_eq!(buffer.to_string(), "Hello World!");
    }

    #[test]
    fn test_clear() {
        let mut buffer = TextBuffer::from_string("Hello".to_string());
        buffer.clear();
        assert!(buffer.is_empty());
    }
}
