# RustNote Architecture Documentation

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 1.0
**Last Updated:** 2026-04-14

---

## 1. Overview

RustNote is a local-first, WYSIWYG Markdown editor built with:
- **Frontend:** React + TypeScript + Vite
- **Backend:** Rust with Tauri
- **Editor:** TipTap (ProseMirror-based) for structured document editing
- **Persistence:** rusqlite for settings, filesystem for documents

---

## 2. Frontend Architecture

### 2.1 Editor Architecture

#### Primary Editor: TipTapEditor

**Location:** `www/src/components/TipTapEditor.jsx`

TipTapEditor is the **sole active editor** in the application. It provides:

- **WYSIWYG editing** with TipTap/ProseMirror
- **Live Markdown preview** - edits render in real-time
- **Structured editing** - headings, lists, blockquotes, tables, code blocks
- **Extensions:**
  - StarterKit (headings, lists, bold, italic, etc.)
  - Highlight (search result highlighting)
  - Placeholder (empty document placeholder)
  - TaskList/TaskItem (task list support)
  - Link (hyperlink support)

**Key Features:**
- Focus mode with paragraph tracking via IntersectionObserver
- Typewriter mode with cursor centering
- Search highlighting with match navigation
- Image paste handling via Turndown service
- Frontmatter block display
- scrollToHeading() for outline panel navigation

#### Legacy Editor: Editor.jsx (DEPRECATED)

**Location:** `www/src/components/Editor.jsx`

**Status:** ⚠️ **DEPRECATED** - Do not use

```javascript
/**
 * @deprecated Use TipTapEditor.jsx instead
 */
```

Editor.jsx is a legacy plaintext editor that has been replaced by TipTapEditor.jsx. It is preserved for historical reference only and should not be used in new development.

**Why TipTapEditor replaced Editor.jsx:**
- Proper WYSIWYG editing vs. plaintext with regex decorations
- Built-in undo/redo via ProseMirror history
- Better cursor mapping and text selection
- Full Markdown live preview
- Structured document editing

### 2.2 Component Structure

```
www/src/
├── components/
│   ├── TipTapEditor.jsx    # PRIMARY ACTIVE EDITOR
│   ├── Editor.jsx          # DEPRECATED (do not use)
│   ├── Sidebar.jsx         # File tree and workspace
│   ├── OutlinePanel.jsx    # Table of contents
│   ├── SearchPanel.jsx     # Find/replace
│   ├── Toolbar.jsx         # Top toolbar with actions
│   ├── ExportModal.jsx     # HTML/PDF export
│   ├── PreferencesModal.jsx # Settings panel
│   ├── RecoveryModal.jsx   # Crash recovery UI
│   ├── ExternalChangeModal.jsx # External file change detection
│   ├── DropZone.jsx        # Drag-and-drop file open
│   ├── Toast.jsx           # Notification system
│   ├── CodeBlockHighlight.jsx # Syntax highlighting for code blocks
│   ├── LinkPopover.jsx     # Link editing popover
│   └── FrontmatterBlock.jsx # Frontmatter display
├── contexts/
│   ├── DocumentContext.jsx  # Document state management
│   ├── SettingsContext.jsx # Settings state management
│   ├── SearchContext.jsx   # Search state management
│   └── ToastContext.jsx    # Toast notifications
├── hooks/
│   ├── useFileWatcher.js   # File watching integration
│   └── useAutoSaveTimer.js # Autosave timer management
└── App.jsx                 # Main application (uses TipTapEditor)
```

### 2.3 Data Flow

```
User Input → TipTapEditor → DocumentContext → Rust Backend
                ↓                              ↓
         ProseMirror DOM              Tauri IPC (invoke)
                ↓                              ↓
         Markdown Source              Rust Commands
                ↓                              ↓
         Content Update              File System / SQLite
```

---

## 3. Backend Architecture

### 3.1 Rust Crate Structure

```
src-tauri/src/
├── commands/           # Tauri command handlers
│   ├── mod.rs
│   ├── document.rs    # Document CRUD
│   ├── render.rs      # Markdown rendering
│   ├── settings.rs    # Settings persistence
│   ├── editor.rs      # Editor transforms
│   ├── export.rs      # HTML/PDF export
│   ├── workspace.rs   # Workspace operations
│   ├── file_tree.rs   # File tree operations
│   ├── file_watcher.rs # File change detection
│   ├── image.rs       # Image handling
│   ├── recovery.rs    # Crash recovery
│   └── autosave.rs    # Autosave operations
├── semantic/          # AST parsing and analysis
│   ├── mod.rs
│   ├── position.rs    # Cursor mapping
│   └── markdown.rs    # Markdown AST
├── parser/            # Markdown parsing
│   ├── mod.rs
│   ├── tree_sitter.rs # tree-sitter integration
│   └── syntax.rs      # Syntax highlighting
├── model/             # Data models
│   ├── mod.rs
│   ├── document.rs
│   ├── settings.rs
│   └── export.rs
├── editor/            # Editor core
│   ├── mod.rs
│   ├── transforms.rs  # Transform engine
│   ├── cursor.rs      # Cursor handling
│   ├── selection.rs   # Selection handling
│   ├── search.rs      # Search operations
│   └── undo.rs        # Undo/redo
├── services/          # Service trait definitions
│   └── mod.rs         # All service traits
├── buffer/            # Text buffer (ropey)
│   └── mod.rs
└── renderer/          # HTML rendering
    ├── mod.rs
    ├── state.rs
    ├── blocks.rs
    └── inline.rs
```

### 3.2 Service Interfaces

All services are defined as traits in `services/mod.rs`:

- **DocumentServiceTrait** - Document CRUD operations
- **EditorServiceTrait** - Editor operations (transforms, search)
- **SettingsServiceTrait** - Settings persistence
- **FileWatcherServiceTrait** - File watching
- **AutosaveServiceTrait** - Autosave operations
- **ExportServiceTrait** - Export operations
- **RecoveryServiceTrait** - Crash recovery
- **WorkspaceServiceTrait** - Workspace operations

---

## 4. Key Design Decisions

### 4.1 TipTap as Primary Editor

**Decision:** Use TipTap (ProseMirror-based) as the primary editor component.

**Rationale:**
1. ProseMirror provides structured document editing with proper DOM representation
2. Built-in undo/redo history management
3. Transaction-based editing with proper state management
4. Extension ecosystem for additional features
5. Better cursor mapping and text selection

**Migration from Editor.jsx:**
- Editor.jsx (plaintext with contentEditable) was deprecated
- TipTapEditor provides WYSIWYG editing with live Markdown preview
- All features from Editor.jsx are available in TipTapEditor

### 4.2 Single Editor Strategy

**Decision:** Single primary editor (TipTapEditor) with no fallback.

**Implementation:**
- App.jsx imports TipTapEditor directly
- No Editor.jsx imports exist in the codebase
- Editor.jsx preserved for historical reference only

### 4.3 Markdown as Source of Truth

**Decision:** All documents are stored as plain Markdown on the filesystem.

**Rationale:**
1. Plain text is durable and universally readable
2. Version control friendly
3. No vendor lock-in
4. Users own their data

---

## 5. Editor Feature Comparison

| Feature | Editor.jsx (DEPRECATED) | TipTapEditor (ACTIVE) |
|---------|------------------------|----------------------|
| WYSIWYG | ❌ Plaintext | ✅ Full WYSIWYG |
| Live Preview | ⚠️ Regex-based | ✅ Built-in |
| Undo/Redo | ⚠️ Manual | ✅ ProseMirror history |
| Cursor Mapping | ⚠️ Basic | ✅ Bidirectional |
| Task Lists | ⚠️ Manual | ✅ Native extension |
| Tables | ❌ Not supported | ✅ Native extension |
| Focus Mode | ✅ Supported | ✅ Supported |
| Typewriter Mode | ✅ Supported | ✅ Supported |
| Search Highlighting | ✅ Supported | ✅ Supported |
| Image Paste | ✅ Supported | ✅ Supported |
| Frontmatter | ❌ Not supported | ✅ Supported |

**Conclusion:** TipTapEditor is a functional superset of Editor.jsx.

---

## 6. Deprecation Notice

### Editor.jsx Deprecation (G-009)

**Status:** ✅ COMPLETED

The Editor.jsx component has been officially deprecated in favor of TipTapEditor.jsx.

**Changes Made:**
1. Added `@deprecated Use TipTapEditor.jsx instead` JSDoc tag
2. App.jsx uses TipTapEditor as the primary editor
3. No code imports Editor.jsx
4. SPEC.md updated with editor deprecation notice
5. This architecture document reflects single-editor strategy

**Migration Path:**
- All new development uses TipTapEditor
- Editor.jsx is preserved for reference only
- Editor.jsx will be removed in post-MVP release

---

*Document maintained as part of RustNote architecture documentation*
