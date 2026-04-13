use rustnote_lib::semantic::ast::SemanticDocument;
use rustnote_lib::semantic::frontmatter::{FrontmatterError, YamlFrontmatter};

/// TC-G012-001: Frontmatter_basic
/// Test basic frontmatter with simple key-value pairs
#[test]
fn test_frontmatter_basic() {
    let source = r#"---
title: Hello
---

"#;
    let doc = SemanticDocument::parse(source);
    assert!(doc.has_frontmatter(), "Document should have frontmatter");
    assert_eq!(doc.get_frontmatter(), Some("title: Hello"));

    let parsed = doc.parse_frontmatter_yaml().unwrap();
    assert_eq!(parsed.title, Some("Hello".to_string()));
    assert_eq!(parsed.author, None);
    assert_eq!(parsed.tags, Vec::<String>::new());
}

/// TC-G012-002: Frontmatter_complex_yaml
/// Test complex YAML frontmatter with nested objects and arrays
#[test]
fn test_frontmatter_complex_yaml() {
    let source = r#"---
title: Hello
tags: [a, b]
metadata:
  author: John
---
"#;
    let doc = SemanticDocument::parse(source);
    assert!(doc.has_frontmatter(), "Document should have frontmatter");

    let parsed = doc.parse_frontmatter_yaml().unwrap();
    assert_eq!(parsed.title, Some("Hello".to_string()));
    assert_eq!(parsed.tags, vec!["a".to_string(), "b".to_string()]);

    let metadata = parsed.get_nested("metadata").unwrap();
    assert_eq!(metadata.get("author").unwrap().as_str().unwrap(), "John");
}

/// TC-G012-003: Frontmatter_invalid_yaml
/// Test graceful handling of invalid YAML - document should still load
#[test]
fn test_frontmatter_invalid_yaml() {
    let source = r#"---
title: [unclosed
---

# Content
"#;
    let doc = SemanticDocument::parse(source);
    assert!(
        doc.has_frontmatter(),
        "Document should still have frontmatter"
    );

    let result = doc.parse_frontmatter_yaml();
    assert!(result.is_err(), "Should return error for invalid YAML");
    assert!(matches!(
        result.unwrap_err(),
        FrontmatterError::InvalidYaml(_)
    ));

    assert!(doc.html().contains("<h1>"), "Document should still render");
}

/// TC-G012-004: Frontmatter_missing
/// Test document without frontmatter loads correctly
#[test]
fn test_frontmatter_missing() {
    let source = "Just paragraph";
    let doc = SemanticDocument::parse(source);
    assert!(
        !doc.has_frontmatter(),
        "Document should not have frontmatter"
    );
    assert_eq!(doc.get_frontmatter(), None);

    let result = doc.parse_frontmatter_yaml();
    assert!(result.is_err(), "Should error when no frontmatter");
    assert!(matches!(
        result.unwrap_err(),
        FrontmatterError::InvalidYaml(_)
    ));

    assert!(
        doc.html().contains("Just paragraph"),
        "Content should be preserved"
    );
}

/// Additional test: complex nested YAML with multiple levels
#[test]
fn test_frontmatter_deeply_nested_yaml() {
    let source = r#"---
title: Deep YAML Test
categories:
  - name: Programming
    subcategories:
      - Rust
      - TypeScript
config:
  debug: true
  max_items: 100
---
"#;
    let doc = SemanticDocument::parse(source);
    let parsed = doc.parse_frontmatter_yaml().unwrap();
    assert_eq!(parsed.title, Some("Deep YAML Test".to_string()));

    let categories = parsed.get_nested("categories").unwrap();
    assert!(categories.is_sequence());

    let config = parsed.get_nested("config").unwrap();
    assert_eq!(config.get("debug").unwrap().as_bool().unwrap(), true);
    assert_eq!(config.get("max_items").unwrap().as_i64().unwrap(), 100);
}

/// Additional test: frontmatter with empty values
#[test]
fn test_frontmatter_empty_values() {
    let source = r#"---
title: ""
tags: []
---
"#;
    let doc = SemanticDocument::parse(source);
    let parsed = doc.parse_frontmatter_yaml().unwrap();
    assert_eq!(parsed.title, Some("".to_string()));
    assert_eq!(parsed.tags, Vec::<String>::new());
}

/// Additional test: invalid YAML but document content still accessible
#[test]
fn test_frontmatter_invalid_yaml_content_accessible() {
    let source = r#"---
invalid: [unclosed
---
# Real Content

This should still be accessible.
"#;
    let doc = SemanticDocument::parse(source);
    assert!(doc.has_frontmatter());

    let headings = doc.get_headings();
    assert_eq!(headings.len(), 1);
    assert_eq!(headings[0].text, "Real Content");

    let paragraphs = doc.get_paragraphs();
    assert!(paragraphs.len() >= 1);
}

/// Test roundtrip: frontmatter serialization
#[test]
fn test_frontmatter_roundtrip() {
    let source = r#"---
title: Roundtrip Test
author: Tester
tags:
  - tag1
  - tag2
---
"#;
    let doc = SemanticDocument::parse(source);
    let output = doc.serialize_with_frontmatter();
    assert!(output.starts_with("---\n"));
    assert!(output.contains("title: Roundtrip Test"));
    assert!(output.contains("author: Tester"));
    assert!(output.contains("tags:"));
    assert!(output.contains("- tag1"));
    assert!(output.contains("- tag2"));
}

/// Test: frontmatter with special characters
#[test]
fn test_frontmatter_special_characters() {
    let source = r#"---
title: "Test: With : Colons"
description: "Line with\nNewline"
---
"#;
    let doc = SemanticDocument::parse(source);
    let parsed = doc.parse_frontmatter_yaml().unwrap();
    assert_eq!(parsed.title, Some("Test: With : Colons".to_string()));
    assert_eq!(parsed.get_string("description"), Some("Line with\nNewline"));
}
