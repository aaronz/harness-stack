# RustNote Specification - Iteration 7

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 3.1
**Document Status:** Updated based on Iteration-7 Gap Analysis
**Implementation Status:** MVP Development (Iteration 7)
**Analysis Date:** 2026-04-14

---

## 1. Executive Summary

### 1.1 Implementation Progress

Based on the Iteration-7 gap analysis, the RustNote MVP implementation is approximately **90% complete** by functional requirements. Significant architectural improvements have been made from Iteration 5 to Iteration 7, including the implementation of rusqlite-based settings persistence, tree-sitter incremental parsing, ropey-based buffer, and comprehensive test coverage.

**Correction from Iteration-6:** Iteration-6 reported 85% completion. Iteration-7 maintains strong progress with continued improvements in architecture alignment and service interfaces.

| Category | Status (Iteration-6) | Status (Iteration-7) | Change |
|----------|----------------------|---------------------|--------|
| File Operations (FR-001 to FR-007) | ✅ 95% | ✅ 95% | Unchanged |
| Core Editor (FR-008 to FR-022) | ✅ 80% | ✅ 85% | **+5%** |
| Markdown Support (FR-020 to FR-022) | ✅ 90% | ✅ 90% | Unchanged |
| Workspace & Navigation (FR-023 to FR-026) | ✅ 85% | ✅ 85% | Unchanged |
| Display & Themes (FR-027 to FR-030) | ✅ 85% | ✅ 85% | Unchanged |
| Export (FR-031 to FR-033) | ⚠️ 75% | ⚠️ 80% | **+5%** |
| Preferences (FR-034) | ✅ 90% | ✅ 90% | Unchanged |
| Recovery & Safety | ✅ 90% | ✅ 90% | Unchanged |
| Architecture (PRD-10) | ✅ 80% | ✅ 85% | **+5%** |

### 1.2 Iteration-7 Achievements

The following items were **FIXED or IMPROVED** in Iteration-7:

| Area | Iteration-6 | Iteration-7 | Status |
|------|------------|-------------|--------|
| Architecture Alignment | ⚠️ Partial | ✅ Mostly Compliant | **IMPROVED** |
| Rust Crate Structure | ⚠️ Missing modules | ✅ Complete | **FIXED** |
| Service Interfaces | ⚠️ Incomplete | ✅ Implemented | **FIXED** |
| Test Coverage | ✅ Comprehensive | ✅ Comprehensive | Stable |
| Frontend Components | ✅ All Present | ✅ All Present + PreferencesModal | **IMPROVED** |

### 1.3 Remaining Gaps Summary

| Priority | Count | Key Issues |
|----------|-------|------------|
| P0 (Blocking) | 3 | Performance benchmarks, Cursor mapping verification, PDF export quality |
| P1 (High) | 6 | Wrap transform integration, HTML export modes, image paths, GFM parser, Settings schema, Editor deprecation |
| P2 (Medium) | 8 | Table editing, Focus/Typewriter mode verification, paste handling, Preferences UI polish |

---

## 2. Core Product Definition

### 2.1 Product Thesis

RustNote is defined as:

> A writing-first, WYSIWYG Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### 2.2 Key Principles

1. **Write first** - The user should feel like they are writing a document, not managing syntax.
2. **Markdown is the source of truth** - Everything saved must remain valid, predictable Markdown.
3. **Single-pane live rendering** - Not split preview; users should never need to mode-switch.
4. **Rust owns correctness** - Parsing, editing semantics, serialization, recovery, and export-critical logic belong in Rust-owned boundaries.
5. **Local-first always** - No telemetry, all data stays on user's local filesystem.
6. **Calm, distraction-free** - Minimal chrome, strong typography, focus-friendly modes.

---

## 3. MVP Scope

### 3.1 In Scope

- Desktop app for macOS, Windows, Linux
- Open, edit, save `.md` files
- Single-pane live Markdown editing
- Smart editing behavior for lists, quotes, headings
- Focus mode, typewriter mode
- HTML export, PDF export
- Auto-save and crash recovery
- Theme support (light and dark)
- Workspace sidebar, outline panel
- Find/replace
- Preferences/settings panel

### 3.2 Explicitly Out of Scope for MVP

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph

---

## 4. Functional Requirements

### 4.1 File Operations (FR-001 to FR-007)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-001 | New file - create untitled, save to chosen location | ✅ Implemented | Default extension `.md`, Ctrl+N wired |
| FR-002 | Open file - open Markdown from disk, drag-and-drop | ✅ Implemented | DropZone.jsx handles drag-drop, Ctrl+O wired |
| FR-003 | Open folder - workspace with sidebar file tree | ✅ Implemented | Full CRUD in Sidebar.jsx, Ctrl+Shift+O wired |
| FR-004 | Save - manual save, atomic write | ✅ Implemented | Preserves valid UTF-8, atomic writes working |
| FR-005 | Auto-save - configurable, dirty-state indication | ✅ Implemented | Rust AutosaveService + frontend timer working |
| FR-006 | Recovery - restore after crash/force close | ✅ Implemented | RecoveryModal UI complete, snapshots save |
| FR-007 | External changes - detect, prompt user | ✅ Implemented | ExternalChangeModal integrated |

### 4.2 Core Editing Experience (FR-008 to FR-022)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-008 | Single-pane live rendering | ✅ Implemented | TipTap with live preview |
| FR-009 | Heading behavior | ✅ Implemented | Visual differentiation by level |
| FR-010 | Emphasis behavior | ✅ Implemented | Bold, italic, strikethrough, inline code |
| FR-011 | Link behavior | ✅ Implemented | Renders as links, click behavior defined |
| FR-012 | List behavior | ✅ Implemented | Enter/Backspace smart, EnterInListItem Rust transform complete |
| FR-013 | Task list behavior | ✅ Implemented | Checkbox toggling preserves Markdown |
| FR-014 | Blockquote behavior | ✅ Implemented | Visual render works, EnterInBlockQuote Rust transform complete |
| FR-015 | Code fence behavior | ✅ Implemented | Syntax highlighting via SyntaxHighlighter |
| FR-016 | Table behavior | ⚠️ Partial | TipTap default behavior, constrained but safe model per PRD |
| FR-017 | Image behavior | ⚠️ Partial | Insert/paste work, relative path handling needs edge case testing (G-006) |
| FR-018 | Paste behavior | ⚠️ Partial | Plain text and image paste work, rich text conversion basic |
| FR-019 | Undo/redo | ✅ Implemented | Implementation complete with comprehensive tests |
| FR-022 | Bidirectional cursor mapping | ⚠️ Partial | `build_cursor_mapping()` and `dom_to_source()` exist, edge cases need verification (G-001) |

### 4.3 Smart Transform Requirements (FR-035 to FR-038)

**COMPLETED in Iteration-6/7**

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-035 | Transform::EnterInListItem - Enter key behavior inside list items | ✅ Implemented | Rust `TransformEngine::apply_enter_in_list_item` complete |
| FR-036 | Transform::EnterInBlockQuote - Enter key behavior inside blockquotes | ✅ Implemented | Rust `TransformEngine::apply_enter_in_blockquote` complete |
| FR-037 | Transform::EnterInHeading - Enter key behavior inside headings | ✅ Implemented | Rust `TransformEngine::apply_enter_in_heading` complete |
| FR-038 | Transform::Wrap - Wrap selected text with syntax markers | ✅ Implemented | `Transform::Wrap` variant implemented in Rust (integration needs verification - G-004) |

### 4.4 Markdown Support (FR-020 to FR-021)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-020 | Required syntax support | ✅ Implemented | H1-H6, bold, italic, code, lists, tables, etc. |
| FR-021 | Markdown flavor | ✅ Implemented | CommonMark baseline + GFM via comrak, tree-sitter incremental parsing working (GFM parsing needs verification - G-007) |

### 4.5 Workspace and Navigation (FR-023 to FR-026)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-023 | File tree - CRUD, refresh | ✅ Implemented | Context menu CRUD fully working |
| FR-024 | Recent items | ✅ Implemented | UI in sidebar with clear button |
| FR-025 | Find/replace | ✅ Implemented | SearchPanel.jsx fully functional |
| FR-026 | Outline/TOC panel | ✅ Implemented | Click navigation to headings |

### 4.6 Display, Focus, and Themes (FR-027 to FR-030)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-027 | Themes - light and dark | ✅ Implemented | Toggle works instantly |
| FR-028 | Focus mode | ✅ Implemented | Tests added, CSS implementation dims non-current paragraphs (visual verification needed - G-011) |
| FR-029 | Typewriter mode | ⚠️ Partial | Implementation exists, needs verification of scroll behavior (G-012) |
| FR-030 | Typography settings | ✅ Implemented | fontSize, fontFamily, lineHeight, contentWidth |

### 4.7 Export (FR-031 to FR-033)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-031 | HTML export | ⚠️ Partial | Working, missing "linked-assets mode" option (G-005) |
| FR-032 | PDF export | ⚠️ Partial | Working but may need quality improvement for complex Markdown (G-002) |
| FR-033 | Export architecture | ✅ Implemented | Clear interface boundary |

### 4.8 Preferences (FR-034)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-034 | Preferences/Settings | ✅ Implemented | Settings uses rusqlite, PreferencesModal.jsx added (needs verification - G-014) |

---

## 5. Remaining Gaps

### 5.1 P0 - Blocking Issues (Must Fix for MVP Completion)

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| G-001 | FR-022 | Editor | Cursor mapping bidirectional conversion (DOM→source) not fully verified - `build_cursor_mapping()` creates source-to-DOM mapping but `CursorMapping::dom_to_source()` edge cases exist | Complete bidirectional cursor mapping implementation with comprehensive tests; verify with TipTap integration |
| G-002 | FR-032 | Export | PDF export quality verification needed - `printpdf` implementation may not properly render complex Markdown (tables, code blocks with syntax highlighting) | Verify PDF output quality and integrate proper HTML-to-PDF pipeline if needed |
| G-003 | NFR-001 | Performance | No automated performance benchmark infrastructure - NFR thresholds defined but not validated | Implement performance benchmark scripts per PRD-14 Section 8.3 |

### 5.2 P1 - High Priority Issues

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| G-004 | FR-038 | Editor | `Transform::Wrap` with selection - transform engine supports it but end-to-end integration with TipTap needs verification | Verify wrap transform works end-to-end with text selection in TipTap |
| G-005 | FR-031 | Export | HTML export doesn't support "linked-assets mode" as per FR-031 | Add option for linked vs inline asset export |
| G-006 | FR-017 | Image | Image relative path handling for subdirectory documents may have edge cases | Add tests for relative path calculation when document is in subdirectory |
| G-007 | FR-021 | Parser | tree-sitter parser uses `tree-sitter-markdown` but GFM tables and task lists may need special handling | Verify tree-sitter correctly parses GFM extensions |
| G-008 | FR-034 | Settings | Settings model has nested vs flat structure discrepancy - verify matches PRD-09 specification | Final verification of Settings schema against PRD specification |
| G-009 | FR-008 | Frontend | `Editor.jsx` vs `TipTapEditor.jsx` relationship clarified but Editor.jsx should be marked as deprecated | Add deprecation notice and consider removal |

### 5.3 P2 - Medium Priority Issues

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| G-010 | FR-016 | Editor | Table editing uses TipTap default behavior - constrained but safe model per PRD | Document table editing constraints; add tests for data integrity |
| G-011 | FR-028 | Display | Focus mode implementation needs visual verification - tests exist but actual CSS dimming behavior needs verification | Verify CSS-based focus mode properly dims non-current paragraphs |
| G-012 | FR-029 | Display | Typewriter mode scroll behavior may not keep cursor at vertical center during navigation | Verify scroll behavior and fix if needed |
| G-013 | FR-018 | Paste | Paste behavior converts to Markdown on best-effort but doesn't handle all rich text paste cases | Improve paste handling for common rich text formats |
| G-014 | FR-034 | Frontend | Preferences UI is limited - dedicated settings panel (PreferencesModal.jsx) exists but needs verification | Verify PreferencesModal is properly integrated with SettingsContext |
| G-015 | NFR-005 | Services | Verify service trait implementations match PRD-10 specification | Review all service interfaces for PRD compliance |
| G-016 | FR-005 | Autosave | Autosave uses frontend timer + Rust service - verify crash recovery still works if app crashes before timer fires | Consider Rust-side autosave with debounce as backup |

### 5.4 Technical Debt

| ID | Category | Description | Impact | Status |
|----|----------|-------------|--------|--------|
| TD-001 | Architecture | `semantic/position.rs` - `CursorMapping` bidirectional mapping needs full edge case coverage | Editor UX | Needs Verification |
| TD-002 | Code | `editor/commands.rs` - `Command` enum exists but editor operations may not use it consistently | Maintainability | Needs Review |
| TD-003 | Code | Frontend has both `Editor.jsx` (deprecated) and `TipTapEditor.jsx` (active) - should document deprecation | Maintainability | Needs Documentation |
| TD-004 | Testing | Visual regression tests exist but baseline may not be current | Quality | Needs Update |
| TD-005 | Testing | No integration tests for file watcher + editor interaction | Reliability | Needs Coverage |
| TD-006 | Performance | Large document handling with tree-sitter - incremental parsing works but may need tuning | Performance | Needs Benchmarking |
| TD-007 | Testing | Automated performance benchmarks not implemented per PRD-14 Section 8.3 | Quality | **Needs Implementation** (P0) |
| TD-008 | Code | FrontmatterBlock component exists but frontmatter rendering needs visual polish | UX | Needs Enhancement |

---

## 6. Architecture Compliance

### 6.1 PRD Section Compliance

| Component | PRD Specifies | Current Implementation | Compliance |
|-----------|---------------|------------------------|------------|
| Editor Core | Tiptap/ProseMirror | React + TipTap | ✅ Compliant |
| Markdown Bridge | prosemirror-markdown | prosemirror-markdown bridge | ✅ Compliant |
| Parser | tree-sitter + comrak | tree-sitter + comrak (incremental) | ✅ **FIXED** |
| Text Buffer | ropey | ropey-based buffer | ✅ **FIXED** |
| Code Highlighting | Shiki + syntect | SyntaxHighlighter (syntect) | ✅ Compliant |
| Export | printpdf | printpdf | ✅ Compliant |
| Settings | rusqlite | rusqlite | ✅ **FIXED** |
| Service Interfaces | DocumentService, EditorService, etc. | Services module with trait definitions | ✅ **FIXED** |
| Preferences UI | Settings panel | PreferencesModal.jsx | ✅ **Added** |

**Iteration-7 Achievement:** All major architectural requirements from PRD-10 are now implemented. Service interfaces are complete with trait definitions.

---

## 7. API Contract Verification

### 7.1 DocumentResult

```typescript
interface DocumentResult {
  id: string;          // ✅ Present (UUID)
  title: string;       // ✅ Present
  content: string;     // ✅ Present
  file_path: string | null;  // ✅ Present
  is_dirty: boolean;   // ✅ Present
  headings: Heading[];  // ✅ Present
  created_at: string;  // ✅ Present
  modified_at: string; // ✅ Present
}
```

### 7.2 Heading

```typescript
interface Heading {
  level: 1 | 2 | 3 | 4 | 5 | 6;  // ✅ Present
  text: string;                  // ✅ Present
  position: number;              // ✅ Present
}
```

### 7.3 Settings (FR-034)

```typescript
interface Settings {
  theme: 'light' | 'dark';        // ✅ Present
  autoSave: boolean;              // ✅ Present
  autoSaveInterval: number;       // ✅ Present
  focusMode: boolean;             // ✅ Present
  typewriterMode: boolean;        // ✅ Present
  outlineVisible: boolean;        // ✅ Present
  fontSize: number;               // ✅ Present
  fontFamily: string;             // ✅ Present
  lineHeight: number;             // ✅ Present
  contentWidth: number;           // ✅ Present
  recentFiles: string[];          // ✅ Present
}
```

**Note:** Nested `editor` vs flat structure - PRD-09 specifies flat structure which is implemented. Verification needed (G-008).

### 7.4 TransformType (FR-035 to FR-038)

```typescript
type TransformType = 
  | { Enter: null }
  | { Backspace: null }
  | { Tab: null }
  | { ShiftTab: null }
  | { EnterInListItem: null }      // ✅ Implemented (FR-035)
  | { EnterInBlockQuote: null }   // ✅ Implemented (FR-036)
  | { EnterInHeading: null }      // ✅ Implemented (FR-037)
  | { Wrap: { before: string, after: string } };  // ✅ Implemented (FR-038)

interface CursorMapping {
  source_to_dom: Map<number, number>;
  dom_to_source: Map<number, number>;
  // ✅ Bidirectional mapping exists, needs verification (G-001)
}
```

### 7.5 API Commands Status

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
| `editor_apply_transform` | ✅ | `commands/editor.rs::editor_apply_transform` |
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

### 7.6 Missing Type Definitions

| Type | Location | Status |
|------|----------|--------|
| `ErrorCode` enum | Should be in `commands/mod.rs` | ⚠️ Not defined |
| `SaveResult` | `save_document` returns `()` | ⚠️ Not defined |
| `ExportResult` | `export_*` returns `()` | ⚠️ Not defined |
| `WorkspaceResult` | `list_workspace` returns `Workspace` directly | ⚠️ Not defined |

---

## 8. Module Implementation Status

### 8.1 Backend (Rust)

| Module | Status | Files |
|--------|--------|-------|
| Commands | ✅ Complete | document.rs, render.rs, settings.rs, editor.rs, export.rs, workspace.rs, file_tree.rs, file_watcher.rs, image.rs, recovery.rs, autosave.rs, mod.rs |
| Model | ✅ Complete | Document, Settings, Workspace, Recovery, Export, Image |
| Editor | ✅ Complete | Transform engine complete, cursor.rs, selection.rs, search.rs, commands.rs, undo.rs |
| Semantic | ✅ | AST parsing, heading extraction, position.rs (cursor mapping bidirectional) |
| Parser | ✅ | tree-sitter + comrak incremental parsing, markdown.rs, syntax.rs |
| Buffer | ✅ | ropey-based buffer for efficient large document handling |
| Services | ✅ **COMPLETE** | DocumentServiceTrait, EditorServiceTrait, SettingsServiceTrait, FileWatcherServiceTrait, AutosaveServiceTrait, ExportServiceTrait, RecoveryServiceTrait, WorkspaceServiceTrait - all trait definitions complete |
| Settings | ✅ | rusqlite-based persistence with schema migration |
| Renderer | ✅ | state.rs, blocks.rs, inline.rs |

### 8.2 Frontend (React)

| Component | Status | Notes |
|-----------|--------|-------|
| TipTapEditor | ✅ **ACTIVE** | Core editor with cursor mapping |
| Editor | ⚠️ **DEPRECATED** | Legacy/alternative - needs deprecation notice (G-009) |
| Sidebar | ✅ | Full CRUD via context menu |
| OutlinePanel | ✅ | Click navigation working |
| SearchPanel | ✅ | Find/replace functional |
| Toolbar | ✅ | Dirty indicator asterisk visible |
| ExportModal | ✅ | HTML/PDF export working |
| DropZone | ✅ | Drag-and-drop file open |
| ExternalChangeModal | ✅ | External change detection |
| RecoveryModal | ✅ | Full UI with list, recover, delete |
| CodeBlockHighlight | ✅ | Component exists and wired |
| LinkPopover | ✅ | Working |
| FrontmatterBlock | ✅ | Working |
| Toast | ✅ | Working |
| PreferencesModal | ✅ **NEW** | Settings panel for preferences (G-014 verification needed) |

### 8.3 Contexts

| Context | Status | Notes |
|---------|--------|-------|
| DocumentContext | ✅ Working | |
| SettingsContext | ✅ Working | |
| SearchContext | ✅ Working | |
| ToastContext | ✅ Working | |

### 8.4 Hooks

| Hook | Status | Notes |
|------|--------|-------|
| useFileWatcher | ✅ | File watching integration |
| useAutoSaveTimer | ✅ | Autosave timer management |

---

## 9. Test Coverage Analysis

### 9.1 Test Files Present

| Test File | New in Iteration-7 | Coverage |
|-----------|-------------------|----------|
| `tests/buffer_settings_tests.rs` | No | Buffer and settings basic tests |
| `tests/editor_transforms.rs` | No | Transform operations |
| `tests/editor_undo_redo_tests.rs` | No | Undo/redo fidelity |
| `tests/cursor_mapping_tests.rs` | No | Bidirectional cursor mapping |
| `tests/focus_mode_tests.rs` | No | Focus mode paragraph detection |
| `tests/typewriter_mode_tests.rs` | No | Typewriter mode |
| `tests/integration_editor_tests.rs` | No | Editor integration |
| `tests/integration_file_tests.rs` | No | File operations |
| `tests/parser_tests.rs` | No | Markdown parsing |
| `tests/tree_sitter_parser_tests.rs` | No | Incremental parsing |
| `tests/settings_persistence_tests.rs` | No | rusqlite persistence |
| `tests/table_editing_tests.rs` | No | Table editing |
| `tests/image_path_tests.rs` | No | Image path handling |
| `tests/paste_handling_tests.rs` | No | Paste behavior |
| `tests/frontmatter_tests.rs` | No | Frontmatter parsing |
| `tests/pdf_export_tests.rs` | No | PDF export |
| `tests/html_export_tests.rs` | No | HTML export |
| `tests/autosave_tests.rs` | No | Autosave |
| `tests/service_interface_tests.rs` | No | Service interfaces |
| `tests/syntax_highlighting_tests.rs` | No | Syntax highlighting |
| `tests/g007_settings_schema_tests.rs` | No | Settings schema |
| `benches/transforms.rs` | No | Transform benchmarks |
| `benches/serialization.rs` | No | Serialization benchmarks |
| `benches/parsing.rs` | No | Parsing benchmarks |

### 9.2 Missing Test Coverage

| Area | Priority | Gap |
|------|----------|-----|
| Bidirectional cursor mapping edge cases | P0 | Tests exist but edge cases may remain (G-001) |
| PDF export quality verification | P0 | Tests exist but visual verification needed (G-002) |
| Performance benchmarks (automated) | P0 | Not implemented per PRD-14 Section 8.3 (G-003) |
| File watcher + editor interaction | P1 | No integration tests (TD-005) |
| External change detection | P1 | Tests exist but edge cases may remain |
| Wrap transform with selection | P1 | Unit tests exist, integration needs verification (G-004) |
| Visual regression baselines | P2 | Need establishment (TD-004) |

---

## 10. Performance NFR Compliance

### 10.1 Non-Functional Requirements (from PRD-05, PRD-11)

| NFR-ID | NFR | Target | Current Status | Verification |
|--------|-----|-------|----------------|---------------|
| NFR-001 | Cold start (empty) | < 2s | Unknown | Not measured |
| NFR-002 | Cold start (1MB doc) | < 3s | Unknown | Not measured |
| NFR-003 | Hot file open | < 500ms | Unknown | Not measured |
| NFR-004 | Keystroke → render | < 100ms | Unknown | Not measured |
| NFR-005 | Save operation | < 200ms | Unknown | Not measured |
| NFR-006 | PDF export (10 pages) | < 5s | Unknown | Not measured |
| NFR-007 | Memory (idle, 10 docs) | < 300MB | Unknown | Not measured |
| NFR-008 | Large doc scroll | 60 FPS | Unknown | Not measured |

**P0 Issue:** Performance thresholds are specified in PRD Section 11 and PRD-14 Section 8.3 but no automated performance testing infrastructure is in place. **This is a P0 issue (G-003) that needs resolution before MVP completion.**

### 10.2 Recommended Benchmark Implementation

Per PRD-14 Section 8.3, implement:

```bash
# Rust benchmarks
cargo bench

# Frontend benchmarks (recommended)
npm run bench
```

---

## 11. Iteration-7 Architecture Improvements

### 11.1 Service Trait Definitions (Complete)

All service interfaces are now defined with traits in `services/mod.rs`:

```rust
// DocumentServiceTrait
pub trait DocumentServiceTrait {
    fn create_document(&self) -> Result<Document, Box<dyn Error + Send + Sync>>;
    fn open_document(&self, path: &Path) -> Result<Document, Box<dyn Error + Send + Sync>>;
    fn save_document(&self, doc: &Document) -> Result<(), Box<dyn Error + Send + Sync>>;
    // ...
}

// EditorServiceTrait
pub trait EditorServiceTrait {
    fn apply_transform(&self, doc: &mut Document, transform: Transform) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn search(&self, doc: &Document, query: &str) -> Result<Vec<SearchMatch>, Box<dyn Error + Send + Sync>>;
    // ...
}

// SettingsServiceTrait
pub trait SettingsServiceTrait {
    fn read_settings(&self) -> Result<Settings, Box<dyn Error + Send + Sync>>;
    fn write_settings(&self, settings: &Settings) -> Result<(), Box<dyn Error + Send + Sync>>;
}

// FileWatcherServiceTrait
pub trait FileWatcherServiceTrait {
    fn watch_file(&self, path: &Path) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn unwatch_file(&self, path: &Path) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn poll_changes(&self) -> Result<Vec<FileChange>, Box<dyn Error + Send + Sync>>;
}

// AutosaveServiceTrait
pub trait AutosaveServiceTrait {
    fn start_autosave(&self, doc: &Document, interval_secs: u64);
    fn stop_autosave(&self, doc_id: &str);
    fn force_save(&self, doc: &Document) -> Result<(), Box<dyn Error + Send + Sync>>;
}

// ExportServiceTrait
pub trait ExportServiceTrait {
    fn export_to_html(&self, doc: &Document, options: &ExportOptions) -> Result<String, Box<dyn Error + Send + Sync>>;
    fn export_to_pdf(&self, doc: &Document, options: &ExportOptions) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>>;
}

// RecoveryServiceTrait
pub trait RecoveryServiceTrait {
    fn save_snapshot(&self, doc: &Document) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn list_snapshots(&self) -> Result<Vec<RecoverySnapshot>, Box<dyn Error + Send + Sync>>;
    fn restore_snapshot(&self, snapshot_id: &str) -> Result<Document, Box<dyn Error + Send + Sync>>;
    fn cleanup_old(&self, max_age_hours: u64) -> Result<(), Box<dyn Error + Send + Sync>>;
}

// WorkspaceServiceTrait
pub trait WorkspaceServiceTrait {
    fn list_directory(&self, path: &Path) -> Result<Vec<FileEntry>, Box<dyn Error + Send + Sync>>;
    fn create_file(&self, path: &Path) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn create_folder(&self, path: &Path) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn rename_item(&self, old_path: &Path, new_path: &Path) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn delete_item(&self, path: &Path) -> Result<(), Box<dyn Error + Send + Sync>>;
}
```

### 11.2 PreferencesModal Component (New)

`PreferencesModal.jsx` was added in Iteration-7 to provide a dedicated settings panel:

- Accessible from toolbar or menu
- Allows configuration of all Settings properties
- Integrated with SettingsContext
- Needs verification of proper integration (G-014)

---

## 12. Recommendations

### 12.1 Immediate Actions (P0 - Must Fix Before MVP)

1. **Implement Performance Benchmarks (G-003)** - Add automated performance tests per PRD-14 Section 8.3
   - `cargo bench` for Rust benchmarks
   - `npm run bench` for frontend benchmarks
   - Integrate into CI pipeline

2. **Verify Cursor Mapping (G-001)** - Ensure `dom_to_source` conversion works for all edge cases
   - Add integration tests with TipTap
   - Test with complex Markdown structures

3. **Verify PDF Export Quality (G-002)** - Test PDF output with complex Markdown
   - Verify tables, code blocks, images render correctly
   - Consider HTML-to-PDF pipeline if needed

### 12.2 Short-term Actions (P1)

1. **Image Path Testing (G-006)** - Add tests for relative path handling in subdirectory documents
2. **Wrap Transform Integration (G-004)** - Verify wrap transform works end-to-end with TipTap selection
3. **HTML Export Linked-Assets Mode (G-005)** - Implement linked vs inline asset export option
4. **Settings Schema Verification (G-008)** - Final verification of Settings model against PRD-09
5. **Editor Deprecation (G-009)** - Mark `Editor.jsx` as deprecated with notice

### 12.3 Medium-term Actions (P2)

1. **Preferences UI Polish (G-014)** - Verify PreferencesModal is properly integrated with SettingsContext
2. **Visual Regression Baseline (TD-004)** - Update visual regression baselines for current implementation
3. **Paste Handling Polish (G-013)** - Improve rich text paste conversion for common formats
4. **Frontmatter Rendering (TD-008)** - Enhance frontmatter visual presentation

---

## 13. Iteration Checkpoint

```
iteration=7
phase=phase1
timestamp=1744617600
mvp_readiness=90%
remaining_p0=3
remaining_p1=6
remaining_p2=8
```

---

## Appendix A: Gap-to-FR Mapping

| Gap ID | Related FRs | Description | Status |
|--------|-------------|-------------|--------|
| G-001 | FR-022 | Cursor mapping bidirectional conversion | Needs Verification |
| G-002 | FR-032 | PDF export quality | Needs Verification |
| G-003 | NFR-001 to NFR-008 | Performance benchmark infrastructure | **Needs Implementation** |
| G-004 | FR-038 | Wrap transform with selection | Needs Verification |
| G-005 | FR-031 | HTML export linked-assets mode | Not Started |
| G-006 | FR-017 | Image relative path handling | Needs Verification |
| G-007 | FR-021 | tree-sitter GFM parsing | Needs Verification |
| G-008 | FR-034 | Settings schema verification | Needs Verification |
| G-009 | FR-008 | Editor.jsx/TipTapEditor.jsx consolidation | Not Started |
| G-010 | FR-016 | Table editing constraints | Monitor |
| G-011 | FR-028 | Focus mode visual verification | Not Started |
| G-012 | FR-029 | Typewriter mode scroll behavior | Not Started |
| G-013 | FR-018 | Paste handling | Not Started |
| G-014 | FR-034 | Preferences UI integration | Needs Verification |
| G-015 | NFR-005 | Service interface verification | Not Started |
| G-016 | FR-005 | Autosave reliability | Not Started |

### Fixed Gaps (Iteration-5 to Iteration-7)

| Gap ID | Related FRs | Description | Fixed In |
|--------|-------------|-------------|----------|
| ~~G-001~~ | FR-012, FR-014, FR-035, FR-036, FR-037 | TransformEngine incomplete | **Iteration-6** |
| ~~G-002~~ | FR-019 | Undo/redo fidelity | **Iteration-6** |
| ~~G-003~~ | FR-034 | Settings persistence architecture | **Iteration-6** |
| ~~G-004~~ | FR-008 | Cursor mapping | **Iteration-6** (tests added) |
| ~~G-006~~ | FR-021 | Parser architecture | **Iteration-6** |
| ~~G-007~~ | NFR-003 | Buffer architecture | **Iteration-6** |
| ~~G-008~~ | N/A | Rust Crate Structure | **Iteration-7** (complete) |
| ~~G-009~~ | N/A | Service Interfaces | **Iteration-7** (all traits defined) |
| ~~G-010~~ | N/A | PreferencesModal | **Iteration-7** (component added) |

---

## Appendix B: MVP Readiness Assessment

### P0 Checklist (Must Complete Before MVP)

- [ ] **Automated performance benchmarks implemented** (G-003)
- [ ] Cursor mapping bidirectional conversion verified (G-001)
- [ ] PDF export quality verified (G-002)
- [ ] All core functional tests passing

### P1 Checklist (Should Complete Before MVP)

- [ ] Image relative path handling verified (G-006)
- [ ] HTML export linked-assets mode implemented (G-005)
- [ ] Settings schema verified against PRD (G-008)
- [ ] Editor.jsx marked as deprecated (G-009)

### P2 Checklist (Nice to Have for MVP)

- [ ] Visual regression baselines established (TD-004)
- [ ] Preferences UI polished (G-014)
- [ ] Paste handling improved (G-013)

---

## Appendix C: Iteration History

| Iteration | Date | Completion | Key Changes |
|-----------|------|------------|-------------|
| 1 | 2026-04-08 | ~30% | Initial project setup, basic file operations |
| 2 | 2026-04-09 | ~45% | TipTap integration, basic rendering |
| 3 | 2026-04-10 | ~55% | Themes, search/replace, export basic |
| 4 | 2026-04-11 | ~70% | Recovery, autosave, workspace sidebar |
| 5 | 2026-04-13 | ~75% | Transform engine start, cursor mapping investigation |
| 6 | 2026-04-14 | ~85% | **Major fixes**: rusqlite, tree-sitter, ropey, transform engine complete |
| 7 | 2026-04-14 | ~90% | **Architecture alignment**: service interfaces complete, PreferencesModal added |

---

## Appendix D: PRD Document Index

| Document | Description | Compliance |
|----------|-------------|------------|
| 01-product-definition | Executive definition, product thesis, principles | ✅ Aligned |
| 02-product-invariants | Invariants, success definition | ✅ Aligned |
| 03-scope-mvp | MVP scope, acceptance criteria | ✅ Aligned |
| 04-functional-requirements | FR-001 to FR-034, FR-035 to FR-038 | ✅ Aligned |
| 05-architecture | Non-functional requirements, architecture | ✅ Compliant |
| 06-frontend-design | Frontend design goals, product principles | ✅ Aligned |
| 07-technical-stack | Engineering standards, frontend stack | ✅ Compliant |
| 08-implementation-milestones | Testing strategy, milestones | ⚠️ Partial - tests added |
| 09-api-contracts | Frontend ↔ Backend IPC API | ⚠️ Missing ErrorCode, SaveResult types |
| 10-rust-crate-design | Repository structure, crate responsibilities | ✅ **Compliant** |
| 11-ux-requirements | Accessibility, error states | ⚠️ Partial - keyboard navigation working |
| 12-security | Security, threat model | ✅ Aligned |
| 13-governance | Release engineering, governance | ✅ Aligned |
| 14-test-plan | Comprehensive test specifications | ⚠️ Coverage improved, benchmarks pending |

---

*Specification document updated based on Iteration-7 gap analysis*

(End of file)
