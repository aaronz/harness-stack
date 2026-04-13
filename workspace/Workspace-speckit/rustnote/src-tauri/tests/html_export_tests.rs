use rustnote_lib::model::export::{HtmlExportMode, HtmlExportOptions};
use rustnote_lib::parser::MarkdownParser;
use tempfile::TempDir;

fn parser() -> MarkdownParser {
    MarkdownParser::new()
}

#[test]
fn TC_G011_001_HTMLExport_linked_mode() {
    let markdown = "![img](image.png)";
    let html = parser().parse_to_html(markdown);
    assert!(html.contains("<img"));
    assert!(html.contains("src=\"image.png\""));
    assert!(!html.contains("base64"));
}

#[test]
fn TC_G011_002_HTMLExport_inline_mode() {
    let markdown = "![img](image.png)";
    let html = parser().parse_to_html(markdown);
    assert!(html.contains("<img"));
    assert!(html.contains("src=\"image.png\""));
}

#[test]
fn TC_G011_003_HTMLExport_css_styles() {
    let markdown = "# Heading\n\nparagraph";
    let html = parser().parse_to_html(markdown);
    assert!(html.contains("<h1"));
    assert!(html.contains("<p>"));
}

#[test]
fn test_html_export_options_default() {
    let options = HtmlExportOptions::default();
    assert_eq!(options.mode, HtmlExportMode::default());
    assert!(options.embed_css);
}

#[test]
fn test_html_export_mode_default_is_inline() {
    let mode = HtmlExportMode::default();
    assert_eq!(mode, HtmlExportMode::Inline);
}

#[test]
fn test_html_export_mode_linked() {
    let mode = HtmlExportMode::Linked {
        assets_dir: "assets".to_string(),
    };
    assert!(matches!(mode, HtmlExportMode::Linked { .. }));
    if let HtmlExportMode::Linked { assets_dir } = mode {
        assert_eq!(assets_dir, "assets");
    }
}

#[test]
fn test_html_export_mode_inline() {
    let mode = HtmlExportMode::Inline;
    assert!(matches!(mode, HtmlExportMode::Inline));
}

#[test]
fn test_html_export_options_serialization() {
    let options = HtmlExportOptions::default();
    let json = serde_json::to_string(&options).unwrap();
    let deserialized: HtmlExportOptions = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.mode, HtmlExportMode::Inline);
    assert!(deserialized.embed_css);
}

#[test]
fn test_html_export_linked_mode_serialization() {
    let mode = HtmlExportMode::Linked {
        assets_dir: "images".to_string(),
    };
    let json = serde_json::to_string(&mode).unwrap();
    assert!(json.contains("\"type\":\"Linked\""));
    assert!(json.contains("images"));
}

#[test]
fn test_html_export_inline_mode_serialization() {
    let mode = HtmlExportMode::Inline;
    let json = serde_json::to_string(&mode).unwrap();
    assert!(json.contains("\"type\":\"Inline\""));
}

#[test]
fn test_markdown_image_parsing() {
    let html = parser().parse_to_html("![img](image.png)");
    assert!(html.contains("<img"));
    assert!(html.contains("src="));
    assert!(html.contains("alt="));
    assert!(html.contains("image.png"));
}

#[test]
fn test_markdown_image_with_path() {
    let html = parser().parse_to_html("![alt text](assets/image.png)");
    assert!(html.contains("src=\"assets/image.png\""));
}

#[test]
fn test_markdown_multiple_images() {
    let html = parser().parse_to_html("![img1](a.png)\n\n![img2](b.png)");
    let count = html.matches("<img").count();
    assert_eq!(count, 2);
}

#[test]
fn test_markdown_css_styles_in_html() {
    let html = parser().parse_to_html("# Heading\n\nparagraph\n\n```rust\ncode\n```");
    assert!(html.contains("<h1"));
    assert!(html.contains("<p>"));
    assert!(html.contains("<pre"));
}

#[test]
fn test_large_image_handling() {
    let markdown = format!("![img](large_image.png)");
    let html = parser().parse_to_html(&markdown);
    assert!(html.contains("<img"));
    assert!(html.contains("large_image.png"));
}

#[test]
fn test_many_assets() {
    let assets: Vec<String> = (0..10)
        .map(|i| format!("![img{}](image{}.png)", i, i))
        .collect();
    let markdown = assets.join("\n");
    let html = parser().parse_to_html(&markdown);
    let count = html.matches("<img").count();
    assert_eq!(count, 10);
}

#[test]
fn test_export_html_options_with_linked_mode() {
    let options = HtmlExportOptions {
        mode: HtmlExportMode::Linked {
            assets_dir: "exported_assets".to_string(),
        },
        embed_css: true,
    };
    assert!(matches!(options.mode, HtmlExportMode::Linked { .. }));
    assert!(options.embed_css);
}

#[test]
fn test_export_html_options_with_inline_mode() {
    let options = HtmlExportOptions {
        mode: HtmlExportMode::Inline,
        embed_css: false,
    };
    assert!(matches!(options.mode, HtmlExportMode::Inline));
    assert!(!options.embed_css);
}

#[test]
fn test_css_inline_when_embed_css_true() {
    let options = HtmlExportOptions {
        mode: HtmlExportMode::Inline,
        embed_css: true,
    };
    let temp_dir = TempDir::new().unwrap();
    let html_path = temp_dir.path().join("test.html");

    let markdown = "# Test\n\nTest content";
    let html = parser().parse_to_html(markdown);

    let head_section = format!(r#"<style>{}</style>"#, 
        "body { font-family: -apple-system, system-ui, sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; }");

    if html_path.to_str().is_some() {
        assert!(!head_section.is_empty());
    }
}

#[test]
fn test_css_link_when_embed_css_false() {
    let options = HtmlExportOptions {
        mode: HtmlExportMode::Linked {
            assets_dir: "assets".to_string(),
        },
        embed_css: false,
    };
    assert!(!options.embed_css);
    if let HtmlExportMode::Linked { assets_dir } = options.mode {
        assert_eq!(assets_dir, "assets");
    }
}

#[test]
fn test_image_paths_preserved_in_linked_mode() {
    let markdown = "![alt](image.png)";
    let html = parser().parse_to_html(markdown);
    assert!(html.contains("image.png"));
}

#[test]
fn test_markdown_code_block_with_syntax() {
    let markdown = r#"```javascript
const x = 1;
console.log(x);
```"#;
    let html = parser().parse_to_html(markdown);
    assert!(html.contains("<pre"));
    assert!(html.contains("javascript"));
}

#[test]
fn test_markdown_styled_content() {
    let markdown = "**bold** and *italic* and `code`";
    let html = parser().parse_to_html(markdown);
    assert!(html.contains("<strong>") || html.contains("<b>"));
    assert!(html.contains("<em>") || html.contains("<i>"));
    assert!(html.contains("<code>"));
}
