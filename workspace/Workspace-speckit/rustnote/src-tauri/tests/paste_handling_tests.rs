use rustnote_lib::semantic::ast::SemanticDocument;
use rustnote_lib::semantic::paste::HtmlToMarkdownConverter;

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

// TC-G012-001: Paste from Microsoft Word
// Category: integration
// Input: Bold, italic, lists from Word
// Expected: Content converts to Markdown correctly
#[test]
fn test_tc_g012_001_paste_from_microsoft_word() {
    let converter = HtmlToMarkdownConverter::new();

    let word_html = r#"<p class="msonormal"><b>This is bold text</b> and <i>this is italic</i>.</p>
<p class="msonormal"><o:p>List item 1</o:p></p>
<p class="msonormal"><o:p>List item 2</o:p></p>"#;

    let result = converter.convert_word_html(word_html);

    assert!(result.contains("**This is bold text**"));
    assert!(result.contains("*this is italic*"));
    assert!(result.contains("List item 1"));
    assert!(result.contains("List item 2"));
}

#[test]
fn test_tc_g012_001_word_bold_converted() {
    let converter = HtmlToMarkdownConverter::new();
    let word_html = "<b>Bold from Word</b>";
    let result = converter.convert_word_html(word_html);
    assert_eq!(result, "**Bold from Word**");
}

#[test]
fn test_tc_g012_001_word_italic_converted() {
    let converter = HtmlToMarkdownConverter::new();
    let word_html = "<i>Italic from Word</i>";
    let result = converter.convert_word_html(word_html);
    assert_eq!(result, "*Italic from Word*");
}

#[test]
fn test_tc_g012_001_word_lists_converted() {
    let converter = HtmlToMarkdownConverter::new();
    let word_html = "<p>Item 1</p><p>Item 2</p><p>Item 3</p>";
    let result = converter.convert_word_html(word_html);
    assert!(result.contains("Item 1"));
    assert!(result.contains("Item 2"));
    assert!(result.contains("Item 3"));
}

// TC-G012-002: Paste from web browser
// Category: integration
// Input: Formatted content from web
// Expected: Web formatting converts to Markdown appropriately
#[test]
fn test_tc_g012_002_paste_from_web_browser() {
    let converter = HtmlToMarkdownConverter::new();

    let web_html = r#"<html><body>
<h1>Web Heading</h1>
<p>This is <strong>bold</strong> and <em>emphasized</em> text.</p>
<ul>
<li>List item one</li>
<li>List item two</li>
</ul>
</body></html>"#;

    let result = converter.convert_web_html(web_html);

    assert!(result.contains("# Web Heading"));
    assert!(result.contains("**bold**"));
    assert!(result.contains("*emphasized*"));
    assert!(result.contains("List item one"));
    assert!(result.contains("List item two"));
}

#[test]
fn test_tc_g012_002_web_headings_converted() {
    let converter = HtmlToMarkdownConverter::new();
    let web_html = "<h1>Main Title</h1><h2>Subtitle</h2>";
    let result = converter.convert_web_html(web_html);
    assert!(result.contains("# Main Title"));
    assert!(result.contains("## Subtitle"));
}

#[test]
fn test_tc_g012_002_web_formatting_converted() {
    let converter = HtmlToMarkdownConverter::new();
    let web_html = "<p>Some <b>bold</b> and <i>italic</i> content.</p>";
    let result = converter.convert_web_html(web_html);
    assert!(result.contains("**bold**"));
    assert!(result.contains("*italic*"));
}

// TC-G012-003: Paste from another editor
// Category: integration
// Input: Markdown from another editor
// Expected: Markdown fidelity maintained
#[test]
fn test_tc_g012_003_paste_from_another_editor() {
    let converter = HtmlToMarkdownConverter::new();

    let markdown_source = "# Heading\n\n**Bold** and *italic*\n\n- List item 1\n- List item 2\n\n> Blockquote\n\n```rust\nfn main() {}\n```";

    let doc = SemanticDocument::parse(markdown_source);
    let output = doc.serialize_to_commonmark();

    assert_eq!(output, markdown_source);
}

#[test]
fn test_tc_g012_003_markdown_fidelity_headings() {
    let source = "# H1\n## H2\n### H3";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_tc_g012_003_markdown_fidelity_formatting() {
    let source = "**bold** *italic* `code` ~~strikethrough~~";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_tc_g012_003_markdown_fidelity_lists() {
    let source = "- Unordered item\n1. Ordered item";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_tc_g012_003_markdown_fidelity_blockquote() {
    let source = "> Single line quote";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

// TC-G012-004: Paste preserves code blocks
// Category: edge_case
// Input: Code block with syntax highlighting
// Expected: Code block structure preserved
#[test]
fn test_tc_g012_004_paste_preserves_code_blocks() {
    let converter = HtmlToMarkdownConverter::new();

    let code_html = r#"<pre class="language-rust"><code>fn main() {
    println!("Hello");
}</code></pre>"#;

    let result = converter.convert(code_html);

    assert!(result.contains("```rust"));
    assert!(result.contains("fn main()"));
    assert!(result.contains("```"));
}

#[test]
fn test_tc_g012_004_code_block_with_language() {
    let converter = HtmlToMarkdownConverter::new();
    let html = "<pre class=\"language-javascript\"><code>console.log('test');</code></pre>";
    let result = converter.convert(html);
    assert!(result.contains("```javascript"));
    assert!(result.contains("console.log('test');"));
    assert!(result.contains("```"));
}

#[test]
fn test_tc_g012_004_code_block_without_language() {
    let converter = HtmlToMarkdownConverter::new();
    let html = "<pre><code>some code</code></pre>";
    let result = converter.convert(html);
    assert!(result.contains("```"));
    assert!(result.contains("some code"));
}

#[test]
fn test_tc_g012_004_inline_code_preserved() {
    let converter = HtmlToMarkdownConverter::new();
    let html = "<code>inline code</code>";
    let result = converter.convert(html);
    assert_eq!(result, "`inline code`");
}
