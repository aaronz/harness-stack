# RustNote Gap Analysis Report

**Project:** Typora-like Markdown Editor in Rust  
**Current Iteration:** Iteration 5  
**Analysis Date:** April 13, 2026  
**PRD Version:** 3.1 (docs/PRD/)

---

## 1. Executive Summary

The RustNote MVP implementation is approximately **75% complete** by functional requirements. Core file operations, editor rendering, themes, search/replace, and export are functional. Critical gaps exist in transform implementations, undo/redo fidelity, table editing model, image handling, and architecture alignment with PRD specifications.

### Implementation Progress by Category

| Category | Status | Notes |
|----------|--------|-------|
| File Operations (FR-001 to FR-007) | ✅ 90% | All core operations implemented, atomic writes working |
| Core Editor (FR-008 to FR-022) | ⚠️ 60% | Smart transforms incomplete, table editing minimal |
| Markdown Support (FR-020 to FR-022) | ✅ 85% | CommonMark + GFM via comrak |
| Workspace & Navigation (FR-023 to FR-026) | ✅ 80% | File tree functional, outline panel working |
| Display & Themes (FR-027 to FR-030) | ✅ 85% | Light/dark themes, focus/typewriter modes exist |
| Export (FR-031 to FR-033) | ⚠️ 70% | HTML/PDF working, highlighting partial |
| Preferences (FR-034) | ✅ 80% | Settings persistence working |
| Recovery & Safety | ✅ 85% | Snapshots functional, external change detection working |

---

## 2. Gap Analysis Table

### P0 - Blocking Issues (Must Fix)

| Gap | Severity | Module | Description | Fix Suggestion |
|-----|----------|--------|-------------|----------------|
| G-001 | **P0** | Editor | `TransformType::EnterInListItem`, `EnterInBlockQuote`, `EnterInHeading` not implemented in Rust `TransformEngine` - frontend has switch cases but Rust backend returns error/unimplemented | Complete Rust `TransformEngine::apply_*` methods for all transform types |
| G-002 | **P0** | Editor | Undo/redo implementation exists but may have fidelity issues with structural edits - no regression tests for round-trip undo | Add comprehensive undo/redo tests; verify transaction log fidelity |
| G-003 | **P0** | Settings | Settings persistence uses file-based JSON (`settings.json`) instead of rusqlite as specified in PRD-10 | Replace with rusqlite-based settings service per PRD architecture |

### P1 - High Priority Issues

| Gap | Severity | Module | Description | Fix Suggestion |
|-----|----------|--------|-------------|----------------|
| G-004 | **P1** | Editor | Cursor mapping between TipTap DOM and source Markdown is incomplete - `build_cursor_mapping()` creates linear mapping but doesn't account for HTML tag insertion | Implement bidirectional cursor mapping with AST-aware offset calculation |
| G-005 | **P1** | Export | PDF export uses basic text extraction + manual layout rather than proper HTML-to-PDF pipeline; code blocks, images not properly rendered | Integrate proper HTML-to-PDF rendering (e.g., html2pdf, puppeteer, or webview print) |
| G-006 | **P1** | Parser | Parser uses `comrak` directly instead of tree-sitter for incremental parsing as specified in PRD-10 | Implement tree-sitter-based incremental parsing wrapper around comrak |
| G-007 | **P1** | Buffer | `buffer` module exists but doesn't use `ropey` for efficient large document handling as specified | Implement ropey-based text buffer for position-to-offset mapping |
| G-008 | **P1** | Editor | `Transform::Wrap` (for wrapping selected text) not implemented | Add Wrap transform for text selection wrapping |
| G-009 | **P1** | Image | Image paste/drop uses `save_image_from_base64_cmd` but relative path handling may not work correctly when document is in subdirectory | Verify and fix relative path calculation based on document location |

### P2 - Medium Priority Issues

| Gap | Severity | Module | Description | Fix Suggestion |
|-----|----------|--------|-------------|----------------|
| G-010 | **P2** | Editor | Table editing uses TipTap default behavior - constrained but safe model per PRD note, but no custom cell-model editing | Document table editing constraints; ensure no data loss |
| G-011 | **P2** | Export | HTML export produces self-contained HTML but doesn't support "linked-assets mode" as per FR-031 | Add option for linked vs inline asset export |
| G-012 | **P2** | Parser | Frontmatter parsing exists but is basic - doesn't handle all YAML frontmatter variations | Improve frontmatter parsing to handle complex YAML |
| G-013 | **P2** | Display | Focus mode implementation is basic CSS (`body.classList.add('focus-mode')`) - should materially reduce visual distraction per FR-028 | Enhance focus mode to properly dim/hide non-current paragraphs |
| G-014 | **P2** | Display | Typewriter mode implementation exists but may not keep cursor at vertical center during navigation | Verify and fix typewriter mode scroll behavior |
| G-015 | **P2** | Editor | Paste behavior converts to Markdown on best-effort but doesn't handle all rich text paste cases | Improve paste handling for common rich text formats |
| G-016 | **P2** | Services | Architecture has `services/` module but doesn't expose `DocumentService`, `EditorService`, `WorkspaceService`, `ExportService`, `SettingsService`, `RecoveryService` as narrow interfaces per PRD-10 | Refactor to expose proper service interfaces |
| G-017 | **P2** | Recovery | Autosave uses frontend timer (`useAutoSaveTimer`) - not guaranteed to fire if app crashes before timeout | Consider Rust-side autosave with debounce, or ensure frontend timer fires reliably |
| G-018 | **P2** | Editor | Code fence syntax highlighting uses `SyntaxHighlighter` (syntect-based) but Shiki is specified in PRD | Verify `SyntaxHighlighter` implementation matches PRD specification |

---

## 3. Technical Debt

| ID | Category | Description | Impact |
|----|----------|-------------|--------|
| TD-001 | Architecture | Buffer module doesn't use `ropey` - large document handling may be inefficient | Performance |
| TD-002 | Architecture | Parser doesn't use tree-sitter - no incremental parsing | Performance on large docs |
| TD-003 | Architecture | Settings uses file-based JSON instead of `rusqlite` | Data integrity |
| TD-004 | Code | `semantic/position.rs` - `CursorMapping` is defined but may not be used correctly for bidirectional mapping | Editor UX |
| TD-005 | Code | `editor/commands.rs` - `Command` enum exists but editor operations may not use it consistently | Code maintainability |
| TD-006 | Code | Frontend has `Editor.jsx` and `TipTapEditor.jsx` - unclear distinction, potential duplication | Code maintainability |
| TD-007 | Testing | No unit tests for `TransformEngine` beyond basic cases | Reliability |
| TD-008 | Testing | No integration tests for file watcher + editor interaction | Reliability |
| TD-009 | Testing | Visual regression tests exist but baseline may not be current | Quality |

---

## 4. Feature Completeness Checklist

### MVP Scope Features (from PRD-03)

| Feature | FR | Status | Notes |
|---------|-----|--------|-------|
| Desktop app (macOS/Windows/Linux) | - | ✅ | Tauri v2 cross-platform |
| Open, edit, save `.md` files | FR-001, FR-002, FR-004 | ✅ | Working |
| Open folder as workspace | FR-003 | ✅ | Working with sidebar |
| Single-pane live Markdown editing | FR-008 | ⚠️ | Working, cursor mapping incomplete |
| Rendered: headings, emphasis, links, images, lists, task lists, code fences, quotes, HR, tables | FR-020 | ⚠️ | Tables use TipTap default |
| Smart editing: lists, quotes, structure | FR-012, FR-014 | ⚠️ | List/blockquote smart editing incomplete |
| In-document search/replace | FR-025 | ✅ | Working |
| Recent files/folders | FR-024 | ✅ | Settings persists recent files |
| Theme support (light/dark) | FR-027 | ✅ | Working |
| Focus mode | FR-028 | ⚠️ | Basic implementation |
| Typewriter mode | FR-029 | ⚠️ | Basic implementation |
| HTML export | FR-031 | ✅ | Working |
| PDF export | FR-032 | ⚠️ | Working but basic formatting |
| Auto-save | FR-005 | ⚠️ | Frontend timer based |
| Crash recovery | FR-006 | ✅ | Working |
| External file change detection | FR-007 | ✅ | Working |
| Code fence syntax highlighting | FR-015 | ✅ | Via SyntaxHighlighter |
| Relative asset paths | FR-017 | ⚠️ | May need verification |
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
| `editor_apply_transform` | ⚠️ | `commands/editor.rs` - incomplete transform types |
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
| Document | PRD-09 DocumentResult | `model/document.rs::Document` | ⚠️ Missing `headings` in serialization, `id` is UUID not string |
| Heading | PRD-09 Heading | `model/document.rs::Heading` | ✅ |
| TransformType | PRD-09 TransformType | `commands/editor.rs::TransformType` | ⚠️ Missing Wrap variant implementation |
| Settings | PRD-09 Settings | `model/settings.rs::Settings` | ⚠️ Nested `editor` vs flat structure |
| Workspace | Not in PRD contracts | `model/workspace.rs::Workspace` | ✅ |
| FileEntry | Not in PRD contracts | `model/workspace.rs::FileEntry` | ✅ |
| RecoverySnapshot | Not in PRD contracts | `model/recovery.rs::RecoverySnapshot` | ✅ |

### Missing Type Definitions

| Type | Location | Description |
|------|----------|-------------|
| `ErrorCode` enum | Should be in `commands/mod.rs` | Missing from implementation |
| `SaveResult` | Not defined | `save_document` returns `()` not `SaveResult` |
| `ExportResult` | Not defined | `export_*` returns `()` not `ExportResult` |
| `WorkspaceResult` | Not defined | `list_workspace` returns `Workspace` directly |

---

## 7. Frontend Completeness

### Implemented Components

| Component | File | Status |
|-----------|------|--------|
| App | `App.jsx` | ✅ Main layout with all panels |
| TipTapEditor | `TipTapEditor.jsx` | ⚠️ Core editor, incomplete cursor mapping |
| Editor | `Editor.jsx` | ⚠️ Legacy/alternative editor |
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
| SettingsContext | ⚠️ Mapping issues between Rust/JS types |
| SearchContext | ✅ Working |
| ToastContext | ✅ Working |

### Missing Components

| Component | Description | Priority |
|-----------|-------------|----------|
| SourceEditor | CodeMirror 6 for source mode (per PRD-10 structure) | P2 |
| Preferences UI | Settings UI beyond toolbar buttons | P2 |

---

## 8. Rust Crate Structure Analysis

### Current Structure vs PRD-10 Recommended

| PRD-10 Module | Current Implementation | Status |
|---------------|----------------------|--------|
| `buffer` (ropey) | `buffer/mod.rs` exists, NOT using ropey | ⚠️ |
| `parser` (tree-sitter + comrak) | `parser/mod.rs` uses comrak only | ⚠️ |
| `editor-engine` | `editor/mod.rs` + `editor/transforms.rs` | ⚠️ |
| `serializer` | `semantic/ast.rs` partial | ⚠️ |
| `workspace` | `commands/workspace.rs` + `commands/file_tree.rs` | ✅ |
| `export` | `commands/export.rs` | ⚠️ |
| `theme` | CSS-based in `www/src/styles/` | ✅ |
| `settings` (rusqlite) | File-based JSON | ⚠️ |
| `recovery` | `commands/recovery.rs` + `model/recovery.rs` | ✅ |
| `app-services` | `commands/mod.rs` | ⚠️ |

---

## 9. Test Coverage Analysis

### Test Files Present

| Test File | Coverage |
|-----------|----------|
| `tests/buffer_settings_tests.rs` | Basic buffer and settings |
| `tests/integration_editor_tests.rs` | Editor integration |
| `tests/integration_file_tests.rs` | File operations |
| `tests/editor_engine_tests.rs` | Editor engine |
| `tests/editor_transforms.rs` | Transform operations |
| `tests/parser_tests.rs` | Markdown parsing |
| `tests/parser_edge_cases.rs` | Edge cases |
| `benches/transforms.rs` | Transform benchmarks |
| `benches/serialization.rs` | Serialization benchmarks |
| `benches/parsing.rs` | Parsing benchmarks |

### Missing Test Coverage

| Area | Priority |
|------|----------|
| Undo/redo round-trip fidelity | P0 |
| Cursor mapping accuracy | P1 |
| Transform fidelity for all edge cases | P1 |
| File watcher race conditions | P2 |
| External change detection | P2 |

---

## 10. Recommendations

### Immediate Actions (P0)

1. **Complete Transform Implementation**: Implement all `TransformType` variants in Rust `TransformEngine`
2. **Fix Undo/Redo**: Add comprehensive tests and fix any fidelity issues
3. **Switch to rusqlite**: Replace file-based settings with rusqlite persistence

### Short-term Actions (P1)

1. **Cursor Mapping**: Implement bidirectional cursor mapping between TipTap DOM and source
2. **Ropey Buffer**: Implement ropey-based buffer for efficient large document handling
3. **Tree-sitter Parser**: Add tree-sitter incremental parsing wrapper
4. **PDF Export**: Improve PDF rendering quality

### Medium-term Actions (P2)

1. **Focus/Typewriter Mode Polish**: Enhance visual effects to meet PRD expectations
2. **Image Path Handling**: Verify and fix relative path calculation
3. **Paste Handling**: Improve rich text paste conversion
4. **Architecture Alignment**: Refactor to expose proper service interfaces

---

## 11. Files Analyzed

### Rust Backend
- `src-tauri/src/lib.rs` - Main entry, command registration
- `src-tauri/src/commands/mod.rs` - Command module
- `src-tauri/src/commands/document.rs` - Document commands
- `src-tauri/src/commands/editor.rs` - Editor commands
- `src-tauri/src/commands/workspace.rs` - Workspace commands
- `src-tauri/src/commands/file_tree.rs` - File tree commands
- `src-tauri/src/commands/settings.rs` - Settings commands
- `src-tauri/src/commands/export.rs` - Export commands
- `src-tauri/src/commands/recovery.rs` - Recovery commands
- `src-tauri/src/commands/render.rs` - Render commands
- `src-tauri/src/model/document.rs` - Document model
- `src-tauri/src/model/settings.rs` - Settings model
- `src-tauri/src/model/workspace.rs` - Workspace model
- `src-tauri/src/model/recovery.rs` - Recovery model
- `src-tauri/src/semantic/mod.rs` - Semantic module
- `src-tauri/src/semantic/ast.rs` - Semantic AST
- `src-tauri/src/editor/mod.rs` - Editor module
- `src-tauri/src/editor/transforms.rs` - Transform engine
- `src-tauri/src/renderer/mod.rs` - Renderer module
- `src-tauri/src/parser/mod.rs` - Parser module

### Frontend
- `www/src/App.jsx` - Main app
- `www/src/components/TipTapEditor.jsx` - TipTap editor
- `www/src/components/Toolbar.jsx` - Toolbar
- `www/src/components/Sidebar.jsx` - Sidebar
- `www/src/components/OutlinePanel.jsx` - Outline panel
- `www/src/components/SearchPanel.jsx` - Search panel
- `www/src/contexts/DocumentContext.jsx` - Document context
- `www/src/contexts/SettingsContext.jsx` - Settings context
- `www/src/contexts/SearchContext.jsx` - Search context

### Configuration
- `Cargo.toml` - Rust workspace
- `package.json` - Node dependencies

---

*Report generated for Iteration 5 gap analysis*
