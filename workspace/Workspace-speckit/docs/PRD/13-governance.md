# Governance

## 31. Release Engineering

### 22.1 Channels
* nightly
* beta
* stable

### 22.2 CI/CD
* build matrix for macOS, Windows, Linux
* lint + tests required on PRs
* release notes generated consistently
* checksums published with releases

### 22.3 Versioning
* semver for core crates where practical
* semantic app releases preferred

### 22.4 Packaging
* native installers or archives for each OS
* clear install docs in README/releases

---

## 32. Documentation Requirements

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

## 33. Governance Model

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

## 34. Risks and Trade-offs

### 26.1 The hard problem is interaction semantics

The biggest risk is not parsing Markdown. It is making live rendering, cursor movement, selection, and structural editing feel natural.

Mitigation:
* invest early in editor-engine design
* maintain editing-invariant regression suites
* treat cursor bugs as product-critical defects

### 26.2 Table editing can consume the roadmap

Mitigation:
* start with constrained, safe table interactions
* optimize for correctness before spreadsheet-like richness

### 26.3 UI-layer logic creep can weaken architecture

Mitigation:
* keep correctness in Rust services
* front-end should dispatch commands and render derived state

### 26.4 Export disappointment can hurt adoption

Mitigation:
* prioritize HTML/PDF fidelity
* test representative docs
* document limits clearly

---

## 35. MVP Release Criteria

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

## 36. Recommended ADR Topics

The project should write ADRs for:

1. Why Tauri v2 for desktop shell
2. Why Tiptap/ProseMirror over raw contenteditable
3. prosemirror-markdown vs custom Markdown serializer
4. ropey vs native JS string for text buffer
5. tree-sitter incremental parsing strategy
6. comrak vs pulldown-cmark for Rust Markdown parsing
7. Shiki vs syntect for code highlighting
8. rusqlite vs Tauri Store for persistence
9. Exact supported Markdown flavor (CommonMark + GFM)
10. Source fidelity and serialization guarantees
11. Table editing scope in MVP
12. Raw HTML handling policy
13. PDF export engine choice
14. Focus mode and typewriter implementation strategy
