use crate::editor::commands::{Command, FormatKind};
use crate::editor::cursor::Cursor;
use crate::editor::selection::SelectionState;
use crate::semantic::ast::Position;
use crate::semantic::position::Selection;
use std::collections::HashMap;
use uuid::Uuid;

pub struct EditorService {
    cursors: HashMap<Uuid, Cursor>,
    selections: HashMap<Uuid, SelectionState>,
}

impl EditorService {
    pub fn new() -> Self {
        Self {
            cursors: HashMap::new(),
            selections: HashMap::new(),
        }
    }

    pub fn cursor(&self, id: Uuid) -> Option<Position> {
        self.cursors.get(&id).map(|c| c.position())
    }

    pub fn set_cursor(&mut self, id: Uuid, pos: Position) {
        self.cursors.insert(id, Cursor::new(pos));
    }

    pub fn selection(&self, id: Uuid) -> Option<Selection> {
        self.selections.get(&id).and_then(|s| s.selection())
    }

    pub fn set_selection(&mut self, id: Uuid, sel: Selection) {
        let state = self.selections.entry(id).or_default();
        state.set_selection(Some(sel));
    }

    pub fn clear_selection(&mut self, id: Uuid) {
        if let Some(state) = self.selections.get_mut(&id) {
            state.clear_selection();
        }
    }

    pub fn is_anchor_mode(&self, id: Uuid) -> bool {
        self.selections
            .get(&id)
            .map(|s| s.is_anchor_mode())
            .unwrap_or(false)
    }

    pub fn set_anchor_mode(&mut self, id: Uuid, mode: bool) {
        let state = self.selections.entry(id).or_default();
        state.set_anchor_mode(mode);
    }

    pub fn start_selection(&mut self, id: Uuid, pos: Position) {
        let state = self.selections.entry(id).or_default();
        state.start_selection(pos);
        self.cursors.insert(id, Cursor::new(pos));
    }

    pub fn update_selection(&mut self, id: Uuid, head: Position) {
        let state = self.selections.entry(id).or_default();
        state.update_selection(head);
        self.cursors.insert(id, Cursor::new(head));
    }

    pub fn move_cursor(&mut self, id: Uuid, pos: Position) {
        self.cursors.insert(id, Cursor::new(pos));
    }

    pub fn has_selection(&self, id: Uuid) -> bool {
        self.selections
            .get(&id)
            .map(|s| s.has_selection())
            .unwrap_or(false)
    }

    pub fn selected_range(&self, id: Uuid) -> Option<(Position, Position)> {
        self.selections.get(&id).and_then(|s| s.selected_range())
    }

    pub fn remove_editor(&mut self, id: Uuid) {
        self.cursors.remove(&id);
        self.selections.remove(&id);
    }
}

impl Default for EditorService {
    fn default() -> Self {
        Self::new()
    }
}

pub fn execute_format(
    content: &mut String,
    range: (Position, Position),
    format: FormatKind,
) -> Command {
    let start = range.0.offset.min(content.len());
    let end = range.1.offset.min(content.len());
    let _old_text = content[start..end].to_string();
    let cmd = Command::Format {
        range: crate::semantic::ast::SourceRange::new(range.0, range.1),
        format,
    };
    cmd.apply(content);
    cmd
}
