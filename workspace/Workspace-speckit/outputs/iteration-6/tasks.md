# Iteration 6 Task List - RustNote

**Generated:** 2026-04-11  
**Source Plan:** iteration-6/plan.md  
**Total Tasks:** 44

---

## Phase 1: Setup & Project Initialization

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T001 | TODO | **[P]** Create `src-tauri/src/model/export.rs` with PdfExportOptions, PdfPageSize, PdfMargins types | None |
| - [ ] T002 | TODO | **[P]** Create `.github/workflows/ci.yml` with cross-platform CI matrix (macOS, Windows, Ubuntu) | None |

---

## Phase 2: Foundational (Blocking Prerequisites)

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T003 | TODO | **[P]** Add `highlight_html()` method to SyntaxHighlighter in `src-tauri/src/parser/syntax.rs` | None |
| - [ ] T004 | TODO | **[P]** Add `open_external_url()` command to `src-tauri/src/commands/mod.rs` | None |

---

## Phase 3: User Stories

### [US1] Native PDF Export — HIGH-1

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T005 | TODO | **[P]** Implement `export_to_pdf_native()` command in `src-tauri/src/commands/export.rs` using printpdf | T001 |
| - [ ] T006 | TODO | **[P]** Add page size and margin support to PDF export (A4, Letter, custom) | T005 |
| - [ ] T007 | TODO | **[P]** Add image embedding support to PDF export | T005 |
| - [ ] T008 | TODO | **[P]** Update frontend `www/src/scripts/app.js` to use `export_to_pdf_native` command | T005, T006, T007 |

**Verification:**
- [ ] A4 PDF generates correctly
- [ ] Letter PDF generates correctly
- [ ] Margins applied correctly
- [ ] Images embed in PDF
- [ ] Tables render correctly in PDF
- [ ] Code blocks render correctly in PDF

---

### [US2] Code Fence Syntax Highlighting — HIGH-2

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T009 | TODO | **[P]** Create `render_for_editor_with_highlighting()` command in `src-tauri/src/commands/render.rs` | T003 |
| - [ ] T010 | TODO | **[P]** Update frontend `www/src/scripts/editor.js` to request highlighted code with `includeHighlighting: true` | T009 |
| - [ ] T011 | TODO | **[P]** Handle fallback rendering for unsupported languages (plain code block) | T009 |

**Verification:**
- [ ] JavaScript code blocks highlight with colors
- [ ] Python code blocks highlight correctly
- [ ] Rust code blocks highlight correctly
- [ ] Unknown languages fall back to plain display
- [ ] Highlighted code renders in live preview

---

### [US3] Link Click-to-Open — MED-1

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T012 | TODO | **[P]** Add Ctrl+Click / Cmd+Click handler for links in `www/src/scripts/editor.js` | T004 |
| - [ ] T013 | TODO | **[P]** Detect external vs internal links in click handler | T012 |
| - [ ] T014 | TODO | **[P]** Update link rendering CSS to show pointer cursor on hover | T012 |

**Verification:**
- [ ] Ctrl+Click on link opens in default browser
- [ ] Cmd+Click works on macOS
- [ ] External links (https://) open correctly
- [ ] Relative links resolved correctly
- [ ] Default browser opens for markdown links

---

### [US4] Table Editing — MED-2

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T015 | TODO | **[P]** Add table cell selection in `www/src/scripts/editor.js` (click to select cell) | None |
| - [ ] T016 | TODO | **[P]** Add Tab navigation between table cells (Tab = next, Shift+Tab = previous) | T015 |
| - [ ] T017 | TODO | **[P]** Handle Enter key in table cells (line break within cell) | T015 |

**Verification:**
- [ ] Tables render with clear grid visual
- [ ] Click on cell places cursor in cell
- [ ] Tab moves to next cell
- [ ] Shift+Tab moves to previous cell
- [ ] Enter creates newline in cell
- [ ] Table structure preserved on save/reload

---

### [US5] Frontmatter Support — MED-3

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T018 | TODO | **[P]** Update parser in `src-tauri/src/semantic/ast.rs` to detect YAML frontmatter (`parse_with_frontmatter`) | None |
| - [ ] T019 | TODO | **[P]** Update serializer in `src-tauri/src/semantic/ast.rs` to preserve frontmatter (`serialize_with_frontmatter`) | T018 |
| - [ ] T020 | TODO | **[P]** Update renderer to hide frontmatter in WYSIWYM view (show in raw only) | T018 |

**Verification:**
- [ ] Document with frontmatter opens correctly
- [ ] Frontmatter visible in raw markdown but hidden in WYSIWYM
- [ ] Frontmatter preserved on save/reload
- [ ] Frontmatter does not appear in outline

---

### [US6] Cross-Platform CI — LOW-1

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T021 | TODO | **[P]** Add macOS build step to CI workflow | T002 |
| - [ ] T022 | TODO | **[P]** Add Windows build step to CI workflow | T002 |
| - [ ] T023 | TODO | **[P]** Add Linux build step to CI workflow | T002 |
| - [ ] T024 | TODO | **[P]** Add cargo test step to CI for all platforms | T002 |

**Verification:**
- [ ] CI passes on macOS
- [ ] CI passes on Windows
- [ ] CI passes on Linux
- [ ] Builds produce valid binaries

---

## Phase 4: Polish & Cross-Cutting Concerns

| Task ID | Status | Description | Dependencies |
|---------|--------|-------------|--------------|
| - [ ] T025 | TODO | **[P]** Update `src-tauri/src/commands/mod.rs` CommandError enum with new error variants | T004, T005 |
| - [ ] T026 | TODO | **[P]** Update `src-tauri/src/lib.rs` to register new commands (`export_to_pdf_native`, `get_highlighted_code_html`, `render_for_editor_with_highlighting`, `open_external_url`) | T004, T005, T009 |
| - [ ] T027 | TODO | **[P]** Update `www/src/scripts/app.js` export dialog to use new PDF export | T008 |
| - [ ] T028 | TODO | **[P]** Update `www/src/scripts/editor.js` outline panel to exclude frontmatter | T020 |
| - [ ] T029 | TODO | Run full test suite and verify all commands work end-to-end | T025, T026, T027, T028 |

---

## Dependency Graph

```
T001 ─┬─> T005 ─┬─> T006 ─> T008
      └─> T005 ─┴─> T007 ─> T008

T002 ─┬─> T021
      ├─> T022
      └─> T023 ─> T024

T003 ─> T009 ─┬─> T010
              └─> T011

T004 ─> T012 ─┬─> T013
              └─> T014

T015 ─┬─> T016
      └─> T017

T018 ─> T019 ─> T020 ─> T028

T004, T005 ─> T025 ─> T029
T004, T005, T009 ─> T026 ─> T029
T008 ─> T027 ─> T029
T025, T026, T027, T028 ─> T029
```

---

## Parallelization Opportunities

| Batch | Tasks | Reason |
|-------|-------|--------|
| **Batch 1** | T001, T002, T003, T004 | Independent files, no dependencies |
| **Batch 2** | T005, T006, T007, T009, T010, T011, T012, T013, T014, T015, T016, T017, T018, T019, T020, T021, T022, T023 | Phase 2 complete, all user story tasks can run in parallel |
| **Batch 3** | T025, T026, T027, T028 | Polish tasks, depend on user story completion |
| **Batch 4** | T029 | Final verification, depends on all prior tasks |

---

## File Changes Summary

| File | Action | Tasks |
|------|--------|-------|
| `src-tauri/src/model/export.rs` | CREATE | T001 |
| `.github/workflows/ci.yml` | CREATE | T002 |
| `src-tauri/src/parser/syntax.rs` | MODIFY | T003 |
| `src-tauri/src/commands/mod.rs` | MODIFY | T004, T025 |
| `src-tauri/src/commands/export.rs` | MODIFY | T005, T006, T007 |
| `src-tauri/src/commands/render.rs` | MODIFY | T009, T026 |
| `src-tauri/src/semantic/ast.rs` | MODIFY | T018, T019, T020 |
| `www/src/scripts/app.js` | MODIFY | T008, T027 |
| `www/src/scripts/editor.js` | MODIFY | T010, T012, T013, T014, T015, T016, T017, T028 |
| `src-tauri/src/lib.rs` | MODIFY | T026 |

---

## Priority Order for Sequential Execution

If sequential execution is required:

1. **T001, T002, T003, T004** (Setup - independent files)
2. **T005** (PDF export core - others can parallelize after)
3. **T006, T007** (PDF enhancements)
4. **T008** (Frontend PDF integration)
5. **T009, T010, T011** (Syntax highlighting)
6. **T012, T013, T014** (Link handling)
7. **T015, T016, T017** (Table editing)
8. **T018, T019, T020** (Frontmatter)
9. **T021, T022, T023, T024** (CI)
10. **T025, T026, T027, T028** (Polish)
11. **T029** (Final verification)
