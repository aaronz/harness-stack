use crate::core::selection::Cursor;
use rope::Rope;
use std::time::Instant;

pub struct Buffer {
    rope: Rope,
    dirty: bool,
    last_save: Instant,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            dirty: false,
            last_save: Instant::now(),
        }
    }

    pub fn from_str(content: &str) -> Self {
        Self {
            rope: Rope::from(content),
            dirty: false,
            last_save: Instant::now(),
        }
    }

    pub fn insert(&mut self, cursor: Cursor, text: &str) -> Cursor {
        let offset = cursor.offset.min(self.rope.len_bytes());
        self.rope.insert(offset, text);
        self.dirty = true;
        Cursor::new(offset + text.len())
    }

    pub fn delete(&mut self, start: usize, end: usize) -> Cursor {
        let start = start.min(self.rope.len_bytes());
        let end = end.min(self.rope.len_bytes());
        if start < end {
            self.rope.delete(start..end);
            self.dirty = true;
        }
        Cursor::new(start)
    }

    pub fn text(&self) -> &Rope {
        &self.rope
    }

    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    pub fn len(&self) -> usize {
        self.rope.len_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.rope.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_saved(&mut self) {
        self.dirty = false;
        self.last_save = Instant::now();
    }

    pub fn last_save(&self) -> Instant {
        self.last_save
    }

    pub fn byte_at(&self, offset: usize) -> Option<u8> {
        self.rope.byte_at(offset)
    }

    pub fn slice(&self, range: std::ops::Range<usize>) -> String {
        self.rope.slice(range).to_string()
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_insert() {
        let mut buffer = Buffer::new();
        let cursor = buffer.insert(Cursor::new(0), "Hello");
        assert_eq!(buffer.to_string(), "Hello");
        assert_eq!(cursor.offset, 5);
    }

    #[test]
    fn test_buffer_delete() {
        let mut buffer = Buffer::from_str("Hello World");
        let cursor = buffer.delete(5, 11);
        assert_eq!(buffer.to_string(), "Hello");
        assert_eq!(cursor.offset, 5);
    }

    #[test]
    fn test_buffer_dirty() {
        let mut buffer = Buffer::new();
        assert!(!buffer.is_dirty());
        buffer.insert(Cursor::new(0), "test");
        assert!(buffer.is_dirty());
        buffer.mark_saved();
        assert!(!buffer.is_dirty());
    }
}
