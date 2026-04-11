# RustNote Project Constitution

> A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

**Document Version:** 1.0  
**Project Name:** RustNote  
**Document Type:** Project Constitution  
**Primary Language:** Rust  
**Target Runtime:** Desktop, cross-platform (Tauri V1)  
**License:** MIT OR Apache-2.0  
**Status:** Active

---

## Preamble

This constitution defines the core principles, non-negotiable rules, and architectural standards that govern the RustNote project. It serves as the foundational contract between contributors, maintainers, and users—establishing what RustNote is, what it is not, and how decisions are made.

All contributors must familiarize themselves with this document before submitting changes. Architectural decisions, PR reviews, and issue triage must align with these principles.

---

## Article I: Project Identity

### Section 1.1 — Core Definition

RustNote is an open-source, local-first, writing-first Markdown editor built in Rust. It provides a **single-pane, seamless reader-writer experience** in which users write in a visually formatted document while the underlying source of truth remains plain Markdown.

RustNote is **not**:
- A raw source editor with Markdown preview
- A split-pane editor where preview is the primary mode
- A rich-text editor that abstracts away Markdown
- A note-taking app with database-backed storage

### Section 1.2 — Product Thesis

> RustNote should be defined as: A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### Section 1.3 — Strategic Positioning

RustNote sits between:
- Developer editors (VS Code, Sublime) that are too complex for writing
- Note apps (Notion, Obsidian) that hide or weaken file portability
- Rich text tools (Google Docs, Word) that do not preserve Markdown cleanly

RustNote feels:
- Calmer than VS Code
- More open and extensible than Typora
- More file-native than note databases
- More polished than most open-source Markdown editors

---

## Article II: Core Principles

### PRINCIPLE 1: Write First

**Name:** Write First  
**Non-Negotiable Rules:**
1. The user should feel like they are writing a document, not managing syntax
2. The main experience is one editing canvas—split preview is not the core workflow
3. Users should never need to mode-switch to understand document appearance
4. Visible chrome, controls, and mode switches must remain minimal

**Rationale:** The primary value proposition is uninterrupted writing flow. If users feel they are fighting the editor, we have failed.

---

### PRINCIPLE 2: Markdown is the Source of Truth

**Name:** Markdown Fidelity  
**Non-Negotiable Rules:**
1. Everything saved must remain valid, predictable Markdown
2. Supported constructs must survive edit-save-reopen cycles reliably
3. When a construct cannot be preserved perfectly, behavior must be documented and tested
4. Raw Markdown text remains the authoritative representation of the document

**Rationale:** Users own their files. Markdown portability is non-negotiable—files must remain Git-friendly and portable across tools.

---

### PRINCIPLE 3: Rendered While Editing

**Name:** Live Rendering  
**Non-Negotiable Rules:**
1. Supported syntax is visually rendered inline during editing
2. Headings, lists, links, images, code blocks must look like their rendered output while editing
3. The document should remain pleasant to read during editing
4. Raw syntax must not constantly get in the way of reading

**Rationale:** This is the defining Typora-like characteristic. Users edit a readable document, not a wall of syntax.

---

### PRINCIPLE 4: Calmness and Minimalism

**Name:** Calm Interface  
**Non-Negotiable Rules:**
1. Default UI should remain visually quiet
2. Toolbar, controls, and settings must not dominate the writing surface
3. Focus mode and typewriter mode are part of product identity, not decoration
4. The editor canvas must always feel like the largest and most important object on screen
5. Motion and transitions must aid continuity, not decorate

**Rationale:** Cognitive overhead destroys writing flow. The interface should visually disappear during writing.

---

### PRINCIPLE 5: Local-First Ownership

**Name:** Local-First  
**Non-Negotiable Rules:**
1. Users own normal files on disk
2. No network dependency in MVP editing path
3. Save, autosave, recovery, reload, and external-change handling must be dependable
4. The app should behave like a trustworthy steward of local files

**Rationale:** Local files are portable, trustworthy, and do not create vendor lock-in. Cloud features are explicitly out of scope for MVP.

---

### PRINCIPLE 6: Rust-Owned Correctness

**Name:** Rust Correctness Boundary  
**Non-Negotiable Rules:**
1. Parsing, editing semantics, serialization, recovery, and export-critical logic belong in Rust-owned boundaries
2. The frontend must not become the owner of Markdown correctness, structural editing semantics, save/recovery correctness, source serialization rules, or export correctness
3. Anything affecting document fidelity must live in Rust interfaces or Rust-controlled services
4. Business logic must not live in UI event handlers

**Rationale:** A Typora-like editor fails when too much logic leaks into ad hoc UI behavior. The architecture must preserve a clear center of correctness in Rust.

---

### PRINCIPLE 7: Interaction Quality Over Feature Count

**Name:** Interaction Quality  
**Non-Negotiable Rules:**
1. High-frequency actions (lists, headings, links, images, tables, code blocks, paste, export) must feel effortless
2. Cursor movement must be predictable around inline formatting, links, code spans, tables, images, and quotes
3. Enter, Backspace, Tab, and Shift+Tab must behave consistently for lists, quotes, and nested structures
4. No cursor traps, no surprising Markdown corruption
5. A feature may exist and still fail if the interaction is awkward

**Rationale:** Users tolerate missing features more than frustrating interactions. Interaction bugs are product-critical defects.

---

### PRINCIPLE 8: Open-Source Discipline

**Name:** Open-Source Standards  
**Non-Negotiable Rules:**
1. Small, reviewable pull requests required
2. ADRs (Architecture Decision Records) for meaningful architectural decisions
3. Documented supported Markdown behavior
4. Tests around editing invariants, not just parser output
5. Low-coupling modules with clear ownership
6. Explicit out-of-scope choices to prevent product drift

**Rationale:** Sustainable open-source requires discipline. Contributors must be able to understand the repo quickly and make changes confidently.

---

## Article III: Non-Negotiable Experience Invariants

These invariants define whether the product actually feels Typora-like. They are not "nice to have" UX details.

### INVARIANT 1: Single-Pane Invariant
- The main experience is one editing canvas
- Split preview is not the core workflow
- Users should never need to mode-switch to understand document appearance

### INVARIANT 2: Readability Invariant
- Headings should look like headings while editing
- Lists should look like lists
- Links and images should appear as document content, not primarily as raw syntax
- The document should remain pleasant to read during editing

### INVARIANT 3: Cursor Invariant
- Cursor movement must be predictable around inline formatting, links, code spans, tables, images, and quotes
- No cursor traps
- Selection boundaries must feel natural

### INVARIANT 4: Structure Invariant
- Enter, Backspace, Tab, and Shift+Tab must behave consistently for lists, quotes, and nested structures
- Structural editing must not cause surprising Markdown corruption

### INVARIANT 5: Fidelity Invariant
- Editing and rendering must not silently destroy supported Markdown constructs
- When a construct cannot be preserved perfectly, behavior must be documented and tested

### INVARIANT 6: Calmness Invariant
- Default UI should remain visually quiet
- Toolbar, controls, and settings must not dominate the writing surface
- Focus mode and typewriter mode are part of product identity, not decoration

### INVARIANT 7: Local-Trust Invariant
- The app should behave like a trustworthy steward of local files
- Save, autosave, recovery, reload, and external-change handling must be dependable

---

## Article IV: Success Criteria

### Section 4.1 — User Success Metrics

RustNote succeeds when users say:
- "I forget I'm editing Markdown."
- "It feels smoother than editing raw `.md` in a code editor."
- "It does not break my files."
- "Images, tables, lists, and export work without friction."
- "It is calm enough for long-form writing."

### Section 4.2 — Maintainer Success Metrics

For open-source maintainers, success also means:
- Contributors can understand the repo quickly
- Editing bugs are reproducible and regression-tested
- Releases are stable across macOS, Windows, Linux

---

## Article V: Scope Boundaries

### Section 5.1 — MVP In Scope

**Desktop Application:**
- macOS, Windows, Linux support
- Open, edit, save `.md` files
- Open a folder as a workspace

**Core Editing:**
- Single-pane live Markdown editing
- Rendered support for: headings, emphasis, links, images, lists, task lists, code fences, quotes, horizontal rules, tables
- Smart editing behavior for lists, quotes, and basic structure transitions

**Navigation and Search:**
- In-document search and replace
- Recent files and recent folders
- Basic outline / table-of-contents panel

**Display Modes:**
- Theme support: light and dark minimum
- Focus mode
- Typewriter mode

**Export:**
- HTML export
- PDF export

**Reliability:**
- Auto-save and crash recovery
- Code fence syntax highlighting
- Relative asset path support

### Section 5.2 — Explicitly Out of Scope for MVP

The following are explicitly excluded to prevent scope creep:
- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph
- Workspace complexity comparable to IDEs

### Section 5.3 — Stretch Goals (Post-MVP)

- Frontmatter helper UI
- Local document history
- Math rendering
- Custom CSS support
- Improved paste-from-rich-text conversion

---

## Article VI: Functional Requirements

### Section 6.1 — File Operations

| ID | Requirement | Criticality |
|----|-------------|-------------|
| FR-001 | New file: create untitled document, save to chosen location, default extension `.md` | P0 |
| FR-002 | Open file: open Markdown file from disk, drag-and-drop file open supported | P0 |
| FR-003 | Open folder: open folder as workspace, show Markdown files in sidebar, optionally show image/assets directory entries | P0 |
| FR-004 | Save: manual save supported, preserves valid UTF-8 Markdown, atomic write strategy | P0 |
| FR-005 | Auto-save: configurable on/off, configurable debounce interval, dirty-state indication required | P1 |
| FR-006 | Recovery: restore unsaved content after crash or force close, recovery prompt presented when relevant | P1 |
| FR-007 | External changes: detect file changes outside the app, user can reload, compare later, or preserve current buffer | P1 |

### Section 6.2 — Core Editing Experience

| ID | Requirement | Criticality |
|----|-------------|-------------|
| FR-008 | Single-pane live rendering: document edited in one primary pane, supported syntax visually rendered inline | P0 |
| FR-009 | Heading behavior: headings visually differentiate by level while editing, intuitive enter/delete | P0 |
| FR-010 | Emphasis behavior: bold, italic, strikethrough, inline code visually render inline, cursor/selection tested | P0 |
| FR-011 | Link behavior: links visually appear as links while editing, users can edit text and target without raw-syntax confusion | P0 |
| FR-012 | List behavior: ordered/unordered lists render correctly, Enter continues/empty exits, Backspace degrades predictably, Tab/Shift+Tab indent/outdent | P0 |
| FR-013 | Task list behavior: task list items render with checkbox affordance, toggling preserves valid Markdown | P0 |
| FR-014 | Blockquote behavior: blockquotes visually render while easy to enter, continue, and exit | P0 |
| FR-015 | Code fence behavior: fenced code blocks render with syntax highlighting, unsurprising enter/exit | P0 |
| FR-016 | Table behavior: tables render clearly in editor, correctness and low friction prioritized | P1 |
| FR-017 | Image behavior: local images insert via picker/drag-and-drop/paste, relative path handling correct, broken images show error state | P1 |
| FR-018 | Paste behavior: plain text paste predictable, rich text paste converts to Markdown on best-effort, no malformed Markdown silently | P1 |
| FR-019 | Undo/redo: session-level undo/redo required, structural operations participate consistently | P0 |

### Section 6.3 — Markdown Support

| ID | Requirement | Criticality |
|----|-------------|-------------|
| FR-020 | Required syntax: H1-H6, bold, italic, strikethrough, inline code, fenced code blocks, ordered/unordered lists, task lists, blockquotes, links, images, horizontal rules, tables, frontmatter preserved | P0 |
| FR-021 | Markdown flavor: CommonMark baseline, GFM support for tables, task lists, strikethrough, autolinks | P0 |
| FR-022 | Serialization fidelity: supported constructs survive edit-save-reopen reliably, lossy/unsupported constructs documented | P0 |

### Section 6.4 — Workspace and Navigation

| ID | Requirement | Criticality |
|----|-------------|-------------|
| FR-023 | File tree: create, rename, delete files/folders, refresh on external changes | P1 |
| FR-024 | Recent items: persist recent files and folders locally | P1 |
| FR-025 | Find/replace: find next/previous, replace one/all, case-sensitive option | P0 |
| FR-026 | Outline/TOC panel: heading outline from current document, clicking navigates to section, panel hide/show | P1 |

### Section 6.5 — Display, Focus, and Themes

| ID | Requirement | Criticality |
|----|-------------|-------------|
| FR-027 | Themes: light and dark themes required, typography and spacing tuned for long-form readability | P0 |
| FR-028 | Focus mode: dim or de-emphasize non-current paragraphs/sections, materially reduce visual distraction | P1 |
| FR-029 | Typewriter mode: active line or paragraph remains vertically centered or near-centered, smooth during navigation and editing | P1 |
| FR-030 | Content width and typography: editor font size, content width, line spacing or equivalent readability control | P1 |

### Section 6.6 — Export

| ID | Requirement | Criticality |
|----|-------------|-------------|
| FR-031 | HTML export: standalone or linked-assets mode, preserve headings, lists, code blocks, tables, images | P0 |
| FR-032 | PDF export: preserve structure and readable layout, configurable page size and margins minimum | P0 |
| FR-033 | Export architecture: export targets implemented behind clear interface boundary to enable future formats | P1 |

### Section 6.7 — Preferences and State

| ID | Requirement | Criticality |
|----|-------------|-------------|
| FR-034 | Preferences: theme, auto-save, focus mode defaults, typewriter mode defaults, editor width/font size, export defaults | P1 |

---

## Article VII: Non-Functional Requirements

### Section 7.1 — Performance

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-001 | Startup | Cold start under 2 seconds on mainstream modern laptop |
| NFR-002 | Typing responsiveness | No visible lag during ordinary editing on typical documents under 1 MB |
| NFR-003 | Large document usability | At least 5 MB documents remain usable, with graceful degradation if necessary |

### Section 7.2 — Reliability

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-004 | Crash-free goal | Stable releases target crash-free session rate above 99% |
| NFR-005 | Safe writes | Partial writes avoided through temp-file replacement strategy where practical |

### Section 7.3 — Privacy and Security

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-006 | Local-first | No network dependency in MVP editing path |
| NFR-007 | Safe rendering | Raw HTML handling policy must be explicit, export sanitization and trust model documented |

### Section 7.4 — Accessibility

| ID | Requirement | Target |
|----|-------------|--------|
| NFR-008 | Keyboard-first | Essential actions reachable by keyboard |
| NFR-009 | Readability | Default themes must maintain good contrast and comfortable typography |

---

## Article VIII: Architecture

### Section 8.1 — High-Level Architecture

**Recommended V1 Architecture:**
- **Rust core** for parsing, editing semantics, serialization, workspace logic, recovery, export orchestration
- **Tauri desktop shell** for packaging, windows, menus, file dialogs, OS integration
- **UI layer** for rendering and interaction, but not as the owner of document correctness

### Section 8.2 — Module Responsibilities

| Crate | Responsibilities |
|-------|------------------|
| `core-model` | Semantic document structures, positions, ranges, identifiers, shared core types and errors |
| `markdown-parser` | Parse Markdown to semantic/intermediate representation, document supported flavor behavior |
| `editor-engine` | Cursor movement rules, selection logic, insert/delete/edit commands, smart Enter/Backspace/Tab behaviors, undo/redo, editing invariants and regression coverage |
| `serializer` | Semantic model to Markdown output, preserve supported constructs predictably |
| `workspace` | File IO, recent files/folders, file watching, asset path resolution |
| `export` | HTML export, PDF export, export option types and pipelines |
| `theme` | Theme tokens, typography defaults, focus/typewriter presentation config as needed |
| `settings` | Persisted configuration, schema versioning |
| `recovery` | Autosave snapshots, crash recovery metadata, stale cleanup |
| `app-services` | Orchestration layer between UI shell and Rust core modules |

### Section 8.3 — Public Service Boundaries

Narrow service interfaces over exposing many unstable internal types:

**Core Services:**
- `DocumentService`: open/save/reload document
- `EditorService`: apply editor command, query selection or outline
- `WorkspaceService`: workspace management
- `ExportService`: export document
- `SettingsService`: settings persistence
- `RecoveryService`: restore snapshot

---

## Article IX: Frontend Architecture

### Section 9.1 — Frontend Layer Model

**A. Presentation Layer**
- Layout containers, typography, colors and spacing, visual states, menus, dialogs, panels, buttons, inputs

**B. Editor Interaction Layer**
- Editor canvas rendering, selection and caret visualization, keyboard event dispatch, mouse/pointer interactions, drag-and-drop handling, contextual affordances

**C. App State Layer**
- Active workspace, open document metadata, visible panels and dialogs, user preferences, command palette state, search/replace state, notifications

**D. Service Bridge Layer**
- Calling Rust-owned services, serializing requests/responses, subscribing to events from core modules, enforcing typed contracts

### Section 9.2 — Frontend State Categories

| Category | Ownership | Examples |
|----------|-----------|----------|
| Persistent User State | Frontend + Rust persistence | theme, focus mode default, content width, font preferences |
| Session App State | Frontend | active workspace, current document, open panels, search state |
| Transient Interaction State | Frontend | hover states, context menu anchor, drag target |
| Derived View State | Frontend (derived from Rust state) | outline tree, dirty state indicator, export availability |

### Section 9.3 — Component Hierarchy

**App-Level Components:**
- `AppShell`, `TitleBar`, `MenuBarBridge`, `WorkspaceSidebar`, `EditorWorkspace`, `OutlinePanel`, `StatusSurface`, `ModalHost`, `ToastHost`, `CommandPalette`

**Workspace Components:**
- `WorkspaceTree`, `WorkspaceTreeNode`, `FileRow`, `FolderRow`, `WorkspaceContextMenu`, `RecentItemsList`

**Editor Components:**
- `EditorCanvas`, `DocumentViewport`, `BlockRenderer`, `InlineRenderer`, `CaretLayer`, `SelectionLayer`, `DropCursorLayer`, `LinkPopover`, `ImageBlock`, `CodeFenceBlock`, `TableBlock`, `QuoteBlock`, `TaskListItem`, `HeadingBlock`

**Utility Components:**
- `SearchBar`, `ReplaceBar`, `ExportDialog`, `PreferencesDialog`, `ShortcutHint`, `EmptyState`, `RecoveryBanner`, `InlineNotice`

---

## Article X: Testing Strategy

### Section 10.1 — Unit Tests

Cover:
- Parser behavior
- Cursor movement
- Selection logic
- Editing commands
- Undo/redo
- Serializer
- Settings and recovery

### Section 10.2 — Editing Invariant Regression Tests

Maintain fixture-based tests for scenarios such as:
- Enter on list item
- Enter on empty list item
- Backspace at heading/list/quote boundaries
- Link text edits
- Cursor motion across inline code and emphasis
- Toggling task lists
- Table edit edge cases

### Section 10.3 — Fixture Tests

Use real Markdown fixtures for:
- Parse expectations
- Roundtrip save behavior
- Export regressions
- Tricky documents from real-world authoring

### Section 10.4 — Snapshot Tests

Useful for:
- HTML export
- Semantic tree output
- Theme token outputs

### Section 10.5 — Integration Tests

Test:
- Open/edit/save flows
- Open folder and navigate files
- External file changes
- Recovery flows
- Export command flows

### Section 10.6 — Performance Benchmarks

Track:
- Startup time
- Parse time
- Edit latency
- Serialization time
- Export time

---

## Article XI: Engineering Standards

### Section 11.1 — Rust Standards

- `cargo fmt` enforced in CI
- `clippy` required in CI
- Avoid unnecessary `unsafe`
- Any `unsafe` must be isolated and justified with comments

### Section 11.2 — Code Organization

- Business logic must not live in UI event handlers
- Public APIs must be documented
- Non-trivial features need tests and short design rationale

### Section 11.3 — PR Standards

- Small PRs preferred
- Issue or rationale linked
- UI changes include screenshots/gifs when practical
- User-visible behavior changes update docs or fixtures

---

## Article XII: Release Engineering

### Section 12.1 — Release Channels

- Nightly
- Beta
- Stable

### Section 12.2 — CI/CD Requirements

- Build matrix for macOS, Windows, Linux
- Lint + tests required on PRs
- Release notes generated consistently
- Checksums published with releases

### Section 12.3 — Versioning

- Semver for core crates where practical
- Semantic app releases preferred

---

## Article XIII: MVP Release Criteria

RustNote is ready for MVP release when:

1. Users can reliably create, open, edit, save, and reopen Markdown files
2. The single-pane editing experience feels readable and stable for supported syntax
3. Core authoring flows for headings, lists, links, images, code blocks, and tables are usable in real work
4. Focus mode and typewriter mode are available and genuinely useful
5. Folder-based workspace flow is viable
6. Autosave and recovery work in common tested scenarios
7. HTML and PDF export are production-usable for ordinary documents
8. Editing invariants are covered by tests and fixtures
9. Stable builds are published for macOS, Windows, Linux

---

## Article XIV: Governance

### Section 14.1 — Initial Structure

- 1–3 maintainers with merge rights
- GitHub Issues for bugs/features
- GitHub Discussions for design topics and Q&A
- ADRs for key architectural changes
- Milestone-driven roadmap visible to contributors

### Section 14.2 — Issue Labels

- `good first issue`
- `help wanted`
- `bug`
- `regression`
- `performance`
- `ux`
- `parser`
- `editor-engine`
- `workspace`
- `export`
- `theme`
- `testing`
- `documentation`
- `security`
- `breaking change`

---

## Article XV: Risk Mitigation

### RISK 1: Interaction Semantics Are Hard

**The biggest risk is not parsing Markdown. It is making live rendering, cursor movement, selection, and structural editing feel natural.**

Mitigation:
- Invest early in editor-engine design
- Maintain editing-invariant regression suites
- Treat cursor bugs as product-critical defects

### RISK 2: Table Editing Can Consume the Roadmap

Mitigation:
- Start with constrained, safe table interactions
- Optimize for correctness before spreadsheet-like richness

### RISK 3: UI-Layer Logic Creep Can Weaken Architecture

Mitigation:
- Keep correctness in Rust services
- Frontend should dispatch commands and render derived state

### RISK 4: Export Disappointment Can Hurt Adoption

Mitigation:
- Prioritize HTML/PDF fidelity
- Test representative docs
- Document limits clearly

---

## Article XVI: Initial Backlog

### P0 (MVP Core)
- Repo bootstrap
- Open/save file
- Parser integration
- Live rendering basics
- Editor engine foundation
- List/quote behaviors
- Autosave/recovery skeleton

### P1 (Essential Features)
- Links/images/code blocks
- Workspace tree
- Tables
- Outline panel
- Find/replace
- Themes

### P2 (Polish)
- Focus mode
- Typewriter mode
- Export polish
- Recent files/folders
- Settings persistence

### P3 (Future)
- Frontmatter helpers
- Local history
- Custom CSS
- Math support

---

## Article XVII: Required Documentation

At minimum:
- `README.md`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- Architecture overview
- Supported Markdown behavior doc
- Known limitations doc
- Testing strategy doc
- Roadmap or milestone doc

---

## Article XVIII: Amendment Process

This constitution may be amended through:

1. A GitHub Discussion proposing the change with rationale
2. Maintainer consensus after 7 days of discussion
3. A PR updating this document with `[AMENDMENT]` prefix in title
4. Merge by maintainer after review

Amendments must not violate the Core Principles (Article II) or the Non-Negotiable Experience Invariants (Article III) without explicit user consent.

---

*Last Updated: 2026-04-11*  
*Document Status: Active*  
*Version: 1.0*
