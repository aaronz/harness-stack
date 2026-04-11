# Product Definition

## 1. Executive Definition

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

## 2. Why Typora-like Products Win

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

## 3. Product Thesis

### 3.1 Core Thesis

RustNote should be defined as:

> A writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### 3.2 Strategic Positioning

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

## 4. Product Principles

### 4.1 Identity Principles

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

### 4.2 Open-Source Principles

* small, reviewable pull requests
* ADRs for meaningful architectural decisions
* documented supported Markdown behavior
* tests around editing invariants, not just parser output
* low-coupling modules with clear ownership
* explicit out-of-scope choices to prevent product drift
