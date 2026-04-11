# RustNote Implementation Plan — Iteration 2

**Document Version:** 1.0  
**Date:** 2026-04-11  
**Based On:** Iteration-2 Gap Analysis + Constitution v2.0  
**Target:** Milestone 2 (Live Rendering) + Milestone 3 (Editing Semantics)

---

## 1. Technical Context

### 1.1 Project Overview

| Field | Value |
|-------|-------|
| Project | RustNote — Typora-like Markdown Editor |
| Runtime | Tauri V1 (Desktop: macOS, Windows, Linux) |
| Language | Rust (core) + TypeScript/HTML/CSS (frontend) |
| Markdown Flavor | CommonMark + GFM |
| License | MIT OR Apache-2.0 |
| Workspace | `/Users/aaronzh/Documents/GitHub/harness-stack/workspace/workspace-speckit/rustnote/` |

### 1.2 Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend (www/)                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ EditorView  │  │ SidebarView │  │ OutlineView / Panels   │  │
│  │ (ProseMirror)│  │ (Workspace) │  │ (TOC, Find/Replace)    │  │
│  └──────┬──────┘  └──────┬──────┘  └───────────┬─────────────┘  │
│         │                │                     │                │
│  ┌──────▼──────────────────────────────────────▼─────────────┐  │
│  │              ProseMirror + prosemirror-markdown           │  │
│  │         (WYSIWYG editing + Markdown ↔ Doc bridge)        │  │
│  └──────────────────────────┬───────────────────────────────┘  │
└─────────────────────────────┼───────────────────────────────────┘
                              │ Tauri IPC (invoke/events)
┌─────────────────────────────▼───────────────────────────────────┐
│                         Rust Backend (src-tauri/)              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Document    │  │ Parser      │  │ Editor Engine           │  │
│  │ Service     │  │ (comrak)    │  │ (Cursor, Selection)     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Serializer  │  │ Workspace   │  │ Export Service          │  │
│  │             │  │ (file IO)   │  │ (HTML/PDF)              │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

**Key Architecture Decisions (Research-Backed):**
- **Frontend:** ProseMirror with prosemirror-markdown for live WYSIWYG editing
- **Backend Parser:** comrak for AST-based parsing (replaces pulldown-cmark)
- **Code Highlighting:** syntect (already integrated)
- **IPC Pattern:** invoke for commands, events for push notifications
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend (www/)                        │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ EditorView  │  │ SidebarView │  │ OutlineView / Panels   │  │
│  │ (Live MD)   │  │ (Workspace) │  │ (TOC, Find/Replace)    │  │
│  └──────┬──────┘  └──────┬──────┘  └───────────┬─────────────┘  │
│         │                │                     │                │
│  ┌──────▼──────────────────────────────────────▼─────────────┐  │
│  │                    Renderer / Editor Engine              │  │
│  │  (Transforms Markdown AST → Rendered Spans/Blocks)        │  │
│  └──────────────────────────┬───────────────────────────────┘  │
└─────────────────────────────┼───────────────────────────────────┘
                              │ Tauri IPC (invoke/events)
┌─────────────────────────────▼───────────────────────────────────┐
│                         Rust Backend (src-tauri/)              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Document    │  │ Parser      │  │ Editor Engine           │  │
│  │ Service     │  │ (pulldown)  │  │ (Cursor, Selection)     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Serializer  │  │ Workspace   │  │ Export Service          │  │
│  │             │  │ (file IO)   │  │ (HTML/PDF)              │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.3 Module Organization

```
rustnote/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs              # Entry point, panic handler
│       ├── lib.rs               # Command registration
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── document.rs      # create, open, save, read, write
│       │   ├── workspace.rs     # list_workspace, file watching
│       │   ├── settings.rs      # read/write settings
│       │   └── export.rs        # export_to_html, export_to_pdf
│       ├── model/
│       │   ├── mod.rs
│       │   ├── document.rs      # Document, DocumentId
│       │   ├── settings.rs      # Theme, EditorSettings
│       │   └── workspace.rs    # Workspace, FileEntry
│       ├── parser/
│       │   ├── mod.rs
│       │   ├── markdown.rs      # MarkdownParser (pulldown-cmark)
│       │   └── syntax.rs        # SyntaxHighlighter (syntect)
│       ├── semantic/            # ⭐ NEW: Semantic AST layer
│       │   ├── mod.rs
│       │   ├── ast.rs           # Markdown AST nodes
│       │   ├── position.rs      # Position, Range, Offset mapping
│       │   └── transform.rs     # Source ↔ AST synchronization
│       ├── editor/              # ⭐ NEW: Editor engine
│       │   ├── mod.rs
│       │   ├── cursor.rs         # Cursor movement rules
│       │   ├── selection.rs      # Selection handling
│       │   ├── commands.rs       # Insert, delete, format commands
│       │   ├── transforms.rs    # Smart Enter/Backspace/Tab
│       │   └── undo.rs          # Undo/redo history
│       ├── renderer/            # ⭐ NEW: Live rendering
│       │   ├── mod.rs
│       │   ├── inline.rs        # Inline span rendering
│       │   ├── blocks.rs        # Block rendering
│       │   └── state.rs         # Render state management
│       ├── recovery/            # ⭐ NEW: Crash recovery
│       │   ├── mod.rs
│       │   ├── snapshots.rs     # Auto-save snapshots
│       │   └── restore.rs       # Recovery logic
│       ├── services/            # ⭐ RESTRUCTURE: Service layer
│       │   ├── mod.rs
│       │   ├── document.rs       # DocumentService
│       │   ├── editor.rs         # EditorService
│       │   ├── workspace.rs      # WorkspaceService
│       │   ├── export.rs         # ExportService
│       │   ├── settings.rs       # SettingsService
│       │   └── recovery.rs       # RecoveryService
│       └── watcher/             # ⭐ NEW: File watching
│           ├── mod.rs
│           └── notify.rs        # notify-based file watcher
└── www/
    ├── index.html
    └── src/
        ├── scripts/
        │   ├── app.js           # App lifecycle, shortcuts
        │   ├── editor.js        # ⭐ REFACTOR: Live editor
        │   ├── renderer.js      # ⭐ NEW: Markdown renderer
        │   ├── search.js        # Find/replace
        │   ├── sidebar.js       # Workspace tree
        │   ├── outline.js       # TOC panel
        │   └── settings.js      # Settings UI
        └── styles/
            ├── main.css
            ├── theme-light.css
            ├── theme-dark.css
            └── editor.css        # ⭐ NEW: Editor styles
```

### 1.4 Technical Decisions (RESOLVED)

| Decision | Choice | Rationale | Evidence |
|----------|--------|-----------|----------|
| Desktop Shell | Tauri V1 | Fast cross-platform, Rust backend | Committed |
| Markdown Flavor | CommonMark + GFM | GitHub standard, widely compatible | Committed |
| Frontend Editor | **ProseMirror** | Robust cursor/selection, mature ecosystem | prosemirror-markdown |
| Markdown Parser | **comrak** | Full AST model, roundtrip to CommonMark | kivikakk/comrak |
| Syntax Highlighting | syntect | Native Rust, fast, themeable | mdBook integration pattern |
| Frontend-Backend IPC | invoke + events | Standard Tauri pattern | tauri/examples/commands |

### 1.5 Technical Context (RESOLVED)

| Unknown | Resolution | Evidence |
|---------|------------|----------|
| Frontend rendering strategy | ProseMirror + prosemirror-markdown | Milkdown architecture pattern |
| Best AST/semantic model crate | comrak | AstNode/NodeValue for WYSIWYM editing |
| ProseMirror-like patterns | prosemirror-markdown bridge | MarkdownParser/Serializer |
| Cursor position mapping approach | ProseMirror Mapping (pros.emirror-transform) | prosemirror-transform/README |
| Live rendering DOM strategy | ProseMirror NodeViews for custom blocks | Milkdown node-view.ts |

---

## 2. Constitution Check

### 2.1 Principles Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| **P1: Write First** | ✅ SUPPORTED | Focus on live rendering enables uninterrupted flow |
| **P2: Markdown as Source of Truth** | ✅ SUPPORTED | Semantic model ↔ source sync maintained |
| **P3: Live Rendering Must Feel Natural** | ✅ SUPPORTED | Cursor/selection system designed for this |
| **P4: Performance Is a Feature** | ✅ SUPPORTED | Rust backend, incremental parsing designed |
| **P5: Local-First Privacy** | ✅ SUPPORTED | No cloud in MVP scope |
| **P6: Beautiful Typography** | ✅ SUPPORTED | Theme system + typography config |
| **P7: Invisible UI** | ✅ SUPPORTED | Calm UI, focus/typewriter modes |
| **P8: Rust Owns Correctness** | ✅ SUPPORTED | Editor engine, parser, serializer in Rust |
| **P9: Tauri + Rust Stack** | ✅ COMMITTED | No change |
| **P10: Document Model Separation** | ✅ ENFORCED | Source → AST → Editing → Presentation layers |
| **P11: CommonMark + GFM** | ✅ COMMITTED | No change |
| **P12: Modular Crate Architecture** | ✅ ENFORCED | semantic/, editor/, renderer/ modules |
| **P13: Single-Pane Invariant** | ✅ ENFORCED | Single editor canvas |
| **P14: Readability Invariant** | ✅ ENFORCED | Live rendering of all constructs |
| **P15: Structural Editing Invariant** | ✅ ENFORCED | Transform rules for Enter/Backspace/Tab |
| **P16: Keyboard-First** | ✅ SUPPORTED | All actions keyboard-accessible |
| **P17: File Operations First-Class** | ✅ SUPPORTED | Workspace, recovery, atomic saves |
| **P18: Crash Recovery** | ✅ IMPLEMENTING | Snapshot + restore system |
| **P19: Atomic Saves** | ✅ SUPPORTED | Temp file + rename |
| **P20: Safe Rendering** | ✅ SUPPORTED | Sanitization policy defined |
| **P21: MVP Feature Focus** | ✅ ENFORCED | Scope limited to M2-M3 |
| **P22: Test-Driven Development** | ✅ ENFORCED | Fixtures + regression tests required |
| **P23: ADR for Architecture** | ✅ IN PROGRESS | This plan serves as ADR |

### 2.2 Experience Invariants (Non-Negotiable)

| Invariant | Implementation Strategy |
|-----------|-------------------------|
| **Single-Pane** | Single editor canvas, no split view |
| **Readability** | All markdown constructs rendered visually |
| **Cursor** | Semantic position tracking, no cursor traps |
| **Structure** | Smart Enter/Backspace/Tab for lists/quotes |
| **Fidelity** | Roundtrip tests ensure no data loss |
| **Calmness** | Minimal chrome, focus/typewriter modes |
| **Local-Trust** | Atomic saves, recovery system, external change detection |

### 2.3 Gate Evaluation

| Gate | Status | Notes |
|------|--------|-------|
| No mode-switching required | ✅ PASS | Single-pane live rendering |
| Document always serializable | ✅ PASS | Source ↔ AST ↔ Source roundtrip |
| Startup < 2s | ⚠️ PENDING | Need benchmark after M2 |
| Edit invariants testable | ✅ PASS | Fixture-based regression tests |
| Crash recovery implemented | ⚠️ PARTIAL | Snapshot system, restore pending |

---

## 3. Phase 0: Research

### 3.1 Open Questions (RESOLVED)

| Question | Resolution | Evidence |
|----------|------------|----------|
| **Q1: DOM rendering strategy?** | ProseMirror with prosemirror-markdown | prosemirror-markdown/index.ts |
| **Q2: Cursor position mapping?** | ProseMirror Mapping via prosemirror-transform | prosemirror-transform/README |
| **Q3: Transform rule implementation?** | ProseMirror keymap + input rules | prosemirror-example-setup |
| **Q4: Undo/redo granularity?** | Command-based via ProseMirror transactions | ProseMirror history plugin |
| **Q5: Incremental parsing approach?** | Full re-parse with comrak (fast enough) | comrak benchmarks |

### 3.2 Research Tasks (COMPLETED)

| Task | Agent | Status | Key Findings |
|------|-------|--------|--------------|
| Tauri editor patterns | bg_8b81e27c | ✅ Complete | Hybrid (ProseMirror + CodeMirror) recommended |
| Rust markdown crates | bg_983a82f5 | ✅ Complete | comrak for AST, syntect for highlighting |
| ProseMirror patterns | bg_a4a4fc9c | ✅ Complete | prosemirror-markdown bridge, Milkdown architecture |

### 3.3 Research Consolidation

Research results consolidated into `research.md`. Key decisions:

1. **Frontend:** ProseMirror + prosemirror-markdown (not custom contenteditable)
2. **Backend:** comrak instead of pulldown-cmark (for AST-based editing)
3. **Architecture:** Milkdown-style layered approach (parser → state → view)

---

## 4. Phase 1: Design

### 4.1 Data Model

#### 4.1.1 Semantic AST (Semantic Layer)

```rust
// src-tauri/src/semantic/ast.rs

// NOTE: We use comrak's AstNode directly rather than defining our own AST.
// The semantic layer wraps comrak's AST with position mapping.

// comrak provides:
// - AstNode: Root AST node type
// - NodeValue: All possible node values (Heading, Paragraph, CodeBlock, etc.)
// - NodeHtml: Optional HTML node representation

use comrak::{Arena, AstNode, NodeValue};

/// Wrapper around comrak AST with position tracking
pub struct SemanticDocument<'a> {
    arena: &'a Arena,
    root: &'a AstNode<'a>,
    /// Maps byte offsets to source positions
    position_map: Vec<Position>,
    source: &'a str,
}

impl<'a> SemanticDocument<'a> {
    /// Parse markdown source into semantic document
    pub fn parse(source: &str) -> (Self, Arena) {
        let arena = Arena::new();
        let root = comrak::parse_document(&arena, source, &comrak::Options::default());
        let position_map = Self::build_position_map(source);
        SemanticDocument { arena, root, position_map, source }
    }
    
    /// Find AST node at byte offset
    pub fn node_at_offset(&self, offset: usize) -> Option<&AstNode> {
        self.find_node(self.root, offset)
    }
    
    /// Get byte range of an AST node in source
    pub fn node_byte_range(&self, node: &AstNode) -> Option<(usize, usize)> {
        // Implementation uses comrak's source_pos
        node.source_pos().first_column.map(|start| {
            let end = node.source_pos().last_column.unwrap_or(start);
            (start, end)
        })
    }
    
    /// Convert byte offset to line/column position
    pub fn offset_to_position(&self, offset: usize) -> Position {
        // Use position_map for efficient lookup
        ...
    }
}
```

#### 4.1.2 Position Mapping (Editing Layer)

```rust
// src-tauri/src/semantic/position.rs

/// Position in source document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub offset: usize,       // Byte offset in source
    pub line: u32,          // 0-indexed line
    pub column: u32,        // Column within line
}

/// Range in source document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Anchor for selections (ProseMirror-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Left(Position),
    Right(Position),
}

/// Selection with anchor and head
#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    pub anchor: Anchor,
    pub head: Position,
}

impl Selection {
    pub fn range(&self) -> Range { ... }
    pub fn is_empty(&self) -> bool { ... }
    pub fn is_forward(&self) -> bool { ... }
}
```

#### 4.1.3 Document Model (Core)

```rust
// src-tauri/src/model/document.rs

// Uses comrak AST under the hood
use comrak::{Arena, AstNode};
use super::super::semantic::{SemanticDocument, Position, Selection};

/// Core document representation
pub struct Document {
    pub id: DocumentId,
    pub path: Option<PathBuf>,         // None for untitled
    pub source: String,                 // Raw markdown source
    pub semantic: SemanticDocument<'static>, // Parsed with comrak
    pub dirty: bool,                    // Unsaved changes
    pub cursor: Cursor,
    pub selection: Option<Selection>,
    pub undo_stack: Vec<Command>,
    pub redo_stack: Vec<Command>,
}

pub struct Cursor {
    pub position: Position,
    pub visible: bool,
}

impl Document {
    pub fn new() -> Self { ... }
    pub fn from_source(source: String) -> Self { 
        let semantic = SemanticDocument::parse(&source);
        Document { source, semantic, ... }
    }
    pub fn from_path(path: PathBuf) -> Result<Self> { ... }
    
    // State queries
    pub fn is_dirty(&self) -> bool { self.dirty }
    pub fn is_untitled(&self) -> bool { self.path.is_none() }
    
    // AST ↔ Source sync
    pub fn update_source(&mut self, new_source: String) { 
        self.source = new_source;
        self.semantic = SemanticDocument::parse(&self.source);
        self.dirty = true;
    }
    
    /// Serialize AST back to markdown
    pub fn serialize(&self) -> String {
        comrak::format_commonmark(self.semantic.root(), &comrak::Options::default())
    }
}
```

#### 4.1.2 Position Mapping (Editing Layer)

```rust
// src-tauri/src/semantic/position.rs

/// Position in source document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub offset: usize,       // Byte offset in source
    pub line: u32,           // 0-indexed line
    pub column: u32,         // Column within line
}

/// Range in source document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Anchor for selections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Anchor {
    Left(Position),
    Right(Position),
}

/// Selection with anchor and head
#[derive(Debug, Clone, PartialEq)]
pub struct Selection {
    pub anchor: Anchor,
    pub head: Position,
}

impl Selection {
    pub fn range(&self) -> Range { ... }
    pub fn is_empty(&self) -> bool { ... }
    pub fn is_forward(&self) -> bool { ... }
}

/// Maps between source positions and AST nodes
pub trait PositionMapper {
    /// Find AST node containing position
    fn node_at(&self, pos: Position) -> Option<&AstNode>;
    
    /// Get range of AST node in source
    fn node_range(&self, node: &AstNode) -> Option<Range>;
    
    /// Convert AST offset to source position
    fn offset_to_position(&self, offset: usize) -> Position;
    
    /// Convert source position to AST offset
    fn position_to_offset(&self, pos: Position) -> usize;
}
```

#### 4.1.3 Document Model (Core)

```rust
// src-tauri/src/model/document.rs

use super::super::semantic::{AstNode, Position, Range, Selection};

/// Core document representation
pub struct Document {
    pub id: DocumentId,
    pub path: Option<PathBuf>,         // None for untitled
    pub source: String,                 // Raw markdown source
    pub ast: AstNode,                   // Parsed semantic AST
    pub dirty: bool,                     // Unsaved changes
    pub cursor: Cursor,
    pub selection: Option<Selection>,
    pub undo_stack: Vec<Command>,
    pub redo_stack: Vec<Command>,
}

pub struct Cursor {
    pub position: Position,
    pub visible: bool,
}

impl Document {
    pub fn new() -> Self { ... }
    pub fn from_source(source: String) -> Self { ... }
    pub fn from_path(path: PathBuf) -> Result<Self> { ... }
    
    // State queries
    pub fn is_dirty(&self) -> bool { self.dirty }
    pub fn isUntitled(&self) -> bool { self.path.is_none() }
    
    // AST ↔ Source sync
    pub fn update_source(&mut self, new_source: String) { ... }
    pub fn reparse(&mut self) { ... }
}
```

#### 4.1.4 Editor Commands (Editing Layer)

```rust
// src-tauri/src/editor/commands.rs

/// Editor commands (operations)
#[derive(Debug, Clone)]
pub enum Command {
    Insert { pos: Position, text: String },
    Delete { range: Range, deleted: String },
    Replace { range: Range, new_text: String },
    Format { range: Range, format: Format },
    Transform { transform: Transform },
}

#[derive(Debug, Clone)]
pub enum Format {
    Bold,
    Italic,
    Strikethrough,
    Code,
    Link { href: String },
    Image { src: String },
}

#[derive(Debug, Clone)]
pub enum Transform {
    Enter,
    Backspace,
    Tab,
    ShiftTab,
    EnterInListItem { empty: bool },
    EnterInBlockQuote,
    // ... more structural transforms
}
```

### 4.2 Interface Contracts

#### 4.2.1 Document Service Interface

```rust
// src-tauri/src/services/document.rs

/// Document service interface
#[tauri::interface]
pub trait DocumentService {
    /// Create new untitled document
    async fn create() -> Result<Document>;
    
    /// Open existing document
    async fn open(path: PathBuf) -> Result<Document>;
    
    /// Save document
    async fn save(id: DocumentId, path: Option<PathBuf>) -> Result<()>;
    
    /// Get document content
    async fn get_content(id: DocumentId) -> Result<String>;
    
    /// Apply command to document
    async fn apply(id: DocumentId, cmd: Command) -> Result<Document>;
    
    /// Get document AST
    async fn get_ast(id: DocumentId) -> Result<AstNode>;
    
    /// Update document source (for live editing sync)
    async fn update_source(id: DocumentId, source: String) -> Result<()>;
}
```

#### 4.2.2 Editor Service Interface

```rust
// src-tauri/src/services/editor.rs

/// Editor service interface
#[tauri::interface]
pub trait EditorService {
    /// Get cursor position
    async fn cursor(id: DocumentId) -> Result<Position>;
    
    /// Set cursor position
    async fn set_cursor(id: DocumentId, pos: Position) -> Result<()>;
    
    /// Get selection
    async fn selection(id: DocumentId) -> Result<Option<Selection>>;
    
    /// Set selection
    async fn set_selection(id: DocumentId, sel: Selection) -> Result<()>;
    
    /// Execute transform
    async fn transform(id: DocumentId, t: Transform) -> Result<()>;
    
    /// Undo
    async fn undo(id: DocumentId) -> Result<()>;
    
    /// Redo
    async fn redo(id: DocumentId) -> Result<()>;
}
```

#### 4.2.3 Recovery Service Interface

```rust
// src-tauri/src/services/recovery.rs

/// Recovery service interface
#[tauri::interface]
pub trait RecoveryService {
    /// List available snapshots
    async fn list_snapshots() -> Result<Vec<Snapshot>>;
    
    /// Restore from snapshot
    async fn restore(snapshot_id: SnapshotId) -> Result<Document>;
    
    /// Delete snapshot
    async fn delete_snapshot(snapshot_id: SnapshotId) -> Result<()>;
    
    /// Get recovery info for startup
    async fn get_recovery_info() -> Result<Option<RecoveryInfo>>;
}
```

### 4.3 Quickstart

#### 4.3.1 Rust Development Setup

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Build the project
cd rustnote/src-tauri
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

#### 4.3.2 Frontend Development Setup

```bash
# Install dependencies (if needed)
cd rustnote/www
# No npm needed - pure static HTML/JS/CSS

# Serve locally (any static server works)
python3 -m http.server 8080

# Or use Tauri dev server
cd rustnote
cargo tauri dev
```

#### 4.3.3 Key Dependencies

```toml
# src-tauri/Cargo.toml key dependencies
[dependencies]
comrak = "0.52"            # ⭐ Markdown AST parsing (replaces pulldown-cmark)
syntect = "0.24"            # Syntax highlighting
notify = "6.1"              # File watching
serde = { version = "1.0" } # Serialization
serde_json = "1.0"          # JSON
thiserror = "1.0"           # Error handling
log = "0.4"                 # Logging
env_logger = "0.10"          # Logging
parking_lot = "0.12"        # Mutexes
uuid = { version = "1.0", features = ["v4"] }
directories = "5.0"         # App dirs
tempfile = "3.8"            # Atomic saves
```

```html
<!-- www/index.html - Add ProseMirror via CDN -->
<script type="module">
  import { EditorState } from 'https://cdn.jsdelivr.net/npm/prosemirror-state@1.4.3/+esm';
  import { EditorView } from 'https://cdn.jsdelivr.net/npm/prosemirror-view@1.33.0/+esm';
  import { Schema } from 'https://cdn.jsdelivr.net/npm/prosemirror-model@1.22.0/+esm';
  import { defaultMarkdownParser, MarkdownSerializer } from 'https://cdn.jsdelivr.net/npm/prosemirror-markdown@1.13.0/+esm';
  import { keymap } from 'https://cdn.jsdelivr.net/npm/prosemirror-keymap@1.2.2/+esm';
  import { baseKeymap } from 'https://cdn.jsdelivr.net/npm/prosemirror-commands@1.6.0/+esm';
</script>
```

---

## 5. Phase 2: Implementation Breakdown

### 5.1 Milestone 2: Live Rendering Foundation

**Timeline:** Iteration 2 (this plan)  
**Goal:** Single-pane live markdown editing with cursor/selection

#### M2.1: Semantic Model Layer (Rust)

| Task | File | Description | Effort |
|------|------|-------------|--------|
| M2.1.1 | `semantic/mod.rs` | Module structure | 1h |
| M2.1.2 | `semantic/ast.rs` | AST node definitions | 4h |
| M2.1.3 | `semantic/position.rs` | Position/Range types | 4h |
| M2.1.4 | `semantic/transform.rs` | Source↔AST sync | 8h |
| M2.1.5 | `parser/markdown.rs` | Parser integration (AST output) | 8h |
| M2.1.6 | `parser/syntax.rs` | Syntect integration | 4h |
| **Subtotal** | | | **29h** |

#### M2.2: Editor Engine (Rust)

| Task | File | Description | Effort |
|------|------|-------------|--------|
| M2.2.1 | `editor/mod.rs` | Module structure | 1h |
| M2.2.2 | `editor/cursor.rs` | Cursor movement rules | 8h |
| M2.2.3 | `editor/selection.rs` | Selection handling | 6h |
| M2.2.4 | `editor/commands.rs` | Command types | 4h |
| M2.2.5 | `editor/undo.rs` | Undo/redo system | 8h |
| **Subtotal** | | | **27h** |

#### M2.3: Renderer (Frontend)

| Task | File | Description | Effort |
|------|------|-------------|--------|
| M2.3.1 | `renderer/state.rs` | Render state management | 4h |
| M2.3.2 | `renderer/inline.rs` | Inline span rendering | 8h |
| M2.3.3 | `renderer/blocks.rs` | Block rendering | 8h |
| M2.3.4 | `editor.js` (refactor) | Integrate with Rust backend | 12h |
| **Subtotal** | | | **32h** |

#### M2.4: Services Integration

| Task | File | Description | Effort |
|------|------|-------------|--------|
| M2.4.1 | `services/document.rs` | DocumentService impl | 4h |
| M2.4.2 | `services/editor.rs` | EditorService impl | 4h |
| M2.4.3 | `services/mod.rs` | Service module restructure | 2h |
| **Subtotal** | | | **10h** |

**M2 Total: ~98h (22 tasks)**

---

### 5.2 Milestone 3: Editing Semantics

**Timeline:** Iteration 3  
**Goal:** Smart Enter/Backspace/Tab, task lists, formatting shortcuts

#### M3.1: Smart Transforms (Rust)

| Task | File | Description | Effort |
|------|------|-------------|--------|
| M3.1.1 | `editor/transforms.rs` | Transform rule engine | 8h |
| M3.1.2 | `transforms/list.rs` | List item transforms | 6h |
| M3.1.3 | `transforms/quote.rs` | BlockQuote transforms | 4h |
| M3.1.4 | `transforms/heading.rs` | Heading transforms | 4h |
| M3.1.5 | `transforms/task.rs` | Task list toggle | 6h |
| **Subtotal** | | | **28h** |

#### M3.2: Formatting Commands

| Task | File | Description | Effort |
|------|------|-------------|--------|
| M3.2.1 | `editor/format.rs` | Bold, italic, etc. | 6h |
| M3.2.2 | `editor/link.rs` | Link insertion | 4h |
| M3.2.3 | `editor/image.rs` | Image insertion | 6h |
| **Subtotal** | | | **16h** |

#### M3.3: Keyboard Shortcuts (Frontend)

| Task | File | Description | Effort |
|------|------|-------------|--------|
| M3.3.1 | `editor.js` | Shortcut handler | 8h |
| M3.3.2 | `shortcuts.md` | Shortcut reference | 1h |
| **Subtotal** | | | **9h** |

#### M3.4: Editor Regression Tests

| Task | Description | Effort |
|------|-------------|--------|
| M3.4.1 | List behavior fixtures | 8h |
| M3.4.2 | Cursor movement fixtures | 8h |
| M3.4.3 | Selection fixtures | 6h |
| M3.4.4 | Task toggle fixtures | 4h |
| **Subtotal** | | | **26h** |

**M3 Total: ~79h (14 tasks)**

---

### 5.3 Task Dependencies

```
M2 (Live Rendering):
├── M2.1 (Semantic Model) ──────────┐
│   └── M2.2 (Editor Engine) ───────┼── M2.3 (Renderer) ─── M2.4 (Services)
│   └── M2.4 (Services) ────────────┘

M3 (Editing Semantics):
├── M3.1 (Transforms) ←──────────────┐
├── M3.2 (Formatting) ←── M2.2 ──────┤
└── M3.3 (Shortcuts) ←── M2.3 ──────┘
    └── M3.4 (Tests) ←── M3.1-M3.3
```

---

## 6. File Structure (Final)

```
rustnote/
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── icons/
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands/
│       │   ├── mod.rs
│       │   ├── document.rs
│       │   ├── workspace.rs
│       │   ├── settings.rs
│       │   └── export.rs
│       ├── model/
│       │   ├── mod.rs
│       │   ├── document.rs
│       │   ├── settings.rs
│       │   └── workspace.rs
│       ├── parser/
│       │   ├── mod.rs
│       │   ├── markdown.rs
│       │   └── syntax.rs
│       ├── semantic/           # ⭐ NEW
│       │   ├── mod.rs
│       │   ├── ast.rs
│       │   ├── position.rs
│       │   └── transform.rs
│       ├── editor/             # ⭐ NEW
│       │   ├── mod.rs
│       │   ├── cursor.rs
│       │   ├── selection.rs
│       │   ├── commands.rs
│       │   ├── transforms.rs
│       │   └── undo.rs
│       ├── renderer/           # ⭐ NEW (lightweight frontend module)
│       │   ├── mod.rs
│       │   ├── inline.rs
│       │   ├── blocks.rs
│       │   └── state.rs
│       ├── services/           # ⭐ RESTRUCTURED
│       │   ├── mod.rs
│       │   ├── document.rs
│       │   ├── editor.rs
│       │   ├── workspace.rs
│       │   ├── export.rs
│       │   ├── settings.rs
│       │   └── recovery.rs
│       ├── recovery/           # ⭐ NEW
│       │   ├── mod.rs
│       │   ├── snapshots.rs
│       │   └── restore.rs
│       └── watcher/            # ⭐ NEW
│           ├── mod.rs
│           └── notify.rs
└── www/
    ├── index.html
    └── src/
        ├── scripts/
        │   ├── app.js
        │   ├── editor.js       # ⭐ REFACTOR
        │   ├── renderer.js     # ⭐ REFACTOR
        │   ├── search.js
        │   ├── sidebar.js
        │   ├── outline.js
        │   └── settings.js
        └── styles/
            ├── main.css
            ├── theme-light.css
            ├── theme-dark.css
            └── editor.css      # ⭐ NEW
```

---

## 7. Testing Strategy

### 7.1 Test Fixtures Required

```
fixtures/
├── markdown/                    # Parse tests
│   ├── headings.md
│   ├── emphasis.md
│   ├── lists.md
│   ├── task-lists.md
│   ├── tables.md
│   └── ...
├── editor/                     # Behavior tests
│   ├── list-behaviors.md
│   ├── cursor-across-inline.md
│   └── selection-across-blocks.md
├── roundtrip/                  # Fidelity tests
│   └── *.md
└── export/                     # Export tests
    └── expected/
```

### 7.2 Test Categories

| Category | Target | Method |
|----------|--------|--------|
| Parse correctness | All CommonMark + GFM | Fixture comparison |
| Roundtrip fidelity | All constructs | Parse → serialize → compare |
| Cursor invariants | Inline/blocks | Automated key sequences |
| Transform rules | Lists, quotes, headings | State machine tests |
| Undo/redo | All commands | Command sequence tests |
| Performance | Large documents | Benchmark suite |

---

## 8. Acceptance Criteria

### 8.1 Milestone 2 Acceptance

| Criterion | Method | Target |
|-----------|--------|--------|
| M2.1 | Semantic AST from parser | All CommonMark constructs parse |
| M2.2 | Editor engine compiles | Cursor/selection in Rust |
| M2.3 | Live rendering works | Headings, emphasis, links render |
| M2.4 | Document service works | New/open/save work end-to-end |

### 8.2 Milestone 3 Acceptance

| Criterion | Method | Target |
|-----------|--------|--------|
| M3.1 | Transform tests pass | Enter/Backspace/Tab for lists |
| M3.2 | Formatting works | Bold, italic, code shortcuts |
| M3.3 | Shortcuts work | All documented shortcuts |
| M3.4 | Regression tests pass | No cursor traps, no data loss |

### 8.3 Experience Acceptance

| Invariant | Verification |
|-----------|--------------|
| Single-Pane | No mode switching required |
| Readability | Headings look like headings, lists look like lists |
| Cursor | Arrow keys navigate predictably |
| Structure | Enter on empty list exits list |
| Fidelity | Edit-save-reopen identical |

---

## 9. Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| Cursor bugs | HIGH | HIGH | Early integration tests, fixture-based regression |
| Performance on large docs | MEDIUM | HIGH | Incremental parsing, virtualized rendering |
| DOM rendering complexity | HIGH | MEDIUM | Research phase before implementation |
| Transform edge cases | MEDIUM | MEDIUM | State machine testing, fixture coverage |

---

## 10. Next Steps

### Immediate (This Iteration)

1. **Wait for research agents** (bg_8b81e27c, bg_983a82f5, bg_a4a4fc9c)
2. **Consolidate research findings** into research.md
3. **Begin M2.1: Semantic Model** implementation
4. **Set up Cargo workspace** with new modules

### Iteration 2 End State

- Live rendered markdown in editor (headings, emphasis, lists, links, code)
- Basic cursor/selection handling
- Undo/redo capability
- Working smart editing (Enter, Backspace, Tab for lists)
- All CommonMark + GFM constructs parseable

### Iteration 3 Scope

- Task list support
- Image support with relative paths
- Table rendering
- Focus mode
- Typewriter mode
- Crash recovery system
- Outline/TOC panel

---

## 11. Appendix

### A. References

| Reference | Path |
|-----------|------|
| Constitution | `outputs/iteration-2/constitution.md` |
| Feature Spec | `outputs/iteration-2/spec.md` |
| Gap Analysis | `outputs/iteration-2/gap-analysis.md` |
| Research | `outputs/iteration-2/research.md` ✅ Complete |

### B. Key Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| **comrak** | 0.52 | ⭐ Markdown AST parsing (replaces pulldown-cmark) |
| syntect | 0.24 | Syntax highlighting |
| notify | 6.1 | File watching |
| serde | 1.0 | Serialization |
| thiserror | 1.0 | Error handling |
| tempfile | 3.8 | Atomic saves |

**Frontend (CDN):**
| Package | Version | Purpose |
|---------|---------|---------|
| prosemirror-state | 1.4.3 | Editor state management |
| prosemirror-view | 1.33.0 | Editor view/rendering |
| prosemirror-model | 1.22.0 | Document model |
| prosemirror-markdown | 1.13.0 | Markdown ↔ Doc bridge |
| prosemirror-keymap | 1.2.2 | Keyboard shortcuts |
| prosemirror-commands | 1.6.0 | Editor commands |

### C. Research Agent Status

| Agent | Task ID | Status |
|-------|---------|--------|
| Tauri editor patterns | bg_8b81e27c | ✅ Complete |
| Rust markdown crates | bg_983a82f5 | ✅ Complete |
| ProseMirror patterns | bg_a4a4fc9c | ✅ Complete | |

---

**Plan Created:** 2026-04-11  
**Plan Version:** 1.0  
**Next Action:** Await research agent completion, then begin M2.1 implementation
