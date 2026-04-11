# RustNote Implementation Plan

## 1. Technical Context

### 1.1 Technology Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| **Desktop Shell** | Tauri v1 | Spec requirement; cross-platform, small binary, Rust-native |
| **Frontend** | Vanilla HTML/CSS + TypeScript | Minimal abstraction over DOM; predictable performance; easier Rust interop |
| **Editor Engine** | Custom Rust implementation on `rope` crate | Cursor/selection semantics owned by Rust; efficient text operations |
| **Markdown Parser** | `pulldown-cmark` + custom extensions | CommonMark + GFM support; extensible for structural editing |
| **Serialization** | `serde` + `serde_json` for settings; direct file I/O for documents | Familiar Rust serialization patterns |
| **Crash Recovery** | Json snapshot files in app data directory | Simple, reliable, platform-independent |
| **Export** | `pulldown-cmark` for HTML; `printpdf` or `wkhtmltopdf` bridge for PDF | Proven libraries; interface boundary allows swap |

### 1.2 Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    UI Layer (WebView)                    │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌──────────┐  │
│  │ Editor  │  │ Sidebar │  │ Outline │  │ Toolbar  │  │
│  │ Canvas  │  │ Tree    │  │ Panel   │  │ /Dialogs │  │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬─────┘  │
│       │            │            │             │         │
│  ─────┴────────────┴────────────┴─────────────┴──────  │
│              Tauri Commands (IPC Boundary)               │
└─────────────────────────────────────────────────────────┘
                          │
┌─────────────────────────────────────────────────────────┐
│                    Rust Core (Tauri Backend)            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ DocumentSvc  │  │ EditorSvc    │  │ WorkspaceSvc │  │
│  │ - open/save  │  │ - commands   │  │ - file tree  │  │
│  │ - parse      │  │ - selection   │  │ - CRUD ops   │  │
│  │ - outline    │  │ - undo/redo   │  │              │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
│         │                 │                  │          │
│  ┌──────┴─────────────────┴──────────────────┴───────┐  │
│  │              EditorEngine (rope-based)             │  │
│  │  - Text storage with efficient insertions/deletions │  │
│  │  - Cursor/selection model                          │  │
│  │  - Structural node awareness (headings, lists, etc) │  │
│  └─────────────────────────────────────────────────────┘  │
│  ┌─────────────────────────────────────────────────────┐  │
│  │              MarkdownParser (pulldown-cmark)         │  │
│  │  - Parse document to semantic tree                  │  │
│  │  - Serialize semantic tree to Markdown               │  │
│  │  - GFM extension: tables, task lists, strikethrough  │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 1.3 Key Rust Crates

```toml
[dependencies]
# Text editing
rope = "0.5"              # Efficient rope-based text storage
unicode-segmentation = "1" # Grapheme cluster awareness

# Markdown
pulldown-cmark = "0.9"    # CommonMark + GFM parsing

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# File operations
walkdir = "2"              # Directory traversal for workspace
notify = "6"               # File watching for external changes
tempfile = "3"             # Safe file writes (temp + rename)

# Async (Tauri commands are sync but internal ops may need)
tokio = { version = "1", features = ["sync"] }

# Logging
log = "0.4"
env_logger = "0.10"

# Error handling
thiserror = "1"
anyhow = "1"

# Date/time for recovery snapshots
chrono = "0.4"
```

---

## 2. Constitution Check

### 2.1 From `constitution.md`

| Constitutional Principle | Alignment Strategy |
|--------------------------|-------------------|
| **Local-first, no network in core** | Export is the only potential network path; architecture keeps export behind interface boundary. Core editing is fully offline. |
| **Rust core ownership** | All parsing, cursor semantics, serialization, recovery, export logic lives in Rust. UI only renders derived state and dispatches commands. |
| **Cross-platform** | Tauri handles platform differences; Rust stdlib ensures consistent behavior; file path handling uses `dirs` crate for platform-appropriate locations. |
| **Markdown fidelity** | Roundtrip tests required: parse → serialize → parse must produce identical AST for all supported constructs. |

### 2.2 Architecture Decisions Derived from Constitution

| Decision | Constitutional Basis |
|----------|---------------------|
| **EditorEngine owns cursor/selection** | Constitution: "Cursor and selection semantics" must be Rust-owned |
| **MarkdownParser produces semantic tree, not just HTML** | Constitution: "Markdown fidelity" requires reversible transformation |
| **Service interfaces are narrow and synchronous** | Constitution: "Narrow service interfaces preferred over exposing unstable internal types" |
| **Export behind interface boundary** | Constitution: "New export formats can be added without core changes" |

---

## 3. Phase 0: Research (Resolve Unknowns)

### 3.1 Unknowns to Resolve Before Implementation

| # | Unknown | Resolution Approach | Deliverable |
|---|---------|-------------------|-------------|
| R1 | **Tauri v1 IPC performance for keystroke-level events** | Benchmark: 1000 character insertions over IPC vs direct Rust benchmark | Decision:是否需要 batch keystrokes or native text input handling |
| R2 | **pulldown-cmark AST fidelity for structural editing** | Test roundtrip: parse → serialize → parse for tables, task lists, nested lists | Decision:是否需要 custom AST layer or use pulldown-cmark directly |
| R3 | **File watching granularity** | `notify` crate benchmarking for folder with 1000 Markdown files | Decision: debounce interval, watcher scope |
| R4 | **Crash recovery granularity** | Design snapshot strategy: time-based (every N seconds) or change-count-based | Decision: snapshot format and trigger conditions |
| R5 | **Undo/redo strategy for structural operations** | Research: operation log vs. command pattern vs. persistent undo stack | Decision: undo granularity (character vs. structural) |
| R6 | **Frontend framework choice** | Evaluate: vanilla TS vs. lighter alternatives (solid, svelte) | Decision: minimal framework or raw DOM |

### 3.2 Resolution Timeline

- **Week 1, Day 1-2**: Resolve R1, R2 via prototype
- **Week 1, Day 3-4**: Resolve R3, R4 via prototype
- **Week 1, Day 5**: Resolve R5, R6 with team discussion

**Note**: This plan assumes resolution in the "optimal" direction. Implementation phases may adjust based on prototype findings.

---

## 4. Phase 1: Design

### 4.1 Data Model

#### 4.1.1 Core Types (Rust)

```rust
// src/core/document.rs

/// Cursor position in the document
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    pub offset: usize,           // Byte offset in rope
    pub affinity: CursorAffinity, // Left/right of grapheme cluster
}

/// Selection range (may be empty for caret)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Cursor,
    pub head: Cursor,
}

/// Semantic node types in the document
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Document,
    Heading { level: u8 },        // 1-6
    Paragraph,
    Bold,
    Italic,
    Strikethrough,
    InlineCode,
    CodeBlock { lang: Option<String> },
    BlockQuote,
    List { kind: ListKind, tight: bool },
    ListItem { kind: ListKind },
    TaskItem { checked: bool },
    Table { cols: usize, header_row: bool },
    TableRow,
    TableCell,
    Link { dest: String },
    Image { alt: String, dest: String },
    HorizontalRule,
    SoftBreak,
    HardBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListKind {
    Unordered,
    Ordered,
}

/// Node in the semantic tree
#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub range: Range<usize>,      // Byte range in source
    pub children: Vec<Node>,
}

/// Semantic document tree (not AST - preserves edit intent)
#[derive(Debug, Clone)]
pub struct Document {
    pub root: Node,
    pub source_len: usize,
}
```

#### 4.1.2 Buffer State

```rust
// src/core/buffer.rs

/// In-memory text representation with dirty tracking
pub struct Buffer {
    rope: rope::Rope,
    dirty: bool,
    last_save: Instant,
}

impl Buffer {
    pub fn new() -> Self;
    pub fn from_file(path: &Path) -> Result<Self>;
    pub fn insert(&mut self, cursor: Cursor, text: &str) -> Cursor;
    pub fn delete(&mut self, range: Range<usize>) -> Cursor;
    pub fn text(&self) -> &rope::Rope;
    pub fn is_dirty(&self) -> bool;
    pub fn mark_saved(&mut self);
}
```

#### 4.1.3 Editor State

```rust
// src/core/editor.rs

/// All state for a single editor instance
pub struct EditorState {
    buffer: Buffer,
    selection: Selection,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
    document: Document,           // Parsed semantic tree
    last_parse: Instant,
}

impl EditorState {
    pub fn new() -> Self;
    pub fn load(&mut self, path: &Path) -> Result<()>;
    pub fn save(&mut self, path: &Path) -> Result<()>;
    pub fn apply(&mut self, cmd: EditorCommand) -> Result<CommandResult>;
    pub fn undo(&mut self) -> Result<CommandResult>;
    pub fn redo(&mut self) -> Result<CommandResult>;
}
```

#### 4.1.4 Workspace Model

```rust
// src/core/workspace.rs

#[derive(Debug, Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root_path: PathBuf,
    pub tree: FileNode,
    pub active_file: Option<PathBuf>,
}

impl Workspace {
    pub fn open(root: &Path) -> Result<Self>;
    pub fn create_file(&mut self, parent: &Path, name: &str) -> Result<PathBuf>;
    pub fn rename_file(&mut self, path: &Path, new_name: &str) -> Result<()>;
    pub fn delete_file(&mut self, path: &Path) -> Result<()>;
}
```

#### 4.1.5 Settings

```rust
// src/core/settings.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: Theme,
    pub auto_save: AutoSaveSettings,
    pub editor: EditorSettings,
    pub export: ExportSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSaveSettings {
    pub enabled: bool,
    pub debounce_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_family: String,
    pub font_size: u16,
    pub content_width: u16,        // in pixels
    pub line_height: f32,
    pub focus_mode: bool,
    pub typewriter_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub html_mode: HtmlExportMode,
    pub pdf_page_size: PdfPageSize,
    pub pdf_margins: PdfMargins,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Light,
            auto_save: AutoSaveSettings {
                enabled: true,
                debounce_ms: 2000,
            },
            editor: EditorSettings {
                font_family: "System UI".into(),
                font_size: 16,
                content_width: 720,
                line_height: 1.6,
                focus_mode: false,
                typewriter_mode: false,
            },
            export: ExportSettings {
                html_mode: HtmlExportMode::Standalone,
                pdf_page_size: PdfPageSize::A4,
                pdf_margins: PdfMargins::default(),
            },
        }
    }
}
```

#### 4.1.6 Recovery Snapshot

```rust
// src/core/recovery.rs

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoverySnapshot {
    pub id: Uuid,
    pub file_path: PathBuf,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub cursor_offset: usize,
}

impl RecoverySnapshot {
    pub fn new(file_path: PathBuf, content: String, cursor_offset: usize) -> Self;
    pub fn save(&self, app_data_dir: &Path) -> Result<()>;
    pub fn list(app_data_dir: &Path) -> Result<Vec<Self>>;
    pub fn restore(&self) -> Result<(String, usize)>;
}
```

### 4.2 Service Contracts (Public API)

#### 4.2.1 Tauri Commands

```rust
// src-tauri/src/commands.rs

/// DocumentService
#[tauri::command]
pub fn doc_open(path: String) -> Result<DocOpenResult, String>;

#[tauri::command]
pub fn doc_save(path: String, content: String) -> Result<(), String>;

#[tauri::command]
pub fn doc_reload(path: String) -> Result<String, String>;

#[tauri::command]
pub fn doc_get_outline(content: String) -> Result<Outline, String>;

/// EditorService
#[tauri::command]
pub fn editor_apply_command(cmd: EditorCommand) -> Result<CommandResult, String>;

#[tauri::command]
pub fn editor_get_state() -> Result<EditorState, String>;

#[tauri::command]
pub fn editor_set_selection(selection: Selection) -> Result<(), String>;

#[tauri::command]
pub fn editor_undo() -> Result<CommandResult, String>;

#[tauri::command]
pub fn editor_redo() -> Result<CommandResult, String>;

/// WorkspaceService
#[tauri::command]
pub fn workspace_open(root: String) -> Result<Workspace, String>;

#[tauri::command]
pub fn workspace_list_files(root: String) -> Result<Vec<FileNode>, String>;

#[tauri::command]
pub fn workspace_create_file(parent: String, name: String) -> Result<String, String>;

#[tauri::command]
pub fn workspace_rename_file(path: String, new_name: String) -> Result<(), String>;

#[tauri::command]
pub fn workspace_delete_file(path: String) -> Result<(), String>;

/// ExportService
#[tauri::command]
pub fn export_html(content: String, mode: HtmlExportMode) -> Result<String, String>;

#[tauri::command]
pub fn export_pdf(content: String, options: PdfOptions) -> Result<Vec<u8>, String>;

/// SettingsService
#[tauri::command]
pub fn settings_get() -> Result<Settings, String>;

#[tauri::command]
pub fn settings_update(settings: Settings) -> Result<(), String>;

/// RecoveryService
#[tauri::command]
pub fn recovery_list() -> Result<Vec<RecoverySnapshotMeta>, String>;

#[tauri::command]
pub fn recovery_restore(id: String) -> Result<RecoveryData, String>;

#[tauri::command]
pub fn recovery_delete(id: String) -> Result<(), String>;
```

### 4.3 Module Organization

```
rustnote/
├── src/                          # Rust core (library)
│   ├── lib.rs                    # Library entry, module exports
│   │
│   ├── core/                     # Core domain logic
│   │   ├── mod.rs
│   │   ├── document.rs           # Document, Node, NodeKind types
│   │   ├── buffer.rs             # Buffer (rope-based text storage)
│   │   ├── editor.rs             # EditorState, EditorCommand, CommandResult
│   │   ├── selection.rs          # Cursor, Selection types
│   │   ├── workspace.rs          # Workspace, FileNode
│   │   ├── settings.rs           # Settings types and defaults
│   │   └── recovery.rs           # RecoverySnapshot
│   │
│   ├── parser/                   # Markdown parsing
│   │   ├── mod.rs
│   │   ├── pulldown_adapter.rs   # Adapter wrapping pulldown-cmark
│   │   ├── semantic_tree.rs      # Convert pulldown AST to semantic tree
│   │   └── serializer.rs         # Serialize semantic tree back to Markdown
│   │
│   ├── services/                 # Service implementations
│   │   ├── mod.rs
│   │   ├── document_service.rs   # DocumentService impl
│   │   ├── editor_service.rs     # EditorService impl
│   │   ├── workspace_service.rs # WorkspaceService impl
│   │   ├── export_service.rs     # ExportService impl (HTML/PDF)
│   │   ├── settings_service.rs  # SettingsService impl
│   │   └── recovery_service.rs   # RecoveryService impl
│   │
│   ├── commands.rs               # Tauri command definitions (glue)
│   │
│   └── error.rs                  # Error types (thiserror)
│
├── src-tauri/                    # Tauri application
│   ├── src/
│   │   ├── main.rs               # Tauri app entry
│   │   ├── lib.rs                # App state, plugin registration
│   │   └── tauri.conf.json       # Tauri configuration
│   ├── Cargo.toml
│   └── icons/
│
├── src-ui/                       # Frontend (web assets)
│   ├── index.html
│   ├── main.ts                   # App entry
│   ├── styles/
│   │   ├── main.css              # Base styles, CSS variables for themes
│   │   ├── editor.css            # Editor canvas styles
│   │   ├── sidebar.css           # File tree styles
│   │   └── themes/               # Theme-specific styles
│   │       ├── light.css
│   │       └── dark.css
│   ├── components/
│   │   ├── editor/
│   │   │   ├── EditorCanvas.ts   # Main editing surface
│   │   │   ├── CursorRenderer.ts # Cursor/selection rendering
│   │   │   ├── Renderer.ts       # Markdown rendering (live preview)
│   │   │   └── SelectionManager.ts
│   │   ├── sidebar/
│   │   │   ├── FileTree.ts       # File tree component
│   │   │   └── TreeNode.ts
│   │   ├── outline/
│   │   │   └── OutlinePanel.ts   # TOC panel
│   │   ├── toolbar/
│   │   │   ├── Toolbar.ts
│   │   │   └── ExportDialog.ts
│   │   └── common/
│   │       ├── Modal.ts
│   │       └── ContextMenu.ts
│   ├── services/
│   │   ├── tauri.ts              # Tauri invoke wrappers
│   │   ├── keyboard.ts            # Keyboard handling
│   │   └── clipboard.ts          # Clipboard handling
│   ├── state/
│   │   ├── AppState.ts           # Global app state
│   │   ├── EditorState.ts        # Editor state (from Rust)
│   │   └── WorkspaceState.ts     # Workspace state
│   └── utils/
│       ├── debounce.ts
│       └── logger.ts
│
├── tests/                         # Integration tests
│   ├── parser_roundtrip.rs       # Markdown roundtrip tests
│   ├── editor_commands.rs        # Command application tests
│   └── recovery.rs               # Crash recovery tests
│
├── Cargo.toml                     # Workspace manifest
├── rust-toolchain.toml            # Rust version pinned
└── README.md
```

---

## 5. Phase 2: Implementation Breakdown

### 2.1 Implementation Order

| Phase | Name | Duration | Focus |
|-------|------|----------|-------|
| **Phase 0** | Research | Week 1, Days 1-5 | Resolve unknowns via prototypes |
| **Phase 1** | Foundation | Week 2 | Project scaffold, core types, buffer, basic commands |
| **Phase 2** | Core Editing | Week 3-4 | Parser, structural editing, undo/redo |
| **Phase 3** | File Operations | Week 5 | Workspace, file tree, save/auto-save |
| **Phase 4** | Display | Week 6 | Themes, focus/typewriter modes |
| **Phase 5** | Export | Week 7 | HTML, PDF export |
| **Phase 6** | Polish | Week 8 | Recovery, settings, find/replace, outline |
| **Phase 7** | Hardening | Week 9 | Testing, cross-platform, bug fixes |

### 2.2 Phase 1: Foundation (Week 2)

#### Milestone: Minimal viable editor that can open, edit, and save Markdown

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Mon** | Set up Tauri project scaffold; configure logging; verify empty shell runs | Tauri app builds and launches |
| **Mon** | Create Rust core module structure; define core types (NodeKind, Node, Document, Cursor, Selection) | Core types compile |
| **Tue** | Implement Buffer with rope; insert/delete operations; is_dirty tracking | Buffer passes unit tests |
| **Tue** | Implement basic Tauri commands (doc_open, doc_save); wire to frontend | Files can be opened and saved |
| **Wed** | Implement SelectionManager in frontend; cursor rendering; keyboard input handling | Characters can be typed |
| **Wed** | Basic live rendering (just paragraphs and headings) | Typed text appears formatted |
| **Thu** | Implement bold, italic, inline code, blockquote rendering | Inline formatting works |
| **Thu** | Implement list rendering (ordered/unordered) | Lists render correctly |
| **Fri** | Manual save with dirty indicator | Save button updates dirty state |
| **Fri** | **Milestone checkpoint**: Editor can open `.md` file, type, see formatting, save |

#### Code Ownership During Phase 1

| Module | Owner | Responsibility |
|--------|-------|----------------|
| `src/core/document.rs` | @core-dev | Define and maintain core types |
| `src/core/buffer.rs` | @core-dev | Rope-based text storage |
| `src/commands.rs` | @core-dev | Tauri command definitions |
| `src-ui/components/editor/*` | @ui-dev | Editor canvas, cursor, rendering |
| `src-ui/services/tauri.ts` | @ui-dev | Rust command invocation |

### 2.3 Phase 2: Core Editing (Week 3-4)

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Mon** | Integrate pulldown-cmark; parse Markdown to semantic tree | Document structure available in Rust |
| **Tue** | Implement semantic_tree.rs; map pulldown AST to our Node types | Full AST conversion |
| **Wed** | Implement serializer.rs; serialize semantic tree back to Markdown | Roundtrip support begins |
| **Wed** | Add roundtrip tests for all GFM constructs | Tests pass for tables, task lists, etc. |
| **Thu** | Implement EditorCommand types (InsertText, DeleteBackward, etc.) | Command enum defined |
| **Fri** | Implement command application with cursor position updates | Commands move cursor correctly |
| **Mon** | Implement smart Enter behavior (continue list, exit on empty) | List smarts work |
| **Mon** | Implement smart Backspace behavior (exit quote, unindent) | Backspace feels natural |
| **Tue** | Implement undo/redo with Command pattern | Undo/redo works for all ops |
| **Tue** | Add task list checkbox toggle | `- [ ]` ↔ `- [x]` works |
| **Wed** | Implement blockquote entry/exit | `>` prefix behavior correct |
| **Wed** | Implement code fence entry with language tag | ``` fenced blocks work |
| **Thu** | Implement table rendering (constrained model) | Tables render clearly |
| **Thu** | Implement image insert with relative path | Images insert correctly |
| **Fri** | **Milestone checkpoint**: All markdown constructs render; structural editing works |

### 2.4 Phase 3: File Operations (Week 5)

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Mon** | Implement Workspace::open; walkdir for file tree | Folder opens as workspace |
| **Mon** | Implement workspace file CRUD via Tauri commands | Create/rename/delete work |
| **Tue** | Implement auto-save with debounce | Auto-save triggers after debounce |
| **Tue** | Add dirty state indicator to UI | User sees unsaved changes |
| **Wed** | Implement external change detection with notify | External changes detected |
| **Wed** | Implement reload/preserve/defer UI for external changes | User can respond to changes |
| **Thu** | Implement Recent items persistence | Recent files appear in UI |
| **Thu** | Implement crash recovery snapshots | Snapshots saved periodically |
| **Fri** | **Milestone checkpoint**: File operations complete |

### 2.5 Phase 4: Display (Week 6)

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Mon** | CSS variable system for themes; implement light theme | Light theme works |
| **Mon** | Implement dark theme | Dark theme works |
| **Tue** | Implement focus mode (dim non-current paragraphs) | Focus mode reduces distraction |
| **Tue** | Implement typewriter mode (center active line) | Typewriter mode works |
| **Wed** | Implement content width setting | Width slider works |
| **Wed** | Implement font size/line height settings | Typography adjustable |
| **Thu** | Heading level visual differentiation (H1-H6 sizing) | Headings visually distinct |
| **Thu** | Link visual styling (underlines, colors) | Links appear as links |
| **Fri** | **Milestone checkpoint**: Display modes complete |

### 2.6 Phase 5: Export (Week 7)

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Mon** | Implement HTML export with pulldown-cmark HTML renderer | HTML export works |
| **Mon** | Implement standalone vs linked-assets HTML mode | Two HTML modes work |
| **Tue** | Implement PDF export using printpdf or html-to-pdf bridge | PDF export works |
| **Tue** | Implement PDF page size/margin settings | PDF settings work |
| **Wed** | Implement export dialog UI | Dialog appears and functions |
| **Wed** | Test export with complex documents | Edge cases handled |
| **Thu** | Verify export roundtrip for all constructs | Export fidelity verified |
| **Fri** | **Milestone checkpoint**: Export complete |

### 2.7 Phase 6: Polish (Week 8)

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Mon** | Implement find/replace UI and commands | Find/replace works |
| **Mon** | Implement outline/TOC panel from headings | TOC panel works |
| **Tue** | Implement settings persistence | Settings survive restart |
| **Tue** | Implement keyboard shortcuts | Shortcuts work |
| **Wed** | Implement paste handling (best-effort Markdown conversion) | Paste works predictably |
| **Wed** | Implement image picker/drag/paste | Image insertion works |
| **Thu** | Implement frontmatter support (YAML/TOML) | Frontmatter roundtrips |
| **Thu** | Refine editor performance for large documents | Large docs smooth |
| **Fri** | **Milestone checkpoint**: Polish complete |

### 2.8 Phase 7: Hardening (Week 9)

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Mon** | Write integration tests for all FR requirements | Tests provide coverage |
| **Mon** | Run roundtrip tests for all Markdown constructs | Roundtrip fidelity proven |
| **Tue** | Cross-platform testing (macOS, Windows, Linux) | All platforms work |
| **Tue** | Performance profiling and optimization | Performance targets met |
| **Wed** | Security review (file I/O, path traversal) | No security issues |
| **Wed** | Crash testing and recovery verification | Recovery works |
| **Thu** | Documentation for contributors | README updated |
| **Thu** | Final bug fixes | All known bugs fixed |
| **Fri** | **Milestone**: Production-ready release |

---

## 6. File Structure and Module Organization

### 6.1 Final File Tree

```
rustnote/
├── Cargo.lock
├── Cargo.toml                  # Workspace root
├── rust-toolchain.toml
│
├── crates/
│   └── rustnote-core/          # Core Rust library
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── error.rs
│       │   ├── core/
│       │   │   ├── mod.rs
│       │   │   ├── document.rs
│       │   │   ├── buffer.rs
│       │   │   ├── editor.rs
│       │   │   ├── selection.rs
│       │   │   ├── workspace.rs
│       │   │   ├── settings.rs
│       │   │   └── recovery.rs
│       │   ├── parser/
│       │   │   ├── mod.rs
│       │   │   ├── pulldown_adapter.rs
│       │   │   ├── semantic_tree.rs
│       │   │   └── serializer.rs
│       │   └── services/
│       │       ├── mod.rs
│       │       ├── document_service.rs
│       │       ├── editor_service.rs
│       │       ├── workspace_service.rs
│       │       ├── export_service.rs
│       │       ├── settings_service.rs
│       │       └── recovery_service.rs
│       └── tests/
│           ├── parser_roundtrip.rs
│           ├── editor_commands.rs
│           └── recovery.rs
│
├── src-tauri/                  # Tauri desktop app
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   └── commands.rs         # Tauri command glue
│   ├── tauri.conf.json
│   ├── capabilities/
│   └── icons/
│
└── src-ui/                     # Frontend web assets
    ├── index.html
    ├── main.ts
    ├── styles/
    ├── components/
    ├── services/
    ├── state/
    └── utils/
```

### 6.2 Module Responsibilities Summary

| Module | Responsibility | Public API |
|--------|----------------|------------|
| `rustnote-core/core/document` | Document, Node, NodeKind types | `Document`, `Node`, `NodeKind` |
| `rustnote-core/core/buffer` | Rope-based text storage | `Buffer::new`, `insert`, `delete`, `text`, `is_dirty` |
| `rustnote-core/core/editor` | Editor state, commands | `EditorState::new`, `apply`, `undo`, `redo` |
| `rustnote-core/core/workspace` | Workspace tree management | `Workspace::open`, `create_file`, `rename`, `delete` |
| `rustnote-core/core/settings` | User preferences | `Settings`, `Settings::default` |
| `rustnote-core/core/recovery` | Crash recovery snapshots | `RecoverySnapshot::new`, `save`, `list`, `restore` |
| `rustnote-core/parser` | Markdown parsing/serialization | `parse`, `serialize` |
| `rustnote-core/services` | Business logic layer | `DocumentService`, `EditorService`, etc. |
| `src-tauri/commands` | Tauri IPC boundary | `#[tauri::command]` functions |
| `src-ui/components/*` | UI rendering | Web components |
| `src-ui/services/tauri.ts` | Rust invocation | `invoke<T>(cmd, args)` |

---

## 7. Open Questions & Risky Areas

| # | Area | Risk | Mitigation |
|---|------|------|------------|
| 1 | **Tauri IPC latency for keystrokes** | Keystroke-level IPC may cause lag | Batch character input; prototype early to verify |
| 2 | **pulldown-cmark roundtrip fidelity** | Tables and task lists may not roundtrip cleanly | Custom serialization layer; comprehensive tests |
| 3 | **Undo/redo granularity** | Structural operations may be hard to undo cleanly | Command pattern with inverse operations; extensive testing |
| 4 | **Large document performance** | Rope operations may be slow for 5MB+ docs | Lazy parsing; viewport-based rendering |
| 5 | **Cross-platform file paths** | Windows/Linux path separators differ | Use `Path`/`PathBuf` consistently; test on all platforms |
| 6 | **PDF export fidelity** | HTML-to-PDF conversion may mangle layout | Use printpdf for native PDF generation; test complex docs |

---

## 8. Appendix

### 8.1 GFM Construct Support Matrix

| Construct | Parser Support | Serializer Support | Roundtrip Tests |
|-----------|---------------|-------------------|-----------------|
| Headings H1-H6 | ✅ | ✅ | ✅ |
| Bold | ✅ | ✅ | ✅ |
| Italic | ✅ | ✅ | ✅ |
| Strikethrough | ✅ | ✅ | ✅ |
| Inline code | ✅ | ✅ | ✅ |
| Fenced code blocks | ✅ | ✅ | ✅ |
| Unordered lists | ✅ | ✅ | ✅ |
| Ordered lists | ✅ | ✅ | ✅ |
| Task lists | ✅ | ✅ | ✅ |
| Blockquotes | ✅ | ✅ | ✅ |
| Links | ✅ | ✅ | ✅ |
| Images | ✅ | ✅ | ✅ |
| Horizontal rules | ✅ | ✅ | ✅ |
| Tables | ✅ | ✅ | ✅ |
| Soft/hard breaks | ✅ | ✅ | ✅ |
| Frontmatter | ⚠️ custom | ⚠️ custom | ⚠️ to verify |

### 8.2 Service Interface Summary

```
┌─────────────────────────────────────────────────────────────────┐
│                         Public API                               │
├─────────────────────────────────────────────────────────────────┤
│  DocumentService                                                │
│    open(path) → DocOpenResult { content, outline, cursor? }    │
│    save(path, content) → ()                                      │
│    reload(path) → content                                       │
│    get_outline(content) → Outline                               │
├─────────────────────────────────────────────────────────────────┤
│  EditorService                                                  │
│    apply_command(cmd) → CommandResult { new_content, cursor }   │
│    get_state() → EditorState                                     │
│    set_selection(selection) → ()                                │
│    undo() → CommandResult                                        │
│    redo() → CommandResult                                        │
├─────────────────────────────────────────────────────────────────┤
│  WorkspaceService                                               │
│    open_folder(root) → Workspace                                 │
│    list_files(root) → Vec<FileNode>                             │
│    create_file(parent, name) → path                             │
│    rename_file(path, new_name) → ()                             │
│    delete_file(path) → ()                                        │
├─────────────────────────────────────────────────────────────────┤
│  ExportService                                                  │
│    export_html(content, mode) → html_string                     │
│    export_pdf(content, options) → bytes                          │
├─────────────────────────────────────────────────────────────────┤
│  SettingsService                                                │
│    get_settings() → Settings                                     │
│    update_settings(settings) → ()                               │
├─────────────────────────────────────────────────────────────────┤
│  RecoveryService                                                │
│    list_snapshots() → Vec<RecoverySnapshotMeta>                │
│    restore_snapshot(id) → RecoveryData                          │
│    delete_snapshot(id) → ()                                     │
└─────────────────────────────────────────────────────────────────┘
```

---

*Plan generated: 2026-04-11*
*Specification: iteration-5/spec.md*
*Constitution: iteration-5/constitution.md*
