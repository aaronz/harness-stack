//! Wrap Transform Integration Tests
//!
//! Tests for P1-005: Wrap Transform — TipTap Selection Integration
//!
//! These tests verify the end-to-end integration of Transform::Wrap
//! through the IPC command `editor_apply_transform`, covering:
//! - Bold, italic, strikethrough, inline code wrapping
//! - Multi-paragraph selection wrapping
//! - Double-bold (existing formatting) wrapping
//! - Unwrap (wrap with empty markers)
//! - Empty selection handling
//!
//! Run with: cargo test --test transform_tests

use rustnote_lib::commands::editor::{editor_apply_transform, TransformType};

// =============================================================================
// TC-P1-005-01: Wrap text with bold markers
// =============================================================================

#[test]
fn tc_p1_005_01_wrap_text_with_bold_markers() {
    // Expected: Content becomes '**hello** world' after bold wrap
    let content = "hello world";
    let selection_start = 0;
    let cursor_offset = 5;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "**hello** world");
    assert_eq!(result.cursor_offset, 9);
}

#[test]
fn tc_p1_005_01_wrap_text_with_bold_markers_middle() {
    // Variant: "hello world" select "world" -> **world**
    let content = "hello world";
    let selection_start = 6;
    let cursor_offset = 11;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "hello **world**");
    assert_eq!(result.cursor_offset, 15);
}

// =============================================================================
// TC-P1-005-02: Wrap selection spanning multiple paragraphs
// =============================================================================

#[test]
fn tc_p1_005_02_wrap_selection_spanning_multiple_paragraphs() {
    // Input: Editor content: 'para1\npara2', selection spans both paragraphs
    // Expected: Both paragraphs wrapped with markers
    let content = "para1\npara2";
    let selection_start = 0;
    let cursor_offset = 11;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "**para1\npara2**");
    assert_eq!(result.cursor_offset, 15);
}

#[test]
fn tc_p1_005_02_wrap_partial_multi_paragraph() {
    // Select "para1\npara2" at positions 6 to 17
    let content = "para0\npara1\npara2\npara3";
    let selection_start = 6;
    let cursor_offset = 17;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "para0\n**para1\npara2**\npara3");
    assert_eq!(result.cursor_offset, 21);
}

// =============================================================================
// TC-P1-005-03: Wrap with existing formatting — double-bold
// =============================================================================

#[test]
fn tc_p1_005_03_wrap_existing_bold_double_bold() {
    // Input: Editor content: '**already** bold'
    // Expected: Becomes '****already** bold**' or markers properly nested
    let content = "**already** bold";
    let selection_start = 0;
    let cursor_offset = 16;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "****already** bold**");
    assert_eq!(result.cursor_offset, 20);
}

#[test]
fn tc_p1_005_03_wrap_inner_content_of_existing_bold() {
    // Select just "already" (without outer **) and wrap with **
    let content = "**already** bold";
    let selection_start = 2;
    let cursor_offset = 9;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "****already**** bold");
    assert_eq!(result.cursor_offset, 13);
}

#[test]
fn tc_p1_005_03_wrap_partial_existing_formatting() {
    // "text with **bold** here" - select "**bold**"
    let content = "text with **bold** here";
    let selection_start = 10;
    let cursor_offset = 18;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "text with ****bold**** here");
    assert_eq!(result.cursor_offset, 22);
}

// =============================================================================
// TC-P1-005-04: Unwrap (wrap with empty markers)
// =============================================================================

#[test]
fn tc_p1_005_04_unwrap_with_empty_markers() {
    // Note: The wrap transform with empty markers doesn't perform true unwrap.
    // True unwrap requires detecting existing markers. This tests the
    // actual behavior of the transform.
    //
    // Input: Editor content: '**bold** text'
    // Expected: With empty markers, selection stays unchanged (no-op)
    let content = "**bold** text";
    let selection_start = 0;
    let cursor_offset = 8;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "".to_string(),
            after: "".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    // Empty markers = no change
    assert_eq!(result.content, "**bold** text");
    assert_eq!(result.cursor_offset, 8);
}

#[test]
fn tc_p1_005_04_unwrap_simulated_scenario() {
    // Simulate unwrap by selecting content without markers and wrapping
    // "**bold**" -> select "bold" (pos 2 to 6) and wrap with empty
    let content = "**bold**";
    let selection_start = 2;
    let cursor_offset = 6;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "".to_string(),
            after: "".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    // Empty markers don't remove existing markers
    assert_eq!(result.content, "**bold**");
    assert_eq!(result.cursor_offset, 6);
}

#[test]
fn tc_p1_005_04_unwrap_full_content() {
    // Unwrap scenario: select full "**bold**" and apply wrap with empty
    // This should be a no-op (current behavior)
    let content = "**bold** text";
    let selection_start = 0;
    let cursor_offset = 8;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "".to_string(),
            after: "".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "**bold** text");
    assert_eq!(result.cursor_offset, 8);
}

// =============================================================================
// TC-P1-005-05: Wrap with italic, strikethrough, inline code
// =============================================================================

#[test]
fn tc_p1_005_05_wrap_with_italic() {
    // Input: Selection: 'text', apply wrap with *
    // Expected: Text wrapped with *text*
    let content = "Some text here";
    let selection_start = 5;
    let cursor_offset = 9;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "*".to_string(),
            after: "*".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "Some *text* here");
    assert_eq!(result.cursor_offset, 11);
}

#[test]
fn tc_p1_005_05_wrap_with_strikethrough() {
    // Input: Selection: 'text', apply wrap with ~~
    // Expected: Text wrapped with ~~text~~
    let content = "Some text here";
    let selection_start = 5;
    let cursor_offset = 9;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "~~".to_string(),
            after: "~~".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "Some ~~text~~ here");
    assert_eq!(result.cursor_offset, 13);
}

#[test]
fn tc_p1_005_05_wrap_with_inline_code() {
    // Input: Selection: 'text', apply wrap with `
    // Expected: Text wrapped with `text`
    let content = "Some text here";
    let selection_start = 5;
    let cursor_offset = 9;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "`".to_string(),
            after: "`".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "Some `text` here");
    assert_eq!(result.cursor_offset, 11);
}

#[test]
fn tc_p1_005_05_wrap_all_markdown_types() {
    // Test all markdown syntax types
    let content = "format me";
    let selection_start = 0;
    let cursor_offset = 9;

    // Bold
    let bold_result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(bold_result.content, "**format me**");

    // Italic
    let italic_result = editor_apply_transform(
        TransformType::Wrap {
            before: "*".to_string(),
            after: "*".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(italic_result.content, "*format me*");

    // Strikethrough
    let strike_result = editor_apply_transform(
        TransformType::Wrap {
            before: "~~".to_string(),
            after: "~~".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(strike_result.content, "~~format me~~");

    // Inline code
    let code_result = editor_apply_transform(
        TransformType::Wrap {
            before: "`".to_string(),
            after: "`".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );
    assert_eq!(code_result.content, "`format me`");
}

// =============================================================================
// TC-P1-005-06: Wrap with empty selection
// =============================================================================

#[test]
fn tc_p1_005_06_wrap_empty_selection_none() {
    // Input: Cursor at position, no selection (selection_start = None)
    // Expected: Insert markers at cursor position
    let content = "hello world";
    let cursor_offset = 5;
    let selection_start = None;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        selection_start,
    );

    // Inserts ** at cursor position: "hello** world"
    assert_eq!(result.content, "hello** world");
    assert_eq!(result.cursor_offset, 7);
}

#[test]
fn tc_p1_005_06_wrap_empty_selection_equal_offsets() {
    // Input: Empty selection where selection_start == cursor_offset
    // Expected: Inserts markers at cursor (no-op or insert)
    let content = "hello world";
    let cursor_offset = 5;
    let selection_start = Some(5); // Same as cursor_offset = empty selection

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        selection_start,
    );

    // Empty selection inserts markers at cursor
    assert_eq!(result.content, "hello** world");
    assert_eq!(result.cursor_offset, 7);
}

#[test]
fn tc_p1_005_06_wrap_empty_selection_at_start() {
    // Cursor at start, no selection
    let content = "hello";
    let cursor_offset = 0;
    let selection_start = None;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        selection_start,
    );

    assert_eq!(result.content, "**hello");
    assert_eq!(result.cursor_offset, 2);
}

#[test]
fn tc_p1_005_06_wrap_empty_selection_at_end() {
    // Cursor at end, no selection
    let content = "hello";
    let cursor_offset = 5;
    let selection_start = None;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        selection_start,
    );

    assert_eq!(result.content, "hello**");
    assert_eq!(result.cursor_offset, 7);
}

// =============================================================================
// Additional edge cases for comprehensive coverage
// =============================================================================

#[test]
fn wrap_integration_cursor_at_document_end() {
    let content = "hello world";
    let cursor_offset = 11; // End of "hello world"
    let selection_start = Some(6); // Select "world"

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        selection_start,
    );

    assert_eq!(result.content, "hello **world**");
    assert_eq!(result.cursor_offset, 15);
}

#[test]
fn wrap_integration_preserves_surrounding_whitespace() {
    let content = "  spaced  text";
    let selection_start = 2;
    let cursor_offset = 8;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "`".to_string(),
            after: "`".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "  `spaced`  text");
    assert_eq!(result.cursor_offset, 10);
}

#[test]
fn wrap_integration_single_character_selection() {
    let content = "text";
    let selection_start = 2;
    let cursor_offset = 3;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "te**x**t");
    assert_eq!(result.cursor_offset, 7);
}

#[test]
fn wrap_integration_full_document_selected() {
    let content = "hello world";
    let selection_start = 0;
    let cursor_offset = 11;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "[[".to_string(),
            after: "]]".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "[[hello world]]");
    assert_eq!(result.cursor_offset, 15);
}

#[test]
fn wrap_integration_custom_markers() {
    let content = "text";
    let selection_start = 0;
    let cursor_offset = 4;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "***".to_string(),
            after: "***".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "***text***");
    assert_eq!(result.cursor_offset, 10);
}

#[test]
fn wrap_integration_newline_in_selection() {
    // Selection includes newline - should still wrap
    let content = "line1\nline2\nline3";
    let selection_start = 0;
    let cursor_offset = 11; // End of "line1\nline2"

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "> ".to_string(),
            after: "".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    // "> line1\nline2" + "\nline3" = "> line1\nline2\nline3"
    assert_eq!(result.content, "> line1\nline2\nline3");
    assert_eq!(result.cursor_offset, 13);
}

#[test]
fn wrap_integration_link_markers() {
    let content = "text with link text here";
    let selection_start = 10;
    let cursor_offset = 19;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "[".to_string(),
            after: "](url)".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    assert_eq!(result.content, "text with [link text](url) here");
    assert_eq!(result.cursor_offset, 26);
}

// =============================================================================
// IPC command integration tests - verify TransformType mapping
// =============================================================================

#[test]
fn wrap_transform_type_mapping_bold() {
    // Verify TransformType::Wrap correctly maps to Transform::Wrap
    let content = "hello";
    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        5,
        Some(0),
    );

    assert_eq!(result.content, "**hello**");
}

#[test]
fn wrap_transform_type_mapping_italic() {
    let content = "hello";
    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "*".to_string(),
            after: "*".to_string(),
        },
        content.to_string(),
        5,
        Some(0),
    );

    assert_eq!(result.content, "*hello*");
}

#[test]
fn wrap_transform_type_mapping_code() {
    let content = "hello";
    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "`".to_string(),
            after: "`".to_string(),
        },
        content.to_string(),
        5,
        Some(0),
    );

    assert_eq!(result.content, "`hello`");
}

#[test]
fn wrap_transform_type_mapping_strikethrough() {
    let content = "hello";
    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "~~".to_string(),
            after: "~~".to_string(),
        },
        content.to_string(),
        5,
        Some(0),
    );

    assert_eq!(result.content, "~~hello~~");
}

// =============================================================================
// Regression tests for the selection_start fix
// =============================================================================

#[test]
fn regression_selection_start_is_passed_correctly() {
    // This test specifically verifies that selection_start is passed through
    // the IPC command to the transform engine.
    //
    // "hello world" - select "hello" (pos 0 to 5)
    // Without selection_start, result would be "hello** world" (markers at cursor)
    // With selection_start=0, result should be "**hello** world"
    let content = "hello world";
    let cursor_offset = 5;
    let selection_start = 0;

    let result = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        cursor_offset,
        Some(selection_start),
    );

    // If selection_start was NOT passed, this would be "hello** world"
    // With selection_start correctly passed, this should be "**hello** world"
    assert_eq!(result.content, "**hello** world");
    assert_eq!(result.cursor_offset, 9);
}

#[test]
fn regression_no_selection_vs_empty_selection() {
    // No selection (selection_start = None) vs empty selection (start == end)
    let content = "hello";

    // No selection
    let no_sel = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        5,
        None,
    );

    // Empty selection (same start and end)
    let empty_sel = editor_apply_transform(
        TransformType::Wrap {
            before: "**".to_string(),
            after: "**".to_string(),
        },
        content.to_string(),
        5,
        Some(5),
    );

    // Both should produce the same result (insert at cursor)
    assert_eq!(no_sel.content, "hello**");
    assert_eq!(empty_sel.content, "hello**");
    assert_eq!(no_sel.cursor_offset, empty_sel.cursor_offset);
}
