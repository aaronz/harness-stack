# Functional Requirements

## 10. Functional Requirements

### 10.1 File Operations

#### FR-001 New file
* create untitled document
* save to chosen location
* default extension `.md`

#### FR-002 Open file
* open Markdown file from disk
* drag-and-drop file open supported

#### FR-003 Open folder
* open folder as workspace
* show Markdown files in sidebar
* optionally show image/assets directory entries

#### FR-004 Save
* manual save supported
* save preserves valid UTF-8 Markdown text
* save uses atomic write strategy where possible

#### FR-005 Auto-save
* configurable on/off
* configurable debounce interval
* dirty-state indication required

#### FR-006 Recovery
* restore unsaved content after crash or force close
* recovery prompt presented when relevant

#### FR-007 External changes
* detect file changes outside the app
* user can reload, compare later, or preserve current buffer depending final implementation

---

### 10.2 Core Editing Experience

#### FR-008 Single-pane live rendering
* document edited in one primary pane
* supported syntax is visually rendered inline
* raw Markdown remains serializable and trustworthy

#### FR-009 Heading behavior
* headings visually differentiate by level while editing
* heading editing must remain intuitive when entering or deleting markers

#### FR-010 Emphasis behavior
* bold, italic, strikethrough, inline code visually render inline
* cursor and selection behavior around inline formatting must be regression-tested

#### FR-011 Link behavior
* links visually appear as links while editing
* users can edit link text and target without raw-syntax confusion
* opening link vs editing link behavior must be intentionally designed

#### FR-012 List behavior
* ordered and unordered lists visually render correctly
* Enter continues list item
* Enter on empty list item exits list
* Backspace at expected structural boundaries degrades list predictably
* Tab/Shift+Tab indent and outdent nested items where applicable

#### FR-013 Task list behavior
* task list items render with interactive-feeling checkbox affordance or a clear equivalent visual treatment
* toggling task state preserves valid Markdown

#### FR-014 Blockquote behavior
* blockquotes visually render while remaining easy to enter, continue, and exit

#### FR-015 Code fence behavior
* fenced code blocks render with syntax highlighting
* entering/exiting code blocks should be unsurprising
* language tag preserved in Markdown source

#### FR-016 Table behavior
* tables render clearly in editor
* table editing behavior must prioritize correctness and low friction
* if full cell-model editing is too risky for MVP, a constrained but safe editing model is acceptable

#### FR-017 Image behavior
* local images insert via picker, drag-and-drop, or paste when feasible
* relative path handling must be correct
* broken images must show obvious but unobtrusive error state

#### FR-018 Paste behavior
* plain text paste works predictably
* rich text paste converts to Markdown on best-effort basis where feasible
* paste should not silently create malformed Markdown structures in common cases

#### FR-019 Undo/redo
* session-level undo/redo required
* structural operations participate consistently

---

### 10.3 Markdown Support

#### FR-020 Required syntax support
* headings H1-H6
* bold, italic, strikethrough
* inline code
* fenced code blocks
* ordered/unordered lists
* task lists
* blockquotes
* links
* images
* horizontal rules
* tables
* frontmatter preserved as text block support

#### FR-021 Markdown flavor
* CommonMark baseline
* GFM support for tables, task lists, strikethrough, autolinks
* supported behavior documented clearly in repo

#### FR-022 Serialization fidelity
* supported constructs must survive edit-save-reopen cycles reliably
* known lossy or unsupported constructs must be documented

---

### 10.4 Workspace and Navigation

#### FR-023 File tree
* create, rename, delete files/folders
* refresh on external changes

#### FR-024 Recent items
* persist recent files and folders locally

#### FR-025 Find/replace
* find next/previous
* replace one/all
* case-sensitive option

#### FR-026 Outline / TOC panel
* heading outline generated from current document
* clicking item navigates to section
* panel hide/show supported

---

### 10.5 Display, Focus, and Themes

#### FR-027 Themes
* light and dark themes required
* typography and spacing tuned for long-form readability

#### FR-028 Focus mode
* dim or de-emphasize non-current paragraphs/sections according to design choice
* should materially reduce visual distraction

#### FR-029 Typewriter mode
* active line or paragraph remains vertically centered or near-centered
* interaction should remain smooth during navigation and editing

#### FR-030 Content width and typography settings
* editor font size
* content width
* line spacing or equivalent readability control

---

### 10.6 Export

#### FR-031 HTML export
* standalone or linked-assets mode
* preserve headings, lists, code blocks, tables, images

#### FR-032 PDF export
* preserve structure and readable layout
* configurable page size and margins minimum

#### FR-033 Export architecture
* export targets implemented behind clear interface boundary to enable future formats

---

### 10.7 Preferences and State

#### FR-034 Preferences
* theme
* auto-save
* focus mode defaults
* typewriter mode defaults
* editor width/font size
* export defaults
