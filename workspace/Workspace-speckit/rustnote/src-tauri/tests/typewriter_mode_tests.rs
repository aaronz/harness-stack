use rustnote_lib::semantic::ast::SemanticDocument;

#[test]
fn tc_g014_001_typewriter_cursor_center() {
    let source = "Paragraph 1\n\nParagraph 2\n\nParagraph 3\n\nParagraph 4\n\nParagraph 5";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 5, "Should have 5 paragraphs");

    let middle_offset = paragraphs[2].offset;
    let active_middle = doc.get_paragraph_at(middle_offset);
    assert_eq!(
        active_middle, 2,
        "Cursor at middle paragraph should be detected for centering"
    );

    let first_offset = paragraphs[0].offset;
    let active_first = doc.get_paragraph_at(first_offset);
    assert_eq!(
        active_first, 0,
        "Cursor at first paragraph should be detected"
    );

    let last_offset = paragraphs[4].offset;
    let active_last = doc.get_paragraph_at(last_offset);
    assert_eq!(
        active_last, 4,
        "Cursor at last paragraph should be detected"
    );
}

#[test]
fn tc_g014_002_typewriter_navigation() {
    let source = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert!(
        paragraphs.len() >= 3,
        "Should have paragraphs for navigation"
    );

    let transitions = [
        (paragraphs[0].offset, 0),
        (paragraphs[1].offset, 1),
        (paragraphs[2].offset, 2),
    ];

    for (offset, expected_idx) in transitions {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, expected_idx,
            "Navigation to offset {} should give paragraph {}",
            offset, expected_idx
        );
    }
}

#[test]
fn tc_g014_003_typewriter_long_doc() {
    let mut source = String::new();
    for i in 1..=200 {
        if i > 1 {
            source.push_str("\n\n");
        }
        source.push_str(&format!("Line {}", i));
    }

    let doc = SemanticDocument::parse(&source);
    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        200,
        "Should have 200 paragraphs in long document"
    );

    let first_offset = paragraphs[0].offset;
    let active_first = doc.get_paragraph_at(first_offset);
    assert_eq!(active_first, 0, "First paragraph should be index 0");

    let middle_offset = paragraphs[99].offset;
    let active_middle = doc.get_paragraph_at(middle_offset);
    assert_eq!(
        active_middle, 99,
        "Middle paragraph should be detected correctly"
    );

    let last_offset = paragraphs[199].offset;
    let active_last = doc.get_paragraph_at(last_offset);
    assert_eq!(active_last, 199, "Last paragraph should be index 199");
}

#[test]
fn tc_g014_edge_document_boundaries_first() {
    let source = "First paragraph only";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 1, "Should have 1 paragraph");

    let first_offset = paragraphs[0].offset;
    let active = doc.get_paragraph_at(first_offset);
    assert_eq!(active, 0, "First and only paragraph should be index 0");

    let offset_at_start = 0;
    let active_at_start = doc.get_paragraph_at(offset_at_start);
    assert_eq!(
        active_at_start, 0,
        "Offset at start should map to first paragraph"
    );
}

#[test]
fn tc_g014_edge_document_boundaries_last() {
    let source = "First paragraph\n\nSecond paragraph\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let _paragraphs = doc.get_paragraphs();
    let last_offset = doc.source().len();
    let active = doc.get_paragraph_at(last_offset);
    assert_eq!(active, 2, "Cursor at end should map to last paragraph");
}

#[test]
fn tc_g014_edge_rapid_typing_position_tracking() {
    let source = "A\n\nB\n\nC\n\nD\n\nE\n\nF\n\nG\n\nH";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 8, "Should have 8 paragraphs");

    for (i, para) in paragraphs.iter().enumerate() {
        let active = doc.get_paragraph_at(para.offset);
        assert_eq!(
            active, i,
            "Rapid position tracking: offset {} should map to paragraph {}",
            para.offset, i
        );
    }
}

#[test]
fn tc_g014_edge_scroll_performance_large_doc() {
    let mut source = String::new();
    for i in 1..=500 {
        if i > 1 {
            source.push_str("\n\n");
        }
        source.push_str(&format!("Performance paragraph {}", i));
    }

    let doc = SemanticDocument::parse(&source);
    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        500,
        "Should have 500 paragraphs for performance test"
    );

    let start_time = std::time::Instant::now();
    for i in [0, 100, 250, 400, 499].iter() {
        let offset = paragraphs[*i].offset;
        let _active = doc.get_paragraph_at(offset);
    }
    let elapsed = start_time.elapsed();

    assert!(
        elapsed.as_millis() < 100,
        "Paragraph lookup for 500 paragraphs should be under 100ms, was {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn tc_g014_001_paragraph_with_headings_typewriter() {
    let source = "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\nContent paragraph 1\n\nContent paragraph 2";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        2,
        "Should have 2 content paragraphs (headings are not paragraphs)"
    );

    let offsets: Vec<usize> = paragraphs.iter().map(|p| p.offset).collect();

    for (i, offset) in offsets.iter().enumerate() {
        let active = doc.get_paragraph_at(*offset);
        assert_eq!(
            active, i,
            "Offset at block {} should give paragraph index {}",
            i, i
        );
    }
}

#[test]
fn tc_g014_002_mixed_content_typewriter() {
    let source = "# Title\n\n> A blockquote\n\n- List item 1\n- List item 2\n\nRegular paragraph\n\nAnother paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        2,
        "Should have 2 regular paragraphs among mixed content"
    );

    let para1_offset = paragraphs[0].offset;
    let para2_offset = paragraphs[1].offset;

    let active1 = doc.get_paragraph_at(para1_offset);
    let active2 = doc.get_paragraph_at(para2_offset);

    assert_eq!(active1, 0, "First paragraph should be index 0");
    assert_eq!(active2, 1, "Second paragraph should be index 1");
}

#[test]
fn tc_g014_003_empty_lines_typewriter() {
    let source = "Paragraph 1\n\n\n\nParagraph 2\n\n\n\n\nParagraph 3";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        3,
        "Should have 3 paragraphs with multiple empty lines between"
    );

    let offsets: Vec<usize> = paragraphs.iter().map(|p| p.offset).collect();

    for (i, offset) in offsets.iter().enumerate() {
        let active = doc.get_paragraph_at(*offset);
        assert_eq!(
            active, i,
            "Multiple empty lines: offset {} should give paragraph index {}",
            offset, i
        );
    }
}
