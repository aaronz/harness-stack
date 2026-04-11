# RustNote Specification v1.0

**Project:** RustNote - Typora-like Markdown Editor  
**Based on:** PRD v3.1 + Gap Analysis (2026-04-11)  
**Status:** MVP Development - Iteration 6  
**Implementation Progress:** ~65-70% Complete

---

## 1. Overview

### 1.1 Product Definition

RustNote is an open-source, local-first, writing-first Markdown editor built in Rust. It provides a single-pane, seamless reader-writer experience where users write in a visually formatted document while the underlying source of truth remains plain Markdown.

**Core Thesis:** A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### 1.2 Implementation Status Summary

| Module | Status | Coverage |
|--------|--------|----------|
| File Operations | ⚠️ Partial | ~90% |
| Document Model | ⚠️ Partial | ~70% |
| Editor Transforms | ✅ Complete | ~85% |
| Semantic Layer | ✅ Complete | ~80% |
| Renderer | ✅ Complete | ~75% |
| Export (HTML) | ✅ Complete | ~90% |
| Export (PDF) | ⚠️ Partial | ~60% |
| Recovery | ✅ Complete | ~85% |
| Settings | ⚠️ Partial | ~60% |
| Workspace | ⚠️ Partial | ~70% |
| Frontend Editor | ⚠️ Partial | ~50% |
| Frontend Sidebar | ⚠️ Partial | ~40% |
| Frontend Outline | ✅ Complete | ~70% |
| Themes | ✅ Complete | ~80% |
| Focus/Typewriter Modes | ❌ Missing | 0% |
| Find/Replace | ❌ Missing | 0% |
| Auto-save Timer | ❌ Missing | 0% |

---

## 2. Functional Requirements

### 2.1 File Operations (FR-001 to FR-007)

#### FR-001 New File
- Create untitled document
- Save to chosen location with default extension `.md`
- **Status:** ✅ Implemented

#### FR-002 Open File
- Open Markdown file from disk
- Drag-and-drop file open supported
- **Status:** ⚠️ Partial - drag-and-drop not implemented

#### FR-003 Open Folder
- Open folder as workspace
- Show Markdown files in sidebar
- Optionally show image/assets directory entries
- **Status:** ✅ Implemented

#### FR-004 Save
- Manual save supported (Ctrl+S / Cmd+S)
- Save preserves valid UTF-8 Markdown text
- Save uses atomic write strategy where possible
- **Status:** ✅ Implemented

#### FR-005 Auto-Save
- Configurable on/off via settings
- Configurable debounce interval (default: 1000ms)
- Dirty-state indication required
- **Auto-save timer logic:** ❌ NOT implemented (P0 gap)
- **Status:** ⚠️ Settings exist, timer not running

#### FR-006 Recovery
- Restore unsaved content after crash or force close
- Recovery prompt presented when relevant
- Snapshot save/restore/list/delete implemented
- **Status:** ✅ Implemented (~85%)

#### FR-007 External Changes
- Detect file changes outside the app via file watcher
- User can reload, compare later, or preserve current buffer
- **Status:** ✅ Implemented

---

### 2.2 Core Editing Experience (FR-008 to FR-022)

#### FR-008 Single-Pane Live Rendering
- Document edited in one primary pane
- Supported syntax visually rendered inline
- Raw Markdown remains serializable and trustworthy
- **Status:** ⚠️ Partial - using contenteditable, not Tiptap/ProseMirror (technical debt)

#### FR-009 Heading Behavior
- Headings visually differentiate by level while editing
- Heading editing intuitive when entering or deleting markers
- **Status:** ✅ Implemented

#### FR-010 Emphasis Behavior
- Bold, italic, strikethrough, inline code visually render inline
- Cursor and selection behavior around inline formatting tested
- **Status:** ✅ Implemented

#### FR-011 Link Behavior
- Links visually appear as links while editing
- Users can edit link text and target without raw-syntax confusion
- **Status:** ✅ Implemented

#### FR-012 List Behavior
- Ordered and unordered lists visually render correctly
- Enter continues list item
- Enter on empty list item exits list
- Backspace at expected structural boundaries degrades list predictably
- Tab/Shift+Tab indent and outdent nested items
- **Status:** ✅ Implemented (~85%)

#### FR-013 Task List Behavior
- Task list items render with checkbox affordance
- Toggling task state preserves valid Markdown
- **Interactive toggle:** ❌ NOT implemented (P2 gap)
- **Status:** ⚠️ Partial - rendering works, toggle not functional

#### FR-014 Blockquote Behavior
- Blockquotes visually render while remaining easy to enter, continue, and exit
- **Status:** ✅ Implemented

#### FR-015 Code Fence Behavior
- Fenced code blocks render with syntax highlighting
- Language tag preserved in Markdown source
- **Status:** ✅ Implemented

#### FR-016 Table Behavior
- Tables render clearly in editor
- Table editing behavior prioritizes correctness
- **Status:** ✅ Implemented

#### FR-017 Image Behavior
- Local images insert via picker, drag-and-drop, or paste when feasible
- Relative path handling correct
- Broken images show error state
- **Paste image:** ❌ NOT implemented (P2 gap)
- **Status:** ⚠️ Partial - basic rendering works, paste not implemented

#### FR-018 Paste Behavior
- Plain text paste works predictably
- Rich text paste converts to Markdown on best-effort basis
- Paste should not silently create malformed Markdown structures
- **HTML to Markdown conversion:** ❌ NOT implemented (P1 gap)
- **Status:** ❌ Not implemented - only plain text paste works

#### FR-019 Undo/Redo
- Session-level undo/redo required
- Structural operations participate consistently
- **Undo engine exists in Rust but not connected to frontend**
- **Status:** ⚠️ Partial - engine exists, Ctrl+Z/Ctrl+Y not wired

---

### 2.3 Markdown Support (FR-020 to FR-022)

#### FR-020 Required Syntax Support
- Headings H1-H6
- Bold, italic, strikethrough
- Inline code
- Fenced code blocks
- Ordered/unordered lists
- Task lists
- Blockquotes
- Links
- Images
- Horizontal rules
- Tables
- Frontmatter preserved as text block support
- **Status:** ✅ Complete

#### FR-021 Markdown Flavor
- CommonMark baseline
- GFM support for tables, task lists, strikethrough, autolinks
- **Status:** ✅ Complete

#### FR-022 Serialization Fidelity
- Supported constructs survive edit-save-reopen cycles reliably
- **Status:** ✅ Complete

---

### 2.4 Workspace and Navigation (FR-023 to FR-026)

#### FR-023 File Tree
- Create, rename, delete files/folders
- Refresh on external changes
- **CRUD operations:** ❌ NOT implemented (P1 gap)
- **Status:** ⚠️ Partial - display only, no modification operations

#### FR-024 Recent Items
- Persist recent files and folders locally
- Recent files UI
- **Status:** ⚠️ Storage exists, UI not implemented (P2 gap)

#### FR-025 Find/Replace
- Find next/previous (Ctrl+F)
- Replace one/all (Ctrl+H)
- Case-sensitive option
- **Search panel UI:** ❌ NOT implemented (P0 gap)
- **Status:** ❌ Not implemented - no search panel or keyboard shortcuts

#### FR-026 Outline / TOC Panel
- Heading outline generated from current document
- Clicking item navigates to section
- Panel hide/show supported
- **Status:** ✅ Functional (~70%)

---

### 2.5 Display, Focus, and Themes (FR-027 to FR-030)

#### FR-027 Themes
- Light and dark themes required
- Typography and spacing tuned for long-form readability
- **Status:** ✅ Complete (~80%)

#### FR-028 Focus Mode
- Dim or de-emphasize non-current paragraphs/sections
- Should materially reduce visual distraction
- CSS class `.focus-mode` exists but **logic not implemented** (P0 gap)
- **Status:** ❌ CSS exists, logic not implemented

#### FR-029 Typewriter Mode
- Active line or paragraph remains vertically centered or near-centered
- Interaction should remain smooth during navigation and editing
- CSS class `.typewriter-mode` exists but **logic not implemented** (P0 gap)
- **Status:** ❌ CSS exists, logic not implemented

#### FR-030 Content Width and Typography Settings
- Editor font size
- Content width
- Line spacing or equivalent readability control
- **Content width control:** ❌ NOT applied (P2 gap)
- **Line height setting:** ❌ NOT applied (P2 gap)
- **Status:** ⚠️ Partial - settings exist, values not used in editor

---

### 2.6 Export (FR-031 to FR-033)

#### FR-031 HTML Export
- Standalone or linked-assets mode
- Preserve headings, lists, code blocks, tables, images
- **Status:** ✅ Complete (~90%)

#### FR-032 PDF Export
- Preserve structure and readable layout
- Configurable page size and margins minimum
- **Status:** ⚠️ Partial (~60%) - basic implementation, limited formatting

#### FR-033 Export Architecture
- Export targets implemented behind clear interface boundary
- **Status:** ⚠️ Partial - export in commands/, not behind interface boundary

---

### 2.7 Preferences and State (FR-034)

#### FR-034 Preferences
| Preference | Status |
|------------|--------|
| theme | ✅ Implemented |
| autoSave | ✅ Implemented |
| autoSaveInterval | ✅ Implemented |
| focusMode | ❌ Missing (P1 gap) |
| typewriterMode | ❌ Missing (P1 gap) |
| outlineVisible | ❌ Missing (P1 gap) |
| fontSize | ✅ Implemented |
| fontFamily | ✅ Implemented |
| lineHeight | ✅ Implemented |
| contentWidth | ❌ Missing (P2 gap) |

**Settings interface missing fields:** focusMode, typewriterMode, outlineVisible (P1 gap)

---

## 3. API Requirements

### 3.1 DocumentResult Interface

```typescript
interface DocumentResult {
  id: string;
  title: string;
  content: string;
  file_path: string | null;
  is_dirty: boolean;
  headings: Heading[];        // ❌ Missing from backend (P1 gap)
  created_at: string;
  modified_at: string;
}

interface Heading {
  level: 1 | 2 | 3 | 4 | 5 | 6;
  text: string;
  position: number;  // character offset in document
}
```

### 3.2 Settings Interface

```typescript
interface Settings {
  theme: 'light' | 'dark';
  autoSave: boolean;
  autoSaveInterval: number;  // milliseconds
  focusMode: boolean;        // ❌ Missing (P1 gap)
  typewriterMode: boolean;  // ❌ Missing (P1 gap)
  outlineVisible: boolean;   // ❌ Missing (P1 gap)
  fontSize: number;
  fontFamily: string;
  lineHeight: number;
  contentWidth?: number;     // ❌ Missing (P2 gap)
}
```

---

## 4. Gap Analysis Summary

### 4.1 P0 - Blocking Issues (Must Fix for MVP)

| Gap | Module | Impact | Fix Required |
|-----|--------|--------|---------------|
| Find/Replace UI not implemented | Frontend | Users cannot search within documents | Implement search panel with Ctrl+F/Ctrl+H shortcuts |
| Focus mode logic not implemented | Frontend | Core UX feature missing | Implement paragraph de-emphasis when `.focus-mode` active |
| Typewriter mode logic not implemented | Frontend | Core UX feature missing | Implement cursor centering on scroll |
| Auto-save timer not running | Frontend/Services | Data loss risk | Implement debounced auto-save timer from settings |

### 4.2 P1 - High Priority Issues

| Gap | Module | Impact | Fix Required |
|-----|--------|--------|---------------|
| Paste from rich text not handled | Frontend | Cannot paste from web/docs cleanly | Implement HTML to Markdown conversion on paste |
| DocumentResult missing `headings` field | Backend/API | API contract violation | Add headings array to Document struct |
| Settings incomplete | Backend/Settings | Cannot persist focus/typewriter/outline | Add focusMode, typewriterMode, outlineVisible booleans |
| File tree CRUD not implemented | Frontend | Cannot manage files from app | Implement create/rename/delete in sidebar |
| Undo/redo not connected | Frontend/Editor | Cannot undo/redo edits | Connect existing Rust engine to Ctrl+Z/Ctrl+Y |
| Drag-and-drop open not implemented | Frontend | Poor UX for file opening | Add drop zone event handlers |

### 4.3 P2 - Medium Priority Issues

| Gap | Module | Impact |
|-----|--------|--------|
| Recent files UI not implemented | Frontend | Cannot access recent files |
| Image paste not implemented | Frontend | Cannot paste screenshots/images |
| Content width control not applied | Frontend | Editor width not responsive to settings |
| Line height setting not applied | Frontend | Line spacing ignores settings |
| Task checkbox toggle not functional | Frontend | Cannot mark tasks complete |

---

## 5. Technical Debt

### 5.1 High Priority Technical Debt

| Item | Description | Impact | Recommended Fix |
|------|-------------|--------|-----------------|
| **Frontend Editor Architecture** | Using contenteditable instead of Tiptap/ProseMirror | High | Migrate to Tiptap/ProseMirror per PRD spec |
| **Buffer Layer** | No ropey-based text buffer | High | Add ropey for efficient large document handling |
| **Parser Layer** | Using pulldown-cmark only | Medium | Consider tree-sitter + comrak for incremental parsing |
| **CSS Architecture** | Inline styles scattered | Medium | Implement design token system |

### 5.2 Medium Priority Technical Debt

| Item | Description | Impact |
|------|-------------|--------|
| **Export Architecture** | Export logic in commands/ not behind interface | Low |
| **State Management** | React Context only | Low |
| **Test Coverage** | Only editor transform tests exist | High |

---

## 6. Architecture Requirements

### 6.1 Recommended Production Architecture

```
User Input → Tiptap (ProseMirror) → prosemirror-markdown → Rust (comrak/ropey)
                                       ↓
Frontend Render ← HTML/CSS ← prosemirror-markdown ← Rust services
```

### 6.2 Key Integration Points

1. **Frontend → Rust:** Tauri IPC commands for file operations, export, settings
2. **Rust → Frontend:** Events for file watching, external changes
3. **Markdown ↔ ProseMirror:** `prosemirror-markdown` for serialization
4. **Text Buffer ↔ Parse Tree:** Ropey for content, tree-sitter for incremental parse

### 6.3 Module Boundaries

- **Rust-owned:** Parsing, editing semantics, serialization, recovery, export correctness
- **Frontend-owned:** Editor surface (Tiptap/ProseMirror), theme system, image/table/code-block widgets

---

## 7. Performance Requirements

### 7.1 Performance Budgets

| Operation | Budget | Notes |
|-----------|--------|-------|
| Cold start (empty) | < 2s | Modern laptop, SSD |
| Cold start (1MB doc) | < 3s | Including initial render |
| Hot file open | < 500ms | After first file opened |
| Keystroke → render | < 100ms | End-to-end |
| Save (Ctrl+S) | < 200ms | File write complete |
| HTML export (10KB) | < 1s | Including syntax highlight |
| PDF export (10 pages) | < 5s | Print-quality |
| Outline update | < 100ms | After edit stops |
| Theme switch | < 200ms | Full dark/light toggle |
| Memory (10 docs idle) | < 300MB | Resident memory |
| Scroll (5MB doc) | 60 FPS | No jank |

### 7.2 Scalability Targets

| Document Size | Behavior |
|--------------|----------|
| < 100KB | Full feature set, no degradation |
| 100KB - 1MB | Full feature set, live render may debounce more |
| 1MB - 5MB | All features work, occasional render pauses < 200ms |
| 5MB - 10MB | Core editing works, disable live preview toggle |
| > 10MB | Warning shown, continue with degraded mode |

---

## 8. Security Requirements

### 8.1 Threat Model

| Threat | Mitigation |
|--------|------------|
| Path traversal via `../` | Validate and normalize all paths in Rust before file operations |
| HTML injection | Sanitize raw HTML on render; strip `<script>` tags |
| Malicious image paths | Validate image paths exist within workspace boundary |
| Export file overwrite | Confirm before overwriting existing files |
| Memory corruption from malformed docs | Fuzz test parser; isolate parsing in separate error context |

### 8.2 HTML Sanitization Policy

**Allowed:** `<p>`, `<h1>`-`<h6>`, `<strong>`, `<em>`, `<ul>`, `<ol>`, `<li>`, `<blockquote>`, `<pre>`, `<code>`, `<a>`, `<img>`, tables, task lists (checked/disabled only)

**Forbidden:** `<script>`, `<iframe>`, `<object>`, `<embed>`, event handlers, `javascript:` URLs, `<style>` injection

---

## 9. Acceptance Criteria

### 9.1 P0 Acceptance Criteria

- [ ] Find/Replace search panel opens with Ctrl+F
- [ ] Find/Replace replace works with Ctrl+H
- [ ] Focus mode de-emphasizes non-active paragraphs
- [ ] Typewriter mode keeps cursor vertically centered
- [ ] Auto-save triggers after configured interval when dirty

### 9.2 P1 Acceptance Criteria

- [ ] Rich text paste converts to Markdown
- [ ] DocumentResult includes headings array
- [ ] Settings persists focusMode, typewriterMode, outlineVisible
- [ ] File tree allows create/rename/delete operations
- [ ] Undo/Redo works with Ctrl+Z/Ctrl+Y
- [ ] Drag-and-drop opens files

### 9.3 P2 Acceptance Criteria

- [ ] Recent files accessible from UI
- [ ] Images can be pasted into editor
- [ ] Content width follows settings
- [ ] Line height follows settings
- [ ] Task checkboxes toggle on click

---

## 10. Out of Scope

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph
- Workspace complexity comparable to IDEs

---

*Specification based on PRD v3.1 and Gap Analysis dated 2026-04-11*