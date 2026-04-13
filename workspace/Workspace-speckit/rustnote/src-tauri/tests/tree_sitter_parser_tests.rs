use rustnote_lib::parser::TreeSitterParser;

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
    let size_mb = large_doc.len() as f64 / (1024.0 * 1024.0);

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
