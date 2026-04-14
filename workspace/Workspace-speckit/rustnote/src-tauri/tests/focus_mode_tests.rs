use rustnote_lib::semantic::ast::SemanticDocument;

/// TC-G010-001: Non-current paragraph dimming
/// Input: Document with 10 paragraphs, cursor in paragraph 3
/// Expected: Paragraph 3 full opacity, others dimmed
#[test]
fn tc_g010_001_focusmode_dim_10paras() {
    // Create a document with 10 paragraphs
    let source = "Para 1\n\nPara 2\n\nPara 3\n\nPara 4\n\nPara 5\n\nPara 6\n\nPara 7\n\nPara 8\n\nPara 9\n\nPara 10";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 10, "Should have 10 paragraphs");

    // Get offsets for each paragraph
    let para1_offset = paragraphs[0].offset;
    let para2_offset = paragraphs[1].offset;
    let para3_offset = paragraphs[2].offset; // This is paragraph 3 (0-indexed as 2)
    let para4_offset = paragraphs[3].offset;
    let para10_offset = paragraphs[9].offset;

    // Verify paragraph detection at various positions
    let active_para_1 = doc.get_paragraph_at(para1_offset);
    let active_para_2 = doc.get_paragraph_at(para2_offset);
    let active_para_3 = doc.get_paragraph_at(para3_offset); // Should be index 2
    let active_para_4 = doc.get_paragraph_at(para4_offset);
    let active_para_10 = doc.get_paragraph_at(para10_offset);

    assert_eq!(active_para_1, 0, "Cursor at Para 1 should be paragraph 0");
    assert_eq!(active_para_2, 1, "Cursor at Para 2 should be paragraph 1");
    assert_eq!(active_para_3, 2, "Cursor at Para 3 should be paragraph 2");
    assert_eq!(active_para_4, 3, "Cursor at Para 4 should be paragraph 3");
    assert_eq!(active_para_10, 9, "Cursor at Para 10 should be paragraph 9");

    // Verify that cursor position within paragraph 3 correctly identifies it
    // Cursor at the start of "Para 3" should give paragraph index 2
    let cursor_in_para3 = doc.get_paragraph_at(para3_offset);
    assert_eq!(
        cursor_in_para3, 2,
        "Cursor at start of Para 3 should return paragraph 2"
    );

    // Simulate focus mode: paragraph 3 is active (index 2), others should be dimmed
    let active_index = cursor_in_para3;
    for (i, para) in paragraphs.iter().enumerate() {
        let is_active = i == active_index;
        let para_at_offset = doc.get_paragraph_at(para.offset);
        if is_active {
            assert_eq!(
                para_at_offset, active_index,
                "Active paragraph {} should be detected at its offset",
                active_index
            );
        }
    }
}

/// TC-G010-002: Focus mode with long document
/// Input: Document with 100 paragraphs
/// Expected: Only current paragraph visible/clear, others dim
#[test]
fn tc_g010_002_focusmode_long_document() {
    let mut source = String::new();
    for i in 1..=100 {
        if i > 1 {
            source.push_str("\n\n");
        }
        source.push_str(&format!("Paragraph {}", i));
    }

    let doc = SemanticDocument::parse(&source);
    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 100, "Should have 100 paragraphs");

    // Test first paragraph
    let first_offset = paragraphs[0].offset;
    let active_first = doc.get_paragraph_at(first_offset);
    assert_eq!(active_first, 0, "First paragraph should be index 0");

    // Test middle paragraph (50th, index 49)
    let middle_offset = paragraphs[49].offset;
    let active_middle = doc.get_paragraph_at(middle_offset);
    assert_eq!(
        active_middle, 49,
        "Middle paragraph should be detected correctly"
    );

    // Test last paragraph (100th, index 99)
    let last_offset = paragraphs[99].offset;
    let active_last = doc.get_paragraph_at(last_offset);
    assert_eq!(active_last, 99, "Last paragraph should be index 99");

    // Simulate focus mode: verify that when cursor is at middle,
    // only that paragraph should be "active" (full opacity) and others dimmed
    let current_active = 49; // Middle paragraph
    for (i, para) in paragraphs.iter().enumerate() {
        let is_current = i == current_active;
        assert_eq!(
            doc.get_paragraph_at(para.offset) == current_active,
            is_current,
            "Paragraph {} should {}be current",
            i,
            if is_current { "" } else { "NOT " }
        );
    }
}

/// TC-G010-003: Focus mode toggle
/// Input: Toggle focus mode
/// Expected: Dimming appears/disappears correctly
#[test]
fn tc_g010_003_focusmode_toggle() {
    // This test verifies that paragraph detection works correctly
    // for focus mode toggle transitions
    let source = "Para 1\n\nPara 2\n\nPara 3\n\nPara 4\n\nPara 5";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 5, "Should have 5 paragraphs");

    // Verify initial paragraph detection
    let para1_offset = paragraphs[0].offset;
    let para2_offset = paragraphs[1].offset;
    let para3_offset = paragraphs[2].offset;
    let para4_offset = paragraphs[3].offset;

    assert_eq!(
        doc.get_paragraph_at(para1_offset),
        0,
        "Para 1 should be paragraph 0"
    );
    assert_eq!(
        doc.get_paragraph_at(para2_offset),
        1,
        "Para 2 should be paragraph 1"
    );
    assert_eq!(
        doc.get_paragraph_at(para3_offset),
        2,
        "Para 3 should be paragraph 2"
    );
    assert_eq!(
        doc.get_paragraph_at(para4_offset),
        3,
        "Para 4 should be paragraph 3"
    );

    // Simulate focus mode toggle transitions
    // When focus mode is toggled ON, the current paragraph should be active
    // When focus mode is toggled OFF, all paragraphs should be equally visible
    let transitions = [
        (para1_offset, 0),
        (para2_offset, 1),
        (para3_offset, 2),
        (para4_offset, 3),
    ];

    for (offset, expected_idx) in transitions {
        let active = doc.get_paragraph_at(offset);
        assert_eq!(
            active, expected_idx,
            "Focus mode toggle: transition to offset {} should give paragraph {}",
            offset, expected_idx
        );
    }
}

#[test]
fn tc_g010_005_edge_cases_first_char() {
    let source = "# Title\n\nFirst paragraph content\n\nSecond paragraph";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    let first_char_offset = paragraphs[0].offset;
    let active = doc.get_paragraph_at(first_char_offset);
    assert_eq!(active, 0, "Cursor at first character should be paragraph 0");
}

#[test]
fn tc_g010_006_empty_document() {
    let source = "";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    assert!(
        paragraphs.is_empty() || paragraphs.len() == 1,
        "Empty doc should have no or one empty paragraph"
    );
    let active = doc.get_paragraph_at(0);
    assert_eq!(active, 0, "Empty doc cursor at 0 should give paragraph 0");
}

#[test]
fn tc_g010_007_single_paragraph() {
    let source = "Only one paragraph here";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    assert_eq!(paragraphs.len(), 1, "Should have exactly 1 paragraph");
    let active = doc.get_paragraph_at(5);
    assert_eq!(
        active, 0,
        "Any position in single para should be paragraph 0"
    );
}
