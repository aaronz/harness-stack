use crate::core::buffer::Buffer;
use crate::core::selection::{Cursor, Selection};
use crate::error::Result;

pub enum EditorCommand {
    InsertText { text: String },
    DeleteBackward,
    DeleteForward,
    InsertNewline,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
}

pub struct CommandResult {
    pub content: String,
    pub selection: Selection,
}

pub struct EditorState {
    buffer: Buffer,
    selection: Selection,
    undo_stack: Vec<Buffer>,
    redo_stack: Vec<Buffer>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            selection: Selection::default(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn with_content(content: String) -> Self {
        Self {
            buffer: Buffer::from_str(&content),
            selection: Selection::caret(Cursor::new(content.len())),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn content(&self) -> &str {
        self.buffer.text().to_string().leak()
    }

    pub fn selection(&self) -> &Selection {
        &self.selection
    }

    pub fn set_selection(&mut self, selection: Selection) {
        self.selection = selection;
    }

    pub fn is_dirty(&self) -> bool {
        self.buffer.is_dirty()
    }

    pub fn save_snapshot(&mut self) {
        self.undo_stack
            .push(Buffer::from_str(&self.buffer.to_string()));
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Result<()> {
        if let Some(snapshot) = self.undo_stack.pop() {
            self.redo_stack
                .push(Buffer::from_str(&self.buffer.to_string()));
            self.buffer = snapshot;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn redo(&mut self) -> Result<()> {
        if let Some(snapshot) = self.redo_stack.pop() {
            self.undo_stack
                .push(Buffer::from_str(&self.buffer.to_string()));
            self.buffer = snapshot;
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn apply_command(&mut self, cmd: EditorCommand) -> CommandResult {
        match cmd {
            EditorCommand::InsertText { text } => {
                self.save_snapshot();
                let start = self.selection.start().offset;
                let end = self.selection.end().offset;
                if start != end {
                    self.buffer.delete(start, end);
                }
                let cursor = self.buffer.insert(Cursor::new(start), &text);
                self.selection = Selection::caret(cursor);
                self.redo_stack.clear();
            }
            EditorCommand::DeleteBackward => {
                if !self.selection.is_empty() {
                    self.save_snapshot();
                    let start = self.selection.start().offset;
                    let end = self.selection.end().offset;
                    let cursor = self.buffer.delete(start, end);
                    self.selection = Selection::caret(cursor);
                } else if self.selection.start().offset > 0 {
                    self.save_snapshot();
                    let offset = self.selection.start().offset;
                    let cursor = self.buffer.delete(offset - 1, offset);
                    self.selection = Selection::caret(cursor);
                }
                self.redo_stack.clear();
            }
            EditorCommand::DeleteForward => {
                if !self.selection.is_empty() {
                    self.save_snapshot();
                    let start = self.selection.start().offset;
                    let end = self.selection.end().offset;
                    let cursor = self.buffer.delete(start, end);
                    self.selection = Selection::caret(cursor);
                } else if self.selection.start().offset < self.buffer.len() {
                    self.save_snapshot();
                    let offset = self.selection.start().offset;
                    let cursor = self.buffer.delete(offset, offset + 1);
                    self.selection = Selection::caret(cursor);
                }
                self.redo_stack.clear();
            }
            EditorCommand::InsertNewline => {
                self.save_snapshot();
                let offset = self.selection.start().offset;
                let cursor = self.buffer.insert(Cursor::new(offset), "\n");
                self.selection = Selection::caret(cursor);
                self.redo_stack.clear();
            }
            EditorCommand::MoveCursorLeft => {
                let mut cursor = self.selection.start();
                cursor.move_left();
                self.selection = Selection::caret(cursor);
            }
            EditorCommand::MoveCursorRight => {
                let mut cursor = self.selection.end();
                cursor.move_right(self.buffer.len());
                self.selection = Selection::caret(cursor);
            }
            EditorCommand::MoveCursorUp | EditorCommand::MoveCursorDown => {}
        }
        CommandResult {
            content: self.buffer.to_string(),
            selection: self.selection.clone(),
        }
    }

    pub fn load_content(&mut self, content: String) {
        self.buffer = Buffer::from_str(&content);
        self.selection = Selection::caret(Cursor::new(content.len()));
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    pub fn mark_saved(&mut self) {
        self.buffer.mark_saved();
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_text() {
        let mut editor = EditorState::new();
        let result = editor.apply_command(EditorCommand::InsertText {
            text: "Hello".to_string(),
        });
        assert_eq!(result.content, "Hello");
        assert_eq!(result.selection.start().offset, 5);
    }

    #[test]
    fn test_delete_backward() {
        let mut editor = EditorState::with_content("Hello".to_string());
        let result = editor.apply_command(EditorCommand::DeleteBackward);
        assert_eq!(result.content, "Hell");
    }

    #[test]
    fn test_undo_redo() {
        let mut editor = EditorState::new();
        editor.apply_command(EditorCommand::InsertText {
            text: "Hello".to_string(),
        });
        editor.undo().unwrap();
        assert_eq!(editor.content(), "");
        editor.redo().unwrap();
        assert_eq!(editor.content(), "Hello");
    }
}
