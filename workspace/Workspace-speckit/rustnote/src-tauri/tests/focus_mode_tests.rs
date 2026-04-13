use rustnote_lib::semantic::ast::SemanticDocument;

#[test]
fn tc_g013_001_focusmode_dim_others() {
    let source = "Para 1\n\nPara 2\n\nPara 3";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 3, "Should have 3 paragraphs");

    let para1_offset = paragraphs[0].offset;
    let para2_offset = paragraphs[1].offset;
    let para3_offset = paragraphs[2].offset;

    let active_para_1 = doc.get_paragraph_at(para1_offset);
    let active_para_2 = doc.get_paragraph_at(para2_offset);
    let active_para_3 = doc.get_paragraph_at(para3_offset);

    assert_eq!(active_para_1, 0, "Cursor at Para 1 should be paragraph 0");
    assert_eq!(active_para_2, 1, "Cursor at Para 2 should be paragraph 1");
    assert_eq!(active_para_3, 2, "Cursor at Para 3 should be paragraph 2");
}

#[test]
fn tc_g013_002_focusmode_long_document() {
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

    let middle_offset = paragraphs[49].offset;
    let active_middle = doc.get_paragraph_at(middle_offset);
    assert_eq!(
        active_middle, 49,
        "Middle paragraph should be detected correctly"
    );

    let first_offset = paragraphs[0].offset;
    let active_first = doc.get_paragraph_at(first_offset);
    assert_eq!(active_first, 0, "First paragraph should be index 0");

    let last_offset = paragraphs[99].offset;
    let active_last = doc.get_paragraph_at(last_offset);
    assert_eq!(active_last, 99, "Last paragraph should be index 99");
}

#[test]
fn tc_g013_003_focusmode_transitions() {
    let source = "Para 1\n\nPara 2\n\nPara 3\n\nPara 4\n\nPara 5";
    let doc = SemanticDocument::parse(source);

    let paragraphs = doc.get_paragraphs();
    assert_eq!(paragraphs.len(), 5, "Should have 5 paragraphs");

    let para1_offset = paragraphs[0].offset;
    let para2_offset = paragraphs[1].offset;
    let para3_offset = paragraphs[2].offset;
    let para4_offset = paragraphs[3].offset;

    let active_at_para1 = doc.get_paragraph_at(para1_offset);
    let active_at_para2 = doc.get_paragraph_at(para2_offset);
    let active_at_para3 = doc.get_paragraph_at(para3_offset);
    let active_at_para4 = doc.get_paragraph_at(para4_offset);

    assert_eq!(active_at_para1, 0, "Para 1 should be paragraph 0");
    assert_eq!(active_at_para2, 1, "Para 2 should be paragraph 1");
    assert_eq!(active_at_para3, 2, "Para 3 should be paragraph 2");
    assert_eq!(active_at_para4, 3, "Para 4 should be paragraph 3");

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
            "Transition to offset {} should give paragraph {}",
            offset, expected_idx
        );
    }
}

#[test]
fn tc_g013_001_paragraph_spans() {
    let source = "Line in para 1\nwith line break\n\nPara 2 start";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    assert_eq!(paragraphs.len(), 2, "Should have 2 paragraphs");
    assert_eq!(paragraphs[0].offset, 0);
    assert_eq!(paragraphs[1].offset, 22);
}

#[test]
fn tc_g013_002_edge_cases_first_char() {
    let source = "# Title\n\nFirst paragraph content\n\nSecond paragraph";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    let first_char_offset = paragraphs[0].offset;
    let active = doc.get_paragraph_at(first_char_offset);
    assert_eq!(active, 0, "Cursor at first character should be paragraph 0");
}

#[test]
fn tc_g013_002_edge_cases_last_char() {
    let source = "# Title\n\nFirst paragraph\n\nSecond paragraph last";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    let last_offset = doc.source().len();
    let active = doc.get_paragraph_at(last_offset);
    assert_eq!(active, 2, "Cursor at end should be last paragraph");
}

#[test]
fn tc_g013_003_empty_document() {
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
fn tc_g013_003_single_paragraph() {
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

#[test]
fn tc_g013_003_paragraph_with_headings() {
    let source = "# H1\n\n## H2\n\n### H3\n\nContent\n\nNormal para";
    let doc = SemanticDocument::parse(source);
    let paragraphs = doc.get_paragraphs();

    assert_eq!(
        paragraphs.len(),
        5,
        "Should have 5 blocks (3 headings + content + para)"
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
