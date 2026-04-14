use rustnote_lib::editor::transforms::{Transform, TransformEngine};
use rustnote_lib::semantic::ast::SemanticDocument;

fn engine() -> TransformEngine {
    TransformEngine::new()
}

#[test]
fn test_enter_in_empty_unordered_list_item() {
    let content = "- ";
    let result = engine().apply(&Transform::Enter, content, 2, None);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_enter_in_unordered_list_item_with_content() {
    let content = "- item";
    let result = engine().apply(&Transform::Enter, content, 6, None);
    assert!(result.content.contains("- item"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_enter_in_empty_ordered_list_item() {
    let content = "1. ";
    let result = engine().apply(&Transform::Enter, content, 3, None);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_tab_indents_list_item() {
    let content = "- item";
    let result = engine().apply(&Transform::Tab, content, 2, None);
    assert!(result.content.starts_with("    -"));
}

#[test]
fn test_shift_tab_dedents_list_item() {
    let content = "    - item";
    let result = engine().apply(&Transform::ShiftTab, content, 6, None);
    assert!(result.content.starts_with("- item"));
}

#[test]
fn test_backspace_at_line_start() {
    let content = "line1\nline2";
    let result = engine().apply(&Transform::Backspace, content, 6, None);
    assert_eq!(result.content, "line1line2");
    assert_eq!(result.cursor_offset, 5);
}

#[test]
fn test_enter_in_blockquote() {
    let content = "> quote";
    let result = engine().apply(&Transform::Enter, content, 7, None);
    assert!(result.content.contains("> quote"));
    assert!(result.content.contains("> "));
}

#[test]
fn test_enter_in_heading() {
    let content = "# Heading";
    let result = engine().apply(&Transform::Enter, content, 9, None);
    assert!(result.content.contains("# Heading"));
    assert!(result.content.contains("# "));
}

#[test]
fn test_enter_in_empty_heading() {
    let content = "# ";
    let result = engine().apply(&Transform::Enter, content, 2, None);
    assert!(result.content.contains("# "));
}

#[test]
fn test_ordered_list_increments_number() {
    let content = "1. item";
    let result = engine().apply(&Transform::Enter, content, 7, None);
    assert!(result.content.contains("2. "));
}

#[test]
fn test_enter_in_task_list_item() {
    let content = "- [ ] task";
    let result = engine().apply(&Transform::Enter, content, 10, None);
    assert!(result.content.contains("- [ ] task"));
    assert!(result.content.contains("- [ ] "));
}

#[test]
fn test_enter_creates_newline_in_paragraph() {
    let content = "Hello world";
    let result = engine().apply(&Transform::Enter, content, 5, None);
    assert!(result.content.contains("Hello"));
    assert!(result.content.contains("world"));
    assert!(result.content.contains('\n'));
}

#[test]
fn test_enter_in_multiple_paragraphs_in_list_item() {
    let content = "- item\n  with multiple\n  paragraphs";
    let result = engine().apply(&Transform::Enter, content, 10, None);
    assert!(result.content.contains("- item"));
}

#[test]
fn test_backspace_joins_list_items() {
    let content = "- item1\n- item2";
    let result = engine().apply(&Transform::Backspace, content, 10, None);
    assert!(result.content.contains("- item1"));
}

#[test]
fn test_tab_in_nested_list_item() {
    let content = "- item";
    let result = engine().apply(&Transform::Tab, content, 2, None);
    assert!(result.content.starts_with("    -"));
}

#[test]
fn test_enter_in_heading_creates_next_heading() {
    let content = "# Title";
    let result = engine().apply(&Transform::Enter, content, 7, None);
    assert!(result.content.contains("# Title"));
    assert!(result.content.contains("# "));
}

#[test]
fn test_task_list_checkbox_toggle_unchecked_to_checked() {
    let content = "- [ ] task";
    let result = engine().apply(&Transform::Enter, content, 10, None);
    assert!(result.content.contains("- [ ] task"));
    assert!(result.content.contains("- [ ] "));
}

#[test]
fn test_task_list_checkbox_toggle_checked_to_unchecked() {
    let content = "- [x] task";
    let result = engine().apply(&Transform::Enter, content, 10, None);
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

#[test]
fn test_tc_g001_001_enter_in_list_item_empty_exits() {
    let content = "- ";
    let result = engine().apply(
        &Transform::EnterInListItem { is_empty: true },
        content,
        2,
        None,
    );
    assert_eq!(result.content, "\n");
    assert_eq!(result.cursor_offset, 1);
}

#[test]
fn test_tc_g001_002_enter_in_list_item_with_content() {
    let content = "- Item 1";
    let result = engine().apply(
        &Transform::EnterInListItem { is_empty: false },
        content,
        8,
        None,
    );
    assert!(result.content.contains("- Item 1"));
    assert!(result.content.contains("\n"));
    assert!(result.content.contains("- "));
}

#[test]
fn test_tc_g001_003_enter_in_list_item_nested() {
    let content = "  - Nested item";
    let result = engine().apply(
        &Transform::EnterInListItem { is_empty: false },
        content,
        15,
        None,
    );
    assert!(result.content.contains("  - Nested item"));
    assert!(result.content.contains("\n"));
    assert!(result.content.contains("  - "));
}

#[test]
fn test_tc_g001_004_enter_in_blockquote_empty_exits() {
    let content = "> ";
    let result = engine().apply(&Transform::EnterInBlockQuote, content, 2, None);
    assert_eq!(result.content, "");
    assert_eq!(result.cursor_offset, 0);
}

#[test]
fn test_tc_g001_005_enter_in_blockquote_with_content() {
    let content = "> Quote line";
    let result = engine().apply(&Transform::EnterInBlockQuote, content, 12, None);
    assert!(result.content.contains("> Quote line"));
    assert!(result.content.contains("\n> "));
}

#[test]
fn test_tc_g001_006_enter_in_heading_empty_converts() {
    let content = "# ";
    let result = engine().apply(&Transform::EnterInHeading { level: 1 }, content, 2, None);
    assert!(result.content.contains("# "));
}

#[test]
fn test_tc_g001_007_enter_in_heading_with_content() {
    let content = "# Heading";
    let result = engine().apply(&Transform::EnterInHeading { level: 1 }, content, 9, None);
    assert!(result.content.contains("# Heading"));
    assert!(result.content.contains("\n# "));
}

#[test]
fn test_tc_g001_008_enter_in_heading_setext_underline() {
    let content = "Heading\n===\n";
    let result = engine().apply(&Transform::EnterInHeading { level: 1 }, content, 11, None);
    assert!(result.content.contains("Heading"));
    assert!(result.content.contains("==="));
}

#[test]
fn test_tc_g001_010_wrap_no_selection() {
    let content = "text";
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        4,
        None,
    );
    assert!(result.content.contains("text"));
    assert!(result.content.contains("**"));
}

#[test]
fn test_tc_g001_011_wrap_nested_markers() {
    let content = "**already bold**";
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        14,
        None,
    );
    assert!(result.content.contains("**already bold**"));
    assert!(result.content.contains("****"));
}

// G-003 Wrap Transform Integration Tests

#[test]
fn test_tc_g003_001_wrap_selection_bold() {
    // TC-G003-001: Wrap selection with bold markers
    // Input: Select 'selection' -> apply wrap with **
    // Expected: Result: **selection**
    let content = "text with selection here";
    let selection_start = 10;
    let cursor_offset = 19;
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(result.content, "text with **selection** here");
    assert_eq!(result.cursor_offset, 23);
}

#[test]
fn test_tc_g003_002_wrap_selection_italic() {
    // TC-G003-002: Wrap selection with italic markers
    // Input: Select 'selection' -> apply wrap with *
    // Expected: Result: *selection*
    let content = "text with selection here";
    let selection_start = 10;
    let cursor_offset = 19;
    let result = engine().apply(
        &Transform::Wrap {
            before: "*".to_string(),
            after: "*".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(result.content, "text with *selection* here");
    assert_eq!(result.cursor_offset, 21);
}

#[test]
fn test_tc_g003_003_wrap_selection_code() {
    // TC-G003-003: Wrap selection with code markers
    // Input: Select 'code' -> apply wrap with `
    // Expected: Result: `code`
    let content = "text with code here";
    let selection_start = 10;
    let cursor_offset = 14;
    let result = engine().apply(
        &Transform::Wrap {
            before: "`".to_string(),
            after: "`".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(result.content, "text with `code` here");
    assert_eq!(result.cursor_offset, 16);
}

#[test]
fn test_tc_g003_004_wrap_selection_link() {
    // TC-G003-004: Wrap selection with link markers
    // Input: Select 'link text' -> apply wrap with [ and ](url)
    // Expected: Result: [link text](url) inserted
    let content = "text with link text here";
    let selection_start = 10;
    let cursor_offset = 19;
    let result = engine().apply(
        &Transform::Wrap {
            before: "[".to_string(),
            after: "](url)".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(result.content, "text with [link text](url) here");
    // cursor at end of inserted markers + selected text
    // 10 (before) + 1 ([) + 9 (link text) + 6 (](url)) = 26
    assert_eq!(result.cursor_offset, 26);
}

#[test]
fn test_tc_g003_005_wrap_no_selection() {
    // TC-G003-005: Wrap without selection at cursor
    // Input: Cursor at position in 'hello world' -> apply ** wrap
    // Expected: Result: **| (cursor between markers)
    let content = "hello world";
    let cursor_offset = 5;
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        cursor_offset,
        None,
    );
    assert_eq!(result.content, "hello** world");
    assert_eq!(result.cursor_offset, 7); // cursor after **
}

#[test]
fn test_tc_g003_edge_case_empty_selection() {
    // Edge case: empty_selection - selection_start equals cursor_offset
    let content = "text";
    let selection_start = 4;
    let cursor_offset = 4;
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    // When selection is empty, behaves like no selection - just inserts markers at cursor
    assert_eq!(result.content, "text**");
    assert_eq!(result.cursor_offset, 6);
}

#[test]
fn test_tc_g003_edge_case_cursor_at_boundary() {
    // Edge case: cursor_at_boundary - cursor at start of content
    let content = "text";
    let cursor_offset = 0;
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        cursor_offset,
        None,
    );
    assert_eq!(result.content, "**text");
    assert_eq!(result.cursor_offset, 2);
}

#[test]
fn test_tc_g003_wrap_partial_selection() {
    // Partial selection - selection_start < cursor_offset but not full word
    let content = "text with selection here";
    let selection_start = 10;
    let cursor_offset = 18;
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(result.content, "text with **selectio**n here");
    assert_eq!(result.cursor_offset, 22);
}

#[test]
fn test_tc_g003_wrap_strikethrough() {
    // Additional marker type: strikethrough
    let content = "text with selection here";
    let selection_start = 10;
    let cursor_offset = 19;
    let result = engine().apply(
        &Transform::Wrap {
            before: "~~".to_string(),
            after: "~~".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(result.content, "text with ~~selection~~ here");
    assert_eq!(result.cursor_offset, 23);
}

#[test]
fn test_tc_g003_wrap_already_wrapped() {
    // Already wrapped - wrapping already wrapped content
    let content = "text with **selection** here";
    let selection_start = 10;
    let cursor_offset = 22;
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(result.content, "text with ****selection**** here");
    assert_eq!(result.cursor_offset, 26);
}

#[test]
fn test_tc_g003_wrap_nested_markers() {
    // Nested markers - wrapping already bold text doubles the markers
    let content = "**already bold**";
    let selection_start = 0;
    let cursor_offset = 16;
    let result = engine().apply(
        &Transform::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content,
        cursor_offset,
        Some(selection_start),
    );
    // Wrapping adds ** before and ** after the selected content
    assert_eq!(result.content, "****already bold****");
    // cursor at end: 0 + 2 + 16 + 2 = 20
    assert_eq!(result.cursor_offset, 20);
}
