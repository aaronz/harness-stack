# Rust Crate Design

## 24. Public Service Boundaries

Prefer narrow service interfaces over exposing many unstable internal types.

Suggested boundaries:
* `DocumentService`
* `EditorService`
* `WorkspaceService`
* `ExportService`
* `SettingsService`
* `RecoveryService`

Suggested operation families:
* open/save/reload document
* apply editor command
* query selection or outline
* insert image/link/table block
* export document
* restore snapshot

---

## 25. Data Model Layers

The implementation keeps these layers separate:

### 33.1 Source Layer
* raw Markdown text

### 33.2 Semantic Layer
* parsed structures: headings, paragraphs, list items, tables, links, code blocks, images

### 33.3 Editing Layer
* selection state
* cursor mapping
* commands
* undo/redo history

### 33.4 Presentation Layer
* rendered spans/blocks
* visual decorations
* focus/typewriter metadata

The UI should render state, not become the source of truth.

---

## 26. Repository Structure

### Recommended Production Structure

```text
rustnote/
  Cargo.toml              # Rust workspace
  README.md
  LICENSE-MIT / LICENSE-APACHE
  CONTRIBUTING.md
  CODE_OF_CONDUCT.md
  SECURITY.md
  ADR/
  docs/
  src-tauri/              # Rust backend services
    Cargo.toml
    src/
      lib.rs              # Main entry, command registration
      commands/          # Tauri command handlers
      model/
      buffer/            # ropey text buffer
      parser/            # tree-sitter + comrak parsing
      services/          # Business logic (DocumentService, etc.)
      export/            # HTML/PDF export with syntect
      workspace/         # File I/O, watching, recent files
      recovery/          # Snapshot management
      settings/          # rusqlite persistence
    tauri.conf.json
    capabilities/
  editor-web/             # Tiptap/ProseMirror frontend
    package.json
    vite.config.ts
    src/
      main.tsx
      App.tsx
      components/
        Editor.tsx       # Tiptap editor instance
        Toolbar.tsx
        Sidebar.tsx
        OutlinePanel.tsx
        SourceEditor.tsx # CodeMirror 6 for source mode
      extensions/        # Tiptap extensions (tables, task lists)
      markdown/
        serializer.ts    # prosemirror-markdown
        parser.ts
      theme/
        tokens.ts       # Design tokens
        light.css
        dark.css
      hooks/
      stores/            # State management
    public/
  shared/                # Shared contract between Rust and frontend
    Cargo.toml
    src/
      schema.rs          # Document schema types
      commands.rs        # Command/event types
      settings.rs        # Settings schema
  fixtures/
  scripts/
```

### Current MVP Structure

```text
rustnote/
  Cargo.toml
  src-tauri/              # Tauri app (Rust services embedded)
    src/
      commands/          # Tauri command handlers
      model/
      parser/            # pulldown-cmark parsing
      renderer/
      semantic/
      services/
  www/                    # React frontend (Vite + React)
    src/
      contexts/          # React Context state
      components/        # UI components
```

---

## 27. Rust Crate Responsibilities

### 31.1 `buffer` (ropey-based)
* text buffer using ropey for efficient large document handling
* position-to-offset mapping
* UTF-8 / UTF-16 cursor handling
* incremental change application

### 31.2 `parser` (tree-sitter + comrak)
* tree-sitter for incremental syntax parsing
* comrak for full Markdown AST parsing (GFM support)
* parse result caching and invalidation
* cursor/selection anchor points from parse

### 31.3 `editor-engine`
* cursor movement rules
* selection logic
* insert/delete/edit commands via transactions
* smart Enter/Backspace/Tab behaviors
* undo/redo (ProseMirror-style transactions)
* editing invariants and regression coverage

### 31.4 `serializer`
* ProseMirror document to Markdown serialization
* preserve supported constructs predictably
* round-trip fidelity validation

### 31.5 `workspace`
* file IO
* recent files/folders (rusqlite persistence)
* file watching (notify crate)
* asset path resolution

### 31.6 `export`
* HTML export with Shiki/syntect highlighting
* PDF export with printpdf
* export option types and pipelines

### 31.7 `theme`
* theme tokens
* typography defaults
* focus/typewriter presentation config

### 31.8 `settings`
* rusqlite-based persisted configuration
* schema versioning
* recent files, workspace state, snippets

### 31.9 `recovery`
* autosave snapshots (tokio async)
* crash recovery metadata
* stale snapshot cleanup

### 31.10 `app-services`
* orchestration layer between Tauri shell and Rust core modules
* Tauri command handlers
* IPC between frontend and Rust services
