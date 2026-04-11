# Frontend Design

## 13. Frontend Design Goals

The frontend is not a thin decorative layer. It is the visible embodiment of the product promise.

A Typora-like product succeeds or fails in the frontend because users directly experience:

* readability while editing
* the amount of visual distraction
* cursor and selection predictability
* interaction smoothness around structured Markdown content
* speed of common authoring tasks

Therefore the frontend design must optimize for three things simultaneously:

1. **calmness** — the interface should visually disappear during writing
2. **clarity** — rendered content, controls, states, and transitions should be understandable
3. **control** — advanced authoring actions should remain discoverable and precise

The frontend must never feel like a generic admin panel, IDE clone, or document app overloaded with chrome.

---

## 14. Frontend Product Principles

### 14.1 Writing Surface First

The central editor canvas is the product. Sidebars, toolbars, dialogs, menus, and status surfaces exist to support it, not compete with it.

### 14.2 Chrome Minimization

Anything persistent on screen must justify its presence. Default UI should be sparse. Optional surfaces may be shown contextually or by explicit user action.

### 14.3 Readable While Editable

The frontend must preserve the feeling that the user is reading a real document while still allowing precise editing of Markdown-backed structures.

### 14.4 Progressive Disclosure

Simple users should see very little UI. Power users should still be able to reach commands, shortcuts, file operations, and configuration without clutter.

### 14.5 Interaction Consistency

Selections, hover states, click targets, keyboard actions, and structural editing interactions must behave consistently across content types.

### 14.6 Platform Familiarity

The product should feel native enough on macOS, Windows, and Linux in menu behavior, shortcut display, window handling, and file dialogs.

---

## 15. Frontend Architecture

### 15.1 Recommended Stack for Typora-Like Editing

#### Production Stack
* **Desktop runtime:** Tauri v2
* **Frontend:** React 18 or Svelte
* **Rich editor core:** Tiptap / ProseMirror
* **Markdown bridge:** `prosemirror-markdown`
* **Rust core services:** `comrak`, `ropey`, `tokio`, `tracing`, `rusqlite`, `printpdf`
* **Optional source mode:** CodeMirror 6
* **Code highlighting:** Shiki for frontend preview, `syntect` for Rust-side export/render pipelines
* **Math/diagram plugins:** KaTeX + Mermaid

#### Current Implementation (MVP Phase)
* **Tauri v2 shell** for packaging and OS integration
* **React 18** with contenteditable for MVP editor
* **Vite** for fast development and optimized builds
* **Tailwind CSS** for utility-first styling with design token integration
* **React Context** for state management (DocumentContext, SettingsContext)
* **Rust services** for parsing, editing semantics, serialization, workspace logic, recovery, export
* **`pulldown-cmark`** for Markdown parsing

#### Future Migration Path

1. **Replace React contenteditable with Tiptap/ProseMirror**
   - Install `@tiptap/core`, `@tiptap/starter-kit`, `@tiptap/markdown`
   - Configure extensions: table, task-list, code-block
   - Wire `editor.getMarkdown()` / `editor.markdown.parse()` to Rust backend

2. **Add CodeMirror 6 for source mode**
   - Add `@tiptap/extension-code-block` with syntax highlighting
   - Implement toggle between rich-text and source mode views

3. **Integrate prosemirror-markdown bridge**
   - `MarkdownParser` converts Markdown → ProseMirror doc
   - `MarkdownSerializer` converts ProseMirror doc → Markdown

4. **Migrate text buffer to ropey + tree-sitter**
   - ropey for efficient in-memory text storage
   - tree-sitter for incremental re-parsing on edits

5. **Add rusqlite for local persistence**
   - Recent files, workspace state, snippets, backlinks

6. **Add Shiki for frontend syntax highlighting**
   - Use `codeToHtml` for static HTML generation

#### Stack Comparison

| Aspect | MVP Stack | Production Stack |
|--------|----------|------------------|
| Editor core | React + contenteditable | Tiptap/ProseMirror |
| Markdown parsing | pulldown-cmark | prosemirror-markdown + comrak |
| Text buffer | JS string | ropey (Rust) |
| Syntax highlighting | basic CSS | Shiki + syntect |
| Local persistence | Tauri settings | rusqlite |
| Async runtime | Tauri commands | tokio |
