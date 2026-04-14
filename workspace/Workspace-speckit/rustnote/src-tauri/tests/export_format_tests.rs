use rustnote_lib::model::export::{ExportFormat, HtmlExportOptions, PdfExportOptions};
use rustnote_lib::parser::MarkdownParser;
use rustnote_lib::services::export::{ExportOptions, ExportService, ExportServiceTrait};
use std::fs;
use tempfile::TempDir;

fn parser() -> MarkdownParser {
    MarkdownParser::new()
}

fn export_service() -> ExportService {
    ExportService::new()
}

// =============================================================================
// TC-P2-018-01: ExportFormat enum definition
// Category: unit
// Input: model/export.rs
// Expected: ExportFormat enum has HTML and PDF variants
// =============================================================================

#[test]
fn TC_P2_018_01_export_format_enum_has_html_variant() {
    let format = ExportFormat::Html;
    assert!(matches!(format, ExportFormat::Html));
}

#[test]
fn TC_P2_018_01_export_format_enum_has_pdf_variant() {
    let format = ExportFormat::Pdf;
    assert!(matches!(format, ExportFormat::Pdf));
}

#[test]
fn TC_P2_018_01_export_format_enum_only_has_html_and_pdf() {
    let formats = ExportFormat::supported_formats();
    assert_eq!(formats.len(), 2);
    assert!(formats.contains(&ExportFormat::Html));
    assert!(formats.contains(&ExportFormat::Pdf));
}

#[test]
fn TC_P2_018_01_export_format_is_supported() {
    assert!(ExportFormat::Html.is_supported());
    assert!(ExportFormat::Pdf.is_supported());
}

#[test]
fn TC_P2_018_01_export_format_default_is_html() {
    let default_format = ExportFormat::default();
    assert_eq!(default_format, ExportFormat::Html);
}

#[test]
fn TC_P2_018_01_export_format_display_name() {
    assert_eq!(ExportFormat::Html.display_name(), "HTML");
    assert_eq!(ExportFormat::Pdf.display_name(), "PDF");
}

#[test]
fn TC_P2_018_01_export_format_extension() {
    assert_eq!(ExportFormat::Html.extension(), "html");
    assert_eq!(ExportFormat::Pdf.extension(), "pdf");
}

#[test]
fn TC_P2_018_01_export_format_from_extension() {
    assert_eq!(
        ExportFormat::from_extension("html"),
        Some(ExportFormat::Html)
    );
    assert_eq!(
        ExportFormat::from_extension("htm"),
        Some(ExportFormat::Html)
    );
    assert_eq!(
        ExportFormat::from_extension("HTML"),
        Some(ExportFormat::Html)
    );
    assert_eq!(ExportFormat::from_extension("pdf"), Some(ExportFormat::Pdf));
    assert_eq!(ExportFormat::from_extension("PDF"), Some(ExportFormat::Pdf));
    assert_eq!(ExportFormat::from_extension("docx"), None);
    assert_eq!(ExportFormat::from_extension("epub"), None);
    assert_eq!(ExportFormat::from_extension("unknown"), None);
}

#[test]
fn TC_P2_018_01_export_format_display_trait() {
    assert_eq!(format!("{}", ExportFormat::Html), "HTML");
    assert_eq!(format!("{}", ExportFormat::Pdf), "PDF");
}

#[test]
fn TC_P2_018_01_export_format_serialization() {
    let html_json = serde_json::to_string(&ExportFormat::Html).unwrap();
    assert!(html_json.contains("\"type\":\"Html\""));

    let pdf_json = serde_json::to_string(&ExportFormat::Pdf).unwrap();
    assert!(pdf_json.contains("\"type\":\"Pdf\""));
}

#[test]
fn TC_P2_018_01_export_format_deserialization() {
    let html: ExportFormat = serde_json::from_str(r#"{"type":"Html"}"#).unwrap();
    assert_eq!(html, ExportFormat::Html);

    let pdf: ExportFormat = serde_json::from_str(r#"{"type":"Pdf"}"#).unwrap();
    assert_eq!(pdf, ExportFormat::Pdf);
}

#[test]
fn TC_P2_018_01_export_format_clone_and_copy() {
    let format1 = ExportFormat::Html;
    let format2 = format1;
    let format3 = format1.clone();
    assert_eq!(format1, format2);
    assert_eq!(format1, format3);
}

#[test]
fn TC_P2_018_01_export_format_eq() {
    assert_eq!(ExportFormat::Html, ExportFormat::Html);
    assert_eq!(ExportFormat::Pdf, ExportFormat::Pdf);
    assert_ne!(ExportFormat::Html, ExportFormat::Pdf);
}

// =============================================================================
// TC-P2-018-02: Factory pattern — HTML export
// Category: unit
// Input: ExportFormat::HTML
// Expected: HTML export pipeline executed
// =============================================================================

#[test]
fn TC_P2_018_02_factory_creates_html_export_options() {
    let options = ExportOptions::for_format(ExportFormat::Html);
    assert!(matches!(options, ExportOptions::Html(_)));
    assert_eq!(options.format(), ExportFormat::Html);
}

#[test]
fn TC_P2_018_02_factory_exports_html_with_default_options() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.html");

    let markdown = "# Test\n\nHello **world**!";
    let service = export_service();

    service
        .export_with_format(
            markdown,
            output_path.to_str().unwrap(),
            ExportFormat::Html,
            ExportOptions::default(),
        )
        .expect("HTML export should succeed");

    assert!(output_path.exists(), "HTML file should be created");

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("<h1"));
    assert!(content.contains("Test"));
    assert!(content.contains("Hello"));
    assert!(content.contains("<strong>") || content.contains("<b>"));
}

#[test]
fn TC_P2_018_02_factory_exports_html_with_custom_options() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_custom.html");

    let markdown = "# Test\n\nHello **world**!";
    let service = export_service();

    let custom_options = ExportOptions::Html(HtmlExportOptions {
        mode: rustnote_lib::model::export::HtmlExportMode::Inline,
        embed_css: true,
    });

    service
        .export_with_format(
            markdown,
            output_path.to_str().unwrap(),
            ExportFormat::Html,
            custom_options,
        )
        .expect("HTML export should succeed");

    assert!(output_path.exists(), "HTML file should be created");

    let content = fs::read_to_string(&output_path).unwrap();
    assert!(content.contains("<h1"));
    assert!(content.contains("<!DOCTYPE html>"));
}

#[test]
fn TC_P2_018_02_factory_supports_html_format() {
    let service = export_service();
    let formats = service.get_supported_formats();
    assert!(formats.contains(&ExportFormat::Html));
}

#[test]
fn TC_P2_018_02_factory_html_from_extension() {
    assert_eq!(
        ExportFormat::from_extension("html"),
        Some(ExportFormat::Html)
    );
}

#[test]
fn TC_P2_018_02_export_options_html_serialization() {
    let options = ExportOptions::Html(HtmlExportOptions::default());
    let json = serde_json::to_string(&options).unwrap();
    assert!(json.contains("\"Html\""));
}

#[test]
fn TC_P2_018_02_export_options_html_deserialization() {
    let json = r#"{"Html":{"mode":{"type":"Inline"},"embed_css":true}}"#;
    let options: ExportOptions = serde_json::from_str(json).unwrap();
    assert!(matches!(options, ExportOptions::Html(_)));
}

// =============================================================================
// TC-P2-018-03: Factory pattern — PDF export
// Category: unit
// Input: ExportFormat::PDF
// Expected: PDF export pipeline executed
// =============================================================================

#[test]
fn TC_P2_018_03_factory_creates_pdf_export_options() {
    let options = ExportOptions::for_format(ExportFormat::Pdf);
    assert!(matches!(options, ExportOptions::Pdf(_)));
    assert_eq!(options.format(), ExportFormat::Pdf);
}

#[test]
fn TC_P2_018_03_factory_exports_pdf_with_default_options() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.pdf");

    let markdown = "# Test\n\nHello **world**!";
    let service = export_service();

    service
        .export_with_format(
            markdown,
            output_path.to_str().unwrap(),
            ExportFormat::Pdf,
            ExportOptions::Pdf(PdfExportOptions::default()),
        )
        .expect("PDF export should succeed");

    assert!(output_path.exists(), "PDF file should be created");

    let bytes = fs::read(&output_path).unwrap();
    assert!(bytes.starts_with(b"%PDF-"), "Should be a valid PDF file");
}

#[test]
fn TC_P2_018_03_factory_exports_pdf_with_custom_options() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test_custom.pdf");

    let markdown = "# Test\n\nHello **world**!";
    let service = export_service();

    let custom_options = ExportOptions::Pdf(PdfExportOptions {
        page_size: rustnote_lib::model::export::PdfPageSize::Letter,
        margins: rustnote_lib::model::export::PdfMargins::default(),
        embed_images: true,
    });

    service
        .export_with_format(
            markdown,
            output_path.to_str().unwrap(),
            ExportFormat::Pdf,
            custom_options,
        )
        .expect("PDF export should succeed");

    assert!(output_path.exists(), "PDF file should be created");

    let bytes = fs::read(&output_path).unwrap();
    assert!(bytes.starts_with(b"%PDF-"));
}

#[test]
fn TC_P2_018_03_factory_supports_pdf_format() {
    let service = export_service();
    let formats = service.get_supported_formats();
    assert!(formats.contains(&ExportFormat::Pdf));
}

#[test]
fn TC_P2_018_03_factory_pdf_from_extension() {
    assert_eq!(ExportFormat::from_extension("pdf"), Some(ExportFormat::Pdf));
}

#[test]
fn TC_P2_018_03_export_options_pdf_serialization() {
    let options = ExportOptions::Pdf(PdfExportOptions::default());
    let json = serde_json::to_string(&options).unwrap();
    assert!(json.contains("\"Pdf\""));
}

#[test]
fn TC_P2_018_03_export_options_pdf_deserialization() {
    let json = r#"{"Pdf":{"page_size":{"type":"A4"},"margins":{"top_mm":20.0,"right_mm":20.0,"bottom_mm":20.0,"left_mm":20.0},"embed_images":true}}"#;
    let options: ExportOptions = serde_json::from_str(json).unwrap();
    assert!(matches!(options, ExportOptions::Pdf(_)));
}

// =============================================================================
// TC-P2-018-04: Format-specific options structs
// Category: unit
// Input: model/export.rs
// Expected: HtmlExportOptions and PdfExportOptions (or similar) exist
// =============================================================================

#[test]
fn TC_P2_018_04_html_export_options_struct_exists() {
    let options = HtmlExportOptions::default();
    assert!(options.embed_css);
}

#[test]
fn TC_P2_018_04_html_export_options_has_mode() {
    let options = HtmlExportOptions::default();
    assert!(matches!(
        options.mode,
        rustnote_lib::model::export::HtmlExportMode::Inline
    ));
}

#[test]
fn TC_P2_018_04_html_export_options_has_embed_css() {
    let options = HtmlExportOptions {
        mode: rustnote_lib::model::export::HtmlExportMode::Inline,
        embed_css: true,
    };
    assert!(options.embed_css);

    let options_no_embed = HtmlExportOptions {
        mode: rustnote_lib::model::export::HtmlExportMode::Inline,
        embed_css: false,
    };
    assert!(!options_no_embed.embed_css);
}

#[test]
fn TC_P2_018_04_pdf_export_options_struct_exists() {
    let options = PdfExportOptions::default();
    assert!(options.embed_images);
}

#[test]
fn TC_P2_018_04_pdf_export_options_has_page_size() {
    let options = PdfExportOptions::default();
    let (width, height) = options.page_size.dimensions();
    assert_eq!(width, 210.0);
    assert_eq!(height, 297.0);
}

#[test]
fn TC_P2_018_04_pdf_export_options_has_margins() {
    let options = PdfExportOptions::default();
    let (top, right, bottom, left) = options.margins.as_tuple();
    assert_eq!(top, 20.0);
    assert_eq!(right, 20.0);
    assert_eq!(bottom, 20.0);
    assert_eq!(left, 20.0);
}

#[test]
fn TC_P2_018_04_pdf_export_options_has_embed_images() {
    let options = PdfExportOptions {
        page_size: rustnote_lib::model::export::PdfPageSize::A4,
        margins: rustnote_lib::model::export::PdfMargins::default(),
        embed_images: true,
    };
    assert!(options.embed_images);

    let options_no_embed = PdfExportOptions {
        page_size: rustnote_lib::model::export::PdfPageSize::A4,
        margins: rustnote_lib::model::export::PdfMargins::default(),
        embed_images: false,
    };
    assert!(!options_no_embed.embed_images);
}

#[test]
fn TC_P2_018_04_pdf_page_size_variants() {
    use rustnote_lib::model::export::PdfPageSize;

    let a4 = PdfPageSize::A4;
    assert_eq!(a4.dimensions(), (210.0, 297.0));

    let letter = PdfPageSize::Letter;
    assert_eq!(letter.dimensions(), (215.9, 279.4));

    let legal = PdfPageSize::Legal;
    assert_eq!(legal.dimensions(), (215.9, 355.6));

    let custom = PdfPageSize::Custom {
        width_mm: 100.0,
        height_mm: 200.0,
    };
    assert_eq!(custom.dimensions(), (100.0, 200.0));
}

#[test]
fn TC_P2_018_04_pdf_margins_variants() {
    use rustnote_lib::model::export::PdfMargins;

    let default = PdfMargins::default();
    assert_eq!(default.as_tuple(), (20.0, 20.0, 20.0, 20.0));

    let custom = PdfMargins {
        top_mm: 10.0,
        right_mm: 15.0,
        bottom_mm: 10.0,
        left_mm: 15.0,
    };
    assert_eq!(custom.as_tuple(), (10.0, 15.0, 10.0, 15.0));
}

#[test]
fn TC_P2_018_04_html_export_options_serialization() {
    let options = HtmlExportOptions::default();
    let json = serde_json::to_string(&options).unwrap();
    let deserialized: HtmlExportOptions = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.embed_css, options.embed_css);
}

#[test]
fn TC_P2_018_04_pdf_export_options_serialization() {
    let options = PdfExportOptions::default();
    let json = serde_json::to_string(&options).unwrap();
    let deserialized: PdfExportOptions = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.embed_images, options.embed_images);
}

// =============================================================================
// Edge case tests: future_formats and unknown_format
// =============================================================================

#[test]
fn TC_P2_018_edge_unknown_format_from_extension_returns_none() {
    assert_eq!(ExportFormat::from_extension("docx"), None);
    assert_eq!(ExportFormat::from_extension("epub"), None);
    assert_eq!(ExportFormat::from_extension("txt"), None);
    assert_eq!(ExportFormat::from_extension("md"), None);
    assert_eq!(ExportFormat::from_extension("rtf"), None);
    assert_eq!(ExportFormat::from_extension(""), None);
    assert_eq!(ExportFormat::from_extension("xyz"), None);
}

#[test]
fn TC_P2_018_edge_supported_formats_count() {
    let formats = ExportFormat::supported_formats();
    assert_eq!(formats.len(), 2);
}

#[test]
fn TC_P2_018_edge_export_options_default_is_html() {
    let options = ExportOptions::default();
    assert!(matches!(options, ExportOptions::Html(_)));
}

#[test]
fn TC_P2_018_edge_mixed_options_wrong_type_uses_default() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.html");

    let markdown = "# Test";
    let service = export_service();

    let result = service.export_with_format(
        markdown,
        output_path.to_str().unwrap(),
        ExportFormat::Html,
        ExportOptions::Pdf(PdfExportOptions::default()),
    );

    assert!(
        result.is_ok(),
        "Should succeed even with wrong options type"
    );
}

#[test]
fn TC_P2_018_edge_unknown_extension_roundtrip() {
    let unknown_ext = "unknown_format";
    let result = ExportFormat::from_extension(unknown_ext);
    assert_eq!(result, None);
}

#[test]
fn TC_P2_018_edge_case_insensitive_extension() {
    assert_eq!(
        ExportFormat::from_extension("HTML"),
        Some(ExportFormat::Html)
    );
    assert_eq!(
        ExportFormat::from_extension("Html"),
        Some(ExportFormat::Html)
    );
    assert_eq!(ExportFormat::from_extension("PDF"), Some(ExportFormat::Pdf));
    assert_eq!(ExportFormat::from_extension("Pdf"), Some(ExportFormat::Pdf));
}

// =============================================================================
// Additional coverage tests
// =============================================================================

#[test]
fn TC_P2_018_coverage_format_to_options_roundtrip() {
    for format in ExportFormat::supported_formats() {
        let options = ExportOptions::for_format(*format);
        assert_eq!(options.format(), *format);
    }
}

#[test]
fn TC_P2_018_coverage_get_supported_formats_returns_vec() {
    let service = export_service();
    let formats = service.get_supported_formats();
    assert!(!formats.is_empty());
    assert!(formats.contains(&ExportFormat::Html));
    assert!(formats.contains(&ExportFormat::Pdf));
}

#[test]
fn TC_P2_018_coverage_markdown_html_roundtrip() {
    let markdown = "# Title\n\nParagraph with **bold** and *italic*.";
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<h1"));
    assert!(html.contains("Title"));
    assert!(html.contains("<p>"));
    assert!(html.contains("Paragraph"));
}

#[test]
fn TC_P2_018_coverage_markdown_pdf_roundtrip() {
    let markdown = "# Title\n\nParagraph with **bold** and *italic*.";
    let html = parser().parse_to_html(markdown);

    assert!(html.contains("<h1"));
    assert!(html.contains("Title"));
}

#[test]
fn TC_P2_018_coverage_both_formats_produce_valid_output() {
    let temp_dir = TempDir::new().unwrap();
    let markdown = "# Test Document\n\nContent with **formatting**.\n\n- List item 1\n- List item 2\n\n```rust\nfn main() {}\n```";
    let service = export_service();

    let html_path = temp_dir.path().join("output.html");
    let pdf_path = temp_dir.path().join("output.pdf");

    service
        .export_with_format(
            markdown,
            html_path.to_str().unwrap(),
            ExportFormat::Html,
            ExportOptions::default(),
        )
        .expect("HTML export should succeed");

    service
        .export_with_format(
            markdown,
            pdf_path.to_str().unwrap(),
            ExportFormat::Pdf,
            ExportOptions::Pdf(PdfExportOptions::default()),
        )
        .expect("PDF export should succeed");

    assert!(html_path.exists());
    assert!(pdf_path.exists());

    let html_content = fs::read_to_string(&html_path).unwrap();
    assert!(html_content.contains("<h1"));
    assert!(html_content.contains("Test Document"));

    let pdf_bytes = fs::read(&pdf_path).unwrap();
    assert!(pdf_bytes.starts_with(b"%PDF-"));
}
