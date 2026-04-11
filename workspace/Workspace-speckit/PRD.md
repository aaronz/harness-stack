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

# 13. Repository Structure

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

# 14. Crate Responsibilities

## 14.1 `core-model`

* semantic document structures
* positions, ranges, identifiers
* shared core types and errors

## 14.2 `markdown-parser`

* parse Markdown to semantic/intermediate representation
* document supported flavor behavior

## 14.3 `editor-engine`

* cursor movement rules
* selection logic
* insert/delete/edit commands
* smart Enter/Backspace/Tab behaviors
* undo/redo
* editing invariants and regression coverage

## 14.4 `serializer`

* semantic model to Markdown output
* preserve supported constructs predictably

## 14.5 `workspace`

* file IO
* recent files/folders
* file watching
* asset path resolution

## 14.6 `export`

* HTML export
* PDF export
* export option types and pipelines

## 14.7 `theme`

* theme tokens
* typography defaults
* focus/typewriter presentation config as needed

## 14.8 `settings`

* persisted configuration
* schema versioning

## 14.9 `recovery`

* autosave snapshots
* crash recovery metadata
* stale cleanup

## 14.10 `app-services`

* orchestration layer between UI shell and Rust core modules

---

# 15. Public Service Boundaries

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

# 16. Data Model Layers

The implementation should keep these layers separate:

## 16.1 Source Layer

* raw Markdown text

## 16.2 Semantic Layer

* parsed structures: headings, paragraphs, list items, tables, links, code blocks, images

## 16.3 Editing Layer

* selection state
* cursor mapping
* commands
* undo/redo history

## 16.4 Presentation Layer

* rendered spans/blocks
* visual decorations
* focus/typewriter metadata

The UI should render state, not become the source of truth.

---

# 17. UX Requirements

## 17.1 Layout

* left sidebar for workspace tree
* central editor canvas
* optional outline panel
* minimal top bar/menu
* find/replace panel
* export dialog
* preferences dialog

## 17.2 Interaction Rules

* split preview is not primary mode
* visible chrome must stay minimal
* command palette recommended
* link, image, and table interactions must be intentional and documented

## 17.3 Quality Bar

The product should feel calm enough for long writing sessions and precise enough for technical documentation.

---

# 18. Implementation Milestones

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
