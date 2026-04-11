# RustNote Project Constitution

**Document Version:** 2.1
**Project Name:** RustNote
**Document Type:** Project Constitution
**Primary Language:** Rust
**Target Runtime:** Desktop, cross-platform via Tauri V1
**License:** MIT OR Apache-2.0
**Status:** Ratified

---

## Preamble

This constitution establishes the foundational principles, architectural rules, and quality standards that govern RustNote's development. It serves as the anchoring document for all architectural decisions, implementation guidelines, and project governance.

RustNote is an open-source, local-first, writing-first Markdown editor built in Rust. This constitution exists to ensure that every decision preserves the core promise: a calm, seamless, Typora-like editing experience where plain Markdown remains the durable source of truth.

---

## 1. Core Identity

### 1.1 Project Definition

RustNote is defined as:

> A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### 1.2 Strategic Positioning

RustNote occupies the space between:
- Developer editors that are too complex for writing
- Note apps that hide or weaken file portability
- Rich text tools that do not preserve Markdown cleanly

RustNote should feel:
- Calmer than VS Code
- More open and extensible than Typora
- More file-native than note databases
- More polished than most open-source Markdown editors

### 1.3 Success Vision

RustNote succeeds when users say:
- "I forget I'm editing Markdown."
- "It feels smoother than editing raw `.md` in a code editor."
- "It does not break my files."
- "Images, tables, lists, and export work without friction."
- "It is calm enough for long-form writing."

---

## 2. Core Principles

### 2.1 Principle: Write First

**Non-Negotiable Rules:**
- The user must feel like they are writing a document, not managing syntax
- The primary editing canvas must be a single pane where formatting is visually rendered inline
- Mode switching (edit/preview split) must never be the core workflow

**Rationale:** The product's competitive advantage lies in the uninterrupted writing flow. If users feel syntax, they will migrate to raw editors or full-featured IDEs. The writing experience IS the product.

---

### 2.2 Principle: Markdown as Source of Truth

**Non-Negotiable Rules:**
- Everything saved must remain valid, predictable CommonMark/GFM Markdown
- Supported constructs must survive edit-save-reopen cycles reliably
- Any lossy behavior must be documented, tested, and communicated to users
- The raw Markdown file must remain portable and Git-friendly

**Rationale:** Users choose Markdown for its durability and portability. If the editor silently degrades files, trust is irreparably broken. This is not a feature—it's the existential contract with the user.

---

### 2.3 Principle: Rendered While Editing, Not Previewed Separately

**Non-Negotiable Rules:**
- Single-pane live rendering is the default and primary identity
- Headings must look like headings while editing
- Lists must look like lists
- Links and images must appear as document content, not raw syntax
- The document must remain pleasant to read during editing

**Rationale:** This is the defining Typora-like characteristic. The editor must resolve the tension between "what you see is what you mean" and "what you get is what you save" without forcing users into a preview mode.

---

### 2.4 Principle: Invisible UI Beats Visible Complexity

**Non-Negotiable Rules:**
- Chrome, controls, and mode switches must remain minimal by default
- Focus mode must materially reduce visual distraction
- Toolbar and settings must not dominate the writing surface
- The default UI must remain visually quiet

**Rationale:** Writers need calm. Every visible control is a potential interruption. UI complexity is easier to add later than to remove after users have learned to ignore it.

---

### 2.5 Principle: High-Frequency Actions Must Feel Effortless

**Non-Negotiable Rules:**
- Lists (ordered, unordered, task lists) must have smart Enter/Backspace/Tab behavior
- Links and images must insert without breaking flow
- Code blocks must enter/exit unsurprisingly
- Tables must be correct and low-friction
- Paste must not create malformed Markdown in common cases
- Export (HTML, PDF) must be production-ready

**Rationale:** These are the product-defining interactions. A feature that exists but feels awkward has failed. These flows need interaction-level testing and polish, not just feature completion.

---

### 2.6 Principle: Local-First Always

**Non-Negotiable Rules:**
- Users own normal files on disk
- No network dependency in the MVP editing path
- File operations must behave like a trustworthy steward of local files
- Save, autosave, recovery, reload, and external-change handling must be dependable

**Rationale:** Local-first is not just a technical choice—it's a promise to users that their data remains theirs. Cloud sync and collaboration are explicitly out of scope for MVP.

---

### 2.7 Principle: Rust Owns Correctness

**Non-Negotiable Rules:**
- Parsing, editing semantics, serialization, recovery, and export-critical logic belong in Rust-owned boundaries
- Anything affecting Markdown fidelity, cursor/selection semantics, structural editing behavior, save/recovery correctness, or export correctness must live in Rust
- Business logic must not live in UI event handlers
- Front-end dispatches commands and renders derived state; it never becomes the source of truth

**Rationale:** A Typora-like editor fails when too much logic leaks into ad hoc UI behavior. The architecture must preserve a clear center of correctness in Rust.

---

### 2.8 Principle: Architectural Integrity Over Convenience

**Non-Negotiable Rules:**
- Document model layers (Source → Semantic → Editing → Presentation) must remain separate
- Public service interfaces must be narrow, not exposing unstable internal types
- Any `unsafe` Rust code must be isolated, justified, and documented
- Architecture decisions require ADRs; no ad hoc architectural changes

**Rationale:** Long-term maintainability and contributor onboarding depend on predictable architecture. Short-term convenience that corrupts architecture is a long-term liability.

---

## 3. User Experience Invariants

These invariants define whether the product actually feels Typora-like. They are non-negotiable experience requirements.

### 3.1 Single-Pane Invariant
- The main experience is one editing canvas
- Split preview is not the core workflow
- Users must never need to mode-switch to understand document appearance

### 3.2 Readability Invariant
- Headings must look like headings while editing
- Lists must look like lists
- Links and images must appear as document content, not raw syntax
- The document must remain pleasant to read during editing

### 3.3 Cursor Invariant
- Cursor movement must be predictable around inline formatting, links, code spans, tables, images, and quotes
- No cursor traps
- Selection boundaries must feel natural

### 3.4 Structure Invariant
- Enter, Backspace, Tab, and Shift+Tab must behave consistently for lists, quotes, and nested structures
- Structural editing must not cause surprising Markdown corruption

### 3.5 Fidelity Invariant
- Editing and rendering must not silently destroy supported Markdown constructs
- When a construct cannot be preserved perfectly, behavior must be documented and tested

### 3.6 Calmness Invariant
- Default UI must remain visually quiet
- Toolbar, controls, and settings must not dominate the writing surface
- Focus mode and typewriter mode are part of product identity, not decoration

### 3.7 Local-Trust Invariant
- The app must behave like a trustworthy steward of local files
- Save, autosave, recovery, reload, and external-change handling must be dependable

---

## 4. Supported Markdown Behavior

### 4.1 Required Syntax Support
- Headings H1-H6
- Bold, italic, strikethrough
- Inline code
- Fenced code blocks
- Ordered/unordered lists
- Task lists
- Blockquotes
- Links (including autolinks)
- Images
- Horizontal rules
- Tables (GFM)
- Frontmatter preserved as text block support

### 4.2 Markdown Flavor
- CommonMark baseline
- GFM support for tables, task lists, strikethrough, autolinks
- Supported behavior must be documented clearly in the repository

### 4.3 Serialization Contract
- Supported constructs must survive edit-save-reopen cycles reliably
- Known lossy or unsupported constructs must be documented
- Roundtrip behavior must be tested via fixture-based tests

---

## 5. Architecture

### 5.1 High-Level Architecture

**Recommended V1 Architecture:**
- **Rust core** for parsing, editing semantics, serialization, workspace logic, recovery, export orchestration
- **Tauri desktop shell** for packaging, windows, menus, file dialogs, OS integration
- **UI layer** for rendering and interaction, but never as owner of document correctness

### 5.2 Module Boundaries and Responsibilities

| Crate | Responsibility |
|-------|----------------|
| `core-model` | Semantic document structures, positions, ranges, identifiers, shared core types and errors |
| `markdown-parser` | Parse Markdown to semantic/intermediate representation, document supported flavor behavior |
| `editor-engine` | Cursor movement rules, selection logic, insert/delete/edit commands, smart Enter/Backspace/Tab behaviors, undo/redo, editing invariants |
| `serializer` | Semantic model to Markdown output, preserve supported constructs predictably |
| `workspace` | File IO, recent files/folders, file watching, asset path resolution |
| `export` | HTML export, PDF export, export option types and pipelines |
| `theme` | Theme tokens, typography defaults, focus/typewriter presentation config |
| `settings` | Persisted configuration, schema versioning |
| `recovery` | Autosave snapshots, crash recovery metadata, stale cleanup |
| `app-services` | Orchestration layer between UI shell and Rust core modules |

### 5.3 Public Service Boundaries

Narrow service interfaces are preferred over exposing many unstable internal types.

**Required Service Boundaries:**
- `DocumentService`: open/save/reload document
- `EditorService`: apply editor command, query selection or outline
- `WorkspaceService`: workspace file operations
- `ExportService`: export document
- `SettingsService`: configuration management
- `RecoveryService`: restore snapshots

### 5.4 Data Model Layers

The implementation must keep these layers separate:

1. **Source Layer:** Raw Markdown text
2. **Semantic Layer:** Parsed structures (headings, paragraphs, list items, tables, links, code blocks, images)
3. **Editing Layer:** Selection state, cursor mapping, commands, undo/redo history
4. **Presentation Layer:** Rendered spans/blocks, visual decorations, focus/typewriter metadata

The UI renders state; it never becomes the source of truth.

---

## 6. MVP Scope

### 6.1 In Scope for MVP

**Desktop Application:**
- macOS, Windows, Linux support via Tauri V1

**File Operations:**
- New, open, edit, save `.md` files
- Open folder as workspace
- Recent files and recent folders
- Auto-save with configurable debounce
- Crash recovery
- External file change detection

**Core Editing:**
- Single-pane live Markdown editing
- Rendered support for headings, emphasis, links, images, lists, task lists, code fences, quotes, horizontal rules, tables
- Smart editing behavior for lists, quotes, and basic structure transitions
- In-document search and replace
- Code fence syntax highlighting
- Relative asset path support

**Workspace:**
- Basic outline / table-of-contents panel
- File tree (create, rename, delete files/folders)

**Display:**
- Theme support: light and dark minimum
- Focus mode
- Typewriter mode
- Editor font size and content width settings

**Export:**
- HTML export (standalone or linked-assets mode)
- PDF export (configurable page size and margins)

### 6.2 Explicitly Out of Scope for MVP

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph
- Workspace complexity comparable to IDEs

### 6.3 Stretch Goals (Post-MVP)

- Frontmatter helper UI
- Local document history
- Math rendering
- Custom CSS support
- Improved paste-from-rich-text conversion

---

## 7. Engineering Standards

### 7.1 Rust Standards
- `cargo fmt` enforced in CI
- `clippy` required in CI
- Avoid unnecessary `unsafe`
- Any `unsafe` must be isolated and justified with documentation
- `cargo test` must pass on all PRs

### 7.2 Code Organization
- Business logic must not live in UI event handlers
- Public APIs must be documented
- Non-trivial features require tests and short design rationale
- Small, reviewable pull requests preferred

### 7.3 PR Standards
- Issue or rationale linked
- UI changes include screenshots/gifs when practical
- User-visible behavior changes update docs or fixtures

### 7.4 Architecture Decisions
- Key architectural decisions require ADRs (Architecture Decision Records)
- ADR Required Topics:
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

## 8. Testing Strategy

### 8.1 Unit Tests
Cover:
- Parser behavior
- Cursor movement
- Selection logic
- Editing commands
- Undo/redo
- Serializer
- Settings and recovery

### 8.2 Editing Invariant Regression Tests
Maintain fixture-based tests for:
- Enter on list item
- Enter on empty list item
- Backspace at heading/list/quote boundaries
- Link text edits
- Cursor motion across inline code and emphasis
- Toggling task lists
- Table edit edge cases

### 8.3 Fixture Tests
Use real Markdown fixtures for:
- Parse expectations
- Roundtrip save behavior
- Export regressions
- Tricky documents from real-world authoring

### 8.4 Snapshot Tests
Useful for:
- HTML export
- Semantic tree output
- Theme token outputs

### 8.5 Integration Tests
Test:
- Open/edit/save flows
- Open folder and navigate files
- External file changes
- Recovery flows
- Export command flows

### 8.6 Performance Benchmarks
Track:
- Startup time (target: < 2 seconds cold start)
- Parse time
- Edit latency
- Serialization time
- Export time

---

## 9. Non-Functional Requirements

### 9.1 Performance
- **NFR-001:** Startup target under 2 seconds on mainstream modern laptop
- **NFR-002:** No visible lag during ordinary editing on typical documents under 1 MB
- **NFR-003:** At least 5 MB documents remain usable with graceful degradation

### 9.2 Reliability
- **NFR-004:** Stable releases target crash-free session rate above 99%
- **NFR-005:** Partial writes avoided through temp-file replacement strategy

### 9.3 Privacy and Security
- **NFR-006:** No network dependency in MVP editing path
- **NFR-007:** Raw HTML handling policy must be explicit; export sanitization and trust model documented

### 9.4 Accessibility
- **NFR-008:** Essential actions reachable by keyboard
- **NFR-009:** Default themes maintain good contrast and comfortable typography

---

## 10. Release Engineering

### 10.1 Release Channels
- Nightly
- Beta
- Stable

### 10.2 CI/CD Requirements
- Build matrix for macOS, Windows, Linux
- Lint + tests required on PRs
- Release notes generated consistently
- Checksums published with releases

### 10.3 Versioning
- Semver for core crates where practical
- Semantic app releases preferred

### 10.4 Packaging
- Native installers or archives for each OS
- Clear install docs in README/releases

---

## 11. Documentation Requirements

The project must maintain:
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

## 12. Governance

### 12.1 Initial Structure
- 1–3 maintainers with merge rights
- GitHub Issues for bugs/features
- GitHub Discussions for design topics and Q&A
- ADRs for key architectural changes
- Milestone-driven roadmap visible to contributors

### 12.2 Issue Labels
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

## 13. Risk Mitigation

### 13.1 The Hard Problem is Interaction Semantics
**Risk:** The biggest risk is not parsing Markdown. It is making live rendering, cursor movement, selection, and structural editing feel natural.

**Mitigation:**
- Invest early in editor-engine design
- Maintain editing-invariant regression suites
- Treat cursor bugs as product-critical defects

### 13.2 Table Editing Can Consume the Roadmap
**Mitigation:** Start with constrained, safe table interactions. Optimize for correctness before spreadsheet-like richness.

### 13.3 UI-Layer Logic Creep Can Weaken Architecture
**Mitigation:** Keep correctness in Rust services. Front-end should dispatch commands and render derived state.

### 13.4 Export Disappointment Can Hurt Adoption
**Mitigation:** Prioritize HTML/PDF fidelity. Test representative docs. Document limits clearly.

---

## 14. Definition of Done

A feature is done only when:
1. Implementation is merged
2. Tests added or updated
3. Supported behavior documented if relevant
4. No known major data-loss risk remains
5. Acceptance criteria are met
6. CI passes

---

## 15. MVP Release Criteria

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

## 16. Amendment Process

This constitution may be amended through:
1. Proposal with documented rationale
2. Community discussion period (minimum 7 days)
3. Maintainer consensus or supermajority (2/3)
4. Documented in commit history with version bump

---

**Ratified:** Iteration 4
**Last Updated:** Based on PRD Version 2.1
