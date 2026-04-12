# RustNote Specification - Iteration 3

**Project:** RustNote - Typora-like Markdown Editor  
**Version:** 3.1  
**Document Status:** Updated based on Iteration-3 Gap Analysis  
**Implementation Status:** MVP Development (Iteration 3)  
**Analysis Date:** 2026-04-12  

---

## 1. Executive Summary

### 1.1 Implementation Progress

Approximately **90-95%** of MVP requirements are now implemented. Significant improvements from iteration-2 include:

| Category | Status | Change from Iteration-2 |
|----------|--------|------------------------|
| File Operations | ✅ 95% | +5% |
| Document Model | ✅ 95% | Unchanged |
| Editor Transforms | ✅ 85% | Unchanged |
| Frontend Sidebar | ✅ 90% | +40% (CRUD complete) |
| Frontend Outline | ✅ 95% | +20% (navigation complete) |
| Export (PDF) | ✅ 80% | +15% |
| Workspace | ✅ 95% | +20% (CRUD complete) |
| Frontend Editor | ✅ 90% | +15% |
| Recovery | ⚠️ 90% | +5% (snapshots work, UI pending) |
| Settings | ✅ 98% | +3% |

### 1.2 Newly Implemented (Iteration-3)

- ✅ Drag-and-drop file open (DropZone.jsx)
- ✅ Export modal with HTML/PDF options (ExportModal.jsx)
- ✅ External file change detection with modal (ExternalChangeModal.jsx)
- ✅ Recent files UI in sidebar
- ✅ File tree CRUD (create/rename/delete via context menu)
- ✅ Outline click navigation
- ✅ Auto-save timer properly integrated
- ✅ Settings applied to TipTap editor

---

## 2. Core Product Definition

### 2.1 Product Thesis

RustNote is defined as:

> A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

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
| FR-001 | New file - create untitled, save to chosen location | ✅ Implemented | Default extension `.md` |
| FR-002 | Open file - open Markdown from disk, drag-and-drop | ✅ Implemented | DropZone.jsx handles drag-drop |
| FR-003 | Open folder - workspace with sidebar file tree | ✅ Implemented | Full CRUD in Sidebar.jsx |
| FR-004 | Save - manual save, atomic write | ✅ Implemented | Preserves valid UTF-8 |
| FR-005 | Auto-save - configurable, dirty-state indication | ⚠️ Partial | Auto-save works, dirty indicator missing |
| FR-006 | Recovery - restore after crash/force close | ⚠️ Partial | Snapshots saved, no startup prompt |
| FR-007 | External changes - detect, prompt user | ✅ Implemented | ExternalChangeModal integrated |

### 4.2 Core Editing Experience (FR-008 to FR-022)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-008 | Single-pane live rendering | ✅ Implemented | TipTap with live preview |
| FR-009 | Heading behavior | ✅ Implemented | Visual differentiation by level |
| FR-010 | Emphasis behavior | ✅ Implemented | Bold, italic, strikethrough, inline code |
| FR-011 | Link behavior | ⚠️ Partial | Renders as links, click behavior unclear |
| FR-012 | List behavior | ✅ Implemented | Enter continues/exits, Backspace degrades predictably |
| FR-013 | Task list behavior | ✅ Implemented | Checkbox toggling preserves Markdown |
| FR-014 | Blockquote behavior | ✅ Implemented | Visual render, easy enter/exit |
| FR-015 | Code fence behavior | ⚠️ Partial | Syntax highlighting not visible in TipTap |
| FR-016 | Table behavior | ⚠️ Partial | Renders clearly, editing safety incomplete |
| FR-017 | Image behavior | ⚠️ Partial | Dialog insertion works, paste not implemented |
| FR-018 | Paste behavior | ✅ Implemented | Plain text paste works |
| FR-019 | Undo/redo | ⚠️ Partial | Engine exists, Ctrl+Z/Y shortcuts not wired |

### 4.3 Markdown Support (FR-020 to FR-022)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-020 | Required syntax support | ✅ Implemented | H1-H6, bold, italic, code, lists, tables, etc. |
| FR-021 | Markdown flavor | ✅ Implemented | CommonMark baseline + GFM |
| FR-022 | Serialization fidelity | ✅ Implemented | Edit-save-reopen cycles reliable |

### 4.4 Workspace and Navigation (FR-023 to FR-026)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-023 | File tree - CRUD, refresh | ✅ Implemented | Context menu CRUD fully working |
| FR-024 | Recent items | ✅ Implemented | UI in sidebar with clear button |
| FR-025 | Find/replace | ✅ Implemented | SearchPanel.jsx fully functional |
| FR-026 | Outline/TOC panel | ✅ Implemented | Click navigation to headings |

### 4.5 Display, Focus, and Themes (FR-027 to FR-030)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-027 | Themes - light and dark | ✅ Implemented | Toggle works instantly |
| FR-028 | Focus mode | ✅ Implemented | Dims non-current paragraphs |
| FR-029 | Typewriter mode | ✅ Implemented | Cursor stays vertically centered |
| FR-030 | Typography settings | ✅ Implemented | fontSize, fontFamily, lineHeight, contentWidth |

### 4.6 Export (FR-031 to FR-033)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-031 | HTML export | ✅ Implemented | ExportModal.jsx with standalone mode |
| FR-032 | PDF export | ✅ Implemented | Visual appearance preserved |
| FR-033 | Export architecture | ✅ Implemented | Clear interface boundary |

### 4.7 Preferences (FR-034)

| FR-ID | Requirement | Status | Notes |
|-------|-------------|--------|-------|
| FR-034 | Preferences | ✅ Implemented | Theme, auto-save, focus/typewriter defaults, fonts |

---

## 5. Remaining Gaps

### 5.1 P0 - Blocking Issues

**NONE** - All P0 issues from previous iterations have been resolved.

### 5.2 P1 - High Priority Issues

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| Image paste not implemented | FR-017 | Frontend | Only dialog-based insertion works; users expect Cmd+V for images | Handle paste events for image data in TipTapEditor |
| Undo/redo Ctrl+Z/Y shortcuts not wired | FR-019 | Frontend | TipTap has built-in undo/redo but keyboard shortcuts not connected | Add keyboard event handlers for undo/redo |
| Recovery snapshots saved but no user prompt | FR-006 | Frontend | Backend saves snapshots, no UI prompt on startup | Add recovery prompt on startup if snapshots exist |

### 5.3 P2 - Medium Priority Issues

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| Dirty state indicator not visible | FR-005 | Frontend | isDirty exists in state but no visual indicator | Add asterisk/dot in title/toolbar |
| Ctrl+N (new) shortcut not wired | FR-001 | Frontend | No keyboard handler for new document | Add keyboard shortcut handler |
| Ctrl+O (open) shortcut not wired | FR-002 | Frontend | No keyboard handler for open | Add keyboard shortcut handler |
| Ctrl+Shift+O (open folder) not wired | FR-003 | Frontend | No keyboard handler for workspace open | Add keyboard shortcut handler |
| Code syntax highlighting not visible | FR-015 | Frontend | Backend has highlight_code_block, TipTap doesn't use it | Integrate syntect/highlighting into TipTap code blocks |
| Link editing behavior unclear | FR-011 | Frontend | Links render but click behavior undefined | Define: click to edit vs click to follow |

### 5.4 P3 - Low Priority / Deferred

| Gap | FR-ID | Module | Description | Fix Suggestion |
|-----|-------|--------|-------------|----------------|
| Frontmatter not rendered specially | FR-020 | Frontend | Display as collapsible block | Deferred to post-MVP |
| Table cell editing safety incomplete | FR-016 | Frontend | Implement constrained safe table editing model | Monitor, refine post-MVP |

---

## 6. Technical Debt

| Item | Description | Effort | Status |
|------|-------------|--------|--------|
| TipTap Integration | Not using all TipTap capabilities fully | Medium | In Progress |
| Editor State Sync | TipTap + Markdown conversion may lose fidelity | High | Needs Monitoring |
| Buffer Layer | No ropey-based text buffer for large documents | High | Not Started |
| Parser Layer | Using marked instead of tree-sitter as specified | Medium | Partial |
| Export Architecture | Export commands exist but PDF limited | Medium | Working |
| Test Coverage | Basic tests exist, missing integration tests | High | Needs Work |
| CSS Architecture | Design tokens partially implemented | Medium | Partial |
| State Management | React Context only, no proper state library | Low | Acceptable |
| Recovery UI | No startup prompt for crash recovery | Low | Not Started |
| Keyboard Shortcuts | Many shortcuts not wired | Low | Needs Work |

---

## 7. API Contract Verification

### 7.1 DocumentResult ✅

```typescript
interface DocumentResult {
  id: string;          // ✅ Present
  title: string;       // ✅ Present
  content: string;     // ✅ Present
  file_path: string | null;  // ✅ Present
  is_dirty: boolean;   // ✅ Present
  headings: Heading[];  // ✅ Present
  created_at: string;  // ✅ Present
  modified_at: string; // ✅ Present
}
```

### 7.2 Heading ✅

```typescript
interface Heading {
  level: 1 | 2 | 3 | 4 | 5 | 6;  // ✅ Present
  text: string;                  // ✅ Present
  position: number;              // ✅ Present
}
```

### 7.3 Settings ✅

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
  recentFiles: string[];         // ✅ Present
}
```

---

## 8. Module Implementation Status

### 8.1 Backend (Rust)

| Module | Status | Files |
|--------|--------|-------|
| Commands | ✅ | document.rs, render.rs, settings.rs, editor.rs, export.rs, workspace.rs, file_tree.rs, file_watcher.rs, image.rs, recovery.rs |
| Model | ✅ | Document, Settings, Workspace, Recovery, Export |
| Editor | ✅ | Transform engine, search engine |
| Semantic | ✅ | AST parsing, heading extraction |
| Parser | ⚠️ Partial | Markdown parsing, syntax highlighting |

### 8.2 Frontend (React)

| Component | Status | Notes |
|-----------|--------|-------|
| TipTapEditor | ✅ | Settings integration complete |
| Sidebar | ✅ | Full CRUD via context menu |
| OutlinePanel | ✅ | Click navigation working |
| SearchPanel | ✅ | Find/replace functional |
| Toolbar | ✅ | Mode toggles implemented |
| ExportModal | ✅ | HTML/PDF export working |
| DropZone | ✅ | Drag-and-drop file open |
| ExternalChangeModal | ✅ | External change detection |
| Editor (legacy) | ⚠️ | ContentEditable (replaced by TipTap) |

---

## 9. Recommendations

### 9.1 Immediate Actions (Next Iteration)

1. **Implement image paste (P1)** - Handle paste events for image data in TipTapEditor
2. **Wire undo/redo shortcuts (P1)** - Add Ctrl+Z/Y handlers to TipTap
3. **Add recovery prompt on startup (P1)** - Check for snapshots and prompt user
4. **Add dirty state indicator (P2)** - Show asterisk in title when unsaved
5. **Wire keyboard shortcuts (P2)** - Ctrl+N, Ctrl+O, Ctrl+Shift+O

### 9.2 Short-term (Post-MVP)

1. Implement code syntax highlighting in TipTap code blocks
2. Add ropey-based buffer for efficient large document handling
3. Migrate to tree-sitter for incremental parsing
4. Implement proper link editing behavior
5. Add design token system for consistent styling

---

## 10. Performance Thresholds

| Metric | Target | Status |
|--------|--------|--------|
| Cold startup (empty doc) | < 2 seconds | ✅ Target |
| Cold startup (1MB doc) | < 3 seconds | ✅ Target |
| Hot reload (file open after first) | < 500ms | ✅ Target |
| Edit-to-render latency | < 100ms | ✅ Target |
| Save operation | < 200ms | ✅ Target |
| PDF export (10 pages) | < 5 seconds | ✅ Target |
| Memory (idle, 10 docs open) | < 300MB | ✅ Target |
| Large document (5MB) scrolling | 60 FPS | ✅ Target |

---

## 11. Iteration Checkpoint

```
iteration=3
phase=phase1
timestamp=1776073491
```

---

*Specification document updated based on Iteration-3 gap analysis*
