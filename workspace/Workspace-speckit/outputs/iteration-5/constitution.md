# RustNote Project Constitution

**Version:** 1.0.0
**Project:** RustNote
**Type:** Open-source Typora-like Markdown Editor
**Ratified:** 2026-04-11
**Last Amended:** 2026-04-11

---

## Preamble

RustNote is an open-source, local-first, writing-first Markdown editor built in Rust. This constitution establishes the principles, non-negotiable rules, and governance framework that define the project's identity and guide all decisions.

---

## 1. Product Identity

### 1.1 Project Name

**RustNote** — A writing-first, WYSIWYM Markdown editor.

### 1.2 Core Thesis

RustNote eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### 1.3 Strategic Positioning

RustNote sits between:
- Developer editors that are too complex for writing
- Note apps that hide or weaken file portability
- Rich text tools that do not preserve Markdown cleanly

It should feel calmer than VS Code, more open than Typora, more file-native than note databases, and more polished than most open-source Markdown editors.

---

## 2. Core Principles

### Principle 1: Write First

**Rule:** The user must feel like they are writing a document, not managing syntax.

**Rationale:** The primary value proposition is uninterrupted writing flow. Any design decision that prioritizes syntax visibility over writing flow contradicts this principle.

**Non-Negotiable Rules:**
- Single-pane live rendering is the default and only primary editing mode
- Supported Markdown syntax must render visually without requiring mode switches
- Raw syntax must not dominate the writing surface

---

### Principle 2: Markdown as Source of Truth

**Rule:** Everything saved must remain valid, predictable Markdown.

**Rationale:** File portability and Git-friendliness are foundational trust properties. Users must never lose Markdown fidelity when saving, reopening, or exporting.

**Non-Negotiable Rules:**
- Supported constructs must survive edit-save-reopen cycles reliably
- Serialization must produce valid UTF-8 Markdown text
- Export must preserve document structure and readability
- Any lossy behavior must be documented and tested

---

### Principle 3: Rendered While Editing

**Rule:** Single-pane live rendering is the defining identity of the product.

**Rationale:** Split preview breaks writing flow. Users should never need a mode switch to understand document appearance.

**Non-Negotiable Rules:**
- The main experience is one editing canvas
- Split preview is not the core workflow
- Users must never need mode-switch to understand document appearance

---

### Principle 4: Invisible UI

**Rule:** Keep chrome, controls, and mode switches minimal.

**Rationale:** Visible complexity creates cognitive overhead that disrupts writing flow.

**Non-Negotiable Rules:**
- Default UI must remain visually quiet
- Toolbar, controls, and settings must not dominate the writing surface
- Focus mode and typewriter mode are product identity, not decoration

---

### Principle 5: Effortless High-Frequency Actions

**Rule:** Lists, headings, links, images, tables, code blocks, paste, and export must feel natural and low-friction.

**Rationale:** These are product-defining paths. If they feel awkward, the product fails regardless of other features.

**Non-Negotiable Rules:**
- Enter continues list item; Enter on empty list item exits list
- Backspace at structural boundaries degrades lists predictably
- Tab/Shift+Tab indent and outdent nested items correctly
- Images insert with correct relative paths
- Rich text paste converts to Markdown on best-effort basis

---

### Principle 6: Local-First Always

**Rule:** Users own normal files on disk.

**Rationale:** Local-first is not a feature—it's a trust contract. Cloud sync and real-time collaboration are explicitly out of scope for MVP.

**Non-Negotiable Rules:**
- No network dependency in MVP editing path
- File operations (open, save, create, delete) work directly on local filesystem
- Workspace is a folder on disk, not a database

---

### Principle 7: Rust Owns Correctness

**Rule:** Parsing, editing semantics, serialization, recovery, and export-critical logic belong in Rust-owned boundaries.

**Rationale:** A Typora-like editor fails when too much logic leaks into ad hoc UI behavior. The architecture must preserve a clear center of correctness.

**Non-Negotiable Rules:**
- Markdown fidelity is owned by Rust interfaces
- Cursor/selection semantics are owned by Rust interfaces
- Structural editing behavior is owned by Rust interfaces
- Save/recovery correctness is owned by Rust interfaces
- Export correctness is owned by Rust interfaces
- UI dispatches commands and renders derived state but is not the source of truth

---

## 3. Experience Invariants

These non-negotiable invariants define whether the product actually feels Typora-like.

### 3.1 Single-Pane Invariant

- The main experience is one editing canvas
- Split preview is not the core workflow
- Users should never need mode-switch to understand document appearance

### 3.2 Readability Invariant

- Headings look like headings while editing
- Lists look like lists
- Links and images appear as document content, not raw syntax
- Document remains pleasant to read during editing

### 3.3 Cursor Invariant

- Cursor movement must be predictable around inline formatting, links, code spans, tables, images, and quotes
- No cursor traps
- Selection boundaries must feel natural

### 3.4 Structure Invariant

- Enter, Backspace, Tab, and Shift+Tab behave consistently for lists, quotes, and nested structures
- Structural editing must not cause surprising Markdown corruption

### 3.5 Fidelity Invariant

- Editing and rendering must not silently destroy supported Markdown constructs
- When a construct cannot be preserved perfectly, behavior must be documented and tested

### 3.6 Calmness Invariant

- Default UI remains visually quiet
- Toolbar, controls, and settings do not dominate the writing surface
- Focus mode and typewriter mode materially reduce visual distraction

### 3.7 Local-Trust Invariant

- App behaves like a trustworthy steward of local files
- Save, autosave, recovery, reload, and external-change handling are dependable

---

## 4. MVP Scope

### 4.1 In Scope

**Desktop App:**
- macOS, Windows, Linux support
- Open, edit, save `.md` files
- Open folder as workspace

**Core Editing:**
- Single-pane live Markdown editing
- Rendered support for: headings, emphasis, links, images, lists, task lists, code fences, quotes, horizontal rules, tables
- Smart editing behavior for lists, quotes, and basic structure transitions
- In-document search and replace

**Workspace:**
- Recent files and recent folders
- File tree sidebar
- Basic outline / table-of-contents panel

**Display:**
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

### 4.2 Out of Scope for MVP

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph
- Workspace complexity comparable to IDEs

### 4.3 Stretch Goals

- Frontmatter helper UI
- Local document history
- Math rendering
- Custom CSS support
- Improved paste-from-rich-text conversion

---

## 5. Architecture Principles

### 5.1 High-Level Approach

- **Rust core** for parsing, editing semantics, serialization, workspace logic, recovery, export orchestration
- **Tauri desktop shell** for packaging, windows, menus, file dialogs, OS integration
- **UI layer** for rendering and interaction, but not as owner of document correctness

### 5.2 Crate Responsibilities

| Crate | Responsibility |
|-------|----------------|
| `core-model` | Semantic document structures, positions, ranges, identifiers, shared core types and errors |
| `markdown-parser` | Parse Markdown to semantic/intermediate representation |
| `editor-engine` | Cursor movement, selection logic, insert/delete/edit commands, smart behaviors, undo/redo |
| `serializer` | Semantic model to Markdown output |
| `workspace` | File IO, recent files/folders, file watching, asset path resolution |
| `export` | HTML export, PDF export, export option types and pipelines |
| `theme` | Theme tokens, typography defaults |
| `settings` | Persisted configuration, schema versioning |
| `recovery` | Autosave snapshots, crash recovery metadata, stale cleanup |
| `app-services` | Orchestration layer between UI shell and Rust core modules |

### 5.3 Public Service Boundaries

Narrow service interfaces over exposing many unstable internal types:

- `DocumentService` — open/save/reload document
- `EditorService` — apply editor command, query selection or outline
- `WorkspaceService` — workspace operations
- `ExportService` — export document
- `SettingsService` — configuration
- `RecoveryService` — restore snapshot

---

## 6. Engineering Standards

### 6.1 Rust Standards

- `cargo fmt` enforced in CI
- `clippy` required in CI
- Avoid unnecessary `unsafe`; any `unsafe` must be isolated and justified

### 6.2 Code Organization

- Business logic must not live in UI event handlers
- Public APIs documented
- Non-trivial features require tests and short design rationale
- Small PRs preferred with issue or rationale linked

### 6.3 Testing Requirements

**Unit Tests:** Parser behavior, cursor movement, selection logic, editing commands, undo/redo, serializer, settings and recovery.

**Editing Invariant Regression Tests:** Maintain fixture-based tests for Enter on list item, Enter on empty list item, Backspace at heading/list/quote boundaries, link text edits, cursor motion across inline code and emphasis, toggling task lists, table edit edge cases.

**Fixture Tests:** Real Markdown fixtures for parse expectations, roundtrip save behavior, export regressions.

**Snapshot Tests:** HTML export, semantic tree output, theme token outputs.

**Integration Tests:** Open/edit/save flows, open folder and navigate files, external file changes, recovery flows, export command flows.

**Performance Benchmarks:** Startup time, parse time, edit latency, serialization time, export time.

---

## 7. Quality Standards

### 7.1 Performance Targets

- Cold start under 2 seconds on mainstream modern laptop
- No visible lag during ordinary editing on typical documents under 1 MB
- At least 5 MB documents remain usable with graceful degradation

### 7.2 Reliability Targets

- Stable releases target crash-free session rate above 99%
- Partial writes avoided through temp-file replacement strategy

### 7.3 Accessibility

- Essential actions reachable by keyboard
- Default themes maintain good contrast and comfortable typography

---

## 8. Documentation Requirements

The project must maintain:
- `README.md`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- Architecture overview
- Supported Markdown behavior documentation
- Known limitations documentation
- Testing strategy documentation
- Roadmap or milestone documentation

---

## 9. Governance

### 9.1 Project Structure

- 1–3 maintainers with merge rights
- GitHub Issues for bugs/features
- GitHub Discussions for design topics and Q&A
- ADRs for key architectural changes
- Milestone-driven roadmap visible to contributors

### 9.2 Issue Labels

`good first issue`, `help wanted`, `bug`, `regression`, `performance`, `ux`, `parser`, `editor-engine`, `workspace`, `export`, `theme`, `testing`, `documentation`, `security`, `breaking change`

### 9.3 Recommended ADR Topics

1. Why Tauri for V1
2. Exact supported Markdown flavor
3. Internal document model choice
4. Source fidelity and serialization guarantees
5. Table editing scope in MVP
6. Raw HTML handling policy
7. PDF export engine choice
8. Future extension/plugin boundary
9. Focus mode and typewriter implementation strategy

### 9.4 Release Channels

- nightly
- beta
- stable

### 9.5 Versioning

- Semver for core crates where practical
- Semantic app releases preferred

---

## 10. Success Criteria

### 10.1 User Success Metrics

RustNote succeeds when users say:
- "I forget I'm editing Markdown."
- "It feels smoother than editing raw `.md` in a code editor."
- "It does not break my files."
- "Images, tables, lists, and export work without friction."
- "It is calm enough for long-form writing."

### 10.2 Contributor Success Metrics

- Contributors can understand the repo quickly
- Editing bugs are reproducible and regression-tested
- Releases are stable across macOS, Windows, Linux

---

## 11. Definition of Done

A feature is done only when:
- Implementation is merged
- Tests added or updated
- Supported behavior documented if relevant
- No known major data-loss risk remains
- Acceptance criteria are met
- CI passes

---

## 12. Amendment Procedure

### 12.1 Version Bump Rules

- **MAJOR:** Backward incompatible governance/principle removals or redefinitions
- **MINOR:** New principle/section added or materially expanded guidance
- **PATCH:** Clarifications, wording, typo fixes, non-semantic refinements

### 12.2 Amendment Process

1. Propose change via GitHub Discussion or ADR
2. Maintainer review and consensus
3. Version bump decision documented
4. Constitution update merged to main
5. Dependent artifacts updated (templates, specs, tasks)

### 12.3 Compliance Review

Before any PR merging that affects:
- Core editing invariants → require editing regression tests
- Serialization fidelity → require roundtrip fixture tests
- Architecture boundaries → require ADR documentation
- Public API surface → require API documentation update

---

*This constitution was derived from the RustNote implementation-ready PRD version 2.1, ratified 2026-04-11.*
