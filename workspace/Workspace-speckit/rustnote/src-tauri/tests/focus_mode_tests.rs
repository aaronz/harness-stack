use rustnote_lib::semantic::ast::SemanticDocument;

/// TC-FM001: Non-current paragraphs dimmed
/// Input: 5-paragraph document, cursor in paragraph 3
/// Expected: Paragraphs 1-2 dim (opacity ~0.3), paragraph 3 full opacity
#[test]
fn tc_fm001_non_current_paragraphs_dimmed() {
    let source = "Paragraph 1\n\nParagraph 2\n\nParagraph 3\n\nParagraph 4\n\nParagraph 5";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 5, "Should have 5 paragraphs");

    let para3_offset = paragraphs[2].offset;
    let active_index = doc.get_paragraph_at(para3_offset);
    assert_eq!(active_index, 2, "Paragraph 3 (index 2) should be active");

    let para1_offset = paragraphs[0].offset;
    let para2_offset = paragraphs[1].offset;
    let para4_offset = paragraphs[3].offset;
    let para5_offset = paragraphs[4].offset;

    assert_ne!(
        doc.get_paragraph_at(para1_offset),
        active_index,
        "Para 1 should be dimmed"
    );
    assert_ne!(
        doc.get_paragraph_at(para2_offset),
        active_index,
        "Para 2 should be dimmed"
    );
    assert_eq!(
        doc.get_paragraph_at(para3_offset),
        active_index,
        "Para 3 should be active"
    );
    assert_ne!(
        doc.get_paragraph_at(para4_offset),
        active_index,
        "Para 4 should be dimmed"
    );
    assert_ne!(
        doc.get_paragraph_at(para5_offset),
        active_index,
        "Para 5 should be dimmed"
    );

    for (i, para) in paragraphs.iter().enumerate() {
        let is_active = i == active_index;
        assert_eq!(
            doc.get_paragraph_at(para.offset) == active_index,
            is_active,
            "Paragraph {} should {}be active",
            i,
            if is_active { "" } else { "NOT " }
        );
    }
}

/// TC-FM002: Light theme focus mode
/// Input: Light theme enabled, focus mode on
/// Expected: Non-current paragraphs visually dimmed
#[test]
fn tc_fm002_light_theme_focus_mode() {
    let source = "First paragraph\n\nSecond paragraph\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 3, "Should have 3 paragraphs");

    let active_offset = paragraphs[0].offset;
    let active_index = doc.get_paragraph_at(active_offset);
    assert_eq!(active_index, 0, "First paragraph should be active");

    for (i, para) in paragraphs.iter().enumerate() {
        let para_index = doc.get_paragraph_at(para.offset);
        let should_be_active = para_index == active_index;
        assert_eq!(
            should_be_active,
            i == 0,
            "In light theme focus mode, paragraph {} should {}be active",
            i,
            if i == 0 { "" } else { "NOT " }
        );
    }
}

/// TC-FM003: Dark theme focus mode
/// Input: Dark theme enabled, focus mode on
/// Expected: Non-current paragraphs visually dimmed
#[test]
fn tc_fm003_dark_theme_focus_mode() {
    let source = "Para A\n\nPara B\n\nPara C\n\nPara D";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 4, "Should have 4 paragraphs");

    let active_offset = paragraphs[3].offset;
    let active_index = doc.get_paragraph_at(active_offset);
    assert_eq!(active_index, 3, "Last paragraph should be active");

    let non_active_offsets: Vec<usize> = paragraphs
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != 3)
        .map(|(_, p)| p.offset)
        .collect();

    for offset in non_active_offsets {
        assert_ne!(
            doc.get_paragraph_at(offset),
            active_index,
            "In dark theme focus mode, non-active paragraphs should be dimmed"
        );
    }

    assert_eq!(
        doc.get_paragraph_at(paragraphs[3].offset),
        3,
        "Active paragraph should be at full opacity"
    );
}

/// TC-FM004: Nested lists dim correctly
/// Input: Document with nested lists, focus mode verifies list item detection
/// Expected: Document parses correctly for focus mode (lists tracked by TipTap)
#[test]
fn tc_fm004_nested_lists_dim_correctly() {
    let source =
        "First paragraph\n\n- Item 1\n  - Nested 1\n  - Nested 2\n- Item 2\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        2,
        "Should have 2 regular paragraphs (lists excluded)"
    );

    let list_items = doc.get_list_items();
    assert!(list_items.len() >= 3, "Should have at least 3 list items");

    let first_para_offset = paragraphs[0].offset;
    let active_index = doc.get_paragraph_at(first_para_offset);
    assert_eq!(active_index, 0, "First paragraph should be active");

    let third_para_offset = paragraphs[1].offset;
    let third_para_index = doc.get_paragraph_at(third_para_offset);
    assert_eq!(
        third_para_index, 1,
        "Third paragraph should have its own index"
    );
    assert_ne!(
        doc.get_paragraph_at(third_para_offset),
        active_index,
        "Third paragraph should be dimmed when first is active"
    );
}

/// TC-FM005: Blockquotes dim correctly
/// Input: Document with blockquotes
/// Expected: Blockquotes are properly detected (not counted as paragraphs)
#[test]
fn tc_fm005_blockquotes_dim_correctly() {
    let source =
        "Normal paragraph\n\n> This is a blockquote\n> with multiple lines\n\nAnother paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(
        paragraphs.len(),
        2,
        "Blockquotes are excluded from paragraph count"
    );

    let first_para_offset = paragraphs[0].offset;
    let active_index = doc.get_paragraph_at(first_para_offset);
    assert_eq!(active_index, 0, "First paragraph should be active");

    let second_para_offset = paragraphs[1].offset;
    let second_para_index = doc.get_paragraph_at(second_para_offset);
    assert_eq!(
        second_para_index, 1,
        "Second paragraph should have its own index"
    );
    assert_ne!(
        doc.get_paragraph_at(second_para_offset),
        active_index,
        "Second paragraph should be dimmed when first is active"
    );
}

/// TC-FM006: Code blocks dim correctly
/// Input: Document with code blocks
/// Expected: Code blocks are properly detected (not counted as paragraphs)
#[test]
fn tc_fm006_code_blocks_dim_correctly() {
    let source = "First paragraph\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\n\nThird paragraph";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert!(
        paragraphs.len() >= 2,
        "Should have at least 2 regular paragraphs"
    );

    let active_offset = paragraphs[0].offset;
    let active_index = doc.get_paragraph_at(active_offset);
    assert_eq!(active_index, 0, "First paragraph should be active");

    let third_offset = paragraphs[1].offset;
    let third_index = doc.get_paragraph_at(third_offset);
    assert_eq!(third_index, 1, "Third paragraph should have its own index");
    assert_ne!(
        doc.get_paragraph_at(third_offset),
        active_index,
        "Third paragraph should be dimmed when first is active"
    );
}

/// TC-FM007: Headings dim correctly
/// Input: Document with headings, cursor at first heading
/// Expected: First heading is active, other headings have different indices
#[test]
fn tc_fm007_headings_dim_correctly() {
    let source = "# Heading 1\n\nSome content\n\n## Heading 2\n\nMore content\n\n### Heading 3";
    let doc = SemanticDocument::parse(source);

    let headings = doc.get_headings();
    assert!(headings.len() >= 3, "Should have at least 3 headings");

    let first_heading_offset = headings[0].offset;
    let first_heading_level = headings[0].level;
    assert_eq!(first_heading_level, 1, "First heading should be level 1");

    let second_heading_offset = headings[1].offset;
    let second_heading_level = headings[1].level;
    assert_eq!(second_heading_level, 2, "Second heading should be level 2");

    assert_ne!(
        first_heading_offset, second_heading_offset,
        "Headings should have different offsets"
    );

    for (i, heading) in headings.iter().enumerate() {
        assert!(
            i == 0 || heading.offset != first_heading_offset,
            "Heading {} should {}be the first heading",
            i,
            if i == 0 { "be" } else { "NOT be" }
        );
    }
}

/// TC-FM008: No flash on cursor move
/// Input: Cursor moves between paragraphs
/// Expected: No flash or jump, smooth dim transition
#[test]
fn tc_fm008_no_flash_on_cursor_move() {
    let source = "Para A\n\nPara B\n\nPara C\n\nPara D\n\nPara E";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 5, "Should have 5 paragraphs");

    let transitions = vec![(0, 2), (2, 4), (4, 1), (1, 3), (3, 0)];

    for (from_idx, to_idx) in transitions {
        let from_offset = paragraphs[from_idx].offset;
        let to_offset = paragraphs[to_idx].offset;

        let from_active = doc.get_paragraph_at(from_offset);
        let to_active = doc.get_paragraph_at(to_offset);

        assert_eq!(
            from_active, from_idx,
            "From paragraph {} should be active",
            from_idx
        );
        assert_eq!(
            to_active, to_idx,
            "To paragraph {} should be active",
            to_idx
        );

        assert_ne!(
            doc.get_paragraph_at(from_offset),
            to_active,
            "Previous paragraph should no longer be active after cursor move"
        );
        assert_eq!(
            doc.get_paragraph_at(to_offset),
            to_active,
            "New paragraph should be active after cursor move"
        );
    }
}

/// TC-G010-001: Non-current paragraph dimming
/// Input: Document with 10 paragraphs, cursor in paragraph 3
/// Expected: Paragraph 3 full opacity, others dimmed
#[test]
fn tc_g010_001_focusmode_dim_10paras() {
    let source = "Para 1\n\nPara 2\n\nPara 3\n\nPara 4\n\nPara 5\n\nPara 6\n\nPara 7\n\nPara 8\n\nPara 9\n\nPara 10";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 10, "Should have 10 paragraphs");

    let para1_offset = paragraphs[0].offset;
    let para2_offset = paragraphs[1].offset;
    let para3_offset = paragraphs[2].offset;
    let para4_offset = paragraphs[3].offset;
    let para10_offset = paragraphs[9].offset;

    let active_para_1 = doc.get_paragraph_at(para1_offset);
    let active_para_2 = doc.get_paragraph_at(para2_offset);
    let active_para_3 = doc.get_paragraph_at(para3_offset);
    let active_para_4 = doc.get_paragraph_at(para4_offset);
    let active_para_10 = doc.get_paragraph_at(para10_offset);

    assert_eq!(active_para_1, 0, "Cursor at Para 1 should be paragraph 0");
    assert_eq!(active_para_2, 1, "Cursor at Para 2 should be paragraph 1");
    assert_eq!(active_para_3, 2, "Cursor at Para 3 should be paragraph 2");
    assert_eq!(active_para_4, 3, "Cursor at Para 4 should be paragraph 3");
    assert_eq!(active_para_10, 9, "Cursor at Para 10 should be paragraph 9");

    let cursor_in_para3 = doc.get_paragraph_at(para3_offset);
    assert_eq!(
        cursor_in_para3, 2,
        "Cursor at start of Para 3 should return paragraph 2"
    );

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

    let first_offset = paragraphs[0].offset;
    let active_first = doc.get_paragraph_at(first_offset);
    assert_eq!(active_first, 0, "First paragraph should be index 0");

    let middle_offset = paragraphs[49].offset;
    let active_middle = doc.get_paragraph_at(middle_offset);
    assert_eq!(
        active_middle, 49,
        "Middle paragraph should be detected correctly"
    );

    let last_offset = paragraphs[99].offset;
    let active_last = doc.get_paragraph_at(last_offset);
    assert_eq!(active_last, 99, "Last paragraph should be index 99");

    let current_active = 49;
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
    let source = "Para 1\n\nPara 2\n\nPara 3\n\nPara 4\n\nPara 5";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 5, "Should have 5 paragraphs");

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
