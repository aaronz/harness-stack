# RustNote Task List — Iteration 7

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 7.0
**Created:** 2026-04-14
**Status:** MVP Completion Phase
**Priority Order:** P0 → P1 → P2
**Total Tasks:** 27

---

## Legend

| Symbol | Meaning |
|--------|---------|
| `[P0]` | Blocking — must complete before MVP |
| `[P1]` | High priority — should complete before MVP |
| `[P2]` | Medium priority — polish/quality |
| `[TD]` | Technical debt resolution |
| `[DONE]` | Completed in previous iterations |

---

## P0 — Blocking Issues (3 tasks)

### [P0-001] Implement Automated Performance Benchmark Infrastructure

**Gap Reference:** G-003
**NFRs Covered:** NFR-001, NFR-002, NFR-003, NFR-004, NFR-005, NFR-006, NFR-007, NFR-008
**PRD Reference:** PRD-14 Section 8.3

**Subtasks:**

- [ ] **T-001:** Create `src-tauri/benches/cold_start.rs` with:
  - [ ] Benchmark: empty document cold start (< 2s target, NFR-001)
  - [ ] Benchmark: 1MB document cold start (< 3s target, NFR-002)
  - [ ] Benchmark: hot file open (< 500ms target, NFR-003)
  - Run: `cargo bench --bench cold_start`

- [ ] **T-002:** Create `src-tauri/benches/nfr_thresholds.rs` with:
  - [ ] Benchmark: keystroke-to-render Rust side (< 100ms target, NFR-004)
  - [ ] Benchmark: save operation (< 200ms target, NFR-005)
  - [ ] Benchmark: PDF export 10 pages (< 5s target, NFR-006)
  - [ ] Benchmark: memory idle with 10 docs (< 300MB target, NFR-007)
  - [ ] Benchmark: large document scroll (< 100ms per frame, NFR-008)

- [ ] **T-003:** Extend `src-tauri/benches/transforms.rs`:
  - [ ] Add benchmark for each `TransformType` operation
  - [ ] Add benchmark for 1000-char document
  - [ ] Add benchmark for 100,000-char document
  - [ ] Run: `cargo bench --bench transforms`

- [ ] **T-004:** Extend `src-tauri/benches/parsing.rs`:
  - [ ] Add benchmark for incremental parse (small change)
  - [ ] Add benchmark for full parse (large document)
  - [ ] Run: `cargo bench --bench parsing`

- [ ] **T-005:** Create `www/scripts/bench.js` frontend benchmarks:
  - [ ] Keystroke latency measurement (NFR-004 frontend side)
  - [ ] Scroll FPS measurement (NFR-008)
  - [ ] Large document load time
  - [ ] Run: `npm run bench`

- [ ] **T-006:** Create `src-tauri/.github/workflows/bench.yml`:
  - [ ] Trigger on every push to `main`
  - [ ] Run `cargo bench` and fail if NFR thresholds exceeded
  - [ ] Upload benchmark results as artifacts
  - [ ] Compare against baseline (delta report)

- [ ] **T-007:** Create `docs/perf-benchmarks.md`:
  - [ ] Document all NFR thresholds with rationale
  - [ ] Document how to run benchmarks locally
  - [ ] Document CI benchmark integration
  - [ ] Document how to update baselines

**Verification:** `cargo bench` passes all NFR thresholds, CI workflow triggers on push, `docs/perf-benchmarks.md` exists and is accurate.

**Files:** `src-tauri/benches/cold_start.rs`, `src-tauri/benches/nfr_thresholds.rs`, `src-tauri/benches/transforms.rs`, `src-tauri/benches/parsing.rs`, `www/scripts/bench.js`, `src-tauri/.github/workflows/bench.yml`, `docs/perf-benchmarks.md`

---

### [P0-002] Verify and Fix Bidirectional Cursor Mapping

**Gap Reference:** G-001
**FR Reference:** FR-022
**Files Involved:** `semantic/position.rs`, `cursor_mapping_tests.rs`, `TipTapEditor.jsx`

**Subtasks:**

- [ ] **T-008:** Audit `src-tauri/src/semantic/position.rs`:
  - [ ] Read full `CursorMapping` implementation
  - [ ] Identify all edge cases not covered
  - [ ] Document findings in code comments
  - [ ] List: nested structures, empty nodes, multi-byte chars, code spans, adjacent code fences

- [ ] **T-009:** Add edge case unit tests to `src-tauri/tests/cursor_mapping_tests.rs`:
  - [ ] Test: cursor in nested list inside blockquote
  - [ ] Test: cursor in empty heading
  - [ ] Test: cursor in empty list item
  - [ ] Test: cursor at start/end of inline code span
  - [ ] Test: cursor in multi-byte UTF-8 text (CJK characters)
  - [ ] Test: cursor adjacent to code fence (backtick boundary)
  - [ ] Test: round-trip DOM→source→DOM converges (X' == X)

- [ ] **T-010:** Fix `dom_to_source()` edge cases:
  - [ ] Fix each identified edge case from T-008
  - [ ] Ensure `build_cursor_mapping()` is called correctly after source edits
  - [ ] Add test for partial re-mapping (incremental source changes)

- [ ] **T-011:** Verify TipTap integration:
  - [ ] Open `TipTapEditor.jsx` cursor mapping integration
  - [ ] Test with complex Markdown: headings + lists + code + tables
  - [ ] Manually verify cursor position maps correctly after 10+ edits
  - [ ] Document integration points and any fixes needed

- [ ] **T-012:** Add `dom_to_source` regression test:
  - [ ] Create 10 complex Markdown documents
  - [ ] For each: parse, convert to DOM, convert each DOM position back to source
  - [ ] Assert: for all positions 0..source_length, round-trip error < 1 character

**Verification:** `cargo test cursor_mapping` passes 100%, manual TipTap verification passes, round-trip tests pass for all 10 complex documents.

---

### [P0-003] Verify and Improve PDF Export Quality

**Gap Reference:** G-002
**FR Reference:** FR-032
**NFR Reference:** NFR-006

**Subtasks:**

- [ ] **T-013:** Create test Markdown fixtures:
  - [ ] `src-tauri/tests/samples/pdf_tables.md` — GFM table with merged cells, borders
  - [ ] `src-tauri/tests/samples/pdf_code.md` — Multi-language code blocks with syntax
  - [ ] `src-tauri/tests/samples/pdf_images.md` — Images at various sizes and positions
  - [ ] `src-tauri/tests/samples/pdf_complex.md` — All features combined
  - [ ] `src-tauri/tests/samples/pdf_nested.md` — Nested blockquotes, lists, code

- [ ] **T-014:** Generate and inspect PDF outputs:
  - [ ] Generate PDFs for each fixture using current `printpdf` pipeline
  - [ ] Inspect visually: tables, code blocks, images, nested structures
  - [ ] Document issues: missing borders, wrong font, truncated content, missing images
  - [ ] Record findings with screenshots

- [ ] **T-015:** Fix PDF rendering issues (if any found in T-014):
  - [ ] If printpdf issues: evaluate HTML-to-PDF pipeline (electron/wkhtmltopdf)
  - [ ] If printpdf sufficient: fix CSS positioning, font embedding, image scaling
  - [ ] Implement fixes for each identified issue
  - [ ] Re-generate and re-inspect

- [ ] **T-016:** Add automated PDF quality checks:
  - [ ] Extend `src-tauri/tests/pdf_export_tests.rs`
  - [ ] Add test: PDF file size reasonable (not 0 bytes, not missing pages)
  - [ ] Add test: PDF contains expected text (extracted via pdf-extract crate or similar)
  - [ ] Add test: PDF page count matches expected
  - [ ] Add test: PDF export time < 5s for 10-page document (NFR-006)

- [ ] **T-017:** Add visual regression baseline for PDF:
  - [ ] Generate reference PDFs for each fixture
  - [ ] Store in `src-tauri/tests/samples/expected_output/`
  - [ ] Add CI check that compares new PDFs against baselines (image diff)

**Verification:** All complex Markdown renders correctly in PDF, NFR-006 passes (< 5s for 10 pages), automated PDF tests pass.

---

## P1 — High Priority (6 tasks)

### [P1-004] Verify and Fix Wrap Transform End-to-End Integration

**Gap Reference:** G-004
**FR Reference:** FR-038

**Subtasks:**

- [ ] **T-018:** Audit `Transform::Wrap` in `src-tauri/src/editor/transforms.rs`:
  - [ ] Verify `apply_wrap()` handles `before`/`after` markers correctly
  - [ ] Test: wrap with `**` (bold), `_` (italic), `~~` (strikethrough), `` ` `` (inline code)
  - [ ] Test: wrap with custom markers (e.g., `[[`, `]]`)
  - [ ] Test: wrap with empty selection (no-op or insert markers)

- [ ] **T-019:** Trace TipTap integration path:
  - [ ] Read `TipTapEditor.jsx` for wrap-related keyboard shortcuts/menu items
  - [ ] Trace: selection → IPC command → `editor_apply_transform` → result → TipTap state
  - [ ] Identify missing integration points
  - [ ] Fix: ensure wrap transform is wired from UI to Rust

- [ ] **T-020:** Add integration tests:
  - [ ] `src-tauri/tests/editor_transforms.rs` — add wrap transform tests
  - [ ] Test with selection spanning multiple paragraphs
  - [ ] Test wrap with existing formatting (double-bold)
  - [ ] Test unwrap (wrap with empty before/after)

**Verification:** `cargo test wrap` passes, manual TipTap test: select text → apply bold wrap → text wrapped with `**`.

---

### [P1-005] Implement HTML Export — Linked Assets Mode

**Gap Reference:** G-005
**FR Reference:** FR-031

**Subtasks:**

- [ ] **T-021:** Add `asset_mode` to `ExportOptions`:
  - [ ] Modify `src-tauri/src/model/export.rs` — add `AssetMode` enum (`Inline`, `Linked`)
  - [ ] Add `asset_mode: AssetMode` field to `ExportOptions`
  - [ ] Update `commands/export.rs` to handle both modes

- [ ] **T-022:** Implement linked-assets export pipeline:
  - [ ] Parse HTML output to extract `<img>` tags
  - [ ] Extract image filenames/URLs from Markdown source
  - [ ] Copy referenced images to `_assets/` subdirectory
  - [ ] Rewrite image URLs to `assets/` relative paths
  - [ ] Handle missing images gracefully (skip or use placeholder)

- [ ] **T-023:** Add UI toggle in ExportModal:
  - [ ] Read `www/src/components/ExportModal.jsx`
  - [ ] Add radio button or dropdown: "Inline assets" vs "Linked assets"
  - [ ] Wire toggle to IPC `export_to_html` call with `asset_mode` option

- [ ] **T-024:** Add tests for linked-assets mode:
  - [ ] `src-tauri/tests/html_export_tests.rs` — add linked mode tests
  - [ ] Test: HTML references `_assets/` directory
  - [ ] Test: `_assets/` directory contains copied images
  - [ ] Test: self-contained mode produces valid HTML with inline base64 images

**Verification:** `cargo test html_export` passes, ExportModal has working toggle, linked mode produces `assets/` directory with images, inline mode produces base64-encoded images.

---

### [P1-006] Image Relative Path Handling — Edge Cases for Subdirectories

**Gap Reference:** G-006
**FR Reference:** FR-017

**Subtasks:**

- [ ] **T-025:** Audit `commands/image.rs`:
  - [ ] Read `image_markdown_from_path()` and `insert_image()`
  - [ ] Identify path calculation logic for relative paths
  - [ ] Document current behavior

- [ ] **T-026:** Add subdirectory test cases:
  - [ ] Document: `/workspace/project/notes/chapter.md`
  - [ ] Image: `/workspace/project/assets/diagram.png`
  - [ ] Expected: `../assets/diagram.png`
  - [ ] Test: 2-level nesting (`notes/sub/chapter.md` → `../../assets/img.png`)
  - [ ] Test: 3-level nesting
  - [ ] Test: image in same directory
  - [ ] Test: image in child directory of document

- [ ] **T-027:** Fix path calculation if broken:
  - [ ] Implement correct relative path calculation using `std::path::Path` methods
  - [ ] Ensure `..` navigation is correct for subdirectory depth
  - [ ] Handle Windows path separators

- [ ] **T-028:** Add tests to `image_path_tests.rs`:
  - [ ] Add test for each subdirectory scenario from T-026
  - [ ] Add test for absolute path handling (should fail or convert to relative)

**Verification:** `cargo test image_path` passes all subdirectory tests, manual test: insert image in subdirectory document → correct relative path appears in Markdown.

---

### [P1-007] Tree-Sitter GFM Parsing Verification

**Gap Reference:** G-007
**FR Reference:** FR-021

**Subtasks:**

- [ ] **T-029:** Create GFM test Markdown:
  - [ ] `src-tauri/tests/samples/gfm_tables.md` — GFM table syntax
  - [ ] `src-tauri/tests/samples/gfm_tasks.md` — Task list items `[ ]` / `[x]`
  - [ ] `src-tauri/tests/samples/gfm_autolinks.md` — Autolinks
  - [ ] `src-tauri/tests/samples/gfm_strikethrough.md` — `~~strikethrough~~`
  - [ ] `src-tauri/tests/samples/gfm_all.md` — All GFM features combined

- [ ] **T-030:** Verify tree-sitter GFM parsing:
  - [ ] Parse each GFM fixture with `tree_sitter.rs`
  - [ ] Verify: tree-sitter correctly parses GFM table syntax
  - [ ] Verify: task list checkboxes are recognized
  - [ ] Verify: strikethrough nodes are identified
  - [ ] If conflicts: check `markdown.rs` (comrak) — comrak has native GFM support

- [ ] **T-031:** Define clear parsing strategy:
  - [ ] Document: tree-sitter for structural analysis (headings, lists, code blocks)
  - [ ] Document: comrak for GFM rendering (tables, task lists, autolinks)
  - [ ] Ensure no conflicts between parsers
  - [ ] Update `docs/parsing-strategy.md` (NEW) with findings

- [ ] **T-032:** Add GFM parser tests:
  - [ ] Extend `src-tauri/tests/tree_sitter_parser_tests.rs`
  - [ ] Add test for each GFM feature
  - [ ] Verify output matches expected AST structure

**Verification:** `cargo test tree_sitter` passes GFM tests, GFM features render correctly in editor, parsing strategy documented.

---

### [P1-008] Settings Schema Final Verification Against PRD-09

**Gap Reference:** G-008
**FR Reference:** FR-034

**Subtasks:**

- [ ] **T-033:** Read PRD-09 Settings specification:
  - [ ] Identify exact field names, types, and default values
  - [ ] Note: nested vs flat structure requirement
  - [ ] Note: any missing fields from implementation

- [ ] **T-034:** Audit `model/settings.rs` and `services/settings.rs`:
  - [ ] Compare each field against PRD-09
  - [ ] Verify: all 11 fields present (`theme`, `autoSave`, `autoSaveInterval`, `focusMode`, `typewriterMode`, `outlineVisible`, `fontSize`, `fontFamily`, `lineHeight`, `contentWidth`, `recentFiles`)
  - [ ] Verify: types match (e.g., `autoSaveInterval` is `number`, not `string`)
  - [ ] Verify: flat structure (not nested under `editor` key)
  - [ ] Verify: default values match PRD-09

- [ ] **T-035:** Fix any discrepancies:
  - [ ] Add missing fields
  - [ ] Fix type mismatches
  - [ ] Flatten nested structures
  - [ ] Update default values

- [ ] **T-036:** Update schema tests:
  - [ ] Extend `src-tauri/tests/g007_settings_schema_tests.rs`
  - [ ] Add test: default settings match PRD-09 defaults
  - [ ] Add test: settings round-trip through JSON serialization
  - [ ] Add test: invalid field values are rejected

**Verification:** `cargo test settings_schema` passes, implementation matches PRD-09 exactly, all 11 fields present with correct types and defaults.

---

### [P1-009] Deprecate Editor.jsx — Mark Legacy Component

**Gap Reference:** G-009
**FR Reference:** FR-008
**Status:** ✅ DONE

**Subtasks:**

- [x] **T-037:** Audit Editor.jsx usage:
  - [x] Search codebase for all imports of `Editor.jsx`
  - [x] List all files that import it
  - [x] Verify feature parity: `TipTapEditor.jsx` must support all `Editor.jsx` features

- [x] **T-038:** Add deprecation notice:
  - [x] Edit `www/src/components/Editor.jsx`
  - [x] Add JSDoc deprecation notice at top: `/** @deprecated Use TipTapEditor.jsx instead */`
  - [x] Add comment explaining: will be removed in post-MVP release

- [x] **T-039:** Update all imports:
  - [x] Update `www/src/App.jsx` to use `TipTapEditor.jsx`
  - [x] Update any barrel export / index files
  - [x] Remove `Editor.jsx` from default export if present

- [x] **T-040:** Update documentation:
  - [x] Update `SPEC.md` architecture section to reflect single-editor strategy
  - [x] Update `docs/architecture.md` to remove Editor.jsx references
  - [x] Document TipTapEditor as the sole active editor

**Verification:** `Editor.jsx` has deprecation notice, no code imports Editor.jsx (only TipTapEditor), `SPEC.md` and `docs/architecture.md` updated.

---

## P2 — Medium Priority (8 tasks)

### [P2-010] Document Table Editing Constraints

**Gap Reference:** G-010
**FR Reference:** FR-016

**Subtasks:**

- [x] **T-041:** Document constraints in `docs/table-editing.md`:
  - [x] Current behavior: TipTap default table editing
  - [x] Constraints: no column resize handles, basic cell navigation
  - [x] Markdown preservation guarantee: table Markdown never corrupted
  - [x] Future enhancements (post-MVP)

- [x] **T-042:** Add table data integrity tests:
  - [x] Extend `src-tauri/tests/table_editing_tests.rs`
  - [x] Test: add row → Markdown table syntax preserved
  - [x] Test: delete row → Markdown table syntax preserved
  - [x] Test: add column → Markdown table syntax preserved
  - [x] Test: delete column → Markdown table syntax preserved
  - [x] Test: cell with pipe character escapes correctly

**Verification:** `cargo test table_editing` passes, `docs/table-editing.md` exists with complete constraint documentation.

---

### [P2-011] Focus Mode Visual Verification and Fix

**Gap Reference:** G-011
**FR Reference:** FR-028

**Subtasks:**

- [ ] **T-043:** Inspect and test focus mode CSS:
  - [ ] Read `www/src/styles/editor.css` — find focus mode styles
  - [ ] Test: non-current paragraphs should be dimmed (opacity reduced)
  - [ ] Test: current paragraph (cursor position) is fully visible
  - [ ] Test: nested content (lists, blockquotes) dims correctly
  - [ ] Test: dimming works in both light and dark themes

- [ ] **T-044:** Fix CSS if dimming broken:
  - [ ] Fix: nested structural elements should inherit dimming
  - [ ] Fix: headings and code blocks should also dim
  - [ ] Ensure: no flash/jump when cursor moves between paragraphs

- [ ] **T-045:** Manual visual verification:
  - [ ] Enable focus mode
  - [ ] Place cursor in middle of 5-paragraph document
  - [ ] Verify: 2 paragraphs above dim, 2 paragraphs below dim, current paragraph full opacity
  - [ ] Screenshot evidence captured for regression baseline

**Verification:** Focus mode dims non-current paragraphs correctly in both themes, no visual artifacts.

---

### [P2-012] Typewriter Mode — Scroll Behavior Fix

**Gap Reference:** G-012
**FR Reference:** FR-029

**Subtasks:**

- [ ] **T-046:** Audit typewriter mode scroll implementation:
  - [ ] Read `TipTapEditor.jsx` typewriter mode logic
  - [ ] Read CSS for typewriter mode styles
  - [ ] Identify scroll-to-center logic

- [ ] **T-047:** Test scroll behavior:
  - [ ] Enable typewriter mode
  - [ ] Type at end of paragraph → cursor stays at vertical center
  - [ ] Press Enter (new line) → cursor stays at vertical center
  - [ ] Navigate with arrow keys → cursor stays at vertical center
  - [ ] Paste text → cursor stays at vertical center

- [ ] **T-048:** Fix scroll behavior:
  - [ ] If cursor doesn't stay centered: fix scroll calculation
  - [ ] Ensure: scroll adjustment happens after cursor position updates
  - [ ] Test: large document (100+ paragraphs) — performance acceptable

**Verification:** Typewriter mode keeps cursor at vertical center during typing, navigation, and paste. 100-paragraph document scrolls at 60 FPS.

---

### [P2-013] Paste Handling — Improve Rich Text Conversion

**Gap Reference:** G-013
**FR Reference:** FR-018

**Subtasks:**

- [ ] **T-049:** Audit paste handling:
  - [ ] Read `semantic/paste.rs`
  - [ ] Read `TipTapEditor.jsx` paste handler
  - [ ] Test: paste from Word, web page, Excel
  - [ ] Document: what works, what doesn't

- [ ] **T-050:** Improve Markdown conversion:
  - [ ] Improve: bold/italic from rich text → `**` / `_`
  - [ ] Improve: hyperlinks from rich text → `[text](url)`
  - [ ] Improve: lists from rich text → `- item` or `1. item`
  - [ ] Handle: plain text fallback when format not recognized

- [ ] **T-051:** Add paste handling tests:
  - [ ] Extend `src-tauri/tests/paste_handling_tests.rs`
  - [ ] Test: plain text paste
  - [ ] Test: Markdown paste
  - [ ] Test: HTML paste (should convert to Markdown)
  - [ ] Test: rich text paste (Word, web — best effort)

**Verification:** `cargo test paste_handling` passes (67 tests), common rich text paste cases convert to valid Markdown.

**Status: ✅ DONE**

---

### [P2-014] PreferencesModal — SettingsContext Integration Verification

**Gap Reference:** G-014
**FR Reference:** FR-034

**Subtasks:**

- [x] **T-052:** Audit PreferencesModal:
  - [x] Read `www/src/components/PreferencesModal.jsx`
  - [x] Read `www/src/contexts/SettingsContext.jsx`
  - [x] Verify: modal reads settings from SettingsContext
  - [x] Verify: modal writes settings changes to SettingsContext
  - [x] Verify: changes persist across modal open/close cycle

- [x] **T-053:** Test all settings controls:
  - [x] Theme toggle → instant switch, persists
  - [x] Font size → instant switch, persists
  - [x] Font family → instant switch, persists
  - [x] Line height → instant switch, persists
  - [x] Content width → instant switch, persists
  - [x] Focus mode → instant switch, persists
  - [x] Typewriter mode → instant switch, persists
  - [x] Outline visible → instant switch, persists
  - [x] Auto-save toggle → instant switch, persists
  - [x] Auto-save interval → instant switch, persists

- [x] **T-054:** Fix integration issues:
  - [x] Fix any missing context bindings
  - [x] Fix: close button updates context before closing
  - [x] Fix: keyboard accessibility (Escape to close, Tab navigation)

**Verification:** All 10 settings controls work correctly, changes persist, keyboard accessibility passes.

---

### [P2-015] Service Interface Verification Against PRD-10

**Gap Reference:** G-015
**FR Reference:** NFR-005

**Subtasks:**

- [ ] **T-055:** Audit service traits:
  - [ ] Read `services/mod.rs` — all trait definitions
  - [ ] Read PRD-10 service interface specifications
  - [ ] Compare: each method name, signature, return type against PRD
  - [ ] List: any missing methods, any extra methods, any signature mismatches

- [ ] **T-056:** Fix discrepancies:
  - [ ] Add missing methods
  - [ ] Fix signature mismatches
  - [ ] Remove extraneous methods
  - [ ] Ensure all 8 service traits are complete: DocumentServiceTrait, EditorServiceTrait, SettingsServiceTrait, FileWatcherServiceTrait, AutosaveServiceTrait, ExportServiceTrait, RecoveryServiceTrait, WorkspaceServiceTrait

- [ ] **T-057:** Update service interface tests:
  - [ ] Extend `src-tauri/tests/service_interface_tests.rs`
  - [ ] Add test for each method in each trait
  - [ ] Mock implementations tested

**Verification:** `cargo test service_interface` passes, all traits match PRD-10 exactly.

---

### [P2-016] Autosave — Rust-Side Debounce Backup

**Gap Reference:** G-016
**FR Reference:** FR-005

**Subtasks:**

- [ ] **T-058:** Audit current autosave architecture:
  - [ ] Read `services/autosave.rs`
  - [ ] Read `commands/autosave.rs`
  - [ ] Read `www/src/hooks/useAutoSaveTimer.js`
  - [ ] Document: frontend timer interval, Rust autosave trigger

- [ ] **T-059:** Implement Rust-side autosave with debounce:
  - [ ] Add Rust timer (tokio or std::thread) independent of frontend
  - [ ] Configurable debounce: default 5 seconds after last change
  - [ ] Backup autosave: runs even if frontend timer hasn't fired
  - [ ] Ensure: Rust autosave and frontend autosave don't conflict

- [ ] **T-060:** Test crash recovery scenario:
  - [ ] Open document, make edits, close app without saving
  - [ ] Wait less than frontend autosave interval (e.g., 10s interval, wait 5s)
  - [ ] Force kill app (simulate crash)
  - [ ] Reopen app → verify Rust-side autosave recovered edits
  - [ ] This test should be automated

- [ ] **T-061:** Update autosave tests:
  - [ ] Extend `src-tauri/tests/autosave_tests.rs`
  - [ ] Add test: Rust autosave fires after debounce period
  - [ ] Add test: Rust autosave + frontend autosave don't conflict

**Verification:** `cargo test autosave` passes, crash recovery test passes, Rust autosave fires independently of frontend timer.

---

## Technical Debt (5 tasks)

### [TD-002] Command Enum Usage Consistency

**Gap Reference:** TD-002

**Subtasks:**

- [ ] **T-062:** Audit `editor/commands.rs` and `editor/mod.rs`:
  - [ ] Verify `Command` enum is the primary dispatch mechanism
  - [ ] Identify any editor operations bypassing `Command` enum
  - [ ] Refactor to use `Command` enum consistently

---

### [TD-003] Editor Deprecation Documentation

**Gap Reference:** TD-003
**Status:** Covered by P1-009 (G-009) above.

---

### [TD-004] Visual Regression Baselines

**Gap Reference:** TD-004

**Subtasks:**

- [ ] **T-063:** Establish visual regression baselines:
  - [ ] Create `tests/visual/` directory
  - [ ] Capture baseline screenshots for: empty editor, focused editor, dark theme, light theme, focus mode, typewriter mode, export modals
  - [ ] Add visual regression CI check (optional: use `jest-image-snapshot` or similar)

---

### [TD-005] File Watcher + Editor Integration Tests

**Gap Reference:** TD-005

**Subtasks:**

- [ ] **T-064:** Create `src-tauri/tests/integration_file_watcher_tests.rs`:
  - [ ] Test: external file modification → editor detects change
  - [ ] Test: editor shows notification after external change
  - [ ] Test: user chooses "Reload" → editor reloads content
  - [ ] Test: user chooses "Ignore" → editor keeps current content
  - [ ] Test: file deleted externally → editor shows error

---

### [TD-008] FrontmatterBlock Visual Polish

**Gap Reference:** TD-008

**Subtasks:**

- [ ] **T-065:** Review and polish `FrontmatterBlock.jsx`:
  - [ ] Read component code and styles
  - [ ] Check visual consistency with editor theme
  - [ ] Verify: frontmatter renders in both light and dark themes
  - [ ] Fix: any visual inconsistencies

---

## Summary

| ID | Priority | Task | Status |
|----|----------|------|--------|
| P0-001 | P0 | Performance Benchmark Infrastructure | **TODO** |
| P0-002 | P0 | Cursor Mapping Verification | **TODO** |
| P0-003 | P0 | PDF Export Quality | **TODO** |
| P1-004 | P1 | Wrap Transform Integration | **TODO** |
| P1-005 | P1 | HTML Linked Assets Mode | **TODO** |
| P1-006 | P1 | Image Path Edge Cases | **TODO** |
| P1-007 | P1 | GFM Parser Verification | **TODO** |
| P1-008 | P1 | Settings Schema Verification | **TODO** |
| P1-009 | P1 | Editor.jsx Deprecation | **DONE** |
| P2-010 | P2 | Table Editing Documentation | **DONE** |
| P2-011 | P2 | Focus Mode Visual Verification | **DONE** |
| P2-012 | P2 | Typewriter Mode Scroll Fix | **TODO** |
| P2-013 | P2 | Paste Handling Improvement | **DONE** |
| P2-014 | P2 | PreferencesModal Integration | ✅ Done |
| P2-015 | P2 | Service Interface Verification | **TODO** |
| P2-016 | P2 | Rust-Side Autosave Backup | **TODO** |
| TD-002 | TD | Command Enum Consistency | **TODO** |
| TD-004 | TD | Visual Regression Baselines | **TODO** |
| TD-005 | TD | File Watcher Integration Tests | **TODO** |
| TD-008 | TD | FrontmatterBlock Polish | **TODO** |

**Total: 20 actionable tasks across P0/P1/P2/TD**
**P0 completion unlocks MVP release readiness.**

---

*Task list updated based on Iteration-7 gap analysis*
*Created: 2026-04-14*
