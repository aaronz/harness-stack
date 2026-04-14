# Performance Benchmarks

This document describes the automated performance benchmark infrastructure for RustNote, covering all Non-Functional Requirements (NFR-001 through NFR-008).

## Overview

Performance benchmarks are essential for ensuring RustNote meets user experience requirements for responsiveness. The benchmark suite validates that all operations complete within specified thresholds.

## Running Benchmarks

### Rust Benchmarks

Run all Rust benchmarks:
```bash
cd rustnote/src-tauri
cargo bench
```

Run specific benchmark suites:
```bash
cargo bench --bench cold_start       # Cold start benchmarks
cargo bench --bench nfr_thresholds   # NFR threshold validation
cargo bench --bench transforms        # Transform operation benchmarks
cargo bench --bench parsing           # Parsing benchmarks
cargo bench --bench serialization     # Serialization benchmarks
```

### Frontend Benchmarks

Run frontend benchmarks:
```bash
cd rustnote/www
npm run bench
```

## Non-Functional Requirements (NFRs)

| NFR ID | Requirement | Target | Test Case |
|--------|-------------|--------|-----------|
| NFR-001 | Cold start (empty document) | < 2000ms | TC-B001 |
| NFR-002 | Cold start (1MB document) | < 3000ms | TC-B002 |
| NFR-003 | Hot file open | < 500ms | TC-B003 |
| NFR-004 | Keystroke-to-render | < 100ms | TC-B004 |
| NFR-005 | Save operation | < 200ms | TC-B005 |
| NFR-006 | PDF export (10 pages) | < 5000ms | TC-B006 |
| NFR-007 | Memory idle (10 docs) | < 300MB | TC-B007 |
| NFR-008 | Large doc scroll | 60 FPS | TC-B008 |

## Test Cases

### TC-B001: Cold Start with Empty Document

**Category:** performance

**Input:** Empty `.md` file

**Expected:** Cold start time < 2000ms

**Benchmark:** `cold_start_empty`, `cold_start_minimal`

### TC-B002: Cold Start with 1MB Document

**Category:** performance

**Input:** 1MB `.md` file with mixed content

**Expected:** Cold start time < 3000ms

**Benchmark:** `cold_start_large`

### TC-B003: Hot File Open Performance

**Category:** performance

**Input:** Re-opening recently closed document

**Expected:** Hot open time < 500ms

**Benchmark:** `hot_open_recently_closed`, `hot_open_medium`

### TC-B004: Keystroke-to-Render Latency

**Category:** performance

**Input:** Single character insertion event

**Expected:** Processing time < 100ms

**Benchmark:** `keystroke_single_char_insert`, `keystroke_sequence_100chars`

### TC-B005: Save Operation Performance

**Category:** performance

**Input:** Document with 1000 lines, trigger save

**Expected:** Save time < 200ms

**Benchmark:** `save_document_1000_lines`, `save_html_serialization`

### TC-B006: PDF Export 10 Pages

**Category:** performance

**Input:** 10-page Markdown document

**Expected:** Export time < 5000ms

**Benchmark:** `pdf_export_html_preparation_10pages`, `pdf_export_html_preparation_50pages`

### TC-B007: Memory Idle with 10 Documents

**Category:** performance

**Input:** 10 open documents, no interaction for 30s

**Expected:** Memory usage < 300MB

**Benchmark:** `parse_10_documents`, `memoryIdle`

### TC-B008: Large Document Scroll

**Category:** performance

**Input:** 50000-line document, scroll event

**Expected:** Frame time < 100ms (10fps minimum)

**Benchmark:** `parse_50000_lines`, `scroll_frame_simulation`, `render_visible_chunks`

### TC-B009: Transform Operations Benchmark

**Category:** performance

**Input:** Document with 1000 and 100000 characters

**Expected:** All transform operations complete within thresholds

**Benchmark:**
- `transform_wrap_bold`, `transform_wrap_italic`, `transform_wrap_code`
- `transform_enter_in_list_item_*`, `transform_enter_in_blockquote_*`, `transform_enter_in_heading_*`
- `transform_wrap_1000_chars`, `transform_wrap_100000_chars`
- `transform_enter_1000_chars`, `transform_enter_100000_chars`

### TC-B010: Incremental Parse Benchmark

**Category:** performance

**Input:** Document with 100 lines, single line change

**Expected:** Incremental parse < 50ms

**Benchmark:** `incremental_parse_initial`, `incremental_parse_line_change`, `incremental_parse_single_char_change`

### TC-B011: Full Parse Benchmark

**Category:** performance

**Input:** 10000-line document

**Expected:** Full parse completes in reasonable time

**Benchmark:** `full_parse_10000_lines`, `full_parse_5000_sections_mixed`, `full_parse_deeply_nested`

## Benchmark Suite Structure

### Rust Benchmarks (`src-tauri/benches/`)

| File | Purpose |
|------|---------|
| `cold_start.rs` | Cold start and hot reload performance |
| `nfr_thresholds.rs` | NFR validation benchmarks |
| `transforms.rs` | Transform operation benchmarks |
| `parsing.rs` | Markdown parsing benchmarks |
| `serialization.rs` | Serialization benchmarks |

### Frontend Benchmarks (`www/scripts/`)

| File | Purpose |
|------|---------|
| `bench.js` | Browser-based performance metrics |

## CI Integration

Performance benchmarks run automatically on:

- Push to `main` or `master` branches
- Pull requests to `main` or `master` branches
- Weekly schedule (Sundays at midnight)

### GitHub Actions Workflow

See `.github/workflows/bench.yml` for the complete workflow configuration.

## Benchmark Results

Benchmark results are saved to:
- Rust: `target/criterion/` (criterion output)
- Frontend: `benchmark-results.json`

## Threshold Definitions

```rust
const NFR001_COLD_START_EMPTY_MS: u128 = 2000;
const NFR002_COLD_START_1MB_MS: u128 = 3000;
const NFR003_HOT_OPEN_MS: u128 = 500;
const NFR004_KEYSTROKE_RENDER_MS: u128 = 100;
const NFR005_SAVE_OPERATION_MS: u128 = 200;
const NFR006_PDF_EXPORT_10PAGES_MS: u128 = 5000;
const NFR007_MEMORY_IDLE_10DOCS_MB: u128 = 300;
const NFR008_SCROLL_FRAME_MS: u128 = 100;
```

## Coverage

### Markdown Syntax Coverage

- Headings (H1-H6)
- Bold, italic, strikethrough
- Inline code and code blocks
- Lists (ordered, unordered, task)
- Blockquotes
- Tables
- Links and images
- Frontmatter

### Edge Cases Covered

- `cold_start_empty`: Empty document initialization
- `cold_start_large`: Large document (>1MB) parsing
- `hot_reload`: Previously parsed document re-parsing
- `memory_idle`: Multi-document memory footprint
- `scroll_performance`: Large document scrolling

## Interpreting Results

### Passing Benchmarks

A benchmark passes when the measured time is below the defined threshold.

### Warning Thresholds

Benchmarks may show warnings when:
- Results are close to thresholds
- There is significant variance between runs

### Failure Indicators

- Results exceeding thresholds consistently
- Large variance in repeated measurements
- Memory usage exceeding limits

## Adding New Benchmarks

To add a new benchmark:

1. Add the benchmark function to the appropriate file in `benches/`
2. Follow the naming convention: `bench_<category>_<name>`
3. Include TC reference in comments
4. Register in the `criterion_group!`
5. Update this documentation
