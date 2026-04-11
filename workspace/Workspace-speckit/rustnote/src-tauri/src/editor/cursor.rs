use crate::semantic::ast::Position;

#[derive(Debug, Clone)]
pub struct Cursor {
    position: Position,
    visible: bool,
}

impl Cursor {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            visible: true,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn move_forward(&mut self, offset: usize) {
        self.position.offset = self.position.offset.saturating_add(offset);
        self.position.column += 1;
    }

    pub fn move_to_line_start(&mut self) {
        self.position.column = 0;
    }

    pub fn move_to_line_end(&mut self, line_length: usize) {
        self.position.column = line_length as u32;
        self.position.offset += line_length;
    }

    pub fn move_to_next_line(&mut self, current_line_length: usize) {
        self.position.line += 1;
        self.position.column = 0;
        self.position.offset += current_line_length + 1;
    }

    pub fn move_to_prev_line(&mut self, prev_line_length: usize) {
        if self.position.line > 0 {
            self.position.line -= 1;
            self.position.column = prev_line_length as u32;
            self.position.offset -= prev_line_length + 1;
        }
    }
}

pub fn move_cursor(word: &str, cursor: &mut Cursor, steps: isize) {
    let new_offset = (cursor.position().offset as isize + steps) as usize;
    cursor.set_position(Position::from_offset(new_offset.min(word.len())));
}

pub fn cursor_at_offset(offset: usize) -> Cursor {
    Cursor::new(Position::from_offset(offset))
}

pub fn cursor_at_end(text: &str) -> Cursor {
    Cursor::new(Position::new(
        text.len(),
        text.lines().count() as u32 - 1,
        0,
    ))
}

pub fn cursor_at_start() -> Cursor {
    Cursor::new(Position::new(0, 0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_new() {
        let pos = Position::new(5, 1, 5);
        let cursor = Cursor::new(pos);
        assert_eq!(cursor.position(), pos);
        assert!(cursor.is_visible());
    }

    #[test]
    fn test_cursor_set_position() {
        let pos1 = Position::new(0, 0, 0);
        let pos2 = Position::new(10, 0, 10);
        let mut cursor = Cursor::new(pos1);
        cursor.set_position(pos2);
        assert_eq!(cursor.position(), pos2);
    }

    #[test]
    fn test_cursor_visibility() {
        let cursor = Cursor::new(Position::new(0, 0, 0));
        assert!(cursor.is_visible());
        let mut cursor = Cursor::new(Position::new(0, 0, 0));
        cursor.set_visible(false);
        assert!(!cursor.is_visible());
    }

    #[test]
    fn test_cursor_at_offset() {
        let cursor = cursor_at_offset(5);
        assert_eq!(cursor.position().offset, 5);
    }

    #[test]
    fn test_cursor_at_start() {
        let cursor = cursor_at_start();
        assert_eq!(cursor.position().offset, 0);
        assert_eq!(cursor.position().line, 0);
        assert_eq!(cursor.position().column, 0);
    }

    #[test]
    fn test_cursor_at_end() {
        let text = "Line 1\nLine 2\nLine 3";
        let cursor = cursor_at_end(text);
        assert_eq!(cursor.position().line, 2);
    }

    #[test]
    fn test_move_cursor_forward() {
        let mut cursor = Cursor::new(Position::new(0, 0, 0));
        cursor.move_forward(1);
        assert_eq!(cursor.position().offset, 1);
        assert_eq!(cursor.position().column, 1);
    }

    #[test]
    fn test_move_cursor_to_line_start() {
        let mut cursor = Cursor::new(Position::new(10, 0, 10));
        cursor.move_to_line_start();
        assert_eq!(cursor.position().column, 0);
    }

    #[test]
    fn test_move_cursor_to_line_end() {
        let mut cursor = Cursor::new(Position::new(0, 0, 0));
        cursor.move_to_line_end(5);
        assert_eq!(cursor.position().column, 5);
    }

    #[test]
    fn test_move_cursor_to_next_line() {
        let mut cursor = Cursor::new(Position::new(5, 0, 5));
        cursor.move_to_next_line(5);
        assert_eq!(cursor.position().line, 1);
        assert_eq!(cursor.position().column, 0);
    }

    #[test]
    fn test_move_cursor_to_prev_line() {
        let mut cursor = Cursor::new(Position::new(12, 1, 5));
        cursor.move_to_prev_line(5);
        assert_eq!(cursor.position().line, 0);
        assert_eq!(cursor.position().column, 5);
    }
}
