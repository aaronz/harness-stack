use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustnote_lib::editor::transforms::{Transform, TransformEngine};

fn engine() -> TransformEngine {
    TransformEngine::new()
}

fn bench_transform_enter(c: &mut Criterion) {
    c.bench_function("transform_enter_simple", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "- item", 6)
        });
    });

    c.bench_function("transform_enter_heading", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "# Heading", 9)
        });
    });

    c.bench_function("transform_enter_paragraph", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "paragraph", 5)
        });
    });

    c.bench_function("transform_enter_ordered_list", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "1. item", 7)
        });
    });

    c.bench_function("transform_enter_blockquote", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "> quote", 7)
        });
    });

    c.bench_function("transform_enter_task_list", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Enter, "- [ ] task", 10)
        });
    });
}

fn bench_transform_backspace(c: &mut Criterion) {
    c.bench_function("backspace_middle", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Backspace, "Hello world", 6)
        });
    });

    c.bench_function("backspace_list_boundary", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Backspace, "- item\n- item2", 10)
        });
    });

    c.bench_function("backspace_line_join", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Backspace, "line1\nline2", 6)
        });
    });
}

fn bench_transform_tab(c: &mut Criterion) {
    c.bench_function("tab_list_item", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Tab, "- item", 2)
        });
    });

    c.bench_function("tab_nested_list", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Tab, "- nested item", 2)
        });
    });

    c.bench_function("tab_paragraph", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::Tab, "paragraph", 5)
        });
    });
}

fn bench_transform_shift_tab(c: &mut Criterion) {
    c.bench_function("shift_tab_four_spaces", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::ShiftTab, "    - item", 6)
        });
    });

    c.bench_function("shift_tab_single_tab", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::ShiftTab, "\t-item", 1)
        });
    });

    c.bench_function("shift_tab_no_indent", |b| {
        b.iter(|| {
            let e = engine();
            e.apply(&Transform::ShiftTab, "paragraph", 5)
        });
    });
}

fn bench_transform_sequence(c: &mut Criterion) {
    c.bench_function("transform_sequence", |b| {
        b.iter(|| {
            let e = engine();
            let r1 = e.apply(&Transform::Enter, "- item", 6);
            let r2 = e.apply(&Transform::Tab, &r1.content, r1.cursor_offset);
            let _ = e.apply(&Transform::Enter, &r2.content, r2.cursor_offset);
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
);
criterion_main!(benches);
