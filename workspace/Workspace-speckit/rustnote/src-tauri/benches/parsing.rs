use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rustnote_lib::semantic::ast::SemanticDocument;

fn bench_parsing_headings(c: &mut Criterion) {
    let sources = vec![
        "# H1",
        "# H1\n## H2\n### H3",
        "# Title\n\n## Section 1\n### Subsection\n\n## Section 2\n### Subsection\n\n### Deep",
    ];

    c.bench_function("parse_headings_single", |b| {
        b.iter(|| SemanticDocument::parse(black_box("# Heading")));
    });

    for (i, source) in sources.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("parse_headings", i), source, |b, s| {
            b.iter(|| SemanticDocument::parse(black_box(*s)))
        });
    }
}

fn bench_parsing_emphasis(c: &mut Criterion) {
    let sources = vec![
        "**bold**",
        "*italic*",
        "~~strike~~",
        "***bold italic***",
        "`inline code`",
        "**bold** *italic* ~~strike~~ `code`",
    ];

    for (i, source) in sources.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("parse_emphasis", i), source, |b, s| {
            b.iter(|| SemanticDocument::parse(black_box(*s)))
        });
    }
}

fn bench_parsing_lists(c: &mut Criterion) {
    let sources = vec![
        "- item",
        "- item 1\n- item 2\n- item 3",
        "- [ ] task",
        "- [x] done",
        "1. first\n2. second\n3. third",
        "- item\n  - nested\n    - deep nested",
    ];

    for (i, source) in sources.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("parse_lists", i), source, |b, s| {
            b.iter(|| SemanticDocument::parse(black_box(*s)))
        });
    }
}

fn bench_parsing_code_blocks(c: &mut Criterion) {
    let sources = vec![
        "```\ncode\n```",
        "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```",
        "```javascript\nconsole.log('hello');\nconst x = 1;\n```",
    ];

    for (i, source) in sources.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("parse_code_blocks", i), source, |b, s| {
            b.iter(|| SemanticDocument::parse(black_box(*s)))
        });
    }
}

fn bench_parsing_tables(c: &mut Criterion) {
    let sources = vec![
        "| A | B |\n|---|---|\n| 1 | 2 |",
        "| Header 1 | Header 2 | Header 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |",
        "| A | B | C | D | E | F | G | H | I | J |\n|---|---|---|---|---|---|---|---|---|---|\n| 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |",
    ];

    for (i, source) in sources.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("parse_tables", i), source, |b, s| {
            b.iter(|| SemanticDocument::parse(black_box(*s)))
        });
    }
}

fn bench_parsing_complex_document(c: &mut Criterion) {
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

    c.bench_function("parse_complex_document", |b| {
        b.iter(|| SemanticDocument::parse(black_box(source)))
    });
}

fn bench_parsing_frontmatter(c: &mut Criterion) {
    let sources = vec![
        "---\ntitle: Test\nauthor: Author\n---\n\n# Content",
        "---\ntitle: Document\nauthor: Author\ndate: 2024-01-01\ntags:\n  - rust\n  - markdown\n---\n\n# Title\n\nContent",
    ];

    for (i, source) in sources.iter().enumerate() {
        c.bench_with_input(BenchmarkId::new("parse_frontmatter", i), source, |b, s| {
            b.iter(|| SemanticDocument::parse(black_box(*s)))
        });
    }
}

criterion_group!(
    benches,
    bench_parsing_headings,
    bench_parsing_emphasis,
    bench_parsing_lists,
    bench_parsing_code_blocks,
    bench_parsing_tables,
    bench_parsing_complex_document,
    bench_parsing_frontmatter,
);
criterion_main!(benches);
