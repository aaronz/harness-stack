use crate::core::editor::{CommandResult, EditorCommand, EditorState};
use crate::core::selection::Selection;

pub struct EditorService;

impl EditorService {
    pub fn apply_command(state: &mut EditorState, cmd: EditorCommand) -> CommandResult {
        state.apply_command(cmd)
    }

    pub fn get_selection(state: &EditorState) -> Selection {
        state.selection().clone()
    }

    pub fn set_selection(state: &mut EditorState, selection: Selection) {
        state.set_selection(selection);
    }

    pub fn undo(state: &mut EditorState) -> Option<CommandResult> {
        state.undo().ok()?;
        Some(CommandResult {
            content: state.content().to_string(),
            selection: state.selection().clone(),
        })
    }

    pub fn redo(state: &mut EditorState) -> Option<CommandResult> {
        state.redo().ok()?;
        Some(CommandResult {
            content: state.content().to_string(),
            selection: state.selection().clone(),
        })
    }

    pub fn load_content(state: &mut EditorState, content: String) {
        state.load_content(content);
    }

    pub fn mark_saved(state: &mut EditorState) {
        state.mark_saved();
    }
}
