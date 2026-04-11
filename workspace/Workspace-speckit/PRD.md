# Open-Source Implementation-Ready PRD

## Typora-like Markdown Editor in Rust

**Project Name:** RustNote
**Document Version:** 2.1
**Document Type:** Open-source implementation-ready PRD
**Primary Language:** Rust
**Target Runtime:** Desktop, cross-platform
**Recommended Shell:** Tauri (V1)
**License Direction:** MIT OR Apache-2.0
**Status:** Refined implementation draft

---

# 1. Executive Definition

RustNote is an open-source, local-first, writing-first Markdown editor built in Rust.

It is **not** merely a text editor with Markdown rendering.
It is a **single-pane, seamless reader-writer experience** in which users write in a visually formatted document while the underlying source of truth remains plain Markdown.

The product goal is to reproduce and eventually surpass the reasons users love Typora:

* the writing flow feels uninterrupted
* the document looks readable while being edited
* Markdown syntax does not constantly get in the way
* common authoring tasks feel natural and low-friction
* local files remain portable and trustworthy

This PRD is intentionally implementation-ready for an open-source team. It defines product identity, user experience invariants, module boundaries, MVP scope, testing strategy, and engineering constraints.

---

# 2. Why Typora-like Products Win

A Typora-like product wins **less because of feature count** and **more because of interaction quality**.

Most Markdown tools force users into one of these trade-offs:

* raw source editing that hurts readability
* split preview that breaks writing flow
* rich text convenience that breaks Markdown fidelity
* powerful workspaces that create UI noise and cognitive overhead

A great Typora-like editor resolves those trade-offs by combining:

1. **Seamless live rendering**
   Users edit a readable document, not a wall of syntax.

2. **Low-distraction writing flow**
   Minimal chrome, strong typography, focus-friendly modes.

3. **High-frequency authoring ergonomics**
   Lists, links, images, tables, code blocks, paste, and export feel easy.

4. **Markdown trustworthiness**
   Markdown remains the source of truth; files stay portable and Git-friendly.

5. **Desktop stability and polish**
   The product feels dependable for everyday writing, not like a fragile demo.

This PRD therefore prioritizes the **experience contract** as much as the feature list.

---

# 3. Product Thesis

## 3.1 Core Thesis

RustNote should be defined as:

> A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

## 3.2 Strategic Positioning

RustNote should sit between:

* developer editors that are too complex for writing
* note apps that hide or weaken file portability
* rich text tools that do not preserve Markdown cleanly

It should feel:

* calmer than VS Code
* more open and extensible than Typora
* more file-native than note databases
* more polished than most open-source Markdown editors

---

# 4. Product Principles

## 4.1 Identity Principles

1. **Write first**
   The user should feel like they are writing a document, not managing syntax.

2. **Markdown is the source of truth**
   Everything saved must remain valid, predictable Markdown.

3. **Rendered while editing, not previewed separately**
   Single-pane live rendering is the default identity of the product.

4. **Invisible UI beats visible complexity**
   Keep chrome, controls, and mode switches minimal.

5. **High-frequency actions must feel effortless**
   Lists, headings, links, images, tables, code blocks, paste, and export are product-defining paths.

6. **Local-first always**
   Users own normal files on disk.

7. **Rust owns correctness**
   Parsing, editing semantics, serialization, recovery, and export-critical logic belong in Rust-owned boundaries.

## 4.2 Open-Source Principles

* small, reviewable pull requests
* ADRs for meaningful architectural decisions
* documented supported Markdown behavior
* tests around editing invariants, not just parser output
* low-coupling modules with clear ownership
* explicit out-of-scope choices to prevent product drift

---

# 5. Success Criteria

RustNote succeeds when users say:

* “I forget I’m editing Markdown.”
* “It feels smoother than editing raw `.md` in a code editor.”
* “It does not break my files.”
* “Images, tables, lists, and export work without friction.”
* “It is calm enough for long-form writing.”

For open-source maintainers, success also means:

* contributors can understand the repo quickly
* editing bugs are reproducible and regression-tested
* releases are stable across macOS, Windows, Linux

---

# 6. Users and Jobs to Be Done

## 6.1 Primary Users

### Writers and note-takers

Need a clean writing space with Markdown portability.

### Developers and technical authors

Need README/docs/spec editing with Git-friendly output and code-block correctness.

### Product managers and researchers

Need a readable drafting environment with headings, lists, tables, images, and export.

## 6.2 Core Jobs

* write a document in Markdown without being distracted by syntax
* edit existing Markdown files from local disk
* structure content with headings, lists, tables, quotes, and code blocks
* insert images and links without breaking paths
* export polished HTML or PDF
* work in a folder-based documentation workspace

---

# 7. Non-Negotiable Experience Invariants

This section is the most important refinement to the PRD.

These are not “nice to have” UX details. They define whether the product actually feels Typora-like.

## 7.1 Single-Pane Invariant

* the main experience is one editing canvas
* split preview is not the core workflow
* users should never need to mode-switch to understand document appearance

## 7.2 Readability Invariant

* headings should look like headings while editing
* lists should look like lists
* links and images should appear as document content, not primarily as raw syntax
* the document should remain pleasant to read during editing

## 7.3 Cursor Invariant

* cursor movement must be predictable around inline formatting, links, code spans, tables, images, and quotes
* no cursor traps
* selection boundaries must feel natural

## 7.4 Structure Invariant

* Enter, Backspace, Tab, and Shift+Tab must behave consistently for lists, quotes, and nested structures
* structural editing must not cause surprising Markdown corruption

## 7.5 Fidelity Invariant

* editing and rendering must not silently destroy supported Markdown constructs
* when a construct cannot be preserved perfectly, behavior must be documented and tested

## 7.6 Calmness Invariant

* default UI should remain visually quiet
* toolbar, controls, and settings must not dominate the writing surface
* focus mode and typewriter mode are part of product identity, not decoration

## 7.7 Local-Trust Invariant

* the app should behave like a trustworthy steward of local files
* save, autosave, recovery, reload, and external-change handling must be dependable

---

# 8. Scope

## 8.1 MVP In Scope

* desktop app for macOS, Windows, Linux
* open, edit, save `.md` files
* open a folder as a workspace
* single-pane live Markdown editing
* rendered support for headings, emphasis, links, images, lists, task lists, code fences, quotes, horizontal rules, tables
* smart editing behavior for lists, quotes, and basic structure transitions
* in-document search and replace
* recent files and recent folders
* theme support: light and dark minimum
* focus mode
* typewriter mode
* HTML export
* PDF export
* auto-save and crash recovery
* code fence syntax highlighting
* relative asset path support
* basic outline / table-of-contents panel

## 8.2 Explicitly Out of Scope for MVP

* real-time collaboration
* cloud sync
* plugin marketplace
* mobile app
* AI writing features
* Mermaid and advanced diagrams
* full DOCX/EPUB support
* database-backed note graph
* workspace complexity comparable to IDEs

## 8.3 Stretch Goals

* frontmatter helper UI
* local document history
* math rendering
* custom CSS support
* improved paste-from-rich-text conversion

---

# 9. What Must Feel Excellent in V1

For a Typora-like editor, some workflows matter disproportionately more than others.

The MVP should optimize aggressively for these:

1. opening an existing Markdown document and continuing writing immediately
2. creating a clean structured document with headings and lists
3. inserting and managing links
4. pasting content from web/docs/apps without making a mess
5. inserting images with correct relative paths
6. writing technical docs with code fences and tables
7. exporting a polished PDF or HTML file
8. recovering safely from accidental close or crash

A feature may exist and still fail if the interaction is awkward. Therefore these flows need interaction-level tests and polish.

---

# 10. Functional Requirements

## 10.1 File Operations

### FR-001 New file

* create untitled document
* save to chosen location
* default extension `.md`

### FR-002 Open file

* open Markdown file from disk
* drag-and-drop file open supported

### FR-003 Open folder

* open folder as workspace
* show Markdown files in sidebar
* optionally show image/assets directory entries

### FR-004 Save

* manual save supported
* save preserves valid UTF-8 Markdown text
* save uses atomic write strategy where possible

### FR-005 Auto-save

* configurable on/off
* configurable debounce interval
* dirty-state indication required

### FR-006 Recovery

* restore unsaved content after crash or force close
* recovery prompt presented when relevant

### FR-007 External changes

* detect file changes outside the app
* user can reload, compare later, or preserve current buffer depending final implementation

---

## 10.2 Core Editing Experience

### FR-008 Single-pane live rendering

* document edited in one primary pane
* supported syntax is visually rendered inline
* raw Markdown remains serializable and trustworthy

### FR-009 Heading behavior

* headings visually differentiate by level while editing
* heading editing must remain intuitive when entering or deleting markers

### FR-010 Emphasis behavior

* bold, italic, strikethrough, inline code visually render inline
* cursor and selection behavior around inline formatting must be regression-tested

### FR-011 Link behavior

* links visually appear as links while editing
* users can edit link text and target without raw-syntax confusion
* opening link vs editing link behavior must be intentionally designed

### FR-012 List behavior

* ordered and unordered lists visually render correctly
* Enter continues list item
* Enter on empty list item exits list
* Backspace at expected structural boundaries degrades list predictably
* Tab/Shift+Tab indent and outdent nested items where applicable

### FR-013 Task list behavior

* task list items render with interactive-feeling checkbox affordance or a clear equivalent visual treatment
* toggling task state preserves valid Markdown

### FR-014 Blockquote behavior

* blockquotes visually render while remaining easy to enter, continue, and exit

### FR-015 Code fence behavior

* fenced code blocks render with syntax highlighting
* entering/exiting code blocks should be unsurprising
* language tag preserved in Markdown source

### FR-016 Table behavior

* tables render clearly in editor
* table editing behavior must prioritize correctness and low friction
* if full cell-model editing is too risky for MVP, a constrained but safe editing model is acceptable

### FR-017 Image behavior

* local images insert via picker, drag-and-drop, or paste when feasible
* relative path handling must be correct
* broken images must show obvious but unobtrusive error state

### FR-018 Paste behavior

* plain text paste works predictably
* rich text paste converts to Markdown on best-effort basis where feasible
* paste should not silently create malformed Markdown structures in common cases

### FR-019 Undo/redo

* session-level undo/redo required
* structural operations participate consistently

---

## 10.3 Markdown Support

### FR-020 Required syntax support

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

### FR-021 Markdown flavor

* CommonMark baseline
* GFM support for tables, task lists, strikethrough, autolinks
* supported behavior documented clearly in repo

### FR-022 Serialization fidelity

* supported constructs must survive edit-save-reopen cycles reliably
* known lossy or unsupported constructs must be documented

---

## 10.4 Workspace and Navigation

### FR-023 File tree

* create, rename, delete files/folders
* refresh on external changes

### FR-024 Recent items

* persist recent files and folders locally

### FR-025 Find/replace

* find next/previous
* replace one/all
* case-sensitive option

### FR-026 Outline / TOC panel

* heading outline generated from current document
* clicking item navigates to section
* panel hide/show supported

---

## 10.5 Display, Focus, and Themes

### FR-027 Themes

* light and dark themes required
* typography and spacing tuned for long-form readability

### FR-028 Focus mode

* dim or de-emphasize non-current paragraphs/sections according to design choice
* should materially reduce visual distraction

### FR-029 Typewriter mode

* active line or paragraph remains vertically centered or near-centered
* interaction should remain smooth during navigation and editing

### FR-030 Content width and typography settings

* editor font size
* content width
* line spacing or equivalent readability control

---

## 10.6 Export

### FR-031 HTML export

* standalone or linked-assets mode
* preserve headings, lists, code blocks, tables, images

### FR-032 PDF export

* preserve structure and readable layout
* configurable page size and margins minimum

### FR-033 Export architecture

* export targets implemented behind clear interface boundary to enable future formats

---

## 10.7 Preferences and State

### FR-034 Preferences

* theme
* auto-save
* focus mode defaults
* typewriter mode defaults
* editor width/font size
* export defaults

---

# 11. Non-Functional Requirements

## 11.1 Performance

### NFR-001 Startup

* target cold start under 2 seconds on mainstream modern laptop

### NFR-002 Typing responsiveness

* no visible lag during ordinary editing on typical documents under 1 MB

### NFR-003 Large document usability

* at least 5 MB documents remain usable, with graceful degradation if necessary

## 11.2 Reliability

### NFR-004 Crash-free goal

* stable releases target crash-free session rate above 99%

### NFR-005 Safe writes

* partial writes avoided through temp-file replacement strategy where practical

## 11.3 Privacy and Security

### NFR-006 Local-first

* no network dependency in MVP editing path

### NFR-007 Safe rendering

* raw HTML handling policy must be explicit
* export sanitization and trust model documented

## 11.4 Accessibility

### NFR-008 Keyboard-first

* essential actions reachable by keyboard

### NFR-009 Readability

* default themes must maintain good contrast and comfortable typography

---

# 12. Architecture Overview

## 12.1 High-Level Approach

Recommended V1 architecture:

* **Rust core** for parsing, editing semantics, serialization, workspace logic, recovery, export orchestration
* **Tauri desktop shell** for packaging, windows, menus, file dialogs, OS integration
* **UI layer** for rendering and interaction, but not as the owner of document correctness

## 12.2 Architectural Rule

Anything affecting:

* Markdown fidelity
* cursor/selection semantics
* structural editing behavior
* save/recovery correctness
* export correctness

must live in Rust-owned interfaces or Rust-controlled services.

## 12.3 Why This Matters

A Typora-like editor fails when too much logic leaks into ad hoc UI behavior. The architecture must preserve a clear center of correctness.

---

# 13. Frontend Design Goals

The frontend is not a thin decorative layer. It is the visible embodiment of the product promise.

A Typora-like product succeeds or fails in the frontend because users directly experience:

* readability while editing
* the amount of visual distraction
* cursor and selection predictability
* interaction smoothness around structured Markdown content
* speed of common authoring tasks

Therefore the frontend design must optimize for three things simultaneously:

1. **calmness** — the interface should visually disappear during writing
2. **clarity** — rendered content, controls, states, and transitions should be understandable
3. **control** — advanced authoring actions should remain discoverable and precise

The frontend must never feel like a generic admin panel, IDE clone, or document app overloaded with chrome.

---

# 14. Frontend Product Principles

## 14.1 Writing Surface First

The central editor canvas is the product. Sidebars, toolbars, dialogs, menus, and status surfaces exist to support it, not compete with it.

## 14.2 Chrome Minimization

Anything persistent on screen must justify its presence. Default UI should be sparse. Optional surfaces may be shown contextually or by explicit user action.

## 14.3 Readable While Editable

The frontend must preserve the feeling that the user is reading a real document while still allowing precise editing of Markdown-backed structures.

## 14.4 Progressive Disclosure

Simple users should see very little UI. Power users should still be able to reach commands, shortcuts, file operations, and configuration without clutter.

## 14.5 Interaction Consistency

Selections, hover states, click targets, keyboard actions, and structural editing interactions must behave consistently across content types.

## 14.6 Platform Familiarity

The product should feel native enough on macOS, Windows, and Linux in menu behavior, shortcut display, window handling, and file dialogs.

---

# 15. Frontend Architecture

## 15.1 Recommended Frontend Stack Direction

For V1 within Tauri:

* frontend view layer with a mature reactive framework
* typed message boundary between UI layer and Rust services
* design token based styling system
* command/event architecture for editor actions

A pragmatic choice is:

* **Tauri shell**
* **Rust core services**
* **TypeScript frontend UI** inside the desktop shell

The PRD does not strictly require a specific UI framework, but the frontend must satisfy these constraints:

* strong component composition
* predictable local and global state handling
* efficient updates under frequent editor changes
* clean keyboard and event handling
* testability for complex UI interactions

## 15.2 Frontend Layer Model

The frontend should be divided into the following layers:

### A. Presentation Layer

Responsible for:

* layout containers
* typography
* colors and spacing
* visual states
* menus, dialogs, panels, buttons, inputs

### B. Editor Interaction Layer

Responsible for:

* editor canvas rendering
* selection and caret visualization
* keyboard event dispatch
* mouse/pointer interactions
* drag-and-drop handling
* contextual affordances around document elements

### C. App State Layer

Responsible for:

* active workspace
* open document metadata
* visible panels and dialogs
* user preferences currently applied
* command palette state
* search/replace state
* notifications and transient UI states

### D. Service Bridge Layer

Responsible for:

* calling Rust-owned services
* serializing requests/responses
* subscribing to events from core modules
* enforcing typed contracts

## 15.3 Core Architectural Rule for Frontend

The frontend may control **presentation and transient interaction state**, but it must not become the owner of:

* Markdown correctness
* structural editing semantics
* save/recovery correctness
* source serialization rules
* export correctness

Those responsibilities remain owned by Rust services.

---

# 16. Frontend Information Architecture

## 16.1 Primary UI Regions

The desktop app should contain the following regions:

1. **Window frame / title bar region**
2. **App menu / top command surface**
3. **Left workspace sidebar**
4. **Main editor canvas**
5. **Optional right-side outline or utility panel**
6. **Transient bottom or top status surface**
7. **Modal and popover layer**

## 16.2 Default Layout

Recommended default layout:

* left sidebar visible on workspace open
* right outline panel collapsed by default or user-configurable
* center editor canvas occupying dominant width
* minimal top bar with only essential controls
* no permanently visible heavy formatting toolbar

## 16.3 Layout Priorities

Visual dominance should follow this order:

1. editor canvas
2. current document title and file context
3. workspace/sidebar navigation
4. command surfaces
5. secondary metadata and settings

The editor should always feel like the largest and most important object on screen.

---

# 17. Detailed Screen and Surface Design

## 17.1 Launch / Home Screen

### Purpose

Provide a minimal but helpful starting point before a workspace or file is opened.

### Required Elements

* app name / simple identity mark
* primary actions: New File, Open File, Open Folder
* recent files list
* recent folders list
* optional “resume recovery” surface if a crash snapshot exists
* optional lightweight tip or shortcut hint

### UX Requirements

* should open quickly
* should not resemble a busy dashboard
* should allow keyboard navigation through recent items
* should disappear as soon as a file/workspace is open

## 17.2 Main Writing Screen

### Purpose

The primary authoring environment.

### Required Regions

* top bar or title row
* optional left sidebar
* central editor canvas
* optional right outline panel
* status micro-surface for save/search/export states

### UX Constraints

* the canvas must remain visually dominant
* panel toggling should be fast and non-jarring
* resizing panels should be supported if practical
* panel widths should persist between sessions

## 17.3 Preferences Screen / Dialog

### Sections

* appearance
* editor
* behavior
* workspace
* export
* keyboard shortcuts overview
* about/version/license

### UX Constraints

* settings should be organized by user mental model, not internal implementation
* advanced settings should not overwhelm common preferences
* changes should apply live where safe

## 17.4 Export Dialog

### Purpose

Allow users to export without overwhelming them with publishing jargon.

### Required Controls

* export format selector
* destination path
* page size and margin controls for PDF
* style/theme selector if applicable
* embed or link assets mode for HTML if supported
* preview summary of export target settings

### UX Constraints

* default options should work for most users
* dialog should clearly explain what happens to images/assets
* errors should be human-readable

## 17.5 Find / Replace Surface

### Required Elements

* search input
* replace input
* next / previous controls
* replace one / replace all
* case-sensitive toggle
* match count or current result indicator

### UX Constraints

* should be lightweight and keyboard-first
* should not obscure too much of the document
* focus should move smoothly between document and search controls

## 17.6 Command Palette

### Purpose

Provide fast access to all important actions without increasing visible UI complexity.

### Requirements

* open via shortcut
* fuzzy search commands
* support keyboard navigation and execution
* show shortcut hints when available
* include document, workspace, view, export, and settings actions

### Importance

This is a major power-user surface and should reduce the need for a cluttered toolbar.

---

# 18. Editor Canvas Frontend Design

## 18.1 Canvas Role

The editor canvas is the central interface element and should behave like a calm, readable document with precise editing capabilities.

## 18.2 Canvas Layout Rules

* content should be horizontally centered by default
* readable max content width should be enforced
* content width should be configurable
* outer whitespace should help reading, not waste space excessively
* vertical rhythm between headings, paragraphs, lists, quotes, tables, and code blocks must be visually consistent

## 18.3 Typography Requirements

The default typography system must optimize for long-form reading and writing.

### Required typography controls

* base font size
* line height or readable equivalent spacing control
* content width
* theme-adjusted text contrast

### Typography guidance

* heading hierarchy must be visually distinct
* paragraph rhythm should feel book-like rather than code-editor-like
* inline code should be visually clear without becoming noisy
* link styling should be identifiable without overpowering the text

## 18.4 Caret and Selection Design

* caret must remain clearly visible on both light and dark themes
* selection highlight must preserve readability
* selections across formatted content should look stable
* selection rendering in code blocks, tables, and quotes must not appear broken or offset

## 18.5 Scroll Behavior

* scrolling should be smooth and precise
* scroll anchoring should minimize jarring jumps during re-render
* typewriter mode must not create erratic scroll oscillation
* programmatic jumps from outline/search should preserve user orientation

## 18.6 Focus Mode Visual Behavior

Possible design pattern:

* current paragraph or block remains fully emphasized
* surrounding content is gently de-emphasized
* dimming must remain subtle enough for context retention

Focus mode should never make the document look disabled or hard to navigate.

## 18.7 Typewriter Mode Visual Behavior

Possible design pattern:

* keep active line or paragraph near vertical center
* transitions should be smooth and not “snap” aggressively
* the feature should remain stable when editing large blocks, lists, or code fences

---

# 19. Component Hierarchy and UI Composition

The frontend should be designed as a composable component system.

## 19.1 App-Level Components

* `AppShell`
* `TitleBar`
* `MenuBarBridge`
* `WorkspaceSidebar`
* `EditorWorkspace`
* `OutlinePanel`
* `StatusSurface`
* `ModalHost`
* `ToastHost`
* `CommandPalette`

## 19.2 Workspace Components

* `WorkspaceTree`
* `WorkspaceTreeNode`
* `FileRow`
* `FolderRow`
* `WorkspaceContextMenu`
* `RecentItemsList`

## 19.3 Editor Components

* `EditorCanvas`
* `DocumentViewport`
* `BlockRenderer`
* `InlineRenderer`
* `CaretLayer`
* `SelectionLayer`
* `DropCursorLayer`
* `LinkPopover`
* `ImageBlock`
* `CodeFenceBlock`
* `TableBlock`
* `QuoteBlock`
* `TaskListItem`
* `HeadingBlock`

## 19.4 Utility Components

* `SearchBar`
* `ReplaceBar`
* `ExportDialog`
* `PreferencesDialog`
* `ShortcutHint`
* `EmptyState`
* `RecoveryBanner`
* `InlineNotice`

## 19.5 Design Rule

Complex blocks such as images, tables, code fences, and links should have dedicated renderers and interaction logic rather than ad hoc conditional branches spread across the editor.

---

# 20. Frontend State Model

## 20.1 State Categories

The frontend should explicitly separate the following state types:

### A. Persistent User State

* theme
* focus mode default
* typewriter mode default
* content width
* font preferences
* panel visibility preferences
* recent items cache display preferences

### B. Session App State

* active workspace
* current document ID/path
* open panel states
* current search query
* current replace query
* export dialog state
* modal visibility
* pending notifications

### C. Transient Interaction State

* hover states
* context menu anchor
* drag target
* current pointer selection preview
* temporary link editing popover state
* inline suggestion visibility if later introduced

### D. Derived View State

* outline tree from current document
* current dirty state indicator
* export availability
* save/recovery banners
* current section in viewport if tracked

## 20.2 State Ownership Rules

* Rust owns document truth and editing semantics
* frontend owns ephemeral view state and interaction affordances
* duplicated source-of-truth state between frontend and Rust should be minimized

## 20.3 Event Model

The frontend should use an explicit event/command model for important actions.

Example categories:

* app commands
* editor commands
* workspace commands
* view commands
* export commands
* preferences commands

This improves consistency across:

* keyboard shortcuts
* menu bar actions
* toolbar buttons
* command palette execution
* context menu actions

---

# 21. Frontend Interaction Design by Content Type

## 21.1 Headings

* headings should have strong visual hierarchy
* clicking into heading should make editing obvious
* inserting/deleting heading markers should not feel fragile
* outline syncing should update promptly

## 21.2 Paragraphs and Inline Formatting

* plain paragraphs must remain visually clean
* inline emphasis boundaries should be editable without cursor confusion
* invisible syntax strategy must not hide editing affordances completely

## 21.3 Links

* distinguish between selecting link text, editing link target, and opening the link
* hover affordance may show URL preview or action affordance if design allows
* accidental navigation while editing should be avoided

## 21.4 Lists and Task Lists

* bullet/number alignment must be visually stable
* nested indentation must remain readable
* checkbox interaction should be simple and fast
* task toggling must synchronize with Markdown faithfully

## 21.5 Blockquotes

* quote styling should be calm and readable
* boundary between quoted and non-quoted text should be clear during editing

## 21.6 Code Fences

* syntax highlighting must enhance readability without resembling a full IDE
* copy affordance may exist but should stay unobtrusive
* code block entry/exit must feel stable with keyboard navigation

## 21.7 Tables

* table borders and cell spacing should be readable
* editing affordances must be clear if interactive editing is supported
* the design should avoid pretending to be a spreadsheet unless the behavior truly supports it

## 21.8 Images

* images should sit naturally within document flow
* oversize images should scale to content width with clear behavior
* broken asset rendering should not destroy layout
* image selection state should be visually distinct from text selection state

## 21.9 Horizontal Rules and Dividers

* should be visible but subtle
* should not appear like accidental UI separators instead of document content

---

# 22. Menus, Toolbars, and Contextual Controls

## 22.1 Top-Level Menu Strategy

The product should rely on:

* native application menus where appropriate
* command palette for broad action access
* small contextual controls for specific content types

It should avoid heavy always-visible formatting ribbons.

## 22.2 Minimal Top Bar

Recommended elements only:

* sidebar toggle
* current document title or path context
* quick search entry or command access if justified
* minimal export/share action if necessary

## 22.3 Context Menus

Context menus should exist for:

* workspace items
* selected text or block
* links
* images
* tables
* code blocks

They should expose the most relevant actions without duplicating every command.

## 22.4 Inline Controls

Inline controls may appear for:

* links
* images
* tables
* code fences

But these must remain subtle and not make the editor feel like a block-based page builder.

---

# 23. Visual Design System

## 23.1 Design Token System

The frontend should use tokens for:

* colors
* spacing
* typography scale
* radii
* shadows
* border treatments
* panel widths
* transition timing

This is required for theme consistency and future theming/custom CSS support.

## 23.2 Color Philosophy

* document text should prioritize readability
* UI chrome should recede behind content
* accent color should be used sparingly for selection, focus, links, active controls
* dark mode should avoid overly saturated or low-contrast text

## 23.3 Spacing Philosophy

* spacing should support reading rhythm
* avoid cramped admin-tool density
* avoid excessive airy emptiness that wastes editor area

## 23.4 Motion and Transitions

* use motion sparingly
* transitions should aid continuity, not decorate
* panel open/close, hover, focus, and dialog transitions should be subtle and fast
* no flashy animation in core writing path

## 23.5 Iconography

* icons should be simple and neutral
* labels should accompany ambiguous icons
* icon-only controls should be limited to well-known actions

---

# 24. Responsiveness and Window Adaptation

Although desktop-first, the frontend must handle different window sizes gracefully.

## 24.1 Large Windows

* maintain readable content width
* avoid stretching paragraphs too wide
* allow side panels without shrinking canvas excessively

## 24.2 Narrow Windows

* collapse secondary panels automatically or on preference
* preserve access to command palette and basic file actions
* maintain editing usability without overlapping controls

## 24.3 Fullscreen

* fullscreen should feel especially strong for writing
* top chrome may simplify further in fullscreen where platform conventions allow

---

# 25. Accessibility and Keyboard Design

## 25.1 Keyboard-First Navigation

Essential actions must be reachable via keyboard:

* new/open/save
* panel toggle
* search/replace
* command palette
* export
* focus/typewriter mode toggle
* heading/list/link/image/code/table insertion actions

## 25.2 Focus Management

* visible focus indicators required for interactive controls
* modals and popovers must trap and restore focus correctly
* command palette and search bar should return focus to editor appropriately

## 25.3 Contrast and Legibility

* default themes must meet a reasonable accessibility baseline
* selected text, inactive dimmed text, code spans, and links must remain legible

## 25.4 Screen Reader Considerations

Full screen reader support for a richly rendered editor may be difficult, but the architecture should avoid unnecessary barriers:

* semantic labeling on controls
* accessible dialogs and menus
* sensible reading order outside the editing canvas

---

# 26. Error States, Empty States, and Recovery UX

## 26.1 Empty States

Examples:

* no file open
* empty workspace
* no search results
* no recent files
* no outline items

These should guide action without feeling verbose.

## 26.2 Error Surfaces

The frontend should distinguish:

* blocking errors
* recoverable errors
* transient notifications
* background warnings

Examples:

* file save failed
* export failed
* image missing
* workspace path unavailable
* file changed externally

## 26.3 Recovery UX

When recovery data exists:

* clearly communicate that unsaved work was found
* allow restore, inspect, or dismiss flow
* avoid alarming language if recovery is normal and safe

---

# 27. Frontend Performance Requirements

## 27.1 Rendering Efficiency

* frontend should avoid full-canvas re-render for trivial updates where possible
* block-level or region-level update strategy is preferred
* panel state changes should not noticeably disturb editor performance

## 27.2 Input Responsiveness

* keyboard input should remain responsive under normal authoring load
* UI reactions to typing, selection, and structure changes should feel immediate

## 27.3 Scroll Stability

* avoid layout thrashing
* maintain stable scroll positions during document updates
* large images and code blocks should not create major jank

## 27.4 Startup Behavior

* initial shell should appear quickly
* expensive non-critical frontend initialization should not delay basic usability

---

# 28. Frontend Testing Strategy

## 28.1 Component Tests

Test isolated components such as:

* sidebar nodes
* search/replace surface
* export dialog
* command palette
* outline panel
* recovery banners

## 28.2 Interaction Tests

Test flows such as:

* opening a file from sidebar
* toggling focus mode and typewriter mode
* editing links/images/tables through intended UI affordances
* dialog open/apply/cancel flows
* context menu actions

## 28.3 Visual Regression Tests

Use visual snapshots for:

* light/dark theme rendering
* headings/lists/quotes/code blocks/tables/images
* focus mode and typewriter mode
* dialogs and panels

## 28.4 End-to-End Tests

Test full flows including:

* launch app
* open document
* edit content
* search and replace
* export document
* restore recovery content

## 28.5 Manual UX Test Checklists

Given the interaction-heavy nature of the product, manual QA checklists are required for:

* cursor predictability
* selection correctness
* link interactions
* image interactions
* table editing behavior
* panel and modal focus behavior
* theme readability

---

# 29. Frontend Implementation Guidance

## 29.1 Avoid These Failure Modes

* turning the product into a generic IDE layout
* adding a permanent formatting ribbon
* duplicating too much editor truth in frontend state
* implementing ad hoc special cases for every block type
* allowing contextual affordances to clutter the writing surface
* over-animating common authoring interactions

## 29.2 Preferred Design Approach

* start from calm document rendering
* add only the smallest amount of interface needed for precision
* centralize command handling
* make structural editing feel consistent before adding visual flourish
* ship fewer, more polished interactions rather than many half-finished ones

---

# 30. Repository Structure

```text
rustnote/
  Cargo.toml
  README.md
  LICENSE-MIT
  LICENSE-APACHE
  CONTRIBUTING.md
  CODE_OF_CONDUCT.md
  SECURITY.md
  ADR/
  docs/
    architecture/
    frontend/
    product/
    testing/
    release/
  crates/
    core-model/
    markdown-parser/
    editor-engine/
    serializer/
    workspace/
    export/
    theme/
    settings/
    recovery/
    app-services/
  apps/
    desktop/
      src/
        app/
        components/
        editor/
        panels/
        dialogs/
        hooks/
        services/
        state/
        styles/
        commands/
        tests/
  fixtures/
    markdown/
    editor/
    export/
    recovery/
  scripts/
    ci/
    release/
```

---

# 31. Crate Responsibilities

## 31.1 `core-model`

* semantic document structures
* positions, ranges, identifiers
* shared core types and errors

## 31.2 `markdown-parser`

* parse Markdown to semantic/intermediate representation
* document supported flavor behavior

## 31.3 `editor-engine`

* cursor movement rules
* selection logic
* insert/delete/edit commands
* smart Enter/Backspace/Tab behaviors
* undo/redo
* editing invariants and regression coverage

## 31.4 `serializer`

* semantic model to Markdown output
* preserve supported constructs predictably

## 31.5 `workspace`

* file IO
* recent files/folders
* file watching
* asset path resolution

## 31.6 `export`

* HTML export
* PDF export
* export option types and pipelines

## 31.7 `theme`

* theme tokens
* typography defaults
* focus/typewriter presentation config as needed

## 31.8 `settings`

* persisted configuration
* schema versioning

## 31.9 `recovery`

* autosave snapshots
* crash recovery metadata
* stale cleanup

## 31.10 `app-services`

* orchestration layer between UI shell and Rust core modules

---

# 32. Public Service Boundaries

Prefer narrow service interfaces over exposing many unstable internal types.

Suggested boundaries:

* `DocumentService`
* `EditorService`
* `WorkspaceService`
* `ExportService`
* `SettingsService`
* `RecoveryService`

Suggested operation families:

* open/save/reload document
* apply editor command
* query selection or outline
* insert image/link/table block
* export document
* restore snapshot

---

# 33. Data Model Layers

The implementation should keep these layers separate:

## 33.1 Source Layer

* raw Markdown text

## 33.2 Semantic Layer

* parsed structures: headings, paragraphs, list items, tables, links, code blocks, images

## 33.3 Editing Layer

* selection state
* cursor mapping
* commands
* undo/redo history

## 33.4 Presentation Layer

* rendered spans/blocks
* visual decorations
* focus/typewriter metadata

The UI should render state, not become the source of truth.

---

# 34. UX Requirements

## 34.1 Layout

* left sidebar for workspace tree
* central editor canvas
* optional outline panel
* minimal top bar/menu
* find/replace panel
* export dialog
* preferences dialog

## 34.2 Interaction Rules

* split preview is not primary mode
* visible chrome must stay minimal
* command palette recommended
* link, image, and table interactions must be intentional and documented

## 34.3 Quality Bar

The product should feel calm enough for long writing sessions and precise enough for technical documentation.

---

# 35. Implementation Milestones

## Milestone 0: Bootstrap

Deliverables:

* repo initialized
* Cargo workspace
* CI on macOS, Windows, Linux
* formatter, clippy, test scaffolding
* desktop shell launches empty app
* core docs: README, CONTRIBUTING, roadmap skeleton

Acceptance criteria:

* contributors can build locally from docs
* CI green on all supported OSes

## Milestone 1: Plain Markdown Document Loop

Deliverables:

* new/open/save file
* text buffer editing
* dirty state
* recent files
* basic recovery plumbing

Acceptance criteria:

* user can safely create and edit `.md`
* no normal-flow data loss

## Milestone 2: Live Rendering Foundation

Deliverables:

* parser integration
* render headings, emphasis, lists, quotes, links, code blocks
* initial cursor/selection mapping

Acceptance criteria:

* supported syntax reads like a document while editing
* edit-save-reopen fidelity passes fixtures for supported cases

## Milestone 3: Editing Semantics

Deliverables:

* smart Enter/Backspace/Tab behaviors
* task list handling
* shortcut formatting commands
* regression tests for editing invariants

Acceptance criteria:

* structural editing feels consistent in core authoring scenarios
* no major cursor traps in supported flows

## Milestone 4: Authoring Essentials

Deliverables:

* workspace sidebar
* image insertion/rendering
* code highlighting
* safe table support
* outline panel

Acceptance criteria:

* real documentation workflow is viable end-to-end

## Milestone 5: Calm Writing Features

Deliverables:

* themes
* focus mode
* typewriter mode
* settings persistence
* find/replace

Acceptance criteria:

* product feels distinctly writing-first, not just editor-first

## Milestone 6: Recovery, Export, Beta

Deliverables:

* robust autosave and recovery
* HTML export
* PDF export
* packaging and release process

Acceptance criteria:

* real documents export cleanly
* crash recovery works in tested scenarios
* public beta builds available for all target platforms

---

# 19. Engineering Standards

## 19.1 Rust Standards

* `cargo fmt` enforced
* `clippy` required in CI
* avoid unnecessary `unsafe`
* any `unsafe` must be isolated and justified

## 19.2 Code Organization

* business logic should not live in UI event handlers
* public APIs documented
* non-trivial features need tests and short design rationale

## 19.3 PR Standards

* small PRs preferred
* issue or rationale linked
* UI changes include screenshots/gifs when practical
* user-visible behavior changes update docs or fixtures

---

# 20. Testing Strategy

## 20.1 Unit Tests

Cover:

* parser behavior
* cursor movement
* selection logic
* editing commands
* undo/redo
* serializer
* settings and recovery

## 20.2 Editing Invariant Regression Tests

This is a key Typora-like requirement.

Maintain fixture-based tests for scenarios such as:

* Enter on list item
* Enter on empty list item
* Backspace at heading/list/quote boundaries
* link text edits
* cursor motion across inline code and emphasis
* toggling task lists
* table edit edge cases

## 20.3 Fixture Tests

Use real Markdown fixtures for:

* parse expectations
* roundtrip save behavior
* export regressions
* tricky documents from real-world authoring

## 20.4 Snapshot Tests

Useful for:

* HTML export
* semantic tree output
* theme token outputs

## 20.5 Integration Tests

Test:

* open/edit/save flows
* open folder and navigate files
* external file changes
* recovery flows
* export command flows

## 20.6 Performance Benchmarks

Track:

* startup time
* parse time
* edit latency
* serialization time
* export time

---

# 21. Definition of Done

A feature is done only when:

* implementation is merged
* tests added or updated
* supported behavior documented if relevant
* no known major data-loss risk remains
* acceptance criteria are met
* CI passes

---

# 22. Release Engineering

## 22.1 Channels

* nightly
* beta
* stable

## 22.2 CI/CD

* build matrix for macOS, Windows, Linux
* lint + tests required on PRs
* release notes generated consistently
* checksums published with releases

## 22.3 Versioning

* semver for core crates where practical
* semantic app releases preferred

## 22.4 Packaging

* native installers or archives for each OS
* clear install docs in README/releases

---

# 23. Security and Dependency Policy

## 23.1 Repo Files

Include:

* `SECURITY.md`
* vulnerability reporting process
* dependency update policy

## 23.2 Threat Areas

* malicious raw HTML in Markdown
* unsafe asset or path handling
* export-time injection issues
* malformed documents triggering parser/editor bugs

## 23.3 Dependency Rules

* minimize unnecessary dependencies
* prefer well-maintained crates
* periodically audit dependencies

---

# 24. Documentation Requirements

At minimum:

* `README.md`
* `CONTRIBUTING.md`
* `CODE_OF_CONDUCT.md`
* `SECURITY.md`
* architecture overview
* supported Markdown behavior doc
* known limitations doc
* testing strategy doc
* roadmap or milestone doc

---

# 25. Governance Model

Recommended initial structure:

* 1–3 maintainers with merge rights
* GitHub Issues for bugs/features
* GitHub Discussions for design topics and Q&A
* ADRs for key architectural changes
* milestone-driven roadmap visible to contributors

Recommended issue labels:

* `good first issue`
* `help wanted`
* `bug`
* `regression`
* `performance`
* `ux`
* `parser`
* `editor-engine`
* `workspace`
* `export`
* `theme`
* `testing`
* `documentation`
* `security`
* `breaking change`

---

# 26. Risks and Trade-offs

## 26.1 The hard problem is interaction semantics

The biggest risk is not parsing Markdown. It is making live rendering, cursor movement, selection, and structural editing feel natural.

Mitigation:

* invest early in editor-engine design
* maintain editing-invariant regression suites
* treat cursor bugs as product-critical defects

## 26.2 Table editing can consume the roadmap

Mitigation:

* start with constrained, safe table interactions
* optimize for correctness before spreadsheet-like richness

## 26.3 UI-layer logic creep can weaken architecture

Mitigation:

* keep correctness in Rust services
* front-end should dispatch commands and render derived state

## 26.4 Export disappointment can hurt adoption

Mitigation:

* prioritize HTML/PDF fidelity
* test representative docs
* document limits clearly

---

# 27. MVP Release Criteria

RustNote is ready for MVP release when:

1. users can reliably create, open, edit, save, and reopen Markdown files
2. the single-pane editing experience feels readable and stable for supported syntax
3. core authoring flows for headings, lists, links, images, code blocks, and tables are usable in real work
4. focus mode and typewriter mode are available and genuinely useful
5. folder-based workspace flow is viable
6. autosave and recovery work in common tested scenarios
7. HTML and PDF export are production-usable for ordinary documents
8. editing invariants are covered by tests and fixtures
9. stable builds are published for macOS, Windows, Linux

---

# 28. Recommended ADR Topics

The project should write ADRs for:

1. Why Tauri for V1
2. Exact supported Markdown flavor
3. Internal document model choice
4. Source fidelity and serialization guarantees
5. Table editing scope in MVP
6. Raw HTML handling policy
7. PDF export engine choice
8. Future extension/plugin boundary
9. Focus mode and typewriter implementation strategy

---

# 29. Initial Backlog Proposal

## P0

* repo bootstrap
* open/save file
* parser integration
* live rendering basics
* editor engine foundation
* list/quote behaviors
* autosave/recovery skeleton

## P1

* links/images/code blocks
* workspace tree
* tables
* outline panel
* find/replace
* themes

## P2

* focus mode
* typewriter mode
* export polish
* recent files/folders
* settings persistence

## P3

* frontmatter helpers
* local history
* custom CSS
* math support

---

# 30. Final Product Definition

RustNote is an open-source, Rust-first, Typora-like Markdown editor whose defining promise is not merely Markdown support, but a **calm, seamless, writing-first editing experience** with plain Markdown as the durable source of truth.

The project should be judged not only by whether features exist, but by whether writing feels smooth, readable, low-friction, and trustworthy.

That is the standard this PRD sets for the implementation.
