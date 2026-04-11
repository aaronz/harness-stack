use crate::semantic::ast::Position;

pub struct TransformEngine;

impl TransformEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn source_to_html(&self, source: &str) -> String {
        comrak::markdown_to_html(source, &comrak::Options::default())
    }

    pub fn offset_to_position(&self, source: &str, offset: usize) -> Position {
        let mut line: u32 = 0;
        let mut column: u32 = 0;

        for (current_offset, ch) in source.chars().enumerate() {
            if current_offset >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }

        Position::new(offset, line, column)
    }

    pub fn position_to_offset(&self, source: &str, pos: Position) -> usize {
        let mut current_line: u32 = 0;
        let mut current_col: u32 = 0;
        let mut offset: usize = 0;

        for ch in source.chars() {
            if current_line == pos.line && current_col >= pos.column {
                break;
            }
            if ch == '\n' {
                current_line += 1;
                current_col = 0;
            } else {
                current_col += 1;
            }
            offset += 1;
        }

        offset
    }
}

impl Default for TransformEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_to_position() {
        let engine = TransformEngine::new();
        let source = "Line 1\nLine 2\nLine 3";
        let pos = engine.offset_to_position(source, 10);
        assert_eq!(pos.line, 1);
    }

    #[test]
    fn test_source_to_html() {
        let engine = TransformEngine::new();
        let html = engine.source_to_html("# Hello");
        assert!(html.contains("<h1>"));
    }
}
