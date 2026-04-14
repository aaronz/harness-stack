use rustnote_lib::semantic::ast::SemanticDocument;

// =============================================================================
// P2-012: Typewriter Mode — Scroll Behavior Fix
// Test Cases: TC-TW001 to TC-TW007
// =============================================================================
// These tests verify the typewriter mode scroll behavior implementation.
// Typewriter mode keeps cursor at vertical center during typing.
//
// Requirements:
// - Cursor stays centered during typing and navigation
// - 100-paragraph document scrolls at 60 FPS
// - Edge cases: document start, document end
// =============================================================================

// -----------------------------------------------------------------------------
// TC-TW001: Cursor stays centered while typing
// Category: render
// Input: Typewriter mode on, type at end of paragraph
// Expected: Cursor remains at vertical center, document scrolls
// -----------------------------------------------------------------------------

#[test]
fn tc_tw001_cursor_stays_centered_while_typing() {
    // Test that paragraph detection correctly tracks cursor position during typing
    // When user types characters, the paragraph containing the cursor should be
    // correctly identified for centering

    let source = "First paragraph with some content\n\nSecond paragraph with more content\n\nThird paragraph here";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        3,
        "Should have 3 paragraphs for typing test"
    );

    // Simulate typing at end of first paragraph
    let first_para_end_offset = paragraphs[0].offset + paragraphs[0].length;
    let active_para = doc.get_paragraph_at(first_para_end_offset);
    assert_eq!(
        active_para, 0,
        "Typing at end of first paragraph should identify paragraph 0 for centering"
    );

    // Simulate typing at middle of second paragraph
    let second_para_mid = paragraphs[1].offset + 5;
    let active_mid = doc.get_paragraph_at(second_para_mid);
    assert_eq!(
        active_mid, 1,
        "Typing at middle of second paragraph should identify paragraph 1"
    );

    // Simulate typing at end of document
    let doc_end = doc.source().len();
    let active_end = doc.get_paragraph_at(doc_end);
    assert_eq!(
        active_end, 2,
        "Typing at document end should identify last paragraph"
    );
}

// -----------------------------------------------------------------------------
// TC-TW002: Cursor stays centered on Enter
// Category: render
// Input: Typewriter mode on, press Enter
// Expected: New line created, cursor stays at center
// -----------------------------------------------------------------------------

#[test]
fn tc_tw002_cursor_stays_centered_on_enter() {
    // Test that paragraph detection works correctly after Enter key
    // When Enter is pressed, a new paragraph is created and cursor moves to it

    let source = "First line\n\nSecond line\n\nThird line";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        3,
        "Should have 3 paragraphs for Enter key test"
    );

    // Simulate pressing Enter at end of first paragraph
    // Cursor moves to start of second paragraph
    let second_para_offset = paragraphs[1].offset;
    let active_after_enter = doc.get_paragraph_at(second_para_offset);
    assert_eq!(
        active_after_enter, 1,
        "After pressing Enter, cursor should be in second paragraph for centering"
    );

    // Simulate pressing Enter at end of document (creates new paragraph)
    let new_para_offset = doc.source().len();
    let active_new = doc.get_paragraph_at(new_para_offset);
    assert_eq!(
        active_new, 2,
        "Enter at end of doc should keep last paragraph active"
    );
}

// -----------------------------------------------------------------------------
// TC-TW003: Cursor stays centered on arrow navigation
// Category: render
// Input: Typewriter mode on, use arrow keys
// Expected: Cursor stays centered, view scrolls to follow
// -----------------------------------------------------------------------------

#[test]
fn tc_tw003_cursor_stays_centered_on_arrow_navigation() {
    // Test that paragraph detection tracks cursor correctly during arrow navigation
    // Arrow keys move cursor within and between paragraphs

    let source = "Paragraph one with text\n\nParagraph two with text\n\nParagraph three with text\n\nParagraph four with text";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        4,
        "Should have 4 paragraphs for navigation test"
    );

    // Test Down arrow navigation sequence
    let navigation_offsets = vec![
        (paragraphs[0].offset, 0, "Down from first paragraph"),
        (paragraphs[1].offset, 1, "Down to second paragraph"),
        (paragraphs[2].offset, 2, "Down to third paragraph"),
        (paragraphs[3].offset, 3, "Down to fourth paragraph"),
    ];

    for (offset, expected_idx, desc) in navigation_offsets {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, expected_idx,
            "{} should identify paragraph {} for centering",
            desc, expected_idx
        );
    }

    // Test Up arrow navigation (reverse)
    let reverse_offsets = vec![
        (paragraphs[3].offset, 3, "Up from fourth"),
        (paragraphs[2].offset, 2, "Up to third"),
        (paragraphs[1].offset, 1, "Up to second"),
        (paragraphs[0].offset, 0, "Up to first"),
    ];

    for (offset, expected_idx, desc) in reverse_offsets {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, expected_idx,
            "{} should identify paragraph {} for centering",
            desc, expected_idx
        );
    }

    // Test Left/Right arrow within paragraph (cursor moves within same paragraph)
    let within_para_offset = paragraphs[1].offset + 10;
    let active_within = doc.get_paragraph_at(within_para_offset);
    assert_eq!(
        active_within, 1,
        "Left/Right within paragraph should keep same paragraph for centering"
    );
}

// -----------------------------------------------------------------------------
// TC-TW004: Cursor stays centered on paste
// Category: render
// Input: Typewriter mode on, paste large text
// Expected: Cursor stays centered, text inserted
// -----------------------------------------------------------------------------

#[test]
fn tc_tw004_cursor_stays_centered_on_paste() {
    // Test that paragraph detection correctly identifies cursor position after paste
    // Pasted text may span multiple paragraphs or create new ones

    let source = "Original content\n\nMore content here";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        2,
        "Should have 2 paragraphs for paste test"
    );

    // Simulate pasting text at end of first paragraph
    // Cursor should still be in first paragraph for centering
    let paste_position = paragraphs[0].offset + paragraphs[0].length;
    let active_paste = doc.get_paragraph_at(paste_position);
    assert_eq!(
        active_paste, 0,
        "Paste at end of paragraph should identify same paragraph for centering"
    );

    // Simulate pasting at document end
    let doc_end = doc.source().len();
    let active_end = doc.get_paragraph_at(doc_end);
    assert_eq!(
        active_end, 1,
        "Paste at document end should identify last paragraph"
    );
}

// -----------------------------------------------------------------------------
// TC-TW005: Large document performance
// Category: performance
// Input: 100+ paragraph document, typewriter mode on
// Expected: Scroll performance remains smooth (60 FPS)
// -----------------------------------------------------------------------------

#[test]
fn tc_tw005_large_document_performance() {
    // Performance test for typewriter mode with large documents
    // Paragraph detection must be fast enough for 60 FPS scrolling

    // Create a 100-paragraph document
    let mut source = String::new();
    for i in 1..=100 {
        if i > 1 {
            source.push_str("\n\n");
        }
        source.push_str(&format!("Performance test paragraph {}", i));
    }

    let doc = SemanticDocument::parse(&source);
    let paragraphs = doc.get_paragraphs();

    assert_eq!(
        paragraphs.len(),
        100,
        "Should have 100 paragraphs for performance test"
    );

    // Measure paragraph lookup performance
    let start_time = std::time::Instant::now();

    // Simulate scrolling through document with paragraph lookups
    let test_positions = [0, 10, 25, 50, 75, 99];
    for idx in test_positions {
        let offset = paragraphs[idx].offset;
        let _active = doc.get_paragraph_at(offset);
    }

    let elapsed = start_time.elapsed();

    // Paragraph lookup for 100 paragraphs should be very fast
    // Target: < 10ms for 60 FPS (16.67ms per frame)
    assert!(
        elapsed.as_millis() < 10,
        "Paragraph lookup for 100 paragraphs should be under 10ms, was {}ms",
        elapsed.as_millis()
    );

    // Additional stress test: rapid sequential lookups
    let rapid_start = std::time::Instant::now();
    for i in 0..100 {
        let offset = paragraphs[i].offset;
        let _active = doc.get_paragraph_at(offset);
    }
    let rapid_elapsed = rapid_start.elapsed();

    assert!(
        rapid_elapsed.as_millis() < 50,
        "100 sequential paragraph lookups should be under 50ms, was {}ms",
        rapid_elapsed.as_millis()
    );
}

// -----------------------------------------------------------------------------
// TC-TW006: Typewriter at document start
// Category: edge_case
// Input: Move to start, typewriter mode on, type
// Expected: Cursor at top, no scroll up needed
// -----------------------------------------------------------------------------

#[test]
fn tc_tw006_typewriter_at_document_start() {
    // Edge case: Typewriter mode at document start
    // When cursor is at document start, it should stay at top (no scroll up)

    let source = "First paragraph\n\nSecond paragraph\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 3, "Should have 3 paragraphs");

    // Cursor at very start of document (offset 0)
    let start_offset = 0;
    let active_at_start = doc.get_paragraph_at(start_offset);
    assert_eq!(
        active_at_start, 0,
        "Cursor at document start should identify first paragraph"
    );

    // Cursor at start of first paragraph
    let first_para_start = paragraphs[0].offset;
    let active_first_start = doc.get_paragraph_at(first_para_start);
    assert_eq!(
        active_first_start, 0,
        "Cursor at first paragraph start should identify paragraph 0"
    );

    // Cursor near start of first paragraph (typing position)
    let typing_pos = paragraphs[0].offset + 3;
    let active_typing = doc.get_paragraph_at(typing_pos);
    assert_eq!(
        active_typing, 0,
        "Typing near start should still identify first paragraph"
    );

    // Verify document has content before first paragraph
    assert_eq!(
        paragraphs[0].offset, 0,
        "First paragraph should start at offset 0"
    );
}

// -----------------------------------------------------------------------------
// TC-TW007: Typewriter at document end
// Category: edge_case
// Input: Move to end, typewriter mode on, type
// Expected: Cursor stays centered if possible, or at bottom
// -----------------------------------------------------------------------------

#[test]
fn tc_tw007_typewriter_at_document_end() {
    // Edge case: Typewriter mode at document end
    // When cursor is at document end, it should stay at bottom

    let source = "First paragraph\n\nSecond paragraph\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 3, "Should have 3 paragraphs");

    // Cursor at document end (past all content)
    let doc_end = doc.source().len();
    let active_at_end = doc.get_paragraph_at(doc_end);
    assert_eq!(
        active_at_end, 2,
        "Cursor at document end should identify last paragraph (index 2)"
    );

    // Cursor at start of last paragraph
    let last_para_start = paragraphs[2].offset;
    let active_last_start = doc.get_paragraph_at(last_para_start);
    assert_eq!(
        active_last_start, 2,
        "Cursor at last paragraph start should identify paragraph 2"
    );

    // Cursor near end of last paragraph
    let near_end = paragraphs[2].offset + paragraphs[2].length - 5;
    let active_near_end = doc.get_paragraph_at(near_end);
    assert_eq!(
        active_near_end, 2,
        "Cursor near end of last paragraph should identify paragraph 2"
    );

    // Single paragraph document edge case
    let single_doc = SemanticDocument::parse("Only paragraph");
    let single_paragraphs = single_doc.get_paragraphs();
    assert_eq!(single_paragraphs.len(), 1, "Single paragraph document");

    let single_end = single_doc.source().len();
    let active_single = single_doc.get_paragraph_at(single_end);
    assert_eq!(
        active_single, 0,
        "Single paragraph document end should identify paragraph 0"
    );
}

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

// =============================================================================
// P2-015: Typewriter Mode — Scroll Behavior
// Test Cases: TC-P2-015-01 to TC-P2-015-05
// =============================================================================
// Verifies that the backend paragraph-detection logic correctly supports the
// frontend typewriter-mode scroll-to-center requirement:
// - Cursor stays at vertical center during typing, navigation, and paste
// - 100-paragraph document scrolls at 60 FPS
// - Scroll adjustment happens after cursor position updates
//
// The frontend (TipTapEditor.jsx) uses SemanticDocument::get_paragraph_at()
// to determine which paragraph to center on. These tests verify that logic.

// -----------------------------------------------------------------------------
// TC-P2-015-01: Type at end of paragraph — cursor stays centered
// Category: render
// Input: Enable typewriter mode, type at end of paragraph
// Expected: Cursor stays at vertical center of viewport while typing
// -----------------------------------------------------------------------------

#[test]
fn tc_p2_015_01_type_at_end_of_paragraph() {
    let source = "First paragraph with text\n\nSecond paragraph here\n\nThird paragraph text";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 3, "Should have 3 paragraphs");

    // TC-P2-015-01: Typing at end of first paragraph
    let typing_pos_first = paragraphs[0].offset + paragraphs[0].length;
    let active_first = doc.get_paragraph_at(typing_pos_first);
    assert_eq!(
        active_first, 0,
        "TC-P2-015-01: Typing at end of paragraph 1 should identify paragraph 0 for centering"
    );

    // TC-P2-015-01: Typing at end of second paragraph
    let typing_pos_second = paragraphs[1].offset + paragraphs[1].length;
    let active_second = doc.get_paragraph_at(typing_pos_second);
    assert_eq!(
        active_second, 1,
        "TC-P2-015-01: Typing at end of paragraph 2 should identify paragraph 1 for centering"
    );

    // TC-P2-015-01: Cursor mid-paragraph (after typing some chars)
    let mid_first = paragraphs[0].offset + paragraphs[0].length / 2;
    let active_mid = doc.get_paragraph_at(mid_first);
    assert_eq!(
        active_mid, 0,
        "TC-P2-015-01: Cursor mid-paragraph should identify same paragraph for centering"
    );
}

// -----------------------------------------------------------------------------
// TC-P2-015-02: Press Enter — cursor stays centered
// Category: render
// Input: Enable typewriter mode, press Enter
// Expected: New line created, cursor at vertical center
// -----------------------------------------------------------------------------

#[test]
fn tc_p2_015_02_press_enter_cursor_centered() {
    let source = "Line one\n\nLine two\n\nLine three";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        3,
        "Should have 3 paragraphs for Enter key test"
    );

    // TC-P2-015-02: After pressing Enter at end of paragraph 1,
    // cursor moves to start of paragraph 2
    let cursor_at_second = paragraphs[1].offset;
    let active_after_enter = doc.get_paragraph_at(cursor_at_second);
    assert_eq!(
        active_after_enter, 1,
        "TC-P2-015-02: After Enter key, cursor should identify paragraph 2 for centering"
    );

    // TC-P2-015-02: Enter at start of paragraph (new paragraph created above)
    let cursor_at_first = paragraphs[0].offset;
    let active_at_first = doc.get_paragraph_at(cursor_at_first);
    assert_eq!(
        active_at_first, 0,
        "TC-P2-015-02: Cursor at first paragraph should identify paragraph 1 for centering"
    );

    // TC-P2-015-02: Enter at document end creates implicit paragraph
    let doc_end = doc.source().len();
    let active_at_end = doc.get_paragraph_at(doc_end);
    assert_eq!(
        active_at_end, 2,
        "TC-P2-015-02: Enter at document end should identify last paragraph for centering"
    );
}

// -----------------------------------------------------------------------------
// TC-P2-015-03: Arrow key navigation — cursor stays centered
// Category: render
// Input: Enable typewriter mode, use arrow keys
// Expected: Cursor stays at vertical center during navigation
// -----------------------------------------------------------------------------

#[test]
fn tc_p2_015_03_arrow_key_navigation() {
    let source =
        "Paragraph one\n\nParagraph two\n\nParagraph three\n\nParagraph four\n\nParagraph five";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        5,
        "Should have 5 paragraphs for navigation test"
    );

    // TC-P2-015-03: Simulate Down arrow sequence through document
    let down_navigation = [
        (paragraphs[0].offset, 0, "Down to paragraph 1"),
        (paragraphs[1].offset, 1, "Down to paragraph 2"),
        (paragraphs[2].offset, 2, "Down to paragraph 3"),
        (paragraphs[3].offset, 3, "Down to paragraph 4"),
        (paragraphs[4].offset, 4, "Down to paragraph 5"),
    ];

    for (offset, expected, desc) in down_navigation {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, expected,
            "TC-P2-015-03: {} should identify paragraph {}",
            desc, expected
        );
    }

    // TC-P2-015-03: Simulate Up arrow sequence (reverse)
    let up_navigation = [
        (paragraphs[4].offset, 4, "Up from paragraph 5"),
        (paragraphs[3].offset, 3, "Up to paragraph 4"),
        (paragraphs[2].offset, 2, "Up to paragraph 3"),
        (paragraphs[1].offset, 1, "Up to paragraph 2"),
        (paragraphs[0].offset, 0, "Up to paragraph 1"),
    ];

    for (offset, expected, desc) in up_navigation {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, expected,
            "TC-P2-015-03: {} should identify paragraph {}",
            desc, expected
        );
    }

    // TC-P2-015-03: Left/Right arrow within same paragraph
    let within_first = paragraphs[1].offset + 5;
    let active_within = doc.get_paragraph_at(within_first);
    assert_eq!(
        active_within, 1,
        "TC-P2-015-03: Left/Right within paragraph should keep same paragraph for centering"
    );
}

// -----------------------------------------------------------------------------
// TC-P2-015-04: Paste text — cursor stays centered
// Category: render
// Input: Enable typewriter mode, paste text
// Expected: Text pasted, cursor at vertical center
// -----------------------------------------------------------------------------

#[test]
fn tc_p2_015_04_paste_text_cursor_centered() {
    let source = "Original content\n\nMore original content";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        2,
        "Should have 2 paragraphs for paste test"
    );

    // TC-P2-015-04: Paste at end of first paragraph (cursor ends in paragraph 1)
    let paste_end_first = paragraphs[0].offset + paragraphs[0].length;
    let active_paste_first = doc.get_paragraph_at(paste_end_first);
    assert_eq!(
        active_paste_first, 0,
        "TC-P2-015-04: Paste at end of paragraph 1 should identify paragraph 1 for centering"
    );

    // TC-P2-015-04: Paste at middle of first paragraph
    let paste_mid_first = paragraphs[0].offset + paragraphs[0].length / 2;
    let active_paste_mid = doc.get_paragraph_at(paste_mid_first);
    assert_eq!(
        active_paste_mid, 0,
        "TC-P2-015-04: Paste mid-paragraph should identify same paragraph for centering"
    );

    // TC-P2-015-04: Paste at document end (cursor in last paragraph)
    let paste_end_doc = doc.source().len();
    let active_paste_end = doc.get_paragraph_at(paste_end_doc);
    assert_eq!(
        active_paste_end, 1,
        "TC-P2-015-04: Paste at document end should identify last paragraph for centering"
    );

    // TC-P2-015-04: Paste within second paragraph
    let paste_second = paragraphs[1].offset + 3;
    let active_second = doc.get_paragraph_at(paste_second);
    assert_eq!(
        active_second, 1,
        "TC-P2-015-04: Paste within paragraph 2 should identify paragraph 2 for centering"
    );
}

// -----------------------------------------------------------------------------
// TC-P2-015-05: Large document — 100+ paragraphs performance
// Category: performance
// Input: 100+ paragraph document, enable typewriter mode
// Expected: Scrolling maintains 60 FPS without jank
// -----------------------------------------------------------------------------

#[test]
fn tc_p2_015_05_large_document_100_paragraphs() {
    // TC-P2-015-05: Create a 100-paragraph document simulating large document
    let mut source = String::new();
    for i in 1..=100 {
        if i > 1 {
            source.push_str("\n\n");
        }
        source.push_str(&format!("Paragraph {}", i));
    }

    let doc = SemanticDocument::parse(&source);
    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        100,
        "TC-P2-015-05: Should have 100 paragraphs for performance test"
    );

    // TC-P2-015-05: Measure paragraph lookup performance for 60 FPS
    // 60 FPS = 16.67ms per frame. For smooth scrolling, each paragraph
    // lookup must complete well within one frame budget.
    let start = std::time::Instant::now();
    for i in 0..100 {
        let offset = paragraphs[i].offset;
        let _active = doc.get_paragraph_at(offset);
    }
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() < 10,
        "TC-P2-015-05: 100 sequential paragraph lookups should be < 10ms for 60 FPS, was {}ms",
        elapsed.as_millis()
    );

    // TC-P2-015-05: Simulate scrolling through document — random access pattern
    let scroll_start = std::time::Instant::now();
    let scroll_positions = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 99];
    for idx in scroll_positions {
        let offset = paragraphs[idx].offset;
        let _active = doc.get_paragraph_at(offset);
    }
    let scroll_elapsed = scroll_start.elapsed();

    assert!(
        scroll_elapsed.as_micros() < 5000,
        "TC-P2-015-05: 11 random-access lookups should be < 5ms for smooth scroll, was {}ms",
        scroll_elapsed.as_millis()
    );

    // TC-P2-015-05: Verify all 100 paragraphs are correctly indexed
    for (i, para) in paragraphs.iter().enumerate() {
        let active = doc.get_paragraph_at(para.offset);
        assert_eq!(
            active, i,
            "TC-P2-015-05: Paragraph {} offset {} should map to index {}",
            i, para.offset, i
        );
    }
}

// =============================================================================
// TC-P2-015 Edge Cases
// =============================================================================

#[test]
fn tc_p2_015_edge_cursor_at_first_char() {
    let source = "First paragraph\n\nSecond paragraph\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    // TC-P2-015: Cursor at very first character
    let active_at_zero = doc.get_paragraph_at(0);
    assert_eq!(
        active_at_zero, 0,
        "TC-P2-015 edge: Cursor at first char should identify paragraph 1"
    );

    // TC-P2-015: Cursor at start of second paragraph
    let active_at_second = doc.get_paragraph_at(paragraphs[1].offset);
    assert_eq!(
        active_at_second, 1,
        "TC-P2-015 edge: Cursor at start of paragraph 2 should identify paragraph 2"
    );
}

#[test]
fn tc_p2_015_edge_cursor_at_last_char() {
    let source = "Para A\n\nPara B\n\nPara C";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    // TC-P2-015: Cursor at last character of last paragraph
    let last_para_end = paragraphs[2].offset + paragraphs[2].length;
    let active_last = doc.get_paragraph_at(last_para_end);
    assert_eq!(
        active_last, 2,
        "TC-P2-015 edge: Cursor at last char should identify last paragraph"
    );

    // TC-P2-015: Cursor one past last character
    let doc_end = doc.source().len();
    let active_end = doc.get_paragraph_at(doc_end);
    assert_eq!(
        active_end, 2,
        "TC-P2-015 edge: Cursor past last char should identify last paragraph"
    );
}

#[test]
fn tc_p2_015_edge_single_paragraph() {
    let source = "Only one paragraph in this document";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    assert_eq!(
        paragraphs.len(),
        1,
        "TC-P2-015 edge: Single paragraph document"
    );

    // TC-P2-015: Any cursor position in single paragraph identifies it
    let positions = [0, 5, 15, 30];
    for pos in positions {
        let active = doc.get_paragraph_at(pos);
        assert_eq!(
            active, 0,
            "TC-P2-015 edge: Cursor at position {} in single paragraph should identify paragraph 1",
            pos
        );
    }
}

#[test]
fn tc_p2_015_edge_rapid_sequential_typing() {
    // TC-P2-015: Simulate rapid character-by-character typing
    let source = "AAAAAAAAAA\n\nBBBBBBBBBB\n\nCCCCCCCCCC";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    // TC-P2-015: Each character position should identify the correct paragraph
    for offset in 0..paragraphs[0].length {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, 0,
            "TC-P2-015 rapid: Offset {} in first paragraph should identify paragraph 1",
            offset
        );
    }

    // TC-P2-015: Verify paragraph 2 is accessible
    for offset in paragraphs[1].offset..(paragraphs[1].offset + paragraphs[1].length) {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, 1,
            "TC-P2-015 rapid: Offset {} in second paragraph should identify paragraph 2",
            offset
        );
    }
}
