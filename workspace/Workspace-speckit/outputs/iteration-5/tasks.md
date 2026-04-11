# RustNote Implementation Tasks

## User Stories (for reference)

| US ID | Title | Phase |
|--------|-------|-------|
| US1 | Open, edit, and save Markdown files | Phase 1 |
| US2 | Rich Markdown rendering with live preview | Phase 1-2 |
| US3 | Structural editing (lists, quotes, code blocks) | Phase 2 |
| US4 | Undo/redo with correct granularity | Phase 2 |
| US5 | File workspace with tree navigation | Phase 3 |
| US6 | Auto-save and crash recovery | Phase 3 |
| US7 | External change detection | Phase 3 |
| US8 | Theme switching (light/dark) | Phase 4 |
| US9 | Focus mode and typewriter mode | Phase 4 |
| US10 | Typography customization | Phase 4 |
| US11 | HTML export | Phase 5 |
| US12 | PDF export | Phase 5 |
| US13 | Find and replace | Phase 6 |
| US14 | Outline/TOC panel | Phase 6 |
| US15 | Settings persistence | Phase 6 |
| US16 | Keyboard shortcuts | Phase 6 |
| US17 | Image handling | Phase 6 |
| US18 | Frontmatter support | Phase 6 |

---

## Phase 0: Research (Week 1)

> **Blocker**: None. All tasks can run in parallel.

### Research Tasks

- [ ] T001 [P] [US1] Research Tauri v1 IPC performance for keystroke-level events - benchmark 1000 char insertions over IPC vs direct Rust `rustnote-core/src/`
- [ ] T002 [P] [US2] Research pulldown-cmark AST fidelity for structural editing - test roundtrip parse→serialize→parse for tables, task lists, nested lists `rustnote-core/src/parser/`
- [ ] T003 [P] [US1] Research file watching granularity with `notify` crate - benchmark folder with 1000 Markdown files `rustnote-core/src/services/`
- [ ] T004 [P] [US6] Research crash recovery granularity - design snapshot strategy (time-based vs change-count-based) `rustnote-core/src/core/recovery.rs`
- [ ] T005 [P] [US4] Research undo/redo strategy - operation log vs command pattern vs persistent undo stack `rustnote-core/src/core/editor.rs`
- [ ] T006 [P] [US2] Research frontend framework choice - evaluate vanilla TS vs solid vs svelte `src-ui/`

---

## Phase 1: Foundation (Week 2)

> **Blocker**: Phase 0 complete

### 1.1 Project Setup

- [ ] T007 [US1] Set up Tauri project scaffold with logging configuration `src-tauri/`
- [ ] T008 [US1] Verify empty shell Tauri app builds and launches `src-tauri/`
- [ ] T009 [US1] Create Rust core module structure with `rustnote-core` crate `rustnote-core/src/`
- [ ] T010 [US1] Create workspace manifest and toolchain configuration `Cargo.toml`, `rust-toolchain.toml`

### 1.2 Core Types

- [ ] T011 [P] [US1] Define `NodeKind` enum with all semantic node types `rustnote-core/src/core/document.rs`
- [ ] T012 [P] [US1] Define `Node` struct with kind, range, and children `rustnote-core/src/core/document.rs`
- [ ] T013 [P] [US1] Define `Document` struct with root node and source length `rustnote-core/src/core/document.rs`
- [ ] T014 [P] [US1] Define `Cursor` struct with offset and affinity `rustnote-core/src/core/selection.rs`
- [ ] T015 [P] [US1] Define `Selection` struct with anchor and head `rustnote-core/src/core/selection.rs`

### 1.3 Buffer Implementation

- [ ] T016 [US1] Implement `Buffer` with rope-based text storage `rustnote-core/src/core/buffer.rs`
- [ ] T017 [US1] Implement `Buffer::insert` with cursor position update `rustnote-core/src/core/buffer.rs`
- [ ] T018 [US1] Implement `Buffer::delete` with range deletion `rustnote-core/src/core/buffer.rs`
- [ ] T019 [US1] Implement `Buffer::is_dirty` and dirty tracking `rustnote-core/src/core/buffer.rs`
- [ ] T020 [US1] Write Buffer unit tests passing 100% `rustnote-core/tests/buffer_tests.rs`

### 1.4 Tauri Commands (Rust Core)

- [ ] T021 [P] [US1] Implement `doc_open` Tauri command `src-tauri/src/commands.rs`
- [ ] T022 [P] [US1] Implement `doc_save` Tauri command `src-tauri/src/commands.rs`
- [ ] T023 [P] [US1] Implement `doc_reload` Tauri command `src-tauri/src/commands.rs`
- [ ] T024 [P] [US1] Wire doc commands to frontend `src-ui/services/tauri.ts`

### 1.5 Frontend Editor Canvas

- [ ] T025 [US1] Implement `EditorCanvas` component with contenteditable div `src-ui/components/editor/EditorCanvas.ts`
- [ ] T026 [US1] Implement `CursorRenderer` for cursor/selection display `src-ui/components/editor/CursorRenderer.ts`
- [ ] T027 [US1] Implement keyboard input handling `src-ui/services/keyboard.ts`
- [ ] T028 [US1] Implement `SelectionManager` in frontend `src-ui/components/editor/SelectionManager.ts`

### 1.6 Live Rendering

- [ ] T029 [US2] Implement basic Markdown renderer (paragraphs, headings) `src-ui/components/editor/Renderer.ts`
- [ ] T030 [P] [US2] Implement bold, italic, inline code rendering `src-ui/components/editor/Renderer.ts`
- [ ] T031 [P] [US2] Implement blockquote rendering `src-ui/components/editor/Renderer.ts`
- [ ] T032 [P] [US2] Implement list rendering (ordered/unordered) `src-ui/components/editor/Renderer.ts`

### 1.7 Save Functionality

- [ ] T033 [US1] Implement manual save with dirty state indicator `src-ui/components/toolbar/Toolbar.ts`
- [ ] T034 [US1] Implement `mark_saved` to clear dirty flag `rustnote-core/src/core/buffer.rs`

### 1.8 Phase 1 Milestone Verification

- [ ] T035 [US1] **Milestone**: Editor can open `.md` file, type, see formatting, save - integration test passes

---

## Phase 2: Core Editing (Week 3-4)

> **Blocker**: Phase 1 complete, Tauri commands wired

### 2.1 Markdown Parser

- [ ] T036 [US2] Integrate `pulldown-cmark` with GFM extensions `rustnote-core/src/parser/pulldown_adapter.rs`
- [ ] T037 [US2] Implement `pulldown_adapter.rs` to parse Markdown to pulldown AST `rustnote-core/src/parser/pulldown_adapter.rs`
- [ ] T038 [US2] Implement semantic tree conversion from pulldown AST `rustnote-core/src/parser/semantic_tree.rs`
- [ ] T039 [US2] Implement `SemanticTree::from_markdown` mapping all NodeKind types `rustnote-core/src/parser/semantic_tree.rs`
- [ ] T040 [P] [US2] Add roundtrip tests for all GFM constructs (tables, task lists, nested lists) `rustnote-core/tests/parser_roundtrip.rs`
- [ ] T041 [P] [US2] Verify roundtrip: parse → serialize → parse produces identical AST `rustnote-core/tests/parser_roundtrip.rs`

### 2.2 Semantic Tree Serializer

- [ ] T042 [US2] Implement serializer to convert semantic tree back to Markdown `rustnote-core/src/parser/serializer.rs`
- [ ] T043 [US2] Implement `Serializer::to_markdown` for all NodeKind types `rustnote-core/src/parser/serializer.rs`
- [ ] T044 [US2] Verify serialization roundtrip fidelity `rustnote-core/tests/parser_roundtrip.rs`

### 2.3 Editor Commands

- [ ] T045 [US3] Define `EditorCommand` enum (InsertText, DeleteBackward, etc.) `rustnote-core/src/core/editor.rs`
- [ ] T046 [US3] Implement `EditorState::apply_command` with cursor position updates `rustnote-core/src/core/editor.rs`
- [ ] T047 [P] [US3] Implement `InsertText` command `rustnote-core/src/core/editor.rs`
- [ ] T048 [P] [US3] Implement `DeleteBackward` command `rustnote-core/src/core/editor.rs`
- [ ] T049 [P] [US3] Implement `DeleteForward` command `rustnote-core/src/core/editor.rs`
- [ ] T050 [P] [US3] Implement `InsertNewline` command `rustnote-core/src/core/editor.rs`

### 2.4 Smart Editing Behaviors

- [ ] T051 [US3] Implement smart Enter (continue list, exit on empty) `rustnote-core/src/core/editor.rs`
- [ ] T052 [US3] Implement smart Backspace (exit quote, unindent) `rustnote-core/src/core/editor.rs`
- [ ] T053 [US3] Implement task list checkbox toggle `- [ ]` ↔ `- [x]` `rustnote-core/src/core/editor.rs`
- [ ] T054 [US3] Implement blockquote entry/exit with `>` prefix behavior `rustnote-core/src/core/editor.rs`
- [ ] T055 [US3] Implement code fence entry with language tag ``` `rustnote-core/src/core/editor.rs`

### 2.5 Undo/Redo

- [ ] T056 [US4] Implement Command pattern with inverse operations `rustnote-core/src/core/editor.rs`
- [ ] T057 [US4] Implement `EditorState::undo` with undo stack `rustnote-core/src/core/editor.rs`
- [ ] T058 [US4] Implement `EditorState::redo` with redo stack `rustnote-core/src/core/editor.rs`
- [ ] T059 [US4] Wire undo/redo Tauri commands `src-tauri/src/commands.rs`
- [ ] T060 [US4] Verify undo/redo works for all operations `rustnote-core/tests/editor_commands.rs`

### 2.6 Advanced Rendering

- [ ] T061 [P] [US2] Implement table rendering with cell alignment `src-ui/components/editor/Renderer.ts`
- [ ] T062 [P] [US2] Implement image rendering with relative path display `src-ui/components/editor/Renderer.ts`
- [ ] T063 [P] [US2] Implement strikethrough, hard/soft breaks rendering `src-ui/components/editor/Renderer.ts`

### 2.7 Phase 2 Milestone Verification

- [ ] T064 [US2] **Milestone**: All markdown constructs render correctly - integration test passes
- [ ] T065 [US3] **Milestone**: Structural editing works (lists, quotes, code blocks) - manual test passes
- [ ] T066 [US4] **Milestone**: Undo/redo works for all ops - unit tests pass

---

## Phase 3: File Operations (Week 5)

> **Blocker**: Phase 2 complete (parser and semantic tree needed)

### 3.1 Workspace Model

- [ ] T067 [US5] Implement `Workspace::open` with `walkdir` for file tree `rustnote-core/src/core/workspace.rs`
- [ ] T068 [US5] Implement `FileNode` struct with path, name, is_dir, children `rustnote-core/src/core/workspace.rs`
- [ ] T069 [P] [US5] Implement `workspace_list_files` Tauri command `src-tauri/src/commands.rs`
- [ ] T070 [P] [US5] Implement `workspace_open` Tauri command `src-tauri/src/commands.rs`

### 3.2 File CRUD Operations

- [ ] T071 [US5] Implement `Workspace::create_file` `rustnote-core/src/core/workspace.rs`
- [ ] T072 [US5] Implement `Workspace::rename_file` `rustnote-core/src/core/workspace.rs`
- [ ] T073 [US5] Implement `Workspace::delete_file` `rustnote-core/src/core/workspace.rs`
- [ ] T074 [P] [US5] Implement workspace CRUD Tauri commands `src-tauri/src/commands.rs`
- [ ] T075 [P] [US5] Wire workspace CRUD to frontend file tree UI `src-ui/components/sidebar/FileTree.ts`

### 3.3 Auto-Save

- [ ] T076 [US6] Implement auto-save with configurable debounce `rustnote-core/src/services/document_service.rs`
- [ ] T077 [US6] Implement dirty state indicator in UI `src-ui/components/toolbar/Toolbar.ts`
- [ ] T078 [US6] Wire auto-save settings to debounce implementation `rustnote-core/src/core/settings.rs`

### 3.4 External Change Detection

- [ ] T079 [US7] Implement file watching with `notify` crate `rustnote-core/src/services/document_service.rs`
- [ ] T080 [US7] Implement reload/preserve/defer UI for external changes `src-ui/components/common/Modal.ts`
- [ ] T081 [US7] Implement `doc_reload` to detect and handle external modifications `src-tauri/src/commands.rs`

### 3.5 Recovery System

- [ ] T082 [US6] Implement `RecoverySnapshot` with timestamp and cursor position `rustnote-core/src/core/recovery.rs`
- [ ] T083 [US6] Implement `RecoverySnapshot::save` to app data directory `rustnote-core/src/core/recovery.rs`
- [ ] T084 [US6] Implement `RecoverySnapshot::list` to enumerate snapshots `rustnote-core/src/core/recovery.rs`
- [ ] T085 [US6] Implement `RecoverySnapshot::restore` to recover content and cursor `rustnote-core/src/core/recovery.rs`
- [ ] T086 [P] [US6] Implement recovery Tauri commands (list, restore, delete) `src-tauri/src/commands.rs`
- [ ] T087 [P] [US6] Implement periodic snapshot saving (time-based or change-count-based) `rustnote-core/src/services/recovery_service.rs`
- [ ] T088 [US6] Wire recovery UI to show available snapshots `src-ui/components/toolbar/Toolbar.ts`

### 3.6 Recent Files

- [ ] T089 [US5] Implement recent files persistence `rustnote-core/src/core/settings.rs`
- [ ] T090 [US5] Display recent files in UI `src-ui/components/sidebar/FileTree.ts`

### 3.7 Phase 3 Milestone Verification

- [ ] T091 [US5] **Milestone**: Folder opens as workspace with file tree - integration test passes
- [ ] T092 [US6] **Milestone**: Auto-save triggers correctly, snapshots saved - integration test passes
- [ ] T093 [US7] **Milestone**: External changes detected and handled - manual test passes

---

## Phase 4: Display (Week 6)

> **Blocker**: Phase 3 complete (workspace, settings available)

### 4.1 Theme System

- [ ] T094 [US8] Implement CSS variable system for themes `src-ui/styles/main.css`
- [ ] T095 [US8] Implement light theme CSS `src-ui/styles/themes/light.css`
- [ ] T096 [US8] Implement dark theme CSS `src-ui/styles/themes/dark.css`
- [ ] T097 [US8] Implement theme switching via Settings `rustnote-core/src/core/settings.rs`
- [ ] T098 [US8] Wire theme to Tauri settings commands `src-tauri/src/commands.rs`

### 4.2 Focus Mode

- [ ] T099 [US9] Implement focus mode (dim non-current paragraphs) `src-ui/styles/editor.css`
- [ ] T100 [US9] Implement focus mode toggle in settings `rustnote-core/src/core/settings.rs`
- [ ] T101 [US9] Wire focus mode to UI `src-ui/components/editor/EditorCanvas.ts`

### 4.3 Typewriter Mode

- [ ] T102 [US9] Implement typewriter mode (center active line) `src-ui/components/editor/EditorCanvas.ts`
- [ ] T103 [US9] Implement typewriter mode toggle in settings `rustnote-core/src/core/settings.rs`

### 4.4 Typography Settings

- [ ] T104 [US10] Implement content width setting with slider `rustnote-core/src/core/settings.rs`
- [ ] T105 [US10] Implement font size setting `rustnote-core/src/core/settings.rs`
- [ ] T106 [US10] Implement line height setting `rustnote-core/src/core/settings.rs`
- [ ] T107 [P] [US10] Apply typography settings to editor CSS `src-ui/styles/editor.css`

### 4.5 Visual Polish

- [ ] T108 [P] [US10] Implement heading level visual differentiation (H1-H6 sizing) `src-ui/styles/editor.css`
- [ ] T109 [P] [US10] Implement link visual styling (underlines, colors) `src-ui/styles/editor.css`
- [ ] T110 [P] [US8] Implement smooth theme transition animations `src-ui/styles/themes/light.css`

### 4.6 Phase 4 Milestone Verification

- [ ] T111 [US8] **Milestone**: Light and dark themes work correctly - manual test passes
- [ ] T112 [US9] **Milestone**: Focus mode and typewriter mode reduce distraction - manual test passes
- [ ] T113 [US10] **Milestone**: Typography adjustable and persists - manual test passes

---

## Phase 5: Export (Week 7)

> **Blocker**: Phase 4 complete (themes and display settings ready)

### 5.1 HTML Export

- [ ] T114 [US11] Implement HTML export with pulldown-cmark HTML renderer `rustnote-core/src/services/export_service.rs`
- [ ] T115 [US11] Implement standalone HTML mode (inline CSS) `rustnote-core/src/services/export_service.rs`
- [ ] T116 [US11] Implement linked-assets HTML mode (separate CSS file) `rustnote-core/src/services/export_service.rs`
- [ ] T117 [P] [US11] Wire HTML export to Tauri command `src-tauri/src/commands.rs`

### 5.2 PDF Export

- [ ] T118 [US12] Implement PDF export using printpdf `rustnote-core/src/services/export_service.rs`
- [ ] T119 [US12] Implement PDF page size settings (A4, Letter, etc.) `rustnote-core/src/core/settings.rs`
- [ ] T120 [US12] Implement PDF margin settings `rustnote-core/src/core/settings.rs`
- [ ] T121 [P] [US12] Wire PDF export to Tauri command `src-tauri/src/commands.rs`

### 5.3 Export Dialog UI

- [ ] T122 [US11] Implement export dialog UI component `src-ui/components/toolbar/ExportDialog.ts`
- [ ] T123 [US11] Implement HTML/PDF format selection in dialog `src-ui/components/toolbar/ExportDialog.ts`
- [ ] T124 [US11] Implement export options (page size, margins) in dialog `src-ui/components/toolbar/ExportDialog.ts`
- [ ] T125 [US12] Wire export dialog to export Tauri commands `src-ui/services/tauri.ts`

### 5.4 Export Verification

- [ ] T126 [US11] Test HTML export with complex documents (tables, task lists) `rustnote-core/tests/export_tests.rs`
- [ ] T127 [US12] Test PDF export with various page sizes and margins `rustnote-core/tests/export_tests.rs`
- [ ] T128 [US11] Verify export fidelity for all GFM constructs `rustnote-core/tests/export_tests.rs`

### 5.5 Phase 5 Milestone Verification

- [ ] T129 [US11] **Milestone**: HTML export works in both modes - integration test passes
- [ ] T130 [US12] **Milestone**: PDF export works with settings - integration test passes

---

## Phase 6: Polish (Week 8)

> **Blocker**: Phase 5 complete (export functionality available)

### 6.1 Find and Replace

- [ ] T131 [US13] Implement find/replace UI component `src-ui/components/editor/FindReplaceDialog.ts`
- [ ] T132 [US13] Implement find Tauri command `src-tauri/src/commands.rs`
- [ ] T133 [US13] Implement replace Tauri command `src-tauri/src/commands.rs`
- [ ] T134 [US13] Wire find/replace to editor `src-ui/services/keyboard.ts`

### 6.2 Outline Panel

- [ ] T135 [US14] Implement outline panel UI `src-ui/components/outline/OutlinePanel.ts`
- [ ] T136 [US14] Implement heading extraction from document `rustnote-core/src/services/document_service.rs`
- [ ] T137 [US14] Implement `doc_get_outline` Tauri command `src-tauri/src/commands.rs`
- [ ] T138 [US14] Implement outline navigation (click to jump) `src-ui/components/outline/OutlinePanel.ts`

### 6.3 Settings Persistence

- [ ] T139 [US15] Implement settings load on app start `rustnote-core/src/services/settings_service.rs`
- [ ] T140 [US15] Implement settings save on change `rustnote-core/src/services/settings_service.rs`
- [ ] T141 [US15] Wire settings to Tauri commands `src-tauri/src/commands.rs`
- [ ] T142 [US15] Implement settings UI panel `src-ui/components/toolbar/SettingsPanel.ts`

### 6.4 Keyboard Shortcuts

- [ ] T143 [US16] Implement keyboard shortcut handler `src-ui/services/keyboard.ts`
- [ ] T144 [US16] Implement standard shortcuts (Ctrl+S save, Ctrl+Z undo, etc.) `src-ui/services/keyboard.ts`
- [ ] T145 [US16] Implement custom shortcut binding in settings `rustnote-core/src/core/settings.rs`

### 6.5 Paste Handling

- [ ] T146 [US13] Implement paste handling with best-effort Markdown conversion `src-ui/services/clipboard.ts`
- [ ] T147 [US13] Implement plain text paste fallback `src-ui/services/clipboard.ts`

### 6.6 Image Handling

- [ ] T148 [US17] Implement image picker dialog `src-ui/components/editor/ImagePicker.ts`
- [ ] T149 [US17] Implement drag-and-drop image insertion `src-ui/components/editor/EditorCanvas.ts`
- [ ] T150 [US17] Implement clipboard image paste `src-ui/services/clipboard.ts`
- [ ] T151 [US17] Implement image insert with relative path `rustnote-core/src/core/editor.rs`

### 6.7 Frontmatter Support

- [ ] T152 [US18] Implement YAML frontmatter parsing `rustnote-core/src/parser/pulldown_adapter.rs`
- [ ] T153 [US18] Implement TOML frontmatter parsing `rustnote-core/src/parser/pulldown_adapter.rs`
- [ ] T154 [US18] Implement frontmatter serialization in serializer `rustnote-core/src/parser/serializer.rs`
- [ ] T155 [US18] Verify frontmatter roundtrip `rustnote-core/tests/parser_roundtrip.rs`

### 6.8 Performance Optimization

- [ ] T156 [US2] Implement lazy parsing for large documents `rustnote-core/src/parser/pulldown_adapter.rs`
- [ ] T157 [US2] Implement viewport-based rendering in editor `src-ui/components/editor/EditorCanvas.ts`

### 6.9 Phase 6 Milestone Verification

- [ ] T158 [US13] **Milestone**: Find/replace works with all options - manual test passes
- [ ] T159 [US14] **Milestone**: Outline panel shows headings and navigates - manual test passes
- [ ] T160 [US15] **Milestone**: Settings persist across app restarts - integration test passes
- [ ] T161 [US16] **Milestone**: All keyboard shortcuts work correctly - manual test passes
- [ ] T162 [US17] **Milestone**: Image handling (pick, drag, paste) works - manual test passes
- [ ] T163 [US18] **Milestone**: Frontmatter roundtrips correctly - unit test passes

---

## Phase 7: Hardening (Week 9)

> **Blocker**: Phase 6 complete (all features implemented)

### 7.1 Testing

- [ ] T164 [US1] Write integration tests for all user stories `rustnote-core/tests/`
- [ ] T165 [US2] Run roundtrip tests for all Markdown constructs `rustnote-core/tests/parser_roundtrip.rs`
- [ ] T166 [US3] Write editor command tests for all operations `rustnote-core/tests/editor_commands.rs`
- [ ] T167 [US6] Write crash recovery tests `rustnote-core/tests/recovery.rs`
- [ ] T168 [US11] Write export fidelity tests `rustnote-core/tests/export_tests.rs`

### 7.2 Cross-Platform Testing

- [ ] T169 [US1] Test on macOS (verify icons, paths, keybindings) `src-tauri/`
- [ ] T170 [US1] Test on Windows (verify paths, line endings) `src-tauri/`
- [ ] T171 [US1] Test on Linux (verify paths, display issues) `src-tauri/`
- [ ] T172 [US5] Verify file operations work on all platforms `rustnote-core/src/core/workspace.rs`

### 7.3 Performance Profiling

- [ ] T173 [US2] Profile large document parsing (>5MB) `rustnote-core/src/parser/`
- [ ] T174 [US2] Profile editor keystroke latency `src-ui/components/editor/`
- [ ] T175 [US1] Optimize Buffer operations if needed `rustnote-core/src/core/buffer.rs`

### 7.4 Security Review

- [ ] T176 [US5] Review file I/O for path traversal vulnerabilities `rustnote-core/src/core/workspace.rs`
- [ ] T177 [US5] Review file operations for symlink attacks `rustnote-core/src/core/workspace.rs`
- [ ] T178 [US11] Review export for injection vulnerabilities `rustnote-core/src/services/export_service.rs`

### 7.5 Crash Testing

- [ ] T179 [US6] Simulate crash during save and verify recovery `rustnote-core/tests/recovery.rs`
- [ ] T180 [US6] Test recovery from corrupted snapshots `rustnote-core/src/core/recovery.rs`
- [ ] T181 [US7] Test external change during edit session `rustnote-core/src/services/document_service.rs`

### 7.6 Documentation

- [ ] T182 [US1] Update README with installation instructions `README.md`
- [ ] T183 [US1] Write CONTRIBUTING.md for developer setup `CONTRIBUTING.md`
- [ ] T184 [US1] Document keyboard shortcuts `src-ui/services/keyboard.ts`
- [ ] T185 [US1] Document export formats and options `src-ui/components/toolbar/ExportDialog.ts`

### 7.7 Final Bug Fixes

- [ ] T186 [US1] Triage and fix all known bugs from testing `*`
- [ ] T187 [US2] Fix any Markdown rendering issues `src-ui/components/editor/Renderer.ts`
- [ ] T188 [US4] Fix any undo/redo edge cases `rustnote-core/src/core/editor.rs`

### 7.8 Phase 7 Milestone Verification

- [ ] T189 [US1] **Milestone**: Production-ready release - all tests pass
- [ ] T190 [US1] **Milestone**: Cross-platform verified on macOS, Windows, Linux
- [ ] T191 [US1] **Milestone**: Performance targets met (<16ms keystroke latency)
- [ ] T192 [US1] **Milestone**: Security review passed with no critical issues

---

## Cross-Cutting Concerns

These tasks span multiple phases and should be tracked in parallel with feature development.

### Error Handling

- [ ] T193 [P] [US1] Implement `thiserror` error types for all modules `rustnote-core/src/error.rs`
- [ ] T194 [P] [US1] Implement `anyhow` for error propagation in services `rustnote-core/src/error.rs`
- [ ] T195 [P] [US1] Implement error UI for user-facing errors `src-ui/components/common/Modal.ts`

### Logging

- [ ] T196 [P] [US1] Configure `env_logger` for Rust core `rustnote-core/src/`
- [ ] T197 [P] [US1] Add logging to all Tauri commands `src-tauri/src/commands.rs`
- [ ] T198 [P] [US1] Implement frontend logger `src-ui/utils/logger.ts`

### Code Organization

- [ ] T199 [P] [US1] Ensure module exports are clean in `rustnote-core/src/lib.rs`
- [ ] T200 [P] [US1] Ensure no circular dependencies between crates `rustnote-core/`, `src-tauri/`

---

## Task Dependencies Summary

```
Phase 0 (Research) ─────────────────────────────────────────────────┐
     │                                                                │
     ├─ T001, T002, T003, T004, T005, T006 (parallel)               │
     │                                                                │
     ▼                                                                │
Phase 1 (Foundation) ───────────────────────────────────────────────┤
     │                                                                │
     ├─ T007-T010 (Setup)                                            │
     ├─ T011-T015 (Core Types) ──────────────────────────────────────┤
     ├─ T016-T020 (Buffer) ──────────────────────────────────────────┤
     ├─ T021-T024 (Tauri Commands) ──────────────────────────────────┤
     ├─ T025-T028 (Frontend Canvas) ─────────────────────────────────┤
     ├─ T029-T032 (Live Rendering) ──────────────────────────────────┤
     ├─ T033-T034 (Save) ────────────────────────────────────────────┤
     │                                                                │
     ▼                                                                │
Phase 2 (Core Editing) ──────────────────────────────────────────────┤
     │                                                                │
     ├─ T036-T041 (Parser) ───────────────────────────────────────────┤
     ├─ T042-T044 (Serializer) ──────────────────────────────────────┤
     ├─ T045-T050 (Editor Commands) ─────────────────────────────────┤
     ├─ T051-T055 (Smart Editing) ───────────────────────────────────┤
     ├─ T056-T060 (Undo/Redo) ───────────────────────────────────────┤
     ├─ T061-T063 (Advanced Rendering)                              │
     │                                                                │
     ▼                                                                │
Phase 3 (File Operations) ───────────────────────────────────────────┤
     │                                                                │
     ├─ T067-T070 (Workspace Model) ─────────────────────────────────┤
     ├─ T071-T075 (File CRUD) ───────────────────────────────────────┤
     ├─ T076-T078 (Auto-Save) ───────────────────────────────────────┤
     ├─ T079-T081 (External Change Detection) ──────────────────────┤
     ├─ T082-T090 (Recovery System) ─────────────────────────────────┤
     │                                                                │
     ▼                                                                │
Phase 4 (Display) ────────────────────────────────────────────────────┤
     │                                                                │
     ├─ T094-T098 (Theme System) ────────────────────────────────────┤
     ├─ T099-T103 (Focus/Typewriter Modes) ──────────────────────────┤
     ├─ T104-T107 (Typography Settings) ─────────────────────────────┤
     ├─ T108-T110 (Visual Polish)                                    │
     │                                                                │
     ▼                                                                │
Phase 5 (Export) ────────────────────────────────────────────────────┤
     │                                                                │
     ├─ T114-T117 (HTML Export) ──────────────────────────────────────┤
     ├─ T118-T121 (PDF Export) ───────────────────────────────────────┤
     ├─ T122-T125 (Export Dialog UI) ───────────────────────────────┤
     ├─ T126-T128 (Export Verification)                             │
     │                                                                │
     ▼                                                                │
Phase 6 (Polish) ────────────────────────────────────────────────────┤
     │                                                                │
     ├─ T131-T134 (Find/Replace) ────────────────────────────────────┤
     ├─ T135-T138 (Outline Panel) ───────────────────────────────────┤
     ├─ T139-T142 (Settings Persistence) ───────────────────────────┤
     ├─ T143-T145 (Keyboard Shortcuts) ─────────────────────────────┤
     ├─ T146-T147 (Paste Handling) ─────────────────────────────────┤
     ├─ T148-T151 (Image Handling) ──────────────────────────────────┤
     ├─ T152-T155 (Frontmatter Support) ─────────────────────────────┤
     ├─ T156-T157 (Performance)                                      │
     │                                                                │
     ▼                                                                │
Phase 7 (Hardening)                                                   │
     │                                                                │
     ├─ T164-T168 (Testing)                                          │
     ├─ T169-T172 (Cross-Platform)                                   │
     ├─ T173-T175 (Performance Profiling)                           │
     ├─ T176-T178 (Security Review)                                  │
     ├─ T179-T181 (Crash Testing)                                   │
     ├─ T182-T185 (Documentation)                                   │
     └─ T186-T188 (Final Bug Fixes) ─────────────────────────────────┘
```

---

## Parallelization Matrix

| Task IDs | Reason for Parallelization |
|----------|---------------------------|
| T001-T006 | Independent research topics, no shared state |
| T011-T015 | Independent type definitions, compile independently |
| T021-T024 | Independent command implementations |
| T029-T032 | Independent rendering components |
| T047-T050 | Independent editor commands |
| T061-T063 | Independent rendering features |
| T069-T070 | Independent workspace commands |
| T074-T075 | Independent frontend wiring |
| T086-T087 | Independent recovery features |
| T094-T098 | Independent theme components |
| T104-T107 | Independent typography settings |
| T108-T110 | Independent visual polish |
| T116-T117 | Independent export modes |
| T120-T121 | Independent PDF settings |
| T122-T124 | Independent dialog components |
| T193-T198 | Independent cross-cutting concerns |

---

## Task Statistics

| Phase | Total Tasks | Parallelizable | Sequential |
|-------|-------------|----------------|------------|
| Phase 0 | 6 | 6 | 0 |
| Phase 1 | 29 | 15 | 14 |
| Phase 2 | 31 | 18 | 13 |
| Phase 3 | 25 | 12 | 13 |
| Phase 4 | 17 | 10 | 7 |
| Phase 5 | 17 | 9 | 8 |
| Phase 6 | 27 | 15 | 12 |
| Phase 7 | 29 | 12 | 17 |
| Cross-Cutting | 8 | 8 | 0 |
| **Total** | **189** | **105 (56%)** | **84 (44%)** |

---

*Generated: 2026-04-11*
*Source: iteration-5/plan.md*
