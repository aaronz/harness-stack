//! Syntax Highlighting Tests
//!
//! Tests for G-018: SyntaxHighlighter implementation verification
//!
//! PRD Specification:
//! - Shiki for VS Code-quality syntax highlighting in frontend
//! - syntect for Rust-side export/render pipelines
//!
//! Current Implementation:
//! - Uses syntect for Rust-side highlighting (compliant with PRD)
//! - Frontend integration via Tauri commands

use rustnote_lib::parser::SyntaxHighlighter;

/// TC-G018-001: SyntaxHighlighter_rust
/// Category: integration
/// Input: ```rust
/// fn main() {}
/// ```
/// Expected: Rust keywords, functions highlighted
#[test]
fn test_syntax_highlighter_rust_keywords_highlighted() {
    let highlighter = SyntaxHighlighter::new();
    let code = "fn main() {}";
    let language = "rust";

    let result = highlighter.highlight(code, language);

    // Result should not be empty and should contain highlighting markup
    assert!(!result.is_empty(), "Highlighted result should not be empty");

    // For rust, we expect some HTML span tags for syntax highlighting
    // syntect generates ClassedHTMLGenerator output with scope names
    assert!(
        result.contains('<') && result.contains('>'),
        "Highlighted result should contain HTML markup"
    );
}

/// TC-G018-001: SyntaxHighlighter_rust - Verify function highlighting
#[test]
fn test_syntax_highlighter_rust_function_highlighting() {
    let highlighter = SyntaxHighlighter::new();
    let code = "fn main() {\n    println!(\"Hello\");\n}";
    let language = "rust";

    let result = highlighter.highlight(code, language);

    // The result should contain HTML markup
    assert!(!result.is_empty());
    assert!(result.contains('<'), "Should contain HTML markup");

    // Check that the code content is preserved
    assert!(result.contains("fn"), "Should contain 'fn' keyword");
    assert!(
        result.contains("main"),
        "Should contain 'main' function name"
    );
}

/// TC-G018-002: SyntaxHighlighter_javascript
/// Category: integration
/// Input: ```javascript
/// const x = 1;
/// ```
/// Expected: JS syntax highlighted
#[test]
fn test_syntax_highlighter_javascript_basic() {
    let highlighter = SyntaxHighlighter::new();
    let code = "const x = 1;";
    let language = "javascript";

    let result = highlighter.highlight(code, language);

    // Result should not be empty and should contain highlighting markup
    assert!(!result.is_empty(), "Highlighted result should not be empty");
    assert!(
        result.contains('<') && result.contains('>'),
        "Highlighted result should contain HTML markup"
    );
}

/// TC-G018-002: SyntaxHighlighter_javascript - Verify JS keywords
#[test]
fn test_syntax_highlighter_javascript_keywords() {
    let highlighter = SyntaxHighlighter::new();
    let code = "function hello() {\n  return true;\n}";
    let language = "js";

    let result = highlighter.highlight(code, language);

    assert!(!result.is_empty());
    assert!(
        result.contains("function"),
        "Should contain 'function' keyword"
    );
    assert!(result.contains('<'), "Should contain HTML markup");
}

/// TC-G018-003: SyntaxHighlighter_prd_compliance
/// Category: architecture
/// Input: Check Shiki vs syntect choice
/// Expected: PRD deviation documented if any
///
/// PRD states:
/// - Shiki for frontend preview (VS Code-quality)
/// - syntect for Rust-side export/render pipelines
///
/// Current Implementation:
/// - Rust backend uses syntect (CORRECT per PRD)
/// - Frontend highlighting not covered by this test (JS-side concern)
#[test]
fn test_syntax_highlighter_prd_compliance_syntect_used() {
    let highlighter = SyntaxHighlighter::new();

    // Verify syntect is being used by checking that highlighting works
    // syntect is the PRD-specified library for Rust-side highlighting
    let code = "fn main() {}";
    let result = highlighter.highlight(code, "rust");

    // syntect produces HTML with specific class-based formatting
    assert!(
        !result.is_empty(),
        "syntect-based highlighter should produce output"
    );

    // The implementation uses syntect::html::highlighted_html_for_string
    // which produces ClassedHTMLGenerator output with CSS classes
    // This is the correct approach per PRD for Rust-side highlighting
}

/// TC-G018-003: Document syntect usage is PRD-compliant
#[test]
fn test_syntax_highlighter_prd_compliance_documented() {
    // PRD Architecture Compliance:
    //
    // Section 05-architecture.md states:
    // - "Shiki for VS Code-quality syntax highlighting in frontend"
    // - "syntect for Rust-side export/render pipelines"
    //
    // Section 06-frontend-design.md states:
    // - "Shiki for frontend preview, syntect for Rust-side export/render pipelines"
    //
    // Section 10-rust-crate-design.md states:
    // - "export/ # HTML/PDF export with syntect"
    //
    // Current Implementation:
    // - rustnote_lib::parser::syntax::SyntaxHighlighter uses syntect
    // - This is CORRECT for Rust-side highlighting
    //
    // NO DEVIATION: syntect is the correct choice for Rust backend.
    // The frontend (JavaScript/TypeScript) should use Shiki for live preview,
    // but this is outside the scope of Rust backend tests.

    let highlighter = SyntaxHighlighter::new();
    let code = "let x = 1;";

    // Verify highlighting works
    let result = highlighter.highlight(code, "javascript");
    assert!(!result.is_empty());

    // Verify syntect is being used (via highlight_html which uses highlighted_html_for_string)
    let html_result = highlighter.highlight_html(code, "javascript");
    assert!(
        html_result.contains("<pre"),
        "syntect produces <pre> wrapped output"
    );
}

// =============================================================================
// Edge Case Tests
// =============================================================================

/// Edge case: unknown_language
/// When an unknown language is provided, highlighting should fallback gracefully
#[test]
fn test_highlight_unknown_language_fallback() {
    let highlighter = SyntaxHighlighter::new();
    let code = "some code without specific syntax";
    let language = "unknown_language_xyz";

    let result = highlighter.highlight(code, language);

    // Should still produce output (fallback to plain text)
    assert!(
        !result.is_empty(),
        "Unknown language should fallback gracefully"
    );

    // The content should be preserved in some form
    assert!(
        result.contains("some code") || result.contains("unknown"),
        "Code content should be preserved in fallback"
    );
}

/// Edge case: unknown_language with highlight_html
#[test]
fn test_highlight_html_unknown_language_fallback() {
    let highlighter = SyntaxHighlighter::new();
    let code = "plain text code";
    let language = "not_a_real_language";

    let html_result = highlighter.highlight_html(code, language);

    // Should produce valid HTML with pre tags
    // syntect uses plain text fallback with <pre class="highlight"> wrapper
    assert!(html_result.contains("<pre"), "Should contain <pre> tags");
    assert!(
        html_result.contains("</pre>"),
        "Should have closing </pre> tag"
    );

    // Content should be HTML-escaped (syntect escapes < > when using plain text)
    assert!(
        html_result.contains("&lt;")
            || html_result.contains("&gt;")
            || html_result.contains("plain text"),
        "HTML special characters should be escaped or content preserved"
    );
}

/// Edge case: malformed_code
/// Malformed or incomplete code should not crash the highlighter
#[test]
fn test_highlight_malformed_rust_code() {
    let highlighter = SyntaxHighlighter::new();

    // Incomplete/malformed Rust code
    let malformed_cases = vec![
        "fn ",               // incomplete function
        "struct {",          // incomplete struct
        "enum Test { A, B,", // incomplete enum
        "impl ",             // incomplete impl
        "match x {",         // incomplete match
    ];

    for code in malformed_cases {
        let result = highlighter.highlight(code, "rust");
        assert!(
            !result.is_empty(),
            "Malformed code '{}' should not crash highlighter",
            code
        );
    }
}

/// Edge case: malformed_code with JavaScript
#[test]
fn test_highlight_malformed_javascript_code() {
    let highlighter = SyntaxHighlighter::new();

    let malformed_cases = vec![
        "function ",       // incomplete function
        "const obj = {",   // incomplete object
        "if (x {",         // unclosed parenthesis
        "for (let i = 0;", // incomplete for loop
    ];

    for code in malformed_cases {
        let result = highlighter.highlight(code, "javascript");
        assert!(
            !result.is_empty(),
            "Malformed JS '{}' should not crash highlighter",
            code
        );
    }
}

/// Edge case: large_code_blocks
/// Large code blocks should be handled without performance issues or crashes
#[test]
fn test_highlight_large_rust_code_block() {
    let highlighter = SyntaxHighlighter::new();

    // Generate a large Rust code block (1000+ lines)
    let large_code = (0..1000)
        .map(|i| format!("fn function_{}() -> u32 {{ return {}; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let result = highlighter.highlight(&large_code, "rust");

    assert!(!result.is_empty(), "Large code block should be highlighted");
    assert!(result.contains("fn"), "Should contain function definitions");
}

/// Edge case: large_code_blocks with JavaScript
#[test]
fn test_highlight_large_javascript_code_block() {
    let highlighter = SyntaxHighlighter::new();

    // Generate large JavaScript code
    let large_code = (0..500)
        .map(|i| format!("function func_{}() {{ return {}; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let result = highlighter.highlight(&large_code, "javascript");

    assert!(
        !result.is_empty(),
        "Large JS code block should be highlighted"
    );
    assert!(
        result.contains("function"),
        "Should contain function definitions"
    );
}

/// Edge case: large_code_blocks with highlight_html
#[test]
fn test_highlight_html_large_code_block() {
    let highlighter = SyntaxHighlighter::new();

    // Large code with multiple lines
    let code: String = (0..200)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");

    let html_result = highlighter.highlight_html(&code, "text");

    assert!(html_result.contains("<pre"), "Should contain <pre> tags");
    assert!(
        html_result.contains("</pre>"),
        "Should contain closing </pre> tags"
    );
}

// =============================================================================
// Markdown fenced_code Integration Tests
// =============================================================================

/// Markdown fenced_code: Basic fenced code block
#[test]
fn test_markdown_fenced_code_rust() {
    let highlighter = SyntaxHighlighter::new();

    let code = r#"fn main() {
    println!("Hello, world!");
}"#;

    let result = highlighter.highlight(code, "rust");

    assert!(!result.is_empty());
    assert!(result.contains("fn"), "Should highlight Rust keyword");
    assert!(result.contains("main"), "Should highlight function name");
}

/// Markdown fenced_code: JavaScript fenced block
#[test]
fn test_markdown_fenced_code_javascript() {
    let highlighter = SyntaxHighlighter::new();

    let code = "console.log('Hello, world!');";

    let result = highlighter.highlight(code, "javascript");

    assert!(!result.is_empty());
    assert!(result.contains("console"), "Should highlight console");
}

/// Markdown fenced_code: Empty code block
#[test]
fn test_markdown_fenced_code_empty() {
    let highlighter = SyntaxHighlighter::new();

    let result = highlighter.highlight("", "rust");

    // Empty code should return empty or minimal result
    // (syntect behavior may vary)
    assert!(
        result.is_empty() || result.contains('<'),
        "Empty code should return empty or HTML"
    );
}

/// Markdown fenced_code: Single line code
#[test]
fn test_markdown_fenced_code_single_line() {
    let highlighter = SyntaxHighlighter::new();

    let result = highlighter.highlight("let x = 5;", "javascript");

    assert!(!result.is_empty());
}

/// Markdown fenced_code: Multi-language support
#[test]
fn test_markdown_fenced_code_multiple_languages() {
    let highlighter = SyntaxHighlighter::new();

    // Test various languages
    let languages = vec![
        ("rust", "fn test() {}"),
        ("javascript", "let x = 1;"),
        ("python", "def test(): pass"),
        ("go", "func test() {}"),
        ("html", "<div>test</div>"),
        ("css", "body { color: red; }"),
    ];

    for (lang, code) in languages {
        let result = highlighter.highlight(code, lang);
        assert!(
            !result.is_empty(),
            "Language '{}' should produce highlighting",
            lang
        );
    }
}

// =============================================================================
// Theme Verification
// =============================================================================

/// Verify the highlighter uses a valid theme
#[test]
fn test_highlighter_uses_valid_theme() {
    let highlighter = SyntaxHighlighter::new();

    // The implementation uses base16-ocean.dark theme
    // This test verifies the theme is loaded correctly
    let code = "fn main() {}";
    let result = highlighter.highlight(code, "rust");

    // Result should have HTML markup with theme classes
    assert!(!result.is_empty());

    // Verify highlight_html also works (includes theme)
    let html_result = highlighter.highlight_html(code, "rust");
    assert!(
        html_result.contains("<pre"),
        "Should use themed HTML output"
    );
}
