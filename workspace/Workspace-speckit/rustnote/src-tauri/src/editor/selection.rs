use crate::semantic::ast::Position;
use crate::semantic::position::Selection;

#[derive(Debug, Clone)]
pub struct SelectionState {
    selection: Option<Selection>,
    anchor_mode: bool,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            selection: None,
            anchor_mode: false,
        }
    }

    pub fn selection(&self) -> Option<Selection> {
        self.selection.clone()
    }

    pub fn set_selection(&mut self, sel: Option<Selection>) {
        self.selection = sel;
    }

    pub fn is_anchor_mode(&self) -> bool {
        self.anchor_mode
    }

    pub fn set_anchor_mode(&mut self, mode: bool) {
        self.anchor_mode = mode;
    }

    pub fn start_selection(&mut self, pos: Position) {
        self.selection = Some(Selection::new(pos, pos));
    }

    pub fn update_selection(&mut self, head: Position) {
        if let Some(ref sel) = self.selection {
            let anchor_pos = sel.anchor().position();
            self.selection = Some(Selection::new(anchor_pos, head));
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
        self.anchor_mode = false;
    }

    pub fn has_selection(&self) -> bool {
        self.selection
            .as_ref()
            .map(|s| !s.is_empty())
            .unwrap_or(false)
    }

    pub fn selected_range(&self) -> Option<(Position, Position)> {
        self.selection.as_ref().map(|s| (s.start(), s.end()))
    }
}

impl Default for SelectionState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn selection_from_positions(start: Position, end: Position) -> Selection {
    Selection::new(start, end)
}

pub fn empty_selection_at(pos: Position) -> Selection {
    Selection::new(pos, pos)
}

pub fn select_word_at(text: &str, offset: usize) -> (usize, usize) {
    let chars: Vec<char> = text.chars().collect();
    if offset >= chars.len() {
        return (offset, offset);
    }

    let mut start = offset;
    let mut end = offset;

    while start > 0 && !chars[start - 1].is_whitespace() {
        start -= 1;
    }

    while end < chars.len() && !chars[end].is_whitespace() {
        end += 1;
    }

    (start, end)
}

pub fn select_line_at(text: &str, offset: usize) -> (usize, usize) {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return (0, 0);
    }

    let mut line_start = 0;
    let mut line_end = bytes.len();
    let mut current_pos = 0;
    let mut last_line_start = 0;

    for (i, &b) in bytes.iter().enumerate() {
        if b == b'\n' {
            if current_pos <= offset && offset < i {
                line_start = current_pos;
                line_end = i;
                break;
            }
            last_line_start = i + 1;
            current_pos = i + 1;
        }
    }

    if line_end == bytes.len() {
        if last_line_start > 0 || bytes[bytes.len() - 1] != b'\n' {
            line_start = last_line_start;
        }
        if offset >= current_pos {
            line_start = current_pos;
        }
        line_end = bytes.len();
    }

    (line_start, line_end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_state_new() {
        let state = SelectionState::new();
        assert!(state.selection().is_none());
        assert!(!state.is_anchor_mode());
    }

    #[test]
    fn test_selection_state_set_selection() {
        let pos = Position::new(0, 0, 0);
        let mut state = SelectionState::new();
        state.set_selection(Some(Selection::new(pos, pos)));
        assert!(state.selection().is_some());
    }

    #[test]
    fn test_selection_state_anchor_mode() {
        let mut state = SelectionState::new();
        assert!(!state.is_anchor_mode());
        state.set_anchor_mode(true);
        assert!(state.is_anchor_mode());
    }

    #[test]
    fn test_selection_state_start_selection() {
        let pos = Position::new(5, 0, 5);
        let mut state = SelectionState::new();
        state.start_selection(pos);
        assert!(state.selection().is_some());
        assert!(state.selection().unwrap().is_empty());
    }

    #[test]
    fn test_selection_state_update_selection() {
        let start = Position::new(0, 0, 0);
        let end = Position::new(10, 0, 10);
        let mut state = SelectionState::new();
        state.start_selection(start);
        state.update_selection(end);
        let sel = state.selection().unwrap();
        assert!(!sel.is_empty());
    }

    #[test]
    fn test_selection_state_clear_selection() {
        let pos = Position::new(0, 0, 0);
        let mut state = SelectionState::new();
        state.set_selection(Some(Selection::new(pos, pos)));
        state.clear_selection();
        assert!(state.selection().is_none());
    }

    #[test]
    fn test_selection_state_has_selection() {
        let pos = Position::new(0, 0, 0);
        let mut state = SelectionState::new();
        assert!(!state.has_selection());
        state.set_selection(Some(Selection::new(pos, pos)));
        assert!(!state.has_selection());
        state.set_selection(Some(Selection::new(pos, Position::new(10, 0, 10))));
        assert!(state.has_selection());
    }

    #[test]
    fn test_selection_from_positions() {
        let start = Position::new(0, 0, 0);
        let end = Position::new(5, 0, 5);
        let sel = selection_from_positions(start, end);
        assert!(sel.is_forward());
    }

    #[test]
    fn test_empty_selection_at() {
        let pos = Position::new(5, 0, 5);
        let sel = empty_selection_at(pos);
        assert!(sel.is_empty());
    }

    #[test]
    fn test_select_word_at() {
        let text = "Hello world";
        let (start, end) = select_word_at(text, 3);
        assert_eq!(start, 0);
        assert_eq!(end, 5);
    }

    #[test]
    fn test_select_word_at_space() {
        let text = "Hello world";
        let (start, end) = select_word_at(text, 6);
        assert_eq!(start, 6);
        assert_eq!(end, 11);
    }

    #[test]
    fn test_select_line_at() {
        let text = "Line 1\nLine 2\nLine 3";
        let (start, end) = select_line_at(text, 7);
        assert_eq!(start, 7);
        assert_eq!(end, 13);
    }

    #[test]
    fn test_select_line_at_end() {
        let text = "Line 1\nLine 2";
        let (start, end) = select_line_at(text, 13);
        assert_eq!(start, 7);
        assert_eq!(end, 13);
    }
}
