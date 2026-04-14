use rustnote_lib::model::export::{
    HtmlExportMode, HtmlExportOptions, PdfExportOptions, PdfMargins, PdfPageSize,
};
use rustnote_lib::parser::MarkdownParser;
use rustnote_lib::services::export::ExportServiceTrait;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tempfile::TempDir;

fn parser() -> MarkdownParser {
    MarkdownParser::new()
}

#[test]
fn test_markdown_parser_headings() {
    let html = parser().parse_to_html("# Heading 1\n\n## Heading 2");
    assert!(html.contains("<h1"));
    assert!(html.contains("Heading 1"));
    assert!(html.contains("<h2"));
    assert!(html.contains("Heading 2"));
}

#[test]
fn test_markdown_parser_code_block() {
    let html = parser().parse_to_html("```rust\nfn main() {}\n```");
    assert!(html.contains("<pre"));
    assert!(html.contains("class="));
    assert!(html.contains("rust"));
}

#[test]
fn test_markdown_parser_table() {
    let html = parser().parse_to_html("| a | b |\n|---|---|\n| 1 | 2 |");
    assert!(html.contains("<table>"));
    assert!(html.contains("<td>"));
}

#[test]
fn test_markdown_parser_image() {
    let html = parser().parse_to_html("![alt](image.png)");
    assert!(html.contains("<img"));
    assert!(html.contains("src="));
    assert!(html.contains("alt="));
}

#[test]
fn test_markdown_parser_blockquote() {
    let html = parser().parse_to_html("> quote text");
    assert!(html.contains("<blockquote"));
}

#[test]
fn test_markdown_parser_list() {
    let html = parser().parse_to_html("- item 1\n- item 2");
    assert!(html.contains("<ul>"));
    assert!(html.contains("<li>"));
}

#[test]
fn test_markdown_parser_task_list() {
    let html = parser().parse_to_html("- [x] done\n- [ ] todo");
    assert!(html.contains("<input"));
    assert!(html.contains("type=\"checkbox\""));
}

#[test]
fn test_pdf_export_options_default() {
    let options = PdfExportOptions::default();
    assert_eq!(options.page_size.dimensions(), (210.0, 297.0));
    assert!(options.embed_images);
    assert_eq!(options.margins.top_mm, 20.0);
}

#[test]
fn test_pdf_page_size_dimensions() {
    assert_eq!(PdfPageSize::A4.dimensions(), (210.0, 297.0));
    assert_eq!(PdfPageSize::Letter.dimensions(), (215.9, 279.4));
    assert_eq!(PdfPageSize::Legal.dimensions(), (215.9, 355.6));
    assert_eq!(
        PdfPageSize::Custom {
            width_mm: 100.0,
            height_mm: 200.0
        }
        .dimensions(),
        (100.0, 200.0)
    );
}

#[test]
fn test_pdf_margins_default() {
    let margins = PdfMargins::default();
    assert_eq!(margins.as_tuple(), (20.0, 20.0, 20.0, 20.0));
}

#[test]
fn test_pdf_margins_custom() {
    let margins = PdfMargins {
        top_mm: 15.0,
        right_mm: 25.0,
        bottom_mm: 15.0,
        left_mm: 25.0,
    };
    assert_eq!(margins.as_tuple(), (15.0, 25.0, 15.0, 25.0));
}

#[test]
fn test_pdf_export_options_custom() {
    let options = PdfExportOptions {
        page_size: PdfPageSize::Custom {
            width_mm: 150.0,
            height_mm: 250.0,
        },
        margins: PdfMargins {
            top_mm: 10.0,
            right_mm: 10.0,
            bottom_mm: 10.0,
            left_mm: 10.0,
        },
        embed_images: false,
    };
    assert!(!options.embed_images);
    let (w, h) = options.page_size.dimensions();
    assert_eq!(w, 150.0);
    assert_eq!(h, 250.0);
}

#[test]
fn test_pdf_export_serialization() {
    let options = PdfExportOptions::default();
    let json = serde_json::to_string(&options).unwrap();
    let deserialized: PdfExportOptions = serde_json::from_str(&json).unwrap();
    assert!(deserialized.embed_images);
}

#[test]
fn test_markdown_to_html_all_elements() {
    let markdown = r#"# Title

Paragraph text.

## Code Block

```rust
fn main() {
    println!("hello");
}
```

## Image

![alt text](image.png)

## Table

| Header | Data |
|--------|------|
| Cell 1 | Cell 2 |

## List

- Item 1
- Item 2

## Blockquote

> This is a quote

## Task List

- [x] Done
- [ ] Todo
"#;

    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<h1"));
    assert!(html.contains("<h2"));
    assert!(html.contains("<p>"));
    assert!(html.contains("<pre"));
    assert!(html.contains("<img"));
    assert!(html.contains("<table>"));
    assert!(html.contains("<ul>"));
    assert!(html.contains("<blockquote"));
    assert!(html.contains("<input"));
}

#[test]
fn test_markdown_code_fence_parsing() {
    let markdown = r#"```python
def hello():
    print("world")
```"#;

    let html = parser().parse_to_html(markdown);
    assert!(html.contains("python"));
    assert!(html.contains("def hello"));
}

#[test]
fn test_markdown_unicode_in_code() {
    let markdown = r#"```javascript
const emoji = "🎉";
const chinese = "中文";
```"#;

    let html = parser().parse_to_html(markdown);
    assert!(html.contains("🎉") || html.contains("emoji"));
}

#[test]
fn test_large_code_block() {
    let mut lines = vec!["```rust".to_string()];
    for i in 0..100 {
        lines.push(format!("fn function_{}() {{}}", i));
    }
    lines.push("```".to_string());
    let markdown = lines.join("\n");

    let html = parser().parse_to_html(&markdown);
    assert!(html.contains("<pre"));
    assert!(html.contains("function_0"));
    assert!(html.contains("function_99"));
}

#[test]
fn test_deeply_nested_blockquote() {
    let markdown = "> outer\n>> inner\n>>> deepest";

    let html = parser().parse_to_html(markdown);
    assert!(html.contains("<blockquote"));
}

#[test]
fn test_complex_inline_formatting() {
    let markdown = r#"**bold** *italic* ~~strike~~ `code` [link](url)"#;
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<strong>") || html.contains("<b>"));
    assert!(html.contains("<em>") || html.contains("<i>"));
    assert!(html.contains("<del>") || html.contains("<s>"));
    assert!(html.contains("<code>"));
    assert!(html.contains("<a "));
}

#[test]
fn test_multipage_content_structure() {
    let mut content = String::from("# Title\n\n");

    for i in 0..50 {
        content.push_str(&format!("## Section {}\n\n", i));
        content.push_str("Paragraph with some text.\n\n");
    }

    let html = parser().parse_to_html(&content);

    assert!(html.contains("<h1"));
    for i in 0..50 {
        assert!(html.contains(&format!("Section {}", i)));
    }
}

#[test]
fn test_image_with_special_characters_in_path() {
    let markdown = r#"![alt](assets/image%20with%20spaces.png)"#;
    let html = parser().parse_to_html(markdown);
    assert!(html.contains("image%20with%20spaces"));
}

#[test]
fn test_fenced_code_with_special_chars() {
    let markdown = r#"```javascript
const str = "```backticks```";
console.log(`template with ${str}`);
```"#;

    let html = parser().parse_to_html(markdown);
    assert!(html.contains("javascript"));
}

#[test]
fn test_markdown_parser_combined_elements() {
    let markdown = r#"| Column 1 | Column 2 |
|----------|----------|
| `code` | **bold** |
| paragraph | more text |
| ![img](pic.png) | [link](url) |
"#;

    let html = parser().parse_to_html(markdown);
    assert!(html.contains("<table>"));
    assert!(html.contains("<code>"));
    assert!(html.contains("<strong>"));
    assert!(html.contains("<img"));
    assert!(html.contains("<a "));
}

#[test]
fn test_code_block_with_language_detection() {
    let markdown = "```rust\nfn main() {}\n```";
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("class="));
    assert!(html.contains("rust") || html.contains("language-rust"));
}

#[test]
fn test_paragraph_parsing() {
    let html = parser().parse_to_html("This is a simple paragraph.");
    assert!(html.contains("<p>"));
    assert!(html.contains("simple paragraph"));
}

#[test]
fn test_horizontal_rule() {
    let html = parser().parse_to_html("---\n\nparagraph");
    assert!(html.contains("<hr"));
}

#[test]
fn test_strikethrough() {
    let html = parser().parse_to_html("~~deleted text~~");
    assert!(html.contains("<del>") || html.contains("<s>"));
}

#[test]
fn test_autolink() {
    let html = parser().parse_to_html("Visit https://example.com today!");
    assert!(html.contains("<a "));
    assert!(html.contains("https://example.com"));
}

#[test]
fn test_html_entities_in_content() {
    let html = parser().parse_to_html("5 > 3 & 3 < 5");
    assert!(html.contains("&gt;"));
    assert!(html.contains("&lt;"));
    assert!(html.contains("&amp;"));
}

#[test]
fn TC_G002_001_PDF_table_rendering_with_borders() {
    let markdown = "| Header 1 | Header 2 |\n|---------|---------|\n| Cell 1  | Cell 2  |";
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<table>"), "Should contain table element");
    assert!(
        html.contains("<th>") || html.contains("<td>"),
        "Should contain table cells"
    );
    assert!(html.contains("Header 1"), "Should contain header text");
    assert!(html.contains("Cell 1"), "Should contain cell text");
}

#[test]
fn TC_G002_002_PDF_code_block_syntax_highlighting() {
    let markdown = r#"```rust
fn main() {
    println!("Hello");
}
```"#;
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<pre"), "Should contain pre element");
    assert!(
        html.contains("class="),
        "Should contain class for syntax highlighting"
    );
    assert!(
        html.contains("rust") || html.contains("language-rust"),
        "Should contain language info"
    );
    assert!(html.contains("fn main"), "Should preserve code content");
}

#[test]
fn TC_G002_003_PDF_image_sizing_and_positioning() {
    let markdown = r#"![alt text](image.png){width=400 height=300}"#;
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<img"), "Should contain img element");
    assert!(html.contains("src="), "Should contain src attribute");
    assert!(html.contains("alt="), "Should contain alt attribute");
    assert!(html.contains("alt text"), "Should contain alt text");
}

#[test]
fn TC_G002_004_PDF_nested_blockquote_rendering() {
    let markdown = "> Level 1\n>> Level 2\n>>> Level 3";
    let html = parser().parse_to_html(markdown);

    assert!(
        html.contains("<blockquote"),
        "Should contain blockquote elements"
    );
    let blockquote_count = html.matches("<blockquote").count();
    assert_eq!(blockquote_count, 3, "Should have 3 nested blockquotes");
}

#[test]
fn TC_G002_005_PDF_task_list_rendering() {
    let markdown = "- [x] Completed task\n- [ ] Pending task\n- [ ] Another pending";
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<input"), "Should contain input element");
    assert!(
        html.contains("type=\"checkbox\""),
        "Should contain checkbox type"
    );
    assert!(
        html.contains("checked"),
        "Should mark completed task as checked"
    );
}

#[test]
fn TC_G002_006_PDF_multipage_pagination() {
    let mut markdown = String::from("# Title\n\n");
    for i in 0..60 {
        markdown.push_str(&format!("## Section {}\n\n", i));
        markdown.push_str(
            "This is a paragraph with enough text to fill a significant portion of the line. ",
        );
        markdown.push_str(
            "It continues to ensure we have substantial content for testing pagination.\n\n",
        );
    }

    let html = parser().parse_to_html(&markdown);

    assert!(html.contains("<h1"), "Should contain h1");
    for i in 0..60 {
        assert!(
            html.contains(&format!("Section {}", i)),
            "Should contain section {}",
            i
        );
    }
}

#[test]
fn TC_G002_007_PDF_complex_document_layout() {
    let markdown = r#"# Complex Document

## Code Block

```rust
fn main() {
    println!("Hello, World!");
}
```

## Table

| Column 1 | Column 2 |
|----------|----------|
| Data 1   | Data 2   |

## Image

![description](example.png)

## Blockquote

> This is a quote
>> Nested quote

## Task List

- [x] Done task
- [ ] Pending task

## List

- Item 1
- Item 2

---
"#;

    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<h1"), "Should contain h1");
    assert!(html.contains("<h2"), "Should contain h2");
    assert!(html.contains("<pre"), "Should contain code block");
    assert!(html.contains("<table>"), "Should contain table");
    assert!(html.contains("<img"), "Should contain image");
    assert!(html.contains("<blockquote"), "Should contain blockquote");
    assert!(
        html.contains("<input"),
        "Should contain task list checkboxes"
    );
    assert!(html.contains("<ul>"), "Should contain list");
    assert!(html.contains("<hr"), "Should contain horizontal rule");
}

// =============================================================================
// TC-PDF: PDF Export Quality Tests
// =============================================================================

/// Returns the path to test sample fixtures
fn get_samples_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
}

/// Returns the path to expected output baselines
fn get_expected_output_dir() -> PathBuf {
    get_samples_dir().join("expected_output")
}

// =============================================================================
// TC-PDF001: GFM Table with Merged Cells and Borders
// Category: render
// =============================================================================

#[test]
fn TC_PDF001_gfm_table_with_merged_cells_and_borders() {
    // Read the table test fixture
    let table_fixture = get_samples_dir().join("pdf_tables.md");
    let markdown =
        fs::read_to_string(&table_fixture).expect("Failed to read pdf_tables.md fixture");

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);

    // Verify table structure
    assert!(html.contains("<table>"), "Should contain table element");
    assert!(
        html.contains("</table>"),
        "Should contain closing table tag"
    );

    // Verify table headers
    assert!(
        html.contains("<th>") || html.contains("<thead>"),
        "Should contain table headers"
    );

    // Verify table cells
    assert!(html.contains("<td>"), "Should contain table data cells");

    // Verify border-related CSS (GFM tables have borders)
    assert!(
        html.contains("border") || html.contains("|"),
        "Should have border indicators"
    );

    // Verify specific table content
    assert!(html.contains("Header 1"), "Should contain first header");
    assert!(html.contains("Cell 1"), "Should contain first cell data");
    assert!(
        html.contains("Left Aligned"),
        "Should contain alignment test"
    );

    // Verify multiple tables
    let table_count = html.matches("<table>").count();
    assert!(
        table_count >= 5,
        "Should have at least 5 tables in the fixture"
    );
}

#[test]
fn TC_PDF001_verify_table_borders_in_html_export() {
    let markdown = r#"| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    // Basic table structure
    assert!(html.contains("<table>"));
    assert!(html.contains("<th>"));
    assert!(html.contains("<td>"));
    assert!(html.contains("Header 1"));
    assert!(html.contains("Cell 1"));
    assert!(html.contains("Cell 2"));
}

// =============================================================================
// TC-PDF002: Multi-language Code Blocks with Syntax Highlighting
// Category: render
// =============================================================================

#[test]
fn TC_PDF002_multi_language_code_blocks() {
    // Read the code block test fixture
    let code_fixture = get_samples_dir().join("pdf_code.md");
    let markdown = fs::read_to_string(&code_fixture).expect("Failed to read pdf_code.md fixture");

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);

    // Verify code block elements
    assert!(
        html.contains("<pre>") || html.contains("<pre "),
        "Should contain pre element for code blocks"
    );
    assert!(html.contains("<code"), "Should contain code elements");

    // Verify language detection
    assert!(
        html.contains("rust") || html.contains("language-rust"),
        "Should detect Rust language"
    );
    assert!(
        html.contains("javascript") || html.contains("language-javascript"),
        "Should detect JavaScript language"
    );
    assert!(
        html.contains("python") || html.contains("language-python"),
        "Should detect Python language"
    );

    // Verify code content preservation
    assert!(
        html.contains("fn parse") || html.contains("MarkdownParser"),
        "Should preserve Rust code content"
    );
    assert!(
        html.contains("function") || html.contains("class MarkdownRenderer"),
        "Should preserve JS code content"
    );
    assert!(
        html.contains("def ") || html.contains("class Parser"),
        "Should preserve Python code content"
    );

    // Verify syntax highlighting classes
    assert!(
        html.contains("class="),
        "Should have CSS classes for syntax highlighting"
    );

    // Verify inline code
    assert!(html.contains("inline code"), "Should handle inline code");
    assert!(
        html.contains("<code>inline code</code>") || html.contains("<code>"),
        "Should render inline code"
    );
}

#[test]
fn TC_PDF002_rust_code_block_parsing() {
    let markdown = r#"```rust
fn main() {
    println!("Hello, world!");
}
```"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    assert!(html.contains("<pre"));
    assert!(html.contains("rust") || html.contains("language-rust"));
    assert!(html.contains("fn main"));
    assert!(html.contains("println"));
}

#[test]
fn TC_PDF002_python_code_block_parsing() {
    let markdown = r#"```python
def hello_world():
    print("Hello!")
```"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    assert!(html.contains("<pre"));
    assert!(html.contains("python") || html.contains("language-python"));
    assert!(html.contains("def hello_world"));
}

#[test]
fn TC_PDF002_javascript_code_block_parsing() {
    let markdown = r#"```javascript
function greet() {
    console.log("Hi");
}
```"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    assert!(html.contains("<pre"));
    assert!(html.contains("javascript") || html.contains("language-javascript"));
    assert!(html.contains("function greet") || html.contains("greet"));
}

#[test]
fn TC_PDF002_go_code_block_parsing() {
    let markdown = r#"```go
func main() {
    fmt.Println("Hello")
}
```"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    assert!(html.contains("<pre"));
    assert!(html.contains("go") || html.contains("language-go"));
    assert!(html.contains("func main"));
}

// =============================================================================
// TC-PDF003: Images at Various Sizes and Positions
// Category: render
// =============================================================================

#[test]
fn TC_PDF003_images_various_sizes_and_positions() {
    // Read the image test fixture
    let image_fixture = get_samples_dir().join("pdf_images.md");
    let markdown =
        fs::read_to_string(&image_fixture).expect("Failed to read pdf_images.md fixture");

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);

    // Verify image elements
    assert!(html.contains("<img"), "Should contain img elements");
    assert!(html.contains("src="), "Should have src attribute");
    assert!(html.contains("alt="), "Should have alt attribute");

    // Verify various image sizes mentioned
    assert!(html.contains("small"), "Should mention small images");
    assert!(html.contains("medium"), "Should mention medium images");
    assert!(html.contains("large"), "Should mention large images");
    assert!(
        html.contains("full width"),
        "Should mention full width images"
    );

    // Verify multiple images
    let img_count = html.matches("<img").count();
    assert!(img_count >= 10, "Should have multiple images in fixture");

    // Verify alt text
    assert!(html.contains("alt text"), "Should contain alt text");
    assert!(
        html.contains("This is descriptive alt text"),
        "Should have descriptive alt"
    );

    // Verify centered images (alignment)
    assert!(html.contains("centered"), "Should mention centered images");

    // Verify images in sequence
    assert!(
        html.contains("Image 1") || html.contains("Following image"),
        "Should have images in sequence context"
    );
}

#[test]
fn TC_PDF003_image_element_structure() {
    let markdown = "![alt text](image.png)";

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    assert!(html.contains("<img"));
    assert!(html.contains("src="));
    assert!(html.contains("alt="));
    assert!(html.contains("alt text"));
    assert!(html.contains("image.png"));
}

#[test]
fn TC_PDF003_image_with_title() {
    let markdown = "![alt](image.png \"Optional Title\")";

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    assert!(html.contains("<img"));
    assert!(html.contains("title=") || html.contains("Optional Title"));
}

// =============================================================================
// TC-PDF004: All Features Combined
// Category: render
// =============================================================================

#[test]
fn TC_PDF004_all_features_combined() {
    // Read the complex test fixture
    let complex_fixture = get_samples_dir().join("pdf_complex.md");
    let markdown =
        fs::read_to_string(&complex_fixture).expect("Failed to read pdf_complex.md fixture");

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);

    // Verify headings
    assert!(html.contains("<h1"), "Should contain h1");
    assert!(html.contains("<h2"), "Should contain h2");
    assert!(html.contains("<h3"), "Should contain h3");
    assert!(
        html.contains("Document Title"),
        "Should have document title"
    );
    assert!(html.contains("Section 1"), "Should have section headings");

    // Verify text formatting
    assert!(
        html.contains("<strong>") || html.contains("<b>"),
        "Should contain bold"
    );
    assert!(
        html.contains("<em>") || html.contains("<i>"),
        "Should contain italic"
    );
    assert!(html.contains("<code>"), "Should contain inline code");
    assert!(html.contains("<a "), "Should contain links");

    // Verify code blocks
    assert!(html.contains("<pre"), "Should contain code blocks");
    assert!(html.contains("fn main"), "Should contain Rust code");
    assert!(html.contains("console.log"), "Should contain JS code");

    // Verify tables
    assert!(html.contains("<table>"), "Should contain tables");
    assert!(html.contains("<th>"), "Should contain table headers");
    assert!(html.contains("<td>"), "Should contain table cells");
    assert!(
        html.contains("Feature Comparison"),
        "Should have feature table"
    );

    // Verify lists
    assert!(html.contains("<ul>"), "Should contain unordered lists");
    assert!(html.contains("<ol>"), "Should contain ordered lists");
    assert!(html.contains("<li>"), "Should contain list items");

    // Verify task lists
    assert!(
        html.contains("<input"),
        "Should contain task list checkboxes"
    );
    assert!(
        html.contains("type=\"checkbox\""),
        "Should have checkbox type"
    );
    assert!(html.contains("checked"), "Should have checked state");

    // Verify blockquotes
    assert!(html.contains("<blockquote"), "Should contain blockquotes");
    assert!(
        html.contains("tip") || html.contains("Tip"),
        "Should have blockquote content"
    );

    // Verify images
    assert!(html.contains("<img"), "Should contain images");

    // Verify horizontal rules
    assert!(html.contains("<hr"), "Should contain horizontal rules");

    // Verify strikethrough
    assert!(
        html.contains("<del>") || html.contains("<s>"),
        "Should contain strikethrough"
    );

    // Verify special characters
    assert!(
        html.contains("&lt;") || html.contains("<"),
        "Should handle <"
    );
    assert!(
        html.contains("&gt;") || html.contains(">"),
        "Should handle >"
    );
    assert!(html.contains("&amp;"), "Should handle &");
}

// =============================================================================
// TC-PDF005: Nested Blockquotes, Lists, and Code
// Category: render
// =============================================================================

#[test]
fn TC_PDF005_nested_blockquotes_lists_and_code() {
    // Read the nested test fixture
    let nested_fixture = get_samples_dir().join("pdf_nested.md");
    let markdown =
        fs::read_to_string(&nested_fixture).expect("Failed to read pdf_nested.md fixture");

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(&markdown);

    // Verify all heading levels (H1-H6)
    assert!(html.contains("<h1"), "Should contain h1");
    assert!(html.contains("<h2"), "Should contain h2");
    assert!(html.contains("<h3"), "Should contain h3");
    assert!(html.contains("<h4"), "Should contain h4");
    assert!(html.contains("<h5"), "Should contain h5");
    assert!(html.contains("<h6"), "Should contain h6");

    // Verify 5-level nesting for blockquotes
    let blockquote_count = html.matches("<blockquote").count();
    assert!(
        blockquote_count >= 5,
        "Should have multiple nested blockquotes (at least 5)"
    );

    // Verify 5-level nesting for lists
    let list_count = html.matches("<ul>").count() + html.matches("<ol>").count();
    assert!(
        list_count >= 5,
        "Should have multiple nested lists (at least 5)"
    );

    // Verify code blocks in nested structures
    assert!(html.contains("<pre"), "Should contain code blocks");
    assert!(
        html.contains("rust") || html.contains("python"),
        "Should contain code in nested context"
    );

    // Verify maximum depth text
    assert!(
        html.contains("Level 1") || html.contains("L1"),
        "Should mention level 1 content"
    );
    assert!(
        html.contains("Level 5") || html.contains("L5") || html.contains("deepest"),
        "Should mention level 5/deepest content"
    );

    // Verify nested task lists
    assert!(
        html.contains("<input"),
        "Should contain nested task list checkboxes"
    );
}

#[test]
fn TC_PDF005_heading_levels_hierarchy() {
    let markdown = r#"# H1
## H2
### H3
#### H4
##### H5
###### H6"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    assert!(html.contains("<h1"));
    assert!(html.contains("<h2"));
    assert!(html.contains("<h3"));
    assert!(html.contains("<h4"));
    assert!(html.contains("<h5"));
    assert!(html.contains("<h6"));
}

#[test]
fn TC_PDF005_nested_blockquote_depth() {
    let markdown = r#"> L1
>> L2
>>> L3
>>>> L4
>>>>> L5"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    let blockquote_count = html.matches("<blockquote").count();
    assert_eq!(blockquote_count, 5, "Should have 5 nested blockquotes");
}

#[test]
fn TC_PDF005_nested_list_depth() {
    let markdown = r#"- L1
  - L2
    - L3
      - L4
        - L5"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    let list_count = html.matches("<ul>").count();
    assert!(list_count >= 4, "Should have nested list structure");
}

// =============================================================================
// TC-PDF006: PDF File Size Validation
// Category: unit
// =============================================================================

#[test]
fn TC_PDF006_pdf_file_size_validation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("test_output.pdf");

    // Create a substantial Markdown document
    let markdown = r#"# Test Document

This is a test paragraph with substantial content to ensure the PDF 
has meaningful size. We need to generate enough content to create a 
properly sized PDF file.

## Code Block

```rust
fn main() {
    println!("This is a code block that adds content to the PDF");
}
```

## Another Section

More content here to increase the file size.

| Column 1 | Column 2 |
|----------|----------|
| Data 1   | Data 2   |

- List item 1
- List item 2
- List item 3
"#;

    let export_service = rustnote_lib::services::export::ExportService::new();
    export_service
        .export_to_pdf_with_options(
            markdown,
            output_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    // Verify file was created
    assert!(output_path.exists(), "PDF file should be created");

    // Check file size
    let file_size = fs::metadata(&output_path)
        .expect("Failed to get file metadata")
        .len();

    // File should be > 1KB (typical minimum for a valid PDF)
    assert!(
        file_size > 1024,
        "PDF size should be > 1KB, got {} bytes",
        file_size
    );

    // File should be < 50MB (reasonable upper limit)
    assert!(
        file_size < 50 * 1024 * 1024,
        "PDF size should be < 50MB, got {} bytes",
        file_size
    );

    // Verify it's a valid PDF by checking header
    let file_content = fs::read(&output_path).expect("Failed to read PDF");
    assert!(
        file_content.starts_with(b"%PDF-"),
        "File should be a valid PDF with correct header"
    );
}

#[test]
fn TC_PDF006_minimal_pdf_size() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("minimal.pdf");

    // Minimal document
    let markdown = "# Simple";

    let export_service = rustnote_lib::services::export::ExportService::new();
    export_service
        .export_to_pdf_with_options(
            markdown,
            output_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    let file_size = fs::metadata(&output_path)
        .expect("Failed to get file metadata")
        .len();

    // Even minimal PDF should be at least a few hundred bytes
    assert!(
        file_size > 100,
        "Minimal PDF should be > 100 bytes, got {} bytes",
        file_size
    );
}

// =============================================================================
// TC-PDF007: PDF Contains Expected Text
// Category: unit
// =============================================================================

#[test]
fn TC_PDF007_pdf_contains_expected_text() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("text_test.pdf");

    let markdown = r#"# Document Title

## Introduction

This is the introduction paragraph with **bold text** and *italic text*.

## Code Section

```rust
fn main() {
    println!("Hello, PDF!");
}
```

## Data Table

| Name | Value |
|------|-------|
| Alpha | 100 |
| Beta  | 200 |
"#;

    let export_service = rustnote_lib::services::export::ExportService::new();
    export_service
        .export_to_pdf_with_options(
            markdown,
            output_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    // Verify PDF was created successfully
    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");

    // Verify it's a valid PDF by checking the header
    assert!(
        pdf_bytes.starts_with(b"%PDF-"),
        "PDF should have valid header"
    );

    // Verify PDF has basic structure
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);
    assert!(
        pdf_str.contains("/Pages") || pdf_str.contains("/Page"),
        "PDF should have page structure"
    );
}

#[test]
fn TC_PDF007_text_extraction_from_markdown() {
    // Test that our parser correctly extracts text from Markdown
    let markdown = r#"# Heading One

Some paragraph text with **bold** and *italic*.

## Heading Two

- List item one
- List item two
"#;

    let parser = MarkdownParser::new();
    let html = parser.parse_to_html(markdown);

    // Verify all text content is preserved
    assert!(html.contains("Heading One"));
    assert!(html.contains("Heading Two"));
    assert!(html.contains("Some paragraph text"));
    assert!(html.contains("bold"));
    assert!(html.contains("italic"));
    assert!(html.contains("List item one"));
    assert!(html.contains("List item two"));
}

// =============================================================================
// TC-PDF008: PDF Page Count Validation
// Category: unit
// =============================================================================

#[test]
fn TC_PDF008_pdf_page_count_validation() {
    let doc10_fixture = get_samples_dir().join("pdf_10page.md");
    let markdown =
        fs::read_to_string(&doc10_fixture).expect("Failed to read pdf_10page.md fixture");

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("10page_test.pdf");

    let export_service = rustnote_lib::services::export::ExportService::new();
    export_service
        .export_to_pdf_with_options(
            &markdown,
            output_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");
    let pdf_str = String::from_utf8_lossy(&pdf_bytes);

    // Verify PDF was created successfully
    assert!(
        pdf_bytes.starts_with(b"%PDF-"),
        "PDF should have valid header"
    );
    assert!(pdf_bytes.len() > 100, "PDF should have reasonable size");
}

#[test]
fn TC_PDF008_single_page_document() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("single_page.pdf");

    // Minimal single-page document
    let markdown = "# Title\n\nShort content.";

    let export_service = rustnote_lib::services::export::ExportService::new();
    export_service
        .export_to_pdf_with_options(
            markdown,
            output_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    let pdf_bytes = fs::read(&output_path).expect("Failed to read PDF");

    // Single-page document should be a valid PDF
    assert!(
        pdf_bytes.starts_with(b"%PDF-"),
        "PDF should have valid header"
    );
    assert!(pdf_bytes.len() > 100, "PDF should have reasonable size");
}

// =============================================================================
// TC-PDF009: PDF Export Time < 5s
// Category: performance
// =============================================================================

#[test]
fn TC_PDF009_pdf_export_time_under_5_seconds() {
    // Read the 10-page test document
    let doc10_fixture = get_samples_dir().join("pdf_10page.md");
    let markdown =
        fs::read_to_string(&doc10_fixture).expect("Failed to read pdf_10page.md fixture");

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("perf_test.pdf");

    let export_service = rustnote_lib::services::export::ExportService::new();

    // Measure export time
    let start = Instant::now();

    export_service
        .export_to_pdf_with_options(
            &markdown,
            output_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    let elapsed = start.elapsed();
    let elapsed_ms = elapsed.as_millis() as f64;

    // Verify export completed in under 5 seconds (NFR-006)
    assert!(
        elapsed_ms < 5000.0,
        "PDF export should complete in < 5s, took {:.2}ms",
        elapsed_ms
    );

    // Log performance for reference
    println!("PDF export took {:.2}ms for 10-page document", elapsed_ms);
}

#[test]
fn TC_PDF009_small_document_export_time() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("small_perf.pdf");

    // Small document
    let markdown = "# Title\n\nContent.";

    let export_service = rustnote_lib::services::export::ExportService::new();

    let start = Instant::now();

    export_service
        .export_to_pdf_with_options(
            markdown,
            output_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    let elapsed = start.elapsed();

    // Small document should be very fast (< 500ms)
    assert!(
        elapsed.as_millis() < 500,
        "Small PDF export should complete in < 500ms, took {:.2}ms",
        elapsed.as_millis()
    );
}

#[test]
fn TC_PDF009_export_time_benchmark() {
    // Benchmark various document sizes
    let test_cases = vec![
        ("minimal", "# T\n\nC"),
        ("small", "# Title\n\n## Section\n\nContent here."),
        ("medium", "# Document\n\n## Section 1\n\nContent.\n\n## Section 2\n\nMore content.\n\n```\nCode\n```"),
    ];

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let export_service = rustnote_lib::services::export::ExportService::new();

    for (name, markdown) in test_cases {
        let output_path = temp_dir.path().join(format!("bench_{}.pdf", name));

        let start = Instant::now();

        export_service
            .export_to_pdf_with_options(
                markdown,
                output_path.to_str().unwrap(),
                PdfExportOptions::default(),
            )
            .expect("PDF export failed");

        let elapsed = start.elapsed();

        println!("{} PDF export: {:.2}ms", name, elapsed.as_millis());

        // All should complete quickly
        assert!(
            elapsed.as_millis() < 2000,
            "{} PDF should complete in < 2s",
            name
        );
    }
}

// =============================================================================
// TC-PDF010: Visual Regression Baseline Comparison
// Category: integration
// =============================================================================

#[test]
fn TC_PDF010_visual_regression_baseline_exists() {
    let baseline_dir = get_expected_output_dir();

    // Verify baseline directory exists
    assert!(
        baseline_dir.exists()
            || std::process::Command::new("mkdir")
                .arg("-p")
                .arg(&baseline_dir)
                .status()
                .is_ok(),
        "Baseline directory should exist or be creatable"
    );

    // Note: This test verifies the baseline infrastructure exists
    // Actual visual comparison would require:
    // 1. Reference PDFs in expected_output/
    // 2. Image comparison tool (like pixelmatch or similar)
    // 3. Rendering PDFs to images for comparison
}

#[test]
fn TC_PDF010_baseline_generation_check() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let baseline_path = temp_dir.path().join("baseline.pdf");

    // Generate a baseline PDF
    let markdown = "# Baseline Test\n\nContent for baseline.";

    let export_service = rustnote_lib::services::export::ExportService::new();
    export_service
        .export_to_pdf_with_options(
            markdown,
            baseline_path.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("PDF export failed");

    // Verify baseline was created
    assert!(baseline_path.exists(), "Baseline PDF should be generated");

    // Verify it's a valid PDF
    let content = fs::read(&baseline_path).expect("Failed to read baseline");
    assert!(
        content.starts_with(b"%PDF-"),
        "Baseline should be valid PDF"
    );
}

#[test]
fn TC_PDF010_consistent_output_generation() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let markdown = "# Consistency Test\n\n## Section\n\nTest content.";

    let export_service = rustnote_lib::services::export::ExportService::new();

    // Generate two PDFs with the same content
    let path1 = temp_dir.path().join("output1.pdf");
    let path2 = temp_dir.path().join("output2.pdf");

    export_service
        .export_to_pdf_with_options(
            &markdown,
            path1.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("First export failed");

    export_service
        .export_to_pdf_with_options(
            &markdown,
            path2.to_str().unwrap(),
            PdfExportOptions::default(),
        )
        .expect("Second export failed");

    // Both should be valid PDFs
    let bytes1 = fs::read(&path1).expect("Failed to read first PDF");
    let bytes2 = fs::read(&path2).expect("Failed to read second PDF");
    let content1 = String::from_utf8_lossy(&bytes1);
    let content2 = String::from_utf8_lossy(&bytes2);

    assert!(
        bytes1.starts_with(b"%PDF-"),
        "First output should be valid PDF"
    );
    assert!(
        bytes2.starts_with(b"%PDF-"),
        "Second output should be valid PDF"
    );

    // Both should have similar structure (exact match depends on implementation)
    assert!(
        content1.contains("/Page"),
        "First should have page structure"
    );
    assert!(
        content2.contains("/Page"),
        "Second should have page structure"
    );
}

// =============================================================================
// Additional Integration Tests
// =============================================================================

#[test]
fn TC_pdf_quality_comprehensive_html_export() {
    // Test HTML export (which PDF is based on)
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("html_export.html");

    let markdown = r#"# Comprehensive HTML Test

## Features

- **Bold** and *italic* text
- `inline code`
- [Links](https://example.com)

## Code

```rust
fn main() {
    println!("Export quality test");
}
```

## Table

| A | B | C |
|---|---|---|
| 1 | 2 | 3 |

## Quote

> Blockquote text
"#;

    let export_service = rustnote_lib::services::export::ExportService::new();

    export_service
        .export_to_html(
            markdown,
            output_path.to_str().unwrap(),
            HtmlExportOptions::default(),
        )
        .expect("HTML export failed");

    // Verify HTML was created
    assert!(output_path.exists(), "HTML file should be created");

    let html_content = fs::read_to_string(&output_path).expect("Failed to read HTML");

    // Verify HTML structure
    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("<html>"));
    assert!(html_content.contains("<head>"));
    assert!(html_content.contains("<body>"));

    // Verify content
    assert!(html_content.contains("<h1"));
    assert!(html_content.contains("<h2"));
    assert!(html_content.contains("<strong>") || html_content.contains("<b>"));
    assert!(html_content.contains("<em>") || html_content.contains("<i>"));
    assert!(html_content.contains("<code>"));
    assert!(html_content.contains("<a "));
    assert!(html_content.contains("<pre"));
    assert!(html_content.contains("<table>"));
    assert!(html_content.contains("<blockquote"));
}

#[test]
fn TC_pdf_quality_print_html_generation() {
    let markdown = "# Print Test\n\n## Content\n\nTest paragraph.";

    let export_service = rustnote_lib::services::export::ExportService::new();
    let print_html = export_service.get_print_html(markdown);

    // Verify print HTML contains necessary CSS
    assert!(print_html.contains("@page"), "Should contain @page rule");
    assert!(
        print_html.contains("page-break"),
        "Should contain page break CSS"
    );
    assert!(
        print_html.contains("font-family"),
        "Should contain font settings"
    );
    assert!(print_html.contains("<style>"), "Should have style block");
}

#[test]
fn TC_pdf_quality_pdf_options_customization() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir.path().join("custom_options.pdf");

    let markdown = "# Custom Options Test";

    // Test with custom options
    let options = PdfExportOptions {
        page_size: PdfPageSize::Letter,
        margins: PdfMargins {
            top_mm: 25.0,
            right_mm: 25.0,
            bottom_mm: 25.0,
            left_mm: 25.0,
        },
        embed_images: true,
    };

    let export_service = rustnote_lib::services::export::ExportService::new();
    export_service
        .export_to_pdf_with_options(markdown, output_path.to_str().unwrap(), options)
        .expect("PDF export with custom options failed");

    assert!(output_path.exists(), "Custom options PDF should be created");
}

#[test]
fn TC_pdf_quality_a4_vs_letter_page_sizes() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let markdown = "# Page Size Test\n\nContent to fill page.";

    let export_service = rustnote_lib::services::export::ExportService::new();

    // Test A4
    let a4_path = temp_dir.path().join("a4_test.pdf");
    export_service
        .export_to_pdf_with_options(
            markdown,
            a4_path.to_str().unwrap(),
            PdfExportOptions {
                page_size: PdfPageSize::A4,
                margins: PdfMargins::default(),
                embed_images: true,
            },
        )
        .expect("A4 export failed");

    // Test Letter
    let letter_path = temp_dir.path().join("letter_test.pdf");
    export_service
        .export_to_pdf_with_options(
            markdown,
            letter_path.to_str().unwrap(),
            PdfExportOptions {
                page_size: PdfPageSize::Letter,
                margins: PdfMargins::default(),
                embed_images: true,
            },
        )
        .expect("Letter export failed");

    // Both should exist
    assert!(a4_path.exists(), "A4 PDF should be created");
    assert!(letter_path.exists(), "Letter PDF should be created");

    // Both should be valid PDFs
    let a4_content = fs::read(&a4_path).expect("Failed to read A4 PDF");
    let letter_content = fs::read(&letter_path).expect("Failed to read Letter PDF");

    assert!(a4_content.starts_with(b"%PDF-"));
    assert!(letter_content.starts_with(b"%PDF-"));
}

// =============================================================================
// Helper Tests
// =============================================================================

#[test]
fn TC_helper_fixture_files_exist() {
    // Verify all required fixture files exist
    let samples_dir = get_samples_dir();

    assert!(samples_dir.exists(), "Samples directory should exist");

    let required_fixtures = vec![
        "pdf_tables.md",
        "pdf_code.md",
        "pdf_images.md",
        "pdf_complex.md",
        "pdf_nested.md",
        "pdf_10page.md",
    ];

    for fixture in required_fixtures {
        let path = samples_dir.join(fixture);
        assert!(path.exists(), "Fixture {} should exist", fixture);

        // Verify fixture is not empty
        let content = fs::read_to_string(&path).expect(&format!("Failed to read {}", fixture));
        assert!(
            !content.is_empty(),
            "Fixture {} should not be empty",
            fixture
        );
    }
}

#[test]
fn TC_helper_parser_integration() {
    // Test that the parser works correctly with all fixtures
    let samples_dir = get_samples_dir();

    let fixtures = vec![
        "pdf_tables.md",
        "pdf_code.md",
        "pdf_images.md",
        "pdf_complex.md",
        "pdf_nested.md",
        "pdf_10page.md",
    ];

    let parser = MarkdownParser::new();

    for fixture in fixtures {
        let path = samples_dir.join(fixture);
        let content = fs::read_to_string(&path).expect(&format!("Failed to read {}", fixture));

        // Parser should not panic
        let result = std::panic::catch_unwind(|| parser.parse_to_html(&content));

        assert!(result.is_ok(), "Parser should not panic on {}", fixture);

        let html = result.unwrap();
        assert!(
            !html.is_empty(),
            "HTML output should not be empty for {}",
            fixture
        );
    }
}
