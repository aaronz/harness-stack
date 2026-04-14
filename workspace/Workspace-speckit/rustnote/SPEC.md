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

## 4. Frontend Architecture

### 4.1 Editor Components

| Component | Status | Notes |
|-----------|--------|-------|
| **TipTapEditor** | ✅ **ACTIVE** | Core editor with TipTap/ProseMirror, WYSIWYG Markdown editing |
| Editor | ⚠️ **DEPRECATED** | Legacy plaintext editor - marked `@deprecated`, see G-009 |

### 4.2 Editor Deprecation (G-009)

**Editor.jsx** has been deprecated in favor of **TipTapEditor.jsx**.

**Deprecation Notice:**
```javascript
/**
 * @deprecated Use TipTapEditor.jsx instead
 * ...
 */
```

**Migration:**
- App.jsx imports and uses `TipTapEditor.jsx` as the primary editor
- No code imports `Editor.jsx` anywhere in the application
- `Editor.jsx` is preserved for historical reference only

**Why TipTapEditor:**
- WYSIWYG editing with proper cursor mapping
- Built-in undo/redo via ProseMirror
- Full Markdown live preview
- Structured document editing (headings, lists, tables, blockquotes)
- Better search highlighting integration
- Feature parity or superset of Editor.jsx functionality

### 4.3 UI Components

| Component | Status | Notes |
|-----------|--------|-------|
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
| PreferencesModal | ✅ **NEW** | Settings panel for preferences |

### 4.4 Contexts

| Context | Status | Notes |
|---------|--------|-------|
| DocumentContext | ✅ Working | |
| SettingsContext | ✅ Working | |
| SearchContext | ✅ Working | |
| ToastContext | ✅ Working | |

### 4.5 Hooks

| Hook | Status | Notes |
|------|--------|-------|
| useFileWatcher | ✅ | File watching integration |
| useAutoSaveTimer | ✅ | Autosave timer management |

---

## 5. Remaining Gaps

### 5.1 P0 - Blocking Issues (Must Fix for MVP Completion)

| Gap | FR-ID | Module | Description | Status |
|-----|-------|--------|-------------|--------|
| G-001 | FR-022 | Editor | Cursor mapping bidirectional conversion | Needs Verification |
| G-002 | FR-032 | Export | PDF export quality verification needed | Needs Verification |
| G-003 | NFR-001 | Performance | No automated performance benchmark infrastructure | **Needs Implementation** |

### 5.2 P1 - High Priority Issues

| Gap | FR-ID | Module | Description | Status |
|-----|-------|--------|-------------|--------|
| G-004 | FR-038 | Editor | Wrap transform with selection integration | Needs Verification |
| G-005 | FR-031 | Export | HTML export doesn't support linked-assets mode | Not Started |
| G-006 | FR-017 | Image | Image relative path handling for subdirectory documents | Needs Verification |
| G-007 | FR-021 | Parser | tree-sitter GFM parsing verification | Needs Verification |
| G-008 | FR-034 | Settings | Settings schema verification | Needs Verification |
| G-009 | FR-008 | Frontend | Editor.jsx deprecated in favor of TipTapEditor.jsx | **COMPLETED** |

### 5.3 P2 - Medium Priority Issues

| Gap | FR-ID | Module | Description | Status |
|-----|-------|--------|-------------|--------|
| G-010 | FR-016 | Editor | Table editing constraints documented | Monitor |
| G-011 | FR-028 | Display | Focus mode visual verification | Not Started |
| G-012 | FR-029 | Display | Typewriter mode scroll behavior | Not Started |
| G-013 | FR-018 | Paste | Paste handling improvement | Not Started |
| G-014 | FR-034 | Frontend | PreferencesModal integration verification | Needs Verification |

---

## 6. Iteration Checkpoint

```
iteration=7
phase=phase1
timestamp=1744617600
mvp_readiness=90%
remaining_p0=3
remaining_p1=5
remaining_p2=5
editor_deprecation=complete
```

---

*Specification document updated based on Iteration-7 gap analysis*
*Editor deprecation task (P1-009) completed: TipTapEditor.jsx is the sole active editor*
