# Product Invariants

## 5. Success Criteria

RustNote succeeds when users say:

* "I forget I'm editing Markdown."
* "It feels smoother than editing raw `.md` in a code editor."
* "It does not break my files."
* "Images, tables, lists, and export work without friction."
* "It is calm enough for long-form writing."

For open-source maintainers, success also means:

* contributors can understand the repo quickly
* editing bugs are reproducible and regression-tested
* releases are stable across macOS, Windows, Linux

---

## 6. Users and Jobs to Be Done

### 6.1 Primary Users

#### Writers and note-takers

Need a clean writing space with Markdown portability.

#### Developers and technical authors

Need README/docs/spec editing with Git-friendly output and code-block correctness.

#### Product managers and researchers

Need a readable drafting environment with headings, lists, tables, images, and export.

### 6.2 Core Jobs

* write a document in Markdown without being distracted by syntax
* edit existing Markdown files from local disk
* structure content with headings, lists, tables, quotes, and code blocks
* insert images and links without breaking paths
* export polished HTML or PDF
* work in a folder-based documentation workspace

---

## 7. Non-Negotiable Experience Invariants

This section is the most important refinement to the PRD. These are not "nice to have" UX details. They define whether the product actually feels Typora-like.

### 7.1 Single-Pane Invariant

* the main experience is one editing canvas
* split preview is not the core workflow
* users should never need to mode-switch to understand document appearance

### 7.2 Readability Invariant

* headings should look like headings while editing
* lists should look like lists
* links and images should appear as document content, not primarily as raw syntax
* the document should remain pleasant to read during editing

### 7.3 Cursor Invariant

* cursor movement must be predictable around inline formatting, links, code spans, tables, images, and quotes
* no cursor traps
* selection boundaries must feel natural

### 7.4 Structure Invariant

* Enter, Backspace, Tab, and Shift+Tab must behave consistently for lists, quotes, and nested structures
* structural editing must not cause surprising Markdown corruption

### 7.5 Fidelity Invariant

* editing and rendering must not silently destroy supported Markdown constructs
* when a construct cannot be preserved perfectly, behavior must be documented and tested

### 7.6 Calmness Invariant

* default UI should remain visually quiet
* toolbar, controls, and settings must not dominate the writing surface
* focus mode and typewriter mode are part of product identity, not decoration

### 7.7 Local-Trust Invariant

* the app should behave like a trustworthy steward of local files
* save, autosave, recovery, reload, and external-change handling must be dependable
