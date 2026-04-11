use crate::semantic::ast::Position;
use crate::semantic::ast::SourceRange;

/// Maps a source offset to its corresponding DOM position in the rendered output.
/// This is used by the frontend to synchronize cursor position between source and rendered view.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CursorMapping {
    /// Position in the source markdown text
    pub source_offset: usize,
    /// Position in the rendered DOM (after decorations are applied)
    pub dom_offset: usize,
    /// Line number in source (0-indexed)
    pub line: u32,
    /// Column number in source (0-indexed)
    pub column: u32,
}

impl CursorMapping {
    /// Create a new CursorMapping
    pub fn new(source_offset: usize, dom_offset: usize, line: u32, column: u32) -> Self {
        Self {
            source_offset,
            dom_offset,
            line,
            column,
        }
    }

    /// Convert a source offset to a DOM offset using binary search on sorted mappings
    pub fn source_to_dom(mappings: &[CursorMapping], source_offset: usize) -> usize {
        if mappings.is_empty() {
            return source_offset;
        }

        // Find the largest mapping with source_offset <= target
        let mut result = mappings[0].dom_offset;
        for mapping in mappings {
            if mapping.source_offset <= source_offset {
                // Calculate the delta between source and dom for this mapping
                let delta = source_offset - mapping.source_offset;
                result = mapping.dom_offset + delta;
            } else {
                break;
            }
        }
        result
    }

    /// Convert a DOM offset back to source offset
    pub fn dom_to_source(mappings: &[CursorMapping], dom_offset: usize) -> usize {
        if mappings.is_empty() {
            return dom_offset;
        }

        // Find the largest mapping with dom_offset <= target
        let mut result = mappings[0].source_offset;
        for mapping in mappings {
            if mapping.dom_offset <= dom_offset {
                let delta = dom_offset - mapping.dom_offset;
                result = mapping.source_offset + delta;
            } else {
                break;
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Left(Position),
    Right(Position),
}

impl Anchor {
    pub fn position(&self) -> Position {
        match self {
            Anchor::Left(pos) | Anchor::Right(pos) => *pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    anchor: Anchor,
    head: Position,
}

impl Selection {
    pub fn new(anchor: Position, head: Position) -> Self {
        let anchor = if head.offset >= anchor.offset {
            Anchor::Left(anchor)
        } else {
            Anchor::Right(anchor)
        };
        Self { anchor, head }
    }

    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    pub fn head(&self) -> Position {
        self.head
    }

    pub fn range(&self) -> SourceRange {
        let start = self.anchor.position();
        let end = self.head;
        if start.offset <= end.offset {
            SourceRange::new(start, end)
        } else {
            SourceRange::new(end, start)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor.position().offset == self.head.offset
    }

    pub fn is_forward(&self) -> bool {
        self.head.offset >= self.anchor.position().offset
    }

    pub fn start(&self) -> Position {
        if self.is_forward() {
            self.anchor.position()
        } else {
            self.head
        }
    }

    pub fn end(&self) -> Position {
        if self.is_forward() {
            self.head
        } else {
            self.anchor.position()
        }
    }

    pub fn from_positions(start: Position, end: Position) -> Self {
        Self::new(start, end)
    }

    pub fn empty_at(pos: Position) -> Self {
        Self::new(pos, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_forward() {
        let start = Position::new(0, 0, 0);
        let end = Position::new(5, 0, 5);
        let sel = Selection::new(start, end);
        assert!(sel.is_forward());
        assert_eq!(sel.start(), start);
        assert_eq!(sel.end(), end);
    }

    #[test]
    fn test_selection_backward() {
        let start = Position::new(5, 0, 5);
        let end = Position::new(0, 0, 0);
        let sel = Selection::new(start, end);
        assert!(!sel.is_forward());
    }

    #[test]
    fn test_selection_empty() {
        let pos = Position::new(5, 0, 5);
        let sel = Selection::empty_at(pos);
        assert!(sel.is_empty());
    }
}
