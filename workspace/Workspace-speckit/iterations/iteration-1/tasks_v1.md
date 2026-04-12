# RustNote Task List v1.0

**Project:** RustNote - Typora-like Markdown Editor  
**Based on:** PRD v3.1 + Gap Analysis (2026-04-11)  
**Status:** MVP Development - Iteration 7  
**Task Version:** v1  
**Updated:** 2026-04-11

---

## Task Priority Legend

| Priority | Description | Must Complete Before Release |
|----------|-------------|---------------------------|
| **P0** | Blocking issues | ✅ YES |
| **P1** | High priority issues | ✅ YES |
| **P2** | Medium priority enhancements | ❌ NO (Post-MVP) |

---

## Section A: P0 Tasks (MVP Critical - MUST COMPLETE)

---

### A.1 Find/Replace UI

**FR Reference:** FR-025  
**Estimated Effort:** 2-3 days

- [x] **A.1.1** Create SearchPanel component (`www/src/components/SearchPanel.jsx`)
  - Search input field
  - Replace input field
  - "Replace" and "Replace All" buttons
  - Case-sensitive toggle checkbox
  - Match count display
  - Close button

- [x] **A.1.2** Add search state management to Editor
  - `searchQuery` state
  - `replaceQuery` state
  - `caseSensitive` state
  - `currentMatchIndex` state
  - `isSearchOpen` state
  - `isReplaceOpen` state

- [x] **A.1.3** Implement search logic
  - Regex-based search using current query
  - Case sensitivity option
  - Match list generation
  - Position tracking for each match

- [x] **A.1.4** Wire keyboard shortcuts
  - Ctrl+F: Open search panel (focus search input)
  - Ctrl+H: Open search panel with replace field
  - Ctrl+G / F3: Find next match
  - Shift+F3 / Shift+Ctrl+G: Find previous match
  - Enter: Find next (when search focused)
  - Escape: Close search panel

- [ ] **A.1.5** Implement highlight matches in editor
  - Add `<mark>` class for current match
  - Add different class for other matches
  - Scroll to keep matches visible
  - Clear highlights when search closes

- [x] **A.1.6** Implement replace functionality
  - Replace single: Replace at `currentMatchIndex`
  - Replace all: Replace all matches
  - Update document state after replace
  - Update match list after replace

- [ ] **A.1.7** Add search panel CSS/styling
  - Position: fixed top of editor
  - Theme-aware styling (light/dark)
  - Proper z-index
  - Animation for open/close

**Dependencies:** None (standalone component)  
**Verification:** Manual test Ctrl+F, type query, navigate matches, replace

---

### A.2 Focus Mode

**FR Reference:** FR-028  
**Estimated Effort:** 1 day

- [x] **A.2.1** Add focusMode to Settings context
  - Add `focusMode` boolean state
  - Add `setFocusMode` setter
  - Persist to Tauri settings

- [x] **A.2.2** Add focus mode class to document body
  - Toggle `.focus-mode` class on body when enabled
  - Ensure class removed when disabled

- [x] **A.2.3** Track active paragraph/block element
  - On cursor position change, identify current block
  - Store reference to active block element
  - Update on selection change

- [ ] **A.2.4** Implement paragraph de-emphasis CSS
  - Non-active blocks: `opacity: 0.4` (or similar)
  - Active block: `opacity: 1.0`
  - Smooth transition (200ms)
  - Exclude certain elements (code blocks, tables)

- [ ] **A.2.5** Add keyboard shortcut (optional)
  - Ctrl+Shift+F: Toggle focus mode

**Dependencies:** Settings context (existing)  
**Verification:** Enable focus mode, verify non-active paragraphs are dimmed

---

### A.3 Typewriter Mode

**FR Reference:** FR-029  
**Estimated Effort:** 1 day

- [x] **A.3.1** Add typewriterMode to Settings context
  - Add `typewriterMode` boolean state
  - Add `setTypewriterMode` setter
  - Persist to Tauri settings

- [ ] **A.3.2** Add typewriter mode class to document body
  - Toggle `.typewriter-mode` class when enabled

- [ ] **A.3.3** Implement scroll centering on cursor move
  - On cursor/selection change:
    - Get cursor line element
    - Call `scrollIntoView({ block: 'center', behavior: 'smooth' })`
  - Debounce to 50ms for performance

- [ ] **A.3.4** Add keyboard shortcut (optional)
  - Ctrl+Shift+T: Toggle typewriter mode

**Dependencies:** Settings context (existing)  
**Verification:** Enable typewriter mode, scroll manually, verify cursor stays centered

---

### A.4 Auto-Save Timer

**FR Reference:** FR-005  
**Estimated Effort:** 1 day

- [ ] **A.4.1** Access auto-save settings
  - Read `autoSave` (boolean) from Settings context
  - Read `autoSaveInterval` (number, ms) from Settings context

- [ ] **A.4.2** Implement auto-save timer hook
  - Watch `isDirty` state
  - Watch `autoSaveInterval` value
  - `useEffect` with setTimeout/debounce pattern:
    ```javascript
    useEffect(() => {
      if (!autoSave || !isDirty) return;
      const timer = setTimeout(() => {
        saveDocument();
      }, autoSaveInterval);
      return () => clearTimeout(timer);
    }, [isDirty, content, autoSave, autoSaveInterval]);
    ```

- [ ] **A.4.3** Reset timer on each edit
  - Dependency array includes `content`
  - Timer resets on every keystroke (debounced)

- [ ] **A.4.4** Show "Saving..." indicator
  - Add `isSaving` state
  - Show indicator during save operation
  - Clear after save completes

- [ ] **A.4.5** Clear dirty flag after auto-save
  - On successful save, set `setIsDirty(false)`

**Dependencies:** Settings context (autoSave, autoSaveInterval)  
**Verification:** Make changes, wait for autoSaveInterval, verify auto-save fires

---

## Section B: P1 Tasks (High Priority - MUST COMPLETE)

---

### B.1 Rich Text Paste Handler

**FR Reference:** FR-018  
**Estimated Effort:** 1-2 days

- [x] **B.1.1** Add paste event handler to Editor
  - Capture `onPaste` event
  - Prevent default browser paste

- [ ] **B.1.2** Detect clipboard content type
  - Check `event.clipboardData.types` for 'text/html'
  - Fallback to plain text if no HTML

- [ ] **B.1.3** Integrate HTML-to-Markdown converter
  - Install turndown package (or similar)
  - Convert HTML content to Markdown
  - Handle edge cases (empty paste, malformed HTML)

- [ ] **B.1.4** Insert converted Markdown at cursor
  - Use existing insert logic
  - Update document state

- [ ] **B.1.5** Add fallback for plain text paste
  - If HTML parsing fails, use plain text
  - Ensure no malformed Markdown created

**Dependencies:** turndown library  
**Verification:** Copy rich text from browser, paste in editor, verify Markdown output

---

### B.2 DocumentResult Headings Field

**FR Reference:** FR-API / FR-026  
**Estimated Effort:** 0.5 day

- [ ] **B.2.1** Add Heading struct to Rust model
  ```rust
  pub struct Heading {
      level: u8,       // 1-6
      text: String,
      position: usize, // character offset
  }
  ```

- [ ] **B.2.2** Add headings field to Document struct
  - `headings: Vec<Heading>`

- [ ] **B.2.3** Populate headings on document load
  - Reuse existing semantic layer heading extraction
  - Call during document open/save

- [ ] **B.2.4** Update DocumentResult serialization
  - Ensure headings included in JSON response

**Dependencies:** Semantic layer (heading extraction exists)  
**Verification:** Open document, check API response includes headings array

---

### B.3 Settings Missing Fields

**FR Reference:** FR-034  
**Estimated Effort:** 0.5 day

- [ ] **B.3.1** Add fields to Rust Settings struct
  - `focus_mode: bool`
  - `typewriter_mode: bool`
  - `outline_visible: bool`

- [ ] **B.3.2** Update Settings JSON serialization
  - Add fields to `serde` Serialize/Deserialize

- [ ] **B.3.3** Update frontend Settings context
  - Add `focusMode`, `typewriterMode`, `outlineVisible` state
  - Add setters for each
  - Add to load/save settings flow

- [ ] **B.3.4** Add UI toggles for new settings (optional)
  - Add to existing settings panel or toolbar

**Dependencies:** None  
**Verification:** Change settings, reload app, verify settings persist

---

### B.4 File Tree CRUD

**FR Reference:** FR-023  
**Estimated Effort:** 2-3 days

- [ ] **B.4.1** Add Tauri commands for file operations
  - `create_file(path: String, name: String) -> Result<FileInfo>`
  - `create_folder(path: String, name: String) -> Result<FileInfo>`
  - `rename_item(path: String, new_name: String) -> Result<FileInfo>`
  - `delete_item(path: String) -> Result<()>`

- [ ] **B.4.2** Implement Rust command handlers
  - Validate paths (no traversal)
  - Create/rename/delete on filesystem
  - Return updated file info

- [ ] **B.4.3** Add context menu to Sidebar component
  - Right-click on file/folder shows menu
  - Menu items: New File, New Folder, Rename, Delete
  - Disable inappropriate actions (can't delete root)

- [ ] **B.4.4** Wire menu actions to Tauri commands
  - Show prompt for new name (rename)
  - Show confirmation for delete
  - Refresh file tree after operation

- [ ] **B.4.5** Handle errors gracefully
  - Show toast/notification on failure
  - Keep UI consistent

**Dependencies:** File system access in Tauri  
**Verification:** Create file, rename file, delete file from sidebar

---

### B.5 Undo/Redo Connection

**FR Reference:** FR-019  
**Estimated Effort:** 1 day

- [ ] **B.5.1** Verify Rust undo/redo engine exists
  - Check `src-tauri/src/editor/undo.rs`
  - Understand command structure

- [ ] **B.5.2** Add Tauri commands for undo/redo
  - `undo() -> Result<DocumentContent>`
  - `redo() -> Result<DocumentContent>`

- [ ] **B.5.3** Implement Rust command handlers
  - Call into existing undo engine
  - Return new document state

- [ ] **B.5.4** Add keyboard shortcuts to Editor
  - Ctrl+Z: Call undo command
  - Ctrl+Y or Ctrl+Shift+Z: Call redo command

- [ ] **B.5.5** Update editor state after undo/redo
  - Replace current content with returned content
  - Update dirty state appropriately

**Dependencies:** Rust undo engine (already exists)  
**Verification:** Make edits, press Ctrl+Z, verify undo works; press Ctrl+Y, verify redo works

---

### B.6 Drag-and-Drop File Open

**FR Reference:** FR-002  
**Estimated Effort:** 0.5 day

- [ ] **B.6.1** Create DropZone overlay component
  - Full-window overlay
  - Visual indicator (dashed border, "Drop file here")
  - Appears on dragenter, disappears on dragleave/drop

- [ ] **B.6.2** Handle drag events on window
  - `dragenter`: Show overlay, prevent default
  - `dragover`: Show overlay, prevent default
  - `dragleave`: Hide overlay (if leaving window)
  - `drop`: Hide overlay, process file

- [ ] **B.6.3** Extract file from drop event
  - Get `event.dataTransfer.files`
  - Validate file is `.md` extension
  - Get file path

- [ ] **B.6.4** Call existing open file logic
  - Use existing `openFile` handler
  - Handle multiple files (open first, or open all)

- [ ] **B.6.5** Add visual feedback
  - Highlight drop zone on dragover
  - Reject non-Markdown files visually

**Dependencies:** Existing file open logic  
**Verification:** Drag .md file onto app, verify it opens

---

## Section C: P2 Tasks (Medium Priority - POST-MVP)

---

### C.1 Recent Files UI

**FR Reference:** FR-024  
**Estimated Effort:** 1 day

- [ ] **C.1.1** Access recent files storage
  - Read from existing storage mechanism
  - Parse list of recent file paths

- [ ] **C.1.2** Create RecentFiles component
  - List view of recent files
  - Show file name and path
  - Click to open

- [ ] **C.1.3** Add to sidebar or menu
  - Integrate in sidebar OR as dropdown menu
  - Show last 10 recent files

- [ ] **C.1.4** Clear recent files option
  - Button to clear history

**Status:** POST-MVP

---

### C.2 Image Paste

**FR Reference:** FR-017  
**Estimated Effort:** 1-2 days

- [ ] **C.2.1** Detect image data in paste
  - Check clipboard for image types (image/png, image/jpeg)

- [ ] **C.2.2** Handle image paste
  - Save image to workspace assets folder
  - Generate unique filename
  - Insert markdown image reference `![](path/to/image.png)`

- [ ] **C.2.3** Handle image drag-and-drop
  - Similar to paste but from drop event
  - Save to assets, insert reference

- [ ] **C.2.4** Show error for unsupported images
  - Toast notification if save fails

**Status:** POST-MVP

---

### C.3 Content Width Control

**FR Reference:** FR-030  
**Estimated Effort:** 0.5 day

- [ ] **C.3.1** Read contentWidth from settings
- [ ] **C.3.2** Apply max-width to editor container
  - `max-width: ${contentWidth}px`
  - Center editor horizontally

**Status:** POST-MVP

---

### C.4 Line Height Setting Applied

**FR Reference:** FR-030  
**Estimated Effort:** 0.5 day

- [ ] **C.4.1** Read lineHeight from settings
- [ ] **C.4.2** Apply to editor content
  - `line-height: ${lineHeight}`

**Status:** POST-MVP

---

### C.5 Task Checkbox Toggle

**FR Reference:** FR-013  
**Estimated Effort:** 0.5 day

- [ ] **C.5.1** Add click handler to task list checkboxes
- [ ] **C.5.2** Toggle markdown state
  - `- [ ]` → `- [x]` on click
  - `- [x]` → `- [ ]` on click
- [ ] **C.5.3** Update document state
  - Replace old task syntax with new

**Status:** POST-MVP

---

## Section D: Verification Checklist

### P0 Verification

- [x] **Find/Replace:**
  - [x] Ctrl+F opens search panel
  - [ ] Typing query highlights matches
  - [ ] F3/Shift+F3 navigates matches
  - [x] Ctrl+H shows replace field
  - [x] Replace replaces current match
  - [x] Replace All replaces all matches
  - [x] Escape closes panel

- [ ] **Focus Mode:**
  - [ ] Toggle in settings enables focus mode
  - [ ] Non-active paragraphs appear dimmed
  - [ ] Active paragraph remains full opacity
  - [ ] Transition is smooth

- [ ] **Typewriter Mode:**
  - [ ] Toggle in settings enables typewriter mode
  - [ ] Cursor stays vertically centered when typing
  - [ ] Scrolling is smooth
  - [ ] Cursor centers on navigation

- [ ] **Auto-Save:**
  - [ ] Auto-save triggers after `autoSaveInterval` ms
  - [ ] "Saving..." indicator shows during save
  - [ ] Dirty flag clears after auto-save
  - [ ] Disabling auto-save in settings prevents auto-save

### P1 Verification

- [ ] **Rich Paste:** Copy from browser, paste in editor, HTML converts to Markdown
- [ ] **Headings API:** Open file, check `DocumentResult.headings` is populated
- [ ] **Settings:** focusMode/typewriterMode/outlineVisible persist after restart
- [ ] **File Tree:** Can create, rename, delete files from sidebar
- [ ] **Undo/Redo:** Ctrl+Z undoes, Ctrl+Y redoes
- [ ] **Drag-Drop:** Dragging .md file onto app opens it

### P2 Verification

- [ ] Recent files appear in sidebar/menu
- [ ] Pasting image inserts markdown reference
- [ ] Content width respects settings
- [ ] Line height respects settings
- [ ] Clicking task checkbox toggles completion

---

## Section E: File Change Summary

### New Files

| File | Purpose | Priority |
|------|---------|----------|
| `www/src/components/SearchPanel.jsx` | Find/Replace UI | P0 |
| `www/src/contexts/SearchContext.jsx` | Search panel state management | P0 |
| `www/src/components/DropZone.jsx` | Drag-and-drop overlay | P1 |
| `www/src/components/RecentFiles.jsx` | Recent files list | P2 |

### Modified Files

| File | Changes | Priority |
|------|---------|----------|
| `www/src/App.jsx` | SearchPanel integration, keyboard shortcuts | P0 |
| `www/src/components/Editor.jsx` | Search, focus, typewriter, auto-save, undo/redo, drop zone | P0, P1 |
| `www/src/components/Sidebar.jsx` | Context menu, CRUD operations | P1 |
| `www/src/contexts/SettingsContext.jsx` | New settings fields | P0, P1 |
| `www/src/styles/editor.css` | Focus mode, typewriter mode styles | P0 |
| `src-tauri/src/model/settings.rs` | New settings fields | P1 |
| `src-tauri/src/model/document.rs` | Headings field | P1 |
| `src-tauri/src/commands/` | New commands for file ops, undo/redo | P1 |

---

*Task list created based on Gap Analysis dated 2026-04-11*