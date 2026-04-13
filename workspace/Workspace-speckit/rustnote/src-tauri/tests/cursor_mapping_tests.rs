use rustnote_lib::commands::render::build_cursor_mapping;
use rustnote_lib::semantic::position::CursorMapping;

#[test]
fn test_tc_g004_001_cursor_simple_mapping() {
    let source = "# Title\n\nParagraph";
    let mappings = build_cursor_mapping(source);

    assert!(!mappings.is_empty(), "Should have cursor mappings");

    let html = comrak::markdown_to_html(source, &comrak::Options::default());

    for mapping in &mappings {
        let computed_dom = CursorMapping::source_to_dom(&mappings, mapping.source_offset);
        assert!(
            computed_dom <= html.len(),
            "DOM offset {} should be within HTML length {} for source offset {}",
            computed_dom,
            html.len(),
            mapping.source_offset
        );
    }
}

#[test]
fn test_tc_g004_002_cursor_with_html_tags() {
    let source = "**bold**|";
    let mappings = build_cursor_mapping(source);

    assert!(mappings.len() > 2, "Should have mappings for bold text");

    let html = comrak::markdown_to_html(source, &comrak::Options::default());
    assert!(
        html.contains("<strong>bold</strong>|"),
        "HTML should contain <strong>bold</strong>|"
    );

    let dom_after_bold = CursorMapping::source_to_dom(&mappings, 7);
    assert!(
        dom_after_bold <= html.len(),
        "Cursor after bold should be within HTML bounds, got {} but HTML len is {}",
        dom_after_bold,
        html.len()
    );
}

#[test]
fn test_tc_g004_003_cursor_nested_formatting() {
    let source = "***bold italic***|";
    let mappings = build_cursor_mapping(source);

    assert!(
        mappings.len() > 3,
        "Should have mappings for nested formatting"
    );

    let dom_offset = CursorMapping::source_to_dom(&mappings, 16);
    let html = comrak::markdown_to_html(source, &comrak::Options::default());

    assert!(
        dom_offset <= html.len(),
        "DOM offset {} should be within HTML length {}",
        dom_offset,
        html.len()
    );
}

#[test]
fn test_tc_g004_004_cursor_preserved_on_edit() {
    let source = "**bold**|";
    let mappings = build_cursor_mapping(source);

    let original_dom = CursorMapping::source_to_dom(&mappings, 7);
    assert!(original_dom <= comrak::markdown_to_html(source, &comrak::Options::default()).len());

    let edited_source = "**bold text**|";
    let edited_mappings = build_cursor_mapping(&edited_source);
    let edited_dom = CursorMapping::source_to_dom(&edited_mappings, 10);

    assert_ne!(
        original_dom, edited_dom,
        "DOM position should change after edit"
    );
}

#[test]
fn test_tc_g004_005_cursor_large_document() {
    use std::time::Instant;

    let mut large_source = String::new();
    for i in 0..1000 {
        large_source.push_str(&format!("# Heading {}\n\n", i));
        large_source.push_str(&format!("Paragraph with **bold** and *italic* text.\n\n"));
        large_source.push_str("- Item 1\n- Item 2\n- Item 3\n\n");
    }

    let start = Instant::now();
    let mappings = build_cursor_mapping(&large_source);
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 200,
        "Cursor mapping should complete in <200ms, took {}ms",
        elapsed.as_millis()
    );

    assert!(
        !mappings.is_empty(),
        "Should have mappings for large document"
    );

    let html = comrak::markdown_to_html(&large_source, &comrak::Options::default());
    for mapping in mappings.iter().take(100) {
        let dom = CursorMapping::source_to_dom(&mappings, mapping.source_offset);
        assert!(
            dom <= html.len(),
            "DOM offset {} should be within HTML length {}",
            dom,
            html.len()
        );
    }
}
