# Product Requirements Document (PRD)

## Rust-based WYSIWYG Markdown Editor (Typora-like)

**Document Version:** 1.0
**Product Codename:** RustNote
**Document Type:** Product Requirements Document
**Target Platform:** Desktop-first, cross-platform
**Primary Implementation Language:** Rust
**Status:** Draft

---

## 1. Executive Summary

RustNote is a desktop-first, cross-platform Markdown editor designed to deliver a clean, immersive, distraction-free writing experience similar to Typora, while leveraging Rust for performance, stability, security, and maintainability.

The product aims to combine the following qualities:

* A seamless “what you see while writing” Markdown experience
* Fast startup and smooth editing for large documents
* Native-feeling desktop performance across macOS, Windows, and Linux
* Reliable local-first document management
* Extensible architecture for future plugin, sync, AI, and publishing workflows

RustNote is targeted at writers, developers, technical authors, students, product managers, and knowledge workers who want a modern Markdown editor that is simpler than IDE-style editors, but more polished and performant than basic text tools.

---

## 2. Product Vision

Build the best Rust-native Markdown writing tool for serious users who value clarity, speed, portability, and a delightful writing experience.

The editor should feel:

* **As clean as a writing app**
* **As capable as a modern Markdown tool**
* **As fast and reliable as a native desktop application**

---

## 3. Background and Opportunity

Markdown has become the default lightweight writing format for developers, technical documentation teams, PKM users, product teams, and open-source communities. Existing tools often fall into one of three categories:

1. **Developer editors**: powerful but too complex for pure writing
2. **Basic note apps**: easy to use but limited in formatting/export
3. **Markdown-focused editors**: polished, but often closed, less extensible, or constrained by architecture choices

A Rust-based implementation creates an opportunity to deliver:

* Better startup time and runtime efficiency
* Lower memory usage for large documents
* Safer systems programming foundations
* Better long-term maintainability
* Shared core logic across multiple platforms
* Future reuse of core parser/editor/export capabilities in CLI, desktop, and mobile products

---

## 4. Product Goals

### 4.1 Primary Goals

* Deliver a Typora-like live preview Markdown editing experience
* Support a full-featured local Markdown writing workflow
* Provide excellent cross-platform desktop performance
* Make document editing intuitive for both technical and non-technical users
* Establish a strong foundation for future extensibility

### 4.2 Secondary Goals

* Enable robust import/export and publishing workflows
* Support themes, document templates, and configurable preferences
* Prepare architecture for plugin and sync capabilities
* Build a reusable Rust core for future products

### 4.3 Non-Goals (V1)

* Real-time multi-user collaboration
* Cloud-first document storage
* Full Notion-like block database features
* Full IDE-level extensibility at launch
* Enterprise document governance in V1
* In-editor code execution or notebook runtime in V1

---

## 5. Target Users

### 5.1 Primary User Segments

#### A. Technical Writers

Need a clean Markdown workflow with headings, tables, images, code blocks, export, and file organization.

#### B. Developers

Write README files, design docs, specs, changelogs, architecture notes, and project documentation.

#### C. Knowledge Workers

Use Markdown for personal notes, meeting notes, drafts, and lightweight documentation.

#### D. Students and Researchers

Need structured writing, citation placeholders, export, image insertion, and clean focus mode.

### 5.2 Secondary User Segments

* Bloggers and content creators
* Product managers writing PRDs and RFCs
* Open-source maintainers
* PKM users managing Markdown file collections

---

## 6. User Problems

Users today commonly face the following frustrations:

* Split preview and source editing interrupts writing flow
* Many editors are too heavy or too developer-centric
* Large Markdown files can become slow or visually unstable
* Export quality is inconsistent across tools
* Theme and typography support is limited or inconsistent
* File management for local Markdown projects is clumsy
* WYSIWYG behavior often breaks Markdown predictability

RustNote should solve these by offering a predictable live-render editing model, high performance, and a polished desktop experience.

---

## 7. Product Positioning

### 7.1 Positioning Statement

For users who want a focused, elegant, high-performance Markdown writing tool, RustNote is a Rust-based desktop editor that provides seamless live Markdown rendering, local-first file workflows, and professional export capabilities without the complexity of a full code editor.

### 7.2 Competitive Positioning

Compared with Typora:

* Similar immersive editing model
* More modern Rust-based architecture
* Stronger extensibility path
* Greater emphasis on performance and core engine reuse

Compared with VS Code + Markdown plugins:

* Simpler and cleaner writing experience
* Lower cognitive overhead
* More document-centric interaction design

Compared with note-taking tools:

* Better file-system compatibility
* Better standards-based Markdown workflow
* More predictable export and publishing pipeline

---

## 8. Core Product Principles

1. **Write first**: writing flow is the highest priority
2. **Markdown remains the source of truth**
3. **Live formatting should feel natural, not magical**
4. **Fast by default**
5. **Local-first and file-friendly**
6. **Beautiful typography matters**
7. **Power features should not clutter the core experience**
8. **Rust core should be modular and reusable**

---

## 9. Key Use Cases

### 9.1 Core Writing Use Cases

* Write and edit Markdown documents with live formatting
* Create headings, lists, links, images, tables, and code blocks
* Organize files and folders in a project sidebar
* Export documents to HTML, PDF, and DOCX-like formats
* Switch themes and writing modes
* Search within and across documents

### 9.2 Professional Documentation Use Cases

* Write product specs and technical design docs
* Maintain repository README and docs content
* Draft blog posts and knowledge-base articles
* Manage a folder-based documentation workspace

### 9.3 Extended Use Cases

* Use templates for recurring document types
* Manage images and relative asset paths
* Use keyboard shortcuts for efficient authoring
* Publish static HTML output for lightweight sharing

---

## 10. Scope Definition

## 10.1 MVP Scope

The MVP should include:

* Cross-platform desktop app (macOS, Windows, Linux)
* Open/edit/save Markdown files
* Live Markdown rendering in a single-pane editing experience
* Basic file explorer for local folders
* Support for common Markdown syntax
* Syntax-aware editing behaviors
* Image insertion and display
* Table editing support
* Code fences with syntax highlighting
* Export to HTML and PDF
* Themes (light/dark at minimum)
* Search in current document
* Auto-save and crash recovery
* Keyboard shortcuts and command palette
* Settings/preferences

## 10.2 Post-MVP Scope

* Tabs/workspaces
* Global search across folders
* Document outline panel
* Template library
* Typewriter mode / focus mode enhancements
* More advanced exports (DOCX, EPUB)
* Plugin system
* Mermaid/diagram support
* Frontmatter editing UI
* Sync integrations
* AI writing assistance

---

## 11. Functional Requirements

## 11.1 Document Lifecycle

### FR-001 Create Document

* User can create a new untitled Markdown document
* User can save it to a selected file path
* Default extension is `.md`

### FR-002 Open Existing Document

* User can open a Markdown file from disk
* User can open a folder/workspace containing Markdown files
* App should preserve recent files and recent folders

### FR-003 Save and Auto-save

* Manual save via command/menu/shortcut
* Auto-save configurable by user
* Unsaved state visibly indicated
* Crash recovery restores recent unsaved content

### FR-004 Rename / Move Awareness

* If files are renamed or moved externally, app should detect and handle gracefully
* User should be notified of broken paths or deleted files

---

## 11.2 Editing Experience

### FR-005 Live Preview Editing

* Markdown syntax should visually render inline while editing
* Headings, bold, italic, links, lists, checklists, quotes, code blocks, and tables should be represented in a rendered manner
* Cursor behavior must remain intuitive when moving through rendered content

### FR-006 Source-of-Truth Integrity

* Underlying content must remain valid Markdown text
* Users must always be able to inspect or edit raw syntax where needed
* Rendered editing should never silently corrupt Markdown structure

### FR-007 Standard Text Editing

* Undo/redo
* Cut/copy/paste
* Select all
* Multi-line selection
* Indentation / outdentation for lists and blocks
* Duplicate line / paragraph
* Delete line / block

### FR-008 Markdown Shortcuts

* Keyboard shortcuts for headings, bold, italic, code, quote, lists, links, images
* Auto-completion or assisted insertion for matching syntax markers where appropriate

### FR-009 Smart Enter / Backspace Behavior

* Continue bullet lists and numbered lists on Enter
* Exit list when pressing Enter on empty item
* Handle block quotes and checklists intelligently
* Backspace should degrade structure predictably

### FR-010 Paste Handling

* Plain text paste
* Rich text paste converted to Markdown where feasible
* Image paste support from clipboard
* URL paste handling for automatic link creation when selected text exists

---

## 11.3 Markdown Syntax Support

### FR-011 Core Syntax

Support at minimum:

* Headings (H1-H6)
* Bold / italic / strikethrough
* Inline code
* Code fences
* Ordered / unordered lists
* Task lists
* Blockquotes
* Links
* Images
* Horizontal rules
* Tables
* Footnotes (post-MVP acceptable if necessary)
* Frontmatter (read/write support)

### FR-012 Extended Markdown Compatibility

The system should support a defined Markdown flavor, such as GitHub Flavored Markdown (GFM) plus selected extensions.

A compatibility spec must be documented for:

* Tables
* Task lists
* Auto links
* Strikethrough
* Frontmatter
* Math support (optional for MVP, preferred post-MVP)

---

## 11.4 File and Workspace Management

### FR-013 Sidebar File Tree

* User can open a folder as a workspace
* File tree displays Markdown files and optionally assets
* User can create, rename, delete, and duplicate files from sidebar
* Folder creation supported

### FR-014 Recent Files / Recent Workspaces

* Show recent documents and folders on app launch or via menu
* Configurable history limit

### FR-015 External File Change Detection

* Changes made by other apps should trigger reload or conflict handling
* User may choose auto-reload or prompt-before-reload behavior

---

## 11.5 Navigation and Discovery

### FR-016 In-document Search

* Find next/previous
* Case-sensitive option
* Regex optional post-MVP
* Replace current / replace all

### FR-017 Outline View

* Generate heading outline from document
* Clicking outline item jumps to relevant section
* Post-MVP acceptable if not in MVP

### FR-018 Go To Elements

* Go to heading
* Go to line / section equivalent where technically appropriate
* Jump between open documents when tabs are supported

---

## 11.6 Visual Presentation and Themes

### FR-019 Themes

* Support at least light and dark themes
* Typography, spacing, code block style, table style, and block quote style should be customizable

### FR-020 Focus and Reading Modes

* Focus mode hides non-essential UI
* Typewriter mode keeps active line or paragraph visually centered
* Reading mode optional for MVP

### FR-021 Typography Controls

* Font family selection
* Font size
* Line height
* Paragraph width / content max width
* Custom CSS or theme customization post-MVP

---

## 11.7 Media and Embedded Content

### FR-022 Image Support

* Insert local image via picker, drag-drop, or paste
* Display relative-path images correctly
* Basic resize/display controls optional
* Broken image states clearly indicated

### FR-023 Code Block Rendering

* Syntax highlighting for common languages
* Copy code action optional
* Code fence language preserved in source

### FR-024 Table Editing

* Easy insertion of tables
* Keyboard navigation across cells
* Row/column add/remove operations
* Rendered visual table editing without breaking Markdown serialization

### FR-025 Math and Diagrams

* Inline/block math optional MVP stretch goal
* Mermaid/diagram rendering post-MVP

---

## 11.8 Import / Export / Publishing

### FR-026 Export to HTML

* Clean standalone HTML export
* Optional CSS theme selection
* Assets linked or embedded based on mode

### FR-027 Export to PDF

* Print-quality PDF export
* Preserve headings, tables, code blocks, and images
* Page margins and paper size configurable

### FR-028 Export to Additional Formats

* DOCX and EPUB are post-MVP goals
* Export architecture should be designed to support multiple targets

### FR-029 Copy as HTML / Rich Text

* Optional post-MVP capability for interoperability with external apps

---

## 11.9 Settings and Preferences

### FR-030 Preferences

* Theme
* Auto-save behavior
* Default opening behavior
* Preferred Markdown flavor/settings
* Font settings
* Window restore behavior
* Export defaults

### FR-031 Keyboard Shortcut Support

* Standard shortcuts per OS
* Shortcut customization post-MVP

---

## 11.10 Reliability and Recovery

### FR-032 Auto Recovery

* Unsaved changes periodically snapshotted
* On crash or forced shutdown, recovery flow restores document drafts

### FR-033 Version Safety

* Basic local history optional MVP stretch or post-MVP
* At minimum, protect users from silent overwrite or serialization corruption

---

## 12. Non-Functional Requirements

## 12.1 Performance

### NFR-001 Startup Performance

* Cold startup should feel near-instant on modern machines
* Target: under 2 seconds for typical systems

### NFR-002 Editing Responsiveness

* Typing latency should remain imperceptible for normal documents
* Large documents should remain usable without major frame drops

### NFR-003 Memory Efficiency

* Idle memory footprint should remain competitive with native desktop tools
* Large document rendering should degrade gracefully

## 12.2 Reliability

### NFR-004 Stability

* App should not crash under standard editing workflows
* File corruption risk must be minimized

### NFR-005 Data Integrity

* Save operations must be atomic where possible
* Backup/temporary write strategy should reduce risk of partial writes

## 12.3 Cross-platform Consistency

### NFR-006 Platform Support

* macOS, Windows, Linux supported with consistent core behavior
* Keyboard/menu conventions should still respect platform expectations

## 12.4 Security and Privacy

### NFR-007 Local-first Privacy

* User content stored locally by default
* No cloud transmission in MVP unless explicitly enabled

### NFR-008 Safe File Handling

* Prevent arbitrary execution via document rendering
* Sanitize HTML rendering and preview pathways appropriately

## 12.5 Accessibility

### NFR-009 Accessibility Support

* Keyboard-first operation
* Screen-reader-friendly baseline where technically feasible
* High contrast themes post-MVP if not fully in MVP

---

## 13. UX Requirements

## 13.1 Interaction Model

The interaction model should emulate the smoothness of direct writing rather than source-code editing.

Key UX expectations:

* Syntax decorations appear when helpful, but do not distract
* Cursor movement through formatted content is predictable
* Selection behavior remains intuitive around links, images, and tables
* Structural editing feels natural for lists, block quotes, and code blocks

## 13.2 Layout

Recommended desktop layout:

* Optional left sidebar: file tree
* Main center: document canvas/editor
* Optional right sidebar: outline / document info / export options (post-MVP)
* Top toolbar should remain minimal
* Command palette available for discoverability without clutter

## 13.3 Empty States

* New user welcome screen
* Recent documents/workspaces
* “Create new” and “Open folder” as primary actions
* Template suggestions optional

---

## 14. Information Architecture

### 14.1 MVP Information Areas

* Home / Recent
* Workspace File Tree
* Active Document Editor
* Search Bar / Find Panel
* Preferences
* Export Modal

### 14.2 Data Objects

Core entities include:

* Document
* Workspace
* Asset
* Theme
* Preference Set
* Export Job
* Recovery Snapshot

---

## 15. Suggested Technical Product Architecture

This section is included to align product design with a Rust-first implementation strategy.

## 15.1 Architecture Goals

* Reusable Rust core for parsing, editing state, serialization, and export
* Thin platform shell for desktop UI and system integration
* Clear separation between content model, render model, and UI state

## 15.2 Recommended High-level Modules

### A. Core Document Engine (Rust)

Responsibilities:

* Markdown parsing
* Document AST / intermediate representation
* Incremental update handling
* Markdown serialization
* Structural editing rules

### B. Editor State Engine (Rust)

Responsibilities:

* Cursor/selection model
* Undo/redo stack
* Block and inline edit operations
* Smart enter/backspace transformations

### C. Renderer / Presentation Mapping Layer

Responsibilities:

* Map Markdown structures into renderable UI blocks/spans
* Support live visual formatting while preserving source-of-truth Markdown
* Efficient incremental re-rendering

### D. File System and Workspace Layer

Responsibilities:

* Read/write files
* Watch file system changes
* Manage recent workspaces
* Asset path resolution

### E. Export Pipeline

Responsibilities:

* HTML generation
* PDF rendering pipeline
* Optional future DOCX/EPUB output

### F. Desktop Shell

Potential choices:

* Tauri + Rust backend + web-based UI layer
* Pure native UI stack where feasible

Recommended pragmatic choice for V1:

* **Tauri + Rust core**, due to strong cross-platform delivery speed and ecosystem practicality

---

## 16. Product Design Decisions

## 16.1 Markdown Flavor Decision

The product must explicitly define supported syntax behavior rather than loosely claiming “Markdown support.”

Recommended V1 standard:

* CommonMark baseline
* GitHub Flavored Markdown extensions
* Frontmatter support
* Optional math behind feature flag

## 16.2 Single-pane Editing Decision

The editor should prioritize a single-pane live-render writing experience.

Optional post-MVP modes:

* Source-only mode
* Split source + preview mode for debugging or advanced workflows

## 16.3 Local-first Decision

Documents are normal files on disk. No proprietary storage model is required for MVP.

Benefits:

* User trust
* Easy interoperability
* Git compatibility
* No platform lock-in

---

## 17. MVP Feature Prioritization

## 17.1 Must Have

* Open/save Markdown files
* Single-pane live Markdown editing
* Core syntax support
* File tree/workspace support
* In-document search
* Theme support
* HTML export
* PDF export
* Auto-save and recovery
* Syntax highlighting in code fences
* Basic image and table support

## 17.2 Should Have

* Outline panel
  n- Typewriter mode
* Better paste conversion
* Frontmatter support
* Recent files/home screen

## 17.3 Could Have

* Template system
* Global search
* Custom shortcuts
* Local version history
* Math rendering

## 17.4 Won’t Have in MVP

* Realtime collaboration
* Cloud sync
* Plugin marketplace
* AI co-writing
* Advanced publishing integrations

---

## 18. Success Metrics

## 18.1 Product Metrics

* Weekly active writers
* Number of documents created/opened per active user
* Export usage rate
* Workspace adoption rate
* Retention at 7/30 days

## 18.2 Quality Metrics

* Crash-free session rate
* Median startup time
* Median typing latency under typical document sizes
* Recovery success rate after crash
* Save failure rate

## 18.3 User Experience Metrics

* Time to first successful document creation
* Time to complete export task
* User-rated writing satisfaction
* User-rated performance satisfaction

---

## 19. Release Plan

## Phase 1: Foundation

* Core document model
* Markdown parsing and serialization
* Desktop shell setup
* Basic editor and save/load

## Phase 2: Writing Experience MVP

* Live formatting
* Lists, code blocks, tables, images
* Find/replace
* Themes and settings

## Phase 3: Workspace and Export

* File tree
* Recent workspaces
* HTML/PDF export
* Crash recovery

## Phase 4: Polish

* Performance optimization
* Cursor behavior refinement
* Paste improvements
* UX polish and platform conventions

## Phase 5: Post-MVP Expansion

* Outline/global search
* Templates
* Math/diagrams
* Plugin architecture
* Sync/AI integrations

---

## 20. Risks and Mitigations

## 20.1 WYSIWYG Complexity Risk

**Risk:** Typora-like live rendering is significantly harder than plain text editing. Cursor, selection, and serialization consistency can become fragile.

**Mitigation:**

* Build a robust intermediate document model
* Clearly separate Markdown source, semantic structure, and presentation state
* Invest early in editing invariants and regression tests

## 20.2 Cross-platform UI Consistency Risk

**Risk:** Desktop behavior differs across platforms.

**Mitigation:**

* Centralize editor logic in Rust core
* Keep platform shell thin
* Use platform-specific conventions only at the edges

## 20.3 Large Document Performance Risk

**Risk:** Full document rerendering becomes slow.

**Mitigation:**

* Use incremental parsing/rendering where possible
* Virtualize heavy regions if necessary
* Benchmark large-file workflows from early milestones

## 20.4 Export Fidelity Risk

**Risk:** PDF and HTML outputs differ from editor appearance.

**Mitigation:**

* Build a documented export styling model
* Use a consistent render/export theme layer
* Test with representative documents

---

## 21. Testing Requirements

## 21.1 Functional Testing

* Markdown syntax editing correctness
* Save/load workflows
* Export accuracy
* File tree operations
* Search/replace behavior

## 21.2 Compatibility Testing

* Different Markdown flavors and edge cases
* OS-level file path variations
* Relative asset loading across platforms

## 21.3 Performance Testing

* Startup time benchmarks
* Typing latency benchmarks
* Large document editing stress tests
* Bulk folder/workspace loading tests

## 21.4 Reliability Testing

* Crash recovery verification
* File system conflict scenarios
* Save interruption simulation

## 21.5 UX Testing

* Cursor movement predictability
* Table editing ergonomics
* Paste behavior quality
* Theme readability and layout comfort

---

## 22. Acceptance Criteria for MVP Launch

The MVP can be considered launch-ready when:

* Users can reliably create, edit, save, and reopen Markdown files
* Live-render writing experience is stable and intuitive
* Core Markdown constructs render and serialize correctly
* HTML and PDF export are production-usable
* The app performs well on mainstream desktop devices
* Crash recovery and auto-save are trustworthy
* macOS, Windows, and Linux builds meet baseline usability standards

---

## 23. Future Roadmap Directions

Potential roadmap investments include:

* Plugin architecture and extension APIs
* Git-aware workflows
* Cloud sync providers
* Collaboration and commenting
* AI-assisted writing, rewrite, summarize, translate, and format tools
* Publishing pipelines for blogs/docs platforms
* Mobile companion app built on shared Rust core
* Database-backed library mode alongside file-based mode

---

## 24. Open Questions

1. Should V1 optimize primarily for writers or developer-doc authors?
2. Should the initial UI be closer to Typora simplicity or Obsidian-lite workspace structure?
3. Is math support important enough for MVP?
4. Should table editing be basic or highly interactive in V1?
5. Should we support custom CSS in MVP or defer to theme presets?
6. Should split preview mode exist at launch for advanced users?
7. What export formats beyond HTML/PDF materially affect adoption?
8. Is Tauri the committed shell strategy for V1, or should the team evaluate native alternatives in parallel?

---

## 25. Appendix A: Suggested Initial Command Set

* New File
* Open File
* Open Folder
* Save
* Save As
* Export HTML
* Export PDF
* Find
* Replace
* Toggle Sidebar
* Toggle Theme
* Toggle Focus Mode
* Toggle Typewriter Mode
* Insert Table
* Insert Image
* Insert Code Block
* Insert Link
* Show Outline
* Open Recent
* Preferences

---

## 26. Appendix B: Suggested Rust-oriented Delivery Strategy

### Recommended Technical Stack Direction

* **Core Language:** Rust
* **Desktop Shell:** Tauri
* **Markdown Parsing:** CommonMark/GFM-capable parser in Rust
* **Syntax Highlighting:** Rust-based highlighting engine or interoperable library
* **Rendering Strategy:** Rust core + frontend presentation layer
* **Persistence:** Native file system + lightweight local config store

### Why Rust Is Strategic Here

* Excellent performance for parsing and editing engines
* Safe concurrency opportunities for background parsing/export
* Portable core logic across desktop and future mobile/CLI products
* Strong fit for building a durable editor engine with low runtime overhead

---

## 27. Final Product Definition

RustNote is a high-performance, local-first, Typora-like Markdown editor built around a reusable Rust core and a carefully designed writing experience. The product should win by offering the clarity of a dedicated writing tool, the practicality of Markdown files, and the speed and reliability expected from a modern native desktop application.
