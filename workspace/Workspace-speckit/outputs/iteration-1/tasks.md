# RustNote Implementation Task List

> Generated from implementation plan - Dependency-ordered task list for agentic execution

## Phase 1: Foundation (Project Setup)

### T001 [P] Initialize Tauri Project Structure
**Description:** Create complete Tauri 2.x project with Cargo workspace, tauri.conf.json, and basic HTML frontend
**Files:** 
- `rustnote/Cargo.toml`
- `rustnote/src-tauri/Cargo.toml`
- `rustnote/src-tauri/tauri.conf.json`
- `rustnote/src-tauri/src/main.rs`
- `rustnote/src-tauri/src/lib.rs`
- `rustnote/src/index.html`
**Dependencies:** None (foundation)
**Steps:**
- [X] T001.1 Create Rust project root Cargo.toml with workspace
- [X] T001.2 Create src-tauri/Cargo.toml with Tauri 2.x dependencies
- [X] T001.3 Create tauri.conf.json with window config
- [X] T001.4 Create src-tauri/src/main.rs entry point
- [X] T001.5 Create src-tauri/src/lib.rs with basic Tauri setup
- [X] T001.6 Create basic src/index.html skeleton
- [X] T001.7 Verify project builds: `cd rustnote && cargo build --release`
- [X] T001.8 Commit: "feat: initialize Tauri project structure"

---

### T002 [P] Set Up Logging and Error Handling
**Description:** Add logging infrastructure and panic handler to Rust backend
**Files:**
- `rustnote/src-tauri/src/lib.rs`
- `rustnote/src-tauri/src/commands/mod.rs`
**Dependencies:** T001 (uses lib.rs)
**Steps:**
- [X] T002.1 Create commands module with CommandError enum
- [X] T002.2 Add logging and panic handler to lib.rs
- [X] T002.3 Verify logging works: `cd rustnote && cargo build 2>&1 | head -20`
- [X] T002.4 Commit: "feat: add logging and panic handling"

---

## Phase 2: Document Model (Rust Core)

### T003 Create Document Model
**Description:** Implement Document, Workspace, and Settings structs with unit tests
**Files:**
- `rustnote/src-tauri/src/model/mod.rs`
- `rustnote/src-tauri/src/model/document.rs`
- `rustnote/src-tauri/src/model/workspace.rs`
- `rustnote/src-tauri/src/model/settings.rs`
**Dependencies:** T002 (error types needed)
**Steps:**
- [X] T003.1 Define Document struct with constructors and methods
- [X] T003.2 Write Document unit tests (verify fail then pass)
- [X] T003.3 Create model module exports
- [X] T003.4 Create Workspace model with FileEntry
- [X] T003.5 Create Settings model with Theme enum
- [X] T003.6 Run all model tests: `cd rustnote/src-tauri && cargo test model`
- [X] T003.7 Commit: "feat: add document, workspace, and settings models"

---

### T004 Markdown Parser Integration
**Description:** Integrate pulldown-cmark for parsing and syntect for syntax highlighting
**Files:**
- `rustnote/src-tauri/src/parser/mod.rs`
- `rustnote/src-tauri/src/parser/markdown.rs`
- `rustnote/src-tauri/src/parser/syntax.rs`
**Dependencies:** T003 (model ready)
**Steps:**
- [X] T004.1 Write MarkdownParser tests for headings, bold, lists, code, tables, tasklists
- [X] T004.2 Verify tests fail (module not found)
- [X] T004.3 Create parser module with exports
- [X] T004.4 Implement MarkdownParser using pulldown-cmark
- [X] T004.5 Run tests to verify pass (6 tests)
- [X] T004.6 Implement SyntaxHighlighter using syntect
- [X] T004.7 Run all parser tests: `cd rustnote/src-tauri && cargo test parser`
- [X] T004.8 Commit: "feat: add markdown parser and syntax highlighter"

---

### T005 [P] Tauri Commands (File Operations)
**Description:** Create Tauri command handlers for document CRUD, workspace, settings, and export
**Files:**
- `rustnote/src-tauri/src/commands/mod.rs`
- `rustnote/src-tauri/src/commands/document.rs`
- `rustnote/src-tauri/src/commands/workspace.rs`
- `rustnote/src-tauri/src/commands/export.rs`
**Dependencies:** T004 (parser ready for export)
**Steps:**
- [X] T005.1 Write file operations tests in document.rs
- [X] T005.2 Create commands module with exports
- [X] T005.3 Register commands in lib.rs
- [X] T005.4 Create workspace commands with recursive directory reading
- [X] T005.5 Create settings commands (read/write settings.json)
- [X] T005.6 Create export commands (HTML and PDF)
- [X] T005.7 Verify compilation: `cd rustnote/src-tauri && cargo build 2>&1 | head -30`
- [X] T005.8 Commit: "feat: add Tauri commands for file operations and export"

---

## Phase 3: Frontend Editor

### T006 Basic Editor UI
**Description:** Create HTML structure, CSS styling, and JavaScript app entry point
**Files:**
- `rustnote/src/styles/main.css`
- `rustnote/src/styles/theme-light.css`
- `rustnote/src/styles/theme-dark.css`
- `rustnote/src/scripts/app.js`
- `rustnote/src/scripts/editor.js`
- `rustnote/src/index.html` (update)
**Dependencies:** T005 (Tauri commands ready for frontend)
**Steps:**
- [X] T006.1 Create main.css with sidebar, editor, toolbar layout
- [X] T006.2 Create theme-light.css with CSS variables
- [X] T006.3 Create theme-dark.css with dark theme variables
- [X] T006.4 Create app.js with state management and event setup
- [X] T006.5 Create editor.js with getContent/setContent/format methods
- [X] T006.6 Update index.html with styles and scripts
- [X] T006.7 Verify frontend loads in browser
- [X] T006.8 Commit: "feat: add frontend editor UI with styles and scripts"

---

### T007 Live Markdown Rendering
**Description:** Implement debounced live rendering with visual markdown styling
**Files:**
- `rustnote/src/scripts/editor.js` (modify)
- `rustnote/src/scripts/renderer.js` (new)
**Dependencies:** T006 (basic UI ready)
**Steps:**
- [X] T007.1 Create MarkdownRenderer class with debouncing (150ms)
- [X] T007.2 Implement parse() method with basic markdown patterns
- [X] T007.3 Integrate live rendering into editor.js input handler
- [X] T007.4 Add visual style update on content change
- [X] T007.5 Commit: "feat: add live markdown rendering with debouncing"

---

### T008 File Operations UI
**Description:** Implement open, save, save-as, workspace sidebar, and recent files
**Files:**
- `rustnote/src/scripts/app.js` (modify)
**Dependencies:** T007 (rendering ready for content display)
**Steps:**
- [X] T008.1 Add createNewDocument() function
- [X] T008.2 Add openDocument() with Tauri dialog
- [X] T008.3 Add saveDocument() with atomic write
- [X] T008.4 Add saveDocumentAs() dialog
- [X] T008.5 Add autoSave() with interval
- [X] T008.6 Add loadWorkspace() to open folder
- [X] T008.7 Add renderFileTree() for sidebar
- [X] T008.8 Add toolbar button event handlers
- [X] T008.9 Commit: "feat: add file operations and workspace sidebar"

---

### T009 [P] Search Functionality
**Description:** Implement find/replace dialog with keyboard shortcut (Ctrl+F)
**Files:**
- `rustnote/src/scripts/search.js`
**Dependencies:** T008 (editor content ready)
**Steps:**
- [X] T009.1 Create SearchManager class with show/hide methods
- [X] T009.2 Implement findAll() with regex matching
- [X] T009.3 Implement findNext/findPrev navigation
- [X] T009.4 Add keyboard shortcut (Ctrl+F / Cmd+F)
- [X] T009.5 Commit: "feat: add search and find functionality"

---

### T010 [P] Theme Switching
**Description:** Add theme toggle button and data-theme attribute switching
**Files:**
- `rustnote/src/styles/theme-dark.css` (update)
- `rustnote/src/scripts/app.js` (update)
**Dependencies:** T006 (theme CSS ready)
**Steps:**
- [X] T010.1 Add toggleTheme() function to app.js
- [X] T010.2 Add theme button click handler
- [X] T010.3 Update theme-dark.css with full dark palette
- [X] T010.4 Enable both theme CSS files in index.html
- [X] T010.5 Commit: "feat: add theme switching (light/dark)"

---

### T011 [P] Export Functionality
**Description:** Add HTML and PDF export via Tauri commands and print dialog
**Files:**
- `rustnote/src/scripts/app.js` (modify)
**Dependencies:** T005 (export commands), T008 (document content)
**Steps:**
- [X] T011.1 Add exportHtml() function with save dialog
- [X] T011.2 Add exportPdf() via HTML print window
- [X] T011.3 Add export button to toolbar with dropdown menu
- [X] T011.4 Commit: "feat: add HTML and PDF export functionality"

---

## Phase 4: Polish and Integration

### T012 Auto-Save and Recovery
**Description:** Implement periodic snapshots and crash recovery on startup
**Files:**
- `rustnote/src/scripts/app.js` (modify)
- `rustnote/src-tauri/src/commands/recovery.rs` (new)
**Dependencies:** T008 (file operations ready)
**Steps:**
- [X] T012.1 Add startSnapshot() with 30s interval
- [X] T012.2 Add saveSnapshot() for .backup files
- [X] T012.3 Add checkRecovery() on startup
- [X] T012.4 Commit: "feat: add auto-save and crash recovery"

---

### T013 Settings Panel
**Description:** Create modal settings panel for theme, auto-save, font settings
**Files:**
- `rustnote/src/scripts/settings.js`
**Dependencies:** T010 (theme), T012 (auto-save)
**Steps:**
- [X] T013.1 Create SettingsPanel class with modal UI
- [X] T013.2 Add form fields: theme, auto-save, interval, font, size
- [X] T013.3 Implement save() with Tauri command
- [X] T013.4 Add right-click handler on theme button to open settings
- [X] T013.5 Commit: "feat: add settings panel"

---

### T014 Build and Verify
**Description:** Final production build with all features and manual testing
**Files:**
- `rustnote/src-tauri/tauri.conf.json` (modify)
**Dependencies:** All previous tasks
**Steps:**
- [X] T014.1 Configure production build in tauri.conf.json
- [X] T014.2 Run: `cd rustnote && cargo tauri build 2>&1 | tail -50`
- [X] T014.3 Verify .exe/.app output exists
- [X] T014.4 Run manual tests on built application
- [X] T014.5 Commit: "feat: complete MVP build with all features"

---

## Dependency Graph Summary

```
Phase 1: Foundation
├── T001 ──► T002

Phase 2: Rust Core
└── T002 ──► T003 ──► T004 ──► T005

Phase 3: Frontend
└── T005 ──► T006 ──► T007 ──► T008 ──► T009, T010, T011
                        │
                        └──► T012 ──► T013

Phase 4: Polish
└── T012, T013 ──► T014
```

## Parallel Opportunities

| Task | Can Parallel With | Reason |
|------|-------------------|--------|
| T001 | T002 | Different files, independent setup |
| T002 | T001 | Different files, independent setup |
| T005 | T004 | Commands don't depend on parser internals |
| T009 | T008 | Different file (search.js vs app.js) |
| T010 | T008 | Different file (theme CSS vs app.js) |
| T011 | T008 | Different file (export vs app.js) |

## Story Mapping

| User Story | Tasks |
|------------|-------|
| [US1] As a user, I want to create and edit Markdown documents | T006, T007, T008 |
| [US2] As a user, I want to manage files and workspaces | T008 (workspace), T012 |
| [US3] As a user, I want to customize the editor appearance | T010, T013 |
| [US4] As a user, I want to export documents | T011 |
| [US5] As a user, I want to search within documents | T009 |

---

**Task list generated:** 2026-04-11
**Total tasks:** 14
**Total steps:** ~70 checkboxes
**Estimated phases:** 4