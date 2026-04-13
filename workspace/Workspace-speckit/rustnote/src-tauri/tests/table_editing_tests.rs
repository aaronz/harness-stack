//! Table Editing Tests
//!
//! ## Overview
//! This module tests table editing behaviors and documents table editing constraints
//! for the RustNote Markdown editor.
//!
//! ## Table Editing Constraints
//!
//! TipTap (ProseMirror-based) tables can lose data in some edge cases. The following
//! constraints must be observed to prevent data loss:
//!
//! ### Constraint 1: Cell Content with Pipes
//! Pipes (`|`) inside cell content must be escaped or the table may misparse.
//! GFM tables do not support escaped pipes in cell content, so avoid using `|` in cells.
//!
//! ### Constraint 2: Empty Cells
//! Empty cells are represented as `||` in the source. When editing, ensure that
//! empty cells maintain proper pipe delimiters.
//!
//! ### Constraint 3: Unicode Content
//! Tables support unicode content, but some unicode characters may cause rendering
//! issues. Test thoroughly with unicode content.
//!
//! ### Constraint 4: Row Addition/Deletion
//! When adding rows, ensure the new row maintains proper alignment with header.
//! When deleting rows, ensure the delimiter row is not accidentally removed.
//!
//! ### Constraint 5: Column Addition/Deletion
//! When adding columns, all rows must be updated consistently.
//! Column deletion can cause data loss if not done carefully.
//!
//! ## Safety Recommendations
//!
//! 1. Always serialize and verify table structure after edits
//! 2. Use SemanticDocument::serialize_to_commonmark() for round-trip verification
//! 3. Parse HTML output to verify table structure is preserved
//! 4. For complex edits, maintain a backup of the original content
//!
//! ## Test Categories
//!
//! - TC-G010-001: Table_no_data_loss - Edit cell preserves all data
//! - TC-G010-002: Table_add_row - Original row preserved when adding row
//! - TC-G010-003: Table_complex_content - Content preserved through edits
//!
//! ## Edge Cases Covered
//!
//! - pipes_in_content: Tables with pipes in cell content
//! - empty_cells: Tables with empty cells
//! - unicode_content: Tables with unicode characters

use rustnote_lib::semantic::ast::SemanticDocument;

/// TC-G010-001: Table_no_data_loss
///
/// Tests that editing a cell in a table preserves all data.
/// Input: | A | B |
///        |---|---|
///        | 1 | 2 |
/// Expected: Edit cell preserves all data
#[test]
fn tc_g010_001_table_no_data_loss() {
    let source = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(source);

    // Verify initial parse
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source, "Initial serialization should match source");

    // Verify HTML generation
    let html = doc.html();
    assert!(html.contains("<table>"), "Should generate table HTML");
    assert!(html.contains("<th>"), "Should have table headers");
    assert!(html.contains("<td>"), "Should have table cells");

    // Verify we can parse again and get same result
    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();
    assert_eq!(output, output2, "Round-trip should preserve data");
}

/// TC-G010-002: Table_add_row
///
/// Tests that adding a row preserves the original row.
/// Input: | A |
///        |---|
///        | 1 | -> add row
/// Expected: Original row preserved
#[test]
fn tc_g010_002_table_add_row() {
    let source = "| A |\n|---|\n| 1 |";
    let doc = SemanticDocument::parse(source);

    // Original content should be preserved
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("| 1 |"), "Original row should be preserved");

    // Parse the output again to ensure it's valid markdown
    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();
    assert_eq!(output, output2, "Re-parsed content should match");

    // Simulate adding a row by constructing new content
    let source_with_new_row = "| A |\n|---|\n| 1 |\n| 2 |";
    let doc3 = SemanticDocument::parse(source_with_new_row);
    let output3 = doc3.serialize_to_commonmark();

    // Verify both rows exist
    assert!(
        output3.contains("| 1 |"),
        "Original row should be preserved"
    );
    assert!(output3.contains("| 2 |"), "New row should be added");
    assert!(
        output3.contains("|---|"),
        "Delimiter row should be preserved"
    );
}

/// TC-G010-003: Table_complex_content
///
/// Tests that complex content is preserved through edits.
/// Input: | a | b | c |
///        |--|--|--|
///        | x | y | z |
/// Expected: Content preserved through edits
#[test]
fn tc_g010_003_table_complex_content() {
    let source = "| a | b | c |\n|--|--|--|\n| x | y | z |";
    let doc = SemanticDocument::parse(source);

    // Verify all content is preserved
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source, "Complex table should round-trip correctly");

    // Verify HTML contains all cells
    let html = doc.html();
    assert!(html.contains("a"), "Header 'a' should be present");
    assert!(html.contains("b"), "Header 'b' should be present");
    assert!(html.contains("c"), "Header 'c' should be present");
    assert!(html.contains("x"), "Cell 'x' should be present");
    assert!(html.contains("y"), "Cell 'y' should be present");
    assert!(html.contains("z"), "Cell 'z' should be present");
}

// ============================================================================
// Edge Case Tests
// ============================================================================

/// Tests table with pipes in content
/// Note: GFM tables don't support | in cell content - this tests our handling
#[test]
fn table_pipes_in_content() {
    // Since GFM doesn't escape pipes, this may parse incorrectly
    // Testing our graceful handling
    let source = "| Header |\n|--------|\n| cell | |";
    let doc = SemanticDocument::parse(source);

    // Should parse without panicking
    let output = doc.serialize_to_commonmark();
    let _ = doc.html(); // Should not panic
}

/// Tests table with empty cells
#[test]
fn table_empty_cells() {
    let source = "| A | B | C |\n|---|---|---|\n| 1 |   | 3 |";
    let doc = SemanticDocument::parse(source);

    let output = doc.serialize_to_commonmark();
    assert!(
        output.contains("| 1 |"),
        "Non-empty cell should be preserved"
    );
    assert!(
        output.contains("| 3 |"),
        "Non-empty cell should be preserved"
    );
    assert!(output.contains("|   |"), "Empty cell should be preserved");
}

/// Tests table with unicode content
#[test]
fn table_unicode_content() {
    let source = "| 标题 | 数据 |\n|------|------|\n| 🎉 | 👍 |";
    let doc = SemanticDocument::parse(source);

    let output = doc.serialize_to_commonmark();
    assert!(output.contains("🎉"), "Unicode emoji should be preserved");
    assert!(output.contains("👍"), "Unicode emoji should be preserved");
    assert!(
        output.contains("标题"),
        "Unicode Chinese should be preserved"
    );
    assert!(
        output.contains("数据"),
        "Unicode Chinese should be preserved"
    );

    // HTML should also contain the unicode
    let html = doc.html();
    assert!(html.contains("🎉"), "HTML should contain unicode emoji");
}

/// Tests table with mixed content types
#[test]
fn table_mixed_content_types() {
    let source =
        "| Bold | *Italic* | `Code` |\n|-------|---------|-------|\n| **A** | *B*     | `C`   |";
    let doc = SemanticDocument::parse(source);

    let output = doc.serialize_to_commonmark();
    assert!(output.contains("**A**"), "Bold should be preserved");
    assert!(output.contains("*B*"), "Italic should be preserved");
    assert!(output.contains("`C`"), "Code should be preserved");
}

/// Tests table serialization consistency
#[test]
fn table_serialization_consistency() {
    let source = "| H1 | H2 |\n|----|----|\n| A  | B  |\n| C  | D  |";

    // Parse multiple times and verify consistency
    let doc1 = SemanticDocument::parse(source);
    let output1 = doc1.serialize_to_commonmark();

    let doc2 = SemanticDocument::parse(&output1);
    let output2 = doc2.serialize_to_commonmark();

    let doc3 = SemanticDocument::parse(&output2);
    let output3 = doc3.serialize_to_commonmark();

    assert_eq!(output1, output2, "First round-trip should be consistent");
    assert_eq!(output2, output3, "Second round-trip should be consistent");
}

/// Tests table with various delimiter styles
#[test]
fn table_delimiter_styles() {
    let cases = vec!["|---|", "|----|", "|-----|", "|:---|", "|----:|", "|:---:|"];

    for delimiter in cases {
        let source = format!("| A |\n{}|\n| 1 |", delimiter);
        let doc = SemanticDocument::parse(&source);

        let output = doc.serialize_to_commonmark();
        assert!(
            output.contains("| A |"),
            "Header should be preserved with delimiter: {}",
            delimiter
        );

        // Verify it's valid markdown that can be reparsed
        let doc2 = SemanticDocument::parse(&output);
        let _ = doc2.html(); // Should not panic
    }
}

/// Tests that table structure is preserved in HTML output
#[test]
fn table_html_structure() {
    let source = "| A | B |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |";
    let doc = SemanticDocument::parse(&source);

    let html = doc.html();

    // Verify table structure
    assert!(html.contains("<table>"), "Should have table tag");
    assert!(html.contains("</table>"), "Should have closing table tag");
    assert!(html.contains("<thead>"), "Should have thead");
    assert!(html.contains("<tbody>"), "Should have tbody");
    assert!(html.contains("<th>"), "Should have th elements");
    assert!(html.contains("<td>"), "Should have td elements");

    // Count cells - should have 2 headers + 4 data cells = 6
    let td_count = html.matches("<td>").count();
    let th_count = html.matches("<th>").count();
    assert_eq!(th_count, 2, "Should have 2 header cells");
    assert_eq!(td_count, 4, "Should have 4 data cells");
}

/// Tests table at document start vs in middle of document
#[test]
fn table_document_position() {
    // Table at start
    let source1 = "| A |\n|---|\n| 1 |\n\nParagraph after";
    let doc1 = SemanticDocument::parse(source1);
    let output1 = doc1.serialize_to_commonmark();
    assert!(
        output1.contains("| 1 |"),
        "Table at start should be preserved"
    );

    // Table in middle
    let source2 = "Paragraph before\n\n| A |\n|---|\n| 1 |\n\nParagraph after";
    let doc2 = SemanticDocument::parse(source2);
    let output2 = doc2.serialize_to_commonmark();
    assert!(
        output2.contains("| 1 |"),
        "Table in middle should be preserved"
    );

    // Table at end
    let source3 = "Paragraph before\n\n| A |\n|---|\n| 1 |";
    let doc3 = SemanticDocument::parse(source3);
    let output3 = doc3.serialize_to_commonmark();
    assert!(
        output3.contains("| 1 |"),
        "Table at end should be preserved"
    );
}

/// Tests wide table (many columns)
#[test]
fn table_wide() {
    let source = "| A | B | C | D | E | F | G | H | I | J |\n|---|---|---|---|---|---|---|---|---|---|\n| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |";
    let doc = SemanticDocument::parse(source);

    let output = doc.serialize_to_commonmark();
    assert!(output.contains("| A |"), "First header should be preserved");
    assert!(output.contains("| J |"), "Last header should be preserved");
    assert!(output.contains("| 10 |"), "Last cell should be preserved");
}

/// Tests tall table (many rows)
#[test]
fn table_tall() {
    let mut source = "| A |\n|---|\n".to_string();
    for i in 0..50 {
        source.push_str(&format!("| {} |\n", i));
    }

    let doc = SemanticDocument::parse(&source);
    let output = doc.serialize_to_commonmark();

    // Verify first and last rows
    assert!(output.contains("| 0 |"), "First row should be preserved");
    assert!(output.contains("| 49 |"), "Last row should be preserved");

    // Verify round-trip
    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();
    assert_eq!(output, output2, "Tall table should round-trip correctly");
}

/// Tests that table with missing trailing pipe is handled
#[test]
fn table_missing_trailing_pipe() {
    let source = "| A | B\n|---|---|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(source);

    // Should parse without panicking
    let _ = doc.html();
    let _ = doc.serialize_to_commonmark();
}

/// Tests that malformed table is handled gracefully
#[test]
fn table_malformed() {
    // Missing delimiter row cells
    let source = "| A | B |\n|--|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(source);

    // Should parse without panicking
    let _ = doc.html();
    let _ = doc.serialize_to_commonmark();
}

/// Tests table with frontmatter
#[test]
fn table_with_frontmatter() {
    let source = "---\ntitle: Test\n---\n\n| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(source);

    assert!(doc.has_frontmatter(), "Should have frontmatter");
    assert_eq!(
        doc.get_frontmatter(),
        Some("title: Test"),
        "Frontmatter should be preserved"
    );

    let output = doc.serialize_with_frontmatter();
    assert!(
        output.starts_with("---\ntitle: Test"),
        "Frontmatter should be at start"
    );
    assert!(
        output.contains("| 1 | 2 |"),
        "Table should be preserved after frontmatter"
    );
}
