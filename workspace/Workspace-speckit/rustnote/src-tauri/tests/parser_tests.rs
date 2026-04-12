use rustnote_lib::semantic::ast::{Position, SemanticDocument};

#[test]
fn test_parse_headings_h1_h6() {
    let cases = vec![
        "# H1",
        "## H2",
        "### H3",
        "#### H4",
        "##### H5",
        "###### H6",
    ];

    for source in cases {
        let doc = SemanticDocument::parse(source);
        let output = doc.serialize_to_commonmark();
        assert_eq!(output, source, "Heading level should roundtrip correctly");
    }
}

#[test]
fn test_roundtrip_headings() {
    let source = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_emphasis_bold() {
    let source = "**bold text**";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_emphasis_italic() {
    let source = "*italic text*";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_emphasis_strikethrough() {
    let source = "~~strikethrough~~";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_inline_code() {
    let source = "`inline code`";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_code_block() {
    let source = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_link() {
    let source = "[link text](https://example.com)";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_image() {
    let source = "![alt text](image.png)";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_unordered_list() {
    let source = "- Item 1\n- Item 2\n- Item 3";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_ordered_list() {
    let source = "1. First\n2. Second\n3. Third";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_task_list() {
    let source = "- [x] Done\n- [ ] Todo";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_blockquote() {
    let source = "> This is a quote";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_table() {
    let source = "| Header 1 | Header 2 |\n| -------- | -------- |\n| Cell 1   | Cell 2   |";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_horizontal_rule_dashes() {
    let source = "---\n\nparagraph\n\n---";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_horizontal_rule_asterisks() {
    let source = "___\n\nparagraph\n\n___";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_frontmatter() {
    let source = "---\ntitle: Hello\nauthor: Test\n---\n\n# Content";
    let doc = SemanticDocument::parse(source);
    assert!(doc.has_frontmatter());
    assert_eq!(doc.get_frontmatter(), Some("title: Hello\nauthor: Test"));
}

#[test]
fn test_parse_frontmatter_roundtrip() {
    let source = "---\ntitle: Hello\nauthor: Test\n---\n\n# Content";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_with_frontmatter();
    assert!(output.starts_with("---\ntitle: Hello\nauthor: Test\n---\n\n"));
    assert!(output.contains("# Content"));
}

#[test]
fn test_parse_no_frontmatter() {
    let source = "# Just a heading";
    let doc = SemanticDocument::parse(source);
    assert!(!doc.has_frontmatter());
    assert_eq!(doc.get_frontmatter(), None);
}

#[test]
fn test_source_access() {
    let source = "# Hello World";
    let doc = SemanticDocument::parse(source);
    assert_eq!(doc.source(), "# Hello World");
}

#[test]
fn test_parse_nested_lists() {
    let source = "- Item 1\n  - Nested 1\n  - Nested 2\n- Item 2";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_mixed_content() {
    let source = "# Heading\n\nSome paragraph text with **bold** and *italic*.\n\n- List item 1\n- List item 2\n\n> A blockquote";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_code_fence_with_language() {
    let source = "```javascript\nconsole.log('hello');\n```";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_empty_document() {
    let source = "";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_parse_whitespace_only() {
    let source = "   \n\n   \n";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_offset_to_position() {
    let source = "Line 1\nLine 2\nLine 3";
    let doc = SemanticDocument::parse(source);

    let pos = doc.offset_to_position(0);
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);

    let pos = doc.offset_to_position(7);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 0);
}

#[test]
fn test_position_to_offset() {
    let source = "Line 1\nLine 2\nLine 3";
    let doc = SemanticDocument::parse(source);

    let offset = doc.position_to_offset(Position::new(0, 0, 0));
    assert_eq!(offset, 0);

    let offset = doc.position_to_offset(Position::new(7, 1, 0));
    assert_eq!(offset, 7);
}
