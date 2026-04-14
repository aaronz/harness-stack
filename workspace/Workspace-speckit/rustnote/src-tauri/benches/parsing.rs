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

/// TC-B010: Incremental parse benchmark
/// Target: Incremental parse < 50ms for single line change
fn bench_incremental_parse(c: &mut Criterion) {
    let base_source = "# Title\n\n## Section 1\n\nThis is paragraph 1 content.\n\n## Section 2\n\nThis is paragraph 2 content.\n\n## Section 3\n\nThis is paragraph 3 content.\n\n".repeat(20);

    c.bench_function("incremental_parse_initial", |b| {
        b.iter(|| SemanticDocument::parse(black_box(&base_source)))
    });

    let offsets: Vec<usize> = vec![10, 50, 100, 500];
    for (i, line_offset) in offsets.iter().enumerate() {
        let source_part = &base_source[..*line_offset];
        let modified = format!("{}modified line\n", source_part);

        c.bench_with_input(
            BenchmarkId::new("incremental_parse_line_change", i),
            &source_part,
            |b, s| {
                let modified_source = format!("{}modified line\n", s);
                b.iter(|| SemanticDocument::parse(black_box(&modified_source)))
            },
        );
    }
}

/// TC-B010: Single character change simulation
fn bench_incremental_parse_single_char(c: &mut Criterion) {
    let source = "This is a test paragraph with some content.\n".repeat(100);

    c.bench_function("incremental_parse_single_char_change", |b| {
        b.iter(|| {
            let mut chars: Vec<char> = source.chars().collect();
            chars[50] = 'X';
            let modified: String = chars.into_iter().collect();
            SemanticDocument::parse(black_box(&modified))
        });
    });
}

/// TC-B011: Full parse benchmark
/// Target: Full parse of 10000-line document completes in reasonable time
fn bench_full_parse_10000_lines(c: &mut Criterion) {
    let mut content = String::new();
    for i in 0..10000 {
        content.push_str(&format!("# Heading {}\n", i));
        content.push_str("Paragraph content with **bold** and *italic*.\n");
        content.push_str("- List item 1\n- List item 2\n");
    }

    c.bench_function("full_parse_10000_lines", |b| {
        b.iter(|| SemanticDocument::parse(black_box(&content)))
    });
}

/// TC-B011: Document with mixed content types
fn bench_full_parse_mixed_content(c: &mut Criterion) {
    let mut content = String::new();

    for i in 0..5000 {
        content.push_str(&format!("# Document Part {}\n\n", i));
        content.push_str("## Section A\n\n");
        content.push_str("Paragraph with **bold**, *italic*, and `code`.\n\n");
        content.push_str("## Section B\n\n");
        for j in 0..5 {
            content.push_str(&format!("- Item {}.{}\n", i, j));
        }
        content.push('\n');
        content.push_str("> Blockquote text here.\n\n");
        content.push_str("```rust\nfn example() {{\n    println!(\"test\");\n}}\n```\n\n");
    }

    c.bench_function("full_parse_5000_sections_mixed", |b| {
        b.iter(|| SemanticDocument::parse(black_box(&content)))
    });
}

/// TC-B011: Complex nested structures
fn bench_full_parse_nested_structures(c: &mut Criterion) {
    let mut content = String::new();

    for i in 0..1000 {
        content.push_str(&format!("# Level 1 - {}\n\n", i));
        for j in 0..5 {
            content.push_str(&format!("  ## Level 2 - {}.{}\n\n", i, j));
            for k in 0..5 {
                content.push_str(&format!("    ### Level 3 - {}.{}.{}\n\n", i, j, k));
                content.push_str("    Content at level 3.\n\n");
                content
                    .push_str("    - Nested item 1\n    - Nested item 2\n    - Nested item 3\n\n");
            }
        }
    }

    c.bench_function("full_parse_deeply_nested", |b| {
        b.iter(|| SemanticDocument::parse(black_box(&content)))
    });
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
    bench_incremental_parse,
    bench_incremental_parse_single_char,
    bench_full_parse_10000_lines,
    bench_full_parse_mixed_content,
    bench_full_parse_nested_structures,
);
criterion_main!(benches);
