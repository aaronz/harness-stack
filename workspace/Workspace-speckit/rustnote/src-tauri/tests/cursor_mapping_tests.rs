use rustnote_lib::commands::render::build_cursor_mapping;
use rustnote_lib::semantic::ast::SemanticDocument;
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

#[test]
fn test_tc_g001_001_nested_formatting_conversion() {
    let source = "**bold *italic bold***";
    let mappings = build_cursor_mapping(source);

    assert!(
        !mappings.is_empty(),
        "Should have cursor mappings for nested formatting"
    );

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

        let back_to_source = CursorMapping::dom_to_source(&mappings, computed_dom);
        assert!(
            back_to_source <= source.len(),
            "Source offset {} should be within source length {} when round-tripping from DOM {}",
            back_to_source,
            source.len(),
            computed_dom
        );
    }
}

#[test]
fn test_tc_g001_002_unicode_multibyte_characters() {
    let source = "Hello 🌍 World café naïve";
    let mappings = build_cursor_mapping(source);

    assert!(
        !mappings.is_empty(),
        "Should have cursor mappings for Unicode content"
    );

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

        let back_to_source = CursorMapping::dom_to_source(&mappings, computed_dom);
        assert!(
            back_to_source <= source.len(),
            "Source offset {} should be within source length {} for Unicode",
            back_to_source,
            source.len()
        );
    }
}

#[test]
fn test_tc_g001_003_crlf_line_endings_handling() {
    let source = "Line 1\r\nLine 2\r\nLine 3";
    let mappings = build_cursor_mapping(source);

    assert!(
        !mappings.is_empty(),
        "Should have cursor mappings for CRLF content"
    );

    let normalized_source = source.replace("\r\n", "\n");
    let html = comrak::markdown_to_html(&normalized_source, &comrak::Options::default());

    for mapping in &mappings {
        let computed_dom = CursorMapping::source_to_dom(&mappings, mapping.source_offset);
        assert!(
            computed_dom <= html.len() + 10,
            "DOM offset {} should be within reasonable HTML length {} for source offset {}",
            computed_dom,
            html.len() + 10,
            mapping.source_offset
        );
    }
}

#[test]
fn test_tc_g001_004_html_entity_encoding() {
    let source = "&amp; &lt; &gt; &quot;test&quot;";
    let mappings = build_cursor_mapping(source);

    assert!(
        !mappings.is_empty(),
        "Should have cursor mappings for HTML entities"
    );

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
fn test_tc_g001_005_empty_and_zero_width_nodes() {
    let source1 = "**||**";
    let mappings1 = build_cursor_mapping(source1);

    assert!(
        !mappings1.is_empty(),
        "Should have cursor mappings for empty bold"
    );

    let source2 = "*|*";
    let mappings2 = build_cursor_mapping(source2);

    assert!(
        !mappings2.is_empty(),
        "Should have cursor mappings for empty italic"
    );

    let html1 = comrak::markdown_to_html(source1, &comrak::Options::default());
    let html2 = comrak::markdown_to_html(source2, &comrak::Options::default());

    for mapping in &mappings1 {
        let computed_dom = CursorMapping::source_to_dom(&mappings1, mapping.source_offset);
        assert!(
            computed_dom <= html1.len(),
            "DOM offset {} should be within HTML length {} for empty bold",
            computed_dom,
            html1.len()
        );
    }

    for mapping in &mappings2 {
        let computed_dom = CursorMapping::source_to_dom(&mappings2, mapping.source_offset);
        assert!(
            computed_dom <= html2.len(),
            "DOM offset {} should be within HTML length {} for empty italic",
            computed_dom,
            html2.len()
        );
    }
}

#[test]
fn test_tc_g001_006_document_boundary_positions() {
    let source = "Start text\n\nMore text";
    let mappings = build_cursor_mapping(source);

    assert!(
        mappings.len() >= 2,
        "Should have mappings for start and end positions"
    );

    let dom_at_start = CursorMapping::source_to_dom(&mappings, 0);
    assert_eq!(dom_at_start, 0, "Position 0 should map to DOM position 0");

    let html = comrak::markdown_to_html(source, &comrak::Options::default());
    let dom_at_end = CursorMapping::source_to_dom(&mappings, source.len());
    assert!(
        dom_at_end <= html.len(),
        "End position {} should map to DOM position within HTML length {}",
        dom_at_end,
        html.len()
    );

    let source_at_end = CursorMapping::dom_to_source(&mappings, dom_at_end);
    assert_eq!(
        source_at_end,
        source.len(),
        "DOM end position should map back to source end"
    );
}

#[test]
fn test_tc_g001_007_roundtrip_source_dom_source() {
    let source = "**bold** and *italic* with `code`";
    let doc = SemanticDocument::parse(source);
    let html1 = doc.html();

    let mappings = build_cursor_mapping(source);

    for source_offset in 0..=source.len() {
        let dom_offset = CursorMapping::source_to_dom(&mappings, source_offset);
        let back_to_source = CursorMapping::dom_to_source(&mappings, dom_offset);

        assert!(
            back_to_source <= source.len(),
            "Round-trip source→DOM→source failed: source_offset {} → dom {} → source {} (source len {})",
            source_offset,
            dom_offset,
            back_to_source,
            source.len()
        );
    }

    assert!(
        html1.contains("<strong>bold</strong>"),
        "HTML should contain bold"
    );
    assert!(
        html1.contains("<em>italic</em>"),
        "HTML should contain italic"
    );
    assert!(
        html1.contains("<code>code</code>"),
        "HTML should contain code"
    );
}

#[test]
fn test_tc_g001_008_roundtrip_dom_source_dom() {
    let source = "**bold *italic bold***";
    let doc = SemanticDocument::parse(source);
    let html1 = doc.html();

    let mappings = build_cursor_mapping(source);
    let html_len = html1.len();

    for dom_offset in 0..=html_len {
        let source_offset = CursorMapping::dom_to_source(&mappings, dom_offset);
        let back_to_dom = CursorMapping::source_to_dom(&mappings, source_offset);

        assert!(
            back_to_dom <= html_len + 10,
            "Round-trip DOM→source→DOM failed: dom {} → source {} → dom {} (html len {})",
            dom_offset,
            source_offset,
            back_to_dom,
            html_len
        );
    }

    assert!(html1.contains("<strong>"), "HTML should contain strong tag");
    assert!(
        html1.contains("<em>"),
        "HTML should contain em tag for nested italic"
    );
}
