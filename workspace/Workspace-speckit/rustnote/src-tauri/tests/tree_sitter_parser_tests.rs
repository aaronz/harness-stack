use rustnote_lib::parser::{MarkdownParser, TreeSitterParser};

#[test]
fn tc_g006_001_parser_basic_markdown() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "# Heading\n\nParagraph";
    let result = parser.parse(source);
    assert!(result.is_ok(), "Parser should handle basic markdown");
    let (tree, duration) = result.unwrap();
    assert!(
        tree.root_node().byte_range().end > 0,
        "Tree should have content"
    );
    assert!(duration.as_millis() < 100, "Parse should be fast");
}

#[test]
fn tc_g006_001_parser_basic_markdown_heading() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_001_parser_paragraph() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "This is a paragraph with **bold** and *italic* text.";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_001_parser_list() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "- Item 1\n- Item 2\n- Item 3";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_001_parser_blockquote() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "> This is a blockquote";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_001_parser_code_block() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_001_parser_table() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "| Header 1 | Header 2 |\n| -------- | -------- |\n| Cell 1   | Cell 2   |";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_001_parser_emphasis() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "**bold** and *italic* and `inline code`";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_002_parser_incremental_edit_small() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let large_source =
        "# Document Title\n\nThis is a long document with many paragraphs. ".repeat(100);
    let initial_parse = parser.parse(&large_source);
    assert!(initial_parse.is_ok());

    let small_edit = large_source + " Small addition.";
    let start = std::time::Instant::now();
    let result = parser.parse(&small_edit);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "Incremental parse should succeed");
    assert!(
        elapsed.as_millis() < 50,
        "Small edit should parse in < 50ms, took {:?}",
        elapsed
    );
}

#[test]
fn tc_g006_002_parser_incremental_edit_typing() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let base = "# Notes\n\nStart typing here...".to_string();
    parser.parse(&base).expect("Initial parse should succeed");

    for i in 0..100 {
        let edited = format!("{} word{}", base, i);
        let start = std::time::Instant::now();
        let result = parser.parse(&edited);
        let elapsed = start.elapsed();
        assert!(result.is_ok());
        assert!(elapsed.as_millis() < 50, "Typing edit should be fast");
    }
}

#[test]
fn tc_g006_002_parser_incremental_edit_large_doc() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let doc = "## Section\n\nContent here. ".repeat(1000);
    parser.parse(&doc).expect("Initial parse should succeed");

    let edited = doc + "\n\nNew paragraph added.";
    let start = std::time::Instant::now();
    let result = parser.parse(&edited);
    let elapsed = start.elapsed();

    assert!(result.is_ok());
    assert!(
        elapsed.as_millis() < 50,
        "Incremental edit on large doc should be fast"
    );
}

#[test]
fn tc_g006_003_parser_large_document_parsing_time() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let base = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";
    let repeat_count = 100000;
    let large_doc = "# Large Document\n\n".to_string() + &(base.repeat(repeat_count));
    let size_mb = large_doc.len() as f64 / (1024.0 * 1024.0);
    assert!(
        size_mb > 4.0,
        "Document should be > 4MB, was {:.2}MB",
        size_mb
    );

    let start = std::time::Instant::now();
    let result = parser.parse(&large_doc);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "Large document should parse successfully");
    assert!(
        elapsed.as_secs() < 5,
        "Should parse in < 5s, took {:?}",
        elapsed
    );
}

#[test]
fn tc_g006_003_parser_large_document_memory() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let large_doc = "# Large Document\n\n".to_string() + &"Content. ".repeat(50000);
    let _size_mb = large_doc.len() as f64 / (1024.0 * 1024.0);

    let result = parser.parse(&large_doc);
    assert!(result.is_ok());

    let (tree, _) = result.unwrap();
    let tree_size_mb = std::mem::size_of_val(&tree) as f64 / (1024.0 * 1024.0);
    assert!(tree_size_mb < 500.0, "Tree memory should be reasonable");
}

#[test]
fn tc_g006_003_parser_5mb_document() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let content = "## Heading\n\nParagraph text here with some content. ".repeat(105000);
    let large_doc = format!("---\ntitle: Test\n---\n\n# Title\n\n{}", content);
    let size_mb = large_doc.len() as f64 / (1024.0 * 1024.0);
    assert!(
        size_mb > 4.5,
        "Document should be ~5MB, was {:.2}MB",
        size_mb
    );

    let start = std::time::Instant::now();
    let result = parser.parse(&large_doc);
    let elapsed = start.elapsed();

    assert!(result.is_ok(), "5MB document should parse");
    assert!(elapsed.as_secs() < 5, "5MB should parse in < 5s");
}

#[test]
fn tc_g006_004_parser_backward_compat_headings() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "# H1\n## H2\n### H3";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_004_parser_backward_compat_emphasis() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "**bold** and *italic* and ~~strikethrough~~ and `code`";
    let result = parser.parse(source);
    assert!(result.is_ok());
}

#[test]
fn tc_g006_004_parser_backward_compat_blockquote() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "> Single line quote\n>\n> Multi-line quote";
    let result = parser.parse(source);
    assert!(result.is_ok());
}

#[test]
fn tc_g006_004_parser_backward_compat_code_block() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "```javascript\nconsole.log('hello');\n```";
    let result = parser.parse(source);
    assert!(result.is_ok());
}

#[test]
fn tc_g006_004_parser_backward_compat_table() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "| Col1 | Col2 |\n|------|------|\n| A    | B    |";
    let result = parser.parse(source);
    assert!(result.is_ok());
}

#[test]
fn tc_g006_004_parser_backward_compat_mixed() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "# Heading\n\nParagraph with **bold**.\n\n- List item\n- Another item\n\n> A quote\n\n```rust\nfn main() {}\n```";
    let result = parser.parse(source);
    assert!(result.is_ok());
    let (tree, _) = result.unwrap();
    assert!(tree.root_node().byte_range().end > 0);
}

#[test]
fn tc_g006_004_parser_backward_compat_frontmatter() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "---\ntitle: Test\nauthor: Author\n---\n\n# Content";
    let result = parser.parse(source);
    assert!(result.is_ok());
}

#[test]
fn tc_g006_004_parser_backward_compat_nested() {
    let mut parser = TreeSitterParser::new().expect("Failed to create parser");
    let source = "1. Outer item\n   - Nested item\n   - Another nested\n2. Second outer";
    let result = parser.parse(source);
    assert!(result.is_ok());
}

#[test]
fn tc_g006_001_gfm_table_parsing() {
    let parser = MarkdownParser::new();
    let source = "| Header | Header |\n|--------|--------|\n| Cell   | Cell   |";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<table"),
        "Should contain table element, got: {}",
        html
    );
    assert!(
        html.contains("<thead"),
        "Should contain thead element, got: {}",
        html
    );
    assert!(
        html.contains("<tbody"),
        "Should contain tbody element, got: {}",
        html
    );
    assert!(
        html.contains("<tr"),
        "Should contain tr (table row) element, got: {}",
        html
    );
    assert!(
        html.contains("<th"),
        "Should contain th (table header cell), got: {}",
        html
    );
    assert!(
        html.contains("<td"),
        "Should contain td (table data cell), got: {}",
        html
    );
}

#[test]
fn tc_g006_001_gfm_table_multiple_columns() {
    let parser = MarkdownParser::new();
    let source = "| Col1 | Col2 | Col3 |\n|------|------|------|\n| A    | B    | C    |\n| D    | E    | F    |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("<td") || html.contains("<th"),
        "Should contain table cells"
    );
    let td_count = html.matches("<td").count();
    assert!(
        td_count >= 6,
        "Should have at least 6 td cells (2 rows x 3 cols), got {}",
        td_count
    );
}

#[test]
fn tc_g006_001_gfm_table_with_alignment() {
    let parser = MarkdownParser::new();
    let source = "| Left | Center | Right |\n|:-----|:------:|------:|\n| L    | C      | R     |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("align=\"left\""),
        "Should have left alignment attribute, got: {}",
        html
    );
    assert!(
        html.contains("align=\"center\""),
        "Should have center alignment attribute, got: {}",
        html
    );
    assert!(
        html.contains("align=\"right\""),
        "Should have right alignment attribute, got: {}",
        html
    );
}

#[test]
fn tc_g006_002_gfm_task_list_parsing() {
    let parser = MarkdownParser::new();
    let source = "- [x] Done\n- [ ] Not done";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<input"),
        "Should contain input elements for checkboxes, got: {}",
        html
    );
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkbox inputs, got: {}",
        html
    );
    assert!(
        html.contains("checked"),
        "Should have checked attribute for done items, got: {}",
        html
    );
}

#[test]
fn tc_g006_002_gfm_task_list_mixed_states() {
    let parser = MarkdownParser::new();
    let source =
        "- [ ] Unchecked task\n- [x] Checked task\n- [X] Also checked\n- [ ] Another unchecked";
    let html = parser.parse_to_html(source);

    let checked_count = html.matches("checked").count();
    let checkbox_count = html.matches("type=\"checkbox\"").count();

    assert!(
        checkbox_count >= 4,
        "Should have at least 4 checkbox inputs, got {}",
        checkbox_count
    );
    assert!(
        checked_count >= 2,
        "Should have at least 2 checked inputs, got {}",
        checked_count
    );
}

#[test]
fn tc_g006_002_gfm_task_list_nested() {
    let parser = MarkdownParser::new();
    let source = "- [x] Main task\n  - [ ] Subtask 1\n  - [x] Subtask 2";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkboxes"
    );
    assert!(html.contains("checked"), "Should have checked items");
}

#[test]
fn tc_g006_003_gfm_strikethrough_parsing() {
    let parser = MarkdownParser::new();
    let source = "~~deleted text~~";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain del/s/strike element for strikethrough, got: {}",
        html
    );
    assert!(
        html.contains("deleted text"),
        "Should contain the deleted text content, got: {}",
        html
    );
}

#[test]
fn tc_g006_003_gfm_strikethrough_in_context() {
    let parser = MarkdownParser::new();
    let source = "This is **bold** and ~~strikethrough~~ and *italic*.";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough element, got: {}",
        html
    );
    assert!(
        html.contains("<strong>"),
        "Should contain strong element, got: {}",
        html
    );
    assert!(
        html.contains("<em>"),
        "Should contain em element, got: {}",
        html
    );
}

#[test]
fn tc_g006_004_gfm_autolink_parsing() {
    let parser = MarkdownParser::new();
    let source = "<https://example.com>";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<a "),
        "Should contain anchor element for autolink, got: {}",
        html
    );
    assert!(
        html.contains("href=\"https://example.com\""),
        "Should contain correct href attribute, got: {}",
        html
    );
    assert!(
        html.contains("https://example.com"),
        "Should contain the URL as link text, got: {}",
        html
    );
}

#[test]
fn tc_g006_004_gfm_autolink_multiple_protocols() {
    let parser = MarkdownParser::new();
    let source = "<https://example.com>\n<http://example.org>\n<ftp://files.example.net>";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("href=\"https://example.com\""),
        "Should contain https URL, got: {}",
        html
    );
    assert!(
        html.contains("href=\"http://example.org\""),
        "Should contain http URL, got: {}",
        html
    );
    assert!(
        html.contains("href=\"ftp://files.example.net\""),
        "Should contain ftp URL, got: {}",
        html
    );
}

#[test]
fn tc_g006_004_gfm_autolink_email() {
    let parser = MarkdownParser::new();
    let source = "<test@example.com>";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<a ") || html.contains("mailto:"),
        "Should contain anchor or mailto for email, got: {}",
        html
    );
}

#[test]
fn tc_g006_005_gfm_complex_document() {
    let parser = MarkdownParser::new();
    let source = "| Table | Header |\n|--------|--------|\n| Cell   | ~~strike~~ |\n\n- [x] Done task\n- [ ] Not done\n\nMore ~~strikethrough~~ here.";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("<table"),
        "Should contain table element, got: {}",
        html
    );
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough element, got: {}",
        html
    );
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkbox for task list, got: {}",
        html
    );
    assert!(
        html.contains("checked"),
        "Should have checked state, got: {}",
        html
    );
}

#[test]
fn tc_g006_005_gfm_complex_mixed_document() {
    let parser = MarkdownParser::new();
    let source = "# Heading with **bold**\n\n\
                  | Column 1 | Column 2 |\n\
                  |----------|----------|\n\
                  | Cell 1   | Cell 2   |\n\n\
                  - [x] Completed task\n\
                  - [ ] Pending task\n\n\
                  ~~Strikethrough~~ and <https://example.com>\n\n\
                  > Blockquote with *italic*\n\n\
                  ```rust\n\
                  fn main() {}\n\
                  ```";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<h1>"), "Should contain h1, got: {}", html);
    assert!(
        html.contains("<strong>"),
        "Should contain bold, got: {}",
        html
    );
    assert!(
        html.contains("<table"),
        "Should contain table, got: {}",
        html
    );
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkboxes, got: {}",
        html
    );
    let checked_count = html.matches("checked").count();
    assert_eq!(
        checked_count, 1,
        "Should have exactly 1 checked item, got {}",
        checked_count
    );
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough, got: {}",
        html
    );
    assert!(
        html.contains("href=\"https://example.com\""),
        "Should contain autolink, got: {}",
        html
    );
    assert!(
        html.contains("<blockquote>"),
        "Should contain blockquote, got: {}",
        html
    );
    assert!(
        html.contains("<code"),
        "Should contain code element, got: {}",
        html
    );
}

#[test]
fn tc_g006_005_gfm_nested_elements() {
    let parser = MarkdownParser::new();
    let source = "| Header | Header |\n|--------|--------|\n| **bold** | *italic* |\n| ~~strike~~ | `code` |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(html.contains("<strong>"), "Should contain bold in table");
    assert!(html.contains("<em>"), "Should contain italic in table");
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough in table cell"
    );
    assert!(html.contains("<code>"), "Should contain code in table");
}

#[test]
fn tc_g006_edge_malformed_table_inconsistent_columns() {
    let parser = MarkdownParser::new();
    let source = "| Col1 | Col2 | Col3 |\n|------|------|\n| A    | B    |\n| C    | D    | E    |";
    let html = parser.parse_to_html(source);
    assert!(!html.is_empty(), "Should produce some HTML output");
}

#[test]
fn tc_g006_edge_malformed_table_missing_delimiter() {
    let parser = MarkdownParser::new();
    let source = "| Header 1 | Header 2 |\n| Cell 1   | Cell 2   |\n| Cell 3    | Cell 4    |";
    let html = parser.parse_to_html(source);
    assert!(!html.is_empty(), "Should produce some HTML output");
}

#[test]
fn tc_g006_edge_empty_table() {
    let parser = MarkdownParser::new();
    let source = "| |\n|--|\n| |";
    let html = parser.parse_to_html(source);
    assert!(html.contains("<table") || !html.is_empty());
}

#[test]
fn tc_g006_edge_nested_gfm_strikethrough_in_table() {
    let parser = MarkdownParser::new();
    let source = "| Header |\n|--------|\n| ~~deleted~~ |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough in table cell, got: {}",
        html
    );
}

#[test]
fn tc_g006_edge_nested_gfm_task_in_blockquote() {
    let parser = MarkdownParser::new();
    let source = "> - [x] Done task\n> - [ ] Pending task";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<blockquote>"), "Should contain blockquote");
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkboxes in blockquote"
    );
}

#[test]
fn tc_g006_edge_nested_gfm_autolink_in_table() {
    let parser = MarkdownParser::new();
    let source = "| Link |\n|------|\n| <https://example.com> |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(
        html.contains("href=\"https://example.com\""),
        "Should contain autolink in table cell, got: {}",
        html
    );
}

#[test]
fn tc_g006_edge_nested_gfm_multiple_in_cell() {
    let parser = MarkdownParser::new();
    let source = "| Combined |\n|----------|\n| **bold** and *italic* and ~~strike~~ and `code` |";
    let html = parser.parse_to_html(source);

    assert!(html.contains("<table"), "Should contain table");
    assert!(html.contains("<strong>"), "Should contain bold");
    assert!(html.contains("<em>"), "Should contain italic");
    assert!(
        html.contains("<del>") || html.contains("<s>") || html.contains("<strike>"),
        "Should contain strikethrough"
    );
    assert!(html.contains("<code>"), "Should contain code");
}

#[test]
fn tc_g006_edge_nested_gfm_task_with_emphasis() {
    let parser = MarkdownParser::new();
    let source = "- [ ] **bold** task\n- [x] *italic* task";
    let html = parser.parse_to_html(source);

    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkboxes"
    );
    assert!(html.contains("<strong>"), "Should contain bold in task");
    assert!(html.contains("<em>"), "Should contain italic in task");
}
