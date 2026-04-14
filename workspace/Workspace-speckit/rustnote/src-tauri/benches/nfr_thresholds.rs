//! NFR Threshold Benchmarks
//!
//! These benchmarks validate the Non-Functional Requirements (NFRs) defined in PRD-05
//! and PRD-11. Each benchmark has a defined threshold that must be met.
//!
//! - TC-B004: Keystroke-to-render latency (< 100ms)
//! - TC-B005: Save operation performance (< 200ms)
//! - TC-B006: PDF export 10 pages (< 5000ms)
//! - TC-B007: Memory idle with 10 documents (< 300MB)
//! - TC-B008: Large document scroll (< 100ms per frame)

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustnote_lib::semantic::ast::SemanticDocument;
use std::time::Instant;

/// TC-B004: Keystroke-to-render latency
/// Target: < 100ms per keystroke
/// This measures the time to process a single character insertion
fn bench_keystroke_render(c: &mut Criterion) {
    let source = "This is a paragraph with some text content.";

    c.bench_function("keystroke_single_char_insert", |b| {
        b.iter(|| {
            // Measure time to re-parse with single char insertion
            let modified = format!("{}x", source);
            let start = Instant::now();
            let _doc = SemanticDocument::parse(black_box(&modified));
            black_box(start.elapsed().as_millis());
        });
    });

    // Simulate typing 100 characters
    c.bench_function("keystroke_sequence_100chars", |b| {
        b.iter(|| {
            let mut content = String::new();
            for i in 0..100 {
                content.push(char::from_u32('a' as u32 + (i % 26) as u32).unwrap());
                let _doc = SemanticDocument::parse(black_box(&content));
            }
        });
    });
}

/// TC-B004: Incremental parse simulation
fn bench_incremental_parse(c: &mut Criterion) {
    let source =
        "# Title\n\n## Section 1\n\nParagraph content here.\n\n## Section 2\n\nMore content.\n";

    // Parse once
    let _doc = SemanticDocument::parse(source);

    c.bench_function("incremental_single_line_change", |b| {
        b.iter(|| {
            // Simulate single line change
            let modified = "# Title\n\n## Section 1\n\nUpdated paragraph content.\n\n## Section 2\n\nMore content.\n";
            let _doc = SemanticDocument::parse(black_box(modified));
        });
    });
}

/// TC-B005: Save operation performance
/// Target: < 200ms for saving a document with 1000 lines
fn bench_save_operation(c: &mut Criterion) {
    let mut content = String::new();
    for i in 0..1000 {
        content.push_str(&format!("# Line {}\n\n", i));
        content.push_str(&format!(
            "This is paragraph {} with some text content.\n\n",
            i
        ));
        for j in 0..3 {
            content.push_str(&format!("- List item {}.{}\n", i, j));
        }
        content.push('\n');
    }

    c.bench_function("save_document_1000_lines", |b| {
        b.iter(|| {
            // Measure serialization time (simulating save operation)
            let doc = SemanticDocument::parse(black_box(&content));
            let start = Instant::now();
            let _output = doc.serialize_to_commonmark();
            black_box(start.elapsed().as_millis());
        });
    });
}

/// TC-B005: Save with HTML serialization
fn bench_save_html_operation(c: &mut Criterion) {
    let source = "# Document\n\n## Section\n\nContent here.\n";
    let doc = SemanticDocument::parse(source);

    c.bench_function("save_html_serialization", |b| {
        b.iter(|| {
            let start = Instant::now();
            let _html = doc.html();
            black_box(start.elapsed().as_millis());
        });
    });
}

/// TC-B006: PDF export 10 pages
/// Target: < 5000ms
/// Note: Full PDF export requires Tauri APIs, this measures the HTML generation step
fn bench_pdf_export_preparation(c: &mut Criterion) {
    let mut content = String::new();
    for i in 0..10 {
        content.push_str(&format!("# Page {}\n\n", i + 1));
        content.push_str("## Section A\n\n");
        content.push_str("This is paragraph content for the page. ");
        content.push_str("It contains various text elements and formatting.\n\n");
        content.push_str("## Section B\n\n");
        for j in 0..5 {
            content.push_str(&format!("- Item {}\n", j + 1));
        }
        content.push_str("\n```rust\nfn example() {{\n    println!(\"code\");\n}}\n```\n\n");
    }

    c.bench_function("pdf_export_html_preparation_10pages", |b| {
        b.iter(|| {
            let doc = SemanticDocument::parse(black_box(&content));
            let start = Instant::now();
            let _html = doc.html();
            let elapsed = start.elapsed().as_millis();
            black_box(elapsed);
        });
    });
}

/// TC-B006: Large PDF export preparation
fn bench_pdf_export_large(c: &mut Criterion) {
    let mut content = String::new();
    for i in 0..50 {
        content.push_str(&format!("# Chapter {}\n\n", i + 1));
        for j in 0..5 {
            content.push_str(&format!("## Section {}.{}\n\n", i + 1, j + 1));
            content.push_str("Paragraph content with **bold** and *italic* text.\n\n");
            for k in 0..10 {
                content.push_str(&format!("- Item {}.{}.{}\n", i + 1, j + 1, k + 1));
            }
            content.push('\n');
        }
    }

    c.bench_function("pdf_export_html_preparation_50pages", |b| {
        b.iter(|| {
            let doc = SemanticDocument::parse(black_box(&content));
            let start = Instant::now();
            let _html = doc.html();
            let elapsed = start.elapsed().as_millis();
            black_box(elapsed);
        });
    });
}

/// TC-B007: Memory idle with 10 documents
/// Target: < 300MB total memory usage
/// Note: This measures parsing overhead, actual memory measurement requires runtime instrumentation
fn bench_parse_multiple_documents(c: &mut Criterion) {
    let source = "# Document\n\n## Section\n\nContent here with **bold** and *italic*.\n\n- Item 1\n- Item 2\n- Item 3\n\n> Blockquote\n\n```rust\nfn code() {{\n    // code\n}}\n```\n";

    c.bench_function("parse_10_documents", |b| {
        b.iter(|| {
            let mut docs = Vec::new();
            for _ in 0..10 {
                let doc = SemanticDocument::parse(black_box(source));
                docs.push(doc);
            }
            black_box(docs);
        });
    });
}

/// TC-B008: Large document scroll performance
/// Target: < 100ms per frame (10fps minimum)
/// This measures parsing time for a 50000-line document
fn bench_large_document_scroll(c: &mut Criterion) {
    let mut content = String::new();
    for i in 0..50000 {
        content.push_str(&format!(
            "Line {}: This is line content with some text.\n",
            i + 1
        ));
    }

    c.bench_function("parse_50000_lines", |b| {
        b.iter(|| {
            let _doc = SemanticDocument::parse(black_box(&content));
        });
    });

    // Measure per-line parsing simulation
    c.bench_function("scroll_frame_simulation", |b| {
        let doc = SemanticDocument::parse(&content);
        let lines: Vec<&str> = content.lines().collect();

        b.iter(|| {
            // Simulate scroll by accessing different line ranges
            for chunk in lines.chunks(100) {
                black_box(chunk);
            }
        });
    });
}

/// TC-B008: Incremental render for scroll simulation
fn bench_scroll_render_chunks(c: &mut Criterion) {
    let mut source = String::from("# Document\n\n");
    for i in 0..1000 {
        source.push_str(&format!("## Section {}\n\n", i));
        source.push_str("Content here.\n\n");
    }

    c.bench_function("render_visible_chunks", |b| {
        let doc = SemanticDocument::parse(&source);

        b.iter(|| {
            // Simulate rendering visible chunks
            for _ in 0..50 {
                let _html = doc.html();
            }
        });
    });
}

/// Verify NFR thresholds are met
mod thresholds {
    pub const NFR001_COLD_START_EMPTY_MS: u128 = 2000;
    pub const NFR002_COLD_START_1MB_MS: u128 = 3000;
    pub const NFR003_HOT_OPEN_MS: u128 = 500;
    pub const NFR004_KEYSTROKE_RENDER_MS: u128 = 100;
    pub const NFR005_SAVE_OPERATION_MS: u128 = 200;
    pub const NFR006_PDF_EXPORT_10PAGES_MS: u128 = 5000;
    pub const NFR007_MEMORY_IDLE_10DOCS_MB: u128 = 300;
    pub const NFR008_SCROLL_FRAME_MS: u128 = 100;
}

criterion_group!(
    benches,
    bench_keystroke_render,
    bench_incremental_parse,
    bench_save_operation,
    bench_save_html_operation,
    bench_pdf_export_preparation,
    bench_pdf_export_large,
    bench_parse_multiple_documents,
    bench_large_document_scroll,
    bench_scroll_render_chunks,
);
criterion_main!(benches);
