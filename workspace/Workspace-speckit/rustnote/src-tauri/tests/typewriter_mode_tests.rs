use rustnote_lib::semantic::ast::SemanticDocument;

// =============================================================================
// TC-G011: Typewriter Mode Scroll Verification Tests
// =============================================================================
// These tests verify the backend support for typewriter mode scroll behavior.
// The frontend uses SemanticDocument::get_paragraph_at() to determine which
// paragraph to scroll to the center of the viewport.
//
// Typewriter Mode Requirement (FR-029):
// - Active line stays at vertical center of viewport during navigation
// - Works with various document lengths
// - Smooth scroll behavior
// =============================================================================

// -----------------------------------------------------------------------------
// TC-G011-001: Typewriter scroll position
// Category: render
// Input: Type in document with typewriter mode enabled
// Expected: Active line stays at vertical center of viewport
// -----------------------------------------------------------------------------

#[test]
fn tc_g011_001_typewriter_scroll_position() {
    // Test that paragraph detection works correctly for typewriter scroll positioning
    // When the user is typing, the cursor position should correctly identify
    // which paragraph needs to be centered for typewriter mode

    let source = "Paragraph 1\n\nParagraph 2\n\nParagraph 3\n\nParagraph 4\n\nParagraph 5";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        5,
        "Should have 5 paragraphs for scroll position test"
    );

    // Test that each paragraph's offset correctly maps back to itself
    // This is essential for typewriter mode to know which paragraph to center
    for (i, para) in paragraphs.iter().enumerate() {
        let detected_index = doc.get_paragraph_at(para.offset);
        assert_eq!(
            detected_index, i,
            "Paragraph {} offset {} should map to index {} for centering",
            i, para.offset, i
        );
    }

    // Test cursor positions within each paragraph (not exactly at paragraph start)
    // Typewriter mode should center the active paragraph even when cursor
    // is mid-paragraph
    if paragraphs.len() >= 3 {
        let para_mid_offset = paragraphs[2].offset + 5; // 5 chars into paragraph
        let active = doc.get_paragraph_at(para_mid_offset);
        assert_eq!(
            active, 2,
            "Cursor mid-paragraph should still center on that paragraph"
        );
    }
}

// -----------------------------------------------------------------------------
// TC-G011-002: Navigation with typewriter mode
// Category: render
// Input: Press up/down arrows with typewriter mode
// Expected: Cursor stays at vertical center, document scrolls
// -----------------------------------------------------------------------------

#[test]
fn tc_g011_002_typewriter_navigation() {
    // Test that navigation through paragraphs works correctly for typewriter mode
    // When user presses up/down arrows, the paragraph index should update
    // correctly so the frontend can scroll to center the new cursor position

    let source = "Line 1 content here\n\nLine 2 content here\n\nLine 3 content here\n\nLine 4 content here\n\nLine 5 content here";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert!(
        paragraphs.len() >= 5,
        "Should have at least 5 paragraphs for navigation test"
    );

    // Simulate navigation from first to last paragraph
    // This represents pressing Down arrow repeatedly in typewriter mode
    let navigation_sequence = [
        (0, "First paragraph"),
        (1, "Second paragraph"),
        (2, "Third paragraph"),
        (3, "Fourth paragraph"),
        (4, "Fifth paragraph"),
    ];

    for (expected_idx, desc) in navigation_sequence.iter() {
        let offset = paragraphs[*expected_idx].offset;
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, *expected_idx as usize,
            "Navigation to {} should map to paragraph index {}",
            desc, expected_idx
        );
    }

    // Test reverse navigation (pressing Up arrow)
    for (rev_pos, &(idx, _desc)) in navigation_sequence.iter().rev().enumerate() {
        let actual_idx = 4 - rev_pos;
        let offset = paragraphs[actual_idx].offset;
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, actual_idx,
            "Reverse navigation position {} should map to paragraph index {}",
            rev_pos, actual_idx
        );
    }
}

// -----------------------------------------------------------------------------
// TC-G011-003: Typewriter with long document
// Category: render
// Input: Navigate long document with typewriter mode
// Expected: Smooth scrolling, cursor always centered
// -----------------------------------------------------------------------------

#[test]
fn tc_g011_003_typewriter_long_document() {
    // Test that paragraph detection works correctly for long documents
    // This is critical for smooth scrolling behavior in typewriter mode

    // Create a long document with 200 paragraphs
    let mut source = String::new();
    for i in 1..=200 {
        if i > 1 {
            source.push_str("\n\n");
        }
        source.push_str(&format!("Long document paragraph {}", i));
    }

    let doc = SemanticDocument::parse(&source);
    let paragraphs = doc.get_paragraphs();

    assert_eq!(
        paragraphs.len(),
        200,
        "Long document should have 200 paragraphs for smooth scroll test"
    );

    // Test key positions for centering verification
    let test_positions = [
        (0, "First paragraph"),
        (49, "Quarter position"),
        (99, "Middle paragraph"),
        (149, "Three-quarter position"),
        (199, "Last paragraph"),
    ];

    for (idx, desc) in test_positions.iter() {
        let offset = paragraphs[*idx].offset;
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, *idx,
            "{} at index {} should be detected correctly for centering",
            desc, idx
        );
    }

    // Test that within-paragraph cursor positions also work
    let mid_doc_offset = paragraphs[100].offset + 10; // 10 chars into paragraph 101
    let active_mid = doc.get_paragraph_at(mid_doc_offset);
    assert_eq!(
        active_mid, 100,
        "Cursor mid-paragraph in long doc should still center on paragraph 100"
    );
}

// -----------------------------------------------------------------------------
// TC-G011 Edge Cases: Scroll Anchoring
// -----------------------------------------------------------------------------

#[test]
fn tc_g011_edge_scroll_anchoring_first_paragraph() {
    // Edge case: Scroll anchoring at document start
    // When cursor is in first paragraph, it should still be detected correctly
    // even though there's no paragraph above to scroll past

    let source = "First paragraph\n\nSecond paragraph\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 3, "Should have 3 paragraphs");

    // Cursor at very start of document
    let active_at_start = doc.get_paragraph_at(0);
    assert_eq!(
        active_at_start, 0,
        "Cursor at document start should map to first paragraph for centering"
    );

    // Cursor within first paragraph
    let first_para_end = paragraphs[0].offset + paragraphs[0].length;
    let active_near_end = doc.get_paragraph_at(first_para_end.saturating_sub(1));
    assert_eq!(
        active_near_end, 0,
        "Cursor near end of first paragraph should still center first paragraph"
    );
}

#[test]
fn tc_g011_edge_scroll_anchoring_last_paragraph() {
    // Edge case: Scroll anchoring at document end
    // When cursor is in last paragraph, it should still be detected correctly

    let source = "First paragraph\n\nSecond paragraph\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();

    // Cursor at end of document (past all content)
    let end_offset = doc.source().len();
    let active_at_end = doc.get_paragraph_at(end_offset);
    assert_eq!(
        active_at_end, 2,
        "Cursor at document end should map to last paragraph for centering"
    );

    // Cursor within last paragraph
    let last_offset = paragraphs[2].offset;
    let active_last = doc.get_paragraph_at(last_offset);
    assert_eq!(
        active_last, 2,
        "Cursor at start of last paragraph should center last paragraph"
    );
}

#[test]
fn tc_g011_edge_long_document_performance() {
    // Performance test for long document navigation
    // Smooth scrolling requires paragraph detection to be fast

    let mut source = String::new();
    for i in 1..=500 {
        if i > 1 {
            source.push_str("\n\n");
        }
        source.push_str(&format!("Performance test paragraph {}", i));
    }

    let doc = SemanticDocument::parse(&source);
    let paragraphs = doc.get_paragraphs();

    assert_eq!(
        paragraphs.len(),
        500,
        "Should have 500 paragraphs for performance test"
    );

    // Test random access pattern that simulates scrolling through document
    let start_time = std::time::Instant::now();
    let scroll_positions = [0, 50, 100, 150, 200, 250, 300, 350, 400, 450, 499];

    for idx in scroll_positions.iter() {
        let offset = paragraphs[*idx].offset;
        let _active = doc.get_paragraph_at(offset);
    }

    let elapsed = start_time.elapsed();

    // Paragraph lookup should be very fast for smooth scrolling
    assert!(
        elapsed.as_millis() < 50,
        "Paragraph lookup for 500 paragraphs should be under 50ms for smooth scroll, was {}ms",
        elapsed.as_millis()
    );
}

#[test]
fn tc_g011_001_typewriter_cursor_center() {
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
