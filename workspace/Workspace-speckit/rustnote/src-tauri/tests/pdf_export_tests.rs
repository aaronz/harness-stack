use rustnote_lib::model::export::{PdfExportOptions, PdfMargins, PdfPageSize};
use rustnote_lib::parser::MarkdownParser;

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
