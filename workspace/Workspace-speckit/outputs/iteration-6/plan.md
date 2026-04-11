# RustNote Implementation Plan - Iteration 6

## 1. Technical Context

### 1.1 Technology Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| **Desktop Shell** | Tauri v2 | Cross-platform, small binary, Rust-native |
| **Frontend** | Vanilla HTML/CSS + TypeScript | Minimal DOM abstraction; predictable performance |
| **Editor Engine** | Custom Rust with `rope` crate | Cursor/selection owned by Rust; efficient text ops |
| **Markdown Parser** | `pulldown-cmark` + `comrak` | CommonMark + GFM support; syntax highlighting via `syntect` |
| **Serialization** | `serde` + `serde_json` | Familiar Rust patterns |
| **File Operations** | `notify` crate | File watching for external changes |
| **Crash Recovery** | JSON snapshots in app data directory | Simple, reliable, platform-independent |
| **Export** | `printpdf` (in Cargo.toml) | Native PDF generation via Tauri print API |

### 1.2 Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop Shell (v2)                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┐  │
│  │              UI Layer (WebView)                          │  │
│  │  EditorCanvas │ Sidebar │ Outline │ Toolbar/Dialogs     │  │
│  └─────────────────────────────────────────────────────────┘  │
│                           │ Tauri IPC                        │
├───────────────────────────┼───────────────────────────────────┤
│  ┌────────────────────────┴───────────────────────────────┐  │
│  │              Rust Core (src-tauri/src/)                 │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │ Commands (27 commands registered)                       │  │
│  │ - document: create/open/save/read/write                 │  │
│  │ - workspace: list_workspace                            │  │
│  │ - settings: read/write_settings                        │  │
│  │ - render: render_markdown, parse_markdown_ast,         │  │
│  │           render_for_editor, highlight_code_block       │  │
│  │ - export: export_to_html, export_to_pdf, get_print_html │  │
│  │ - editor: editor_apply_transform                        │  │
│  │ - image: insert_image, image_markdown_from_path         │  │
│  │ - recovery: save/list/restore/delete_snapshot           │  │
│  │ - file_watcher: watch/unwatch/poll/check_external       │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │ Services Layer                                          │  │
│  │ - DocumentService: document lifecycle, undo/redo        │  │
│  │ - EditorService: cursor/selection                       │  │
│  │ - FileWatcherService: notify crate integration          │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │ Semantic Model (semantic/)                              │  │
│  │ - SemanticDocument: parse, html, serialize, headings    │  │
│  │ - CursorMapping: position tracking                       │  │
│  │ - TransformEngine: Enter/Backspace/Tab/ShiftTab        │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │ Editor Engine (editor/)                                 │  │
│  │ - Cursor: movement rules                                 │  │
│  │ - Selection: selection state                            │  │
│  │ - Commands: insert/delete/transform operations           │  │
│  │ - Transforms: structural editing behaviors              │  │
│  │ - UndoManager: undo/redo stack                          │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │ Parser (parser/)                                        │  │
│  │ - MarkdownParser: pulldown-cmark + comrak               │  │
│  │ - SyntaxHighlighter: syntect integration                │  │
│  ├────────────────────────────────────────────────────────┤  │
│  │ Renderer (renderer/)                                    │  │
│  │ - BlockRenderer: document rendering                      │  │
│  │ - InlineRenderer: emphasis, code, link, image           │  │
│  │ - RenderState: render state management                  │  │
│  └────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### 1.3 Current Implementation State

| Component | Status | Notes |
|-----------|--------|-------|
| Document Model | ✅ Complete | Document with id, title, content, dirty state |
| Semantic Model | ✅ Complete | SemanticDocument with parse, html, serialize |
| Markdown Parser | ✅ Complete | pulldown-cmark + comrak with GFM |
| Syntax Highlighter | ✅ Complete | syntect integrated, available via command |
| Editor Engine | ✅ Complete | Cursor, selection, commands, transforms, undo/redo |
| Transform Engine | ✅ Complete | Smart Enter/Backspace/Tab wired to frontend |
| File Operations | ✅ Complete | Open/save with atomic writes |
| Workspace | ✅ Complete | Recursive file tree, filtered for .md |
| Settings | ✅ Complete | Theme, auto-save, editor settings, recent_files |
| Live Rendering | ✅ Complete | `render_for_editor` returns html, cursor_mapping, headings |
| Image Handling | ✅ Complete | Backend copy_to_workspace, get_image_info |
| Recovery System | ✅ Complete | Full snapshot lifecycle |
| File Watching | ✅ Complete | notify crate integration |
| Find/Replace | ✅ Complete | Full find/replace/replace-all |
| Focus Mode | ✅ Complete | Implemented in editor.js |
| Typewriter Mode | ✅ Complete | Implemented in editor.js |
| Outline Panel | ✅ Complete | Shows headings, click to navigate |
| HTML Export | ✅ Complete | Working with standalone mode |
| PDF Export | ⚠️ Workaround | `export_to_pdf` writes HTML; frontend uses `window.print()` |

### 1.4 Remaining Gaps (Iteration 6 Focus)

| Priority | Gap | Impact | Solution |
|----------|-----|--------|----------|
| **HIGH-1** | PDF Export | MVP blocker | Implement native Tauri PDF via `printpdf` |
| **HIGH-2** | Code Fence Highlighting | UX | Integrate syntect into editor live preview |
| **MED-1** | Link Click-to-Open | UX | Ctrl+Click to open links in browser |
| **MED-2** | Table Editing | UX | Visual table editing in editor |
| **MED-3** | Frontmatter Support | Feature | Render/preserve YAML frontmatter |
| **LOW-1** | Cross-Platform CI | DevOps | CI verification on macOS/Windows/Linux |

---

## 2. Constitution Check

### 2.1 Alignment with Core Principles

| Constitutional Principle | Alignment Strategy |
|--------------------------|-------------------|
| **Write First** | Single-pane live rendering already implemented; focus on polishing interaction quality |
| **Markdown Fidelity** | Serialization fidelity verified; roundtrip tests exist |
| **Rendered While Editing** | Live rendering integrated; focus on code fence highlighting integration |
| **Calm Interface** | Focus/typewriter modes implemented; PDF via native print reduces UI chrome |
| **Local-First** | File watching, recovery, auto-save all functional |
| **Rust-Owned Correctness** | Parser, transform engine, serialization all in Rust |
| **Interaction Quality** | Transform engine wired; focus on finishing link/table interactions |

### 2.2 Architecture Decisions

| Decision | Constitutional Basis |
|----------|---------------------|
| **PDF via printpdf crate** | Constitution: "Export behind interface boundary for future format extensibility" |
| **syntect for highlighting** | Constitution: "Rust-owned correctness" for code fence rendering |
| **Tauri v2 print API** | Cross-platform PDF without external dependencies |

---

## 3. Phase 0: Research (Iteration 6)

### 3.1 Unknowns to Resolve

| # | Unknown | Resolution Approach | Status |
|---|---------|-------------------|--------|
| R1 | **Tauri v2 Print API for PDF** | Check tauri-plugin-print documentation | Needed |
| R2 | **syntect highlighting in frontend** | Design: highlighted HTML injected vs. frontend syntax parsing | Needed |
| R3 | **Link click handling** | Check existing click handler in editor.js | Needed |
| R4 | **Table editing model** | Check if comrak/parser supports table preservation | Needed |

### 3.2 Resolution Plan

- **Day 1**: Resolve R1 (PDF API), R2 (syntax highlighting integration)
- **Day 2**: Resolve R3 (link handling), R4 (table editing)
- **Day 3-5**: Implementation of HIGH priority items

---

## 4. Phase 1: Design (Iteration 6)

### 4.1 Data Model Updates

#### 4.1.1 PDF Export Options (New)

```rust
// src-tauri/src/model/export.rs (new file)

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdfExportOptions {
    pub page_size: PdfPageSize,
    pub margins: PdfMargins,
    pub embed_images: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PdfPageSize {
    A4,
    Letter,
    Legal,
    Custom { width_mm: f32, height_mm: f32 },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PdfMargins {
    pub top_mm: f32,
    pub right_mm: f32,
    pub bottom_mm: f32,
    pub left_mm: f32,
}

impl Default for PdfMargins {
    fn default() -> Self {
        Self {
            top_mm: 20.0,
            right_mm: 20.0,
            bottom_mm: 20.0,
            left_mm: 20.0,
        }
    }
}
```

#### 4.1.2 Syntax Highlighted Code Block (Enhancement)

```rust
// src-tauri/src/parser/syntax.rs (existing, add method)

impl SyntaxHighlighter {
    /// Generate HTML with syntax highlighting for code blocks
    pub fn highlight_html(&self, code: &str, language: &str) -> String {
        use syntect::parsing::SyntaxSet;
        use syntect::html::{highlighted_html_for_string, ClassedHTMLGenerator};
        
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax = syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
        
        let mut html_generator = ClassedHTMLGenerator::new_from_syntax(syntax, &syntax_set);
        // ... implementation
    }
}
```

### 4.2 Service Contract Updates

#### 4.2.1 New Tauri Commands

```rust
// src-tauri/src/commands/export.rs (add)

#[tauri::command]
pub async fn export_to_pdf_native(
    markdown: String,
    output_path: String,
    options: PdfExportOptions,
) -> Result<(), CommandError>;

#[tauri::command]
pub fn get_highlighted_code_html(code: String, language: String) -> String;
```

#### 4.2.2 Enhanced Render Command

```rust
// src-tauri/src/commands/render.rs (enhance)

#[tauri::command]
pub fn render_for_editor_with_highlighting(
    markdown: String, 
    cursor_offset: usize,
    include_highlighting: bool,
) -> EditorRenderResult {
    // When include_highlighting=true, code fences get syntect highlighting
}
```

### 4.3 Module Organization

```
rustnote/src-tauri/src/
├── commands/
│   ├── mod.rs           # CommandError enum
│   ├── document.rs     # ✅ create/open/save/read/write_document
│   ├── workspace.rs    # ✅ list_workspace
│   ├── settings.rs     # ✅ read/write_settings
│   ├── export.rs       # [ENHANCE] export_to_pdf_native, get_highlighted_code_html
│   ├── render.rs       # [ENHANCE] render_for_editor_with_highlighting
│   ├── editor.rs       # ✅ editor_apply_transform
│   ├── image.rs         # ✅ insert_image, image_markdown_from_path
│   ├── recovery.rs      # ✅ save/list/restore/delete_snapshot
│   └── file_watcher.rs # ✅ watch/unwatch/poll/check_external
├── model/
│   ├── mod.rs
│   ├── document.rs     # ✅ Document with dirty state
│   ├── settings.rs     # ✅ Theme, EditorSettings, recent_files
│   ├── workspace.rs    # ✅ Workspace, FileEntry
│   ├── image.rs        # ✅ ImageInfo, copy_image_to_workspace
│   ├── recovery.rs     # ✅ RecoverySnapshot, RecoveryData
│   └── export.rs       # [NEW] PdfExportOptions
├── parser/
│   ├── mod.rs          # ✅ MarkdownParser, SyntaxHighlighter
│   ├── markdown.rs     # ✅ pulldown-cmark with GFM
│   └── syntax.rs       # [ENHANCE] highlight_html method
├── semantic/
│   ├── mod.rs          # ✅ SemanticDocument, TransformEngine
│   ├── ast.rs          # ✅ parse, html, serialize, get_headings
│   ├── position.rs     # ✅ Anchor, Selection, CursorMapping
│   └── transform.rs    # ✅ TransformEngine with Enter/Backspace/Tab
├── editor/
│   ├── mod.rs          # ✅ exports
│   ├── cursor.rs       # ✅ Cursor with movement
│   ├── selection.rs    # ✅ SelectionState
│   ├── commands.rs     # ✅ Command enum
│   ├── transforms.rs   # ✅ Enter/Backspace/Tab/ShiftTab logic
│   └── undo.rs         # ✅ UndoManager
├── renderer/
│   ├── mod.rs          # ✅ BlockRenderer, InlineRenderer
│   ├── blocks.rs       # ✅ document rendering
│   ├── inline.rs       # ✅ emphasis, code, link, image
│   └── state.rs        # ✅ RenderState
├── services/
│   ├── mod.rs          # ✅ DocumentService, EditorService
│   ├── document.rs     # ✅ Document lifecycle, undo/redo
│   ├── editor.rs       # ✅ cursor/selection
│   └── file_watcher.rs # ✅ FileWatcherService with notify
└── lib.rs              # ✅ 27 commands registered
```

---

## 5. Phase 2: Implementation Breakdown

### 5.1 Implementation Order

| Phase | Focus | Duration | Items |
|-------|-------|----------|-------|
| **Phase A** | PDF Export | 2 days | Native Tauri PDF via printpdf |
| **Phase B** | Syntax Highlighting | 2 days | Integrate syntect into live editor |
| **Phase C** | Link Handling | 1 day | Ctrl+Click to open links |
| **Phase D** | Table Editing | 1 day | Visual table editing improvements |
| **Phase E** | Frontmatter | 0.5 day | YAML frontmatter support |
| **Phase F** | CI Setup | 0.5 day | Cross-platform build verification |

### 5.2 Phase A: Native PDF Export (HIGH-1)

#### Day 1-2: Implement printpdf-based PDF export

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Day 1** | Research Tauri v2 print API; verify printpdf integration | Tauri print API usage documented |
| **Day 1** | Create `PdfExportOptions` struct in model/export.rs | PdfExportOptions type defined |
| **Day 1** | Implement `export_to_pdf_native` command using printpdf | Command compiles and basic PDF generates |
| **Day 2** | Add page size/margin support to PDF export | A4/Letter/custom size works |
| **Day 2** | Add image embedding support | Images appear in PDF |
| **Day 2** | Update frontend to use native PDF export | Export dialog uses new command |

**Technical Approach:**

```rust
// src-tauri/src/commands/export.rs

#[tauri::command]
pub async fn export_to_pdf_native(
    markdown: String,
    output_path: String,
    options: PdfExportOptions,
) -> Result<(), CommandError> {
    use printpdf::*;
    
    let (doc, page1, layer1) = PdfDocument::new(
        "RustNote Export",
        Mm(options.page_size.width_mm()),
        Mm(options.page_size.height_mm()),
        "Layer 1",
    );
    
    let current_layer = doc.get_page(page1).get_layer(layer1);
    
    // Parse markdown to HTML
    let html = crate::parser::MarkdownParser::new()
        .parse_to_html(&markdown);
    
    // Convert HTML to PDF using printpdf
    // ... implementation
    
    doc.save(&mut std::fs::File::create(&output_path)?)?;
    Ok(())
}
```

**Code Ownership:** Backend (Rust) - `export.rs`, `model/export.rs`

#### Verification

- [ ] A4 PDF generates correctly
- [ ] Letter PDF generates correctly
- [ ] Margins applied correctly
- [ ] Images embed in PDF
- [ ] Tables render correctly in PDF
- [ ] Code blocks render correctly in PDF

---

### 5.3 Phase B: Code Fence Syntax Highlighting (HIGH-2)

#### Day 3-4: Integrate syntect into editor live preview

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Day 3** | Add `highlight_html` method to SyntaxHighlighter | Returns highlighted HTML string |
| **Day 3** | Create enhanced `render_for_editor_with_highlighting` | Code fences get syntax highlighting |
| **Day 4** | Update frontend editor.js to request highlighted code | Code blocks show colors |
| **Day 4** | Handle fallback for unsupported languages | Plain code block for unknown langs |

**Technical Approach:**

```rust
// src-tauri/src/parser/syntax.rs

impl SyntaxHighlighter {
    /// Generate highlighted HTML for code block content
    pub fn highlight_html(&self, code: &str, language: &str) -> String {
        use syntect::html::{ClassedHTMLGenerator, prefixed_lines};
        use syntect::parsing::SyntaxSet;
        
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax = syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
        
        let mut html_generator = ClassedHTMLGenerator::new_from_syntax(syntax, &syntax_set);
        
        for line in code.lines() {
            html_generator.append_generated_content_for_line(line);
        }
        
        html_generator.finalize_string()
            .unwrap_or_else(|_| code.to_string())
    }
}
```

**Frontend Integration:**

```javascript
// www/src/scripts/editor.js (update)

async function renderDocument() {
    const includeHighlighting = true;
    const result = await invoke('render_for_editor_with_highlighting', {
        markdown: currentContent,
        cursorOffset: cursorOffset,
        includeHighlighting: includeHighlighting
    });
    
    // result.html now contains highlighted code fences
    editorElement.innerHTML = result.html;
}
```

**Code Ownership:** Backend (Rust) - `syntax.rs`, `render.rs`; Frontend (JS) - `editor.js`

#### Verification

- [ ] JavaScript code blocks highlight with colors
- [ ] Python code blocks highlight correctly
- [ ] Rust code blocks highlight correctly
- [ ] Unknown languages fall back to plain display
- [ ] Highlighted code renders in live preview

---

### 5.4 Phase C: Link Click-to-Open (MED-1)

#### Day 5: Implement Ctrl+Click link opening

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Day 5** | Add click handler for links in editor | Ctrl+Click opens link |
| **Day 5** | Detect external vs internal links | External opens in browser |
| **Day 5** | Update link rendering for clickability | Links show pointer cursor |

**Technical Approach:**

```javascript
// www/src/scripts/editor.js (add)

editorElement.addEventListener('click', async (e) => {
    if (!e.ctrlKey && !e.metaKey) return;
    
    const link = e.target.closest('a');
    if (!link) return;
    
    e.preventDefault();
    const href = link.getAttribute('href');
    
    if (href) {
        // Open in default browser
        await invoke('open_external_url', { url: href });
    }
});
```

```rust
// src-tauri/src/commands/mod.rs (add)

#[tauri::command]
pub fn open_external_url(url: String) -> Result<(), CommandError> {
    open::that(&url).map_err(|e| CommandError::Io(e))?;
    Ok(())
}
```

**Code Ownership:** Frontend (JS) - `editor.js`; Backend (Rust) - `commands/mod.rs`

#### Verification

- [ ] Ctrl+Click on link opens in default browser
- [ ] Cmd+Click works on macOS
- [ ] External links (https://) open correctly
- [ ] Relative links resolved correctly
- [ ] Default browser opens for markdown links

---

### 5.5 Phase D: Table Editing (MED-2)

#### Day 6: Visual table editing improvements

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Day 6** | Add table cell selection in editor | Click to select cell |
| **Day 6** | Add Tab navigation between cells | Tab moves to next cell |
| **Day 6** | Handle Enter in table cells | Creates line break within cell |

**Technical Approach:**

The table editing requires enhancing the editor to handle table-specific navigation. Focus on:
1. Cell-level selection (click on cell)
2. Tab navigation (Shift+Tab to go backward)
3. Content preservation on save/reload

```javascript
// www/src/scripts/editor.js (add table handling)

function handleTableCellNavigation(e) {
    if (e.key === 'Tab') {
        e.preventDefault();
        // Navigate to next/prev cell
        const currentCell = window.getSelection().anchorNode;
        if (e.shiftKey) {
            // Move to previous cell
        } else {
            // Move to next cell
        }
    }
}
```

**Code Ownership:** Frontend (JS) - `editor.js`

#### Verification

- [ ] Tables render with clear grid visual
- [ ] Click on cell places cursor in cell
- [ ] Tab moves to next cell
- [ ] Shift+Tab moves to previous cell
- [ ] Enter creates newline in cell
- [ ] Table structure preserved on save/reload

---

### 5.6 Phase E: Frontmatter Support (MED-3)

#### Day 7: YAML frontmatter rendering and preservation

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Day 7** | Update parser to detect frontmatter | Frontmatter parsed separately |
| **Day 7** | Update serializer to preserve frontmatter | Frontmatter included in save |
| **Day 7** | Update renderer to hide frontmatter in editor | Frontmatter not shown in WYSIWYM |

**Technical Approach:**

```rust
// src-tauri/src/semantic/ast.rs (enhance)

impl SemanticDocument {
    /// Parse frontmatter if present
    pub fn parse_with_frontmatter(source: &str) -> (Option<String>, &str) {
        if source.starts_with("---") {
            // Find closing ---
            // Return (frontmatter_content, remaining_markdown)
        }
        (None, source)
    }
    
    /// Serialize with frontmatter preserved
    pub fn serialize_with_frontmatter(&self, frontmatter: Option<&str>) -> String {
        let mut result = String::new();
        if let Some(fm) = frontmatter {
            result.push_str("---\n");
            result.push_str(fm);
            result.push_str("\n---\n");
        }
        result.push_str(self.serialize_to_commonmark());
        result
    }
}
```

**Code Ownership:** Backend (Rust) - `semantic/ast.rs`

#### Verification

- [ ] Document with frontmatter opens correctly
- [ ] Frontmatter visible in raw markdown but hidden in WYSIWYM
- [ ] Frontmatter preserved on save/reload
- [ ] Frontmatter does not appear in outline

---

### 5.7 Phase F: Cross-Platform CI (LOW-1)

#### Day 8: CI verification for macOS/Windows/Linux

| Day | Tasks | Deliverables |
|-----|-------|--------------|
| **Day 8** | Set up GitHub Actions workflow | CI runs on PR |
| **Day 8** | Add macOS build step | Binary builds |
| **Day 8** | Add Windows build step | Binary builds |
| **Day 8** | Add Linux build step | Binary builds |

**Technical Approach:**

```yaml
# .github/workflows/ci.yml

name: CI
on: [push, pull_request]
jobs:
  build:
    strategy:
      matrix:
        os: [macos-latest, windows-latest, ubuntu-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: cargo build --release --manifest-path rustnote/src-tauri/Cargo.toml
      - name: Run tests
        run: cargo test --manifest-path rustnote/src-tauri/Cargo.toml
```

**Code Ownership:** DevOps (YAML)

#### Verification

- [ ] CI passes on macOS
- [ ] CI passes on Windows
- [ ] CI passes on Linux
- [ ] Builds produce valid binaries

---

## 6. File Structure

### 6.1 Updated File Tree

```
rustnote/
├── Cargo.toml
├── src-tauri/
│   ├── Cargo.toml              ✅ Dependencies: tauri, pulldown-cmark, comrak, syntect, notify, printpdf
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs             ✅ Entry point
│   │   ├── lib.rs              ✅ 27+ commands registered
│   │   ├── build.rs
│   │   ├── commands/
│   │   │   ├── mod.rs          ✅ CommandError enum
│   │   │   ├── document.rs     ✅ create/open/save/read/write_document
│   │   │   ├── workspace.rs    ✅ list_workspace
│   │   │   ├── settings.rs     ✅ read/write_settings
│   │   │   ├── export.rs       [ENHANCE] export_to_pdf_native, get_highlighted_code_html
│   │   │   ├── render.rs       [ENHANCE] render_for_editor_with_highlighting
│   │   │   ├── editor.rs       ✅ editor_apply_transform
│   │   │   ├── image.rs        ✅ insert_image, image_markdown_from_path
│   │   │   ├── recovery.rs     ✅ save/list/restore/delete_snapshot
│   │   │   └── file_watcher.rs ✅ watch/unwatch/poll/check_external
│   │   ├── model/
│   │   │   ├── mod.rs
│   │   │   ├── document.rs     ✅ Document with dirty state
│   │   │   ├── settings.rs     ✅ Theme, EditorSettings, recent_files
│   │   │   ├── workspace.rs    ✅ Workspace, FileEntry
│   │   │   ├── image.rs        ✅ ImageInfo, copy_image_to_workspace
│   │   │   ├── recovery.rs     ✅ RecoverySnapshot, RecoveryData
│   │   │   └── export.rs       [NEW] PdfExportOptions
│   │   ├── parser/
│   │   │   ├── mod.rs          ✅ MarkdownParser, SyntaxHighlighter
│   │   │   ├── markdown.rs     ✅ pulldown-cmark with GFM
│   │   │   └── syntax.rs       [ENHANCE] highlight_html method
│   │   ├── semantic/
│   │   │   ├── mod.rs          ✅ SemanticDocument, TransformEngine
│   │   │   ├── ast.rs          [ENHANCE] parse_with_frontmatter, serialize_with_frontmatter
│   │   │   ├── position.rs     ✅ Anchor, Selection, CursorMapping
│   │   │   └── transform.rs    ✅ TransformEngine with Enter/Backspace/Tab
│   │   ├── editor/
│   │   │   ├── mod.rs          ✅ exports
│   │   │   ├── cursor.rs       ✅ Cursor with movement
│   │   │   ├── selection.rs    ✅ SelectionState
│   │   │   ├── commands.rs     ✅ Command enum
│   │   │   ├── transforms.rs   ✅ Enter/Backspace/Tab/ShiftTab logic
│   │   │   └── undo.rs         ✅ UndoManager
│   │   ├── renderer/
│   │   │   ├── mod.rs          ✅ BlockRenderer, InlineRenderer
│   │   │   ├── blocks.rs       ✅ document rendering
│   │   │   ├── inline.rs       ✅ emphasis, code, link, image
│   │   │   └── state.rs        ✅ RenderState
│   │   └── services/
│   │       ├── mod.rs          ✅ DocumentService, EditorService
│   │       ├── document.rs     ✅ Document lifecycle, undo/redo
│   │       ├── editor.rs       ✅ cursor/selection
│   │       └── file_watcher.rs ✅ FileWatcherService with notify
│   └── www/
│       ├── index.html          ✅ Toolbar, editor, outline panel
│       └── src/
│           ├── scripts/
│           │   ├── app.js          ✅ Document lifecycle, settings, theme, export, recovery, file watching
│           │   ├── editor.js       [ENHANCE] highlighted code, link click, table cell nav
│           │   ├── outline.js      ✅ Outline panel manager
│           │   ├── renderer.js     ✅ Markdown to HTML (fallback)
│           │   ├── search.js       ✅ Full find/replace functionality
│           │   ├── recovery.js     ✅ Recovery UI
│           │   └── settings.js     ✅ Settings panel
│           └── styles/
│               ├── main.css        ✅ Layout
│               ├── editor.css      ✅ WYSIWYM styles, focus, typewriter
│               ├── theme-light.css ✅ Light theme
│               └── theme-dark.css  ✅ Dark theme
└── .github/
    └── workflows/
        └── ci.yml             [NEW] Cross-platform CI
```

---

## 7. Open Questions & Risks

| # | Area | Risk | Mitigation |
|---|------|------|------------|
| 1 | **printpdf PDF generation** | Complex HTML-to-PDF conversion | Use printpdf for direct PDF construction; fallback to HTML-to-PDF if needed |
| 2 | **syntect performance** | Highlighting large code blocks may be slow | Cache highlighted results; highlight only visible blocks |
| 3 | **Table editing complexity** | Tables can become very complex | Start with constrained interactions; expand later |
| 4 | **Frontmatter edge cases** | YAML/TOML parsing edge cases | Use existing frontmatter crates (gray_matter) |

---

## 8. Success Criteria

### 8.1 Iteration 6 Success Metrics

| Criterion | Target | Verification |
|-----------|--------|--------------|
| **PDF Export** | Native PDF generation via Tauri | A4 PDF with images, tables, code generates correctly |
| **Syntax Highlighting** | Code fences show colors in editor | JS/Python/Rust blocks highlight correctly |
| **Link Handling** | Ctrl/Cmd+Click opens links | Links open in default browser |
| **Table Editing** | Basic cell navigation works | Tab navigates cells; Enter works in cells |
| **Frontmatter** | YAML frontmatter preserved | Document with frontmatter saves/reloads correctly |
| **CI** | Builds on all platforms | GitHub Actions passes for macOS/Windows/Linux |

### 8.2 MVP Criteria Assessment

| # | Criterion | Current | After Iteration 6 |
|---|-----------|---------|-------------------|
| 1 | Create, open, edit, save, reopen Markdown | ✅ | ✅ |
| 2 | Single-pane experience readable/stable | ✅ | ✅ |
| 3 | Headings, lists, links, images, code, tables usable | ⚠️ | ✅ |
| 4 | Focus mode and typewriter mode available | ✅ | ✅ |
| 5 | Folder-based workspace viable | ✅ | ✅ |
| 6 | Autosave and recovery work | ✅ | ✅ |
| 7 | HTML and PDF export production-usable | ⚠️ | ✅ |
| 8 | Editing invariants tested | ✅ | ✅ |
| 9 | Stable builds for all platforms | ⚠️ | ✅ |

**MVP Readiness: 9/9 after Iteration 6**

---

## 9. Appendix

### 9.1 Command Summary

| Command | Module | Status | Description |
|---------|--------|--------|-------------|
| `create_document` | document | ✅ | Create new document |
| `open_document` | document | ✅ | Open document from path |
| `save_document` | document | ✅ | Save document with atomic write |
| `read_document_content` | document | ✅ | Read document content |
| `write_document_content` | document | ✅ | Write document content |
| `list_workspace` | workspace | ✅ | List workspace files |
| `read_settings` | settings | ✅ | Read user settings |
| `write_settings` | settings | ✅ | Write user settings |
| `export_to_html` | export | ✅ | Export to HTML file |
| `export_to_pdf` | export | ⚠️ | [ENHANCE] Native PDF export |
| `export_to_pdf_native` | export | [NEW] | Printpdf-based PDF |
| `get_print_html` | export | ✅ | Get HTML for printing |
| `render_markdown` | render | ✅ | Basic markdown to HTML |
| `parse_markdown_ast` | render | ✅ | Parse to semantic AST |
| `render_for_editor` | render | ✅ | Live editor rendering |
| `render_for_editor_with_highlighting` | render | [NEW] | Live rendering with syntax highlighting |
| `highlight_code_block` | render | ✅ | Highlight code block |
| `get_highlighted_code_html` | render | [NEW] | Get highlighted code HTML |
| `editor_apply_transform` | editor | ✅ | Apply edit transform |
| `update_source` | render | ✅ | Update source with parsing |
| `serialize_markdown` | render | ✅ | Serialize to Markdown |
| `get_markdown_info` | render | ✅ | Get markdown info |
| `insert_image` | image | ✅ | Insert image |
| `image_markdown_from_path` | image | ✅ | Get image markdown |
| `save_recovery_snapshot` | recovery | ✅ | Save recovery snapshot |
| `list_recovery_snapshots` | recovery | ✅ | List snapshots |
| `restore_recovery_snapshot` | recovery | ✅ | Restore snapshot |
| `delete_recovery_snapshot` | recovery | ✅ | Delete snapshot |
| `cleanup_old_snapshots` | recovery | ✅ | Cleanup stale snapshots |
| `watch_file` | file_watcher | ✅ | Watch file for changes |
| `unwatch_file` | file_watcher | ✅ | Stop watching file |
| `poll_file_changes` | file_watcher | ✅ | Poll for changes |
| `check_external_change` | file_watcher | ✅ | Check external changes |
| `update_watched_file_state` | file_watcher | ✅ | Update watched state |
| `open_external_url` | commands | [NEW] | Open URL in browser |

### 9.2 Files to Create/Modify

| File | Action | Description |
|------|--------|-------------|
| `src-tauri/src/model/export.rs` | CREATE | PdfExportOptions type |
| `src-tauri/src/commands/export.rs` | MODIFY | Add export_to_pdf_native |
| `src-tauri/src/commands/mod.rs` | MODIFY | Add open_external_url |
| `src-tauri/src/parser/syntax.rs` | MODIFY | Add highlight_html method |
| `src-tauri/src/commands/render.rs` | MODIFY | Add render_for_editor_with_highlighting |
| `src-tauri/src/semantic/ast.rs` | MODIFY | Add frontmatter methods |
| `www/src/scripts/editor.js` | MODIFY | Add highlighting request, link click, table nav |
| `www/src/scripts/app.js` | MODIFY | Use new PDF export command |
| `.github/workflows/ci.yml` | CREATE | Cross-platform CI |

---

*Plan generated: 2026-04-11*
*Specification: iteration-6/spec.md*
*Constitution: iteration-6/constitution.md*
*Gap Analysis: iteration-6/gap-analysis.md*
*Previous Plan: iteration-5/plan.md*