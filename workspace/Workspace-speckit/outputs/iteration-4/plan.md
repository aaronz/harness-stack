# RustNote Implementation Plan — Iteration 4

**Document Version:** 1.0  
**Date:** 2026-04-11  
**Based On:** Iteration-4 Gap Analysis + Constitution v2.1  
**Target:** Complete core Typora-like WYSIWYM experience + Focus/Typewriter modes

---

## 1. Technical Context

### 1.1 Project Overview

| Field | Value |
|-------|-------|
| Project | RustNote — Typora-like Markdown Editor |
| Runtime | Tauri V1 (Desktop: macOS, Windows, Linux) |
| Language | Rust (core) + TypeScript/JavaScript (frontend) |
| Markdown Flavor | CommonMark + GFM |
| License | MIT OR Apache-2.0 |
| Workspace | `/Users/aaronzh/Documents/GitHub/harness-stack/workspace/workspace-speckit/rustnote/` |

### 1.2 Architecture Overview (Current State)

```
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend (www/)                        │
│  ┌─────────────────────┐  ┌─────────────────────────────────┐  │
│  │  source (textarea)  │  │  display (rendered HTML)         │  │
│  │  - Raw markdown     │  │  - Separate preview pane         │  │
│  │  - NOT WYSIWYM      │  │  - Not inline with editing       │  │
│  └─────────────────────┘  └─────────────────────────────────┘  │
│                              │ Tauri IPC (invoke/events)        │
└──────────────────────────────┼───────────────────────────────────┘
                               │
┌──────────────────────────────▼───────────────────────────────────┐
│                         Rust Backend (src-tauri/)              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Document    │  │ Parser      │  │ Editor Engine           │  │
│  │ Service     │  │ (comrak)   │  │ (Cursor, Selection)    │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Serializer  │  │ Workspace   │  │ Renderer                │  │
│  │             │  │ (file IO)   │  │ (blocks, inline)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.3 Target Architecture (WYSIWYM)

```
┌─────────────────────────────────────────────────────────────────┐
│                         Frontend (www/)                        │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │  editor (contenteditable) - SINGLE PANE WYSIWYM            ││
│  │  - Markdown renders inline while editing                  ││
│  │  - Cursor position maps to semantic positions             ││
│  │  - Keyboard shortcuts trigger Rust transforms             ││
│  └─────────────────────────────────────────────────────────────┘│
│                              │ Tauri IPC (invoke/events)        │
└──────────────────────────────┼───────────────────────────────────┘
                               │
┌──────────────────────────────▼───────────────────────────────────┐
│                         Rust Backend (src-tauri/)              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Document    │  │ Parser      │  │ Editor Engine           │  │
│  │ Service     │  │ (comrak)   │  │ (Cursor, Selection)    │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ Serializer  │  │ Workspace   │  │ Renderer                │  │
│  │             │  │ (file IO)   │  │ (blocks, inline)       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.4 Module Organization (Current State)

```
rustnote/
├── src-tauri/
│   ├── Cargo.toml              ✅ comrak, syntect, pulldown-cmark
│   └── src/
│       ├── main.rs            ✅ Entry point
│       ├── lib.rs             ✅ Command registration
│       ├── commands/          ✅ All commands implemented
│       │   ├── document.rs   ✅ create/open/save/read/write
│       │   ├── workspace.rs  ✅ list_workspace
│       │   ├── settings.rs   ✅ read/write_settings
│       │   ├── export.rs     ✅ export_to_html/pdf
│       │   ├── render.rs     ✅ render_markdown, apply_transform
│       │   └── editor.rs     ✅ apply_transform
│       ├── model/            ✅ Complete
│       │   ├── document.rs  ✅ Document with dirty state
│       │   ├── settings.rs   ✅ Theme, EditorSettings
│       │   └── workspace.rs  ✅ Workspace, FileEntry
│       ├── parser/           ✅ Complete
│       │   ├── markdown.rs   ✅ pulldown-cmark + comrak GFM
│       │   └── syntax.rs    ✅ syntect integration
│       ├── semantic/         ✅ Complete (NEW)
│       │   ├── ast.rs       ✅ SemanticDocument
│       │   ├── position.rs  ✅ Position, Selection
│       │   └── transform.rs ✅ TransformEngine
│       ├── editor/           ✅ Complete (NEW)
│       │   ├── cursor.rs    ✅ Cursor movement
│       │   ├── selection.rs ✅ Selection handling
│       │   ├── commands.rs  ✅ Command types
│       │   ├── transforms.rs✅ Enter/Backspace/Tab logic
│       │   └── undo.rs     ✅ UndoManager
│       ├── renderer/        ✅ Complete (NEW)
│       │   ├── blocks.rs    ✅ BlockRenderer
│       │   ├── inline.rs    ✅ InlineRenderer
│       │   └── state.rs     ✅ RenderState
│       └── services/        ✅ Complete (NEW)
│           ├── document.rs  ✅ DocumentService
│           └── editor.rs    ✅ EditorService
└── www/
    └── src/
        ├── scripts/
        │   ├── app.js       ⚠️  Basic app lifecycle
        │   ├── editor.js    ⚠️  Split view, NOT WYSIWYM
        │   ├── renderer.js  ⚠️  Basic renderer
        │   ├── search.js    ⚠️  Find only
        │   └── settings.js  ⚠️  Basic settings
        └── styles/
            ├── main.css
            ├── theme-light.css
            └── theme-dark.css
```

### 1.5 Technical Decisions (RESOLVED - From Previous Iterations)

| Decision | Choice | Rationale | Status |
|----------|--------|-----------|--------|
| Desktop Shell | Tauri V1 | Fast cross-platform, Rust backend | ✅ Committed |
| Markdown Flavor | CommonMark + GFM | GitHub standard, widely compatible | ✅ Committed |
| Backend Parser | pulldown-cmark + comrak | Full AST model, GFM support | ✅ Complete |
| Syntax Highlighting | syntect | Native Rust, fast, themeable | ✅ Integrated |
| Frontend Editor | contenteditable | WYSIWYM requires custom rendering | ⚠️ NEEDS WORK |
| IPC Pattern | invoke + events | Standard Tauri pattern | ✅ Established |

### 1.6 Unknowns (NEEDS CLARIFICATION)

| Unknown | Status | Resolution Plan |
|---------|--------|-----------------|
| **Frontend rendering strategy** | NEEDS DECISION | Use contenteditable with decorated spans approach |
| **Cursor position mapping** | Backend ready | Wire frontend to Rust position system |
| **Transform rule implementation** | Backend ready | Frontend keyboard events → Rust invoke |
| **Undo/redo granularity** | Backend ready | Frontend calls Rust undo/redo commands |
| **WYSIWYM architecture** | NEEDS IMPLEMENTATION | Single contenteditable with inline decorations |

---

## 2. Constitution Check

### 2.1 Principles Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| **P1: Write First** | ⚠️ PARTIAL | Backend ready, frontend NOT WYSIWYM |
| **P2: Markdown as Source of Truth** | ✅ SUPPORTED | Semantic model maintains source fidelity |
| **P3: Live Rendering Must Feel Natural** | ❌ NOT MET | Split view, not inline rendering |
| **P4: Performance Is a Feature** | ✅ SUPPORTED | Rust backend, incremental design |
| **P5: Local-First Privacy** | ✅ SUPPORTED | No cloud in MVP scope |
| **P6: Beautiful Typography** | ⚠️ PARTIAL | Theme system exists, not integrated |
| **P7: Invisible UI** | ⚠️ PARTIAL | Basic UI, focus mode missing |
| **P8: Rust Owns Correctness** | ✅ SUPPORTED | All logic in Rust modules |
| **P9: Tauri + Rust Stack** | ✅ COMMITTED | No change |
| **P10: Document Model Separation** | ✅ ENFORCED | Source → AST → Editing → Presentation |
| **P11: CommonMark + GFM** | ✅ COMMITTED | pulldown-cmark + comrak |
| **P12: Modular Crate Architecture** | ✅ ENFORCED | semantic/, editor/, renderer/ |
| **P13: Single-Pane Invariant** | ❌ NOT MET | Split view (source/display) |
| **P14: Readability Invariant** | ❌ NOT MET | Cannot read while editing |
| **P15: Structural Editing Invariant** | ⚠️ PARTIAL | Backend ready, not wired |
| **P16: Keyboard-First** | ✅ SUPPORTED | All shortcuts exist |
| **P17: File Operations First-Class** | ✅ SUPPORTED | Workspace, recovery partial |
| **P18: Crash Recovery** | ❌ NOT IMPLEMENTED | Snapshot system missing |
| **P19: Atomic Saves** | ✅ SUPPORTED | Temp file + rename |
| **P20: Safe Rendering** | ✅ SUPPORTED | Sanitization in renderer |
| **P21: MVP Feature Focus** | ⚠️ ENFORCING | Focus on frontend integration |
| **P22: Test-Driven Development** | ✅ ENFORCED | Backend tests exist |
| **P23: ADR for Architecture** | ✅ IN PROGRESS | This plan serves as ADR |

### 2.2 Experience Invariants (Non-Negotiable)

| Invariant | Current Status | Target for Iteration 4 |
|-----------|----------------|----------------------|
| **Single-Pane** | ❌ Split view | ✅ Single contenteditable |
| **Readability** | ❌ Raw textarea | ✅ WYSIWYM inline rendering |
| **Cursor** | ⚠️ Raw textarea cursor | ✅ Semantic position tracking |
| **Structure** | ⚠️ Backend ready | ✅ Wired to keyboard |
| **Fidelity** | ✅ Roundtrip tests | ✅ Maintained |
| **Calmness** | ⚠️ Basic UI | ✅ Focus/typewriter modes |
| **Local-Trust** | ⚠️ Auto-save exists | ⚠️ Recovery still missing |

### 2.3 Gate Evaluation

| Gate | Current Status | Target |
|------|----------------|--------|
| No mode-switching required | ❌ Must switch to preview | ✅ Single pane |
| Document always serializable | ✅ Backend ready | ✅ Verified |
| Startup < 2s | ⚠️ Need benchmark | TBD after integration |
| Edit invariants testable | ✅ Backend tests exist | ✅ Expand fixtures |
| Crash recovery implemented | ❌ Not started | ⚠️ Deferred to iteration 5 |

---

## 3. Phase 0: Research

### 3.1 Open Questions (From Iteration 2)

| Question | Resolution | Evidence |
|----------|------------|----------|
| **Q1: DOM rendering strategy?** | **RESOLVED**: contenteditable with decorated spans | Typora, StackEdit patterns |
| **Q2: Cursor position mapping?** | **RESOLVED**: Backend SemanticDocument provides positions | Backend ready |
| **Q3: Transform rule implementation?** | **RESOLVED**: Rust TransformEngine via invoke | Backend ready |
| **Q4: Undo/redo granularity?** | **RESOLVED**: Command-based via UndoManager | Backend ready |
| **Q5: Incremental parsing approach?** | **RESOLVED**: Full re-parse with debounce | comrak is fast enough |

### 3.2 Research Tasks (Iteration 4 Specific)

| Task | Goal | Approach |
|------|------|----------|
| **T1: WYSIWYM Architecture** | Understand how Typora achieves inline rendering | Analyze StackEdit, Mark Text open source |
| **T2: Contenteditable Patterns** | Learn decorated span approach | MDN, ProseMirror, CodeMirror examples |
| **T3: Cursor Mapping in WYSIWYM** | Understand position synchronization | ProseMirror selection mapping |

### 3.3 Technical Context Required

Based on gap analysis, the following need clarification:

1. **WYSIWYM Editor Architecture**:
   - Single contenteditable div (not textarea + preview split)
   - Markdown syntax decorated with hidden characters or spans
   - Rendered output overlaid or inline
   - Cursor position tracked in source coordinates

2. **Frontend-Backend Integration**:
   - `update_source` invoked on every keystroke (debounced)
   - Backend returns rendered HTML + cursor position mapping
   - Frontend applies decorations without losing cursor

3. **Focus Mode Implementation**:
   - Track active paragraph (cursor position)
   - Apply opacity CSS to non-active blocks
   - Update on cursor move

4. **Typewriter Mode Implementation**:
   - On cursor move, scroll active line to vertical center
   - Smooth scroll on typing
   - Disable when user scrolls manually

---

## 4. Phase 1: Design

### 4.1 Data Model

The data model from iteration-2 remains valid. Key types:

```rust
// src-tauri/src/semantic/position.rs (exists)
Position { offset, line, column }
Selection { anchor: Anchor, head: Position }
Anchor { Left(Position), Right(Position) }

// src-tauri/src/semantic/ast.rs (exists)
SemanticDocument {
    parse(source: &str) -> Self,
    html(&self) -> String,
    serialize_to_commonmark(&self) -> String,
    get_headings() -> Vec<Heading>,
    get_list_items() -> Vec<ListItem>,
}

// src-tauri/src/editor/transforms.rs (exists)
Transform::Enter | Backspace | Tab | ShiftTab
TransformEngine::apply(transform, content, cursor_offset) -> TransformResult
```

### 4.2 Frontend Data Model (NEW for Iteration 4)

```javascript
// editor.js - New WYSIWYM state
const editor = {
    // DOM elements
    container: null,        // Single contenteditable container
    source: null,          // Hidden textarea for editing
    decorationLayer: null, // Overlay for rendered styling
    
    // State
    content: '',           // Current source markdown
    semanticDoc: null,      // Parsed semantic document from backend
    cursorOffset: 0,       // Cursor in source coordinates
    selectionRange: null,   // Selection in source coordinates
    
    // Modes
    focusMode: false,
    typewriterMode: false,
    activeParagraph: null, // For focus mode
    
    // Rendering
    isRendering: false,
    renderDebounce: null,
};
```

### 4.3 Interface Contracts

#### 4.3.1 Document Service Interface (EXISTING - No Change)

```rust
// src-tauri/src/services/document.rs
pub trait DocumentService {
    async fn create() -> Result<Document>;
    async fn open(path: PathBuf) -> Result<Document>;
    async fn save(id: DocumentId, path: Option<PathBuf>) -> Result<()>;
    async fn get_content(id: DocumentId) -> Result<String>;
    async fn apply(id: DocumentId, cmd: Command) -> Result<Document>;
    async fn update_source(id: DocumentId, source: String) -> Result<()>;
}
```

#### 4.3.2 Render Service Interface (EXISTING - Add New Methods)

```rust
// src-tauri/src/commands/render.rs - Additions needed
pub trait RenderService {
    // Existing
    async fn render_markdown(markdown: String) -> Result<String>;
    async fn parse_markdown_ast(markdown: String) -> Result<AstInfo>;
    
    // NEW for WYSIWYM
    async fn render_for_editor(markdown: String, cursor_offset: usize) -> Result<EditorRenderResult>;
}

#[derive(Serialize)]
pub struct EditorRenderResult {
    pub html: String,                    // Rendered HTML
    pub cursor_mapping: Vec<CursorMapping>, // Source offset → DOM position
    pub active_paragraph: usize,         // Paragraph index at cursor
    pub headings: Vec<HeadingInfo>,      // For TOC
}

#[derive(Serialize)]
pub struct CursorMapping {
    pub source_offset: usize,
    pub dom_offset: usize,   // Position in decorated DOM
    pub line: u32,
    pub column: u32,
}
```

#### 4.3.3 Transform Service Interface (EXISTING - No Change)

```rust
// src-tauri/src/commands/editor.rs
pub trait TransformService {
    async fn apply_transform(
        transform: Transform,
        content: String,
        cursor_offset: usize
    ) -> Result<TransformResult>;
}

#[derive(Deserialize)]
pub enum Transform {
    Enter,
    Backspace,
    Tab,
    ShiftTab,
}

#[derive(Serialize)]
pub struct TransformResult {
    pub content: String,
    pub cursor_offset: usize,
    pub success: bool,
}
```

### 4.4 Quickstart

#### 4.4.1 Development Setup (Unchanged)

```bash
# Build the project
cd rustnote/src-tauri
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

#### 4.4.2 Frontend Development

```bash
# Serve locally (for Tauri, use cargo tauri dev)
cd rustnote
cargo tauri dev
```

#### 4.4.3 Key Dependencies

```toml
# src-tauri/Cargo.toml - UNCHANGED
[dependencies]
pulldown-cmark = "0.12"
comrak = "0.52"
syntect = "5"
notify = "6.1"  # For file watching (future)
serde = { version = "1.0" }
serde_json = "1.0"
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
tempfile = "3.8"
directories = "5.0"
parking_lot = "0.12"
log = "0.4"
env_logger = "0.11"
tauri = { version = "2", features = [] }
tauri-plugin-log = "2"
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-shell = "2"
```

---

## 5. Phase 2: Implementation Breakdown

### 5.1 Iteration 4 Focus Areas

Based on gap analysis, iteration 4 targets:

1. **P0: Frontend WYSIWYM Integration** (Critical path)
2. **P0: Focus Mode Implementation**
3. **P0: Typewriter Mode Implementation**
4. **P1: Outline/TOC Panel**

### 5.2 Task Breakdown

#### P0: Frontend WYSIWYM Integration

| Task | File | Description | Effort | Dependencies |
|------|------|-------------|--------|--------------|
| **F1: Architecture Redesign** | www/src/scripts/editor.js | Replace split view with single contenteditable | 16h | None |
| **F2: Decoration System** | www/src/scripts/editor.js | Add span-based markdown decorations | 12h | F1 |
| **F3: Cursor Position Mapping** | www/src/scripts/editor.js + Rust | Wire backend cursor mapping to frontend | 12h | F1, R1 |
| **F4: Inline Rendering** | www/src/scripts/editor.js | Render markdown inline in contenteditable | 16h | F2, F3 |
| **F5: Keyboard Shortcuts** | www/src/scripts/editor.js | Connect all shortcuts to Rust transforms | 8h | F3 |
| **F6: Undo/Redo Integration** | www/src/scripts/editor.js | Wire undo/redo to Rust UndoManager | 4h | F1 |
| **F7: Live Sync** | www/src/scripts/editor.js | Sync on keystroke with debounce | 4h | F1 |
| **F8: Code Block Highlighting** | www/src/scripts/editor.js + Rust | Integrate syntect highlighting in editor | 8h | F4 |

**Frontend Subtotal: 80h**

#### R1: Backend WYSIWYM Support (Rust Additions)

| Task | File | Description | Effort | Dependencies |
|------|------|-------------|--------|--------------|
| **R1: Editor Render Result** | commands/render.rs | Add render_for_editor returning cursor mapping | 8h | None |
| **R2: Cursor Mapping** | semantic/position.rs | Add mapping from source offset to DOM position | 8h | None |
| **R3: Active Paragraph** | semantic/ast.rs | Add method to get paragraph at offset | 4h | None |
| **R4: Heading TOC** | semantic/ast.rs | Add get_headings_with_positions | 4h | None |

**Backend Additions Subtotal: 24h**

#### P1: Focus Mode

| Task | File | Description | Effort | Dependencies |
|------|------|-------------|--------|--------------|
| **FM1: Focus Mode State** | www/src/scripts/editor.js | Add focusMode state and toggle | 2h | F1 |
| **FM2: Dim Non-Active** | www/src/styles/editor.css | CSS for dimmed paragraphs | 4h | FM1 |
| **FM3: Active Paragraph Tracking** | www/src/scripts/editor.js | Update on cursor move | 4h | FM1, F3 |
| **FM4: Toggle UI** | www/src/scripts/app.js | Menu/keyboard shortcut | 2h | FM1 |

**Focus Mode Subtotal: 12h**

#### P1: Typewriter Mode

| Task | File | Description | Effort | Dependencies |
|------|------|-------------|--------|--------------|
| **TM1: Typewriter Mode State** | www/src/scripts/editor.js | Add typewriterMode state | 2h | F1 |
| **TM2: Scroll to Center** | www/src/scripts/editor.js | Scroll active line to center on move | 6h | TM1 |
| **TM3: Smooth Scroll** | www/src/styles/editor.css | CSS scroll-behavior | 2h | TM1 |
| **TM4: Toggle UI** | www/src/scripts/app.js | Menu/keyboard shortcut | 2h | TM1 |

**Typewriter Mode Subtotal: 12h**

#### P1: Outline/TOC Panel

| Task | File | Description | Effort | Dependencies |
|------|------|-------------|--------|--------------|
| **TOC1: TOC Extraction** | www/src/scripts/outline.js | Get headings from backend | 4h | R4 |
| **TOC2: TOC Panel UI** | www/src/scripts/outline.js | Render heading list | 8h | TOC1 |
| **TOC3: Click Navigation** | www/src/scripts/outline.js | Scroll to heading on click | 4h | TOC2 |
| **TOC4: Panel Toggle** | www/src/scripts/app.js | Show/hide TOC panel | 2h | TOC1 |

**TOC Panel Subtotal: 18h**

### 5.3 Summary

| Category | Tasks | Effort |
|----------|-------|--------|
| Frontend WYSIWYM | F1-F8 | 80h |
| Backend Additions | R1-R4 | 24h |
| Focus Mode | FM1-FM4 | 12h |
| Typewriter Mode | TM1-TM4 | 12h |
| Outline/TOC | TOC1-TOC4 | 18h |
| **Total** | **24 tasks** | **146h** |

### 5.4 Task Dependencies

```
F1 (Architecture Redesign)
├── F2 (Decoration System)
│   └── F4 (Inline Rendering)
│       └── F8 (Code Highlighting)
├── F3 (Cursor Mapping) ← R1, R2, R3
│   └── F5 (Keyboard Shortcuts)
│       └── F6 (Undo/Redo)
├── F7 (Live Sync)
│
├── FM1-FM4 (Focus Mode) ← F1
├── TM1-TM4 (Typewriter Mode) ← F1
└── TOC1-TOC4 (Outline) ← R4
```

### 5.5 File Structure (Updated)

```
rustnote/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── commands/
│       │   ├── mod.rs           # Add render_for_editor command
│       │   ├── document.rs      # (unchanged)
│       │   ├── workspace.rs     # (unchanged)
│       │   ├── settings.rs      # (unchanged)
│       │   ├── export.rs        # (unchanged)
│       │   ├── render.rs        # ADD: render_for_editor, cursor_mapping
│       │   └── editor.rs       # (unchanged)
│       ├── model/               # (unchanged)
│       │   ├── mod.rs
│       │   ├── document.rs
│       │   ├── settings.rs
│       │   └── workspace.rs
│       ├── parser/              # (unchanged)
│       │   ├── mod.rs
│       │   ├── markdown.rs
│       │   └── syntax.rs
│       ├── semantic/            # ADD: cursor mapping methods
│       │   ├── mod.rs
│       │   ├── ast.rs          # ADD: get_paragraph_at, get_headings_with_positions
│       │   ├── position.rs     # ADD: CursorMapping types
│       │   └── transform.rs
│       ├── editor/              # (unchanged - complete)
│       │   ├── mod.rs
│       │   ├── cursor.rs
│       │   ├── selection.rs
│       │   ├── commands.rs
│       │   ├── transforms.rs
│       │   └── undo.rs
│       ├── renderer/            # (unchanged - complete)
│       │   ├── mod.rs
│       │   ├── blocks.rs
│       │   ├── inline.rs
│       │   └── state.rs
│       └── services/            # (unchanged - complete)
│           ├── mod.rs
│           ├── document.rs
│           └── editor.rs
└── www/
    ├── index.html              # UPDATE: single editor container
    └── src/
        ├── scripts/
        │   ├── app.js          # UPDATE: mode toggles, TOC panel
        │   ├── editor.js       # REWRITE: WYSIWYM architecture
        │   ├── renderer.js     # (may be merged into editor.js)
        │   ├── search.js       # (unchanged - basic)
        │   ├── settings.js     # (unchanged - basic)
        │   └── outline.js      # NEW: TOC panel logic
        └── styles/
            ├── main.css
            ├── theme-light.css
            ├── theme-dark.css
            └── editor.css      # UPDATE: focus/typewriter styles
```

---

## 6. Testing Strategy

### 6.1 Test Fixtures (Existing - Maintain)

```
rustnote/tests/
├── markdown/                    # Parse tests (existing)
│   ├── headings.md
│   ├── emphasis.md
│   ├── lists.md
│   ├── task-lists.md
│   └── tables.md
├── editor/                     # Behavior tests (existing)
│   ├── list-behaviors.md
│   ├── cursor-across-inline.md
│   └── selection-across-blocks.md
└── roundtrip/                  # Fidelity tests (existing)
    └── *.md
```

### 6.2 New Tests for Iteration 4

| Test Category | Target | Method |
|---------------|--------|--------|
| WYSIWYM cursor mapping | Source offset ↔ DOM position | Automated position tracking |
| Focus mode behavior | Dimming applies to correct paragraphs | Visual/automated |
| Typewriter centering | Active line at center after move | Automated scroll position check |
| TOC navigation | Click heading → correct scroll position | Automated |
| Keyboard shortcuts | All shortcuts work in WYSIWYM | Integration tests |

### 6.3 Performance Benchmarks

| Criterion | Target | Method |
|-----------|--------|--------|
| Typing latency | < 16ms (60fps) | Measure keystroke to render |
| Cursor move latency | < 50ms | Measure arrow key to visual update |
| Mode toggle | < 100ms | Focus/typewriter toggle |
| TOC generation | < 100ms for 500 headings | Benchmark |

---

## 7. Acceptance Criteria

### 7.1 Iteration 4 Acceptance

| Criterion | Method | Target |
|-----------|--------|--------|
| **WYSIWYM Editor** | Visual inspection | Single pane, markdown renders inline |
| **Cursor Mapping** | Manual test | Arrow keys navigate correctly |
| **Smart Transforms** | Test lists/quotes | Enter/Backspace/Tab work correctly |
| **Focus Mode** | Visual inspection | Non-active paragraphs dim |
| **Typewriter Mode** | Visual inspection | Active line stays centered |
| **Outline/TOC** | Manual test | Headings listed, click navigates |
| **Theme Support** | Toggle test | Light/dark work correctly |
| **Undo/Redo** | Manual test | Undo/redo works for all operations |

### 7.2 Experience Acceptance

| Invariant | Verification |
|-----------|---------------|
| Single-Pane | No separate preview pane visible |
| Readability | Headings look like headings while editing |
| Cursor | Arrow keys move predictably around formatted text |
| Structure | Enter on empty list exits list |
| Fidelity | Edit-save-reopen identical |

### 7.3 Architecture Acceptance

| Criterion | Verification |
|-----------|--------------|
| Rust owns correctness | All transforms, parsing, serialization in Rust |
| Source of truth | Plain markdown always available |
| Service boundaries | UI dispatches commands, renders derived state |

---

## 8. Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| **WYSIWYM cursor mapping complexity** | HIGH | HIGH | Start with simple model, iterate |
| **Contenteditable edge cases** | HIGH | MEDIUM | Use proven patterns from StackEdit/Mark Text |
| **Performance on large docs** | MEDIUM | HIGH | Debounce, virtual rendering if needed |
| **Focus mode flicker** | MEDIUM | LOW | CSS transitions, efficient updates |
| **Typewriter scroll jank** | MEDIUM | MEDIUM | Smooth scroll API, RAF for updates |

---

## 9. Out of Scope for Iteration 4

The following are explicitly deferred to iteration 5:

- Crash recovery (snapshots + restore)
- File watching (external change detection)
- Image paste/insert with preview
- Table cell navigation
- Find/Replace in WYSIWYM context
- Real PDF export (print-to-PDF)

---

## 10. Next Steps

### Immediate (Iteration 4 Start)

1. **Begin F1: Architecture Redesign**
   - Replace textarea/div split with single contenteditable
   - Design decoration layer approach

2. **Begin R1: Backend Editor Render**
   - Add `render_for_editor` returning cursor mapping
   - Add `CursorMapping` types

3. **Set up iteration 4 branch**
   - Create branch from current state
   - Begin incremental implementation

### Iteration 4 End State

- ✅ Single-pane WYSIWYM editor with inline rendering
- ✅ Cursor position tracking via Rust backend
- ✅ Smart transforms (Enter/Backspace/Tab) wired
- ✅ Focus mode (dim non-active paragraphs)
- ✅ Typewriter mode (centered active line)
- ✅ Outline/TOC panel with click navigation
- ✅ Code block syntax highlighting

### Iteration 5 Scope

- Crash recovery system
- File watching
- Image support
- Find/Replace
- Real PDF export

---

## Appendix A: References

| Reference | Path |
|-----------|------|
| Constitution | `outputs/iteration-4/constitution.md` |
| Feature Spec | `outputs/iteration-4/spec.md` |
| Gap Analysis | `outputs/iteration-4/gap-analysis.md` |
| Previous Plan | `outputs/iteration-2/plan.md` |

## Appendix B: Key Dependencies (Frontend)

```javascript
// No new npm dependencies needed
// Using vanilla JS with Tauri IPC
// Existing: Tauri v2 API via window.__TAURI__.core.invoke
```

## Appendix C: Backend Changes Summary

### Additions Required

1. **commands/render.rs**:
   - `render_for_editor(markdown, cursor_offset)` → `EditorRenderResult`

2. **semantic/position.rs**:
   - `CursorMapping` struct
   - Methods for offset ↔ DOM position mapping

3. **semantic/ast.rs**:
   - `get_paragraph_at(offset)` → paragraph index
   - `get_headings_with_positions()` → for TOC

---

**Plan Created:** 2026-04-11  
**Plan Version:** 1.0  
**Target:** Iteration 4 - Core WYSIWYM Experience  
**Effort Estimate:** 146 hours (24 tasks)