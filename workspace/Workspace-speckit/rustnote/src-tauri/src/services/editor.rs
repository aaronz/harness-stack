use crate::editor::commands::{Command, FormatKind};
use crate::editor::cursor::Cursor;
use crate::editor::selection::SelectionState;
use crate::semantic::ast::Position;
use crate::semantic::position::Selection;
use crate::services::{EditorResult, EditorServiceTrait};
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
}

impl Default for EditorService {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorServiceTrait for EditorService {
    fn cursor(&self, id: Uuid) -> Option<Position> {
        self.get_cursor(id)
    }

    fn set_cursor(&mut self, id: Uuid, pos: Position) {
        self.set_cursor_impl(id, pos);
    }

    fn selection(&self, id: Uuid) -> Option<Selection> {
        self.get_selection(id)
    }

    fn set_selection(&mut self, id: Uuid, sel: Selection) {
        self.set_selection_impl(id, sel);
    }

    fn clear_selection(&mut self, id: Uuid) {
        self.clear_selection_impl(id);
    }

    fn is_anchor_mode(&self, id: Uuid) -> bool {
        self.get_anchor_mode(id)
    }

    fn set_anchor_mode(&mut self, id: Uuid, mode: bool) {
        self.set_anchor_mode_impl(id, mode);
    }

    fn start_selection(&mut self, id: Uuid, pos: Position) {
        self.start_selection_impl(id, pos);
    }

    fn update_selection(&mut self, id: Uuid, head: Position) {
        self.update_selection_impl(id, head);
    }

    fn move_cursor(&mut self, id: Uuid, pos: Position) {
        self.move_cursor_impl(id, pos);
    }

    fn has_selection(&self, id: Uuid) -> bool {
        self.check_has_selection(id)
    }

    fn selected_range(&self, id: Uuid) -> Option<(Position, Position)> {
        self.get_selected_range(id)
    }

    fn remove_editor(&mut self, id: Uuid) {
        self.remove_editor_impl(id);
    }
}

impl EditorService {
    pub fn get_cursor(&self, id: Uuid) -> Option<Position> {
        self.cursors.get(&id).map(|c| c.position())
    }

    pub fn set_cursor_impl(&mut self, id: Uuid, pos: Position) {
        self.cursors.insert(id, Cursor::new(pos));
    }

    pub fn get_selection(&self, id: Uuid) -> Option<Selection> {
        self.selections.get(&id).and_then(|s| s.selection())
    }

    pub fn set_selection_impl(&mut self, id: Uuid, sel: Selection) {
        let state = self.selections.entry(id).or_default();
        state.set_selection(Some(sel));
    }

    pub fn clear_selection_impl(&mut self, id: Uuid) {
        if let Some(state) = self.selections.get_mut(&id) {
            state.clear_selection();
        }
    }

    pub fn get_anchor_mode(&self, id: Uuid) -> bool {
        self.selections
            .get(&id)
            .map(|s| s.is_anchor_mode())
            .unwrap_or(false)
    }

    pub fn set_anchor_mode_impl(&mut self, id: Uuid, mode: bool) {
        let state = self.selections.entry(id).or_default();
        state.set_anchor_mode(mode);
    }

    pub fn start_selection_impl(&mut self, id: Uuid, pos: Position) {
        let state = self.selections.entry(id).or_default();
        state.start_selection(pos);
        self.cursors.insert(id, Cursor::new(pos));
    }

    pub fn update_selection_impl(&mut self, id: Uuid, head: Position) {
        let state = self.selections.entry(id).or_default();
        state.update_selection(head);
        self.cursors.insert(id, Cursor::new(head));
    }

    pub fn move_cursor_impl(&mut self, id: Uuid, pos: Position) {
        self.cursors.insert(id, Cursor::new(pos));
    }

    pub fn check_has_selection(&self, id: Uuid) -> bool {
        self.selections
            .get(&id)
            .map(|s| s.has_selection())
            .unwrap_or(false)
    }

    pub fn get_selected_range(&self, id: Uuid) -> Option<(Position, Position)> {
        self.selections.get(&id).and_then(|s| s.selected_range())
    }

    pub fn remove_editor_impl(&mut self, id: Uuid) {
        self.cursors.remove(&id);
        self.selections.remove(&id);
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
