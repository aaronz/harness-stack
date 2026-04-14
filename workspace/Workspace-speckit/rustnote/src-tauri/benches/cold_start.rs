//! Cold Start Benchmarks
//!
//! These benchmarks measure the time required to perform cold start operations,
//! which are critical for user experience. They test:
//!
//! - TC-B001: Cold start with empty document (< 2000ms)
//! - TC-B002: Cold start with 1MB document (< 3000ms)
//! - TC-B003: Hot file open performance (< 500ms)

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustnote_lib::semantic::ast::SemanticDocument;

/// Generate a large document (approximately 1MB) with mixed content
fn generate_large_document() -> String {
    let mut content = String::with_capacity(1_000_000);

    // Repeat the same pattern to fill ~1MB
    // Each section is approximately 2KB, so we need ~500 sections
    let section = format!(
        "# Section {{}}\n\n\
        ## Subsection {{}}.1\n\n\
        This is a paragraph with **bold** and *italic* text. \
        It contains `inline code` and various Markdown elements. \
        This paragraph is intentionally longer to help fill the document size.\n\n\
        - List item {{}}.1\n\
        - List item {{}}.2\n\
        - List item {{}}.3\n\
        - List item {{}}.4\n\
        - List item {{}}.5\n\
        - List item {{}}.6\n\
        - List item {{}}.7\n\
        - List item {{}}.8\n\
        - List item {{}}.9\n\
        - List item {{}}.10\n\n\
        > This is a blockquote that spans multiple lines.\n\
        > It contains important information.\n\
        > And even more lines of blockquote text.\n\n\
        ```rust\n\
        fn example_{{}}(x: i32) -> i32 {{\n\
            let result = x * 42;\n\
            println!(\"Hello, world! The result is: {{}}\", result);\n\
            return result;\n\
        }}\n\
        ```\n\n\
        Some more text here to pad the document size.\n\
        And even more text to make sure we reach 1MB.\n\
        Adding some Lorem Ipsum style text: \
        Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n\n",
    );

    // Generate sections until we reach approximately 1MB
    let mut i = 0;
    while content.len() < 950_000 {
        let section_content = section.replace("{{}}", &i.to_string());
        content.push_str(&section_content);
        i += 1;
    }

    content
}

/// TC-B001: Cold start with empty document
/// Target: < 2000ms
fn bench_cold_start_empty(c: &mut Criterion) {
    c.bench_function("cold_start_empty", |b| {
        b.iter(|| {
            // Measure parsing of empty document
            let _doc = SemanticDocument::parse(black_box(""));
        });
    });
}

/// TC-B001: Cold start with minimal document (1 line)
/// Target: < 2000ms
fn bench_cold_start_minimal(c: &mut Criterion) {
    c.bench_function("cold_start_minimal", |b| {
        b.iter(|| {
            let _doc = SemanticDocument::parse(black_box("# Title\n"));
        });
    });
}

/// TC-B002: Cold start with 1MB document
/// Target: < 3000ms
fn bench_cold_start_large(c: &mut Criterion) {
    let large_doc = generate_large_document();
    let size = large_doc.len();

    // Ensure document is approximately 1MB
    assert!(
        size >= 900_000 && size <= 1_100_000,
        "Generated document size {} is not approximately 1MB",
        size
    );

    c.bench_function("cold_start_large", |b| {
        b.iter(|| {
            let _doc = SemanticDocument::parse(black_box(&large_doc));
        });
    });

    // Also log the actual size for reference
    println!("Large document size: {} bytes", size);
}

/// TC-B002: Cold start with moderately sized document (100KB)
fn bench_cold_start_medium(c: &mut Criterion) {
    let mut content = String::new();
    for i in 0..100 {
        content.push_str(&format!("# Section {}\n\n", i));
        content.push_str("This is a paragraph with some content.\n\n");
        for j in 0..5 {
            content.push_str(&format!("- Item {}.{}\n", i, j));
        }
        content.push('\n');
    }

    c.bench_function("cold_start_medium", |b| {
        b.iter(|| {
            let _doc = SemanticDocument::parse(black_box(&content));
        });
    });
}

/// TC-B003: Hot file open performance
/// Measures re-parsing a previously parsed document (simulating hot reload)
/// Target: < 500ms
fn bench_hot_open(c: &mut Criterion) {
    let source = r#"# Document Title

## Section 1

This is a paragraph with **bold** and *italic* text.

### Subsection 1.1

- List item 1
- List item 2
- List item 3

> A blockquote

## Section 2

| Column 1 | Column 2 |
|----------|----------|
| Data 1   | Data 2   |

```rust
fn main() {
    println!("Hello, world!");
}
```

- [x] Completed task
- [ ] Pending task

Another paragraph with `inline code`.

## Section 3

More content here.
"#;

    // First, parse the document once (cold)
    let _cold_doc = SemanticDocument::parse(source);

    // Now benchmark re-parsing (hot)
    c.bench_function("hot_open_recently_closed", |b| {
        b.iter(|| {
            let _doc = SemanticDocument::parse(black_box(source));
        });
    });
}

/// TC-B003: Hot open with larger document
fn bench_hot_open_medium(c: &mut Criterion) {
    let mut content = String::new();
    for i in 0..50 {
        content.push_str(&format!("# Section {}\n\n", i));
        content.push_str("This is a paragraph with some content.\n\n");
        for j in 0..10 {
            content.push_str(&format!("- Item {}.{}\n", i, j));
        }
        content.push('\n');
    }

    // Cold parse
    let _cold_doc = SemanticDocument::parse(&content);

    // Hot parse
    c.bench_function("hot_open_medium", |b| {
        b.iter(|| {
            let _doc = SemanticDocument::parse(black_box(&content));
        });
    });
}

/// Memory benchmark baseline for cold start
fn bench_memory_baseline(c: &mut Criterion) {
    c.bench_function("memory_baseline_empty_doc", |b| {
        b.iter(|| {
            let doc = SemanticDocument::parse("");
            // Force evaluation
            black_box(doc.get_headings());
        });
    });
}

criterion_group!(
    benches,
    bench_cold_start_empty,
    bench_cold_start_minimal,
    bench_cold_start_large,
    bench_cold_start_medium,
    bench_hot_open,
    bench_hot_open_medium,
    bench_memory_baseline,
);
criterion_main!(benches);
