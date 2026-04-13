# RustNote Specification - Iteration 5

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 3.1
**Document Status:** Updated based on Iteration-5 Gap Analysis
**Implementation Status:** MVP Development (Iteration 5)
**Analysis Date:** 2026-04-13

---

## 1. Executive Summary

### 1.1 Implementation Progress

Based on the Iteration-5 gap analysis, the RustNote MVP implementation is approximately **75% complete** by functional requirements. Core file operations, editor rendering, themes, search/replace, and export are functional. Critical gaps exist in transform implementations, undo/redo fidelity, table editing model, image handling, and architecture alignment with PRD specifications.

**Correction from Iteration-4:** The 90-95% progress claimed in iteration-4 was overstated. Re-analysis reveals significant gaps in Rust transform engine implementation, cursor mapping, and architecture compliance.

| Category | Status (Iteration-4) | Status (Iteration-5) | Change |
|----------|---------------------|---------------------|--------|
| File Operations (FR-001 to FR-007) | ✅ 95% | ✅ 90% | -5% |
| Core Editor (FR-008 to FR-022) | ✅ 85% | ⚠️ 60% | -25% |
| Markdown Support (FR-020 to FR-022) | ✅ 85% | ✅ 85% | Unchanged |
| Workspace & Navigation (FR-023 to FR-026) | ✅ 95% | ✅ 80% | -15% |
| Display & Themes (FR-027 to FR-030) | ✅ 85% | ✅ 85% | Unchanged |
| Export (FR-031 to FR-033) | ✅ 85% | ⚠️ 70% | -15% |
| Preferences (FR-034) | ✅ 98% | ✅ 80% | -18% |
| Recovery & Safety | ✅ 100% | ✅ 85% | -15% |

### 1.2 Newly Identified Gaps (Iteration-5)

The Iteration-5 gap analysis identified **3 P0 blocking issues**, **6 P1 high-priority issues**, and **9 P2 medium-priority issues** that were not adequately addressed in iteration-4:

| Priority | Count | Key Issues |
|----------|-------|------------|
| P0 (Blocking) | 3 | TransformEngine incomplete, Undo/redo fidelity, Settings persistence |
| P1 (High) | 6 | Cursor mapping, PDF quality, Parser architecture, Buffer architecture |
| P2 (Medium) | 9 | Table editing, HTML export modes, Frontmatter parsing, Focus mode quality |

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
| FR-004 | Save - manual save, atomic write | ✅ Implemented | Preserves valid UTF-8 |
| FR-005 | Auto-save - configurable, dirty-state indication | ⚠️ Partial | Frontend timer-based, not guaranteed if app crashes |
| FR-006 | Recovery - restore after crash/force close | ✅ Implemented | RecoveryModal UI complete, snapshots save |
| FR-007 | External changes - detect, prompt user | ✅ Implemented | ExternalChangeModal integrated |

### 4.2 Core Editing Experience (FR-008 to FR-022)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-008 | Single-pane live rendering | ✅ Implemented | TipTap with live preview |
| FR-009 | Heading behavior | ✅ Implemented | Visual differentiation by level |
| FR-010 | Emphasis behavior | ✅ Implemented | Bold, italic, strikethrough, inline code |
| FR-011 | Link behavior | ⚠️ Partial | Renders as links, click behavior partially defined |
| FR-012 | List behavior | ⚠️ Partial | Enter/Backspace basic, EnterInListItem Rust transform not implemented (G-001) |
| FR-013 | Task list behavior | ✅ Implemented | Checkbox toggling preserves Markdown |
| FR-014 | Blockquote behavior | ⚠️ Partial | Visual render works, EnterInBlockQuote Rust transform not implemented (G-001) |
| FR-015 | Code fence behavior | ⚠️ Partial | Syntax highlighting partial (P1) |
| FR-016 | Table behavior | ⚠️ Partial | TipTap default behavior, constrained but safe model per PRD |
| FR-017 | Image behavior | ⚠️ Partial | Insert/paste work, relative path handling may have issues (G-009) |
| FR-018 | Paste behavior | ⚠️ Partial | Plain text and image paste work, rich text conversion basic |
| FR-019 | Undo/redo | ⚠️ Partial | Implementation exists but fidelity issues with structural edits (G-002) |

### 4.3 Smart Transform Requirements (FR-035 to FR-038)

**NEW - Identified from Gap Analysis**

The smart editing behaviors require complete Rust TransformEngine implementation:

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-035 | Transform::EnterInListItem - Enter key behavior inside list items | ❌ Not Implemented | Rust `TransformEngine::apply_enter_in_list_item` missing (G-001) |
| FR-036 | Transform::EnterInBlockQuote - Enter key behavior inside blockquotes | ❌ Not Implemented | Rust `TransformEngine::apply_enter_in_blockquote` missing (G-001) |
| FR-037 | Transform::EnterInHeading - Enter key behavior inside headings | ❌ Not Implemented | Rust `TransformEngine::apply_enter_in_heading` missing (G-001) |
| FR-038 | Transform::Wrap - Wrap selected text with syntax markers | ❌ Not Implemented | `Transform::Wrap` variant not implemented (G-008) |

### 4.4 Markdown Support (FR-020 to FR-022)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-020 | Required syntax support | ✅ Implemented | H1-H6, bold, italic, code, lists, tables, etc. |
| FR-021 | Markdown flavor | ✅ Implemented | CommonMark baseline + GFM via comrak |
| FR-022 | Serialization fidelity | ⚠️ Partial | Round-trip mostly works, prosemirror-markdown bridge deferred |

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
| FR-028 | Focus mode | ⚠️ Partial | Basic CSS implementation, should materially reduce visual distraction (G-013) |
| FR-029 | Typewriter mode | ⚠️ Partial | Basic implementation, may not keep cursor at vertical center (G-014) |
| FR-030 | Typography settings | ✅ Implemented | fontSize, fontFamily, lineHeight, contentWidth |

### 4.7 Export (FR-031 to FR-033)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-031 | HTML export | ⚠️ Partial | Working, missing "linked-assets mode" option (G-011) |
| FR-032 | PDF export | ⚠️ Partial | Working but basic - code blocks, images not properly rendered (G-005) |
| FR-033 | Export architecture | ✅ Implemented | Clear interface boundary |

### 4.8 Preferences (FR-034)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-034 | Preferences | ⚠️ Partial | Settings persistence uses file-based JSON instead of rusqlite (G-003) |

---

## 5. Remaining Gaps

### 5.1 P0 - Blocking Issues (Must Fix for MVP Completion)

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| G-001 | FR-012, FR-014, FR-035, FR-036, FR-037 | Editor | `TransformType::EnterInListItem`, `EnterInBlockQuote`, `EnterInHeading` not implemented in Rust `TransformEngine` - frontend has switch cases but Rust backend returns error/unimplemented | Complete Rust `TransformEngine::apply_*` methods for all transform types |
| G-002 | FR-019 | Editor | Undo/redo implementation exists but may have fidelity issues with structural edits - no regression tests for round-trip undo | Add comprehensive undo/redo tests; verify transaction log fidelity |
| G-003 | FR-034 | Settings | Settings persistence uses file-based JSON (`settings.json`) instead of rusqlite as specified in PRD-10 | Replace with rusqlite-based settings service per PRD architecture |

### 5.2 P1 - High Priority Issues

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| G-004 | FR-008 | Editor | Cursor mapping between TipTap DOM and source Markdown is incomplete - `build_cursor_mapping()` creates linear mapping but doesn't account for HTML tag insertion | Implement bidirectional cursor mapping with AST-aware offset calculation |
| G-005 | FR-032 | Export | PDF export uses basic text extraction + manual layout rather than proper HTML-to-PDF pipeline; code blocks, images not properly rendered | Integrate proper HTML-to-PDF rendering (e.g., html2pdf, puppeteer, or webview print) |
| G-006 | FR-021 | Parser | Parser uses `comrak` directly instead of tree-sitter for incremental parsing as specified in PRD-10 | Implement tree-sitter-based incremental parsing wrapper around comrak |
| G-007 | NFR-003 | Buffer | `buffer` module exists but doesn't use `ropey` for efficient large document handling as specified | Implement ropey-based text buffer for position-to-offset mapping |
| G-008 | FR-038 | Editor | `Transform::Wrap` (for wrapping selected text) not implemented | Add Wrap transform for text selection wrapping |
| G-009 | FR-017 | Image | Image paste/drop uses `save_image_from_base64_cmd` but relative path handling may not work correctly when document is in subdirectory | Verify and fix relative path calculation based on document location |

### 5.3 P2 - Medium Priority Issues

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| G-010 | FR-016 | Editor | Table editing uses TipTap default behavior - constrained but safe model per PRD note, but no custom cell-model editing | Document table editing constraints; ensure no data loss |
| G-011 | FR-031 | Export | HTML export produces self-contained HTML but doesn't support "linked-assets mode" as per FR-031 | Add option for linked vs inline asset export |
| G-012 | FR-020 | Parser | Frontmatter parsing exists but is basic - doesn't handle all YAML frontmatter variations | Improve frontmatter parsing to handle complex YAML |
| G-013 | FR-028 | Display | Focus mode implementation is basic CSS (`body.classList.add('focus-mode')`) - should materially reduce visual distraction per FR-028 | Enhance focus mode to properly dim/hide non-current paragraphs |
| G-014 | FR-029 | Display | Typewriter mode implementation exists but may not keep cursor at vertical center during navigation | Verify and fix typewriter mode scroll behavior |
| G-015 | FR-018 | Editor | Paste behavior converts to Markdown on best-effort but doesn't handle all rich text paste cases | Improve paste handling for common rich text formats |
| G-016 | NFR-005 | Services | Architecture has `services/` module but doesn't expose `DocumentService`, `EditorService`, `WorkspaceService`, `ExportService`, `SettingsService`, `RecoveryService` as narrow interfaces per PRD-10 | Refactor to expose proper service interfaces |
| G-017 | FR-005 | Recovery | Autosave uses frontend timer (`useAutoSaveTimer`) - not guaranteed to fire if app crashes before timeout | Consider Rust-side autosave with debounce, or ensure frontend timer fires reliably |
| G-018 | FR-015 | Display | Code fence syntax highlighting uses `SyntaxHighlighter` (syntect-based) but Shiki is specified in PRD | Verify `SyntaxHighlighter` implementation matches PRD specification |

### 5.4 P3 - Low Priority / Deferred

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| G-019 | FR-016 | Editor | Table cell editing safety incomplete | Monitor, PRD acceptable - no action for MVP |
| G-020 | Security | Security | Fuzz testing not implemented | Deferred post-MVP |
| G-021 | Frontend | Frontend | IME composition handling not tested | Test post-MVP |

---

## 6. Technical Debt

| Item | Category | Description | Impact | PRD Reference |
|------|----------|-------------|--------|---------------|
| TD-001 | Architecture | Buffer module doesn't use `ropey` - large document handling may be inefficient | Performance | PRD 31.1 |
| TD-002 | Architecture | Parser doesn't use tree-sitter - no incremental parsing | Performance | PRD 31.2 |
| TD-003 | Architecture | Settings uses file-based JSON instead of `rusqlite` | Data integrity | PRD 31.8 |
| TD-004 | Code | `semantic/position.rs` - `CursorMapping` is defined but may not be used correctly for bidirectional mapping | Editor UX | N/A |
| TD-005 | Code | `editor/commands.rs` - `Command` enum exists but editor operations may not use it consistently | Maintainability | N/A |
| TD-006 | Code | Frontend has `Editor.jsx` and `TipTapEditor.jsx` - unclear distinction, potential duplication | Maintainability | N/A |
| TD-007 | Testing | No unit tests for `TransformEngine` beyond basic cases | Reliability | N/A |
| TD-008 | Testing | No integration tests for file watcher + editor interaction | Reliability | N/A |
| TD-009 | Testing | Visual regression tests exist but baseline may not be current | Quality | N/A |

---

## 7. Architecture Compliance

### 7.1 PRD Section Compliance

| Component | PRD Specifies | Current Implementation | Compliance |
|-----------|---------------|------------------------|------------|
| Editor Core | Tiptap/ProseMirror | React + contenteditable | ⚠️ MVP Acceptable |
| Markdown Bridge | prosemirror-markdown | marked.js | ⚠️ MVP Acceptable |
| Parser | tree-sitter + comrak | pulldown-cmark only | ❌ Not Compliant |
| Text Buffer | ropey | JS string | ❌ Not Compliant |
| Code Highlighting | Shiki + syntect | syntect (backend only) | ⚠️ Partial |
| Export | printpdf | printpdf | ✅ Compliant |
| Settings | rusqlite | File-based JSON | ❌ Not Compliant |
| Service Interfaces | DocumentService, EditorService, etc. | commands/ module | ❌ Not Compliant |

**Note:** PRD Section 15.1 explicitly states "Current MVP Architecture uses React 18 with contenteditable" - some deviations are acceptable for MVP phase. However, the buffer, parser, and settings architecture issues represent significant technical debt that should be addressed.

---

## 8. API Contract Verification

### 8.1 DocumentResult

```typescript
interface DocumentResult {
  id: string;          // ✅ Present (UUID)
  title: string;       // ✅ Present
  content: string;     // ✅ Present
  file_path: string | null;  // ✅ Present
  is_dirty: boolean;   // ✅ Present
  headings: Heading[];  // ⚠️ Missing in serialization, present in model
  created_at: string;  // ✅ Present
  modified_at: string; // ✅ Present
}
```

**Issue:** `headings` field present in model but may not serialize correctly.

### 8.2 Heading

```typescript
interface Heading {
  level: 1 | 2 | 3 | 4 | 5 | 6;  // ✅ Present
  text: string;                  // ✅ Present
  position: number;              // ✅ Present
}
```

### 8.3 Settings

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
  contentWidth: number;          // ✅ Present
  recentFiles: string[];          // ✅ Present
}
```

**Issue:** Nested `editor` vs flat structure - PRD specifies flat structure which is implemented.

### 8.4 TransformType

```typescript
type TransformType = 
  | { Enter: null }
  | { Backspace: null }
  | { Tab: null }
  | { ShiftTab: null }
  | { Wrap: { before: string, after: string } };

// ⚠️ Backend has Enter/Backspace/Tab commands
// ❌ Frontend returns error for EnterInListItem, EnterInBlockQuote, EnterInHeading
// ❌ Wrap variant not implemented
```

### 8.5 Missing Type Definitions

| Type | Location | Status |
|------|----------|--------|
| `ErrorCode` enum | Should be in `commands/mod.rs` | ❌ Missing |
| `SaveResult` | `save_document` returns `()` | ❌ Should return `SaveResult` |
| `ExportResult` | `export_*` returns `()` | ❌ Should return `ExportResult` |
| `WorkspaceResult` | `list_workspace` returns `Workspace` directly | ❌ Should be `WorkspaceResult` |

---

## 9. Module Implementation Status

### 9.1 Backend (Rust)

| Module | Status | Files |
|--------|--------|-------|
| Commands | ⚠️ Partial | document.rs, render.rs, settings.rs, editor.rs, export.rs, workspace.rs, file_tree.rs, file_watcher.rs, image.rs, recovery.rs, mod.rs |
| Model | ⚠️ Partial | Document, Settings, Workspace, Recovery, Export, Image |
| Editor | ⚠️ Partial | Transform engine incomplete (G-001, G-008), cursor mapping incomplete (G-004) |
| Semantic | ✅ | AST parsing, heading extraction |
| Parser | ⚠️ Partial | Markdown parsing, syntax highlighting (pulldown-cmark MVP) |
| Buffer | ❌ | Module exists but NOT using ropey (G-007) |
| Services | ❌ | Not exposing narrow service interfaces per PRD-10 (G-016) |
| Settings | ❌ | Using file-based JSON, not rusqlite (G-003) |

### 9.2 Frontend (React)

| Component | Status | Notes |
|-----------|--------|-------|
| TipTapEditor | ⚠️ | Core editor, incomplete cursor mapping (G-004) |
| Editor | ⚠️ | Legacy/alternative editor (contenteditable) |
| Sidebar | ✅ | Full CRUD via context menu |
| OutlinePanel | ✅ | Click navigation working |
| SearchPanel | ✅ | Find/replace functional |
| Toolbar | ✅ | Dirty indicator asterisk visible |
| ExportModal | ✅ | HTML/PDF export working |
| DropZone | ✅ | Drag-and-drop file open |
| ExternalChangeModal | ✅ | External change detection |
| RecoveryModal | ✅ | Full UI with list, recover, delete |
| CodeBlockHighlight | ⚠️ | Component exists, not fully wired |
| LinkPopover | ✅ | Working |
| FrontmatterBlock | ✅ | Working |

### 9.3 Contexts

| Context | Status | Notes |
|---------|--------|-------|
| DocumentContext | ✅ Working | |
| SettingsContext | ⚠️ | Mapping issues between Rust/JS types |
| SearchContext | ✅ Working | |
| ToastContext | ✅ Working | |

---

## 10. Test Coverage Analysis

### 10.1 Test Files Present

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

### 10.2 Missing Test Coverage (Critical Gaps)

| Area | Priority | Gap |
|------|----------|-----|
| Undo/redo round-trip fidelity | P0 | No regression tests for round-trip undo (G-002) |
| TransformEngine | P0 | No comprehensive tests for all transform types |
| Cursor mapping accuracy | P1 | No tests for bidirectional cursor mapping |
| Transform fidelity for all edge cases | P1 | No tests for EnterInListItem, EnterInBlockQuote, etc. |
| File watcher race conditions | P2 | No integration tests |
| External change detection | P2 | No integration tests |

---

## 11. Performance NFR Compliance

| NFR | Target | Current Status | Notes |
|-----|--------|----------------|-------|
| Cold start (empty) | < 2s | Unknown | Not measured |
| Cold start (1MB doc) | < 3s | Unknown | Not measured |
| Hot file open | < 500ms | Unknown | Not measured |
| Keystroke → render | < 100ms | Unknown | Not measured |
| Save operation | < 200ms | Unknown | Not measured |
| PDF export (10 pages) | < 5s | Unknown | Not measured |
| Memory (idle, 10 docs) | < 300MB | Unknown | Not measured |
| Large doc scroll | 60 FPS | Unknown | Not measured |

**Note:** Performance thresholds are specified in PRD Section 11 but no automated performance testing infrastructure is in place. Without ropey-based buffer (G-007), large document performance may degrade significantly.

---

## 12. Security Analysis

### 12.1 Security Features Implemented

| Feature | Status | Implementation |
|---------|--------|----------------|
| Path traversal prevention | ✅ | Paths validated in Rust backend |
| HTML sanitization | ✅ | comrak handles sanitization |
| Export sanitization | ✅ | HTML exports are self-contained |
| No telemetry | ✅ | No analytics in MVP |

### 12.2 Security Gaps

| Issue | Severity | Status |
|-------|----------|--------|
| Fuzz testing | P2 | Not implemented |
| cargo-audit in CI | P2 | Not verified |
| XSS in link URLs | P1 | Not explicitly sanitized |

---

## 13. Accessibility Analysis

| Requirement | Status | Implementation |
|-------------|--------|----------------|
| Keyboard navigation | ✅ | All shortcuts wired |
| Focus indicators | ✅ | CSS outlines present |
| Color contrast | ⚠️ | Not measured |
| Screen reader support | ⚠️ | Not tested |
| WCAG AA compliance | ⚠️ | Not verified |

---

## 14. Recommendations

### 14.1 Immediate Actions (P0 - Next Iteration Must Address)

1. **Complete Transform Implementation (FR-035 to FR-038)** - Implement all `TransformType` variants in Rust `TransformEngine::apply_*` methods, specifically:
   - `EnterInListItem` - Continue list, exit on empty
   - `EnterInBlockQuote` - Continue quote, exit on empty
   - `EnterInHeading` - Create new heading or paragraph
   - `Wrap` - Wrap selection with markers

2. **Fix Undo/Redo Fidelity (FR-019)** - Add comprehensive tests and fix any fidelity issues with structural edits

3. **Switch to rusqlite Settings (FR-034)** - Replace file-based settings persistence with rusqlite per PRD-10 architecture

### 14.2 Short-term Actions (P1)

1. **Cursor Mapping (FR-008)** - Implement bidirectional cursor mapping between TipTap DOM and source with AST-aware offset calculation
2. **Ropey Buffer (NFR-003)** - Implement ropey-based buffer for efficient large document handling
3. **Tree-sitter Parser (FR-021)** - Add tree-sitter incremental parsing wrapper around comrak
4. **PDF Export Quality (FR-032)** - Integrate proper HTML-to-PDF rendering

### 14.3 Medium-term Actions (P2)

1. **Focus/Typewriter Mode Polish (FR-028, FR-029)** - Enhance visual effects to meet PRD expectations
2. **Image Path Handling (FR-017)** - Verify and fix relative path calculation
3. **Paste Handling (FR-018)** - Improve rich text paste conversion
4. **HTML Export Modes (FR-031)** - Add linked vs inline asset export option
5. **Architecture Alignment (NFR-005)** - Refactor to expose proper service interfaces

---

## 15. Iteration Checkpoint

```
iteration=5
phase=phase1
timestamp=1744569600
```

---

## Appendix A: Gap-to-FR Mapping

| Gap ID | Related FRs | Description |
|--------|-------------|-------------|
| G-001 | FR-012, FR-014, FR-035, FR-036, FR-037 | TransformEngine incomplete |
| G-002 | FR-019 | Undo/redo fidelity |
| G-003 | FR-034 | Settings persistence architecture |
| G-004 | FR-008 | Cursor mapping |
| G-005 | FR-032 | PDF export quality |
| G-006 | FR-021 | Parser architecture |
| G-007 | NFR-003 | Buffer architecture |
| G-008 | FR-038 | Wrap transform |
| G-009 | FR-017 | Image path handling |
| G-010 | FR-016 | Table editing |
| G-011 | FR-031 | HTML export modes |
| G-012 | FR-020 | Frontmatter parsing |
| G-013 | FR-028 | Focus mode quality |
| G-014 | FR-029 | Typewriter mode quality |
| G-015 | FR-018 | Paste handling |
| G-016 | NFR-005 | Service interfaces |
| G-017 | FR-005 | Autosave reliability |
| G-018 | FR-015 | Syntax highlighter |

---

## Appendix B: PRD Document Index

| Document | Description | Compliance |
|----------|-------------|------------|
| 01-product-definition | Executive definition, product thesis, principles | ✅ Aligned |
| 02-product-invariants | Invariants, success definition | ✅ Aligned |
| 03-scope-mvp | MVP scope, acceptance criteria | ✅ Aligned |
| 04-functional-requirements | FR-001 to FR-034 | ⚠️ FR-035 to FR-038 missing from PRD |
| 05-architecture | Non-functional requirements, architecture | ⚠️ Partial |
| 06-frontend-design | Frontend design goals, product principles | ✅ Aligned |
| 07-technical-stack | Engineering standards, frontend stack | ⚠️ MVP uses React+contenteditable |
| 08-implementation-milestones | Testing strategy, milestones | ⚠️ Missing benchmarks |
| 09-api-contracts | Frontend ↔ Backend IPC API | ⚠️ Missing ErrorCode, SaveResult types |
| 10-rust-crate-design | Repository structure, crate responsibilities | ⚠️ Not fully implemented |
| 11-ux-requirements | Accessibility, error states | ⚠️ Partial |
| 12-security | Security, threat model | ⚠️ Partial |
| 13-governance | Release engineering, governance | ✅ Aligned |
| 14-test-plan | Comprehensive test specifications | ⚠️ Missing critical tests |

---

*Specification document updated based on Iteration-5 gap analysis*

(End of file - total 753 lines)