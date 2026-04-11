use rustnote_lib::editor::transforms::{Transform, TransformEngine};
use rustnote_lib::semantic::ast::SemanticDocument;

fn engine() -> TransformEngine {
    TransformEngine::new()
}

#[test]
fn test_enter_in_empty_unordered_list_item() {
    let content = "- ";
    let result = engine().apply(&Transform::Enter, content, 2);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_enter_in_unordered_list_item_with_content() {
    let content = "- item";
    let result = engine().apply(&Transform::Enter, content, 6);
    assert!(result.content.contains("- item"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_enter_in_empty_ordered_list_item() {
    let content = "1. ";
    let result = engine().apply(&Transform::Enter, content, 3);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_tab_indents_list_item() {
    let content = "- item";
    let result = engine().apply(&Transform::Tab, content, 2);
    assert!(result.content.starts_with("    -"));
}

#[test]
fn test_shift_tab_dedents_list_item() {
    let content = "    - item";
    let result = engine().apply(&Transform::ShiftTab, content, 6);
    assert!(result.content.starts_with("- item"));
}

#[test]
fn test_backspace_at_line_start() {
    let content = "line1\nline2";
    let result = engine().apply(&Transform::Backspace, content, 6);
    assert_eq!(result.content, "line1line2");
    assert_eq!(result.cursor_offset, 5);
}

#[test]
fn test_enter_in_blockquote() {
    let content = "> quote";
    let result = engine().apply(&Transform::Enter, content, 7);
    assert!(result.content.contains("> quote"));
    assert!(result.content.contains("> "));
}

#[test]
fn test_enter_in_heading() {
    let content = "# Heading";
    let result = engine().apply(&Transform::Enter, content, 9);
    assert!(result.content.contains("# Heading"));
    assert!(result.content.contains("# "));
}

#[test]
fn test_enter_in_empty_heading() {
    let content = "# ";
    let result = engine().apply(&Transform::Enter, content, 2);
    assert!(result.content.contains("# "));
}

#[test]
fn test_ordered_list_increments_number() {
    let content = "1. item";
    let result = engine().apply(&Transform::Enter, content, 7);
    assert!(result.content.contains("2. "));
}

#[test]
fn test_enter_in_task_list_item() {
    let content = "- [ ] task";
    let result = engine().apply(&Transform::Enter, content, 10);
    assert!(result.content.contains("- [ ] task"));
    assert!(result.content.contains("- [ ] "));
}

#[test]
fn test_enter_creates_newline_in_paragraph() {
    let content = "Hello world";
    let result = engine().apply(&Transform::Enter, content, 5);
    assert!(result.content.contains("Hello"));
    assert!(result.content.contains("world"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_enter_in_multiple_paragraphs_in_list_item() {
    let content = "- item\n  with multiple\n  paragraphs";
    let result = engine().apply(&Transform::Enter, content, 10);
    assert!(result.content.contains("- item"));
}

#[test]
fn test_backspace_joins_list_items() {
    let content = "- item1\n- item2";
    let result = engine().apply(&Transform::Backspace, content, 10);
    assert!(result.content.contains("- item1"));
}

#[test]
fn test_tab_in_nested_list_item() {
    let content = "- item";
    let result = engine().apply(&Transform::Tab, content, 2);
    assert!(result.content.starts_with("    -"));
}

#[test]
fn test_enter_in_heading_creates_next_heading() {
    let content = "# Title";
    let result = engine().apply(&Transform::Enter, content, 7);
    assert!(result.content.contains("# Title"));
    assert!(result.content.contains("# "));
}

#[test]
fn test_task_list_checkbox_toggle_unchecked_to_checked() {
    let content = "- [ ] task";
    let result = engine().apply(&Transform::Enter, content, 10);
    assert!(result.content.contains("- [ ] task"));
    assert!(result.content.contains("- [ ] "));
}

#[test]
fn test_task_list_checkbox_toggle_checked_to_unchecked() {
    let content = "- [x] task";
    let result = engine().apply(&Transform::Enter, content, 10);
    assert!(result.content.contains("- [x] task"));
}

#[test]
fn test_roundtrip_commonmark_headings() {
    let source = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_emphasis() {
    let source = "**bold** *italic* ***bolditalic*** `code` ~~strike~~";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_links() {
    let source = "[link](http://example.com) ![alt](image.png)";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_blockquote() {
    let source = "> single line quote\n>\n> multi line\n> quote";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_unordered_list() {
    let source = "- item 1\n- item 2\n- item 3";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_ordered_list() {
    let source = "1. first\n2. second\n3. third";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_task_list() {
    let source = "- [ ] unchecked\n- [x] checked";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_code_block() {
    let source = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_table() {
    let source = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_commonmark_horizontal_rule() {
    let source = "---\n\nparagraph\n\n---\n\nanother paragraph";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_roundtrip_complex_document() {
    let source = "# Document Title\n\n## Section 1\n\nThis is a paragraph with **bold** and *italic* text.\n\n- List item 1\n- List item 2\n\n> A blockquote\n\n## Section 2\n\n| Column 1 | Column 2 |\n|----------|----------|\n| Data 1   | Data 2   |\n\n```python\nprint(\"Hello, World!\")\n```\n\n- [x] Completed task\n- [ ] Pending task\n\n---\n\nFinal paragraph.";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}
