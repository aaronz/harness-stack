# RustNote MVP Test Plan - Tasks

**Feature:** RustNote MVP Test Plan v3.1  
**Branch:** `feature/test-plan-v3`  
**Total Tasks:** 89  
**Generated:** 2026-04-12

---

## Phase 1: Test Infrastructure Setup

### Story Goal
Set up test infrastructure and directory structure for all testing needs.

### Independent Test Criteria
- `cargo test` runs successfully
- `npm test` runs successfully  
- Playwright tests can be discovered and run
- Fixtures directory structure exists

### Implementation Tasks

- [X] T001 Create Vitest test directory at `rustnote/www/__tests__/`
- [X] T002 Create fixtures directory structure at `rustnote/fixtures/` with subdirs: `markdown/`, `edge-cases/`, `export/`, `recovery/`
- [X] T003 Add Vitest configuration to `rustnote/www/package.json`
- [X] T004 Create `rustnote/scripts/benchmark.ts` for Playwright performance tests
- [X] T005 Add @axe-core/playwright dependency to `rustnote/package.json`
- [X] T006 Create `rustnote/e2e/visual/` directory for visual regression tests
- [X] T007 Create `rustnote/e2e/performance/` directory for performance tests
- [X] T008 Create `rustnote/e2e/a11y/` directory for accessibility tests
- [X] T009 Create `rustnote/e2e/security/` directory for security tests
- [X] T010 Add criterion dependency to `rustnote/Cargo.toml` for Rust benchmarks

---

## Phase 2: Unit Tests - Parser Module (FR-T001-004)

### Story Goal
Ensure all Markdown constructs parse correctly to AST with 100% coverage.

### Independent Test Criteria
- `cargo test parser` passes with all FR-T001-004 test cases
- All Markdown constructs (headings, emphasis, code spans, links, images, lists, blockquotes, tables, code fences, horizontal rules, frontmatter) have test coverage
- Roundtrip serialization produces byte-for-byte identical output
- Malformed input handled gracefully

### Implementation Tasks

- [X] T011 [P] Create parser unit tests for headings H1-H6 at `src-tauri/tests/parser_tests.rs`
- [X] T012 [P] Create parser unit tests for emphasis (bold, italic, strike) at `src-tauri/tests/parser_tests.rs`
- [X] T013 [P] Create parser unit tests for code spans (inline and block) at `src-tauri/tests/parser_tests.rs`
- [X] T014 [P] Create parser unit tests for links and images at `src-tauri/tests/parser_tests.rs`
- [X] T015 [P] Create parser unit tests for unordered and ordered lists at `src-tauri/tests/parser_tests.rs`
- [X] T016 [P] Create parser unit tests for task lists at `src-tauri/tests/parser_tests.rs`
- [X] T017 [P] Create parser unit tests for blockquotes at `src-tauri/tests/parser_tests.rs`
- [X] T018 [P] Create parser unit tests for GFM tables at `src-tauri/tests/parser_tests.rs`
- [X] T019 [P] Create parser unit tests for code fences with language tags at `src-tauri/tests/parser_tests.rs`
- [X] T020 [P] Create parser unit tests for horizontal rules at `src-tauri/tests/parser_tests.rs`
- [X] T021 [P] Create parser unit tests for frontmatter at `src-tauri/tests/parser_tests.rs`
- [X] T022 Create roundtrip serialization tests for all constructs at `src-tauri/tests/parser_tests.rs`
- [X] T023 Create malformed input handling tests at `src-tauri/tests/parser_edge_cases.rs`
- [X] T024 Create deep nesting stress test (100+ levels) at `src-tauri/tests/parser_edge_cases.rs`

---

## Phase 3: Unit Tests - Editor Engine (FR-T010-015)

### Story Goal
Verify cursor movement, selection, smart editing behaviors work correctly.

### Independent Test Criteria
- Cursor correctly moves within and across all inline/block elements
- Double-click selects word, triple-click selects line
- Enter key contextually continues lists, blockquotes, headings
- Backspace at structural boundaries exits appropriately
- Tab/Shift+Tab indent/outdent lists
- IME composition does not cause cursor jump

### Implementation Tasks

- [X] T025 [P] Create cursor movement tests for inline elements at `src-tauri/tests/editor_engine_tests.rs`
- [X] T026 [P] Create cursor movement tests for block elements at `src-tauri/tests/editor_engine_tests.rs`
- [X] T027 [P] Create selection tests (double-click, triple-click, shift+arrow) at `src-tauri/tests/editor_engine_tests.rs`
- [X] T028 [P] Create smart Enter behavior tests (lists, blockquotes, headings) at `src-tauri/tests/editor_transforms.rs`
- [X] T029 [P] Create smart Backspace behavior tests at `src-tauri/tests/editor_transforms.rs`
- [X] T030 [P] Create Tab/Shift+Tab indent/outdent tests at `src-tauri/tests/editor_transforms.rs`
- [X] T031 Create IME composition tests at `src-tauri/tests/editor_engine_tests.rs`

---

## Phase 4: Unit Tests - Buffer & Settings (FR-T020-034)

### Story Goal
Verify buffer operations, Unicode handling, settings persistence, and recovery.

### Independent Test Criteria
- Insert/delete at any position maintains document integrity
- Unicode (CJK, emoji) handled correctly
- 5MB document operations complete in < 100ms
- Settings persist across restarts
- Corrupted settings fall back to defaults
- Snapshots created/recovered correctly

### Implementation Tasks

- [X] T032 [P] Create buffer insert/delete tests at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T033 [P] Create Unicode handling tests at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T034 [P] Create large document performance test at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T035 [P] Create UTF-8 validation tests at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T036 [P] Create settings persistence tests at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T037 [P] Create settings corruption handling tests at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T038 [P] Create snapshot creation tests at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T039 [P] Create recovery from snapshot tests at `src-tauri/tests/buffer_settings_tests.rs`
- [X] T040 Create snapshot tests at `src-tauri/tests/buffer_settings_tests.rs`

---

## Phase 5: Integration Tests - File Operations (FR-T040-048)

### Story Goal
Verify file operations (new, open, save, save-as, drag-drop, workspace) work correctly.

### Independent Test Criteria
- New document opens with untitled state
- Save creates file at specified path
- Save-as creates new file
- Drag-drop opens .md files
- Workspace shows file tree
- Recent files persist
- Read-only files handled gracefully
- Unsaved changes prompt shown

### Implementation Tasks

- [X] T041 [P] Add integration tests for document model at `src-tauri/tests/integration_file_tests.rs`
- [X] T042 [P] Add integration tests for workspace model at `src-tauri/tests/integration_file_tests.rs`
- [X] T043 [P] Add integration tests for export options at `src-tauri/tests/integration_file_tests.rs`
- [X] T044 [P] Add integration tests for file serialization at `src-tauri/tests/integration_file_tests.rs`
- [X] T045 [P] Add integration tests for recovery snapshot at `src-tauri/tests/integration_file_tests.rs`
- [ ] T046 [P] Add Playwright tests for workspace folder open at `e2e/integration/workspace.spec.ts`
- [ ] T047 [P] Add Playwright tests for recent files persistence at `e2e/integration/file_operations.spec.ts`
- [ ] T048 [P] Add Playwright tests for read-only file handling at `e2e/integration/file_operations.spec.ts`
- [ ] T049 Add Playwright tests for unsaved changes prompt at `e2e/integration/file_operations.spec.ts`

---

## Phase 6: Integration Tests - Editor & Export (FR-T050-067)

### Story Goal
Verify editor integration (live render, undo/redo, paste, find) and export functionality.

### Independent Test Criteria
- Markdown renders immediately as user types
- Undo/redo restores correct states
- Paste converts rich text to Markdown
- Find highlights matches
- HTML export is valid and self-contained
- PDF export is valid with correct page sizing
- Export can be cancelled

### Implementation Tasks

- [X] T050 [P] Add integration tests for parser + transform at `src-tauri/tests/integration_editor_tests.rs`
- [X] T051 [P] Add integration tests for cursor + selection at `src-tauri/tests/integration_editor_tests.rs`
- [X] T052 [P] Add integration tests for document operations at `src-tauri/tests/integration_editor_tests.rs`
- [ ] T053 [P] Add Playwright tests for paste plain text at `e2e/integration/editor_integration.spec.ts`
- [ ] T054 [P] Add Playwright tests for paste rich text conversion at `e2e/integration/editor_integration.spec.ts`
- [ ] T055 [P] Add Playwright tests for find in document at `e2e/integration/editor_integration.spec.ts`
- [X] T056 [P] Add integration tests for HTML export at `src-tauri/tests/integration_editor_tests.rs`
- [X] T057 [P] Add integration tests for PDF export at `src-tauri/tests/integration_editor_tests.rs`
- [ ] T058 [P] Add Playwright tests for HTML export with images at `e2e/integration/export.spec.ts`
- [ ] T059 [P] Add Playwright tests for HTML export code highlighting at `e2e/integration/export.spec.ts`
- [ ] T060 [P] Add Playwright tests for PDF export validity at `e2e/integration/export.spec.ts`
- [ ] T061 [P] Add Playwright tests for PDF export page sizing at `e2e/integration/export.spec.ts`
- [ ] T062 [P] Add Playwright tests for export overwrite prompt at `e2e/integration/export.spec.ts`
- [ ] T063 Add Playwright tests for export cancellation at `e2e/integration/export.spec.ts`

---

## Phase 7: Integration Tests - Workspace & External Changes (FR-T070-083)

### Story Goal
Verify workspace CRUD operations and external change detection.

### Independent Test Criteria
- Create/rename/delete files via context menu works
- Sidebar refreshes on external changes
- Folders expand to show children
- Prompt shown when open file modified externally
- Error shown when open file deleted externally

### Implementation Tasks

- [X] T064 [P] Add integration tests for workspace model at `src-tauri/tests/integration_file_tests.rs`
- [ ] T065 [P] Add Playwright tests for workspace rename file at `e2e/integration/workspace.spec.ts`
- [ ] T066 [P] Add Playwright tests for workspace delete file at `e2e/integration/workspace.spec.ts`
- [ ] T067 [P] Add Playwright tests for external change detection at `e2e/integration/external_change.spec.ts`
- [ ] T068 [P] Add Playwright tests for nested folder navigation at `e2e/integration/workspace.spec.ts`
- [ ] T069 [P] Add Playwright tests for recent folders persistence at `e2e/integration/workspace.spec.ts`
- [ ] T070 Add Playwright tests for external file modification prompt at `e2e/integration/external_change.spec.ts`
- [ ] T071 Add Playwright tests for external file deletion error at `e2e/integration/external_change.spec.ts`
- [ ] T072 Add Playwright tests for keep/reload external version at `e2e/integration/external_change.spec.ts`

---

## Phase 8: E2E Tests - Critical User Flows (FR-T090-097)

### Story Goal
Verify all critical user flows work end-to-end.

### Independent Test Criteria
- User can open, edit, save, and verify persistence of existing documents
- User can create structured documents with all Markdown elements
- User can insert and manage images
- User can write and export code-heavy documentation
- User can export to HTML and PDF
- User can recover unsaved work after crash
- Focus mode and typewriter mode work correctly

### Implementation Tasks

- [ ] T073 [P] Add E2E test for open and edit existing document at `e2e/critical/open-edit.spec.ts`
- [ ] T074 [P] Add E2E test for create structured document at `e2e/critical/create-structured.spec.ts`
- [ ] T075 [P] Add E2E test for insert and manage images at `e2e/critical/image-management.spec.ts`
- [ ] T076 [P] Add E2E test for technical documentation with code at `e2e/critical/tech-doc.spec.ts`
- [ ] T077 [P] Add E2E test for export workflow at `e2e/critical/export-workflow.spec.ts`
- [ ] T078 [P] Add E2E test for crash recovery at `e2e/critical/crash-recovery.spec.ts`
- [ ] T079 [P] Add E2E test for focus mode at `e2e/critical/focus-mode.spec.ts`
- [ ] T080 [P] Add E2E test for typewriter mode at `e2e/critical/typewriter-mode.spec.ts`

---

## Phase 9: E2E Tests - Regression Suite (FR-T100-109)

### Story Goal
Ensure existing functionality remains working through comprehensive regression testing.

### Independent Test Criteria
- Open/save 50 different .md files without corruption
- Edit-save-reopen 20 files with fidelity verification
- Undo/redo 30 operations without crash
- Export 10 files to HTML and PDF successfully
- Theme toggle works on all platforms
- 5MB document scrolls at 60 FPS
- 1000-item list renders correctly

### Implementation Tasks

- [ ] T081 [P] Add regression test for open/save 50 files at `e2e/regression/file-operations.spec.ts`
- [ ] T082 [P] Add regression test for edit-save-reopen fidelity at `e2e/regression/fidelity.spec.ts`
- [ ] T083 [P] Add regression test for undo/redo operations at `e2e/regression/undo-redo.spec.ts`
- [ ] T084 [P] Add regression test for HTML export (10 files) at `e2e/regression/export.spec.ts`
- [ ] T085 [P] Add regression test for PDF export (10 files) at `e2e/regression/export.spec.ts`
- [ ] T086 [P] Add regression test for Markdown fixtures parsing at `e2e/regression/fixtures.spec.ts`
- [ ] T087 [P] Add regression test for theme toggle at `e2e/regression/theme.spec.ts`
- [ ] T088 [P] Add regression test for find/replace with regex at `e2e/regression/find-replace.spec.ts`
- [ ] T089 [P] Add regression test for 5MB document scroll at `e2e/regression/performance.spec.ts`
- [ ] T090 Add regression test for 1000-item list rendering at `e2e/regression/large-list.spec.ts`

---

## Phase 10: Visual Regression Tests (FR-T110-127)

### Story Goal
Detect UI regressions in themes, modes, and components.

### Independent Test Criteria
- Dark/light themes render correctly with proper contrast
- Focus mode properly dims non-active paragraphs
- Typewriter mode centers cursor at 40-60% viewport height
- All Markdown elements (headings, lists, tables, code, etc.) render correctly
- Visual styling is consistent

### Implementation Tasks

- [X] T091 [P] Add visual test for themes at `e2e/visual/visual-regression.spec.ts`
- [X] T092 [P] Add visual test for headings at `e2e/visual/visual-regression.spec.ts`
- [X] T093 [P] Add visual test for emphasis at `e2e/visual/visual-regression.spec.ts`
- [X] T094 [P] Add visual test for code blocks at `e2e/visual/visual-regression.spec.ts`
- [X] T095 [P] Add visual test for lists at `e2e/visual/visual-regression.spec.ts`
- [X] T096 [P] Add visual test for blockquotes at `e2e/visual/visual-regression.spec.ts`
- [X] T097 [P] Add visual test for layout at `e2e/visual/visual-regression.spec.ts`
- [ ] T098 [P] Add visual test for code fence highlighting at `e2e/visual/code-fences.spec.ts`
- [ ] T099 [P] Add visual test for table alignment at `e2e/visual/tables.spec.ts`
- [ ] T100 [P] Add visual test for link styling at `e2e/visual/links.spec.ts`
- [ ] T101 [P] Add visual test for image placeholder at `e2e/visual/images.spec.ts`
- [ ] T102 [P] Add visual test for task list checkboxes at `e2e/visual/task-lists.spec.ts`
- [ ] T103 [P] Add visual test for outline panel at `e2e/visual/outline-panel.spec.ts`
- [ ] T104 [P] Add visual test for search highlight at `e2e/visual/search-highlight.spec.ts`
- [ ] T105 [P] Add visual test for selection highlight at `e2e/visual/selection.spec.ts`
- [ ] T106 [P] Add visual test for empty state at `e2e/visual/empty-state.spec.ts`
- [ ] T107 [P] Add visual test for toolbar styling at `e2e/visual/toolbar.spec.ts`
- [ ] T108 [P] Add visual test for scrollbar styling at `e2e/visual/scrollbar.spec.ts`

---

## Phase 11: Performance Benchmarks

### Story Goal
Validate all NFR thresholds are met.

### Independent Test Criteria
- Cold start < 2s (target), < 3s (critical)
- Hot file open < 500ms
- Keystroke to render < 100ms
- Save operation < 200ms
- Export times within thresholds
- Memory usage < 300MB
- Scroll maintains 60 FPS

### Implementation Tasks

- [X] T109 [P] Create performance benchmarks at `scripts/benchmark.ts`
- [X] T110 [P] Create Rust benchmarks for parsing at `rustnote/src-tauri/benches/parsing.rs` ✓ (Note: criterion blocked by Rust 1.93.1 compatibility)
- [X] T111 [P] Create Rust benchmarks for serialization at `rustnote/src-tauri/benches/serialization.rs` ✓ (Note: criterion blocked by Rust 1.93.1 compatibility)
- [X] T112 [P] Create Rust benchmarks for transforms at `rustnote/src-tauri/benches/transforms.rs` ✓ (Note: criterion blocked by Rust 1.93.1 compatibility)
- [ ] T113 [P] Add Playwright test for cold start at `e2e/performance/cold-start.spec.ts`
- [ ] T114 [P] Add Playwright test for hot file open at `e2e/performance/hot-open.spec.ts`
- [ ] T115 [P] Add Playwright test for keystroke to render at `e2e/performance/keystroke.spec.ts`
- [ ] T116 [P] Add Playwright test for save operation at `e2e/performance/save.spec.ts`
- [ ] T117 [P] Add Playwright test for HTML export at `e2e/performance/export.spec.ts`
- [ ] T118 [P] Add Playwright test for PDF export at `e2e/performance/export.spec.ts`
- [ ] T119 [P] Add Playwright test for theme switch at `e2e/performance/theme-switch.spec.ts`
- [ ] T120 [P] Add Playwright test for memory usage at `e2e/performance/memory.spec.ts`
- [ ] T121 [P] Add Playwright test for scroll FPS at `e2e/performance/scroll.spec.ts`

---

## Phase 12: Security Tests (FR-T130-154)

### Story Goal
Ensure no security vulnerabilities in input validation, HTML sanitization, and export.

### Independent Test Criteria
- Path traversal attempts rejected
- Null byte injection prevented
- Invalid UTF-8 handled safely
- HTML exports contain no scripts, event handlers, iframes, styles
- No JavaScript URLs or data URLs in exports
- Export cannot write outside target directory

### Implementation Tasks

- [X] T122 [P] Add security tests for input sanitization at `e2e/security/security.spec.ts`
- [X] T123 [P] Add security tests for XSS prevention at `e2e/security/security.spec.ts`
- [X] T124 [P] Add security tests for HTML sanitization at `e2e/security/security.spec.ts`
- [X] T125 [P] Add security tests for export security at `e2e/security/security.spec.ts`
- [X] T126 [P] Add security tests for memory security at `e2e/security/security.spec.ts`
- [ ] T127 [P] Add security tests for null byte injection at `e2e/security/null-byte.spec.ts`
- [ ] T128 [P] Add security tests for long path handling at `e2e/security/long-path.spec.ts`
- [ ] T129 [P] Add security tests for invalid UTF-8 at `e2e/security/invalid-utf8.spec.ts`
- [ ] T130 [P] Add security tests for control characters at `e2e/security/control-chars.spec.ts`
- [ ] T131 [P] Add security tests for HTML sanitization (script injection) at `e2e/security/html-sanitization.spec.ts`
- [ ] T132 [P] Add security tests for HTML sanitization (event handlers) at `e2e/security/html-sanitization.spec.ts`
- [ ] T133 [P] Add security tests for HTML sanitization (JavaScript URLs) at `e2e/security/html-sanitization.spec.ts`
- [ ] T134 [P] Add security tests for HTML sanitization (iframe/style tags) at `e2e/security/html-sanitization.spec.ts`
- [ ] T135 [P] Add security tests for export script removal at `e2e/security/export-security.spec.ts`
- [ ] T136 Configure cargo-fuzz for parser fuzzing at `rustnote/fuzz/runner.rs`

---

## Phase 13: Accessibility Tests (FR-T160-183)

### Story Goal
Ensure WCAG AA compliance for keyboard navigation and screen readers.

### Independent Test Criteria
- Tab moves focus to editor, through toolbar, to sidebar
- Ctrl+O opens file, Ctrl+S saves, Ctrl+F finds
- Escape closes dialogs
- Screen reader announces headings, links, images, lists, tables correctly
- Color contrast meets 4.5:1 (text) and 3:1 (large text)
- Focus indicators visible on all interactive elements

### Implementation Tasks

- [X] T137 [P] Add a11y tests for document structure at `e2e/a11y/accessibility.spec.ts`
- [X] T138 [P] Add a11y tests for ARIA roles at `e2e/a11y/accessibility.spec.ts`
- [X] T139 [P] Add a11y tests for headings structure at `e2e/a11y/accessibility.spec.ts`
- [X] T140 [P] Add a11y tests for keyboard navigation at `e2e/a11y/accessibility.spec.ts`
- [X] T141 [P] Add a11y tests for focus visibility at `e2e/a11y/accessibility.spec.ts`
- [X] T142 [P] Add a11y tests for color contrast at `e2e/a11y/accessibility.spec.ts`
- [X] T143 [P] Add a11y tests for screen reader support at `e2e/a11y/accessibility.spec.ts`
- [X] T144 [P] Add a11y tests for form labels at `e2e/a11y/accessibility.spec.ts`
- [ ] T145 [P] Add a11y tests for screen reader headings at `e2e/a11y/screen-reader.spec.ts`
- [ ] T146 [P] Add a11y tests for screen reader links at `e2e/a11y/screen-reader.spec.ts`
- [ ] T147 [P] Add a11y tests for screen reader images at `e2e/a11y/screen-reader.spec.ts`
- [ ] T148 [P] Add a11y tests for screen reader lists/tables at `e2e/a11y/screen-reader.spec.ts`
- [ ] T149 [P] Add a11y tests for non-color information at `e2e/a11y/non-color-info.spec.ts`

---

## Phase 14: CI/CD Integration

### Story Goal
Integrate all tests into CI pipeline with proper gates.

### Independent Test Criteria
- All required gates pass (cargo fmt, clippy, test, eslint, build)
- cargo audit runs without vulnerabilities
- Tests run on every PR and merge
- Visual regression baseline captured

### Implementation Tasks

- [X] T150 Add test commands to CI configuration in `.github/workflows/test.yml`
- [ ] T151 Configure cargo audit in CI at `.github/workflows/security.yml`
- [ ] T152 Set up cargo-fuzz configuration at `.cargo/fuzz.toml`
- [ ] T153 Add visual regression baseline capture step to CI
- [ ] T154 Add performance benchmark collection to CI
- [ ] T155 Verify all CI gates pass successfully

---

## Dependencies

```
Phase 1 (Setup)
  └── All subsequent phases depend on setup

Phase 2 (Unit Tests - Parser)
  └── Phase 3, 4 can run in parallel

Phase 3 (Unit Tests - Editor Engine)
  └── Phase 5

Phase 4 (Unit Tests - Buffer/Settings)
  └── Phase 5

Phase 5 (Integration - File Ops)
  └── Phase 6, 7 (can run in parallel)

Phase 6 (Integration - Editor/Export)
  └── Phase 8

Phase 7 (Integration - Workspace)
  └── Phase 8

Phase 8 (E2E - Critical Flows)
  └── Phase 9

Phase 9 (E2E - Regression)
  └── Phase 10

Phase 10 (Visual Regression)
  └── Phase 11

Phase 11 (Performance)
  └── Phase 12

Phase 12 (Security)
  └── Phase 13

Phase 13 (Accessibility)
  └── Phase 14

Phase 14 (CI/CD)
  └── DONE
```

---

## Parallel Execution Opportunities

| Phase | Parallel Tasks | Can Run With |
|-------|---------------|--------------|
| Phase 2 | T011-T024 (12 parallel parser tests) | Phase 3, 4 |
| Phase 3 | T025-T031 (7 parallel editor tests) | Phase 2, 4 |
| Phase 4 | T032-T040 (9 parallel buffer tests) | Phase 2, 3 |
| Phase 5 | T041-T049 (9 parallel file ops tests) | Phase 6, 7 |
| Phase 6 | T050-T063 (14 parallel editor/export tests) | Phase 5, 7 |
| Phase 7 | T064-T072 (9 parallel workspace tests) | Phase 5, 6 |
| Phase 8 | T073-T080 (8 parallel E2E tests) | Phase 9 |
| Phase 9 | T081-T090 (10 parallel regression tests) | Phase 8 |
| Phase 10 | T091-T108 (18 parallel visual tests) | Phase 11 |
| Phase 11 | T109-T121 (13 parallel performance tests) | Phase 10 |
| Phase 12 | T122-T136 (15 parallel security tests) | Phase 13 |
| Phase 13 | T137-T149 (13 parallel a11y tests) | Phase 14 |

---

## Suggested MVP Scope

For MVP release, implement in this order:
1. **Phase 1** - Setup (T001-T010)
2. **Phase 2** - Unit Tests Parser (T011-T024)
3. **Phase 8** - E2E Critical Flows (T073-T080)

This covers the core test requirements and can be completed in ~1 week.

---

## Task Count Summary

| Phase | Task Count | Parallel Opportunities |
|-------|-----------|----------------------|
| Phase 1: Setup | 10 | 10 |
| Phase 2: Unit Tests - Parser | 14 | 12 |
| Phase 3: Unit Tests - Editor Engine | 7 | 7 |
| Phase 4: Unit Tests - Buffer/Settings | 9 | 9 |
| Phase 5: Integration - File Operations | 9 | 9 |
| Phase 6: Integration - Editor/Export | 14 | 14 |
| Phase 7: Integration - Workspace | 9 | 6 |
| Phase 8: E2E - Critical Flows | 8 | 8 |
| Phase 9: E2E - Regression | 10 | 9 |
| Phase 10: Visual Regression | 18 | 18 |
| Phase 11: Performance | 13 | 13 |
| Phase 12: Security | 15 | 15 |
| Phase 13: Accessibility | 13 | 13 |
| Phase 14: CI/CD | 6 | 1 |
| **TOTAL** | **155** | |

---

*Generated: 2026-04-12*
