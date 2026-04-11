# RustNote Project Constitution

## Document Overview

| Field | Value |
|---|---|
| Project Name | RustNote |
| Product Codename | RustNote |
| Version | 2.0 |
| Document Type | Project Constitution |
| Status | Active |
| Created | 2026-04-11 |
| Implementation Language | Rust |
| License | MIT OR Apache-2.0 |
| Runtime | Tauri V1 (Desktop, cross-platform) |

---

## 1. Identity and Purpose

### 1.1 Product Identity

RustNote is an open-source, local-first, writing-first Markdown editor built in Rust. It is a single-pane, seamless reader-writer experience in which users write in a visually formatted document while the underlying source of truth remains plain Markdown.

### 1.2 Product Vision Statement

Reproduce and eventually surpass the reasons users love Typora: uninterrupted writing flow, readable documents while editing, Markdown syntax that does not get in the way, natural low-friction authoring, and portable trustworthy local files.

### 1.3 Target Users

- Writers and note-takers who need a clean writing space with Markdown portability
- Developers and technical authors who need README/docs/spec editing with Git-friendly output and code-block correctness
- Product managers and researchers who need readable drafting with headings, lists, tables, images, and export

---

## 2. Core Product Principles

These principles are non-negotiable and guide all product decisions.

### PRINCIPLE 1: Write First

**Statement:** Writing flow is the highest priority in RustNote. Every design decision must preserve the writer's cognitive momentum.

**Rules:**
- The editor must never interrupt the writer with modal dialogs during active typing
- All auxiliary actions (save, export, search) must be accessible without leaving the editing context
- Focus mode and typewriter mode must be one shortcut away
- Settings changes must apply immediately without requiring editor restart
- Single-pane live rendering is the default editing experience

**Rationale:** Writers lose momentum when interrupted. This principle ensures RustNote remains a tool that serves the writing, not the reverse.

---

### PRINCIPLE 2: Markdown as Source of Truth

**Statement:** The Markdown source file is always the authoritative document. The rendered view is a presentation layer, never a separate data model.

**Rules:**
- All document content must be serializable to valid Markdown at any moment
- Users must always be able to view and edit raw Markdown source
- Rendered editing must never silently modify or corrupt Markdown structure
- File I/O must read and write standard Markdown files (.md extension)
- Supported constructs must survive edit-save-reopen cycles reliably
- Known lossy or unsupported constructs must be documented

**Rationale:** Locking users into a proprietary format destroys trust. Markdown portability is the product's fundamental value proposition.

---

### PRINCIPLE 3: Live Rendering Must Feel Natural

**Statement:** WYSIWYG rendering should enhance writing without creating magical or unpredictable behavior.

**Rules:**
- Cursor movement through rendered content must be intuitive and predictable
- Selection behavior around links, images, and tables must follow user expectations
- Structural elements (lists, blockquotes, code blocks) must feel editable
- Rendered formatting must not hide Markdown syntax in ways that confuse power users
- No cursor traps in inline formatting, links, code spans, tables, images, and quotes

**Rationale:** Live rendering is the core differentiator. If it feels broken, the product fails its value proposition.

---

### PRINCIPLE 4: Performance Is a Feature

**Statement:** RustNote must feel near-instant on modern hardware. Performance is not optional optimization — it is a core feature.

**Rules:**
- Cold startup must complete in under 2 seconds on typical hardware
- Typing latency must be imperceptible (< 50ms response time)
- Large documents (5MB+) must remain usable without major frame drops
- Memory footprint must remain competitive with native desktop tools

**Rationale:** Writers are sensitive to latency. A sluggish editor destroys the writing experience and drives users to alternatives.

---

### PRINCIPLE 5: Local-First Privacy

**Statement:** User content belongs to the user. No cloud transmission or proprietary storage in MVP unless explicitly enabled.

**Rules:**
- All documents are stored as native files on the local file system
- No automatic cloud sync or transmission in MVP
- Auto-save writes to local temporary storage, not cloud services
- User must explicitly enable any network features
- No network dependency in MVP editing path

**Rationale:** Privacy is trust. Writers handle sensitive content. Local-first is both a practical principle and a marketing differentiator.

---

### PRINCIPLE 6: Beautiful Typography Matters

**Statement:** The reading experience must be visually polished. Typography is not an afterthought — it is part of the core experience.

**Rules:**
- Default themes must have professional, readable typography
- Line height, font size, and content width must be configurable
- Code blocks, tables, and blockquotes must have distinct, readable styling
- Theme switching must be instant and preserve document state
- Light and dark themes required as minimum

**Rationale:** Writers stare at their screen for hours. Poor typography causes fatigue and reduces writing quality.

---

### PRINCIPLE 7: Invisible UI Beats Visible Complexity

**Statement:** Advanced features must be accessible but not intrusive. The default experience should be simple; power tools should be discoverable.

**Rules:**
- Core editing UI must remain minimal in MVP
- Default UI should remain visually quiet
- Toolbar, controls, and settings must not dominate the writing surface
- Focus mode and typewriter mode are part of product identity, not decoration
- Command palette must provide access to all advanced features

**Rationale:** Simple tools get used; complex tools get ignored. Clutter creates cognitive overhead that hurts the writing experience.

---

### PRINCIPLE 8: Rust Owns Correctness

**Statement:** Parsing, editing semantics, serialization, recovery, and export-critical logic belong in Rust-owned boundaries.

**Rules:**
- Document parsing, serialization, and edit operations must live in a standalone Rust crate
- Desktop shell (Tauri) must be thin, delegating to Rust core for all document operations
- Export pipeline must be callable from Rust without UI dependencies
- Anything affecting Markdown fidelity, cursor/selection semantics, structural editing, save/recovery, export must live in Rust interfaces

**Rationale:** WYSIWYG complexity explodes without clear separation. A Typora-like editor fails when too much logic leaks into ad hoc UI behavior.

---

## 3. Technical Architecture Principles

### PRINCIPLE 9: Tauri + Rust Stack

**Statement:** The desktop shell uses Tauri V1 with a Rust backend. This is the committed technical direction for MVP.

**Rules:**
- Editor logic, parsing, serialization, and state management are Rust
- Frontend handles presentation and user input handling only
- Platform-specific code is isolated to the shell layer
- Desktop integration (file dialogs, menus, shortcuts) uses Tauri APIs

**Rationale:** Tauri provides fast cross-platform delivery with a mature ecosystem. The alternative (native UI stacks) would delay MVP.

---

### PRINCIPLE 10: Document Model Separation

**Statement:** The architecture must clearly separate source Markdown, semantic document model, editing layer, and presentation layer.

**Rules:**
- Markdown source → AST/Parsetree (Rust) → Semantic Model → Editing Layer (selection, commands, undo/redo) → Presentation Model → Rendered View
- Incremental updates must flow through this pipeline efficiently
- No direct mutation of source from rendered view
- Undo/redo must operate on semantic operations, not text diffs

**Rationale:** This architecture prevents cursor and serialization bugs that plague WYSIWYG editors.

---

### PRINCIPLE 11: Markdown Flavor: CommonMark + GFM

**Statement:** RustNote supports CommonMark baseline plus GitHub Flavored Markdown extensions.

**Rules:**
- Core syntax follows CommonMark specification
- Extensions include: tables, task lists, strikethrough, autolinks, fenced code blocks
- Frontmatter (YAML) read/write supported as text block
- Math support is optional MVP stretch goal

**Rationale:** GFM is the de facto standard for Markdown. Compatibility with GitHub-flavored documents is essential for the target user base.

---

### PRINCIPLE 12: Modular Crate Architecture

**Statement:** The Rust core must be decomposed into clear crate responsibilities for maintainability and testability.

**Rules:**
- core-model: semantic document structures, positions, ranges, identifiers
- markdown-parser: parse Markdown to semantic representation
- editor-engine: cursor, selection, commands, smart behaviors, undo/redo
- serializer: semantic model to Markdown output
- workspace: file IO, recent files, file watching
- export: HTML and PDF export
- theme: theme tokens, typography, focus/typewriter config
- settings: persisted configuration
- recovery: autosave snapshots, crash recovery

**Rationale:** Clear boundaries enable independent development, testing, and potential future frontend variations.

---

## 4. UX Principles

### PRINCIPLE 13: Single-Pane Invariant

**Statement:** The main experience is one editing canvas. Split preview is not the core workflow.

**Rules:**
- Single-pane live rendering is the default and primary mode
- Users should never need to mode-switch to understand document appearance
- Source-only mode and split preview are post-MVP features

**Rationale:** Single-pane is the Typora-like experience that differentiates RustNote. Split view is a fallback for debugging, not the primary experience.

---

### PRINCIPLE 14: Readability Invariant

**Statement:** The document should remain pleasant to read during editing.

**Rules:**
- Headings should look like headings while editing
- Lists should look like lists
- Links and images should appear as document content, not primarily as raw syntax
- Code blocks render with syntax highlighting

**Rationale:** The writing flow feels uninterrupted when the document looks readable while being edited.

---

### PRINCIPLE 15: Structural Editing Invariant

**Statement:** Enter, Backspace, Tab, and Shift+Tab must behave consistently for lists, quotes, and nested structures.

**Rules:**
- Enter continues list item
- Enter on empty list item exits list
- Backspace at expected structural boundaries degrades list predictably
- Tab/Shift+Tab indent and outdent nested items where applicable
- Structural editing must not cause surprising Markdown corruption

**Rationale:** High-frequency actions must feel effortless. Lists, headings, links are product-defining paths.

---

### PRINCIPLE 16: Keyboard-First Operation

**Statement:** All core actions must be accessible via keyboard. Mouse is optional; keyboard is primary.

**Rules:**
- All commands available in command palette
- Standard shortcuts respect platform conventions (Cmd on macOS, Ctrl on Windows/Linux)
- Focus mode, theme toggle, save, search must be keyboard-accessible
- Essential actions reachable by keyboard

**Rationale:** Writers prefer keyboards. Mouse interaction breaks flow. Accessibility requires keyboard support.

---

### PRINCIPLE 17: File Operations Are First-Class

**Statement:** File and workspace management is core functionality, not an afterthought.

**Rules:**
- Sidebar workspace tree must support create, rename, delete, duplicate
- Recent files and workspaces shown on launch
- External file changes detected and handled (prompt or auto-reload)
- Unsaved state must be clearly indicated
- Drag-and-drop file open supported

**Rationale:** Local-first means treating files as first-class citizens. Poor file management undermines the core value proposition.

---

## 5. Reliability Principles

### PRINCIPLE 18: Crash Recovery

**Statement:** Writers must never lose work due to crashes. Auto-save and recovery are mandatory.

**Rules:**
- Unsaved changes periodically snapshotted to local recovery storage
- On crash, recovery flow offers to restore recent drafts
- Recovery snapshots expire after user-defined period (default: 7 days)
- Recovery success must be clearly indicated

**Rationale:** Losing work to a crash is the most painful experience a writer can have. Recovery is trust-building.

---

### PRINCIPLE 19: Atomic Saves

**Statement:** Save operations must be atomic where possible. No partial writes.

**Rules:**
- Write to temporary file, then rename (atomic on most filesystems)
- Preserve original file until write completes successfully
- Backup or Recovery Snapshot available on save failure

**Rationale:** Partial writes corrupt documents. Corruption destroys trust permanently.

---

### PRINCIPLE 20: Safe Rendering

**Statement:** Raw HTML handling policy must be explicit to prevent security vulnerabilities.

**Rules:**
- Export sanitization and trust model documented
- Raw HTML in Markdown handled according to explicit policy
- Malformed documents must not crash the parser

**Rationale:** Security is not optional. Malicious Markdown could compromise user systems.

---

## 6. Process Principles

### PRINCIPLE 21: MVP Feature Focus

**Statement:** RustNote MVP includes only features that directly serve the core writing experience.

**Rules:**
- MVP scope: open/save, live editing, core syntax, file tree, search, themes, export (HTML/PDF), auto-save, focus mode, typewriter mode
- Features not in MVP scope must not be started until core is solid
- Post-MVP features are explicitly deferred (see Section 9)

**Rationale:** Scope creep kills products. Focus enables shipping. MVP is a discipline.

---

### PRINCIPLE 22: Test-Driven Development

**Statement:** Core editing logic requires test coverage. Tests validate Markdown parsing, serialization, and edit invariants.

**Rules:**
- Markdown parsing and serialization require unit tests
- Editing operations (smart enter, backspace, list continuation) require regression tests
- Export pipeline requires round-trip tests (Markdown → AST → Rendered → Export → Match original)
- Performance benchmarks required for startup and typing latency
- Editing invariant regression tests: Enter on list item, Backspace at boundaries, cursor across inline code, toggling task lists, table edge cases

**Rationale:** WYSIWYG editing is complex and fragile. Without tests, regressions destroy the product. The hardest problem is interaction semantics.

---

### PRINCIPLE 23: ADR for Architecture

**Statement:** Key architectural decisions require documented ADRs.

**Rules:**
- Write ADRs for: Why Tauri for V1, exact Markdown flavor, internal document model, source fidelity guarantees, table editing scope, raw HTML policy, PDF export choice
- Small, reviewable pull requests preferred
- Non-trivial features need tests and short design rationale

**Rationale:** ADRs create institutional knowledge and enable contributors to understand design rationale.

---

## 7. Feature Scope Boundaries

### 7.1 MVP Must Have

Required for MVP release:

- Open/save Markdown files (.md)
- Single-pane live Markdown editing
- Core Markdown syntax support: headings H1-H6, bold, italic, strikethrough, inline code, fenced code blocks, ordered/unordered lists, task lists, blockquotes, links, images, horizontal rules, tables, frontmatter
- Workspace sidebar: create, rename, delete files/folders
- In-document search and replace (find next/previous, replace one/all)
- Theme support (light and dark)
- Focus mode
- Typewriter mode
- HTML export (standalone and linked-assets mode)
- PDF export (configurable page size and margins)
- Auto-save and crash recovery
- Syntax highlighting in code fences
- Relative asset path support
- Recent files and recent folders
- Basic outline / table-of-contents panel

### 7.2 Explicitly Out of Scope for MVP

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph
- Workspace complexity comparable to IDEs

### 7.3 Stretch Goals (Post-MVP)

- Frontmatter helper UI
- Local document history
- Math rendering
- Custom CSS support
- Improved paste-from-rich-text conversion

---

## 8. Non-Functional Requirements Summary

### Performance

| Metric | Target |
|---|---|
| Cold startup time | < 2 seconds |
| Typing latency | < 50ms imperceptible |
| Large document (5MB+) | Usable without major frame drops |
| Memory footprint | Competitive with native tools |

### Reliability

| Metric | Target |
|---|---|
| Crash recovery | Restores unsaved work |
| Save atomicity | No partial writes |
| Document corruption | Prevented via backup strategy |
| Crash-free goal | Stable releases target >99% session success |

### Platform

| Metric | Target |
|---|---|
| Supported platforms | macOS, Windows, Linux |
| Keyboard conventions | Platform-native (Cmd vs Ctrl) |
| File dialogs | Native OS dialogs |

### Privacy

| Metric | Target |
|---|---|
| Cloud transmission | None in MVP (local-first) |
| Document storage | Native file system (.md files) |
| Config storage | Local only |

---

## 9. Experience Invariants (Non-Negotiable UX)

### 9.1 Single-Pane Invariant

- The main experience is one editing canvas
- Split preview is not the core workflow
- Users should never need to mode-switch to understand document appearance

### 9.2 Readability Invariant

- Headings should look like headings while editing
- Lists should look like lists
- Links and images should appear as document content, not primarily as raw syntax
- The document should remain pleasant to read during editing

### 9.3 Cursor Invariant

- Cursor movement must be predictable around inline formatting, links, code spans, tables, images, and quotes
- No cursor traps
- Selection boundaries must feel natural

### 9.4 Structure Invariant

- Enter, Backspace, Tab, and Shift+Tab must behave consistently for lists, quotes, and nested structures
- Structural editing must not cause surprising Markdown corruption

### 9.5 Fidelity Invariant

- Editing and rendering must not silently destroy supported Markdown constructs
- When a construct cannot be preserved perfectly, behavior must be documented and tested

### 9.6 Calmness Invariant

- Default UI should remain visually quiet
- Toolbar, controls, and settings must not dominate the writing surface
- Focus mode and typewriter mode are part of product identity, not decoration

### 9.7 Local-Trust Invariant

- The app should behave like a trustworthy steward of local files
- Save, autosave, recovery, reload, and external-change handling must be dependable

---

## 10. Governance

### 10.1 Amendment Process

This constitution may be amended through the following process:

1. **Proposal:** Any team member may propose an amendment
2. **Discussion:** Amendment discussed in pull request or design meeting
3. **Approval:** Requires consensus from 2+ team members
4. **Documentation:** Amendment recorded with rationale in this document
5. **Propagation:** Dependent artifacts updated accordingly

### 10.2 Version Policy

- **MAJOR**: Backward incompatible governance/principle removals or redefinitions
- **MINOR**: New principle/section added or materially expanded guidance
- **PATCH**: Clarifications, wording, typo fixes, non-semantic refinements

### 10.3 Compliance Review

- All new features must be evaluated against this constitution
- Architecture decisions require principle alignment verification
- UI changes must be evaluated against experience invariants

---

## 11. Document History

| Version | Date | Author | Changes |
|---|---|---|---|
| 1.0 | 2026-04-11 | Sisyphus | Initial constitution derived from PRD |
| 2.0 | 2026-04-11 | Sisyphus | Updated with PRD v2.1 refined implementation details, added experience invariants, expanded architecture principles |

---

## 12. Appendix: Key Product Decisions

### Decision 1: Single-Pane Default

**Decision:** Single-pane live rendering is the default editing mode.

**Alternatives Considered:** Split view (source + preview), source-only mode

**Rationale for Decision:** Single-pane is the Typora-like differentiator. Split view is available post-MVP.

### Decision 2: Markdown Flavor

**Decision:** CommonMark + GitHub Flavored Markdown.

**Alternatives Considered:** CommonMark only, full GitHub Flavored Markdown + extensions (math, footnotes)

**Rationale for Decision:** GFM is the practical standard. Most users expect table and task list support.

### Decision 3: Tauri Shell

**Decision:** Tauri V1 for desktop shell.

**Alternatives Considered:** Native Qt, native UI frameworks (Cocoa, Win32, GTK)

**Rationale for Decision:** Tauri provides fastest path to cross-platform with mature ecosystem. Allows focus on Rust core.

### Decision 4: Local-First

**Decision:** No cloud sync in MVP.

**Alternatives Considered:** Early cloud integration, optional sync in MVP

**Rationale for Decision:** Local-first builds trust. Cloud sync introduces complexity and privacy concerns. Defer to post-MVP.

### Decision 5: The Hard Problem

**Decision:** Invest early in editor-engine design; treat cursor bugs as product-critical defects.

**Alternatives Considered:** Defer interaction semantics until UI layer is stable

**Rationale for Decision:** The biggest risk is not parsing Markdown. It is making live rendering, cursor movement, selection, and structural editing feel natural.

---

*End of Constitution*