# RustNote Implementation Plan — Iteration 8

**Project:** RustNote - Typora-like Markdown Editor
**Phase:** Phase 1 — MVP Completion
**Version:** 8.0
**Created:** 2026-04-14
**Based on:** Iteration-8 Gap Analysis (Spec v3.1)
**MVP Readiness:** ~93% → Target 100%
**Priority Order:** P0 → P1 → P2

---

## 1. Priority Summary

| Priority | Count | Description | Unblock |
|----------|-------|-------------|---------|
| **P0** | 4 | Blocking issues before MVP release | MVP completion |
| **P1** | 8 | High-priority improvements | Feature completeness |
| **P2** | 9 | Polish and medium-priority items | Quality |
| **Tech Debt** | 4 | Code quality and documentation | Maintainability |

---

## 2. P0 — Must Fix Before MVP Release

### 2.1 G-003: Run & Document NFR Benchmark Results

**Status:** Infrastructure exists (5 benchmark files created in iter-8), results not captured
**NFRs Affected:** NFR-001 through NFR-008
**PRD Reference:** PRD-05, PRD-14 Section 8.3
**Gap ID:** G-003 (Iteration-8, P0)

**Context from Iteration-7→8:** Iteration-7 plan created benchmark infrastructure as P0-001. Iteration-8 actually built the infrastructure (5 files: `cold_start.rs`, `nfr_thresholds.rs`, `transforms.rs`, `parsing.rs`, `serialization.rs`). G-003 remains P0 because the infrastructure exists but results are not captured, documented, or compared to PRD thresholds.

**Implementation Approach:**
1. Execute `cargo bench` and capture all benchmark results
2. Document each result against PRD-05 Section 11.1 NFR thresholds
3. Identify which thresholds pass/fail
4. For failing thresholds, implement fixes
5. Publish results as CI artifacts (extend `bench.yml` if it exists, or create)
6. Create `docs/perf-benchmarks.md` documenting results and thresholds

**Success Criteria:**
- `cargo bench` results documented in machine-readable format (JSON)
- Each NFR threshold has explicit pass/fail against measured results
- Failing thresholds have associated fix tasks created
- CI publishes benchmark artifacts on every push

**Files to Modify:**
- `src-tauri/benches/*.rs` — Ensure all benches compile and run
- `src-tauri/.github/workflows/bench.yml` — Create if missing, extend if exists
- `docs/perf-benchmarks.md` — Document all NFR thresholds and measured results

---

### 2.2 G-001: Bidirectional Cursor Mapping — End-to-End Verification with TipTap

**Status:** `build_cursor_mapping()` and `dom_to_source()` exist in Rust, TipTap integration NOT verified
**FR Affected:** FR-022
**PRD Reference:** PRD-09 Section 23.1, Article 15 + Addendum 15A
**Gap ID:** G-001 (Iteration-7→8 persistent, P0)

**Context:** This issue persists from Iteration-7. The gap is that Rust-side unit tests exist, but no end-to-end integration test exercises `dom_to_source()` with real TipTap-generated DOM output. Article 15 Addendum (proposed iter-8 constitution) makes integration-level verification mandatory.

**Implementation Approach:**
1. Audit `semantic/position.rs` cursor mapping implementation — identify all edge cases
2. Add comprehensive edge case unit tests: nested structures, empty nodes, multi-byte UTF-8, code span boundaries, adjacent code fences
3. Create TipTap integration test: parse complex Markdown → render to DOM → convert each DOM position back to source → verify round-trip converges
4. Fix identified edge cases in `dom_to_source()` binary search
5. Verify `build_cursor_mapping()` handles incremental source edits

**Success Criteria:**
- All edge case unit tests pass
- TipTap round-trip test (source → DOM → source) converges with zero divergence for all Markdown constructs
- Integration test exercises real TipTap DOM offsets, not mocked structures

**Files to Modify:**
- `src-tauri/src/semantic/position.rs` — Fix edge cases
- `src-tauri/tests/cursor_mapping_tests.rs` — Add edge case tests
- `www/src/components/TipTapEditor.jsx` — Verify cursor mapping integration points

---

### 2.3 G-002: PDF Export Quality — Visual Verification & Fix

**Status:** printpdf implementation exists, manual visual verification NOT done
**FR Affected:** FR-032
**NFR Reference:** NFR-006 (< 5s for 10 pages)
**PRD Reference:** Article 16 + Addendum 16B
**Gap ID:** G-002 (Iteration-7→8 persistent, P0)

**Context:** This issue persists from Iteration-7. Article 16 Addendum 16B (proposed iter-8 constitution) requires reference document fixtures with explicit pass/fail criteria for visual output.

**Implementation Approach:**
1. Create test Markdown fixtures covering all Markdown types: tables with borders, code blocks with syntax highlighting, images, nested blockquotes, task lists, horizontal rules
2. Generate PDFs using current printpdf pipeline
3. Inspect visual output: document issues (missing borders, wrong fonts, truncated content, missing images)
4. If printpdf quality is inadequate, evaluate alternatives:
   - Option A: HTML-to-PDF via `wkhtmltopdf` or headless Chrome
   - Option B: printpdf with enhanced CSS/positioning fixes
   - Option C: Hybrid — render Markdown to styled HTML, convert to PDF
5. Fix identified issues
6. Add automated PDF quality checks (file size, page count, text extraction)

**Success Criteria:**
- All complex Markdown (tables, code blocks, images, nested structures) renders correctly in PDF
- NFR-006 passes: PDF export completes in < 5s for 10-page document
- Reference fixtures exist with explicit pass/fail criteria per Markdown type

**Files to Modify/Create:**
- `src-tauri/tests/samples/pdf_tables.md` — GFM table test
- `src-tauri/tests/samples/pdf_code.md` — Code block test
- `src-tauri/tests/samples/pdf_images.md` — Image test
- `src-tauri/tests/samples/pdf_complex.md` — Combined test
- `src-tauri/src/commands/export.rs` — PDF pipeline fix/upgrade
- `src-tauri/tests/pdf_export_tests.rs` — Extend with quality verification

---

### 2.4 G-004: Paste Rich-Text Conversion — Complete TurndownService Rules

**Status:** TurndownService used for HTML→Markdown, incomplete for Word/Excel/HTML clipboard formats
**FR Affected:** FR-018
**PRD Reference:** Article 16 + Addendum 16A (paste/clipboard conversion)
**Gap ID:** G-004 (NEW P0, Iteration-8)

**Context:** Article 16 Addendum 16A (proposed iter-8 constitution) extends "Format Conversion Fidelity" to explicitly cover paste/clipboard conversion. This was not covered by the original "Export Output Fidelity" scope.

**Implementation Approach:**
1. Audit `semantic/paste.rs` and `TurndownService` configuration
2. Test common rich text paste scenarios: Word copy, Excel table copy, web page copy, plain text copy
3. Expand TurndownService rules for:
   - Tables (Excel HTML tables → GFM table syntax)
   - Lists with proper nesting
   - Bold/italic/strikethrough formatting
   - Hyperlinks → `[text](url)`
   - Images (inline and linked)
   - Code blocks with language detection
4. Add specific handling for Word/Excel clipboard formats (HTML clipboard data)
5. Add tests covering all paste scenarios
6. Document fallback behavior when format is unrecognized (strip to plain text)

**Success Criteria:**
- `cargo test paste_handling` passes with expanded test suite
- Word table paste → valid GFM table syntax
- Web page HTML paste → valid Markdown
- Excel data paste → valid Markdown table
- Unrecognized formats fall back to plain text gracefully

**Files to Modify:**
- `src-tauri/src/semantic/paste.rs` — Expand TurndownService rules
- `src-tauri/tests/paste_handling_tests.rs` — Add comprehensive paste tests
- `www/src/components/TipTapEditor.jsx` — Paste handler integration

---

## 3. P1 — High Priority (Should Complete for MVP)

### 3.1 G-005: Wrap Transform — TipTap Selection Integration

**Status:** `Transform::Wrap` implemented in Rust, end-to-end with TipTap NOT verified
**FR Affected:** FR-038
**PRD Reference:** Article 12 (Engine Operation Completeness)
**Gap ID:** G-005 (P1)

**Implementation Approach:**
1. Audit `Transform::Wrap` in `editor/transforms.rs`
2. Trace integration path: TipTap selection → IPC `editor_apply_transform` → Result → TipTap state update
3. Test with TipTap: select text → apply wrap markers (e.g., `**`, `_`, `` ` ``)
4. Add integration tests for wrap with existing formatting, unwrap behavior

**Files:** `editor/transforms.rs`, `commands/editor.rs`, `TipTapEditor.jsx`

---

### 3.2 G-006: HTML Export — Linked-Assets Mode

**Status:** HTML export works, linked-assets mode NOT implemented
**FR Affected:** FR-031
**PRD Reference:** Article 16 (Format Conversion Fidelity)
**Gap ID:** G-006 (P1)

**Implementation Approach:**
1. Add `asset_mode: "inline" | "linked"` to `ExportOptions`
2. In linked mode: extract images, copy to `_assets/` directory, rewrite URLs to relative paths
3. Add UI toggle in `ExportModal.jsx`
4. Add tests for both modes

**Files:** `model/export.rs`, `commands/export.rs`, `ExportModal.jsx`

---

### 3.3 G-007: Image Relative Path — Subdirectory Edge Cases

**Status:** Basic relative path works, subdirectory combinations NOT tested
**FR Affected:** FR-017
**Gap ID:** G-007 (P1)

**Implementation Approach:**
1. Audit `image_markdown_from_path()` and `insert_image()` in `commands/image.rs`
2. Add tests for: doc in subdir referencing image in sibling dir, doc in subdir referencing image in parent dir, nested subdirectory depth
3. Fix any broken path calculations
4. Ensure Windows path separator handling

**Files:** `commands/image.rs`, `tests/image_path_tests.rs`

---

### 3.4 G-008: tree-sitter GFM Parsing — Verification

**Status:** tree-sitter-markdown loaded, GFM extensions NOT verified
**FR Affected:** FR-021
**PRD Reference:** Article 14 (Architecture Technology Conformance)
**Gap ID:** G-008 (P1)

**Implementation Approach:**
1. Create GFM test fixtures: GFM tables (`| col | col |`), task lists (`- [ ]`), strikethrough (`~~text~~`), autolinks
2. Verify tree-sitter correctly parses GFM constructs
3. If tree-sitter is deficient for GFM, define clear strategy: comrak for GFM rendering, tree-sitter for structural analysis
4. Add GFM parser tests

**Files:** `parser/tree_sitter.rs`, `parser/markdown.rs`, `tests/tree_sitter_parser_tests.rs`

---

### 3.5 G-009: Settings Schema — Add `recent_folders`

**Status:** Rust Settings has 11 fields, PRD-09 specifies 9. Missing `recent_folders: Vec<String>`
**FR Affected:** FR-024, FR-034
**PRD Reference:** Article 3 (Settings-Implementation Parity)
**Gap ID:** G-009 (P1)

**Implementation Approach:**
1. Add `recent_folders: Vec<String>` field to Rust Settings model
2. Update `services/settings.rs` persistence
3. Update frontend Settings interface to include `recentFolders`
4. Update `PreferencesModal.jsx` to display/edit recent folders
5. Add tests for `recent_folders` persistence

**Files:** `model/settings.rs`, `services/settings.rs`, `SettingsContext.jsx`, `PreferencesModal.jsx`

---

### 3.6 G-010: Editor.jsx — Deprecation Notice

**Status:** Both `Editor.jsx` and `TipTapEditor.jsx` exist, no deprecation notice
**FR Affected:** FR-008
**Gap ID:** G-010 (P1)

**Implementation Approach:**
1. Add `/** @deprecated Use TipTapEditor.jsx instead */` JSDoc comment to `Editor.jsx`
2. Verify no remaining imports reference `Editor.jsx`
3. Ensure `TipTapEditor.jsx` has full feature parity
4. Update architecture docs to reflect single-editor strategy

**Files:** `Editor.jsx`, `TipTapEditor.jsx`, `App.jsx`

---

### 3.7 G-011: i18n Architecture — Install i18next

**Status:** No i18n library installed, all UI strings hardcoded
**FR Affected:** NFR-014
**PRD Reference:** Article 19 (UI String Externalization, proposed iter-8)
**Gap ID:** G-011 (P1, NEW constitutional gap)

**Implementation Approach:**
1. Install `i18next` + `react-i18next` in `www/`
2. Create `locales/en.json` with hierarchical key naming convention: `{section}.{component}.{purpose}`
3. Externalize all UI strings from React components to locale file
4. Configure i18next in `www/src/`
5. Add lint rule to fail build on hardcoded strings (optional)

**Files:** `package.json`, `locales/en.json`, `i18n.ts` or `i18n.js`, all React components

---

### 3.8 G-016: LinkPopover URL Validation

**Status:** LinkPopover doesn't validate URLs, PRD-12 Section 12.4.3 requires validation
**FR Affected:** FR-011
**PRD Reference:** Article 20 (Security Input Validation, proposed iter-8)
**Gap ID:** G-016 (P1, security)

**Implementation Approach:**
1. Add URL validation in `LinkPopover.jsx` before invoking Tauri commands
2. Block dangerous protocols: `javascript:`, `data:`, `vbscript:`
3. Allow only allowlisted schemes: `http`, `https`, `mailto`
4. Show validation error message to user for invalid URLs
5. Add tests for URL validation

**Files:** `LinkPopover.jsx`

---

## 4. P2 — Medium Priority (Quality & Polish)

### 4.1 G-012: Table Editing — Document Constraints

**Status:** TipTap default table behavior, constrained but safe per PRD
**FR Affected:** FR-016
**Gap ID:** G-012 (P2)

**Implementation Approach:**
1. Document current table editing limitations in `docs/table-editing.md`
2. Add tests for table data integrity during: add row, delete row, add column, delete column
3. No implementation changes unless bugs found

**Files:** `docs/table-editing.md`, `tests/table_editing_tests.rs`

---

### 4.2 G-013: Focus Mode — Visual Verification

**Status:** IntersectionObserver + CSS dimming implemented, NOT visually verified
**FR Affected:** FR-028
**Gap ID:** G-013 (P2)

**Implementation Approach:**
1. Manual visual test: enable focus mode, verify non-current paragraphs dim
2. Verify cursor-anchored paragraph is NOT dimmed
3. Verify dimming works in nested structures (lists, blockquotes)
4. Fix CSS if dimming doesn't apply to nested content

**Files:** `editor.css`, `TipTapEditor.jsx` — visual verification required

---

### 4.3 G-014: Typewriter Mode — Scroll Behavior

**Status:** scrollIntoView implementation exists, scroll centering NOT verified
**FR Affected:** FR-029
**Gap ID:** G-014 (P2)

**Implementation Approach:**
1. Test typewriter mode: cursor should stay at vertical center during typing
2. Test edge cases: near document start/end, window resize events
3. Fix scroll calculation if cursor drifts from center
4. Verify 60 FPS with large documents (100+ paragraphs)

**Files:** `TipTapEditor.jsx` — scroll verification required

---

### 4.4 G-015: PreferencesModal — Full Coverage Verification

**Status:** PreferencesModal exists, full settings coverage NOT verified
**FR Affected:** FR-034
**Gap ID:** G-015 (P2)

**Implementation Approach:**
1. Verify all settings in PreferencesModal read/write through SettingsContext correctly
2. Test that settings persist across app restarts
3. Test keyboard accessibility (Escape to close, Tab navigation)

**Files:** `PreferencesModal.jsx`, `SettingsContext.jsx`

---

### 4.5 G-017: FrontmatterBlock — Visual Polish

**Status:** FrontmatterBlock exists, basic rendering only
**FR Affected:** FR-017
**Gap ID:** G-017 (P2)

**Implementation Approach:**
1. Review `FrontmatterBlock.jsx` styling
2. Enhance with key-value display, consistent theme styling
3. Verify rendering in both light and dark themes

**Files:** `FrontmatterBlock.jsx`

---

### 4.6 G-018: Export — Define ExportFormat Enum

**Status:** ExportServiceTrait exists, only HTML/PDF implemented, no format enum
**FR Affected:** FR-033
**Gap ID:** G-018 (P2)

**Implementation Approach:**
1. Define `ExportFormat` enum (HTML, PDF; future: DOCX, EPUB)
2. Ensure trait supports format-specific options
3. Add factory pattern for format selection

**Files:** `model/export.rs`, `services/export.rs`

---

### 4.7 G-019: PDF Export — Overwrite Protection

**Status:** Atomic write pattern may exist, NOT explicitly verified
**FR Affected:** FR-032
**PRD Reference:** PRD-12 Section 12.5.3
**Gap ID:** G-019 (P2)

**Implementation Approach:**
1. Verify export commands check for existing files
2. Test atomic write (temp→final) pattern
3. Add user confirmation prompt before overwrite

**Files:** `commands/export.rs`

---

### 4.8 G-020: Large Document Behavior — Test at Scale

**Status:** Graceful degradation for >5MB documents NOT tested
**FR Affected:** NFR-003, NFR-014
**Gap ID:** G-020 (P2)

**Implementation Approach:**
1. Create test fixtures at 1MB, 5MB, 10MB
2. Verify editor remains responsive with incremental parsing
3. Test graceful degradation: disable live preview toggle for >5MB

**Files:** `tests/parsing_tests.rs`, `tests/samples/` (large fixtures)

---

## 5. Technical Debt Resolution

### 5.1 TD-002: Command Enum Usage Consistency

**Status:** `editor/commands.rs` defines `Command` enum, many operations bypass it
**Action:** Audit all editor commands, ensure `Command` enum is the primary dispatch mechanism

### 5.2 TD-004: Visual Regression Baselines

**Status:** Visual regression tests exist, baselines may not reflect current implementation
**Action:** Update baseline screenshots to current implementation state

### 5.3 TD-005: File Watcher + Editor Integration Tests

**Status:** External file change detection tested in isolation, NOT with live editor
**Action:** Add integration test: external file modification → editor receives notification → user prompt

### 5.4 TD-006: Incremental Parsing — Large Document Profiling

**Status:** `TreeSitterParser` implements incremental re-parse, NOT profiled with large documents
**Action:** Profile incremental parsing with documents at 1MB, 5MB, 10MB scale

### 5.5 TD-007: FrontmatterBlock — Structured Editing API

**Status:** `FrontmatterBlock.jsx` is basic, `semantic/frontmatter.rs` parses but no structured editing API
**Action:** Covered by G-017 (FrontmatterBlock polish)

### 5.6 TD-008: XSS in Link URLs

**Status:** Covered by G-016 (LinkPopover URL validation)

### 5.7 TD-009: prosemirror-markdown Bridge

**Status:** MVP acceptable per PRD, bridge deferred to post-MVP
**Action:** No action needed — MVP acceptable

### 5.8 TD-010: Settings Model — Field Discrepancy

**Status:** Covered by G-009 (recent_folders missing)

### 5.9 TD-011: cargo-audit in CI

**Status:** Security vulnerability scanning NOT verified in CI
**Action:** Add `cargo-audit` to CI pipeline

### 5.10 TD-012: i18n Library Not Installed

**Status:** Covered by G-011 (install i18next)

---

## 6. Iteration-7 Carry-Forward Status

The following Iteration-7 plan items were COMPLETED in Iteration-8:

| Iter-7 Task | Status | Notes |
|-------------|--------|-------|
| P0-001: Benchmark Infrastructure | ✅ **Infrastructure Built** | 5 benchmark files created. G-003 now is: run + document results |
| P1-009: Editor.jsx Deprecation | ✅ **Partially Done** | TipTap is primary. G-010 now is: add deprecation notice |
| P2-010: Table Editing Documentation | ✅ **Done** | Documented in `docs/table-editing.md` |
| P2-011: Focus Mode Visual Verification | ✅ **Done** | Implemented with IntersectionObserver |
| P2-013: Paste Handling Improvement | ✅ **Done** | Basic paste working. G-004 now: expand TurndownService |
| P2-014: PreferencesModal Integration | ✅ **Done** | SettingsContext integration complete |
| P2-015: Service Interface Verification | ✅ **Done** | All 8 service traits defined |
| P2-016: Rust-Side Autosave | ✅ **Done** | Rust autosave working |
| TD-004: Visual Regression Baselines | ✅ **Done** | Baselines exist, may need update (TD-004 P2) |

---

## 7. Parallelization Strategy

### Phase A — P0 Tasks (Sequential per dependency chain)

| Task | Dependencies | Reason |
|------|-------------|--------|
| G-003: Benchmark Results | None | Can run immediately; infrastructure exists |
| G-001: Cursor Mapping | G-003 infrastructure | Benchmarks validate performance of fix |
| G-002: PDF Export Quality | None | Independent export path |
| G-004: Paste Rich-Text | None | Independent paste pipeline |

### Phase B — P1 Tasks (Independent, can parallelize)

All 8 P1 tasks (G-005 through G-016) are independent and can run in parallel.

### Phase C — P2 Tasks (Independent, can parallelize)

All 9 P2 tasks (G-012 through G-020) are independent and can run in parallel.

---

## 8. Execution Order

```
Week 1 (Days 1-2): G-003: Run cargo bench, document results vs NFR thresholds
Week 1 (Days 1-2): G-001: Cursor mapping TipTap integration tests + edge case fixes
Week 1 (Days 2-3): G-002: PDF export visual verification + quality fixes
Week 1 (Days 3-4): G-004: Expand TurndownService rules for paste conversion
Week 2 (Days 1-5): P1 Tasks (parallel — G-005 through G-016)
Week 3 (Days 1-3): P2 Tasks (parallel — G-012 through G-020)
Week 3 (Days 4-5): TD Resolution (parallel — TD-002, TD-004, TD-005, TD-011)
Week 4: Final verification, regression testing, MVP release prep
```

---

## 9. Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| P0 issues resolved | 4/4 (100%) | All P0 tasks pass |
| P1 issues resolved | 8/8 (100%) | All P1 tasks pass |
| NFR thresholds | 8/8 documented | `cargo bench` results vs PRD-05 thresholds |
| MVP readiness | 100% | All P0 + P1 tasks complete |
| Test coverage | Maintain 95%+ | `cargo test` coverage |

---

## 10. Gap-to-Article Mapping (Constitutional Context)

| Gap | Article Invoked | Constitutional Coverage |
|-----|-----------------|------------------------|
| G-001 | Article 15 + Addendum 15A | Integration-level bidirectional verification |
| G-002 | Article 16 + Addendum 16B | Visual verification with reference fixtures |
| G-003 | Article 17 + Addendum 17A | Benchmark results must be documented |
| G-004 | Article 16 + Addendum 16A | Paste/clipboard conversion fidelity |
| G-005 | Article 12 | Engine operation completeness |
| G-006 | Article 16 | Format conversion fidelity |
| G-007 | Article 1 | Complete feature implementation |
| G-008 | Article 14 | Architecture technology conformance |
| G-009 | Article 3 | Settings-implementation parity |
| G-010 | Maintenance | Editor.jsx deprecation |
| G-011 | Article 19 (NEW) | UI string externalization |
| G-012 | Article 16 | Table editing constraints |
| G-013 | Article 11 | Interactive element behavior |
| G-014 | Article 11 | Interactive element behavior |
| G-015 | Article 3 | Settings-implementation parity |
| G-016 | Article 20 (NEW) | Security input validation |
| G-017 | UX Polish | FrontmatterBlock rendering |
| G-018 | Article 16 | Export architecture boundary |
| G-019 | PRD-12 Section 12.5.3 | Export overwrite protection |
| G-020 | NFR-014 | Large document graceful degradation |

---

*Plan updated based on Iteration-8 gap analysis*
*Iteration-7 plan completed: benchmark infrastructure built, P1/P2 items resolved*
*Iteration-8 focus: run benchmarks, verify integrations, close P0s*
