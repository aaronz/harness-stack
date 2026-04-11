# RustNote Project Constitution

## Document Overview

| Field | Value |
|---|---|
| Project Name | RustNote |
| Product Codename | RustNote |
| Version | 1.0 |
| Document Type | Project Constitution |
| Status | Active |
| Created | 2026-04-11 |
| Implementation Language | Rust |

---

## 1. Identity and Purpose

### 1.1 Product Identity

RustNote is a desktop-first, cross-platform WYSIWYG Markdown editor that combines the seamless live-preview writing experience of Typora with the performance, stability, and maintainability benefits of Rust.

### 1.2 Product Vision Statement

Build the best Rust-native Markdown writing tool for serious users who value clarity, speed, portability, and a delightful writing experience — as clean as a writing app, as capable as a modern Markdown tool, as fast and reliable as a native desktop application.

### 1.3 Target Users

- Technical Writers who need clean Markdown workflows with headings, tables, images, code blocks, export, and file organization
- Developers writing README files, design docs, specs, changelogs, architecture notes
- Knowledge Workers using Markdown for personal notes, meeting notes, drafts
- Students and Researchers needing structured writing, citation placeholders, export

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

**Rationale:** Writers lose momentum when interrupted. This principle ensures RustNote remains a tool that serves the writing, not the reverse.

---

### PRINCIPLE 2: Markdown as Source of Truth

**Statement:** The Markdown source file is always the authoritative document. The rendered view is a presentation layer, never a separate data model.

**Rules:**
- All document content must be serializable to valid Markdown at any moment
- Users must always be able to view and edit raw Markdown source
- Rendered editing must never silently modify or corrupt Markdown structure
- File I/O must read and write standard Markdown files (.md extension)

**Rationale:** Locking users into a proprietary format destroys trust. Markdown portability is the product's fundamental value proposition.

---

### PRINCIPLE 3: Live Rendering Must Feel Natural

**Statement:** WYSIWYG rendering should enhance writing without creating magical or unpredictable behavior.

**Rules:**
- Cursor movement through rendered content must be intuitive and predictable
- Selection behavior around links, images, and tables must follow user expectations
- Structural elements (lists, blockquotes, code blocks) must feel editable
- Rendered formatting must not hide Markdown syntax in ways that confuse power users

**Rationale:** Live rendering is the core differentiator. If it feels broken, the product fails its value proposition.

---

### PRINCIPLE 4: Performance Is a Feature

**Statement:** RustNote must feel near-instant on modern hardware. Performance is not optional optimization — it is a core feature.

**Rules:**
- Cold startup must complete in under 2 seconds on typical hardware
- Typing latency must be imperceptible (< 50ms response time)
- Large documents (10,000+ lines) must remain usable without major frame drops
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

**Rationale:** Privacy is trust. Writers handle sensitive content. Local-first is both a practical principle and a marketing differentiator.

---

### PRINCIPLE 6: Beautiful Typography Matters

**Statement:** The reading experience must be visually polished. Typography is not an afterthought — it is part of the core experience.

**Rules:**
- Default themes must have professional, readable typography
- Line height, font size, and paragraph width must be configurable
- Code blocks, tables, and blockquotes must have distinct, readable styling
- Theme switching must be instant and preserve document state

**Rationale:** Writers stare at their screen for hours. Poor typography causes fatigue and reduces writing quality.

---

### PRINCIPLE 7: Power Without Clutter

**Statement:** Advanced features must be accessible but not intrusive. The default experience should be simple; power tools should be discoverable.

**Rules:**
- Core editing UI must remain minimal in MVP
- Command palette must provide access to all advanced features
- Settings must be organized by frequency of use, not technical complexity
- Features that most users don't need must not appear in the default UI

**Rationale:** Simple tools get used; complex tools get ignored. Clutter creates cognitive overhead that hurts the writing experience.

---

### PRINCIPLE 8: Modular Rust Core

**Statement:** The Rust editing engine must be designed for reuse. Platform shell is thin; core logic is portable.

**Rules:**
- Document parsing, serialization, and edit operations must live in a standalone Rust crate
- Desktop shell (Tauri) must be thin, delegating to Rust core for all document operations
- Export pipeline must be callable from Rust without UI dependencies
- Architecture must support future CLI, mobile, or web frontends

**Rationale:** Reusable core logic multiplies value. A Rust core enables the product to evolve beyond desktop-only.

---

## 3. Technical Architecture Principles

### PRINCIPLE 9: Tauri + Rust Stack

**Statement:** The desktop shell uses Tauri with a Rust backend. This is the committed technical direction for MVP.

**Rules:**
- Editor logic, parsing, serialization, and state management are Rust
- Frontend handles presentation and user input handling only
- Platform-specific code is isolated to the shell layer
- Desktop integration (file dialogs, menus, shortcuts) uses Tauri APIs

**Rationale:** Tauri provides fast cross-platform delivery with a mature ecosystem. The alternative (native UI stacks) would delay MVP.

---

### PRINCIPLE 10: Document Model Separation

**Statement:** The architecture must clearly separate source Markdown, semantic document model, and presentation state.

**Rules:**
- Markdown source → AST/Parsetree (Rust) → Presentation Model (Frontend) → Rendered View
- Incremental updates must flow through this pipeline efficiently
- No direct mutation of source from rendered view
- Undo/redo must operate on semantic operations, not text diffs

**Rationale:** WYSIWYG complexity explodes without clear separation. This architecture prevents cursor and serialization bugs.

---

### PRINCIPLE 11: Markdown Flavor: CommonMark + GFM

**Statement:** RustNote supports CommonMark baseline plus GitHub Flavored Markdown extensions.

**Rules:**
- Core syntax follows CommonMark specification
- Extensions include: tables, task lists, strikethrough, autolinks, fenced code blocks
- Frontmatter (YAML) read/write supported
- Math support is optional MVP stretch goal

**Rationale:** GFM is the de facto standard for Markdown. Compatibility with GitHub-flavored documents is essential for the target user base.

---

## 4. UX Principles

### PRINCIPLE 12: Single-Pane Editing

**Statement:** The default editing experience is single-pane live rendering. Split view is optional, not default.

**Rules:**
- Single-pane live rendering is the default and primary mode
- Source-only mode and split preview are post-MVP features
- Toggle between modes must be instant

**Rationale:** Single-pane is the Typora-like experience that differentiates RustNote. Split view is a fallback for debugging, not the primary experience.

---

### PRINCIPLE 13: Keyboard-First Operation

**Statement:** All core actions must be accessible via keyboard. Mouse is optional; keyboard is primary.

**Rules:**
- All commands available in command palette
- Standard shortcuts respect platform conventions (Cmd on macOS, Ctrl on Windows/Linux)
- Focus mode, theme toggle, save, search must be keyboard-accessible
- Tab navigation between UI panels must be supported

**Rationale:** Writers prefer keyboards. Mouse interaction breaks flow. Accessibility requires keyboard support.

---

### PRINCIPLE 14: File Operations Are First-Class

**Statement:** File and workspace management is core functionality, not an afterthought.

**Rules:**
- Sidebar workspace tree must support create, rename, delete, duplicate
- Recent files and workspaces shown on launch
- External file changes detected and handled (prompt or auto-reload)
- Unsaved state must be clearly indicated

**Rationale:** Local-first means treating files as first-class citizens. Poor file management undermines the core value proposition.

---

## 5. Reliability Principles

### PRINCIPLE 15: Crash Recovery

**Statement:** Writers must never lose work due to crashes. Auto-save and recovery are mandatory.

**Rules:**
- Unsaved changes periodically snapshotted to local recovery storage
- On crash, recovery flow offers to restore recent drafts
- Recovery snapshots expire after user-defined period (default: 7 days)
- Recovery success must be clearly indicated

**Rationale:** Losing work to a crash is the most painful experience a writer can have. Recovery is trust-building.

---

### PRINCIPLE 16: Atomic Saves

**Statement:** Save operations must be atomic where possible. No partial writes.

**Rules:**
- Write to temporary file, then rename (atomic on most filesystems)
- Preserve original file until write completes successfully
- Backup or Recovery Snapshot available on save failure

**Rationale:** Partial writes corrupt documents. Corruption destroys trust permanently.

---

## 6. Process Principles

### PRINCIPLE 17: MVP Feature Focus

**Statement:** RustNote MVP includes only features that directly serve the core writing experience.

**Rules:**
- MVP scope: open/save, live editing, core syntax, file tree, search, themes, export (HTML/PDF), auto-save
- Features not in MVP scope must not be started until core is solid
- Post-MVP features are explicitly deferred (see Section 9)

**Rationale:** Scope creep kills products. Focus enables shipping. MVP is a discipline.

---

### PRINCIPLE 18: Test-Driven Development

**Statement:** Core editing logic requires test coverage. Tests validate Markdown parsing, serialization, and edit invariants.

**Rules:**
- Markdown parsing and serialization require unit tests
- Editing operations (smart enter, backspace, list continuation) require regression tests
- Export pipeline requires round-trip tests (Markdown → AST → Rendered → Export → Match original)
- Performance benchmarks required for startup and typing latency

**Rationale:** WYSIWYG editing is complex and fragile. Without tests, regressions destroy the product.

---

## 7. Feature Scope Boundaries

### 7.1 MVP Must Have

Required for MVP release:

- Open/save Markdown files
- Single-pane live Markdown editing
- Core Markdown syntax support (headings, bold, italic, links, lists, code blocks, tables, images)
- File tree/workspace support
- In-document search and replace
- Theme support (light/dark)
- HTML export
- PDF export
- Auto-save and crash recovery
- Syntax highlighting in code fences
- Recent files/home screen

### 7.2 Should Have (Pre-MVP if time permits)

- Outline panel
- Typewriter mode
- Frontmatter support
- Better paste conversion (rich text → Markdown)

### 7.3 Won’t Have in MVP

Explicitly deferred beyond MVP:

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- AI co-writing
- Advanced publishing integrations
- DOCX/EPUB export
- Global search across folders
- Template system
- Math rendering
- Mermaid/diagram support

---

## 8. Non-Functional Requirements Summary

### Performance

| Metric | Target |
|---|---|
| Cold startup time | < 2 seconds |
| Typing latency | < 50ms imperceptible |
| Large document (10K+ lines) | Usable without major frame drops |
| Memory footprint | Competitive with native tools |

### Reliability

| Metric | Target |
|---|---|
| Crash recovery | Restores unsaved work |
| Save atomicity | No partial writes |
| Document corruption | Prevented via backup strategy |

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

## 9. Amendment Process

This constitution may be amended through the following process:

1. **Proposal:** Any team member may propose an amendment
2. **Discussion:** Amendment discussed inpull request or design meeting
3. **Approval:** Requires consensus from 2+ team members
4. **Documentation:** Amendment recorded with rationale in this document
5. **Propagation:** Dependent artifacts updated accordingly

---

## 10. Document History

| Version | Date | Author | Changes |
|---|---|---|---|
| 1.0 | 2026-04-11 | Sisyphus | Initial constitution derived from PRD |

---

## 11. Appendix: Key Product Decisions

### Decision 1: Single-Pane Default

**Decision:** Single-pane live rendering is the default editing mode.

**Alternatives Considered:** Split view (source + preview), source-only mode

**Rationale for Decision:** Single-pane is the Typora-like differentiator. Split view is available post-MVP.

### Decision 2: Markdown Flavor

**Decision:** CommonMark + GitHub Flavored Markdown.

**Alternatives Considered:** CommonMark only, full GitHub Flavored Markdown + extensions (math, footnotes)

**Rationale for Decision:** GFM is the practical standard. Most users expect table and task list support.

### Decision 3: Tauri Shell

**Decision:** Tauri for desktop shell.

**Alternatives Considered:** Native Qt, native UI frameworks (Cocoa, Win32, GTK)

**Rationale for Decision:** Tauri provides fastest path to cross-platform with mature ecosystem. Allows focus on Rust core.

### Decision 4: Local-First

**Decision:** No cloud sync in MVP.

**Alternatives Considered:** Early cloud integration, optional sync in MVP

**Rationale for Decision:** Local-first builds trust. Cloud sync introduces complexity and privacy concerns. Defer to post-MVP.

---

*End of Constitution*