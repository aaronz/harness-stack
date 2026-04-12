use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rustnote_lib::semantic::ast::SemanticDocument;

fn create_test_doc(source: &str) -> SemanticDocument {
    SemanticDocument::parse(source)
}

fn bench_serialize_commonmark(c: &mut Criterion) {
    let sources = vec![
        "# Heading",
        "**bold** *italic*",
        "- item 1\n- item 2\n- item 3",
        "| A | B |\n|---|---|\n| 1 | 2 |",
    ];

    let docs: Vec<_> = sources.iter().map(|s| create_test_doc(s)).collect();

    c.bench_function("serialize_commonmark_simple", |b| {
        b.iter(|| {
            let doc = black_box(&docs[0]);
            doc.serialize_to_commonmark()
        });
    });

    for (i, doc) in docs.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("serialize_commonmark", i), doc, |b, d| {
            b.iter(|| d.serialize_to_commonmark())
        });
    }
}

fn bench_serialize_with_frontmatter(c: &mut Criterion) {
    let source = "---\ntitle: Test\nauthor: Author\n---\n\n# Content";
    let doc = create_test_doc(source);

    c.bench_function("serialize_with_frontmatter", |b| {
        b.iter(|| {
            let d = black_box(&doc);
            d.serialize_with_frontmatter()
        });
    });
}

fn bench_serialize_html(c: &mut Criterion) {
    let sources = vec![
        "# Heading",
        "**bold** *italic* `code`",
        "- item 1\n- item 2\n- item 3\n\n> quote",
    ];

    let docs: Vec<_> = sources.iter().map(|s| create_test_doc(s)).collect();

    for (i, doc) in docs.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("serialize_html", i), doc, |b, d| {
            b.iter(|| d.html())
        });
    }
}

fn bench_serialize_html_with_frontmatter(c: &mut Criterion) {
    let source = "---\ntitle: Test\n---\n\n# Content";
    let doc = create_test_doc(source);

    c.bench_function("serialize_html_with_frontmatter", |b| {
        b.iter(|| {
            let d = black_box(&doc);
            d.html_with_frontmatter()
        });
    });
}

fn bench_get_headings(c: &mut Criterion) {
    let source = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let doc = create_test_doc(source);

    c.bench_function("get_headings", |b| {
        b.iter(|| {
            let d = black_box(&doc);
            d.get_headings()
        });
    });
}

fn bench_position_conversion(c: &mut Criterion) {
    let source = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let doc = create_test_doc(source);

    c.bench_function("offset_to_position", |b| {
        b.iter(|| {
            let d = black_box(&doc);
            d.offset_to_position(black_box(15))
        });
    });

    c.bench_function("position_to_offset", |b| {
        b.iter(|| {
            let d = black_box(&doc);
            use rustnote_lib::semantic::ast::Position;
            d.position_to_offset(Position::new(15, 1, 5))
        });
    });
}

fn bench_roundtrip(c: &mut Criterion) {
    let source = r#"# Title

## Section 1

This is **bold** and *italic*.

- Item 1
- Item 2

> A blockquote

```rust
fn main() {}
```
"#;
    let doc = create_test_doc(source);

    c.bench_function("roundtrip_commonmark", |b| {
        b.iter(|| {
            let d = black_box(&doc);
            let output = d.serialize_to_commonmark();
            SemanticDocument::parse(&output)
        });
    });
}

criterion_group!(
    benches,
    bench_serialize_commonmark,
    bench_serialize_with_frontmatter,
    bench_serialize_html,
    bench_serialize_html_with_frontmatter,
    bench_get_headings,
    bench_position_conversion,
    bench_roundtrip,
);
criterion_main!(benches);
