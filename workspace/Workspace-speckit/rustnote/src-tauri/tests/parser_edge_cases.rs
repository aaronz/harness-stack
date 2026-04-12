use rustnote_lib::semantic::ast::SemanticDocument;

/// FR-T003: Malformed Input Tests
/// Test parser resilience to malformed markdown input

#[test]
fn test_unclosed_code_fence() {
    let source = "```rust\nfn main() {\n    println!(\"Hello\");\n";
    let doc = SemanticDocument::parse(source);
    // Should still parse and serialize
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("```rust"));
}

#[test]
fn test_unclosed_blockquote() {
    let source = "> quote without closing\nparagraph";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("> quote"));
}

#[test]
fn test_mismatched_emphasis() {
    let source = "**bold text*"; // mismatched - opened with ** but closed with *
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    // Should not panic, should serialize
    let _ = doc.serialize_to_commonmark();
}

#[test]
fn test_mismatched_link_syntax() {
    let source = "[link text]"; // missing URL
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_incomplete_link_with_url_only() {
    let source = "(https://example.com)"; // missing link text
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_incomplete_image_syntax() {
    let source = "![alt text]"; // missing URL
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert_eq!(output, source);
}

#[test]
fn test_task_list_without_space() {
    let source = "-[ ] task"; // missing space after hyphen
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    // Should still parse, space might be inserted
    let _ = doc.serialize_to_commonmark();
}

#[test]
fn test_invalid_table_syntax() {
    let source = "| Header | Header |\n|--------|\n| Cell |"; // missing cell in last row
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    let _ = doc.serialize_to_commonmark(); // should not panic
}

#[test]
fn test_ordered_list_with_letters() {
    let source = "a. item\nb. item\nc. item"; // invalid - should be numbers
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    // Should parse as plain text, not as ordered list
}

#[test]
fn test_nested_blockquotes_incorrectly() {
    let source = "> outer\n>> inner\n> back to outer";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("> outer"));
}

#[test]
fn test_heading_without_space_after_hash() {
    let source = "#NoSpace"; // missing space after #
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    // Should parse as regular text or heading depending on implementation
}

#[test]
fn test_multiple_blank_lines_in_heading() {
    let source = "# Heading\n\n\n\nContent";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("# Heading"));
}

#[test]
fn test_empty_list_items() {
    let source = "- \n- \n-";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("- "));
}

#[test]
fn test_frontmatter_with_missing_closing() {
    let source = "---\ntitle: Test\nauthor: Test\n\nContent without proper closing";
    let doc = SemanticDocument::parse(source);
    // Should handle gracefully
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("title: Test"));
}

#[test]
fn test_html_in_markdown() {
    let source = "<div>HTML content</div>\n\nParagraph";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("<div>"));
}

#[test]
fn test_unicode_in_markdown() {
    let source = "# 标题\n\n内容 with émojis: 🎉 👍";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("🎉"));
}

#[test]
fn test_control_characters() {
    let source = "Text with unicode: \u{0}\u{1}\u{2}\u{3}";
    let doc = SemanticDocument::parse(source);
    let _ = doc.serialize_to_commonmark();
}

/// FR-T004: Deep Nesting Tests
/// Test parser with deeply nested structures

#[test]
fn test_deeply_nested_unordered_lists() {
    let source = "- level 1\n  - level 2\n    - level 3\n      - level 4\n        - level 5\n          - level 6\n            - level 7\n              - level 8";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("- level 1"));
    assert!(output.contains("level 8"));
}

#[test]
fn test_deeply_nested_ordered_lists() {
    let source =
        "1. level 1\n   1. level 2\n      1. level 3\n         1. level 4\n            1. level 5";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("1. level 1"));
}

#[test]
fn test_deeply_nested_blockquotes() {
    let source = "> level 1\n> > level 2\n> > > level 3\n> > > > level 4\n> > > > > level 5";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("> level 1"));
    assert!(output.contains("level 5"));
}

#[test]
fn test_mixed_deep_nesting() {
    let source = "- List item\n  > Blockquote in list\n    - Nested list item\n      > Deep nested blockquote\n        - Even deeper";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("- List item"));
    assert!(output.contains("> Blockquote"));
}

#[test]
fn test_many_sibling_elements() {
    let mut source = String::new();
    for i in 0..100 {
        source.push_str(&format!("# Heading {}\n\nParagraph {}\n\n", i, i));
    }
    let doc = SemanticDocument::parse(&source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("Heading 0"));
    assert!(output.contains("Heading 99"));
}

#[test]
fn test_deep_code_fence_nesting() {
    let source = "````\nouter\n`````\ninner\n````\n```";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    // Should handle backtick count variations
    let _ = doc.serialize_to_commonmark();
}

#[test]
fn test_wide_tables() {
    let source = "| A | B | C | D | E | F | G | H | I | J |\n|---|---|---|---|---|---|---|---|---|---|\n| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("| A |"));
}

#[test]
fn test_long_code_block() {
    let mut source = "```rust\n".to_string();
    for i in 0..1000 {
        source.push_str(&format!("fn function_{}() {{}}\n", i));
    }
    source.push_str("```");
    let doc = SemanticDocument::parse(&source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("function_0"));
    assert!(output.contains("function_999"));
}

#[test]
fn test_wide_headings() {
    let source = "# This is a very long heading that goes on and on and on and on and on and on and on and on and on and on";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("#"));
}

#[test]
fn test_long_inline_code() {
    let source = "`this is a very long inline code block that contains a lot of text and should still be recognized as inline code`";
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_to_commonmark();
    assert!(output.contains("`this is a very long inline"));
}
