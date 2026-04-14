use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustnote_lib::editor::transforms::{Transform, TransformEngine};

fn engine() -> TransformEngine {
    TransformEngine::new()
}

fn bench_transform_enter(c: &mut Criterion) {
    c.bench_function("transform_enter_simple", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "- item", 6, None)
        });
    });

    c.bench_function("transform_enter_heading", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "# Heading", 9, None)
        });
    });

    c.bench_function("transform_enter_paragraph", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "paragraph", 5, None)
        });
    });

    c.bench_function("transform_enter_ordered_list", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "1. item", 7, None)
        });
    });

    c.bench_function("transform_enter_blockquote", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "> quote", 7, None)
        });
    });

    c.bench_function("transform_enter_task_list", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "- [ ] task", 10, None)
        });
    });
}

fn bench_transform_backspace(c: &mut Criterion) {
    c.bench_function("backspace_middle", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Backspace, "Hello world", 6, None)
        });
    });

    c.bench_function("backspace_list_boundary", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Backspace, "- item\n- item2", 10, None)
        });
    });

    c.bench_function("backspace_line_join", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Backspace, "line1\nline2", 6, None)
        });
    });
}

fn bench_transform_tab(c: &mut Criterion) {
    c.bench_function("tab_list_item", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Tab, "- item", 2, None)
        });
    });

    c.bench_function("tab_nested_list", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Tab, "- nested item", 2, None)
        });
    });

    c.bench_function("tab_paragraph", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Tab, "paragraph", 5, None)
        });
    });
}

fn bench_transform_shift_tab(c: &mut Criterion) {
    c.bench_function("shift_tab_four_spaces", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::ShiftTab, "    - item", 6, None)
        });
    });

    c.bench_function("shift_tab_single_tab", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::ShiftTab, "\t-item", 1, None)
        });
    });

    c.bench_function("shift_tab_no_indent", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::ShiftTab, "paragraph", 5, None)
        });
    });
}

fn bench_transform_sequence(c: &mut Criterion) {
    c.bench_function("transform_sequence", |b| {
        b.iter(|| {
            let e = engine();
            let r1 = e.apply(&Transform::Enter, "- item", 6, None);
            let r2 = e.apply(&Transform::Tab, &r1.content, r1.cursor_offset, None);
            let _ = e.apply(&Transform::Enter, &r2.content, r2.cursor_offset, None);
        });
    });
}

fn bench_is_empty_list_item(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::is_empty_list_item;

    c.bench_function("is_empty_list_item_unordered", |b| {
        b.iter(|| is_empty_list_item("- ", 2))
    });

    c.bench_function("is_empty_list_item_task", |b| {
        b.iter(|| is_empty_list_item("- [ ] ", 6))
    });

    c.bench_function("is_empty_list_item_ordered", |b| {
        b.iter(|| is_empty_list_item("1. ", 3))
    });

    c.bench_function("is_empty_list_item_with_content", |b| {
        b.iter(|| is_empty_list_item("- item", 6))
    });
}

fn bench_is_in_blockquote(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::is_in_blockquote;

    c.bench_function("is_in_blockquote_true", |b| {
        b.iter(|| is_in_blockquote("> quote", 3))
    });

    c.bench_function("is_in_blockquote_false", |b| {
        b.iter(|| is_in_blockquote("not a quote", 3))
    });

    c.bench_function("is_in_blockquote_after_newline", |b| {
        b.iter(|| is_in_blockquote("# Heading\n> quote", 15))
    });
}

fn bench_is_in_heading(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::is_in_heading;

    c.bench_function("is_in_heading_h1", |b| {
        b.iter(|| is_in_heading("# Heading", 3))
    });

    c.bench_function("is_in_heading_false", |b| {
        b.iter(|| is_in_heading("paragraph", 5))
    });

    c.bench_function("is_in_heading_h2", |b| {
        b.iter(|| is_in_heading("## Subheading", 4))
    });
}

/// TC-B009: Transform operations benchmark
/// Target: All transform operations complete within thresholds
/// Tests with 1000 and 100000 character documents
fn bench_transform_wrap(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::Transform;

    c.bench_function("transform_wrap_bold", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::Wrap {
                    before: "**".to_string(),
                    after: "**".to_string(),
                },
                "text",
                4,
                Some(0),
            )
        });
    });

    c.bench_function("transform_wrap_italic", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::Wrap {
                    before: "*".to_string(),
                    after: "*".to_string(),
                },
                "text",
                4,
                Some(0),
            )
        });
    });

    c.bench_function("transform_wrap_code", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::Wrap {
                    before: "`".to_string(),
                    after: "`".to_string(),
                },
                "text",
                4,
                Some(0),
            )
        });
    });
}

fn bench_transform_enter_in_list_item(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::Transform;

    c.bench_function("transform_enter_in_list_item_simple", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::EnterInListItem { is_empty: false },
                "- item",
                6,
                None,
            )
        });
    });

    c.bench_function("transform_enter_in_list_item_task", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::EnterInListItem { is_empty: false },
                "- [ ] task",
                10,
                None,
            )
        });
    });

    c.bench_function("transform_enter_in_list_item_numbered", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::EnterInListItem { is_empty: false },
                "1. item",
                7,
                None,
            )
        });
    });
}

fn bench_transform_enter_in_blockquote(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::Transform;

    c.bench_function("transform_enter_in_blockquote_simple", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::EnterInBlockQuote, "> quote", 7, None)
        });
    });

    c.bench_function("transform_enter_in_blockquote_nested", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::EnterInBlockQuote, "> > nested quote", 14, None)
        });
    });
}

fn bench_transform_enter_in_heading(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::Transform;

    c.bench_function("transform_enter_in_heading_h1", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::EnterInHeading { level: 1 },
                "# Heading",
                9,
                None,
            )
        });
    });

    c.bench_function("transform_enter_in_heading_h2", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::EnterInHeading { level: 2 },
                "## Subheading",
                13,
                None,
            )
        });
    });
}

fn bench_transform_large_document(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::Transform;
    let large_content = "# Title\n\n".to_string() + &"x".repeat(1000);

    c.bench_function("transform_enter_1000_chars", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, &large_content, 1000, None)
        });
    });

    let huge_content = "# Title\n\n".to_string() + &"x".repeat(100000);

    c.bench_function("transform_enter_100000_chars", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, &huge_content, 100000, None)
        });
    });
}

fn bench_transform_wrap_large_document(c: &mut Criterion) {
    use rustnote_lib::editor::transforms::Transform;

    let large_content = "x".repeat(1000);
    let huge_content = "x".repeat(100000);

    c.bench_function("transform_wrap_1000_chars", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::Wrap {
                    before: "**".to_string(),
                    after: "**".to_string(),
                },
                &large_content,
                1000,
                Some(0),
            )
        });
    });

    c.bench_function("transform_wrap_100000_chars", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(
                &Transform::Wrap {
                    before: "**".to_string(),
                    after: "**".to_string(),
                },
                &huge_content,
                100000,
                Some(0),
            )
        });
    });
}

criterion_group!(
    benches,
    bench_transform_enter,
    bench_transform_backspace,
    bench_transform_tab,
    bench_transform_shift_tab,
    bench_transform_sequence,
    bench_is_empty_list_item,
    bench_is_in_blockquote,
    bench_is_in_heading,
    bench_transform_wrap,
    bench_transform_enter_in_list_item,
    bench_transform_enter_in_blockquote,
    bench_transform_enter_in_heading,
    bench_transform_large_document,
    bench_transform_wrap_large_document,
);
criterion_main!(benches);
