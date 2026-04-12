# RustNote Implementation Plan v2.0

**Project:** RustNote - Typora-like Markdown Editor  
**Based on:** PRD v3.1 + Gap Analysis (Iteration 2, 2026-04-12)  
**Status:** MVP Development - Iteration 2  
**Implementation Progress:** ~80-85% Complete  
**Plan Version:** v2  
**Updated:** 2026-04-12

---

## 1. Executive Summary

This plan addresses the remaining gaps to achieve MVP completion. Based on the Iteration-2 gap analysis, **all P0 blocking issues from iteration-1 have been resolved**. 

**Current Priority Order:**

| Priority | Issues | Status |
|----------|--------|--------|
| **P0** | All resolved (Find/Replace, Focus, Typewriter, Auto-save) | ✅ Complete |
| **P1** | File Tree CRUD, Drag-and-Drop File Open | Remaining P1 |
| **P2** | Settings→TipTap, Recent Files UI, Undo/Redo shortcuts, Image Paste, Outline Click Nav, Export UI | Enhanced UX |

---

## 2. P0 Status (Iteration-1 Resolution)

The following P0 issues from iteration-1 have been **successfully resolved**:

| Issue | Resolution | Status |
|-------|------------|--------|
| Find/Replace UI not implemented | SearchPanel.jsx fully implements find/next/prev, replace, replace-all | ✅ Complete |
| Focus mode non-functional | Editor.jsx updateActiveBlock() + CSS de-emphasis working | ✅ Complete |
| Typewriter mode non-functional | Editor.jsx scrollToCursor() + scroll-behavior: smooth working | ✅ Complete |
| Auto-save not automatically triggered | useAutoSaveTimer hook implemented with debounce | ✅ Complete |
| Paste from rich text not handled | TurndownService in Editor.jsx converts HTML to Markdown | ✅ Complete |
| DocumentResult missing headings | document.rs now includes `headings: Vec<Heading>` | ✅ Complete |
| Settings incomplete | settings.rs now includes focusMode, typewriterMode, outlineVisible | ✅ Complete |

---

## 3. P1 Implementation Plan (High Priority - MVP Critical)

### 3.1 File Tree CRUD (FR-023)

**Current State:** File tree display-only, no create/rename/delete  
**Target State:** Full file/folder CRUD from sidebar context menu

**Implementation Steps:**
1. Add Tauri commands: `create_file`, `create_folder`, `rename_item`, `delete_item`
2. Implement Rust command handlers with path validation (prevent traversal)
3. Add context menu in Sidebar component for CRUD operations
4. Wire menu actions to Tauri commands
5. Show prompts for rename confirmation, delete confirmation
6. Refresh file tree after operations
7. Handle errors gracefully with toast notifications

**Files to Modify:**
- `src-tauri/src/commands/` (new commands for file operations)
- `src-tauri/src/services/workspace.rs` (workspace service logic)
- `www/src/components/Sidebar.jsx` (add context menu)

**Estimated Effort:** 2-3 days

---

### 3.2 Drag-and-Drop File Open (FR-002)

**Current State:** No drop zone handlers  
**Target State:** Files can be opened by dragging onto the app

**Implementation Steps:**
1. Create DropZone overlay component
2. Handle dragenter, dragover, dragleave, drop events on window
3. Extract file paths from drop event
4. Validate file is `.md` extension
5. Call existing open file logic
6. Show visual feedback during drag

**Files to Modify:**
- `www/src/components/DropZone.jsx` (new)
- `www/src/App.jsx` or `www/src/components/Editor.jsx` (integrate drop zone)

**Estimated Effort:** 0.5 day

---

## 4. P2 Implementation Plan (Medium Priority)

### 4.1 Settings Applied to TipTap Editor (FR-030)

**Current State:** Settings store values (fontSize, fontFamily, lineHeight, contentWidth) but TipTap ignores them  
**Target State:** TipTap editor respects all typography settings

**Implementation Steps:**
1. Read settings values from SettingsContext
2. Apply `font-size` style to TipTap editor content
3. Apply `font-family` style to TipTap editor content
4. Apply `line-height` style to TipTap editor content
5. Apply `max-width` / content width constraint to editor container

**Files to Modify:**
- `www/src/components/TipTapEditor.jsx` (apply styles from settings)
- `www/src/components/Editor.jsx` (if using original editor)

**Estimated Effort:** 1 day

---

### 4.2 Recent Files UI (FR-024)

**Current State:** `recent_files` exists in Settings storage but no UI  
**Target State:** Recent files accessible from sidebar or menu

**Implementation Steps:**
1. Create RecentFiles component
2. Read recent files from settings storage
3. Display list with file name and path
4. Click to open file
5. Add "Clear recent files" option
6. Integrate in sidebar or as dropdown menu

**Files to Modify:**
- `www/src/components/RecentFiles.jsx` (new)
- `www/src/components/Sidebar.jsx` (integrate)
- `www/src/contexts/SettingsContext.jsx` (expose recent files)

**Estimated Effort:** 1 day

---

### 4.3 Undo/Redo Keyboard Shortcuts (FR-019)

**Current State:** TipTap has built-in undo/redo but Ctrl+Z/Ctrl+Y shortcuts not wired  
**Target State:** Ctrl+Z undoes, Ctrl+Y redoes

**Implementation Steps:**
1. Add keyboard event handlers for Ctrl+Z and Ctrl+Y
2. Call TipTap's built-in undo/redo commands
3. Test with various edit operations

**Files to Modify:**
- `www/src/components/TipTapEditor.jsx` (add keyboard handlers)

**Estimated Effort:** 0.5 day

---

### 4.4 Image Paste (FR-017)

**Current State:** Only insert via dialog, paste not implemented  
**Target State:** Paste images directly into editor

**Implementation Steps:**
1. Handle paste events for image data (image/png, image/jpeg)
2. Save image to workspace assets folder
3. Generate unique filename
4. Insert markdown image reference `![](path/to/image.png)`
5. Show error toast if save fails

**Files to Modify:**
- `www/src/components/TipTapEditor.jsx` (paste handler)
- `src-tauri/src/commands/` (image save command)

**Estimated Effort:** 1-2 days

---

### 4.5 Outline Click Navigation (FR-026)

**Current State:** Panel shows headings but clicking does nothing  
**Target State:** Click heading → scroll to that position

**Implementation Steps:**
1. Add click handler to outline items
2. On click, find heading position in document
3. Scroll editor to that position
4. Optionally set cursor to heading start

**Files to Modify:**
- `www/src/components/OutlinePanel.jsx` (add click handler)
- `www/src/components/TipTapEditor.jsx` (scroll-to-position method)

**Estimated Effort:** 0.5 day

---

### 4.6 Export UI (FR-031, FR-032)

**Current State:** Backend export commands exist but no frontend trigger  
**Target State:** Modal/dialog to trigger HTML/PDF export

**Implementation Steps:**
1. Create ExportModal component
2. Add export format selection (HTML, PDF)
3. Add export options (standalone vs linked assets for HTML)
4. Wire to existing backend export commands
5. Show progress indicator during export
6. Show success/error notification

**Files to Modify:**
- `www/src/components/ExportModal.jsx` (new)
- `www/src/App.jsx` or `www/src/components/Toolbar.jsx` (add export button)
- `src-tauri/src/commands/export.rs` (existing, wire to UI)

**Estimated Effort:** 1-2 days

---

### 4.7 File Watcher UI Integration (FR-007)

**Current State:** File watcher exists in backend, external changes not communicated to user  
**Target State:** Prompt user when file changes externally

**Implementation Steps:**
1. Connect file watcher events to frontend
2. Show notification/dialog when external change detected
3. Provide options: Reload, Keep Current, Compare Later

**Files to Modify:**
- `src-tauri/src/services/file_watcher.rs` (emit events to frontend)
- `www/src/contexts/DocumentContext.jsx` (handle file watcher events)

**Estimated Effort:** 1 day

---

## 5. Technical Debt (Post-MVP)

| Item | Description | Priority | Effort |
|------|-------------|----------|--------|
| TipTap Migration | Using TipTap but not fully utilizing its capabilities | High | Medium |
| Editor State Sync | Dual editor (contenteditable + TipTap) causes confusion | High | High |
| Buffer Layer | Add ropey for large document handling | High | High |
| Parser Layer | Using pulldown-cmark + comrak, not tree-sitter + comrak | Medium | Medium |
| Export Architecture | Export commands not integrated with frontend | Medium | Started |
| CSS Architecture | Inline styles scattered, no design token system | Medium | Medium |

---

## 6. Implementation Order

```
Iteration-2 Phase 2 (Current):

Week 1 (P1 - Core Workspace):
├── File Tree CRUD (2-3 days)
└── Drag-and-Drop (0.5 day)

Week 2 (P2 - Polish):
├── Settings → TipTap (1 day)
├── Recent Files UI (1 day)
├── Undo/Redo Shortcuts (0.5 day)
├── Outline Click Nav (0.5 day)
├── Export UI (1-2 days)
└── File Watcher UI (1 day)

Buffer Days: 2-3 days for integration/testing
```

---

## 7. Success Criteria

### MVP Release Criteria

- [x] Find/Replace works with Ctrl+F / Ctrl+H (✅ iteration-1)
- [x] Focus mode de-emphasizes non-active paragraphs (✅ iteration-1)
- [x] Typewriter mode keeps cursor centered (✅ iteration-1)
- [x] Auto-save triggers after configured interval (✅ iteration-1)
- [x] Rich text paste converts to Markdown (✅ iteration-1)
- [x] Document.headings available in API (✅ iteration-1)
- [x] Settings persists all preferences (✅ iteration-1)
- [ ] File tree allows CRUD operations
- [ ] Drag-and-drop opens files
- [ ] Settings (fontSize, lineHeight, contentWidth) applied to TipTap
- [ ] Undo/Redo works with Ctrl+Z/Ctrl+Y
- [ ] Outline click navigation scrolls to heading
- [ ] Export UI triggers HTML/PDF export

### Post-MVP Quality Bar

- [ ] Test coverage increased
- [ ] Performance meets budgets
- [ ] No security vulnerabilities

---

## 8. Checkpoint

```
iteration=2
phase=phase2
timestamp=嵌入时间戳
```

---

*Plan updated based on Gap Analysis dated 2026-04-12 (Iteration 2)*