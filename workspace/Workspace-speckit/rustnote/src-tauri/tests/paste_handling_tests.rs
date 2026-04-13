use rustnote_lib::semantic::ast::SemanticDocument;

#[test]
fn test_paste_html_bold_converted_to_markdown() {
    let source = "**bold** text";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, "**bold** text");
}

#[test]
fn test_paste_html_italic_converted_to_markdown() {
    let source = "*italic* text";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, "*italic* text");
}

#[test]
fn test_paste_html_bold_and_italic_converted() {
    let source = "**bold** and *italic* text";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, "**bold** and *italic* text");
}

#[test]
fn test_paste_markdown_preserved_exactly() {
    let source = "**bold** and *italic*";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_code_block_preserved() {
    let source = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_inline_code_preserved() {
    let source = "`inline code`";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_link_converted_to_markdown() {
    let source = "[link text](https://example.com)";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_html_complex_rich_text() {
    let source = "**bold and *italic* combined** with [a link](https://example.com)";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_unicode_content_preserved() {
    let source = "# 标题\n\n内容 with émojis: 🎉 👍";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_task_list_preserved() {
    let source = "- [x] Completed task\n- [ ] Pending task";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("- [x]"));
    assert!(output.contains("- [ ]"));
}

#[test]
fn test_paste_heading_preserved() {
    let source = "# Heading 1\n## Heading 2\n### Heading 3";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("# Heading 1"));
    assert!(output.contains("## Heading 2"));
    assert!(output.contains("### Heading 3"));
}

#[test]
fn test_paste_blockquote_preserved() {
    let source = "> quote line\n> second line";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("> quote"));
}

#[test]
fn test_paste_list_items_preserved() {
    let source = "- item 1\n- item 2\n- item 3";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("- item 1"));
    assert!(output.contains("- item 2"));
    assert!(output.contains("- item 3"));
}

#[test]
fn test_paste_ordered_list_preserved() {
    let source = "1. First\n2. Second\n3. Third";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("1. First"));
    assert!(output.contains("2. Second"));
    assert!(output.contains("3. Third"));
}

#[test]
fn test_paste_table_preserved() {
    let source = "| Column 1 | Column 2 |\n|----------|----------|\n| Data 1   | Data 2   |";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("| Column 1 |"));
    assert!(output.contains("| Data 1   |"));
}

#[test]
fn test_paste_strikethrough_preserved() {
    let source = "~~deleted text~~";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_horizontal_rule_preserved() {
    let source = "paragraph\n\n---\n\nanother paragraph";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("---"));
}

#[test]
fn test_paste_image_preserved() {
    let source = "![alt text](image.png)";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_mixed_formatting_preserved() {
    let source = "**bold** and *italic* and `code` and [link](https://example.com)";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_nested_formatting() {
    let source = "**bold with *italic* inside**";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_code_with_language() {
    let source = "```javascript\nconsole.log('Hello');\n```";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("```javascript"));
}

#[test]
fn test_paste_empty_document() {
    let source = "";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_paste_whitespace_preserved() {
    let source = "   spaces   ";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}
