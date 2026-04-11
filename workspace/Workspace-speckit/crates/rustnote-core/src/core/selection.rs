use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorAffinity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub offset: usize,
    pub affinity: CursorAffinity,
}

impl Cursor {
    pub fn new(offset: usize) -> Self {
        Self {
            offset,
            affinity: CursorAffinity::Left,
        }
    }

    pub fn with_affinity(mut self, affinity: CursorAffinity) -> Self {
        self.affinity = affinity;
        self
    }

    pub fn move_right(&mut self, max: usize) {
        if self.offset < max {
            self.offset += 1;
            self.affinity = CursorAffinity::Left;
        }
    }

    pub fn move_left(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
            self.affinity = CursorAffinity::Right;
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new(0)
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cursor({})", self.offset)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Cursor,
    pub head: Cursor,
}

impl Selection {
    pub fn new(anchor: Cursor, head: Cursor) -> Self {
        Self { anchor, head }
    }

    pub fn caret(cursor: Cursor) -> Self {
        Self {
            anchor: cursor,
            head: cursor,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    pub fn start(&self) -> Cursor {
        if self.anchor.offset <= self.head.offset {
            self.anchor
        } else {
            self.head
        }
    }

    pub fn end(&self) -> Cursor {
        if self.anchor.offset >= self.head.offset {
            self.anchor
        } else {
            self.head
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::caret(Cursor::default())
    }
}
