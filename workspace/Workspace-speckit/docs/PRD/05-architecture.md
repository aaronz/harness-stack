# Architecture

## 11. Non-Functional Requirements

### 11.1 Performance

#### NFR-001 Startup
* target cold start under 2 seconds on mainstream modern laptop
* measured from app icon click to ready-to-type state

#### NFR-002 Typing responsiveness
* no visible lag during ordinary editing on typical documents under 1 MB
* edit-to-render latency < 100ms for keystrokes
* render debounced at 50ms to batch rapid inputs

#### NFR-003 Large document usability
* at least 5 MB documents remain usable, with graceful degradation if necessary
* documents > 10 MB may show degraded performance but remain safe to edit

#### NFR-013 Performance Budgets

| Operation | Budget | Notes |
|-----------|--------|-------|
| Cold start (empty) | < 2s | Modern laptop, SSD |
| Cold start (1MB doc) | < 3s | Including initial render |
| Hot file open | < 500ms | After first file opened |
| Keystroke → render | < 100ms | End-to-end |
| Save (Ctrl+S) | < 200ms | File write complete |
| HTML export (10KB) | < 1s | Including syntax highlight |
| PDF export (10 pages) | < 5s | Print-quality |
| Outline update | < 100ms | After edit stops |
| Theme switch | < 200ms | Full dark/light toggle |
| Memory (10 docs idle) | < 300MB | Resident memory |
| Scroll (5MB doc) | 60 FPS | No jank |

#### NFR-014 Scalability Targets

| Document Size | Behavior |
|--------------|----------|
| < 100KB | Full feature set, no degradation |
| 100KB - 1MB | Full feature set, live render may debounce more |
| 1MB - 5MB | All features work, occasional render pauses < 200ms |
| 5MB - 10MB | Core editing works, disable live preview toggle |
| > 10MB | Warning shown, continue with degraded mode |

---

### 11.2 Reliability

#### NFR-004 Crash-free goal
* stable releases target crash-free session rate above 99%

#### NFR-005 Safe writes
* partial writes avoided through temp-file replacement strategy where practical

---

### 11.3 Privacy and Security

#### NFR-006 Local-first
* no network dependency in MVP editing path
* all document data stays on user's local filesystem
* no telemetry or analytics in MVP

#### NFR-007 Safe rendering
* raw HTML handling policy must be explicit
* export sanitization and trust model documented

#### NFR-010 Threat Model

**Assets to protect:**
- User documents (read/write access)
- User workspace filesystem (path traversal prevention)
- Export outputs (HTML injection prevention)

**Threats and mitigations:**

| Threat | Mitigation |
|--------|-----------|
| Path traversal via `../` in file paths | Validate and normalize all paths in Rust backend before file operations |
| Malicious Markdown HTML injection | Sanitize raw HTML on render; strip `<script>` tags |
| Malicious image paths | Validate image paths exist within workspace boundary |
| Export file overwrite | Confirm before overwriting existing files |
| Memory corruption from malformed docs | Fuzz test parser with malformed inputs; isolate parsing in separate error context |

#### NFR-011 HTML Sanitization Policy

**Allowed HTML in rendered output:**
- Semantic HTML: `<p>`, `<h1>`-`<h6>`, `<strong>`, `<em>`, `<ul>`, `<ol>`, `<li>`, `<blockquote>`, `<pre>`, `<code>`, `<a>`, `<img>`
- Tables: `<table>`, `<thead>`, `<tbody>`, `<tr>`, `<th>`, `<td>`
- Task lists: `<input type="checkbox">` (checked/disabled only)

**Forbidden (stripped on render):**
- `<script>`, `<iframe>`, `<object>`, `<embed>`
- `onclick`, `onerror`, `onload` event handlers
- `javascript:` URLs
- `<style>` injection

#### NFR-012 Export Security
* HTML exports are self-contained (no external resource dependencies)
* PDF exports use sandboxed PDF generation
* No active content in exports (no JavaScript)

---

### 11.4 Accessibility

#### NFR-008 Keyboard-first
* essential actions reachable by keyboard

#### NFR-009 Readability
* default themes must maintain good contrast and comfortable typography

---

## 12. Architecture Overview

### 12.1 High-Level Approach

#### Recommended Production Architecture
* **Tauri v2 desktop shell** for packaging, windows, menus, file dialogs, OS integration
* **Tiptap/ProseMirror** for editor surface with clean document semantics
* **prosemirror-markdown** for Markdown serialization bridge
* **Rust core** for parsing (Comrak, tree-sitter), text buffer (ropey), workspace logic, recovery, export
* **`printpdf`** for PDF export
* **rusqlite** for local persistence (recent files, settings, history)
* **Shiki** for VS Code-quality syntax highlighting in frontend; **syntect** for Rust-side export

#### Current MVP Architecture
* **Tauri v2 shell** for packaging and OS integration
* **React 18** with contenteditable for MVP editor
* **Rust services** (pulldown-cmark, custom rendering) for Markdown handling
* **Tauri plugins** for dialog, filesystem, settings

#### Architectural Rule
Anything affecting Markdown fidelity, cursor/selection semantics, structural editing behavior, save/recovery correctness, or export correctness must live in Rust-owned interfaces or Rust-controlled services.

### 12.2 Why This Matters
A Typora-like editor fails when too much logic leaks into ad hoc UI behavior. The architecture must preserve a clear center of correctness.

### 12.3 Recommended Production Architecture Blueprint

#### Separation of Concerns
The recommended module split:
* `src-tauri/` - file I/O, settings, export, search/index, recent files/history, native menus/dialogs/updater
* `editor-web/` - Tiptap/ProseMirror editor, markdown serializer/parser bridge, theme system, image/table/code-block widgets, math/diagram rendering
* `shared contract` - document schema, command/event types, settings schema

#### Data Flow
```
User Input → Tiptap (ProseMirror) → prosemirror-markdown → Rust (comrak/ropey)
                                       ↓
Frontend Render ← HTML/CSS ← prosemirror-markdown ← Rust services
```

#### Key Integration Points
1. **Frontend → Rust**: Tauri IPC commands for file operations, export, settings
2. **Rust → Frontend**: Events for file watching, external changes
3. **Markdown ↔ ProseMirror**: `prosemirror-markdown` for serialization
4. **Text Buffer ↔ Parse Tree**: Ropey for content, tree-sitter for incremental parse

#### Why Tiptap/ProseMirror Over contenteditable
ProseMirror is purpose-built for **structured rich-text editing** with:
- Clean document semantics (not HTML DOM as source of truth)
- Transaction-based undo/redo
- Structured selection and cursor movement
- Extension system for custom nodes/marks
- Better handling of IME, tables, and complex content

#### Why ropey for Text Buffer
Ropey uses a **B-tree rope** structure for:
- Efficient editing of very large texts
- Good locality for line-based operations
- Memory efficiency (minimizes pointer chasing)

#### Why tree-sitter + comrak
* **tree-sitter**: Incremental parsing - re-parse only changed sections
* **comrak**: Full Markdown AST parsing with GFM support

#### Why Shiki + syntect
* **Shiki**: TextMate grammars/themes, `codeToHtml` API, JS/Node ecosystem
* **syntect**: Sublime Text definitions, `ClassedHTMLGenerator`, Rust-native
* Decision: JS workflow → Shiki, Rust toolchain → syntect
