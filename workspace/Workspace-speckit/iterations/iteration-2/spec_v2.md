# RustNote Specification v2.0

**Project:** RustNote - Typora-like Markdown Editor
**Based on:** PRD v3.1 + Gap Analysis (Iteration 2, 2026-04-12)
**Status:** MVP Development - Iteration 2
**Implementation Progress:** ~80-85% Complete

---

## 1. Overview

### 1.1 Product Definition

RustNote is an open-source, local-first, writing-first Markdown editor built in Rust. It provides a single-pane, seamless reader-writer experience where users write in a visually formatted document while the underlying source of truth remains plain Markdown.

**Core Thesis:** A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### 1.2 Implementation Status Summary

| Module | Iteration-1 | Iteration-2 | Change |
|--------|-------------|-------------|--------|
| File Operations | ⚠️ 90% | ⚠️ 90% | Unchanged |
| Document Model | ⚠️ 70% | ✅ 95% | +25% |
| Editor Transforms | ✅ 85% | ✅ 85% | Unchanged |
| Semantic Layer | ✅ 80% | ✅ 80% | Unchanged |
| Renderer | ✅ 75% | ✅ 75% | Unchanged |
| Export (HTML) | ✅ 90% | ✅ 90% | Unchanged |
| Export (PDF) | ⚠️ 60% | ⚠️ 65% | +5% |
| Recovery | ✅ 85% | ✅ 85% | Unchanged |
| Settings | ⚠️ 60% | ✅ 95% | +35% |
| Workspace | ⚠️ 70% | ⚠️ 75% | +5% |
| Frontend Editor | ⚠️ 50% | ✅ 75% | +25% |
| Frontend Sidebar | ⚠️ 40% | ⚠️ 50% | +10% |
| Frontend Outline | ✅ 70% | ⚠️ 75% | +5% |
| Themes | ✅ 80% | ✅ 85% | +5% |
| Focus Mode | ❌ 0% | ✅ 100% | +100% |
| Typewriter Mode | ❌ 0% | ✅ 100% | +100% |
| Find/Replace | ❌ 0% | ✅ 100% | +100% |
| Auto-save Timer | ❌ 0% | ✅ 100% | +100% |

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
- **Drag-and-drop:** ❌ NOT implemented (P1 gap)
- **Status:** ⚠️ Partial - standard file dialog works, drag-and-drop missing

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
- **Auto-save timer:** ✅ Implemented with useAutoSaveTimer hook
- **Status:** ✅ Implemented

#### FR-006 Recovery
- Restore unsaved content after crash or force close
- Recovery prompt presented when relevant
- Snapshot save/restore/list/delete implemented
- **Status:** ✅ Implemented (~85%)

#### FR-007 External Changes
- Detect file changes outside the app via file watcher
- User can reload, compare later, or preserve current buffer
- **Status:** ⚠️ Partial - file watcher exists, UI integration incomplete (P2 gap)

---

### 2.2 Core Editing Experience (FR-008 to FR-022)

#### FR-008 Single-Pane Live Rendering
- Document edited in one primary pane
- Supported syntax visually rendered inline
- Raw Markdown remains serializable and trustworthy
- **TipTap Integration:** ✅ In Progress - TipTapEditor.jsx added, dual editor state
- **Status:** ⚠️ Partial - TipTap integrated but not fully migrated (technical debt)

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
- **Interactive toggle:** ⚠️ Partial - rendering works, toggle may need verification
- **Status:** ⚠️ Partial

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
- **Safe editing model:** ⚠️ Not constrained (P3 gap)
- **Status:** ⚠️ Partial - basic rendering works, cell editing not constrained

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
- **HTML to Markdown conversion:** ✅ Implemented via TurndownService
- **Status:** ✅ Implemented

#### FR-019 Undo/Redo
- Session-level undo/redo required
- Structural operations participate consistently
- **Undo engine:** ⚠️ Partial - TipTap has built-in undo/redo but Ctrl+Z/Ctrl+Y shortcuts not wired (P2 gap)
- **Status:** ⚠️ Partial - engine exists, shortcuts missing

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
- **Frontmatter rendering:** ⚠️ Not specially rendered (P3 gap)
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
- **Recent files UI:** ❌ NOT implemented (P2 gap)
- **Storage:** ✅ Implemented
- **Status:** ⚠️ Partial - storage exists, UI not implemented

#### FR-025 Find/Replace
- Find next/previous (Ctrl+F)
- Replace one/all (Ctrl+H)
- Case-sensitive option
- **Search panel UI:** ✅ Fully implemented via SearchPanel.jsx
- **Status:** ✅ Implemented

#### FR-026 Outline / TOC Panel
- Heading outline generated from current document
- Clicking item navigates to section
- Panel hide/show supported
- **Click navigation:** ❌ NOT implemented (P2 gap)
- **Document.headings:** ✅ Added to API
- **Status:** ⚠️ Functional - headings display but click doesn't navigate

---

### 2.5 Display, Focus, and Themes (FR-027 to FR-030)

#### FR-027 Themes
- Light and dark themes required
- Typography and spacing tuned for long-form readability
- **Status:** ✅ Complete (~85%)

#### FR-028 Focus Mode
- Dim or de-emphasize non-current paragraphs/sections
- Should materially reduce visual distraction
- **Focus mode logic:** ✅ Implemented via updateActiveBlock() + CSS de-emphasis
- **Status:** ✅ Implemented

#### FR-029 Typewriter Mode
- Active line or paragraph remains vertically centered or near-centered
- Interaction should remain smooth during navigation and editing
- **Typewriter mode logic:** ✅ Implemented via scrollToCursor() + scroll-behavior: smooth
- **Status:** ✅ Implemented

#### FR-030 Content Width and Typography Settings
- Editor font size
- Content width
- Line spacing or equivalent readability control
- **Settings storage:** ✅ Implemented (fontSize, fontFamily, lineHeight, contentWidth)
- **Settings applied to editor:** ❌ NOT applied to TipTap (P2 gap)
- **Status:** ⚠️ Partial - settings exist, values not applied to editor

---

### 2.6 Export (FR-031 to FR-033)

#### FR-031 HTML Export
- Standalone or linked-assets mode
- Preserve headings, lists, code blocks, tables, images
- **Backend:** ✅ Implemented
- **Frontend UI:** ❌ NOT implemented (P2 gap)
- **Status:** ⚠️ Partial - backend ready, UI missing

#### FR-032 PDF Export
- Preserve structure and readable layout
- Configurable page size and margins minimum
- **Backend:** ⚠️ Partial (~65%)
- **Frontend UI:** ❌ NOT implemented (P2 gap)
- **Status:** ⚠️ Partial - backend partial, UI missing

#### FR-033 Export Architecture
- Export targets implemented behind clear interface boundary
- **Status:** ⚠️ Partial - export in commands/, interface boundary not explicit

---

### 2.7 Preferences and State (FR-034)

#### FR-034 Preferences
| Preference | Iteration-1 | Iteration-2 |
|------------|-------------|-------------|
| theme | ✅ Implemented | ✅ Implemented |
| autoSave | ✅ Implemented | ✅ Implemented |
| autoSaveInterval | ✅ Implemented | ✅ Implemented |
| focusMode | ❌ Missing | ✅ Implemented |
| typewriterMode | ❌ Missing | ✅ Implemented |
| outlineVisible | ❌ Missing | ✅ Implemented |
| fontSize | ✅ Implemented | ✅ Implemented |
| fontFamily | ✅ Implemented | ✅ Implemented |
| lineHeight | ✅ Implemented | ✅ Implemented |
| contentWidth | ❌ Missing | ✅ Implemented |

**Settings interface:** ✅ Complete - all fields now implemented

---

## 3. Resolved Issues (Iteration-1 → Iteration-2)

The following P0/P1 issues from iteration-1 have been successfully fixed:

| Issue | Iteration-1 | Iteration-2 Resolution |
|-------|-------------|----------------------|
| Find/Replace UI not implemented | ❌ Missing | ✅ SearchPanel.jsx fully implements find/next/prev, replace, replace-all |
| Focus mode non-functional | ❌ Missing | ✅ Editor.jsx updateActiveBlock() + CSS de-emphasis working |
| Typewriter mode non-functional | ❌ Missing | ✅ Editor.jsx scrollToCursor() + scroll-behavior: smooth working |
| Auto-save not automatically triggered | ❌ Missing | ✅ useAutoSaveTimer hook implemented with debounce |
| Paste from rich text not handled | ❌ Missing | ✅ TurndownService in Editor.jsx converts HTML to Markdown |
| DocumentResult missing headings | ❌ Missing | ✅ document.rs now includes `headings: Vec<Heading>` |
| Settings incomplete | ⚠️ Partial | ✅ settings.rs now includes focusMode, typewriterMode, outlineVisible |

---

## 4. Remaining Gaps

### 4.1 P0 - Blocking Issues

**NONE** - All P0 issues from iteration-1 have been resolved.

### 4.2 P1 - High Priority Issues

| Gap | Module | Impact | Fix Required |
|-----|--------|--------|--------------|
| File tree CRUD not implemented | Frontend/Sidebar | Cannot manage workspace files | Add context menu with create/rename/delete operations |
| Drag-and-drop file open | Frontend | Poor UX for file opening | Add drop zone event handlers for files |

### 4.3 P2 - Medium Priority Issues

| Gap | Module | Impact |
|-----|--------|-------|
| Settings not applied to TipTap | Frontend | User preferences not honored (lineHeight, fontSize, contentWidth) |
| Recent files UI not implemented | Frontend | No quick access to recent documents |
| Undo/redo shortcuts not wired | Frontend | Users cannot easily undo mistakes (Ctrl+Z/Y) |
| Image paste not implemented | Frontend | Poor image insertion UX |
| Outline click navigation | Frontend | Outline is display-only |
| Export UI not implemented | Frontend | Export feature inaccessible |
| File watcher UI integration | Frontend/Services | External changes not communicated to user |

### 4.4 P3 - Lower Priority Issues

| Gap | Module | Impact |
|-----|--------|-------|
| Frontmatter not rendered specially | Frontend | Displayed as plain text |
| Table cell editing not constrained | Frontend | Potential for malformed tables |

---

## 5. API Requirements

### 5.1 DocumentResult Interface

```typescript
interface DocumentResult {
  id: string;
  title: string;
  content: string;
  file_path: string | null;
  is_dirty: boolean;
  headings: Heading[];        // ✅ Added in iteration-2
  created_at: string;
  modified_at: string;
}

interface Heading {
  level: 1 | 2 | 3 | 4 | 5 | 6;
  text: string;
  position: number;  // character offset in document
}
```

### 5.2 Settings Interface

```typescript
interface Settings {
  theme: 'light' | 'dark';
  autoSave: boolean;
  autoSaveInterval: number;  // milliseconds
  focusMode: boolean;        // ✅ Added in iteration-2
  typewriterMode: boolean;    // ✅ Added in iteration-2
  outlineVisible: boolean;    // ✅ Added in iteration-2
  fontSize: number;
  fontFamily: string;
  lineHeight: number;
  contentWidth?: number;      // ✅ Added in iteration-2
}
```

---

## 6. Technical Debt

### 6.1 High Priority Technical Debt

| Item | Description | Impact | Status |
|------|-------------|--------|--------|
| **TipTap Migration** | Using TipTap but not fully utilizing its capabilities | High | In Progress |
| **Editor State Sync** | Dual editor (contenteditable + TipTap) causes confusion | High | Needs Decision |
| **Buffer Layer** | No ropey-based text buffer for large document handling | High | Not Started |
| **Parser Layer** | Using pulldown-cmark + comrak, not tree-sitter + comrak as specified | Medium | Partial |

### 6.2 Medium Priority Technical Debt

| Item | Description | Impact | Status |
|------|-------------|--------|--------|
| **Export Architecture** | Export commands not integrated with frontend | Medium | Not Started |
| **Test Coverage** | Only editor transform tests exist | High | Needs Work |
| **CSS Architecture** | Inline styles scattered, no design token system | Medium | Partial |
| **State Management** | React Context only | Low | Acceptable |

---

## 7. Architecture Requirements

### 7.1 Recommended Production Architecture

```
User Input → Tiptap (ProseMirror) → prosemirror-markdown → Rust (comrak/ropey)
                                       ↓
Frontend Render ← HTML/CSS ← prosemirror-markdown ← Rust services
```

### 7.2 Current MVP Architecture

```
User Input → React 18 + contenteditable/TipTap → Rust services (pulldown-cmark)
                                      ↓
Frontend Render ← HTML/CSS ← Rust services
```

### 7.3 Module Boundaries

- **Rust-owned:** Parsing, editing semantics, serialization, recovery, export correctness
- **Frontend-owned:** Editor surface (TipTap/ProseMirror), theme system, image/table/code-block widgets

---

## 8. Performance Requirements

### 8.1 Performance Budgets

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

### 8.2 Scalability Targets

| Document Size | Behavior |
|--------------|----------|
| < 100KB | Full feature set, no degradation |
| 100KB - 1MB | Full feature set, live render may debounce more |
| 1MB - 5MB | All features work, occasional render pauses < 200ms |
| 5MB - 10MB | Core editing works, disable live preview toggle |
| > 10MB | Warning shown, continue with degraded mode |

---

## 9. Security Requirements

### 9.1 Threat Model

| Threat | Mitigation |
|--------|------------|
| Path traversal via `../` | Validate and normalize all paths in Rust before file operations |
| HTML injection | Sanitize raw HTML on render; strip `<script>` tags |
| Malicious image paths | Validate image paths exist within workspace boundary |
| Export file overwrite | Confirm before overwriting existing files |
| Memory corruption from malformed docs | Fuzz test parser; isolate parsing in separate error context |

### 9.2 HTML Sanitization Policy

**Allowed:** `<p>`, `<h1>`-`<h6>`, `<strong>`, `<em>`, `<ul>`, `<ol>`, `<li>`, `<blockquote>`, `<pre>`, `<code>`, `<a>`, `<img>`, tables, task lists (checked/disabled only)

**Forbidden:** `<script>`, `<iframe>`, `<object>`, `<embed>`, event handlers, `javascript:` URLs, `<style>` injection

---

## 10. Acceptance Criteria

### 10.1 Resolved Acceptance Criteria (Iteration-2)

- [x] Find/Replace search panel opens with Ctrl+F
- [x] Find/Replace replace works with Ctrl+H
- [x] Focus mode de-emphasizes non-active paragraphs
- [x] Typewriter mode keeps cursor vertically centered
- [x] Auto-save triggers after configured interval when dirty
- [x] Rich text paste converts to Markdown
- [x] DocumentResult includes headings array
- [x] Settings persists focusMode, typewriterMode, outlineVisible

### 10.2 P1 Acceptance Criteria (Remaining)

- [ ] File tree allows create/rename/delete operations
- [ ] Drag-and-drop opens files

### 10.3 P2 Acceptance Criteria (Remaining)

- [ ] Recent files accessible from UI
- [ ] Images can be pasted into editor
- [ ] Content width follows settings
- [ ] Line height follows settings
- [ ] Font size follows settings
- [ ] Task checkboxes toggle on click
- [ ] Undo/Redo works with Ctrl+Z/Ctrl+Y
- [ ] Outline click navigation scrolls to heading
- [ ] Export UI triggers HTML/PDF export
- [ ] File watcher prompts on external changes

---

## 11. Out of Scope

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

## 12. Iteration-2 Checkpoint

```
iteration=2
phase=phase1
timestamp=1775995091
```

---

*Specification based on PRD v3.1 and Gap Analysis dated 2026-04-12 (Iteration 2)*
