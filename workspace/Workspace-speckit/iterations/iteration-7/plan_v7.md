# RustNote Implementation Plan — Iteration 7

**Project:** RustNote - Typora-like Markdown Editor
**Phase:** Phase 1 — MVP Completion
**Version:** 7.0
**Created:** 2026-04-14
**Based on:** Iteration-7 Gap Analysis (Spec v3.1)
**MVP Readiness:** ~90% → Target 98%
**Priority Order:** P0 → P1 → P2

---

## 1. Priority Summary

| Priority | Count | Description | Unblock |
|----------|-------|-------------|---------|
| **P0** | 3 | Blocking issues before MVP release | MVP completion |
| **P1** | 6 | High-priority improvements | Feature completeness |
| **P2** | 8 | Polish and medium-priority items | Quality |
| **Tech Debt** | 5 | Code quality and documentation | Maintainability |

---

## 2. P0 — Must Fix Before MVP Release

### 2.1 G-003: Automated Performance Benchmark Infrastructure

**Status:** Not implemented
**NFRs Affected:** NFR-001 through NFR-008
**Root Cause:** No `cargo bench` integration, no frontend benchmark scripts

**Implementation Approach:**
1. Create Rust benchmark suite under `src-tauri/benches/`
   - Extend existing benches (`transforms.rs`, `serialization.rs`, `parsing.rs`) with NFR-specific benchmarks
   - Add cold start benchmark (empty doc, 1MB doc)
   - Add hot file open benchmark
   - Add keystroke-to-render benchmark (mock Rust side)
   - Add save operation benchmark
   - Add PDF export benchmark
   - Add memory usage benchmark (idle, 10 docs)
2. Create frontend benchmark script `www/scripts/bench.js`
   - Keystroke latency measurement
   - Scroll FPS measurement
   - Large document load time
3. Add benchmark results baseline files
4. Document NFR thresholds in `docs/perf-benchmarks.md`
5. Integrate into CI pipeline (GitHub Actions workflow)

**Success Criteria:**
- `cargo bench` runs all NFR benchmarks and reports pass/fail
- Frontend benchmarks report within NFR thresholds
- Benchmarks are reproducible (deterministic where possible)

**Files to Create/Modify:**
- `src-tauri/benches/cold_start.rs` (NEW)
- `src-tauri/benches/nfr_thresholds.rs` (NEW)
- `src-tauri/benches/transforms.rs` (EXTEND)
- `src-tauri/benches/serialization.rs` (EXTEND)
- `src-tauri/benches/parsing.rs` (EXTEND)
- `src-tauri/.github/workflows/bench.yml` (NEW)
- `www/scripts/bench.js` (NEW)
- `docs/perf-benchmarks.md` (NEW)

---

### 2.2 G-001: Bidirectional Cursor Mapping Verification

**Status:** `build_cursor_mapping()` and `dom_to_source()` exist, edge cases unverified
**FR Affected:** FR-022
**Root Cause:** `semantic/position.rs` — `CursorMapping` lacks full edge case coverage

**Implementation Approach:**
1. Audit existing `semantic/position.rs` cursor mapping implementation
2. Identify edge cases not covered:
   - Nested structures (list inside blockquote)
   - Code spans adjacent to structural elements
   - Multi-byte character handling (UTF-8)
   - Empty nodes (empty headings, empty list items)
   - Adjacent code fences vs inline code
3. Add unit tests for each edge case
4. Add TipTap integration test (manual verification with real editor state)
5. Fix `dom_to_source()` for identified edge cases
6. Verify `build_cursor_mapping()` for source edits that don't invalidate full mapping

**Success Criteria:**
- All edge cases in `tests/cursor_mapping_tests.rs` pass
- DOM position X → Source position Y → DOM position X' converges (X' == X)
- TipTap integration test passes with complex Markdown

**Files to Modify:**
- `src-tauri/src/semantic/position.rs` — Fix edge cases
- `src-tauri/tests/cursor_mapping_tests.rs` — Add edge case tests
- `www/src/components/TipTapEditor.jsx` — Integration verification

---

### 2.3 G-002: PDF Export Quality Verification & Improvement

**Status:** `printpdf` implementation exists, quality unverified
**FR Affected:** FR-032
**Root Cause:** Complex Markdown (tables, code blocks) may not render correctly in printpdf output

**Implementation Approach:**
1. Generate sample PDFs with test Markdown covering:
   - Tables (GFM)
   - Code blocks with syntax highlighting
   - Images (relative paths)
   - Nested blockquotes
   - Task lists
   - Horizontal rules
2. Inspect output quality visually
3. If quality issues found, evaluate alternatives:
   - Option A: HTML-to-PDF via `electron` or `wkhtmltopdf`
   - Option B: `printpdf` with enhanced CSS/positioning
   - Option C: Hybrid — render Markdown to HTML, convert to PDF
4. Fix identified issues
5. Add automated visual comparison (image diff) for PDF regression tests

**Success Criteria:**
- Generated PDFs match rendered Markdown visually
- Tables render with proper borders and cell alignment
- Code blocks render with syntax highlighting
- Images render at correct size and position
- PDF export completes within NFR-006 threshold (< 5s for 10 pages)

**Files to Modify/Create:**
- `src-tauri/src/commands/export.rs` — PDF pipeline
- `src-tauri/tests/pdf_export_tests.rs` — Extend with visual verification
- `src-tauri/tests/samples/complex_markdown.md` (test fixture)
- `src-tauri/tests/samples/expected_output/` (expected PDF reference)

---

## 3. P1 — High Priority (Should Complete for MVP)

### 3.1 G-004: Wrap Transform End-to-End Integration

**Status:** `Transform::Wrap` implemented in Rust, TipTap integration unverified
**FR Affected:** FR-038

**Implementation Approach:**
1. Verify `Transform::Wrap` implementation in `editor/transforms.rs`
2. Trace integration path: TipTap selection → IPC → Rust `editor_apply_transform` → Result
3. Test with TipTap — select text, apply wrap markers (e.g., `**`, `_`, `` ` ``)
4. Verify wrap works for: bold, italic, strikethrough, inline code, links, custom markers
5. Add integration tests

**Files to Modify:**
- `src-tauri/src/commands/editor.rs`
- `src-tauri/tests/editor_transforms.rs`
- `www/src/components/TipTapEditor.jsx`

---

### 3.2 G-005: HTML Export — Linked Assets Mode

**Status:** HTML export works, linked-assets mode not implemented
**FR Affected:** FR-031

**Implementation Approach:**
1. Add `ExportOptions` field: `asset_mode: "inline" | "linked"`
2. Implement linked-assets export:
   - Extract images from Markdown
   - Copy to `_assets/` directory
   - Rewrite image URLs to relative `_assets/` paths
3. Add UI toggle in `ExportModal.jsx`
4. Add tests for both modes

**Files to Modify:**
- `src-tauri/src/model/export.rs` — Add `asset_mode` to `ExportOptions`
- `src-tauri/src/commands/export.rs` — Implement linked-assets export
- `src-tauri/tests/html_export_tests.rs` — Add linked-assets tests
- `www/src/components/ExportModal.jsx` — Add toggle

---

### 3.3 G-006: Image Relative Path Handling — Edge Cases

**Status:** Basic relative path handling works, subdirectory edge cases unverified
**FR Affected:** FR-017

**Implementation Approach:**
1. Audit `image_markdown_from_path()` and `insert_image()` in `commands/image.rs`
2. Add tests for subdirectory document scenarios:
   - Document in `/workspace/project/notes/chapter.md`
   - Image at `/workspace/project/assets/diagram.png`
   - Expected: `../assets/diagram.png`
3. Test for nested subdirectory levels
4. Fix any broken path calculations

**Files to Modify:**
- `src-tauri/src/commands/image.rs`
- `src-tauri/tests/image_path_tests.rs` — Extend with subdirectory tests

---

### 3.4 G-007: Tree-Sitter GFM Parsing Verification

**Status:** tree-sitter-markdown in use, GFM extensions may need special handling
**FR Affected:** FR-021

**Implementation Approach:**
1. Create test Markdown covering all GFM features
2. Verify tree-sitter parses: GFM tables, task lists, strikethrough, autolinks, extended image syntax
3. Check if `comrak` parser is the right tool for GFM (it has native GFM support)
4. If conflicts exist, define clear parsing strategy: comrak for GFM rendering, tree-sitter for structural analysis

**Files to Modify:**
- `src-tauri/src/parser/tree_sitter.rs`
- `src-tauri/tests/tree_sitter_parser_tests.rs` — Add GFM tests
- `src-tauri/src/parser/markdown.rs` — Verify comrak GFM config

---

### 3.5 G-008: Settings Schema Final Verification

**Status:** Flat structure implemented, needs verification against PRD-09
**FR Affected:** FR-034

**Implementation Approach:**
1. Read PRD-09 specification for Settings schema
2. Compare with `model/settings.rs` implementation
3. Verify all fields match: `theme`, `autoSave`, `autoSaveInterval`, `focusMode`, `typewriterMode`, `outlineVisible`, `fontSize`, `fontFamily`, `lineHeight`, `contentWidth`, `recentFiles`
4. Check nested vs flat structure — PRD specifies flat
5. Verify default values match PRD
6. Update `tests/g007_settings_schema_tests.rs` with comprehensive schema validation

**Files to Modify:**
- `src-tauri/src/model/settings.rs`
- `src-tauri/src/services/settings.rs`
- `src-tauri/tests/g007_settings_schema_tests.rs`

---

### 3.6 G-009: Editor.jsx Deprecation

**Status:** Both `Editor.jsx` and `TipTapEditor.jsx` exist, relationship unclear
**FR Affected:** FR-008

**Implementation Approach:**
1. Add deprecation notice comment to `Editor.jsx`
2. Update all imports that reference `Editor.jsx` to use `TipTapEditor.jsx`
3. Remove `Editor.jsx` from `App.jsx` import chain
4. Verify `TipTapEditor.jsx` is the single active editor component
5. Update `SPEC.md` / architecture docs to reflect single-editor strategy

**Files to Modify:**
- `www/src/components/Editor.jsx` — Add deprecation notice
- `www/src/components/TipTapEditor.jsx` — Ensure full feature parity
- `www/src/App.jsx` — Remove Editor.jsx references
- `www/src/components/index.js` or barrel exports — Update

---

## 4. P2 — Medium Priority (Quality & Polish)

### 4.1 G-010: Table Editing — Document Constraints

**Status:** TipTap default behavior, constrained but safe
**FR Affected:** FR-016

**Implementation Approach:**
1. Document current table editing constraints in `docs/table-editing.md`
2. Add tests for data integrity during table operations
3. Verify no Markdown corruption during: add row, delete row, add column, delete column
4. No implementation changes unless bugs found

**Files to Modify:**
- `docs/table-editing.md` (NEW)
- `src-tauri/tests/table_editing_tests.rs` — Extend

---

### 4.2 G-011: Focus Mode — Visual Verification

**Status:** CSS implementation exists, actual dimming behavior unverified
**FR Affected:** FR-028

**Implementation Approach:**
1. Inspect `editor.css` focus mode styles
2. Verify CSS selector targets non-current paragraphs correctly
3. Test with nested structures (list items, blockquotes inside paragraphs)
4. Fix CSS if dimming doesn't apply to nested content

**Files to Modify:**
- `www/src/styles/editor.css`
- Visual/manual verification required

---

### 4.3 G-012: Typewriter Mode — Scroll Behavior

**Status:** Implementation exists, scroll-to-center behavior unverified
**FR Affected:** FR-029

**Implementation Approach:**
1. Test typewriter mode: cursor should stay at vertical center during typing
2. Verify scroll behavior during: new lines, navigation (arrow keys), paste
3. Fix scroll behavior if cursor doesn't stay centered

**Files to Modify:**
- `www/src/components/TipTapEditor.jsx` — Typewriter mode scroll logic
- Visual/manual verification required

---

### 4.4 G-013: Paste Handling — Rich Text

**Status:** Basic paste works, rich text conversion incomplete
**FR Affected:** FR-018

**Implementation Approach:**
1. Audit current paste handling in `semantic/paste.rs` and frontend
2. Test common rich text formats: Word copy, web page copy, Excel table copy
3. Improve Markdown conversion for common cases
4. Add tests for paste edge cases

**Files to Modify:**
- `src-tauri/src/semantic/paste.rs`
- `src-tauri/tests/paste_handling_tests.rs`
- `www/src/components/TipTapEditor.jsx` — Paste handler

---

### 4.5 G-014: PreferencesModal — SettingsContext Integration

**Status:** `PreferencesModal.jsx` exists, integration with `SettingsContext` needs verification
**FR Affected:** FR-034

**Implementation Approach:**
1. Audit `PreferencesModal.jsx` — verify it reads/writes through `SettingsContext`
2. Test that all settings changes persist and apply immediately
3. Verify modal opens/closes correctly
4. Check keyboard accessibility (Escape to close, Tab navigation)

**Files to Modify:**
- `www/src/components/PreferencesModal.jsx`
- Visual/manual verification required

---

### 4.6 G-015: Service Interface Verification Against PRD-10

**Status:** All traits defined, PRD compliance needs review
**FR Affected:** NFR-005

**Implementation Approach:**
1. Read PRD-10 service interface specifications
2. Compare each trait method signature against PRD
3. Verify all methods are implemented consistently
4. Update `tests/service_interface_tests.rs` to cover all PRD-specified methods

**Files to Modify:**
- `src-tauri/src/services/mod.rs`
- `src-tauri/tests/service_interface_tests.rs`

---

### 4.7 G-016: Autosave — Rust-Side Debounce Backup

**Status:** Frontend timer + Rust service, crash-before-timer-fire scenario
**FR Affected:** FR-005

**Implementation Approach:**
1. Implement Rust-side autosave with configurable debounce (e.g., 5s after last change)
2. Ensure Rust autosave runs independently of frontend timer
3. Test crash scenario: edit document, wait less than frontend interval, force crash, verify Rust autosave recovered
4. Document autosave strategy

**Files to Modify:**
- `src-tauri/src/services/autosave.rs`
- `src-tauri/src/commands/autosave.rs`
- `src-tauri/tests/autosave_tests.rs`

---

## 5. Technical Debt Resolution

### 5.1 TD-002: Command Enum Usage Consistency

**Status:** `editor/commands.rs` — `Command` enum exists, inconsistent usage
**Action:** Audit all editor commands, ensure `Command` enum is the primary dispatch mechanism
**Files:** `src-tauri/src/editor/commands.rs`, `src-tauri/src/editor/mod.rs`

### 5.2 TD-003: Editor Deprecation Documentation

**Status:** Covered by G-009 above
**Action:** Mark `Editor.jsx` deprecated

### 5.3 TD-004: Visual Regression Baselines

**Status:** Visual regression tests exist, baseline not current
**Action:** Update baseline images to current implementation state
**Files:** `tests/visual/` (if exists)

### 5.4 TD-005: File Watcher + Editor Integration Tests

**Status:** No integration tests
**Action:** Add integration test: external file change → editor receives notification → user prompt
**Files:** `src-tauri/tests/integration_file_watcher_tests.rs` (NEW)

### 5.5 TD-006: Large Document Benchmarking

**Status:** Incremental parsing works, may need tuning
**Action:** Covered by G-003 (performance benchmarks)

### 5.6 TD-008: FrontmatterBlock Visual Polish

**Status:** Component exists, visual presentation needs polish
**Action:** Review `FrontmatterBlock.jsx` styling, ensure it matches overall editor aesthetic

---

## 6. Parallelization Strategy

### Phase A — Independent P0/P1 Tasks (Parallel)

| Task | Dependencies | Category |
|------|-------------|----------|
| G-003: Performance Benchmarks | None | Backend testing |
| G-006: Image Path Tests | None | Backend testing |
| G-007: GFM Parser Verification | None | Backend testing |
| G-008: Settings Schema Verification | None | Backend testing |
| G-009: Editor.jsx Deprecation | None | Frontend cleanup |
| G-013: Paste Handling | None | Frontend/backend |

### Phase B — Dependent P0/P1 Tasks (Sequential per path)

| Task | Depends On | Reason |
|------|-----------|--------|
| G-001: Cursor Mapping Fix | G-003 (testing infra) | Benchmarks need to validate fix |
| G-002: PDF Export Fix | G-003 (testing infra) | Need automated PDF quality checks |
| G-004: Wrap Transform Integration | None | Can be verified standalone |
| G-005: Linked Assets Export | None | Independent export feature |

### Phase C — P2 Polish (Parallel, lower priority)

All P2 tasks (G-010 through G-016, TD items) are independent and can run in parallel across team members.

---

## 7. Execution Order

```
Week 1 (Days 1-3): G-003 Performance Benchmarks (creates testing infra for G-001, G-002)
Week 1 (Days 2-4): G-001 Cursor Mapping Fix + Verification
Week 1 (Days 2-4): G-002 PDF Export Fix + Verification
Week 1 (Days 3-5): G-004 Wrap Transform Integration
Week 1 (Days 3-5): G-006 Image Path Edge Cases
Week 2 (Days 1-2): G-005 HTML Linked Assets
Week 2 (Days 2-3): G-007 GFM Parser Verification
Week 2 (Days 3-4): G-008 Settings Schema Verification
Week 2 (Days 4-5): G-009 Editor Deprecation
Week 3 (Days 1-3): P2 Tasks (parallel)
Week 3 (Days 4-5): TD Resolution
Week 4: Final verification, regression testing, MVP release prep
```

---

## 8. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| P0 issues resolved | 3/3 (100%) | All P0 tasks pass |
| P1 issues resolved | 6/6 (100%) | All P1 tasks pass |
| NFR thresholds | 8/8 pass | `cargo bench` output |
| Test coverage | Maintain 95%+ | `cargo test` coverage |
| MVP readiness | 98%+ | Gap analysis checklist |

---

*Plan updated based on Iteration-7 gap analysis*
*Next: Execute tasks per Phase A/B/C parallelization strategy*
