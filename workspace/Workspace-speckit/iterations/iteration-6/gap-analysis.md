# RustNote Gap Analysis Report

**Project:** Typora-like Markdown Editor in Rust  
**Current Iteration:** Iteration 6  
**Analysis Date:** April 14, 2026  
**PRD Version:** 3.1 (docs/PRD/)

---

## 1. Executive Summary

The RustNote MVP implementation is approximately **85% complete** by functional requirements. Significant architectural improvements have been made in Iteration 6, including the implementation of rusqlite-based settings persistence, tree-sitter incremental parsing, and ropey-based buffer. However, some P0 issues remain, and several P1/P2 items need completion.

### Implementation Progress by Category

| Category | Status | Notes |
|----------|--------|-------|
| File Operations (FR-001 to FR-007) | ✅ 95% | All core operations implemented, atomic writes working |
| Core Editor (FR-008 to FR-022) | ✅ 80% | Smart transforms now implemented in Rust, cursor mapping tests added |
| Markdown Support (FR-020 to FR-022) | ✅ 90% | CommonMark + GFM via comrak, tree-sitter incremental parsing working |
| Workspace & Navigation (FR-023 to FR-026) | ✅ 85% | File tree functional, outline panel working |
| Display & Themes (FR-027 to FR-030) | ✅ 85% | Light/dark themes, focus/typewriter modes have tests |
| Export (FR-031 to FR-033) | ⚠️ 75% | HTML/PDF working, highlighting in progress |
| Preferences (FR-034) | ✅ 90% | Settings now uses rusqlite, persistence working |
| Recovery & Safety | ✅ 90% | Snapshots functional, autosave service implemented |
| Architecture (PRD-10) | ✅ 80% | Services module refactored, buffer/parser/tree-sitter integrated |

### Progress Since Iteration 5

| Area | Iteration 5 | Iteration 6 | Change |
|------|------------|-------------|--------|
| Settings Persistence | ❌ JSON file | ✅ rusqlite | **FIXED** |
| TransformEngine | ❌ Incomplete | ✅ Complete | **FIXED** |
| Tree-sitter Parser | ❌ Not implemented | ✅ Implemented | **FIXED** |
| Ropey Buffer | ❌ Not using ropey | ✅ Implemented | **FIXED** |
| Undo/Redo Tests | ❌ Missing | ✅ Added | **FIXED** |
| Cursor Mapping Tests | ❌ Missing | ✅ Added | **FIXED** |
| Focus Mode Tests | ❌ Missing | ✅ Added | **FIXED** |

---

## 2. Gap Analysis Table

### P0 - Blocking Issues (Must Fix)

| Gap | Severity | Module | Description | Fix Suggestion |
|-----|----------|--------|-------------|----------------|
| G-001 | **P0** | Editor | Cursor mapping bidirectional conversion (DOM→source) not fully implemented - `build_cursor_mapping()` creates source-to-DOM mapping but `CursorMapping::dom_to_source()` may have edge cases | Complete bidirectional cursor mapping implementation with comprehensive tests |
| G-002 | **P0** | Export | PDF export `printpdf` implementation may not properly render complex Markdown (tables, code blocks with syntax highlighting) | Verify PDF output quality and integrate proper HTML-to-PDF pipeline |

### P1 - High Priority Issues

| Gap | Severity | Module | Description | Fix Suggestion |
|-----|----------|--------|-------------|----------------|
| G-003 | **P1** | Editor | `Transform::Wrap` with selection - the transform engine supports it but integration with TipTap editor may be incomplete | Verify wrap transform works end-to-end with text selection |
| G-004 | **P1** | Export | HTML export doesn't support "linked-assets mode" as per FR-031 | Add option for linked vs inline asset export |
| G-005 | **P1** | Image | Image relative path handling for subdirectory documents may have edge cases | Add tests for relative path calculation when document is in subdirectory |
| G-006 | **P1** | Parser | tree-sitter parser uses `tree-sitter-markdown` but GFM tables and task lists may need special handling | Verify tree-sitter correctly parses GFM extensions |
| G-007 | **P1** | Settings | `Settings` model may have nested `editor` struct vs flat structure per PRD-09 | Verify Settings schema matches PRD specification |
| G-008 | **P1** | Frontend | `Editor.jsx` and `TipTapEditor.jsx` - unclear distinction, potential duplication | Consolidate or clearly document the relationship |

### P2 - Medium Priority Issues

| Gap | Severity | Module | Description | Fix Suggestion |
|-----|----------|--------|-------------|----------------|
| G-009 | **P2** | Editor | Table editing uses TipTap default behavior - constrained but safe model per PRD note | Document table editing constraints; add tests for data integrity |
| G-010 | **P2** | Display | Focus mode implementation needs verification - tests exist but actual visual dimming behavior in browser | Verify CSS-based focus mode properly dims non-current paragraphs |
| G-011 | **P2** | Display | Typewriter mode scroll behavior may not keep cursor at vertical center during navigation | Verify scroll behavior and fix if needed |
| G-012 | **P2** | Paste | Paste behavior converts to Markdown on best-effort but doesn't handle all rich text paste cases | Improve paste handling for common rich text formats |
| G-013 | **P2** | Frontend | Preferences UI is limited - no dedicated settings panel beyond toolbar buttons | Consider adding a proper Preferences/Settings UI |
| G-014 | **P2** | Services | Architecture has services but `DocumentService`, `EditorService`, `WorkspaceService`, `ExportService`, `RecoveryService` interfaces may need verification | Verify service trait implementations match PRD-10 specification |
| G-015 | **P2** | Autosave | Autosave uses frontend timer + Rust service - verify crash recovery still works if app crashes before timer fires | Consider Rust-side autosave with debounce as backup |

---

## 3. Technical Debt

| ID | Category | Description | Impact | Status |
|----|----------|-------------|--------|--------|
| TD-001 | Architecture | `semantic/position.rs` - `CursorMapping` bidirectional mapping needs full edge case coverage | Editor UX | In Progress |
| TD-002 | Code | `editor/commands.rs` - `Command` enum exists but editor operations may not use it consistently | Maintainability | Needs Review |
| TD-003 | Code | Frontend has both `Editor.jsx` and `TipTapEditor.jsx` - potential duplication | Maintainability | Needs Consolidation |
| TD-004 | Testing | Visual regression tests exist but baseline may not be current | Quality | Needs Update |
| TD-005 | Testing | No integration tests for file watcher + editor interaction | Reliability | Needs Coverage |
| TD-006 | Performance | Large document handling with tree-sitter - incremental parsing works but may need tuning | Performance | Needs Benchmarking |

---

## 4. Feature Completeness Checklist

### MVP Scope Features (from PRD-03)

| Feature | FR | Status | Notes |
|---------|-----|--------|-------|
| Desktop app (macOS/Windows/Linux) | - | ✅ | Tauri v2 cross-platform |
| Open, edit, save `.md` files | FR-001, FR-002, FR-004 | ✅ | Working |
| Open folder as workspace | FR-003 | ✅ | Working with sidebar |
| Single-pane live Markdown editing | FR-008 | ✅ | Working with TipTap |
| Rendered: headings, emphasis, links, images, lists, task lists, code fences, quotes, HR, tables | FR-020 | ✅ | Working, tables use TipTap default |
| Smart editing: lists, quotes, structure | FR-012, FR-014 | ✅ | TransformEngine implements all smart behaviors |
| In-document search/replace | FR-025 | ✅ | Working |
| Recent files/folders | FR-024 | ✅ | Settings persists recent files |
| Theme support (light/dark) | FR-027 | ✅ | Working |
| Focus mode | FR-028 | ✅ | Tests added, CSS implementation exists |
| Typewriter mode | FR-029 | ⚠️ | Implementation exists, needs verification |
| HTML export | FR-031 | ✅ | Working |
| PDF export | FR-032 | ⚠️ | Working but may need quality improvement |
| Auto-save | FR-005 | ✅ | Rust AutosaveService + frontend timer |
| Crash recovery | FR-006 | ✅ | Working |
| External file change detection | FR-007 | ✅ | Working |
| Code fence syntax highlighting | FR-015 | ✅ | Via SyntaxHighlighter |
| Relative asset paths | FR-017 | ⚠️ | Needs edge case testing |
| Outline/TOC panel | FR-026 | ✅ | Working |

---

## 5. API Completeness Check

### API Contracts (from PRD-09)

| Command | Status | Implementation |
|---------|--------|----------------|
| `create_document` | ✅ | `commands/document.rs::create_document` |
| `open_document` | ✅ | `commands/document.rs::open_document` |
| `save_document` | ✅ | `commands/document.rs::save_document` |
| `read_document_content` | ✅ | `commands/document.rs::read_document_content` |
| `write_document_content` | ✅ | `commands/document.rs::write_document_content` |
| `list_workspace` | ✅ | `commands/workspace.rs::list_workspace` |
| `create_file` | ✅ | `commands/file_tree.rs::create_file` |
| `create_folder` | ✅ | `commands/file_tree.rs::create_folder` |
| `rename_item` | ✅ | `commands/file_tree.rs::rename_item` |
| `delete_item` | ✅ | `commands/file_tree.rs::delete_item` |
| `read_settings` | ✅ | `commands/settings.rs::read_settings` |
| `write_settings` | ✅ | `commands/settings.rs::write_settings` |
| `export_to_html` | ✅ | `commands/export.rs::export_to_html` |
| `export_to_pdf` | ✅ | `commands/export.rs::export_to_pdf` |
| `export_to_pdf_native` | ✅ | `commands/export.rs::export_to_pdf_native` |
| `get_print_html` | ✅ | `commands/export.rs::get_print_html` |
| `render_markdown` | ✅ | `commands/render.rs::render_markdown` |
| `parse_markdown_ast` | ✅ | `commands/render.rs::parse_markdown_ast` |
| `serialize_markdown` | ✅ | `commands/render.rs::serialize_markdown` |
| `get_markdown_info` | ✅ | `commands/render.rs::get_markdown_info` |
| `editor_apply_transform` | ✅ | `commands/editor.rs::editor_apply_transform` - **FIXED** |
| `editor_search` | ✅ | `commands/editor.rs::editor_search` |
| `editor_find_next` | ✅ | `commands/editor.rs::editor_find_next` |
| `editor_find_previous` | ✅ | `commands/editor.rs::editor_find_previous` |
| `editor_replace_match` | ✅ | `commands/editor.rs::editor_replace_match` |
| `editor_replace_all` | ✅ | `commands/editor.rs::editor_replace_all` |
| `update_source` | ✅ | `commands/render.rs::update_source` |
| `render_for_editor` | ✅ | `commands/render.rs::render_for_editor` |
| `render_for_editor_with_highlighting` | ✅ | `commands/render.rs::render_for_editor_with_highlighting` |
| `highlight_code_block` | ✅ | `commands/render.rs::highlight_code_block` |
| `get_highlighted_code_html` | ✅ | `commands/render.rs::get_highlighted_code_html` |
| `prehighlight_markdown` | ✅ | `commands/render.rs::prehighlight_markdown` |
| `insert_image` | ✅ | `commands/image.rs::insert_image` |
| `image_markdown_from_path` | ✅ | `commands/image.rs::image_markdown_from_path` |
| `save_image_from_base64_cmd` | ✅ | `commands/image.rs::save_image_from_base64_cmd` |
| `save_recovery_snapshot` | ✅ | `commands/recovery.rs::save_recovery_snapshot` |
| `list_recovery_snapshots` | ✅ | `commands/recovery.rs::list_recovery_snapshots` |
| `restore_recovery_snapshot` | ✅ | `commands/recovery.rs::restore_recovery_snapshot` |
| `delete_recovery_snapshot` | ✅ | `commands/recovery.rs::delete_recovery_snapshot` |
| `cleanup_old_snapshots` | ✅ | `commands/recovery.rs::cleanup_old_snapshots` |
| `watch_file` | ✅ | `commands/file_watcher.rs::watch_file` |
| `unwatch_file` | ✅ | `commands/file_watcher.rs::unwatch_file` |
| `poll_file_changes` | ✅ | `commands/file_watcher.rs::poll_file_changes` |
| `check_external_change` | ✅ | `commands/file_watcher.rs::check_external_change` |
| `update_watched_file_state` | ✅ | `commands/file_watcher.rs::update_watched_file_state` |
| `open_external_url` | ✅ | `commands/mod.rs::open_external_url` |

---

## 6. Data Model Analysis

### Implemented Entities

| Entity | PRD Definition | Implementation | Status |
|--------|---------------|----------------|--------|
| Document | PRD-09 DocumentResult | `model/document.rs::Document` | ✅ |
| Heading | PRD-09 Heading | `model/document.rs::Heading` | ✅ |
| TransformType | PRD-09 TransformType | `editor/transforms.rs::Transform` | ✅ Complete |
| Settings | PRD-09 Settings | `model/settings.rs::Settings` + `services/settings.rs` | ⚠️ Nested structure |
| Workspace | Not in PRD contracts | `model/workspace.rs::Workspace` | ✅ |
| FileEntry | Not in PRD contracts | `model/workspace.rs::FileEntry` | ✅ |
| RecoverySnapshot | Not in PRD contracts | `model/recovery.rs::RecoverySnapshot` | ✅ |

### Missing Type Definitions

| Type | Location | Description | Status |
|------|----------|-------------|--------|
| `ErrorCode` enum | Should be in `commands/mod.rs` | Missing from implementation | ⚠️ Not defined |
| `SaveResult` | Not defined in PRD-09 | `save_document` returns `()` not `SaveResult` | ⚠️ Not defined |
| `ExportResult` | Not defined in PRD-09 | `export_*` returns `()` not `ExportResult` | ⚠️ Not defined |
| `WorkspaceResult` | Not defined in PRD-09 | `list_workspace` returns `Workspace` directly | ⚠️ Not defined |

---

## 7. Frontend Completeness

### Implemented Components

| Component | File | Status |
|-----------|------|--------|
| App | `App.jsx` | ✅ Main layout with all panels |
| TipTapEditor | `TipTapEditor.jsx` | ✅ Core editor with cursor mapping |
| Editor | `Editor.jsx` | ⚠️ Legacy/alternative - needs consolidation |
| Toolbar | `Toolbar.jsx` | ✅ |
| Sidebar | `Sidebar.jsx` | ✅ File tree working |
| OutlinePanel | `OutlinePanel.jsx` | ✅ |
| SearchPanel | `SearchPanel.jsx` | ✅ |
| ExportModal | `ExportModal.jsx` | ✅ |
| ExternalChangeModal | `ExternalChangeModal.jsx` | ✅ |
| RecoveryModal | `RecoveryModal.jsx` | ✅ |
| DropZone | `DropZone.jsx` | ✅ |
| LinkPopover | `LinkPopover.jsx` | ✅ |
| CodeBlockHighlight | `CodeBlockHighlight.jsx` | ✅ |
| FrontmatterBlock | `FrontmatterBlock.jsx` | ✅ |

### Contexts

| Context | Status |
|---------|--------|
| DocumentContext | ✅ Working |
| SettingsContext | ✅ Working |
| SearchContext | ✅ Working |
| ToastContext | ✅ Working |

### Missing Components

| Component | Description | Priority |
|-----------|-------------|----------|
| Preferences UI | Settings UI beyond toolbar buttons | P2 |
| SourceEditor | CodeMirror 6 for source mode (per PRD-10 structure) | P2 |

---

## 8. Rust Crate Structure Analysis

### Current Structure vs PRD-10 Recommended

| PRD-10 Module | Current Implementation | Status |
|---------------|----------------------|--------|
| `buffer` (ropey) | `buffer/mod.rs` using ropey | ✅ **FIXED** |
| `parser` (tree-sitter + comrak) | `parser/mod.rs` + `parser/tree_sitter.rs` | ✅ **FIXED** |
| `editor-engine` | `editor/mod.rs` + `editor/transforms.rs` | ✅ |
| `serializer` | `semantic/ast.rs` | ✅ |
| `workspace` | `commands/workspace.rs` + `commands/file_tree.rs` | ✅ |
| `export` | `commands/export.rs` | ⚠️ |
| `theme` | CSS-based in `www/src/styles/` | ✅ |
| `settings` (rusqlite) | `services/settings.rs` using rusqlite | ✅ **FIXED** |
| `recovery` | `commands/recovery.rs` + `model/recovery.rs` | ✅ |
| `app-services` | `services/mod.rs` with trait definitions | ✅ |

---

## 9. Test Coverage Analysis

### Test Files Present

| Test File | Coverage |
|-----------|----------|
| `tests/buffer_settings_tests.rs` | Buffer and settings basic tests |
| `tests/editor_transforms.rs` | Transform operations |
| `tests/editor_undo_redo_tests.rs` | **NEW** Undo/redo fidelity |
| `tests/cursor_mapping_tests.rs` | **NEW** Bidirectional cursor mapping |
| `tests/focus_mode_tests.rs` | **NEW** Focus mode paragraph detection |
| `tests/integration_editor_tests.rs` | Editor integration |
| `tests/integration_file_tests.rs` | File operations |
| `tests/parser_tests.rs` | Markdown parsing |
| `tests/tree_sitter_parser_tests.rs` | **NEW** Incremental parsing |
| `tests/settings_persistence_tests.rs` | **NEW** rusqlite persistence |
| `tests/table_editing_tests.rs` | Table editing |
| `tests/image_path_tests.rs` | Image path handling |
| `tests/typewriter_mode_tests.rs` | **NEW** Typewriter mode |
| `tests/paste_handling_tests.rs` | Paste behavior |
| `tests/frontmatter_tests.rs` | Frontmatter parsing |
| `tests/pdf_export_tests.rs` | PDF export |
| `tests/html_export_tests.rs` | HTML export |
| `tests/autosave_tests.rs` | Autosave |
| `tests/service_interface_tests.rs` | Service interfaces |
| `benches/transforms.rs` | Transform benchmarks |
| `benches/serialization.rs` | Serialization benchmarks |
| `benches/parsing.rs` | Parsing benchmarks |

### Missing Test Coverage

| Area | Priority | Notes |
|------|----------|-------|
| Bidirectional cursor mapping edge cases | P0 | Tests exist but edge cases may remain |
| PDF export quality verification | P0 | Tests exist but visual verification needed |
| File watcher + editor interaction | P1 | No integration tests |
| External change detection | P1 | Tests exist but edge cases may remain |
| Wrap transform with selection | P1 | Unit tests exist, integration needed |

---

## 10. Recommendations

### Immediate Actions (P0)

1. **Complete Cursor Mapping**: Ensure `dom_to_source` conversion works for all edge cases
2. **Verify PDF Export**: Test PDF output quality with complex Markdown structures

### Short-term Actions (P1)

1. **Image Path Testing**: Add tests for relative path handling in subdirectory documents
2. **Wrap Transform Integration**: Verify wrap transform works end-to-end with TipTap selection
3. **HTML Export Linked-Assets Mode**: Implement linked vs inline asset export option
4. **Settings Schema Verification**: Verify Settings model matches PRD-09 flat structure
5. **Editor Consolidation**: Clarify relationship between `Editor.jsx` and `TipTapEditor.jsx`

### Medium-term Actions (P2)

1. **Preferences UI**: Consider adding dedicated settings panel
2. **Visual Regression Baseline**: Update visual regression baselines for current implementation
3. **Performance Benchmarking**: Run benchmarks for large document handling with tree-sitter
4. **Paste Handling Polish**: Improve rich text paste conversion for common formats

---

## 11. Files Analyzed

### Rust Backend (src-tauri/src/)

| Directory | Files | Status |
|-----------|-------|--------|
| commands/ | `mod.rs`, `document.rs`, `editor.rs`, `workspace.rs`, `file_tree.rs`, `settings.rs`, `export.rs`, `recovery.rs`, `render.rs`, `file_watcher.rs`, `image.rs`, `autosave.rs` | ✅ All present |
| services/ | `mod.rs`, `settings.rs`, `document.rs`, `editor.rs`, `file_watcher.rs`, `autosave.rs` | ✅ **FIXED** |
| model/ | `mod.rs`, `document.rs`, `settings.rs`, `workspace.rs`, `recovery.rs`, `export.rs`, `image.rs` | ✅ All present |
| editor/ | `mod.rs`, `transforms.rs`, `undo.rs`, `cursor.rs`, `selection.rs`, `search.rs`, `commands.rs` | ✅ All present |
| parser/ | `mod.rs`, `tree_sitter.rs`, `markdown.rs`, `syntax.rs` | ✅ **FIXED** |
| semantic/ | `mod.rs`, `ast.rs`, `position.rs`, `frontmatter.rs`, `transform.rs` | ✅ All present |
| renderer/ | `mod.rs`, `state.rs`, `blocks.rs`, `inline.rs` | ✅ All present |
| buffer/ | `mod.rs` | ✅ **FIXED** - now using ropey |

### Frontend (www/src/)

| Directory | Files | Status |
|-----------|-------|--------|
| components/ | `TipTapEditor.jsx`, `Editor.jsx`, `Toolbar.jsx`, `Sidebar.jsx`, `OutlinePanel.jsx`, `SearchPanel.jsx`, `ExportModal.jsx`, `ExternalChangeModal.jsx`, `RecoveryModal.jsx`, `DropZone.jsx`, `LinkPopover.jsx`, `CodeBlockHighlight.jsx`, `FrontmatterBlock.jsx`, `Toast.jsx` | ✅ All present |
| contexts/ | `DocumentContext.jsx`, `SettingsContext.jsx`, `SearchContext.jsx`, `ToastContext.jsx` | ✅ All present |
| hooks/ | `useFileWatcher.js`, `useAutoSaveTimer.js` | ✅ All present |
| styles/ | `main.css`, `editor.css`, `theme-light.css`, `theme-dark.css` | ✅ All present |
| scripts/ | `app.js`, `editor.js`, `settings.js`, `search.js`, `recovery.js`, `renderer.js`, `outline.js` | ✅ All present |

### Test Files (src-tauri/tests/)

| Test File | New in Iteration 6 |
|-----------|-------------------|
| `cursor_mapping_tests.rs` | ✅ Yes |
| `focus_mode_tests.rs` | ✅ Yes |
| `typewriter_mode_tests.rs` | ✅ Yes |
| `settings_persistence_tests.rs` | ✅ Yes |
| `tree_sitter_parser_tests.rs` | ✅ Yes |
| `editor_undo_redo_tests.rs` | ✅ Yes |

---

## 12. Iteration 6 Changes Summary

### Architecture Improvements

1. **rusqlite Settings Persistence** - `services/settings.rs` now uses rusqlite with proper schema migration
2. **Tree-sitter Incremental Parsing** - `parser/tree_sitter.rs` implements efficient incremental parsing
3. **Ropey Text Buffer** - `buffer/mod.rs` now uses ropey for O(log n) operations
4. **Service Trait Definitions** - `services/mod.rs` defines `DocumentServiceTrait`, `EditorServiceTrait`, `SettingsServiceTrait`, `FileWatcherServiceTrait`

### Test Coverage Improvements

1. **Cursor Mapping Tests** - `tests/cursor_mapping_tests.rs` with 6 test cases
2. **Focus Mode Tests** - `tests/focus_mode_tests.rs` with 9 test cases
3. **Typewriter Mode Tests** - `tests/typewriter_mode_tests.rs`
4. **Settings Persistence Tests** - `tests/settings_persistence_tests.rs`
5. **Tree-sitter Parser Tests** - `tests/tree_sitter_parser_tests.rs`
6. **Editor Undo/Redo Tests** - `tests/editor_undo_redo_tests.rs`

### Transform Engine Completion

The `TransformEngine` in `editor/transforms.rs` now implements all transform types:
- `Transform::Enter` - General enter with smart list/blockquote/heading detection
- `Transform::Backspace` - Smart backspace at structural boundaries
- `Transform::Tab` / `Transform::ShiftTab` - List indentation
- `Transform::EnterInListItem` - Dedicated list enter handling
- `Transform::EnterInBlockQuote` - Dedicated blockquote enter handling
- `Transform::EnterInHeading` - Dedicated heading enter handling
- `Transform::Wrap` - Text selection wrapping with markers

---

*Report generated for Iteration 6 gap analysis*