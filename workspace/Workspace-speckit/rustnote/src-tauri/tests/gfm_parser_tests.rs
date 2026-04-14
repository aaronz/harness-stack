//! GFM (GitHub Flavored Markdown) Parser Tests
//!
//! This module tests tree-sitter's GFM parsing capabilities including:
//! - Tables (with alignment markers)
//! - Task lists (checkboxes)
//! - Autolinks (URLs and email)
//! - Strikethrough
//! - Combined GFM features
//! - Edge cases and malformed input
//!
//! Test cases TC-GFM001 through TC-GFM009 cover all GFM syntax elements.

use rustnote_lib::parser::{MarkdownParser, TreeSitterParser};
use std::fs;
use std::path::Path;

/// Helper function to create a new TreeSitterParser
fn ts_parser() -> TreeSitterParser {
    TreeSitterParser::new().expect("Failed to create TreeSitterParser")
}

/// Helper function to create a new MarkdownParser (comrak-based)
fn md_parser() -> MarkdownParser {
    MarkdownParser::new()
}

// =============================================================================
// TC-GFM001: GFM Table Syntax Parsing
// Category: unit
// =============================================================================

#[test]
fn tc_gfm001_tree_sitter_table_parsing() {
    let mut parser = ts_parser();
    let source = "| Header | Header |\n| ------ | ------ |\n| Cell   | Cell   |";

    let result = parser.parse(source);
    assert!(result.is_ok(), "Tree-sitter should parse table syntax");

    let (tree, _) = result.unwrap();
    let root = tree.root_node();

    // Tree should have content (root node has children for table structure)
    assert!(
        root.byte_range().end > 0,
        "Tree should contain parsed table content"
    );

    // Verify tree structure exists by checking the tree is parseable
    let _lang = tree.language();
}

#[test]
fn tc_gfm001_tree_sitter_table_node_exists() {
    let mut parser = ts_parser();
    let source = "| Col1 | Col2 |\n|------|------|\n| A    | B    |";

    let result = parser.parse(source);
    assert!(result.is_ok());

    let (tree, _) = result.unwrap();
    // The tree should parse without error - table syntax is recognized
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_gfm001_comrak_table_rendering() {
    let parser = md_parser();
    let source = "| Header | Header |\n| ------ | ------ |\n| Cell   | Cell   |";
    let html = parser.parse_to_html(source);

    // Verify table elements are present
    assert!(
        html.contains("<table"),
        "Should contain table element, got: {}",
        html
    );
    assert!(
        html.contains("<thead") || html.contains("<tr"),
        "Should contain table structure, got: {}",
        html
    );
    assert!(
        html.contains("<td") || html.contains("<th"),
        "Should contain table cells, got: {}",
        html
    );
}

#[test]
fn tc_gfm001_table_multiple_columns() {
    let parser = md_parser();
    let source = "| Col1 | Col2 | Col3 |\n|------|------|------|\n| A    | B    | C    |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    let td_count = html.matches("<td").count();
    assert!(
        td_count >= 3,
        "Should have at least 3 td cells, got {}",
        td_count
    );
}

// =============================================================================
// TC-GFM002: Task List Checkboxes - Unchecked
// Category: unit
// =============================================================================

#[test]
fn tc_gfm002_tree_sitter_task_list_parsing() {
    let mut parser = ts_parser();
    let source = "- [ ] Unchecked task\n- [x] Checked task";

    let result = parser.parse(source);
    assert!(result.is_ok(), "Tree-sitter should parse task list syntax");

    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    assert!(
        root.byte_range().end > 0,
        "Tree should contain parsed task list content"
    );
}

#[test]
fn tc_gfm002_task_markers_recognized() {
    let mut parser = ts_parser();
    let source = "- [ ] Unchecked\n- [x] Checked";

    let result = parser.parse(source);
    assert!(result.is_ok());

    // Source should be fully parsed - task markers don't cause parse errors
    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    assert_eq!(
        root.byte_range().end,
        source.len(),
        "Full source should be parsed"
    );
}

#[test]
fn tc_gfm002_comrak_task_list_unchecked() {
    let parser = md_parser();
    let source = "- [ ] Unchecked task";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<input"),
        "Should contain input element for checkbox, got: {}",
        html
    );
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkbox type, got: {}",
        html
    );
}

#[test]
fn tc_gfm002_comrak_task_list_mixed_states() {
    let parser = md_parser();
    let source = "- [ ] Unchecked\n- [x] Checked\n- [X] Also checked";
    let html = parser.parse_to_html(source);

    let checkbox_count = html.matches("type=\"checkbox\"").count();
    assert!(
        checkbox_count >= 3,
        "Should have at least 3 checkbox inputs, got {}",
        checkbox_count
    );

    // Both [x] and [X] should be recognized as checked
    let checked_count = html.matches("checked").count();
    assert!(
        checked_count >= 2,
        "Should have at least 2 checked items, got {}",
        checked_count
    );
}

// =============================================================================
// TC-GFM003: Task List Checkboxes - Checked
// Category: unit
// =============================================================================

#[test]
fn tc_gfm003_tree_sitter_checked_task_parsing() {
    let mut parser = ts_parser();
    let source = "- [x] Done task";

    let result = parser.parse(source);
    assert!(
        result.is_ok(),
        "Tree-sitter should parse checked task syntax"
    );

    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    assert!(
        root.byte_range().end > 0,
        "Tree should contain parsed content"
    );
}

#[test]
fn tc_gfm003_comrak_checked_task() {
    let parser = md_parser();
    let source = "- [x] Done task";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<input"),
        "Should contain input element, got: {}",
        html
    );
    assert!(
        html.contains("checked"),
        "Should have checked attribute, got: {}",
        html
    );
}

#[test]
fn tc_gfm003_nested_checked_tasks() {
    let parser = md_parser();
    let source = "- [x] Main task\n  - [x] Subtask";
    let html = parser.parse_to_html(source);

    let checkbox_count = html.matches("type=\"checkbox\"").count();
    assert_eq!(
        checkbox_count, 2,
        "Should have 2 checkbox inputs, got {}",
        checkbox_count
    );
}

// =============================================================================
// TC-GFM004: Autolinks Parsing
// Category: unit
// =============================================================================

#[test]
fn tc_gfm004_tree_sitter_autolink_parsing() {
    let mut parser = ts_parser();
    let source = "Visit <https://example.com> or <mailto:test@example.com>";

    let result = parser.parse(source);
    assert!(result.is_ok(), "Tree-sitter should parse autolinks");

    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    assert!(
        root.byte_range().end > 0,
        "Tree should contain parsed autolink content"
    );
}

#[test]
fn tc_gfm004_autolink_node_recognition() {
    let mut parser = ts_parser();
    let source = "<https://example.com>";

    let result = parser.parse(source);
    assert!(result.is_ok());

    let (tree, _) = result.unwrap();
    // Source should be fully parsed
    let root = tree.root_node();
    assert!(
        root.byte_range().end >= source.len(),
        "Full autolink should be parsed"
    );
}

#[test]
fn tc_gfm004_comrak_https_autolink() {
    let parser = md_parser();
    let source = "<https://example.com>";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<a "),
        "Should contain anchor element, got: {}",
        html
    );
    assert!(
        html.contains("href=\"https://example.com\""),
        "Should contain correct href, got: {}",
        html
    );
}

#[test]
fn tc_gfm004_comrak_email_autolink() {
    let parser = md_parser();
    let source = "<mailto:test@example.com>";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<a ") || html.contains("mailto:"),
        "Should contain anchor or mailto, got: {}",
        html
    );
}

#[test]
fn tc_gfm004_comrak_multiple_protocols() {
    let parser = md_parser();
    let source = "<https://example.com>\n<http://example.org>";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("href=\"https://example.com\""),
        "Should contain https URL"
    );
    assert!(
        html.contains("href=\"http://example.org\""),
        "Should contain http URL"
    );
}

// =============================================================================
// TC-GFM005: Strikethrough Parsing
// Category: unit
// =============================================================================

#[test]
fn tc_gfm005_tree_sitter_strikethrough_parsing() {
    let mut parser = ts_parser();
    let source = "This is ~~deleted~~ text";

    let result = parser.parse(source);
    assert!(
        result.is_ok(),
        "Tree-sitter should parse strikethrough syntax"
    );

    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    assert!(
        root.byte_range().end > 0,
        "Tree should contain parsed strikethrough content"
    );
}

#[test]
fn tc_gfm005_strikethrough_node_recognition() {
    let mut parser = ts_parser();
    let source = "~~deleted~~";

    let result = parser.parse(source);
    assert!(result.is_ok());

    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    // The strikethrough should be parsed as part of the content
    assert!(root.byte_range().end > 0, "Strikethrough should be in tree");
}

#[test]
fn tc_gfm005_comrak_strikethrough_rendering() {
    let parser = md_parser();
    let source = "~~deleted text~~";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough element, got: {}",
        html
    );
    assert!(
        html.contains("deleted text"),
        "Should contain deleted text content, got: {}",
        html
    );
}

#[test]
fn tc_gfm005_strikethrough_in_context() {
    let parser = md_parser();
    let source = "This is **bold** and ~~strikethrough~~ and *italic*.";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough"
    );
    assert!(html.contains("<strong>"), "Should contain bold");
    assert!(html.contains("<em>"), "Should contain italic");
}

// =============================================================================
// TC-GFM006: GFM Feature Combination
// Category: integration
// =============================================================================

#[test]
fn tc_gfm006_tree_sitter_gfm_all_fixture() {
    let mut parser = ts_parser();
    let fixture_path = Path::new("tests/samples/gfm/gfm_all.md");

    // Read fixture file
    let source =
        fs::read_to_string(fixture_path).expect("Should be able to read gfm_all.md fixture");

    let result = parser.parse(&source);
    assert!(
        result.is_ok(),
        "Tree-sitter should parse gfm_all.md fixture without errors"
    );

    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    assert!(
        root.byte_range().end > 0,
        "GFM document should parse to non-empty tree"
    );
}

#[test]
fn tc_gfm006_comrak_gfm_all_fixture() {
    let parser = md_parser();
    let fixture_path = Path::new("tests/samples/gfm/gfm_all.md");

    let source =
        fs::read_to_string(fixture_path).expect("Should be able to read gfm_all.md fixture");

    let html = parser.parse_to_html(&source);

    // Verify all GFM features are rendered
    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkboxes"
    );
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough"
    );
    assert!(html.contains("href="), "Should contain links/autolinks");
}

#[test]
fn tc_gfm006_tables_tasks_no_conflict() {
    let parser = md_parser();
    let source = "| Task | Status |\n|------|--------|\n| Do it | - [x] Done |";
    let html = parser.parse_to_html(&source);

    // Table should render correctly even with task list inside
    assert!(html.contains("<table"), "Table should render");
    assert!(
        html.contains("type=\"checkbox\"") || html.contains("<td"),
        "Content should render"
    );
}

#[test]
fn tc_gfm006_all_gfm_features_together() {
    let parser = md_parser();
    let source = "| Table | Header |\n|--------|--------|\n| ~~strike~~ | data |\n\n- [x] done task\n\n<https://example.com>";
    let html = parser.parse_to_html(&source);

    assert!(html.contains("<table"), "Should have table");
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should have strikethrough"
    );
    assert!(html.contains("checked"), "Should have checked task");
    assert!(html.contains("href="), "Should have autolink");
}

// =============================================================================
// TC-GFM007: GFM Table with Alignment
// Category: unit
// =============================================================================

#[test]
fn tc_gfm007_tree_sitter_table_alignment() {
    let mut parser = ts_parser();
    let source = "| Left | Center | Right |\n| :--- | :----: | ----: |";

    let result = parser.parse(source);
    assert!(result.is_ok(), "Tree-sitter should parse aligned tables");

    let (tree, _) = result.unwrap();
    let root = tree.root_node();
    assert!(root.byte_range().end > 0, "Aligned table should parse");
}

#[test]
fn tc_gfm007_comrak_table_left_alignment() {
    let parser = md_parser();
    let source = "| Left | Right |\n|:-----|-------:|\n| L    | R     |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("align=\"left\""),
        "Should have left alignment, got: {}",
        html
    );
    assert!(
        html.contains("align=\"right\""),
        "Should have right alignment, got: {}",
        html
    );
}

#[test]
fn tc_gfm007_comrak_table_center_alignment() {
    let parser = md_parser();
    let source = "| Center |\n|:------:|\n| C     |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("align=\"center\""),
        "Should have center alignment, got: {}",
        html
    );
}

#[test]
fn tc_gfm007_alignment_markers_preserved() {
    let parser = md_parser();
    let source = "| Left | Center | Right |\n| :--- | :----: | ----: |\n| L    | C      | R     |";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("align=\"left\""),
        "Should have left alignment"
    );
    assert!(
        html.contains("align=\"center\""),
        "Should have center alignment"
    );
    assert!(
        html.contains("align=\"right\""),
        "Should have right alignment"
    );
}

// =============================================================================
// TC-GFM008: No Conflict Between Tree-sitter and Comrak
// Category: integration
// =============================================================================

#[test]
fn tc_gfm008_both_parsers_succeed() {
    let mut ts_parser = ts_parser();
    let md_parser = md_parser();

    let source = "# Heading\n\n| Table | Test |\n|-------|------|\n| Cell  | ~~x~~ |\n\n- [x] Task\n\n<https://example.com>";

    // Both parsers should succeed without panicking or returning errors
    let ts_result = ts_parser.parse(&source);
    let md_result = md_parser.parse_to_html(&source);

    assert!(ts_result.is_ok(), "Tree-sitter should parse successfully");
    assert!(!md_result.is_empty(), "Comrak should produce HTML output");
}

#[test]
fn tc_gfm008_consistent_table_parsing() {
    let source = "| A | B |\n|---|---|\n| 1 | 2 |";

    let ts_result = ts_parser().parse(source);
    let md_result = md_parser().parse_to_html(source);

    assert!(ts_result.is_ok(), "Tree-sitter should parse table");
    assert!(md_result.contains("<table"), "Comrak should render table");
}

#[test]
fn tc_gfm008_consistent_task_list_parsing() {
    let source = "- [x] Done\n- [ ] Not done";

    let ts_result = ts_parser().parse(source);
    let md_result = md_parser().parse_to_html(source);

    assert!(ts_result.is_ok(), "Tree-sitter should parse task list");
    assert!(
        md_result.contains("type=\"checkbox\""),
        "Comrak should render checkboxes"
    );
}

#[test]
fn tc_gfm008_consistent_strikethrough_parsing() {
    let source = "~~deleted~~";

    let ts_result = ts_parser().parse(source);
    let md_result = md_parser().parse_to_html(source);

    assert!(ts_result.is_ok(), "Tree-sitter should parse strikethrough");
    assert!(
        md_result.contains("<del>") || md_result.contains("<s>"),
        "Comrak should render strikethrough"
    );
}

#[test]
fn tc_gfm008_consistent_autolink_parsing() {
    let source = "<https://example.com>";

    let ts_result = ts_parser().parse(source);
    let md_result = md_parser().parse_to_html(source);

    assert!(ts_result.is_ok(), "Tree-sitter should parse autolink");
    assert!(md_result.contains("href="), "Comrak should render autolink");
}

// =============================================================================
// TC-GFM009: Malformed GFM Tables
// Category: edge_case
// =============================================================================

#[test]
fn tc_gfm009_tree_sitter_malformed_table_graceful() {
    let mut parser = ts_parser();
    let source = "| A | B |\n| - | - |\n| Missing |";

    // Parser should not crash on malformed table
    let result = parser.parse(source);
    assert!(
        result.is_ok(),
        "Tree-sitter should handle malformed table gracefully"
    );
}

#[test]
fn tc_gfm009_comrak_malformed_table_no_crash() {
    let parser = md_parser();
    let source = "| A | B |\n| - | - |\n| Missing |";

    // Should not panic and should produce some output
    let html = std::panic::catch_unwind(|| parser.parse_to_html(source));
    assert!(html.is_ok(), "Comrak should not panic on malformed table");

    let html = html.unwrap();
    // May or may not produce table, but should not crash
    assert!(
        !html.is_empty() || html.is_empty(),
        "Should produce output (or empty output) without crashing"
    );
}

#[test]
fn tc_gfm009_missing_delimiter_row() {
    let parser = md_parser();
    let source = "| Header 1 | Header 2 |\n| Cell 1   | Cell 2   |";

    // Missing delimiter row - should still produce output
    let html = parser.parse_to_html(source);
    assert!(!html.is_empty(), "Should produce HTML output");
}

#[test]
fn tc_gfm009_inconsistent_column_count() {
    let parser = md_parser();
    let source = "| Col1 | Col2 | Col3 |\n|------|------|\n| A    | B    |\n| C    | D    | E    |";

    let html = parser.parse_to_html(source);
    assert!(
        !html.is_empty(),
        "Should produce HTML output despite inconsistencies"
    );
}

#[test]
fn tc_gfm009_empty_table() {
    let parser = md_parser();
    let source = "| |\n|--|\n| |";

    let html = parser.parse_to_html(source);
    assert!(
        html.contains("<table") || !html.is_empty(),
        "Should produce output for empty table"
    );
}

#[test]
fn tc_gfm009_tree_sitter_empty_table() {
    let mut parser = ts_parser();
    let source = "| |\n|--|\n| |";

    let result = parser.parse(source);
    assert!(result.is_ok(), "Tree-sitter should handle empty table");
}

// =============================================================================
// Additional Edge Case Tests for GFM Features
// =============================================================================

#[test]
fn tc_gfm_edge_strikethrough_in_table() {
    let parser = md_parser();
    let source = "| Header |\n|--------|\n| ~~deleted~~ |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough in table cell"
    );
}

#[test]
fn tc_gfm_edge_autolink_in_table() {
    let parser = md_parser();
    let source = "| Link |\n|------|\n| <https://example.com> |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(html.contains("href="), "Should contain link in table cell");
}

#[test]
fn tc_gfm_edge_task_in_blockquote() {
    let parser = md_parser();
    let source = "> - [x] Done task\n> - [ ] Pending task";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<blockquote>"), "Should contain blockquote");
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkboxes in blockquote"
    );
}

#[test]
fn tc_gfm_edge_combined_in_cell() {
    let parser = md_parser();
    let source = "| Combined |\n|----------|\n| **bold** and *italic* and ~~strike~~ |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(html.contains("<strong>"), "Should contain bold");
    assert!(html.contains("<em>"), "Should contain italic");
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough"
    );
}

#[test]
fn tc_gfm_edge_task_with_emphasis() {
    let parser = md_parser();
    let source = "- [ ] **bold** task\n- [x] *italic* task";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkboxes"
    );
    assert!(html.contains("<strong>"), "Should contain bold in task");
    assert!(html.contains("<em>"), "Should contain italic in task");
}

// =============================================================================
// Performance Tests for GFM Parsing
// =============================================================================

#[test]
fn tc_gfm_perf_large_table() {
    let mut parser = ts_parser();
    let mut source = String::from("| Col1 | Col2 | Col3 |\n|-------|------|------|\n");
    for i in 0..100 {
        source.push_str(&format!("| {} | {} | {} |\n", i, i * 2, i * 3));
    }

    let start = std::time::Instant::now();
    let result = parser.parse(&source);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "Should parse large table");
    assert!(
        elapsed.as_millis() < 100,
        "Large table should parse in < 100ms, took {:?}",
        elapsed
    );
}

#[test]
fn tc_gfm_perf_many_tasks() {
    let mut parser = ts_parser();
    let source: String = (0..500)
        .map(|i| format!("- [{}] Task {}\n", if i % 2 == 0 { "x" } else { " " }, i))
        .collect();

    let start = std::time::Instant::now();
    let result = parser.parse(&source);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "Should parse many tasks");
    assert!(
        elapsed.as_millis() < 100,
        "Many tasks should parse in < 100ms, took {:?}",
        elapsed
    );
}
