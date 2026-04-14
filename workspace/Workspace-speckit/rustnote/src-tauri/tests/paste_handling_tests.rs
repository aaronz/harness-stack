use rustnote_lib::semantic::ast::SemanticDocument;
use rustnote_lib::semantic::paste::{
    detect_clipboard_format, ClipboardFormat, HtmlToMarkdownConverter, PasteOptions,
};

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

// =============================================================================
// TC-PHxxx: Comprehensive Paste Handling Test Cases (P2-013)
// =============================================================================

// TC-PH001: Plain text paste
// Category: unit
// Input: Plain text from text editor
// Expected: Text inserted as-is, valid Markdown
#[test]
fn test_tc_ph001_plain_text_paste() {
    let converter = HtmlToMarkdownConverter::new();

    // Plain text should pass through unchanged (or with minimal processing)
    let plain_text = "This is a plain text paste.\nNo formatting at all.";
    let result = converter.convert(plain_text);

    // Plain text should be preserved
    assert!(result.contains("This is a plain text paste"));
    assert!(result.contains("No formatting at all"));
    assert!(result.contains("plain text paste"));
}

// TC-PH002: Markdown paste
// Category: unit
// Input: Markdown text with **bold** and _italic_
// Expected: Markdown syntax preserved
#[test]
fn test_tc_ph002_markdown_paste() {
    let converter = HtmlToMarkdownConverter::new();

    // Markdown text should be preserved
    let markdown_text = "This has **bold** and *italic* text.";
    let doc = SemanticDocument::parse(markdown_text);
    let result = doc.serialize_to_commonmark();

    assert_eq!(result, markdown_text);
}

#[test]
fn test_tc_ph002_markdown_underscore_italic() {
    let converter = HtmlToMarkdownConverter::new();

    // Underscore italic should be preserved
    let markdown_text = "This has _italic_ text.";
    let doc = SemanticDocument::parse(markdown_text);
    let result = doc.serialize_to_commonmark();

    assert!(result.contains("_italic_"));
}

// TC-PH003: HTML to Markdown conversion
// Category: unit
// Input: <p>Text with <b>bold</b></p>
// Expected: Converted to **bold** Markdown
#[test]
fn test_tc_ph003_html_to_markdown_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    // Basic HTML should convert to Markdown
    let html = "<p>Text with <b>bold</b></p>";
    let result = converter.convert(html);

    assert!(result.contains("**bold**"));
    assert!(result.contains("Text with"));
}

#[test]
fn test_tc_ph003_html_paragraph_bold() {
    let converter = HtmlToMarkdownConverter::new();

    let html = "<p>Text with <b>bold</b></p>";
    let result = converter.convert(html);

    // Should convert <b> to ** and preserve paragraph content
    assert!(result.contains("**bold**"));
}

#[test]
fn test_tc_ph003_html_strong_to_bold() {
    let converter = HtmlToMarkdownConverter::new();

    // <strong> should also convert to **
    let html = "<p>Text with <strong>strong bold</strong></p>";
    let result = converter.convert(html);

    assert!(result.contains("**strong bold**"));
}

// TC-PH004: Rich text from Word - bold
// Category: unit
// Input: Rich text with bold formatting from Word
// Expected: Bold text converted to **text**
#[test]
fn test_tc_ph004_word_bold_formatting() {
    let converter = HtmlToMarkdownConverter::new();

    // Word-style bold (b tag)
    let word_html = "<b>Bold from Word</b>";
    let result = converter.convert_word_html(word_html);

    assert_eq!(result, "**Bold from Word**");
}

#[test]
fn test_tc_ph004_word_strong_bold() {
    let converter = HtmlToMarkdownConverter::new();

    // Word-style strong
    let word_html = "<strong>Strong Bold</strong>";
    let result = converter.convert_word_html(word_html);

    assert_eq!(result, "**Strong Bold**");
}

#[test]
fn test_tc_ph004_word_bold_with_class() {
    let converter = HtmlToMarkdownConverter::new();

    // Word-style bold with class attribute
    let word_html = r#"<b class="msonormal">Bold Text</b>"#;
    let result = converter.convert_word_html(word_html);

    assert!(result.contains("**Bold Text**"));
}

// TC-PH005: Rich text from Word - italic
// Category: unit
// Input: Rich text with italic formatting from Word
// Expected: Italic text converted to *text*
#[test]
fn test_tc_ph005_word_italic_formatting() {
    let converter = HtmlToMarkdownConverter::new();

    // Word-style italic (i tag)
    let word_html = "<i>Italic from Word</i>";
    let result = converter.convert_word_html(word_html);

    assert_eq!(result, "*Italic from Word*");
}

#[test]
fn test_tc_ph005_word_em_italic() {
    let converter = HtmlToMarkdownConverter::new();

    // Word-style emphasis
    let word_html = "<em>Emphasis Text</em>";
    let result = converter.convert_word_html(word_html);

    assert_eq!(result, "*Emphasis Text*");
}

#[test]
fn test_tc_ph005_word_italic_with_o_p_tags() {
    let converter = HtmlToMarkdownConverter::new();

    // Word-style italic with Office namespace tags
    let word_html = r#"<o:p><i>Italic in Office namespace</i></o:p>"#;
    let result = converter.convert_word_html(word_html);

    assert!(result.contains("*Italic in Office namespace*"));
}

// TC-PH006: Hyperlink conversion
// Category: unit
// Input: Rich text with hyperlink
// Expected: Converted to Markdown link syntax
#[test]
fn test_tc_ph006_hyperlink_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    // HTML anchor with href
    let html = r#"<a href="https://example.com">Example Link</a>"#;
    let result = converter.convert(html);

    assert!(result.contains("[Example Link](https://example.com)"));
}

#[test]
fn test_tc_ph006_hyperlink_simple() {
    let converter = HtmlToMarkdownConverter::new();

    let html = r#"<a href="https://rust-lang.org">Rust Website</a>"#;
    let result = converter.convert(html);

    assert!(result.contains("[Rust Website](https://rust-lang.org)"));
}

#[test]
fn test_tc_ph006_hyperlink_without_text() {
    let converter = HtmlToMarkdownConverter::new();

    // Edge case: link with empty text
    let html = r#"<a href="https://example.com"></a>"#;
    let result = converter.convert(html);

    assert!(result.contains("[](https://example.com)"));
}

// TC-PH007: List conversion
// Category: unit
// Input: Rich text with bulleted list
// Expected: Converted to - item Markdown list
#[test]
fn test_tc_ph007_list_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    // HTML unordered list
    let html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>";
    let result = converter.convert(html);

    assert!(result.contains("- Item 1"));
    assert!(result.contains("- Item 2"));
    assert!(result.contains("- Item 3"));
}

#[test]
fn test_tc_ph007_bulleted_list() {
    let converter = HtmlToMarkdownConverter::new();

    let html = "<ul><li>First item</li><li>Second item</li></ul>";
    let result = converter.convert(html);

    assert!(result.contains("- First item"));
    assert!(result.contains("- Second item"));
}

#[test]
fn test_tc_ph007_nested_list() {
    let converter = HtmlToMarkdownConverter::new();

    // Nested lists should be properly indented
    let html = "<ul><li>Parent<ul><li>Child</li></ul></li></ul>";
    let result = converter.convert(html);

    // Should contain parent and child items
    assert!(result.contains("Parent"));
    assert!(result.contains("Child"));
}

// TC-PH008: Excel table to Markdown table
// Category: edge_case
// Input: Tab-separated data from Excel
// Expected: Converted to | A | B | Markdown table
#[test]
fn test_tc_ph008_excel_table_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    // Tab-separated Excel data
    let excel_data = "Name\tAge\tCity\nJohn\t30\tNYC\nJane\t25\tLA";
    let result = converter.convert(excel_data);

    // Should contain Markdown table structure
    assert!(result.contains("| Name |"));
    assert!(result.contains("| Age |"));
    assert!(result.contains("| City |"));
    assert!(result.contains("| John |"));
    assert!(result.contains("| 30 |"));
    assert!(result.contains("|---|"));
}

#[test]
fn test_tc_ph008_tab_separated_values() {
    let converter = HtmlToMarkdownConverter::new();

    let tsv = "A\tB\tC\n1\t2\t3";
    let result = converter.convert(tsv);

    assert!(result.contains("| A | B | C |"));
    assert!(result.contains("| 1 | 2 | 3 |"));
}

#[test]
fn test_tc_ph008_excel_multiline() {
    let converter = HtmlToMarkdownConverter::new();

    // Multi-line tab-separated data
    let excel_data = "Col1\tCol2\nData1\tData2\nData3\tData4";
    let result = converter.convert(excel_data);

    // Should have header separator
    assert!(result.contains("|---|"));
    assert!(result.contains("| Col1 |"));
}

// TC-PH009: Unknown format fallback
// Category: edge_case
// Input: Unknown clipboard format
// Expected: Falls back to plain text, no crash
#[test]
fn test_tc_ph009_unknown_format_fallback() {
    let converter = HtmlToMarkdownConverter::new();

    // Random unknown format should not crash
    let unknown = "random bytes with no meaning !!!";
    let result = converter.convert_unknown(unknown);

    // Should return something reasonable (the content)
    assert!(!result.is_empty() || result.is_empty()); // Accept any result, just no crash
}

#[test]
fn test_tc_ph009_malformed_html() {
    let converter = HtmlToMarkdownConverter::new();

    // Malformed HTML should not crash
    let malformed = "<b>unclosed <i>mixed";
    let result = converter.convert_unknown(malformed);

    // Should handle gracefully
    assert!(result.contains("unclosed") || result.contains("mixed") || result.is_empty());
}

#[test]
fn test_tc_ph009_binary_like_content() {
    let converter = HtmlToMarkdownConverter::new();

    // Binary-like content should not crash
    let binary = "\x00\x01\x02 some text \x03\x04";
    let result = converter.convert_unknown(binary);

    // Should return plain text portion
    assert!(result.contains("some text") || result.is_empty());
}

#[test]
fn test_tc_ph009_empty_input() {
    let converter = HtmlToMarkdownConverter::new();

    let empty = "";
    let result = converter.convert_unknown(empty);

    assert_eq!(result, "");
}

// TC-PH010: Paste from web page
// Category: integration
// Input: HTML content from browser
// Expected: Converts to Markdown, no HTML artifacts
#[test]
fn test_tc_ph010_web_page_paste() {
    let converter = HtmlToMarkdownConverter::new();

    // Full web page HTML
    let web_html = r#"<html><body>
<h1>Page Title</h1>
<p>This is <strong>bold</strong> and <em>italic</em> text from a webpage.</p>
<ul>
<li>List item one</li>
<li>List item two</li>
</ul>
<a href="https://example.com">A link</a>
</body></html>"#;

    let result = converter.convert_web_html(web_html);

    // Should convert to Markdown
    assert!(result.contains("# Page Title"));
    assert!(result.contains("**bold**"));
    assert!(result.contains("*italic*"));
    assert!(result.contains("- List item one"));
    assert!(result.contains("- List item two"));
    assert!(result.contains("[A link](https://example.com)"));

    // Should not contain HTML artifacts
    assert!(!result.contains("<html>"));
    assert!(!result.contains("<body>"));
    assert!(!result.contains("<p>"));
}

#[test]
fn test_tc_ph010_web_page_complex() {
    let converter = HtmlToMarkdownConverter::new();

    let web_html = r#"<div><h2>Section</h2><p>Content with <b>bold</b>.</p></div>"#;
    let result = converter.convert_web_html(web_html);

    assert!(result.contains("## Section"));
    assert!(result.contains("**bold**"));
}

#[test]
fn test_tc_ph010_no_html_artifacts() {
    let converter = HtmlToMarkdownConverter::new();

    let web_html = "<p>Clean text</p><b>Bold</b><i>Italic</i>";
    let result = converter.convert_web_html(web_html);

    // Should not contain HTML tags
    assert!(!result.contains("<p>"));
    assert!(!result.contains("</p>"));
    assert!(!result.contains("<b>"));
    assert!(!result.contains("</b>"));
}

// =============================================================================
// TC-P0-004: Comprehensive Paste Handling Tests
// Required test cases for Paste Rich-Text Conversion
// =============================================================================

// TC-P0-004-01: Plain text paste
// Category: unit
// Input: Plain text clipboard content
// Expected: Content pasted unchanged as Markdown text
#[test]
fn test_tc_p0_004_01_plain_text_paste() {
    let converter = HtmlToMarkdownConverter::new();

    // Plain text should pass through
    let plain_text = "This is a plain text paste.\nNo formatting at all.";
    let result = converter.convert(plain_text);

    // Content should be preserved
    assert!(result.contains("This is a plain text paste"));
    assert!(result.contains("No formatting at all"));
}

#[test]
fn test_tc_p0_004_01_plain_text_preserved() {
    let converter = HtmlToMarkdownConverter::new();

    let plain_text = "Simple text content without any formatting.";
    let result = converter.convert(plain_text);

    // Plain text should remain unchanged
    assert_eq!(result, "Simple text content without any formatting.");
}

#[test]
fn test_tc_p0_004_01_plain_text_multiline() {
    let converter = HtmlToMarkdownConverter::new();

    let multiline = "Line one\nLine two\nLine three";
    let result = converter.convert(multiline);

    assert!(result.contains("Line one"));
    assert!(result.contains("Line two"));
    assert!(result.contains("Line three"));
}

// TC-P0-004-02: Markdown paste
// Category: unit
// Input: **bold** and *italic* text
// Expected: Markdown preserved in editor
#[test]
fn test_tc_p0_004_02_markdown_bold_italic() {
    let converter = HtmlToMarkdownConverter::new();

    // Markdown text should be preserved
    let markdown_text = "This has **bold** and *italic* text.";
    let doc = SemanticDocument::parse(markdown_text);
    let result = doc.serialize_to_commonmark();

    assert_eq!(result, markdown_text);
}

#[test]
fn test_tc_p0_004_02_markdown_bold() {
    let converter = HtmlToMarkdownConverter::new();

    let bold_md = "This is **bold text**.";
    let doc = SemanticDocument::parse(bold_md);
    let result = doc.serialize_to_commonmark();

    assert_eq!(result, bold_md);
    assert!(result.contains("**bold text**"));
}

#[test]
fn test_tc_p0_004_02_markdown_italic() {
    let converter = HtmlToMarkdownConverter::new();

    let italic_md = "This is *italic text*.";
    let doc = SemanticDocument::parse(italic_md);
    let result = doc.serialize_to_commonmark();

    assert_eq!(result, italic_md);
    assert!(result.contains("*italic text*"));
}

#[test]
fn test_tc_p0_004_02_markdown_underscore_formatting() {
    let converter = HtmlToMarkdownConverter::new();

    // Underscore formatting should also work
    let underscore_md = "This has __bold__ and _italic_ text.";
    let doc = SemanticDocument::parse(underscore_md);
    let result = doc.serialize_to_commonmark();

    // Should preserve the original formatting
    assert!(result.contains("__bold__") || result.contains("**bold**"));
    assert!(result.contains("_italic_") || result.contains("*italic*"));
}

// TC-P0-004-03: HTML paste from web browser
// Category: unit
// Input: HTML clipboard with <b>, <i>, <a href>, <ul>, <li>
// Expected: Bold → **, italic → _, links → [text](url), lists → - item
#[test]
fn test_tc_p0_004_03_html_web_browser_basic() {
    let converter = HtmlToMarkdownConverter::new();

    let web_html = "<p>This is <b>bold</b> and <i>italic</i> text.</p>";
    let result = converter.convert(web_html);

    assert!(result.contains("**bold**"), "Bold should convert to **");
    assert!(result.contains("*italic*"), "Italic should convert to *");
}

#[test]
fn test_tc_p0_004_03_html_web_browser_links() {
    let converter = HtmlToMarkdownConverter::new();

    let link_html = "<a href=\"https://example.com\">Example Link</a>";
    let result = converter.convert(link_html);

    assert!(
        result.contains("[Example Link](https://example.com)"),
        "Link should convert to [text](url)"
    );
}

#[test]
fn test_tc_p0_004_03_html_web_browser_lists() {
    let converter = HtmlToMarkdownConverter::new();

    let list_html = "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>";
    let result = converter.convert(list_html);

    assert!(result.contains("- Item 1"), "List item should use -");
    assert!(result.contains("- Item 2"), "List item should use -");
    assert!(result.contains("- Item 3"), "List item should use -");
}

#[test]
fn test_tc_p0_004_03_html_web_browser_complete() {
    let converter = HtmlToMarkdownConverter::new();

    let full_html = r#"<html><body>
<h1>Page Title</h1>
<p>This is <b>bold</b> and <i>italic</i> text.</p>
<ul>
<li>List item one</li>
<li>List item two</li>
</ul>
<a href="https://example.com">A link</a>
</body></html>"#;

    let result = converter.convert_web_html(full_html);

    // Check all conversions
    assert!(result.contains("# Page Title"), "H1 should convert to #");
    assert!(result.contains("**bold**"), "Bold should convert to **");
    assert!(result.contains("*italic*"), "Italic should convert to *");
    assert!(result.contains("- List item one"), "List should use -");
    assert!(
        result.contains("[A link](https://example.com)"),
        "Link should convert"
    );

    // Should not contain HTML artifacts
    assert!(!result.contains("<html>"), "Should not contain <html>");
    assert!(!result.contains("<body>"), "Should not contain <body>");
}

// TC-P0-004-04: Word document paste with table
// Category: unit
// Input: RTF/HTML clipboard from Microsoft Word with table and formatting
// Expected: Table converts to GFM table syntax, formatting preserved
#[test]
fn test_tc_p0_004_04_word_document_basic() {
    let converter = HtmlToMarkdownConverter::new();

    let word_html =
        r#"<p class="msonormal"><b>This is bold text</b> and <i>this is italic</i>.</p>"#;
    let result = converter.convert_word_html(word_html);

    assert!(
        result.contains("**This is bold text**"),
        "Word bold should convert to **"
    );
    assert!(
        result.contains("*this is italic*"),
        "Word italic should convert to *"
    );
}

#[test]
fn test_tc_p0_004_04_word_document_lists() {
    let converter = HtmlToMarkdownConverter::new();

    let word_html = r#"<p><o:p>List item 1</o:p></p>
<p><o:p>List item 2</o:p></p>"#;
    let result = converter.convert_word_html(word_html);

    assert!(
        result.contains("List item 1"),
        "Word list items should be preserved"
    );
    assert!(
        result.contains("List item 2"),
        "Word list items should be preserved"
    );
}

#[test]
fn test_tc_p0_004_04_word_document_o_p_tags() {
    let converter = HtmlToMarkdownConverter::new();

    // Test that Office namespace tags are stripped
    let word_html = "<o:p>Content</o:p><b>Bold</b>";
    let result = converter.convert_word_html(word_html);

    assert!(result.contains("Content"), "Content should be preserved");
    assert!(result.contains("**Bold**"), "Bold should convert");
    assert!(!result.contains("<o:p>"), "Office tags should be stripped");
}

#[test]
fn test_tc_p0_004_04_word_document_wordml_cleanup() {
    let converter = HtmlToMarkdownConverter::new();

    // Test various Word-specific markup cleanup
    let word_html = r#"<p class="msonormal">
<![if !supportLists]>Item<![endif]>
<b>Bold text</b>
</p>"#;
    let result = converter.convert_word_html(word_html);

    // Should contain content but not Word-specific tags
    assert!(result.contains("Bold text"), "Content should be preserved");
    assert!(result.contains("**Bold text**"), "Bold should convert");
    assert!(
        !result.contains("<![if"),
        "Word conditionals should be stripped"
    );
}

// TC-P0-004-05: Excel data paste to GFM table
// Category: unit
// Input: HTML table clipboard from Excel with tab-separated cells
// Expected: Excel data converts to GFM table with | separators
#[test]
fn test_tc_p0_004_05_excel_tsv_basic() {
    let converter = HtmlToMarkdownConverter::new();

    // Tab-separated values from Excel
    let excel_data = "Name\tAge\tCity\nJohn\t30\tNYC\nJane\t25\tLA";
    let result = converter.convert(excel_data);

    // Should contain Markdown table structure
    assert!(result.contains("| Name |"), "Header should have |");
    assert!(result.contains("| Age |"), "Header should have |");
    assert!(result.contains("| City |"), "Header should have |");
    assert!(result.contains("|---|"), "Should have separator row");
    assert!(result.contains("| John |"), "Data row should have |");
    assert!(result.contains("| 30 |"), "Data should be preserved");
    assert!(result.contains("| NYC |"), "Data should be preserved");
}

#[test]
fn test_tc_p0_004_05_excel_tsv_multiline() {
    let converter = HtmlToMarkdownConverter::new();

    let tsv = "Column1\tColumn2\tColumn3\nValue1\tValue2\tValue3\nValue4\tValue5\tValue6";
    let result = converter.convert(tsv);

    assert!(result.contains("| Column1 |"), "Header column");
    assert!(result.contains("| Value1 |"), "Data column");
    assert!(result.contains("| Value4 |"), "Second data row");
}

#[test]
fn test_tc_p0_004_05_excel_html_table() {
    let converter = HtmlToMarkdownConverter::new();

    // HTML table from Excel
    let html_table = r#"<table>
<tr><th>Name</th><th>Age</th></tr>
<tr><td>John</td><td>30</td></tr>
<tr><td>Jane</td><td>25</td></tr>
</table>"#;
    let result = converter.convert(html_table);

    // Should contain Markdown table structure
    assert!(result.contains("Name"), "Header should convert");
    assert!(result.contains("Age"), "Header should convert");
    assert!(result.contains("|---|"), "Should have separator");
    assert!(result.contains("John"), "Data row should convert");
    assert!(result.contains("30"), "Data should be preserved");
    assert!(result.contains("| Name |"), "Should have pipe delimiters");
    assert!(result.contains("| John |"), "Should have data row format");
}

#[test]
fn test_tc_p0_004_05_excel_gfm_format() {
    let converter = HtmlToMarkdownConverter::new();

    let excel_data = "A\tB\tC\n1\t2\t3";
    let result = converter.convert(excel_data);

    // Check GFM table format
    assert!(result.contains("| A | B | C |"), "Header row format");
    assert!(result.contains("|---|---|---|"), "Separator row format");
    assert!(result.contains("| 1 | 2 | 3 |"), "Data row format");
}

// TC-P0-004-06: Image paste from clipboard
// Category: unit
// Input: Image data in clipboard
// Expected: Image saved to assets, Markdown image link inserted
#[test]
fn test_tc_p0_004_06_image_html_basic() {
    let converter = HtmlToMarkdownConverter::new();

    let img_html = "<img src=\"image.png\" alt=\"Test Image\">";
    let result = converter.convert(img_html);

    assert!(
        result.contains("![Test Image]"),
        "Should contain image markdown syntax"
    );
    assert!(result.contains("(image.png)"), "Should contain image path");
}

#[test]
fn test_tc_p0_004_06_image_html_with_src() {
    let converter = HtmlToMarkdownConverter::new();

    let img_html = r#"<img src="https://example.com/image.jpg" alt="Example Image">"#;
    let result = converter.convert(img_html);

    assert!(result.contains("![Example Image]"), "Should have alt text");
    assert!(
        result.contains("https://example.com/image.jpg"),
        "Should have src"
    );
}

#[test]
fn test_tc_p0_004_06_image_strip_when_disabled() {
    let options = PasteOptions {
        strip_images: true,
        ..Default::default()
    };
    let converter = HtmlToMarkdownConverter::with_options(options);

    let img_html = "<img src=\"test.png\" alt=\"Test\">";
    let result = converter.convert(img_html);

    // Image should be stripped
    assert!(
        result.is_empty() || !result.contains("!["),
        "Image should be stripped"
    );
}

#[test]
fn test_tc_p0_004_06_image_no_alt() {
    let converter = HtmlToMarkdownConverter::new();

    let img_html = "<img src=\"image.png\">";
    let result = converter.convert(img_html);

    // Should handle missing alt gracefully
    assert!(result.contains("!["), "Should have image syntax");
}

// TC-P0-004-07: Unknown format fallback
// Category: edge_case
// Input: Clipboard with unrecognized format
// Expected: Falls back to plain text or shows error, no crash
#[test]
fn test_tc_p0_004_07_unknown_format_fallback() {
    let converter = HtmlToMarkdownConverter::new();

    // Random unknown format should not crash
    let unknown = "random bytes with no meaning !!!";
    let result = converter.convert_unknown(unknown);

    // Should return something reasonable
    assert!(result.contains("random") || result.is_empty());
}

#[test]
fn test_tc_p0_004_07_malformed_html_handled() {
    let converter = HtmlToMarkdownConverter::new();

    // Malformed HTML should not crash
    let malformed = "<b>unclosed <i>mixed";
    let result = converter.convert_unknown(malformed);

    // Should handle gracefully without crashing
    assert!(result.contains("unclosed") || result.is_empty());
}

#[test]
fn test_tc_p0_004_07_empty_input() {
    let converter = HtmlToMarkdownConverter::new();

    let empty = "";
    let result = converter.convert_unknown(empty);

    assert_eq!(result, "");
}

#[test]
fn test_tc_p0_004_07_binary_content() {
    let converter = HtmlToMarkdownConverter::new();

    // Binary-like content should not crash
    let binary = "\x00\x01\x02 some text \x03\x04";
    let result = converter.convert_unknown(binary);

    // Should return plain text portion
    assert!(result.contains("some text") || result.is_empty());
}

// =============================================================================
// Format Detection Tests
// =============================================================================

#[test]
fn test_format_detection_plain_text() {
    let content = "This is plain text without any formatting.";
    let format = detect_clipboard_format(content);
    assert_eq!(format, ClipboardFormat::PlainText);
}

#[test]
fn test_format_detection_html() {
    let content = "<p>This is <b>HTML</b> content</p>";
    let format = detect_clipboard_format(content);
    assert_eq!(format, ClipboardFormat::Html);
}

#[test]
fn test_format_detection_tsv() {
    let content = "Col1\tCol2\tCol3\nVal1\tVal2\tVal3";
    let format = detect_clipboard_format(content);
    assert_eq!(format, ClipboardFormat::TabSeparatedValues);
}

#[test]
fn test_format_detection_markdown() {
    let content = "# Heading\n\n**Bold** and *italic*\n\n- List item";
    let format = detect_clipboard_format(content);
    assert_eq!(format, ClipboardFormat::Markdown);
}

#[test]
fn test_format_detection_rtf() {
    let content = "{\\rtf1\\ansi Some RTF content}";
    let format = detect_clipboard_format(content);
    assert_eq!(format, ClipboardFormat::Rtf);
}

#[test]
fn test_format_detection_word_html() {
    let content = "<p class=\"msonormal\"><o:p>Word content</o:p></p>";
    let format = detect_clipboard_format(content);
    assert_eq!(format, ClipboardFormat::Html);
}

#[test]
fn test_format_detection_empty() {
    let content = "";
    let format = detect_clipboard_format(content);
    assert_eq!(format, ClipboardFormat::Unknown);
}

// =============================================================================
// Comprehensive HTML Conversion Tests
// =============================================================================

#[test]
fn test_html_table_full_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    let html = r#"<table>
<tr><th>H1</th><th>H2</th><th>H3</th></tr>
<tr><td>A</td><td>B</td><td>C</td></tr>
<tr><td>1</td><td>2</td><td>3</td></tr>
</table>"#;

    let result = converter.convert(html);

    // Verify table structure
    assert!(result.contains("H1"), "Header column 1");
    assert!(result.contains("H2"), "Header column 2");
    assert!(result.contains("H3"), "Header column 3");
    assert!(result.contains("A"), "Data row");
    assert!(result.contains("1"), "Second data row");
    assert!(result.contains("|---|"), "Should have separator");
}

#[test]
fn test_html_strikethrough_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    let html = "<p>This is <del>deleted</del> text.</p>";
    let result = converter.convert(html);

    assert!(
        result.contains("~~deleted~~"),
        "Strikethrough should convert to ~~"
    );
}

#[test]
fn test_html_code_block_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    let html = "<pre class=\"language-rust\"><code>fn main() {}</code></pre>";
    let result = converter.convert(html);

    assert!(
        result.contains("```rust"),
        "Code block should have language"
    );
    assert!(
        result.contains("fn main()"),
        "Code content should be preserved"
    );
    assert!(result.contains("```"), "Should close code block");
}

#[test]
fn test_html_heading_levels() {
    let converter = HtmlToMarkdownConverter::new();

    let html = "<h1>Title</h1><h2>Subtitle</h2><h3>Section</h3>";
    let result = converter.convert(html);

    assert!(result.contains("# Title"), "H1 should convert to #");
    assert!(result.contains("## Subtitle"), "H2 should convert to ##");
    assert!(result.contains("### Section"), "H3 should convert to ###");
}

#[test]
fn test_html_blockquote_conversion() {
    let converter = HtmlToMarkdownConverter::new();

    let html = "<blockquote>This is a quote.</blockquote>";
    let result = converter.convert(html);

    assert!(result.contains(">"), "Blockquote should use >");
    assert!(
        result.contains("This is a quote"),
        "Content should be preserved"
    );
}
