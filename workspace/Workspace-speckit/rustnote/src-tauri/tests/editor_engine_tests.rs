use rustnote_lib::editor::cursor::{
    cursor_at_end, cursor_at_offset, cursor_at_start, move_cursor, Cursor,
};
use rustnote_lib::editor::selection::{select_line_at, select_word_at, SelectionState};
use rustnote_lib::editor::transforms::{Transform, TransformEngine};
use rustnote_lib::semantic::ast::Position;

fn engine() -> TransformEngine {
    TransformEngine::new()
}

#[test]
fn test_cursor_movement_forward() {
    let text = "Hello world";
    let mut cursor = cursor_at_offset(0);
    move_cursor(text, &mut cursor, 5);
    assert_eq!(cursor.position().offset, 5);
}

#[test]
fn test_cursor_movement_backward() {
    let text = "Hello world";
    let mut cursor = cursor_at_offset(5);
    move_cursor(text, &mut cursor, -3);
    assert_eq!(cursor.position().offset, 2);
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
fn test_cursor_move_forward_boundary() {
    let text = "Hi";
    let mut cursor = cursor_at_offset(1);
    move_cursor(text, &mut cursor, 10);
    assert_eq!(cursor.position().offset, 2);
}

#[test]
fn test_cursor_move_forward_within_bounds() {
    let text = "Hello";
    let mut cursor = cursor_at_offset(0);
    move_cursor(text, &mut cursor, 3);
    assert_eq!(cursor.position().offset, 3);
}

#[test]
fn test_cursor_move_backward_stops_at_start() {
    let text = "Hi";
    let mut cursor = cursor_at_offset(1);
    move_cursor(text, &mut cursor, -1);
    assert_eq!(cursor.position().offset, 0);
}

#[test]
fn test_selection_word_at_start() {
    let text = "Hello world";
    let (start, end) = select_word_at(text, 0);
    assert_eq!(start, 0);
    assert_eq!(end, 5);
}

#[test]
fn test_selection_word_at_middle() {
    let text = "Hello world";
    let (start, end) = select_word_at(text, 3);
    assert_eq!(start, 0);
    assert_eq!(end, 5);
}

#[test]
fn test_selection_word_at_space() {
    let text = "Hello world";
    let (start, end) = select_word_at(text, 5);
    assert_eq!(start, 0);
    assert_eq!(end, 5);
}

#[test]
fn test_selection_word_at_second_word() {
    let text = "Hello world";
    let (start, end) = select_word_at(text, 7);
    assert_eq!(start, 6);
    assert_eq!(end, 11);
}

#[test]
fn test_selection_line_single() {
    let text = "Line 1\nLine 2\nLine 3";
    let (start, end) = select_line_at(text, 7);
    assert_eq!(start, 7);
    assert_eq!(end, 13);
}

#[test]
fn test_selection_line_at_newline() {
    let text = "Line 1\nLine 2";
    let (start, end) = select_line_at(text, 6);
    assert_eq!(start, 7);
    assert_eq!(end, 13);
}

#[test]
fn test_selection_state_new() {
    let state = SelectionState::new();
    assert!(state.selection().is_none());
    assert!(!state.has_selection());
}

#[test]
fn test_selection_state_word() {
    let text = "Hello world";
    let mut state = SelectionState::new();
    let (start, end) = select_word_at(text, 3);
    let pos_start = Position::new(start, 0, start as u32);
    let pos_end = Position::new(end, 0, end as u32);
    state.set_selection(Some(rustnote_lib::semantic::position::Selection::new(
        pos_start, pos_end,
    )));
    assert!(state.has_selection());
    let (s, e) = state.selected_range().unwrap();
    assert_eq!(s.offset, 0);
    assert_eq!(e.offset, 5);
}

#[test]
fn test_ime_composition_basic() {
    let text = "ABC";
    let mut cursor = cursor_at_start();
    move_cursor(text, &mut cursor, 3);
    assert_eq!(cursor.position().offset, 3);
}

#[test]
fn test_ime_composition_with_composing() {
    let text = "hello\u{0300}";
    let mut cursor = cursor_at_offset(5);
    move_cursor(text, &mut cursor, 1);
    assert_eq!(cursor.position().offset, 6);
}

#[test]
fn test_double_click_selection_word() {
    let text = "Hello world";
    let (start, end) = select_word_at(text, 3);
    assert_eq!(start, 0);
    assert_eq!(end, 5);
}

#[test]
fn test_triple_click_selection_line() {
    let text = "Line 1\nLine 2\nLine 3";
    let (start, end) = select_line_at(text, 0);
    assert_eq!(start, 0);
    assert_eq!(end, 6);
}

#[test]
fn test_shift_arrow_selection() {
    let text = "Hello world";
    let mut state = SelectionState::new();
    let start_pos = Position::new(0, 0, 0);
    state.start_selection(start_pos);
    let end_pos = Position::new(5, 0, 5);
    state.update_selection(end_pos);
    assert!(state.has_selection());
}

#[test]
fn test_transform_enter_in_empty_unordered_list_item() {
    let content = "- ";
    let result = engine().apply(&Transform::Enter, content, 2);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_transform_enter_in_unordered_list_item_with_content() {
    let content = "- item";
    let result = engine().apply(&Transform::Enter, content, 6);
    assert!(result.content.contains("- item"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_transform_enter_in_ordered_list_item() {
    let content = "1. item";
    let result = engine().apply(&Transform::Enter, content, 7);
    assert!(result.content.contains("2. "));
}

#[test]
fn test_transform_enter_in_task_list() {
    let content = "- [ ] task";
    let result = engine().apply(&Transform::Enter, content, 10);
    assert!(result.content.contains("- [ ] task"));
}

#[test]
fn test_transform_enter_in_blockquote() {
    let content = "> quote";
    let result = engine().apply(&Transform::Enter, content, 7);
    assert!(result.content.contains("> quote"));
    assert!(result.content.contains("> "));
}

#[test]
fn test_transform_enter_in_heading() {
    let content = "# Heading";
    let result = engine().apply(&Transform::Enter, content, 9);
    assert!(result.content.contains("# Heading"));
    assert!(result.content.contains("# "));
}

#[test]
fn test_transform_backspace_at_line_start_joins_lines() {
    let content = "line1\nline2";
    let result = engine().apply(&Transform::Backspace, content, 6);
    assert_eq!(result.content, "line1line2");
    assert_eq!(result.cursor_offset, 5);
}

#[test]
fn test_transform_backspace_in_middle() {
    let content = "Hello world";
    let result = engine().apply(&Transform::Backspace, content, 5);
    assert_eq!(result.content, "Hell world");
    assert_eq!(result.cursor_offset, 4);
}

#[test]
fn test_transform_tab_indents_list_item() {
    let content = "- item";
    let result = engine().apply(&Transform::Tab, content, 2);
    assert!(result.content.starts_with("    -"));
}

#[test]
fn test_transform_shift_tab_dedents_list_item() {
    let content = "    - item";
    let result = engine().apply(&Transform::ShiftTab, content, 6);
    assert!(result.content.starts_with("- item"));
}

#[test]
fn test_transform_tab_in_paragraph_inserts_tab() {
    let content = "Paragraph";
    let result = engine().apply(&Transform::Tab, content, 4);
    assert!(result.content.contains('\t'));
}

#[test]
fn test_transform_enter_in_empty_ordered_list_item() {
    let content = "1. ";
    let result = engine().apply(&Transform::Enter, content, 3);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_transform_enter_creates_newline_in_paragraph() {
    let content = "Hello world";
    let result = engine().apply(&Transform::Enter, content, 5);
    assert!(result.content.contains("Hello"));
    assert!(result.content.contains("world"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_transform_enter_in_multiple_paragraphs_in_list_item() {
    let content = "- item\n  with multiple\n  paragraphs";
    let result = engine().apply(&Transform::Enter, content, 10);
    assert!(result.content.contains("- item"));
}

#[test]
fn test_transform_backspace_joins_list_items() {
    let content = "- item1\n- item2";
    let result = engine().apply(&Transform::Backspace, content, 10);
    assert!(result.content.contains("- item1"));
}

#[test]
fn test_transform_enter_in_task_list_with_content() {
    let content = "- [x] done";
    let result = engine().apply(&Transform::Enter, content, 10);
    assert!(result.content.contains("- [x] done"));
    assert!(result.content.contains("\n"));
}

#[test]
fn test_transform_enter_in_nested_list() {
    let content = "- item\n  - nested";
    let result = engine().apply(&Transform::Enter, content, 12);
    assert!(result.content.contains("- item"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_cursor_line_boundary_start() {
    let text = "Line 1\nLine 2";
    let mut cursor = cursor_at_offset(7);
    cursor.move_to_line_start();
    assert_eq!(cursor.position().column, 0);
}

#[test]
fn test_cursor_line_boundary_end() {
    let text = "Line 1\nLine 2";
    let mut cursor = cursor_at_offset(0);
    cursor.move_to_line_end(6);
    assert_eq!(cursor.position().column, 6);
}

#[test]
fn test_cursor_next_line() {
    let text = "Line 1\nLine 2";
    let mut cursor = cursor_at_offset(6);
    cursor.move_to_next_line(6);
    assert_eq!(cursor.position().line, 1);
    assert_eq!(cursor.position().column, 0);
}

#[test]
fn test_cursor_prev_line() {
    let mut cursor = Cursor::new(Position::new(12, 1, 6));
    cursor.move_to_prev_line(6);
    assert_eq!(cursor.position().line, 0);
    assert_eq!(cursor.position().column, 6);
}
