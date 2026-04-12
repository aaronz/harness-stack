# RustNote Task List v2.0

**Project:** RustNote - Typora-like Markdown Editor  
**Based on:** PRD v3.1 + Gap Analysis (Iteration 2, 2026-04-12)  
**Status:** MVP Development - Iteration 2  
**Task Version:** v2  
**Updated:** 2026-04-12

---

## Task Priority Legend

| Priority | Description | Must Complete Before Release |
|----------|-------------|---------------------------|
| **P0** | Blocking issues | ✅ YES (All resolved in iteration-1) |
| **P1** | High priority issues | ✅ YES |
| **P2** | Medium priority enhancements | ❌ NO (Post-MVP) |

---

## Section A: P0 Tasks (✅ COMPLETED - Iteration-1 Resolution)

---

### A.1 Find/Replace UI ✅ COMPLETED

**FR Reference:** FR-025  
**Estimated Effort:** 2-3 days  
**Actual Status:** ✅ COMPLETE

- [x] **A.1.1** Create SearchPanel component (`www/src/components/SearchPanel.jsx`)
- [x] **A.1.2** Add search state management to Editor
- [x] **A.1.3** Implement search logic (regex-based)
- [x] **A.1.4** Wire keyboard shortcuts (Ctrl+F, Ctrl+H, F3, Shift+F3, Escape)
- [x] **A.1.5** Highlight matches in editor
- [x] **A.1.6** Implement replace functionality (single and all)
- [x] **A.1.7** Add search panel CSS/styling

**Verification:** Manual test Ctrl+F, type query, navigate matches, replace ✅

---

### A.2 Focus Mode ✅ COMPLETED

**FR Reference:** FR-028  
**Estimated Effort:** 1 day  
**Actual Status:** ✅ COMPLETE

- [x] **A.2.1** Add focusMode to Settings context
- [x] **A.2.2** Add focus mode class to document body
- [x] **A.2.3** Track active paragraph/block element
- [x] **A.2.4** Implement paragraph de-emphasis CSS
- [x] **A.2.5** Keyboard shortcut (Ctrl+Shift+F) - optional

**Verification:** Enable focus mode, verify non-active paragraphs are dimmed ✅

---

### A.3 Typewriter Mode ✅ COMPLETED

**FR Reference:** FR-029  
**Estimated Effort:** 1 day  
**Actual Status:** ✅ COMPLETE

- [x] **A.3.1** Add typewriterMode to Settings context
- [x] **A.3.2** Add typewriter mode class to document body
- [x] **A.3.3** Implement scroll centering on cursor move
- [x] **A.3.4** Keyboard shortcut (Ctrl+Shift+T) - optional

**Verification:** Enable typewriter mode, scroll manually, verify cursor stays centered ✅

---

### A.4 Auto-Save Timer ✅ COMPLETED

**FR Reference:** FR-005  
**Estimated Effort:** 1 day  
**Actual Status:** ✅ COMPLETE

- [x] **A.4.1** Access auto-save settings
- [x] **A.4.2** Implement auto-save timer hook (`useAutoSaveTimer.js`)
- [x] **A.4.3** Reset timer on each edit (debounce)
- [x] **A.4.4** Show "Saving..." indicator
- [x] **A.4.5** Clear dirty flag after auto-save

**Verification:** Make changes, wait for autoSaveInterval, verify auto-save fires ✅

---

## Section B: P1 Tasks (High Priority - MUST COMPLETE)

---

### B.1 Rich Text Paste Handler ✅ COMPLETED

**FR Reference:** FR-018  
**Estimated Effort:** 1-2 days  
**Actual Status:** ✅ COMPLETE

- [x] **B.1.1** Add paste event handler to Editor
- [x] **B.1.2** Detect clipboard content type
- [x] **B.1.3** Integrate HTML-to-Markdown converter (TurndownService)
- [x] **B.1.4** Insert converted Markdown at cursor
- [x] **B.1.5** Add fallback for plain text paste

**Verification:** Copy rich text from browser, paste in editor, verify Markdown output ✅

---

### B.2 DocumentResult Headings Field ✅ COMPLETED

**FR Reference:** FR-API / FR-026  
**Estimated Effort:** 0.5 day  
**Actual Status:** ✅ COMPLETE

- [x] **B.2.1** Add Heading struct to Rust model
- [x] **B.2.2** Add headings field to Document struct
- [x] **B.2.3** Populate headings on document load
- [x] **B.2.4** Update DocumentResult serialization

**Verification:** Open document, check API response includes headings array ✅

---

### B.3 Settings Missing Fields ✅ COMPLETED

**FR Reference:** FR-034  
**Estimated Effort:** 0.5 day  
**Actual Status:** ✅ COMPLETE

- [x] **B.3.1** Add fields to Rust Settings struct (focusMode, typewriterMode, outlineVisible)
- [x] **B.3.2** Update Settings JSON serialization
- [x] **B.3.3** Update frontend Settings context
- [x] **B.3.4** Add UI toggles for new settings

**Verification:** Change settings, reload app, verify settings persist ✅

---

### B.4 File Tree CRUD

**FR Reference:** FR-023  
**Estimated Effort:** 2-3 days  
**Actual Status:** ✅ DONE

- [x] **B.4.1** Add Tauri commands for file operations
  - `create_file(path: String, name: String) -> Result<FileInfo>`
  - `create_folder(path: String, name: String) -> Result<FileInfo>`
  - `rename_item(path: String, new_name: String) -> Result<FileInfo>`
  - `delete_item(path: String) -> Result<()>`

- [x] **B.4.2** Implement Rust command handlers
  - Validate paths (no traversal)
  - Create/rename/delete on filesystem
  - Return updated file info

- [x] **B.4.3** Add context menu to Sidebar component
  - Right-click on file/folder shows menu
  - Menu items: New File, New Folder, Rename, Delete
  - Disable inappropriate actions (can't delete root)

- [x] **B.4.4** Wire menu actions to Tauri commands
  - Show prompt for new name (rename)
  - Show confirmation for delete
  - Refresh file tree after operation

- [x] **B.4.5** Handle errors gracefully
  - Show toast/notification on failure
  - Keep UI consistent

**Dependencies:** File system access in Tauri  
**Verification:** Create file, rename file, delete file from sidebar

---

### B.5 Undo/Redo Connection ⚠️ PARTIAL

**FR Reference:** FR-019  
**Estimated Effort:** 1 day  
**Actual Status:** ⚠️ PARTIAL - Shortcuts not wired

- [x] **B.5.1** Verify Rust undo/redo engine exists
- [x] **B.5.2** TipTap has built-in undo/redo
- [ ] **B.5.3** Add keyboard shortcuts to TipTap Editor
  - Ctrl+Z: Call undo
  - Ctrl+Y or Ctrl+Shift+Z: Call redo

**Dependencies:** TipTap integration  
**Verification:** Make edits, press Ctrl+Z, verify undo; press Ctrl+Y, verify redo

---

### B.6 Drag-and-Drop File Open

**FR Reference:** FR-002  
**Estimated Effort:** 0.5 day  
**Actual Status:** ✅ DONE

- [x] **B.6.1** Create DropZone overlay component
  - Full-window overlay
  - Visual indicator (dashed border, "Drop file here")
  - Appears on dragenter, disappears on dragleave/drop

- [x] **B.6.2** Handle drag events on window
  - `dragenter`: Show overlay, prevent default
  - `dragover`: Show overlay, prevent default
  - `dragleave`: Hide overlay (if leaving window)
  - `drop`: Hide overlay, process file

- [x] **B.6.3** Extract file from drop event
  - Get `event.dataTransfer.files`
  - Validate file is `.md` extension
  - Get file path

- [x] **B.6.4** Call existing open file logic
  - Use existing `openFile` handler
  - Handle multiple files (open first)

- [x] **B.6.5** Add visual feedback
  - Highlight drop zone on dragover
  - Reject non-Markdown files visually

**Dependencies:** Existing file open logic  
**Verification:** Drag .md file onto app, verify it opens

---

## Section C: P2 Tasks (Medium Priority - POST-MVP)

---

### C.1 Settings Applied to TipTap Editor

**FR Reference:** FR-030  
**Estimated Effort:** 1 day  
**Actual Status:** ✅ DONE

- [x] **C.1.1** Read contentWidth from settings
- [x] **C.1.2** Apply max-width to TipTap editor container
  - `max-width: ${contentWidth}px`
  - Center editor horizontally

- [x] **C.1.3** Read lineHeight from settings
- [x] **C.1.4** Apply line-height to TipTap content
  - `line-height: ${lineHeight}`

- [x] **C.1.5** Read fontSize from settings
- [x] **C.1.6** Apply font-size to TipTap content
  - `font-size: ${fontSize}px`

- [x] **C.1.7** Read fontFamily from settings
- [x] **C.1.8** Apply font-family to TipTap content

**Status:** COMPLETED

---

### C.2 Recent Files UI

**FR Reference:** FR-024  
**Estimated Effort:** 1 day  
**Actual Status:** ⚠️ STORAGE EXISTS, UI NOT IMPLEMENTED

- [ ] **C.2.1** Access recent files storage
  - Read from existing storage mechanism
  - Parse list of recent file paths

- [ ] **C.2.2** Create RecentFiles component
  - List view of recent files
  - Show file name and path
  - Click to open

- [ ] **C.2.3** Add to sidebar or menu
  - Integrate in sidebar OR as dropdown menu
  - Show last 10 recent files

- [ ] **C.2.4** Clear recent files option
  - Button to clear history

**Status:** POST-MVP

---

### C.3 Image Paste

**FR Reference:** FR-017  
**Estimated Effort:** 1-2 days  
**Actual Status:** ✅ IMPLEMENTED

- [x] **C.3.1** Detect image data in paste
  - Check clipboard for image types (image/png, image/jpeg)

- [x] **C.3.2** Handle image paste
  - Save image to workspace assets folder
  - Generate unique filename
  - Insert markdown image reference `![](path/to/image.png)`

- [x] **C.3.3** Handle image drag-and-drop
  - Similar to paste but from drop event
  - Save to assets, insert reference

- [x] **C.3.4** Show error for unsupported images
  - Toast notification if save fails

**Status:** POST-MVP

---

### C.4 Outline Click Navigation

**FR Reference:** FR-026  
**Estimated Effort:** 0.5 day  
**Actual Status:** ✅ DONE

- [x] **C.4.1** Add click handler to outline items in OutlinePanel
- [x] **C.4.2** Get heading position (character offset) from headings array
- [x] **C.4.3** Scroll TipTap editor to heading position
- [x] **C.4.4** Optionally set cursor to heading start

**Status:** POST-MVP

---

### C.5 Export UI

**FR Reference:** FR-031, FR-032  
**Estimated Effort:** 1-2 days  
**Actual Status:** ⚠️ BACKEND EXISTS, UI NOT IMPLEMENTED

- [ ] **C.5.1** Create ExportModal component
  - Modal dialog with format selection
  - HTML (standalone or linked-assets mode)
  - PDF with page size options

- [ ] **C.5.2** Wire export format selection
- [ ] **C.5.3** Call backend export commands
- [ ] **C.5.4** Show progress indicator during export
- [ ] **C.5.5** Show success/error notification
- [ ] **C.5.6** Add export button to Toolbar

**Status:** POST-MVP

---

### C.6 File Watcher UI Integration

**FR Reference:** FR-007  
**Estimated Effort:** 1 day  
**Status:** ⚠️ PARTIAL - FILE WATCHER EXISTS, UI NOT INTEGRATED

- [ ] **C.6.1** Connect file watcher events to frontend
- [ ] **C.6.2** Show notification when external change detected
- [ ] **C.6.3** Provide user options: Reload, Keep Current, Compare Later
- [ ] **C.6.4** Handle file watcher errors gracefully

**Status:** POST-MVP

---

### C.7 Task Checkbox Toggle

**FR Reference:** FR-013  
**Estimated Effort:** 0.5 day  
**Status:** ⚠️ PARTIAL - RENDERING WORKS, TOGGLE MAY NEED VERIFICATION

- [ ] **C.7.1** Add click handler to task list checkboxes in TipTap
- [ ] **C.7.2** Toggle markdown state
  - `- [ ]` → `- [x]` on click
  - `- [x]` → `- [ ]` on click
- [ ] **C.7.3** Update document state
  - Replace old task syntax with new

**Status:** POST-MVP

---

## Section D: Verification Checklist

### P0/P1 Verification (Completed Items)

- [x] **Find/Replace:**
  - [x] Ctrl+F opens search panel
  - [x] Typing query highlights matches
  - [x] F3/Shift+F3 navigates matches
  - [x] Ctrl+H shows replace field
  - [x] Replace replaces current match
  - [x] Replace All replaces all matches
  - [x] Escape closes panel

- [x] **Focus Mode:**
  - [x] Toggle in settings enables focus mode
  - [x] Non-active paragraphs appear dimmed
  - [x] Active paragraph remains full opacity
  - [x] Transition is smooth

- [x] **Typewriter Mode:**
  - [x] Toggle in settings enables typewriter mode
  - [x] Cursor stays vertically centered when typing
  - [x] Scrolling is smooth
  - [x] Cursor centers on navigation

- [x] **Auto-Save:**
  - [x] Auto-save triggers after `autoSaveInterval` ms
  - [x] "Saving..." indicator shows during save
  - [x] Dirty flag clears after auto-save
  - [x] Disabling auto-save in settings prevents auto-save

- [x] **Rich Paste:** HTML converts to Markdown
- [x] **Headings API:** DocumentResult.headings is populated
- [x] **Settings:** focusMode/typewriterMode/outlineVisible persist

### P1 Verification (Remaining)

- [x] **File Tree:** Can create, rename, delete files from sidebar
- [ ] **Undo/Redo:** Ctrl+Z undoes, Ctrl+Y redoes
- [ ] **Drag-Drop:** Dragging .md file onto app opens it

### P2 Verification (Remaining)

- [ ] Settings (fontSize, lineHeight, contentWidth) applied to TipTap
- [ ] Recent files appear in sidebar/menu
- [ ] Pasting image inserts markdown reference
- [ ] Clicking outline item scrolls to heading
- [ ] Export modal triggers HTML/PDF export
- [ ] File watcher prompts on external changes
- [ ] Clicking task checkbox toggles completion

---

## Section E: File Change Summary

### New Files (Iteration-2)

| File | Purpose | Priority |
|------|---------|----------|
| `www/src/components/DropZone.jsx` | Drag-and-drop overlay | P1 |
| `www/src/components/ExportModal.jsx` | Export dialog | P2 |
| `www/src/components/RecentFiles.jsx` | Recent files list | P2 |
| `www/src/hooks/useAutoSaveTimer.js` | Auto-save hook | P0 ✅ |

### Files to Modify (Iteration-2)

| File | Changes | Priority |
|------|---------|----------|
| `src-tauri/src/commands/` | New commands for file ops | P1 |
| `www/src/components/Sidebar.jsx` | Context menu, CRUD operations | P1 |
| `www/src/components/TipTapEditor.jsx` | Settings application, undo/redo shortcuts | P1, P2 |
| `www/src/components/OutlinePanel.jsx` | Click navigation | P2 |
| `www/src/components/Toolbar.jsx` | Export button | P2 |
| `www/src/contexts/SettingsContext.jsx` | Recent files exposure | P2 |

---

## Section F: Iteration-2 Checkpoint

```
iteration=2
phase=phase2
timestamp=嵌入时间戳
```

**Summary of Changes from v1 to v2:**
- All P0 tasks marked complete (A.1-A.4)
- B.1-B.3 (Rich Paste, Headings API, Settings fields) marked complete
- B.4 (File Tree CRUD) remains as primary P1
- B.5 (Undo/Redo) marked partial (TipTap has engine, shortcuts not wired)
- B.6 (Drag-Drop) remains P1
- Added C.1-C.7 for P2 items (Settings, Recent Files, Image Paste, Outline Nav, Export UI, File Watcher, Task Checkbox)

---

*Task list updated based on Gap Analysis dated 2026-04-12 (Iteration 2)*