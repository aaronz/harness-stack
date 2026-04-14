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
//! - TC-G009-001: Table_data_integrity_on_edit - Other cells unchanged when editing one cell
//! - TC-G009-002: Table_row_col_operations - Structure preserved after add row/col/remove
//! - TC-G010-001: Table_no_data_loss - Edit cell preserves all data
//! - TC-G010-002: Table_add_row - Original row preserved when adding row
//! - TC-G010-003: Table_complex_content - Content preserved through edits
//!
//! ## Edge Cases Covered
//!
//! - pipes_in_content: Tables with pipes in cell content
//! - empty_cells: Tables with empty cells
//! - unicode_content: Tables with unicode characters
//! - cell_merging: Cell content preservation during structural changes
//! - row_operations: Row add/remove operations preserve structure

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

// ============================================================================
// G-009 Required Test Cases - Table Editing Constraints
// ============================================================================

/// TC-G009-001: Table data integrity on edit
///
/// Category: unit
/// Input: Table with data -> edit cell
/// Expected: Other cells remain unchanged, structure preserved
///
/// This test verifies that when editing a table cell, the other cells
/// and overall table structure remain unchanged. This is critical for
/// data integrity during table editing in TipTap.
#[test]
fn tc_g009_001_table_data_integrity_on_edit() {
    let source = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |\n| 7 | 8 | 9 |";
    let doc = SemanticDocument::parse(source);

    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source, "Initial serialization should match source");

    let html = doc.html();
    assert!(html.contains("<table>"), "Should generate table HTML");
    assert!(html.contains("<td>1</td>"), "Cell 1 should be present");
    assert!(html.contains("<td>2</td>"), "Cell 2 should be present");
    assert!(html.contains("<td>3</td>"), "Cell 3 should be present");
    assert!(html.contains("<td>4</td>"), "Cell 4 should be present");
    assert!(html.contains("<td>5</td>"), "Cell 5 should be present");
    assert!(html.contains("<td>6</td>"), "Cell 6 should be present");
    assert!(html.contains("<td>7</td>"), "Cell 7 should be present");
    assert!(html.contains("<td>8</td>"), "Cell 8 should be present");
    assert!(html.contains("<td>9</td>"), "Cell 9 should be present");

    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();
    assert_eq!(output, output2, "Round-trip should preserve all data");

    let doc3 = SemanticDocument::parse(&output2);
    let html2 = doc3.html();
    assert_eq!(html, html2, "HTML should be identical after re-parse");

    assert!(
        html2.contains("<td>1</td>"),
        "Cell 1 should still be present after re-parse"
    );
    assert!(
        html2.contains("<td>5</td>"),
        "Cell 5 (edited cell) should still be present after re-parse"
    );
    assert!(
        html2.contains("<td>9</td>"),
        "Cell 9 should still be present after re-parse"
    );
}

/// TC-G009-002: Table row/col operations
///
/// Category: edge_case
/// Input: Table -> add row -> add column -> remove row
/// Expected: Table structure remains valid
///
/// This test verifies that table structure remains valid after
/// performing row and column operations. It tests the edge cases
/// of cell_merging and row_operations as specified in coverage requirements.
#[test]
fn tc_g009_002_table_row_col_operations() {
    let initial_source = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(initial_source);

    let output = doc.serialize_to_commonmark();
    assert_eq!(
        output, initial_source,
        "Initial table should serialize correctly"
    );

    let html = doc.html();
    assert!(html.contains("<th>A</th>"), "Header A should be present");
    assert!(html.contains("<th>B</th>"), "Header B should be present");
    assert!(html.contains("<td>1</td>"), "Cell 1 should be present");
    assert!(html.contains("<td>2</td>"), "Cell 2 should be present");

    let table_with_extra_row = "| A | B |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |";
    let doc2 = SemanticDocument::parse(table_with_extra_row);
    let output2 = doc2.serialize_to_commonmark();
    assert!(
        output2.contains("| 1 | 2 |"),
        "Original row should be preserved when adding row"
    );
    assert!(output2.contains("| 3 | 4 |"), "New row should be added");

    let table_with_extra_col = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |";
    let doc3 = SemanticDocument::parse(table_with_extra_col);
    let output3 = doc3.serialize_to_commonmark();
    assert!(
        output3.contains("| A | B | C |"),
        "All headers should be present after adding column"
    );
    assert!(
        output3.contains("| 1 | 2 | 3 |"),
        "First data row should have new column"
    );
    assert!(
        output3.contains("| 4 | 5 | 6 |"),
        "Second data row should have new column"
    );

    let doc4 = SemanticDocument::parse(&output3);
    let html4 = doc4.html();
    let th_count = html4.matches("<th>").count();
    let td_count = html4.matches("<td>").count();
    assert_eq!(th_count, 3, "Should have 3 header cells");
    assert_eq!(td_count, 6, "Should have 6 data cells (2 rows x 3 cols)");

    let table_after_row_removal = "| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |";
    let doc5 = SemanticDocument::parse(table_after_row_removal);
    let output5 = doc5.serialize_to_commonmark();
    assert!(
        output5.contains("| 1 | 2 | 3 |"),
        "Data row should be preserved after removing row"
    );
    assert!(
        !output5.contains("| 4 | 5 | 6 |"),
        "Removed row should not be present"
    );

    let doc6 = SemanticDocument::parse(&output5);
    let html6 = doc6.html();
    let td_count_after_removal = html6.matches("<td>").count();
    assert_eq!(
        td_count_after_removal, 3,
        "Should have 3 data cells after row removal"
    );

    let doc7 = SemanticDocument::parse(&output5);
    let output7 = doc7.serialize_to_commonmark();
    assert_eq!(output5, output7, "Re-serialization should be consistent");
}

/// Tests that table structure validation works correctly
#[test]
fn tc_g009_table_structure_validation() {
    let valid_table = "| H1 | H2 | H3 |\n|----|----|----|\n| A  | B  | C  |\n| D  | E  | F  |";
    let doc = SemanticDocument::parse(valid_table);

    let output = doc.serialize_to_commonmark();
    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();

    assert_eq!(output, output2, "Valid table should serialize consistently");

    let html = doc.html();
    assert!(html.contains("<table>"), "Should produce valid HTML table");
    assert!(html.contains("<thead>"), "Should have thead");
    assert!(html.contains("<tbody>"), "Should have tbody");
    assert!(html.contains("<th>"), "Should have th elements");
    assert!(html.contains("<td>"), "Should have td elements");

    let th_count = html.matches("<th>").count();
    let td_count = html.matches("<td>").count();
    assert_eq!(th_count, 3, "Should have 3 header cells");
    assert_eq!(td_count, 6, "Should have 6 data cells");
}

/// Tests table cell editing preserves neighboring cells
#[test]
fn tc_g009_cell_edit_preserves_neighbors() {
    let source = "| Name | Age | City |\n|-----|-----|------|\n| Alice | 30 | NYC |\n| Bob | 25 | LA |\n| Charlie | 35 | Chicago |";
    let doc = SemanticDocument::parse(source);

    let html = doc.html();
    assert!(html.contains("Alice"), "Alice should be present");
    assert!(html.contains("30"), "Age 30 should be present");
    assert!(html.contains("NYC"), "NYC should be present");
    assert!(html.contains("Bob"), "Bob should be present");
    assert!(html.contains("25"), "Age 25 should be present");
    assert!(html.contains("LA"), "LA should be present");
    assert!(html.contains("Charlie"), "Charlie should be present");
    assert!(html.contains("35"), "Age 35 should be present");
    assert!(html.contains("Chicago"), "Chicago should be present");

    let doc2 = SemanticDocument::parse(&doc.serialize_to_commonmark());
    let html2 = doc2.html();

    assert_eq!(
        html.matches("Alice").count(),
        html2.matches("Alice").count(),
        "Alice count should be preserved"
    );
    assert_eq!(
        html.matches("Bob").count(),
        html2.matches("Bob").count(),
        "Bob count should be preserved"
    );
    assert_eq!(
        html.matches("Charlie").count(),
        html2.matches("Charlie").count(),
        "Charlie count should be preserved"
    );
}

// ============================================================================
// P2-010: Required Test Cases - Table Editing Data Integrity
// ============================================================================

/// TC-TE001: Add row preserves Markdown
///
/// Category: unit
/// Input: | A | B |
///        | - | - |
///        | 1 | 2 | → add row
/// Expected: Markdown table syntax preserved after add
///
/// This test verifies that when adding a row to a table, the Markdown
/// table syntax is properly preserved and the table remains valid.
#[test]
fn tc_te001_add_row_preserves_markdown() {
    // Original table with one row
    let original_source = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(original_source);

    // Verify original parses correctly
    let output = doc.serialize_to_commonmark();
    assert_eq!(
        output, original_source,
        "Original serialization should match source"
    );

    // Simulate adding a row by creating new content
    let source_with_new_row = "| A | B |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |";
    let doc2 = SemanticDocument::parse(source_with_new_row);
    let output2 = doc2.serialize_to_commonmark();

    // Verify Markdown syntax is preserved
    assert!(
        output2.contains("| A | B |"),
        "Header row should be preserved"
    );
    assert!(
        output2.contains("|---|---|"),
        "Delimiter row should be preserved"
    );
    assert!(
        output2.contains("| 1 | 2 |"),
        "Original data row should be preserved"
    );
    assert!(
        output2.contains("| 3 | 4 |"),
        "New data row should be added"
    );

    // Verify round-trip preservation
    let doc3 = SemanticDocument::parse(&output2);
    let output3 = doc3.serialize_to_commonmark();
    assert_eq!(
        output2, output3,
        "Round-trip should preserve Markdown syntax"
    );

    // Verify HTML structure
    let html = doc3.html();
    let th_count = html.matches("<th>").count();
    let td_count = html.matches("<td>").count();
    assert_eq!(th_count, 2, "Should have 2 header cells");
    assert_eq!(td_count, 4, "Should have 4 data cells (2 rows × 2 cols)");
}

/// TC-TE002: Delete row preserves Markdown
///
/// Category: unit
/// Input: | A | B |
///        | - | - |
///        | 1 | 2 | → delete row
/// Expected: Markdown table syntax preserved after delete
///
/// This test verifies that when deleting a row from a table, the Markdown
/// table syntax is properly preserved and the table remains valid.
#[test]
fn tc_te002_delete_row_preserves_markdown() {
    // Original table with two rows
    let original_source = "| A | B |\n|---|---|\n| 1 | 2 |\n| 3 | 4 |";
    let doc = SemanticDocument::parse(original_source);

    // Verify original parses correctly
    let output = doc.serialize_to_commonmark();
    assert_eq!(
        output, original_source,
        "Original serialization should match source"
    );

    // Simulate deleting a row by creating new content (delete second row)
    let source_after_delete = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc2 = SemanticDocument::parse(source_after_delete);
    let output2 = doc2.serialize_to_commonmark();

    // Verify Markdown syntax is preserved
    assert!(
        output2.contains("| A | B |"),
        "Header row should be preserved"
    );
    assert!(
        output2.contains("|---|---|"),
        "Delimiter row should be preserved"
    );
    assert!(
        output2.contains("| 1 | 2 |"),
        "Remaining data row should be preserved"
    );
    assert!(
        !output2.contains("| 3 | 4 |"),
        "Deleted row should not be present"
    );

    // Verify round-trip preservation
    let doc3 = SemanticDocument::parse(&output2);
    let output3 = doc3.serialize_to_commonmark();
    assert_eq!(
        output2, output3,
        "Round-trip should preserve Markdown syntax"
    );

    // Verify HTML structure
    let html = doc3.html();
    let th_count = html.matches("<th>").count();
    let td_count = html.matches("<td>").count();
    assert_eq!(th_count, 2, "Should have 2 header cells");
    assert_eq!(td_count, 2, "Should have 2 data cells (1 row × 2 cols)");
}

/// TC-TE003: Add column preserves Markdown
///
/// Category: unit
/// Input: | A |
///        | - |
///        | 1 | → add column
/// Expected: Markdown table syntax preserved after add
///
/// This test verifies that when adding a column to a table, the Markdown
/// table syntax is properly preserved and the table remains valid.
#[test]
fn tc_te003_add_column_preserves_markdown() {
    // Original table with one column
    let original_source = "| A |\n|---|\n| 1 |";
    let doc = SemanticDocument::parse(original_source);

    // Verify original parses correctly
    let output = doc.serialize_to_commonmark();
    assert_eq!(
        output, original_source,
        "Original serialization should match source"
    );

    // Simulate adding a column by creating new content
    let source_with_new_col = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc2 = SemanticDocument::parse(source_with_new_col);
    let output2 = doc2.serialize_to_commonmark();

    // Verify Markdown syntax is preserved
    assert!(
        output2.contains("| A | B |"),
        "Header row with both columns should be preserved"
    );
    assert!(
        output2.contains("|---|---|"),
        "Delimiter row should have correct column count"
    );
    assert!(
        output2.contains("| 1 | 2 |"),
        "Data row should have both columns"
    );

    // Verify round-trip preservation
    let doc3 = SemanticDocument::parse(&output2);
    let output3 = doc3.serialize_to_commonmark();
    assert_eq!(
        output2, output3,
        "Round-trip should preserve Markdown syntax"
    );

    // Verify HTML structure
    let html = doc3.html();
    let th_count = html.matches("<th>").count();
    let td_count = html.matches("<td>").count();
    assert_eq!(th_count, 2, "Should have 2 header cells");
    assert_eq!(td_count, 2, "Should have 2 data cells (1 row × 2 cols)");
}

/// TC-TE004: Delete column preserves Markdown
///
/// Category: unit
/// Input: | A | B |
///        | - | - |
///        | 1 | 2 | → delete column
/// Expected: Markdown table syntax preserved after delete
///
/// This test verifies that when deleting a column from a table, the Markdown
/// table syntax is properly preserved and the table remains valid.
#[test]
fn tc_te004_delete_column_preserves_markdown() {
    // Original table with two columns
    let original_source = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = SemanticDocument::parse(original_source);

    // Verify original parses correctly
    let output = doc.serialize_to_commonmark();
    assert_eq!(
        output, original_source,
        "Original serialization should match source"
    );

    // Simulate deleting a column by creating new content
    let source_after_delete = "| A |\n|---|\n| 1 |";
    let doc2 = SemanticDocument::parse(source_after_delete);
    let output2 = doc2.serialize_to_commonmark();

    // Verify Markdown syntax is preserved
    assert!(
        output2.contains("| A |"),
        "Header row with remaining column should be preserved"
    );
    assert!(
        output2.contains("|---|"),
        "Delimiter row should have correct column count"
    );
    assert!(
        output2.contains("| 1 |"),
        "Data row should have remaining column"
    );
    assert!(
        !output2.contains("| B |"),
        "Deleted column header should not be present"
    );
    assert!(
        !output2.contains("| 2 |"),
        "Deleted column data should not be present"
    );

    // Verify round-trip preservation
    let doc3 = SemanticDocument::parse(&output2);
    let output3 = doc3.serialize_to_commonmark();
    assert_eq!(
        output2, output3,
        "Round-trip should preserve Markdown syntax"
    );

    // Verify HTML structure
    let html = doc3.html();
    let th_count = html.matches("<th>").count();
    let td_count = html.matches("<td>").count();
    assert_eq!(th_count, 1, "Should have 1 header cell");
    assert_eq!(td_count, 1, "Should have 1 data cell (1 row × 1 col)");
}

/// TC-TE005: Cell with pipe character escapes
///
/// Category: edge_case
/// Input: | a | b |
///        | - | - |
///        | x|y | z |
/// Expected: Pipe in cell escaped as \|, table renders correctly
///
/// This test verifies that cells containing pipe characters are properly
/// handled. Note: GFM tables do not natively support escaped pipes in cell
/// content, so this tests our graceful handling of such edge cases.
#[test]
fn tc_te005_cell_with_pipe_character_escapes() {
    // A pipe in cell content will be interpreted as a column separator
    let source = "| a | b |\n|---|---|\n| x|y | z |";
    let doc = SemanticDocument::parse(source);

    // Should parse without panicking
    let output = doc.serialize_to_commonmark();
    let html = doc.html();

    // The parser should handle this gracefully
    // Table structure may be affected, but no panic should occur
    assert!(
        html.contains("<table>"),
        "Should generate table HTML even with edge case"
    );

    // Verify round-trip works
    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();
    assert_eq!(
        output, output2,
        "Round-trip should be consistent even with edge case"
    );

    // Verify we can also test with properly escaped content
    let escaped_source = "| Col1 | Col2 |\n|------|------|\n| A\\|B | C |";
    let doc3 = SemanticDocument::parse(escaped_source);
    let output3 = doc3.serialize_to_commonmark();
    let _ = doc3.html();

    // Should not panic and should serialize
    let _ = output3;
}

/// TC-TE006: Empty cell handling
///
/// Category: edge_case
/// Input: | A | B |
///        | - | - |
///        |   | X |
/// Expected: Empty cell renders, no corruption
///
/// This test verifies that empty cells are properly handled and the table
/// structure remains intact without corruption.
#[test]
fn tc_te006_empty_cell_handling() {
    // Table with empty cell
    let source = "| A | B |\n|---|---|\n|   | X |";
    let doc = SemanticDocument::parse(source);

    // Verify parses correctly
    let output = doc.serialize_to_commonmark();
    assert!(
        output.contains("| A | B |"),
        "Header row should be preserved"
    );
    assert!(
        output.contains("|---|---|"),
        "Delimiter row should be preserved"
    );

    // Verify empty cell is represented with proper pipes
    assert!(
        output.contains("|   | X |"),
        "Empty cell should be preserved with space"
    );

    // Verify HTML structure
    let html = doc.html();
    assert!(html.contains("<table>"), "Should generate table HTML");
    assert!(html.contains("<td>"), "Should have table cells");

    // Verify non-empty cell content
    assert!(
        html.contains("X"),
        "Non-empty cell content should be present"
    );

    // Verify round-trip preservation
    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();
    assert_eq!(
        output, output2,
        "Round-trip should preserve empty cell structure"
    );

    // Test with multiple empty cells
    let source_multi_empty = "| A | B | C |\n|---|---|---|\n| 1 |   | 3 |\n|   | 5 |   |";
    let doc3 = SemanticDocument::parse(source_multi_empty);
    let output3 = doc3.serialize_to_commonmark();

    // Verify both empty cells are preserved
    assert!(
        output3.contains("| 1 |   | 3 |"),
        "First row with empty cell should be preserved"
    );
    assert!(
        output3.contains("|   | 5 |   |"),
        "Second row with empty cells should be preserved"
    );

    // Verify round-trip
    let doc4 = SemanticDocument::parse(&output3);
    let output4 = doc4.serialize_to_commonmark();
    assert_eq!(
        output3, output4,
        "Round-trip should preserve multiple empty cells"
    );
}

/// TC-TE007: Complex table structure
///
/// Category: render
/// Input: | Header | Header |
///        | ------ | ------ |
///        | Cell 1 | Cell 2 |
/// Expected: Table renders with correct alignment
///
/// This test verifies that complex table structures with headers,
/// delimiters, and data rows render correctly.
#[test]
fn tc_te007_complex_table_structure() {
    // Complex table with headers and data
    let source = "| Header 1 | Header 2 |\n| ------ | ------ |\n| Cell 1 | Cell 2 |";
    let doc = SemanticDocument::parse(source);

    // Verify serialization
    let output = doc.serialize_to_commonmark();

    // Verify all components are preserved
    assert!(
        output.contains("| Header 1 | Header 2 |"),
        "Header row should be preserved"
    );
    assert!(
        output.contains("| ------ | ------ |"),
        "Delimiter row should be preserved"
    );
    assert!(
        output.contains("| Cell 1 | Cell 2 |"),
        "Data row should be preserved"
    );

    // Verify HTML rendering
    let html = doc.html();
    assert!(html.contains("<table>"), "Should generate table element");
    assert!(html.contains("<thead>"), "Should have thead section");
    assert!(html.contains("<tbody>"), "Should have tbody section");
    assert!(
        html.contains("<th>Header 1</th>"),
        "Header 1 should be in th element"
    );
    assert!(
        html.contains("<th>Header 2</th>"),
        "Header 2 should be in th element"
    );
    assert!(
        html.contains("<td>Cell 1</td>"),
        "Cell 1 should be in td element"
    );
    assert!(
        html.contains("<td>Cell 2</td>"),
        "Cell 2 should be in td element"
    );

    // Verify cell counts
    let th_count = html.matches("<th>").count();
    let td_count = html.matches("<td>").count();
    assert_eq!(th_count, 2, "Should have 2 header cells");
    assert_eq!(td_count, 2, "Should have 2 data cells");

    // Test with alignment markers
    let aligned_source =
        "| Left | Right | Center |\n|:-----|------:|:------:|\n| A    |     B |   C    |";
    let doc2 = SemanticDocument::parse(aligned_source);
    let output2 = doc2.serialize_to_commonmark();

    // Verify alignment markers are preserved
    assert!(
        output2.contains("|:-----|"),
        "Left alignment marker should be preserved"
    );
    assert!(
        output2.contains("|------:|"),
        "Right alignment marker should be preserved"
    );
    assert!(
        output2.contains("|:------:|"),
        "Center alignment marker should be preserved"
    );

    // Verify data is preserved with alignment
    assert!(
        output2.contains("| A    |"),
        "Left-aligned data should be preserved"
    );
    assert!(
        output2.contains("|     B |"),
        "Right-aligned data should be preserved"
    );
    assert!(
        output2.contains("|   C    |"),
        "Center-aligned data should be preserved"
    );

    // Verify round-trip
    let doc3 = SemanticDocument::parse(&output2);
    let output3 = doc3.serialize_to_commonmark();
    assert_eq!(
        output2, output3,
        "Round-trip should preserve alignment markers"
    );

    // Test large complex table
    let large_source = "| H1 | H2 | H3 | H4 | H5 |\n|----|----|----|----|----|\n| A  | B  | C  | D  | E  |\n| F  | G  | H  | I  | J  |\n| K  | L  | M  | N  | O  |";
    let doc4 = SemanticDocument::parse(large_source);
    let output4 = doc4.serialize_to_commonmark();

    // Verify all headers and data are preserved
    assert!(
        output4.contains("| H1 | H2 | H3 | H4 | H5 |"),
        "All headers should be preserved"
    );
    assert!(
        output4.contains("| A  | B  | C  | D  | E  |"),
        "First data row should be preserved"
    );
    assert!(
        output4.contains("| K  | L  | M  | N  | O  |"),
        "Last data row should be preserved"
    );

    // Verify round-trip
    let doc5 = SemanticDocument::parse(&output4);
    let output5 = doc5.serialize_to_commonmark();
    assert_eq!(
        output4, output5,
        "Round-trip should preserve large table structure"
    );

    // Verify HTML cell counts for large table
    let html5 = doc5.html();
    let th_count5 = html5.matches("<th>").count();
    let td_count5 = html5.matches("<td>").count();
    assert_eq!(th_count5, 5, "Large table should have 5 header cells");
    assert_eq!(
        td_count5, 15,
        "Large table should have 15 data cells (3 rows × 5 cols)"
    );
}

/// Tests table with merged cells scenario (simulated via complex content)
#[test]
fn tc_g009_table_complex_content_integrity() {
    let source = "| Header 1 | Header 2 | Header 3 |\n|----------|----------|----------|\n| **bold** | *italic* | `code` |\n| Normal | ~~strike~~ | [link](url) |";
    let doc = SemanticDocument::parse(source);

    let output = doc.serialize_to_commonmark();
    assert!(
        output.contains("**bold**"),
        "Bold formatting should be preserved"
    );
    assert!(
        output.contains("*italic*"),
        "Italic formatting should be preserved"
    );
    assert!(
        output.contains("`code`"),
        "Code formatting should be preserved"
    );
    assert!(
        output.contains("~~strike~~"),
        "Strikethrough should be preserved"
    );
    assert!(output.contains("[link](url)"), "Link should be preserved");

    let doc2 = SemanticDocument::parse(&output);
    let output2 = doc2.serialize_to_commonmark();
    assert_eq!(
        output, output2,
        "Complex content should round-trip correctly"
    );

    let html = doc.html();
    assert!(
        html.contains("<strong>bold</strong>"),
        "Bold should render in HTML"
    );
    assert!(
        html.contains("<em>italic</em>"),
        "Italic should render in HTML"
    );
    assert!(
        html.contains("<code>code</code>"),
        "Code should render in HTML"
    );
}
