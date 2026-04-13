use rustnote_lib::editor::cursor::{cursor_at_offset, cursor_at_start};
use rustnote_lib::editor::selection::{select_line_at, select_word_at, SelectionState};
use rustnote_lib::editor::transforms::{Transform, TransformEngine};
use rustnote_lib::semantic::ast::{Position, SemanticDocument};

fn engine() -> TransformEngine {
    TransformEngine::new()
}

#[test]
fn test_full_document_parse_and_transform() {
    let source = "# Title\n\n## Section\n\nSome **bold** and *italic* text.\n\n- Item 1\n- Item 2";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_cursor_and_transform_integration() {
    let content = "# Heading\n\nParagraph";
    let mut cursor = cursor_at_start();
    cursor.move_forward(10);

    let result = engine().apply(&Transform::Enter, content, cursor.position().offset, None);
    assert!(result.content.contains('\n'));
}

#[test]
fn test_selection_and_transform_integration() {
    let text = "Hello world";
    let (start, end) = select_word_at(text, 3);
    let pos_start = Position::new(start, 0, start as u32);
    let pos_end = Position::new(end, 0, end as u32);

    let mut state = SelectionState::new();
    state.set_selection(Some(rustnote_lib::semantic::position::Selection::new(
        pos_start, pos_end,
    )));

    assert!(state.has_selection());
}

#[test]
fn test_parse_heading_then_transform() {
    let source = "# My Heading";
    let doc = SemanticDocument::parse(source);
    assert!(doc.get_headings().len() == 1);

    let result = engine().apply(&Transform::Enter, source, 11, None);
    assert!(result.content.contains('#'));
}

#[test]
fn test_parse_list_then_transform() {
    let source = "- [x] Completed task";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("- [x]"));

    let result = engine().apply(&Transform::Enter, source, source.len(), None);
    assert!(result.content.contains("- [x] Completed task"));
}

#[test]
fn test_multiple_transforms_sequence() {
    let content = "- item";
    let mut current = content.to_string();
    let mut offset = content.len();

    let result1 = engine().apply(&Transform::Enter, &current, offset, None);
    current = result1.content;
    offset = result1.cursor_offset;

    let result2 = engine().apply(&Transform::Tab, &current, offset, None);
    current = result2.content;

    assert!(current.contains('\n'));
}

#[test]
fn test_cursor_positioning_after_transform() {
    let content = "# Heading";
    let result = engine().apply(&Transform::Enter, content, 9, None);

    let doc = SemanticDocument::parse(&result.content);
    let pos = doc.offset_to_position(result.cursor_offset);
    assert!(pos.line >= 0);
}

#[test]
fn test_transform_in_blockquote_context() {
    let content = "> quote line\n> second line";
    let result = engine().apply(&Transform::Enter, content, 10, None);
    assert!(result.content.contains("> quote"));
}

#[test]
fn test_transform_in_nested_list() {
    let content = "- item\n  - nested item";
    let result = engine().apply(&Transform::Enter, content, 14, None);
    assert!(result.content.contains('-'));
}

#[test]
fn test_parse_and_serialize_complex_document() {
    let source = r#"# Document Title

## Section 1

This is a paragraph with **bold** and *italic*.

### Subsection 1.1

- List item 1
- List item 2

> A blockquote

## Section 2

| Column 1 | Column 2 |
|----------|----------|
| Data 1   | Data 2   |

```rust
fn main() {
    println!("Hello");
}
```

- [x] Completed task
- [ ] Pending task
"#;
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_selection_across_multiple_lines() {
    let text = "Line 1\nLine 2\nLine 3";
    let (start, end) = select_line_at(text, 7);
    assert_eq!(start, 7);
    assert_eq!(end, 13);
}

#[test]
fn test_cursor_to_position_conversion() {
    let source = "Line 1\nLine 2";
    let doc = SemanticDocument::parse(source);

    let pos = doc.offset_to_position(7);
    assert_eq!(pos.line, 1);

    let offset = doc.position_to_offset(Position::new(7, 1, 0));
    assert_eq!(offset, 7);
}

#[test]
fn test_transform_state_independence() {
    let content = "- item";

    let result1 = engine().apply(&Transform::Tab, content, 2, None);
    let result2 = engine().apply(&Transform::Tab, content, 2, None);

    assert_eq!(result1.content, result2.content);
}

#[test]
fn test_heading_level_detection() {
    let cases = vec![
        ("# H1", 1),
        ("## H2", 2),
        ("### H3", 3),
        ("#### H4", 4),
        ("##### H5", 5),
        ("###### H6", 6),
    ];

    for (source, expected_level) in cases {
        let doc = SemanticDocument::parse(source);
        let headings = doc.get_headings();
        assert_eq!(headings[0].level, expected_level);
    }
}

#[test]
fn test_empty_document_edge_cases() {
    let doc = SemanticDocument::parse("");
    assert!(doc.serialize_to_commonmark().is_empty());
    assert!(doc.get_headings().is_empty());
}

#[test]
fn test_whitespace_only_document() {
    let source = "   \n\n   \n";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_frontmatter_roundtrip() {
    let source = "---\ntitle: Test\nauthor: Author\n---\n\n# Content";
    let doc = SemanticDocument::parse(source);
    assert!(doc.has_frontmatter());

    let output = doc.serialize_with_frontmatter();
    assert!(output.starts_with("---\ntitle: Test"));
}

#[test]
fn test_transform_backspace_joins_correctly() {
    let content = "line1\nline2";
    let result = engine().apply(&Transform::Backspace, content, 6, None);
    assert_eq!(result.content, "line1line2");
}

#[test]
fn test_transform_enter_in_empty_list_exits() {
    let content = "- ";
    let result = engine().apply(&Transform::Enter, content, 2, None);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_transform_enter_in_empty_ordered_list_exits() {
    let content = "1. ";
    let result = engine().apply(&Transform::Enter, content, 3, None);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_shift_tab_dedent_when_not_indented() {
    let content = "not indented";
    let result = engine().apply(&Transform::ShiftTab, content, 5, None);
    assert_eq!(result.content, content);
}

#[test]
fn test_ordered_list_number_increment() {
    let content = "1. item";
    let result = engine().apply(&Transform::Enter, content, 7, None);
    assert!(result.content.contains("2. "));
}

#[test]
fn test_task_list_continuation() {
    let content = "- [ ] todo";
    let result = engine().apply(&Transform::Enter, content, content.len(), None);
    assert!(result.content.contains("- [ ] todo"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_code_block_language_preservation() {
    let source = "```javascript\nconsole.log('test');\n```";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("```javascript"));
}

#[test]
fn test_table_structure_preservation() {
    let source = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("| A |"));
}

#[test]
fn test_horizontal_rule_variations() {
    let cases = vec![
        "---\n\npara\n\n---",
        "***\n\npara\n\n***",
        "___\n\npara\n\n___",
    ];
    for source in cases {
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert_eq!(output, source);
    }
}
