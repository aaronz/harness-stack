# RustNote — Feature Specification

## 1. Short Name

**RustNote** — A Typora-like Markdown Editor

---

## 2. Overview

RustNote is a local-first, writing-first Markdown editor built in Rust. It provides a single-pane, WYSIWYM (What You See Is What You Mean) editing experience where users write in a visually formatted document while plain Markdown remains the durable source of truth. Target runtime is desktop via Tauri (V1), supporting macOS, Windows, and Linux.

---

## 3. Key Concepts

### 3.1 Actors

| Actor | Goals |
|-------|-------|
| **Writers / Note-takers** | Clean writing space with Markdown portability |
| **Developers / Technical authors** | README/docs/spec editing with Git-friendly output, code-block correctness |
| **Product managers / Researchers** | Readable drafting with headings, lists, tables, images, and export |

### 3.2 Core Actions

- **Document lifecycle**: Create, open, edit, save, auto-save, recover `.md` files
- **Workspace management**: Open folder as workspace, navigate file tree
- **Live editing**: Single-pane Markdown editing with inline rendering
- **Structural editing**: Smart Enter/Backspace/Tab for lists, quotes, headings, tables
- **Content insertion**: Links, images, code fences, tables, task lists
- **Navigation**: In-document find/replace, outline/TOC panel
- **Export**: HTML and PDF export
- **Display modes**: Themes (light/dark), focus mode, typewriter mode
- **Preferences**: Persist user settings

### 3.3 Data

| Data Type | Description |
|-----------|-------------|
| **Markdown files** | `.md` source files on local disk |
| **Recovery snapshots** | Autosaved state for crash recovery |
| **Settings** | Persisted preferences (theme, autosave interval, editor width, etc.) |
| **Recent items** | Recently opened files and folders |
| **Workspace metadata** | File tree state, active file |

### 3.4 Constraints

- **Local-first**: No network dependency in MVP editing path
- **Single-pane invariant**: Split preview is not the core workflow
- **Markdown fidelity**: Supported constructs must survive edit-save-reopen cycles
- **Rust ownership**: Parsing, editing semantics, serialization, recovery, export-critical logic reside in Rust
- **Architecture rule**: UI renders derived state; it is not the source of truth

---

## 4. User Scenarios & Testing

### 4.1 Scenario: Create and Edit a Document

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Launch the app | Empty untitled document appears |
| 2 | Type `# My Document` | H1 heading renders visually |
| 3 | Press Enter, type `First paragraph with **bold** text` | Text renders with inline bold |
| 4 | Press Enter twice, type `- Item 1`, Enter, `- Item 2` | Unordered list renders with proper indentation |
| 5 | Press Ctrl/Cmd+S | Save dialog opens; user chooses location |
| 6 | File is saved | Dirty state clears; status confirms saved |

**Testable outcome**: File saved can be reopened and content matches exactly.

### 4.2 Scenario: Open Existing File from Disk

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Press Ctrl/Cmd+O or drag file onto window | File opens in editor |
| 2 | Document renders | Headings, lists, code blocks render inline |
| 3 | User edits content | Changes reflect immediately in rendered view |
| 4 | User saves (Ctrl/Cmd+S) | File persists with valid Markdown |

**Testable outcome**: All supported Markdown constructs survive open→edit→save→reopen cycle.

### 4.3 Scenario: Workspace Navigation

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Open folder as workspace | Sidebar shows file tree |
| 2 | Click `.md` file in tree | File opens in editor pane |
| 3 | Right-click to create/rename/delete | Appropriate file operation executes |
| 4 | External tool modifies a file | Tree refreshes; if file is open, prompt to reload |

**Testable outcome**: File tree reflects actual filesystem state within 2 seconds of external change.

### 4.4 Scenario: Structured List Editing

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Start a list with `- ` | Unordered list item created |
| 2 | Press Enter on non-empty item | New list item created |
| 3 | Press Enter on empty item (`- `) | List exits; new paragraph created |
| 4 | Press Backspace at start of list item | List item removed; cursor joins previous item or exits list |
| 5 | Press Tab on list item | Item indents (nested list) |
| 6 | Press Shift+Tab on nested item | Item outdents |

**Testable outcome**: List structure is valid Markdown after each operation.

### 4.5 Scenario: Task List Interaction

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Type `- [ ] Task item` | Task list checkbox renders |
| 2 | Click checkbox | Checkbox toggles to `[x]` or `[ ]` |
| 3 | Save and reopen file | Task state persists correctly |

**Testable outcome**: Checkbox state survives edit-save-reopen cycle.

### 4.6 Scenario: Insert and Render Image

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Type `![alt text](relative/path.png)` | Image renders inline with alt text |
| 2 | Insert image via menu/dialog | Relative path is inserted correctly |
| 3 | Open file on different machine | Relative path resolves correctly |

**Testable outcome**: Image paths are preserved as entered; broken images show error state.

### 4.7 Scenario: Focus Mode

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Toggle focus mode (View menu or shortcut) | Non-active paragraphs/sections dim |
| 2 | Move cursor to different section | Previously active section dims; new section becomes active |
| 3 | Continue typing | Active section remains fully visible |

**Testable outcome**: Only the paragraph containing cursor is fully emphasized; others are de-emphasized.

### 4.8 Scenario: Typewriter Mode

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Toggle typewriter mode | Active line/paragraph stays vertically centered |
| 2 | Scroll or type | View adjusts to keep active line centered |
| 3 | Navigate with arrow keys or mouse | Centering follows cursor movement |

**Testable outcome**: Active line remains within center 40% of viewport height during typing.

### 4.9 Scenario: Find and Replace

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Press Ctrl/Cmd+F | Find panel opens |
| 2 | Type search term | Matches highlight in document |
| 3 | Press Enter or click Next | Cursor jumps to next match |
| 4 | Optionally type replacement | Replacement string shown |
| 5 | Replace one or all | Document updates; matches replaced |

**Testable outcome**: All replacements preserve valid Markdown structure.

### 4.10 Scenario: Export to HTML

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Press Ctrl/Cmd+Shift+E or File → Export HTML | Export dialog appears |
| 2 | Choose standalone or linked-assets mode | Export processes document |
| 3 | Choose destination | HTML file written |
| 4 | Open exported HTML in browser | Headings, lists, code blocks, tables, images render correctly |

**Testable outcome**: Exported HTML preserves document structure and styling.

### 4.11 Scenario: Export to PDF

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | File → Export PDF | Export dialog appears |
| 2 | Choose page size, margins | Export processes document |
| 3 | Choose destination | PDF file written |
| 4 | Open PDF | Document renders with correct pagination and structure |

**Testable outcome**: PDF matches expected structure and typography.

### 4.12 Scenario: Crash Recovery

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Edit a document (unsaved changes) | Autosave snapshot created per configured interval |
| 2 | Force-close the app or crash | On relaunch, recovery prompt appears |
| 3 | Choose to restore | Unsaved content is recovered into new buffer |
| 4 | Save restored content | Document persisted |

**Testable outcome**: Recoverable content is restored within 5 seconds of recovery decision.

### 4.13 Scenario: Code Fence Editing

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Type triple backticks + language tag | Code fence begins |
| 2 | Type code content | Syntax highlighting applies based on language |
| 3 | Close fence with triple backticks | Code block complete |
| 4 | Place cursor inside fence, press Enter | New line within fence with proper indentation |
| 5 | Press Enter twice at end of fence | Exit code fence; new block created |

**Testable outcome**: Language tag preserved; syntax highlighting correct; fence structure valid.

### 4.14 Scenario: Table Editing

| Step | Actor Action | System Response |
|------|--------------|-----------------|
| 1 | Insert table via menu or markdown | Default table grid created |
| 2 | Tab between cells | Cursor moves to next cell |
| 3 | Type content | Cell content updates |
| 4 | Navigate to boundary cell and Tab | New column or row added as needed |
| 5 | Delete cell content, navigate away | Markdown source remains valid |

**Testable outcome**: Table survives edit-save-reopen; alignment preserved in rendered view.

---

## 5. Functional Requirements

### 5.1 File Operations

| ID | Requirement | Validation |
|----|-------------|------------|
| **FR-001** | User can create a new untitled Markdown document | New document buffer opens; content is empty |
| **FR-002** | User can open an existing `.md` file from disk via dialog or drag-drop | File content loads and renders correctly |
| **FR-003** | User can open a folder as a workspace | Sidebar shows file tree; `.md` files navigable |
| **FR-004** | User can save a document to chosen location | File persists as valid UTF-8 Markdown |
| **FR-005** | Auto-save triggers after configurable debounce when enabled | Dirty state clears; snapshot stored |
| **FR-006** | After crash/force-close, user is prompted to recover unsaved content | Recovery dialog shows on next launch |
| **FR-007** | When external tool modifies an open file, user is notified and can reload | Notification appears within 2 seconds of external change detection |

### 5.2 Core Editing Experience

| ID | Requirement | Validation |
|----|-------------|------------|
| **FR-008** | Document is edited in a single pane with inline rendering | No separate preview pane; syntax visually formatted |
| **FR-009** | Headings H1–H6 render with distinct visual hierarchy while editing | Font size, weight, spacing indicate heading level |
| **FR-010** | Bold, italic, strikethrough, inline code render inline | Cursor/selection behavior tested around each format |
| **FR-011** | Links render as interactive elements while editable | Clicking can open URL or edit link per design |
| **FR-012** | Ordered and unordered lists render with correct structure | Enter continues item; Enter on empty exits; Backspace degrades predictably |
| **FR-013** | Task list items render with checkbox visual; toggling preserves Markdown | `[ ]` and `[x]` states persist |
| **FR-014** | Blockquotes render with visual indent/marker and are easy to enter/exit | Enter on non-empty quote continues quote |
| **FR-015** | Fenced code blocks render with syntax highlighting | Language tag preserved; highlighting accurate |
| **FR-016** | Tables render with clear grid alignment | Cell navigation via Tab; structure valid after edit |
| **FR-017** | Local images insert with relative paths and render inline | Path preserved; broken images show error state |
| **FR-018** | Paste from clipboard handles plain text cleanly | No malformed Markdown in common cases |
| **FR-019** | Undo/redo operates at session level for all structural operations | Undo reverses last command; redo reapplies |

### 5.3 Markdown Support

| ID | Requirement | Validation |
|----|-------------|------------|
| **FR-020** | Parser supports: H1–H6, bold, italic, strikethrough, inline code, fenced code blocks, ordered/unordered lists, task lists, blockquotes, links, images, horizontal rules, tables, frontmatter (as text) | Parse output matches expected AST for each construct |
| **FR-021** | Markdown flavor: CommonMark baseline + GFM (tables, task lists, strikethrough, autolinks) | Compliance test suite passes |
| **FR-022** | Supported constructs survive edit-save-reopen cycle | Roundtrip fixture tests pass |

### 5.4 Workspace and Navigation

| ID | Requirement | Validation |
|----|-------------|------------|
| **FR-023** | File tree allows create, rename, delete files and folders | Operations reflect immediately in filesystem |
| **FR-024** | Recent files and folders are persisted and accessible from startup | List shows last 10 items |
| **FR-025** | Find/replace supports next/previous, replace one/all, case-sensitivity toggle | All matches found; replacements valid |
| **FR-026** | Outline panel shows heading hierarchy; clicking navigates | TOC updates on content change; navigation accurate |

### 5.5 Display, Focus, and Themes

| ID | Requirement | Validation |
|----|-------------|------------|
| **FR-027** | Light and dark themes apply to entire UI with correct typography | All interactive elements themed; contrast ratios meet accessibility thresholds |
| **FR-028** | Focus mode dims non-active paragraphs/sections | Only active paragraph is fully visible |
| **FR-029** | Typewriter mode keeps active line/paragraph vertically centered | Active line within center 40% of viewport |
| **FR-030** | Editor font size, content width, line spacing are configurable | Changes apply immediately to editor |

### 5.6 Export

| ID | Requirement | Validation |
|----|-------------|------------|
| **FR-031** | HTML export produces standalone or linked-assets output | Structure, styling, and images (if linked) render in browser |
| **FR-032** | PDF export produces paginated output with configurable page size and margins | PDF opens correctly in standard viewers |
| **FR-033** | Export pipeline is implemented behind a clear interface boundary | New export formats can be added without modifying core logic |

### 5.7 Preferences and State

| ID | Requirement | Validation |
|----|-------------|------------|
| **FR-034** | Settings persist across sessions: theme, auto-save toggle and interval, focus/typewriter mode defaults, editor width/font size, export defaults | Settings restored on relaunch |

---

## 6. Success Criteria

### 6.1 Functional Criteria

| Criterion | Measure |
|-----------|---------|
| User can create, open, edit, save, and reopen `.md` files | 100% of basic roundtrip tests pass |
| Single-pane editing renders supported Markdown inline | Visual inspection passes for all FR-008–FR-019 scenarios |
| Structural editing (lists, quotes, tables) produces valid Markdown | Fixture tests pass with zero structural corruption |
| Workspace sidebar enables folder-based navigation | File tree operations complete within 500ms |
| Find/replace operates correctly across all matches | All matches found; no false positives |
| Export to HTML and PDF preserves document structure | Exported files match source structure 100% |
| Crash recovery restores unsaved content | Content recoverable within 5s of user decision |
| Recent items persist across sessions | Last 10 files/folders accessible |
| Theme switching applies immediately | No flash or incorrect styling |
| Focus mode and typewriter mode function as specified | Behavior matches FR-028 and FR-029 |

### 6.2 Performance Criteria

| Criterion | Target |
|-----------|--------|
| Cold start time | < 2 seconds on mainstream modern laptop |
| Typing latency | No visible lag for documents under 1 MB |
| Large document usability | Documents up to 5 MB remain functional |
| Parse time | Under 100ms for typical 100 KB document |
| Serialization time | Under 50ms for typical 100 KB document |
| Export time | Under 5s for typical 50-page document to HTML/PDF |

### 6.3 Reliability Criteria

| Criterion | Target |
|-----------|--------|
| Crash-free session rate | > 99% for stable releases |
| Safe writes | Atomic write strategy prevents partial writes |
| Autosave reliability | Snapshots stored within 1s of configured interval |

### 6.4 Accessibility Criteria

| Criterion | Target |
|-----------|--------|
| Keyboard-first operation | All essential actions reachable via keyboard |
| Readability | Default themes meet WCAG AA contrast ratios |
| Typography | Comfortable reading size (16px minimum), proper line spacing (1.5–1.8×) |

### 6.5 Experience Criteria

Users report:
- "I forget I'm editing Markdown"
- "It feels smoother than editing raw `.md` in a code editor"
- "It does not break my files"
- "Images, tables, lists, and export work without friction"
- "It is calm enough for long-form writing"

---

## 7. Key Entities

### 7.1 Core Entities

| Entity | Description | Key Attributes |
|--------|-------------|-----------------|
| **Document** | In-memory representation of a Markdown file | content (raw string), is_dirty, file_path, encoding |
| **Buffer** | Editable text buffer with cursor/selection state | content, cursor_position, selection_range, undo_stack, redo_stack |
| **Workspace** | Folder-based project context | root_path, file_tree, active_file, recent_files |
| **FileTreeNode** | Node in workspace file tree | name, path, is_directory, children, is_expanded |
| **EditorCommand** | Action that modifies buffer state | command_type, parameters, execution_context |
| **CursorState** | Cursor and selection model | position, anchor, mode (normal/selection) |
| **MarkdownNode** | Semantic node from parsed Markdown | node_type (heading, paragraph, list, etc.), content, attributes, children |
| **RenderSpan** | Inline rendering unit | text, style (bold, italic, code, link), start_offset, end_offset |
| **RenderBlock** | Block rendering unit | block_type (paragraph, heading, code_block, table), children, attributes |
| **Theme** | Visual styling configuration | name, colors, typography, spacing |
| **Settings** | Persisted user preferences | theme, auto_save_interval, focus_mode_default, typewriter_mode_default, editor_font_size, editor_width |
| **Snapshot** | Autosaved document state for recovery | content, timestamp, file_path |
| **ExportOptions** | Export configuration | format (html/pdf), standalone, linked_assets, page_size, margins |
| **OutlineItem** | TOC entry | heading_level, text, position_in_document |

### 7.2 Service Entities

| Entity | Responsibility |
|--------|----------------|
| **DocumentService** | Open, save, reload, create documents |
| **EditorService** | Apply commands, query selection, manage undo/redo |
| **WorkspaceService** | File tree operations, recent items, path resolution |
| **ExportService** | Orchestrate HTML and PDF export pipelines |
| **SettingsService** | Load, persist, and broadcast settings changes |
| **RecoveryService** | Autosave snapshots, crash recovery, stale cleanup |

### 7.3 Relationship Summary

```
Workspace
  └── FileTree (many FileTreeNodes)
  └── active_file → Document
Document
  └── Buffer (cursor, selection, undo/redo)
  └── ParsedMarkdown → AST (MarkdownNodes)
  └── RenderTree (RenderBlocks, RenderSpans)
Settings → Theme
RecoveryService → Snapshot
ExportService → ExportOptions
```

---

## 8. Module Boundaries

| Module | Owned By | Responsibility |
|--------|----------|----------------|
| **core-model** | Rust | Semantic document structures, positions, ranges, shared types, errors |
| **markdown-parser** | Rust | Parse Markdown to semantic representation |
| **editor-engine** | Rust | Cursor movement, selection, commands, smart editing behaviors, undo/redo |
| **serializer** | Rust | Semantic model → Markdown output |
| **workspace** | Rust | File I/O, recent items, file watching, asset path resolution |
| **export** | Rust | HTML and PDF export pipelines |
| **theme** | Rust | Theme tokens, typography defaults |
| **settings** | Rust | Persisted configuration, schema versioning |
| **recovery** | Rust | Autosave snapshots, crash recovery metadata |
| **app-services** | Rust | Orchestration layer between UI and Rust modules |
| **desktop-shell** | Tauri | Windows, menus, file dialogs, OS integration, packaging |

---

## 9. Out-of-Scope for MVP

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph
- Workspace complexity comparable to IDEs

---

## 10. Milestone Summary

| Milestone | Key Deliverables |
|-----------|------------------|
| **M0: Bootstrap** | Repo, Cargo workspace, CI, desktop shell launches |
| **M1: Plain Document Loop** | New/open/save, text buffer, dirty state, recent files, recovery plumbing |
| **M2: Live Rendering Foundation** | Parser integration, inline rendering of core syntax, cursor mapping |
| **M3: Editing Semantics** | Smart Enter/Backspace/Tab, task lists, shortcut formatting, editing invariant tests |
| **M4: Authoring Essentials** | Workspace sidebar, images, code highlighting, table support, outline panel |
| **M5: Calm Writing Features** | Themes, focus mode, typewriter mode, settings persistence, find/replace |
| **M6: Recovery, Export, Beta** | Robust autosave/recovery, HTML/PDF export, packaging for all platforms |
