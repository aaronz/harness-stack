# RustNote Implementation Plan v1.0

**Project:** RustNote - Typora-like Markdown Editor  
**Based on:** PRD v3.1 + Gap Analysis (2026-04-11)  
**Status:** MVP Development - Iteration 7  
**Implementation Progress:** ~65-70% Complete  
**Plan Version:** v1  
**Updated:** 2026-04-11

---

## 1. Executive Summary

This plan addresses the remaining gaps to achieve MVP completion. Based on the gap analysis, **4 P0 blocking issues** must be resolved first, followed by **6 P1 high-priority issues**, then **5 P2 medium-priority issues**.

### Priority Order

| Priority | Issues | Impact |
|----------|--------|--------|
| **P0** | Find/Replace UI, Focus Mode, Typewriter Mode, Auto-save Timer | Blocking MVP release |
| **P1** | Rich Paste, Headings API, Settings fields, File Tree CRUD, Undo/Redo, Drag-Drop | Core UX completeness |
| **P2** | Recent Files UI, Image Paste, Content Width, Line Height, Task Checkboxes | Enhanced UX |

---

## 2. P0 Implementation Plan (Blocking - MVP Critical)

### 2.1 Find/Replace UI (FR-025)

**Current State:** No search panel or keyboard shortcuts  
**Target State:** Ctrl+F opens search panel, Ctrl+H opens replace, find next/previous works

**Implementation Steps:**
1. Create search panel component (`SearchPanel.jsx`)
2. Add search input with case-sensitive toggle
3. Add replace input field with "Replace" and "Replace All" buttons
4. Implement Tauri command for backend search (or use frontend regex)
5. Wire Ctrl+F / Ctrl+H keyboard shortcuts
6. Highlight matches in editor
7. Navigate between matches

**Files to Modify:**
- `www/src/components/SearchPanel.jsx` (new)
- `www/src/components/Editor.jsx` (add keyboard handlers, highlight logic)
- `src-tauri/src/commands/` (optional backend search)

**Estimated Effort:** 2-3 days

---

### 2.2 Focus Mode (FR-028)

**Current State:** CSS class `.focus-mode` exists but no logic  
**Target State:** Non-active paragraphs de-emphasized when focus mode enabled

**Implementation Steps:**
1. Add `.focus-mode` body class toggle via React state
2. Track cursor position (paragraph/block element)
3. Add CSS to de-emphasize non-active blocks (opacity, background)
4. Connect to Settings toggle

**Files to Modify:**
- `www/src/components/Editor.jsx` (cursor tracking, class toggle)
- `www/src/contexts/SettingsContext.jsx` (add focusMode state)
- `www/src/styles/editor.css` (add focus mode styles)

**Estimated Effort:** 1 day

---

### 2.3 Typewriter Mode (FR-029)

**Current State:** CSS class `.typewriter-mode` exists but no logic  
**Target State:** Active line remains vertically centered during editing

**Implementation Steps:**
1. Add `.typewriter-mode` body class toggle
2. Track cursor line position on every keystroke/movement
3. Implement scroll centering logic (scrollIntoView with center option)
4. Debounce scroll updates for performance
5. Connect to Settings toggle

**Files to Modify:**
- `www/src/components/Editor.jsx` (scroll centering logic)
- `www/src/contexts/SettingsContext.jsx` (add typewriterMode state)
- `www/src/styles/editor.css` (add typewriter mode styles)

**Estimated Effort:** 1 day

---

### 2.4 Auto-Save Timer (FR-005)

**Current State:** Settings exist for autoSave and autoSaveInterval, but no timer running  
**Target State:** Document auto-saves after configured debounce interval when dirty

**Implementation Steps:**
1. Add `useEffect` hook in Editor that watches `isDirty` and `autoSaveInterval`
2. Implement debounce timer (reset on each edit, fire after interval)
3. Call save command when timer fires
4. Show "Saving..." indicator during auto-save
5. Reset dirty flag after successful auto-save

**Files to Modify:**
- `www/src/components/Editor.jsx` (add auto-save timer logic)
- `www/src/contexts/SettingsContext.jsx` (ensure autoSave settings available)

**Estimated Effort:** 1 day

---

## 3. P1 Implementation Plan (High Priority)

### 3.1 Rich Text Paste Handler (FR-018)

**Current State:** Only plain text paste works  
**Target State:** HTML from clipboard converted to Markdown on paste

**Implementation Steps:**
1. Add paste event handler in Editor.jsx
2. Detect if clipboard contains HTML
3. Use turndown or equivalent for HTML→Markdown conversion
4. Insert converted Markdown at cursor position
5. Fallback to plain text if HTML parsing fails

**Estimated Effort:** 1-2 days

---

### 3.2 DocumentResult Headings Field (FR-API)

**Current State:** Document struct missing headings array  
**Target State:** DocumentResult includes `headings: Heading[]`

**Implementation Steps:**
1. Add `Heading` struct to Rust model
2. Add `headings` field to `Document` struct
3. Populate headings on document load (reuse semantic layer)
4. Update `DocumentResult` serialization

**Files to Modify:**
- `src-tauri/src/model/document.rs`
- `src-tauri/src/semantic/` (heading extraction logic exists)

**Estimated Effort:** 0.5 day

---

### 3.3 Settings Missing Fields (FR-034)

**Current State:** Settings missing focusMode, typewriterMode, outlineVisible  
**Target State:** Settings struct includes all required fields

**Implementation Steps:**
1. Add `focus_mode`, `typewriter_mode`, `outline_visible` to Rust Settings struct
2. Update Settings JSON serialization/deserialization
3. Update frontend Settings context to expose these fields
4. Add Settings UI toggles (or ensure they're in existing UI)

**Files to Modify:**
- `src-tauri/src/model/settings.rs`
- `www/src/contexts/SettingsContext.jsx`

**Estimated Effort:** 0.5 day

---

### 3.4 File Tree CRUD (FR-023)

**Current State:** File tree display-only, no create/rename/delete  
**Target State:** Full file/folder CRUD from sidebar

**Implementation Steps:**
1. Add Tauri commands: `create_file`, `create_folder`, `rename_item`, `delete_item`
2. Add context menu in Sidebar for CRUD operations
3. Wire menu actions to Tauri commands
4. Refresh file tree after operations

**Files to Modify:**
- `src-tauri/src/commands/` (new commands)
- `src-tauri/src/services/workspace.rs`
- `www/src/components/Sidebar.jsx` (add context menu)

**Estimated Effort:** 2-3 days

---

### 3.5 Undo/Redo Connection (FR-019)

**Current State:** Undo engine exists in Rust, not connected to frontend  
**Target State:** Ctrl+Z / Ctrl+Y work for undo/redo

**Implementation Steps:**
1. Add Tauri commands: `undo`, `redo`
2. Wire Ctrl+Z / Ctrl+Y keyboard handlers in Editor
3. Call Rust undo/redo commands on shortcut
4. Update editor state after undo/redo

**Files to Modify:**
- `src-tauri/src/commands/` (add undo/redo commands)
- `src-tauri/src/editor/` (undo engine exists)
- `www/src/components/Editor.jsx` (keyboard handlers)

**Estimated Effort:** 1 day

---

### 3.6 Drag-and-Drop File Open (FR-002)

**Current State:** No drop zone handlers  
**Target State:** Files can be opened by dragging onto the app

**Implementation Steps:**
1. Add drop zone overlay component
2. Handle dragenter, dragover, dragleave, drop events
3. Extract file paths from drop event
4. Call existing open file logic
5. Show visual feedback during drag

**Files to Modify:**
- `www/src/components/DropZone.jsx` (new)
- `www/src/components/Editor.jsx` (integrate drop zone)

**Estimated Effort:** 0.5 day

---

## 4. P2 Implementation Plan (Medium Priority)

### 4.1 Recent Files UI (FR-024)
- Add recent files list to sidebar or menu
- Wire to open file logic
- **Estimated Effort:** 1 day

### 4.2 Image Paste (FR-017)
- Handle paste events with image data
- Save image to workspace, insert markdown reference
- **Estimated Effort:** 1-2 days

### 4.3 Content Width Control (FR-030)
- Apply `max-width` from settings to editor
- Wire to settings change
- **Estimated Effort:** 0.5 day

### 4.4 Line Height Setting Applied (FR-030)
- Apply `line-height` CSS from settings to editor content
- **Estimated Effort:** 0.5 day

### 4.5 Task Checkbox Toggle (FR-013)
- Add click handler on task list checkboxes
- Toggle `- [ ]` ↔ `- [x]` in source markdown
- **Estimated Effort:** 0.5 day

---

## 5. Technical Debt (Post-MVP)

| Item | Description | Priority | Effort |
|------|-------------|----------|--------|
| Frontend Editor Architecture | Migrate to Tiptap/ProseMirror | High | High |
| Buffer Layer | Add ropey for large docs | High | High |
| Parser Layer | Add tree-sitter + comrak | Medium | Medium |
| CSS Architecture | Design token system | Medium | Medium |

---

## 6. Implementation Order

```
Week 1 (P0 - Blocking):
├── Find/Replace UI (2-3 days)
├── Focus Mode (1 day)
├── Typewriter Mode (1 day)
└── Auto-Save Timer (1 day)

Week 2 (P1 - Core UX):
├── Rich Text Paste (1-2 days)
├── DocumentResult Headings (0.5 day)
├── Settings Missing Fields (0.5 day)
├── File Tree CRUD (2-3 days)
├── Undo/Redo Connection (1 day)
└── Drag-and-Drop (0.5 day)

Week 3 (P2 - Polish):
├── Recent Files UI (1 day)
├── Image Paste (1-2 days)
├── Content Width Control (0.5 day)
├── Line Height Applied (0.5 day)
└── Task Checkbox Toggle (0.5 day)

Buffer Days: 2-3 days for integration/testing
```

---

## 7. Success Criteria

### MVP Release Criteria

- [ ] Find/Replace works with Ctrl+F / Ctrl+H
- [ ] Focus mode de-emphasizes non-active paragraphs
- [ ] Typewriter mode keeps cursor centered
- [ ] Auto-save triggers after configured interval
- [ ] Rich text paste converts to Markdown
- [ ] Settings persists all preferences
- [ ] File tree allows CRUD operations
- [ ] Undo/Redo works with Ctrl+Z/Ctrl+Y
- [ ] Drag-and-drop opens files

### Post-MVP Quality Bar

- [ ] All P2 features complete
- [ ] Test coverage increased
- [ ] Performance meets budgets
- [ ] No security vulnerabilities

---

*Plan created based on Gap Analysis dated 2026-04-11*