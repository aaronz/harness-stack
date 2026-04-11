# RustNote Feature Specification

**Short Name:** RustNote — Typora-like Markdown Editor

**Document Version:** 1.0  
**Based On:** Open-Source Implementation-Ready PRD v2.1  
**Date:** 2026-04-11

---

## 1. Short Name

**RustNote** — A writing-first, WYSIWYM (What You See Is What You Mean) Markdown editor built in Rust.

---

## 2. Key Concepts Extracted

### 2.1 Actors

| Actor | Description |
|-------|-------------|
| **Writers & Note-takers** | Primary users seeking a clean writing space with Markdown portability |
| **Developers & Technical Authors** | Users editing README, docs, specs with Git-friendly output and code-block correctness |
| **Product Managers & Researchers** | Users drafting with headings, lists, tables, images, and export needs |

### 2.2 Core Actions

| Category | Actions |
|----------|---------|
| **File Operations** | New, Open, Save, Auto-save, Recovery, External-change detection |
| **Editing** | Live rendered editing, Cursor/Selection, Undo/Redo, Structural editing (Enter/Backspace/Tab) |
| **Content Creation** | Headings, Emphasis, Links, Images, Lists, Task Lists, Tables, Code Blocks, Quotes, HR |
| **Navigation** | Workspace tree, Find/Replace, Outline/TOC panel |
| **Display** | Themes (light/dark), Focus mode, Typewriter mode, Typography settings |
| **Export** | HTML export, PDF export |

### 2.3 Data

| Data Type | Description |
|-----------|-------------|
| **Source Layer** | Raw Markdown text (UTF-8) |
| **Semantic Layer** | Parsed structures (headings, paragraphs, lists, tables, code blocks) |
| **Editing Layer** | Selection state, cursor mapping, commands, undo/redo history |
| **Presentation Layer** | Rendered spans/blocks, visual decorations, focus/typewriter metadata |
| **Settings** | Theme preferences, auto-save config, editor width/font size |
| **Recovery Data** | Autosave snapshots, crash recovery metadata |

### 2.4 Constraints

| Constraint | Details |
|------------|---------|
| **Platform** | Desktop: macOS, Windows, Linux |
| **Runtime** | Tauri (V1) |
| **Language** | Rust (core correctness) |
| **Markdown Flavor** | CommonMark baseline + GFM (tables, task lists, strikethrough, autolinks) |
| **License** | MIT OR Apache-2.0 |
| **Performance** | Cold start <2s, typing no visible lag on <1MB docs, 5MB usable |

---

## 3. User Scenarios & Testing

### 3.1 Scenario: Creating a New Document

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | Launch RustNote | Empty editor opens within 2 seconds |
| 2 | Begin typing | Text appears in readable formatted style |
| 3 | Use keyboard shortcuts (Ctrl/Cmd+B, Ctrl/Cmd+I) | Bold/italic formatting applied visually |
| 4 | Type "# Heading" | Heading renders with proper visual hierarchy |
| 5 | Save document | File saved as `.md` with valid Markdown content |

**Test Fixtures Required:**
- `fixtures/editor/new-document-flow.md` — sequence of Markdown inputs and expected cursor positions

### 3.2 Scenario: Opening and Editing Existing File

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | File > Open or drag-drop `.md` file | File content loads, renders correctly |
| 2 | Edit content | Changes reflected immediately in rendered view |
| 3 | Save (Ctrl/Cmd+S) | File saved preserving valid Markdown |
| 4 | Close and reopen | Document reopens with exact same content |

**Test Fixtures Required:**
- `fixtures/markdown/headings.md`, `fixtures/markdown/lists.md`, `fixtures/markdown/tables.md`, `fixtures/markdown/code-fences.md`, `fixtures/markdown/links-images.md`, `fixtures/markdown/task-lists.md`, `fixtures/markdown/quotes.md`

### 3.3 Scenario: List and Structure Editing

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | Type "- item" | Unordered list renders |
| 2 | Press Enter | New list item created |
| 3 | Press Enter on empty item | List exits, plain paragraph starts |
| 4 | Press Backspace at list start | List item deleted, cursor at end of previous item |
| 5 | Press Tab on list item | Item indents (nested list) |
| 6 | Press Shift+Tab on nested item | Item outdents |

**Regression Test Fixtures Required:**
- `fixtures/editor/list-behaviors.md` — Enter/Backspace/Tab sequences at list boundaries

### 3.4 Scenario: Workspace Folder Workflow

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | Open folder as workspace | Sidebar shows file tree |
| 2 | Click `.md` file | File opens in editor |
| 3 | Create new file via sidebar | New untitled file opens |
| 4 | Rename file via sidebar | File renamed, editor title updates |
| 5 | Delete file via sidebar | File deleted, confirmation shown |

### 3.5 Scenario: Image Insertion

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | Paste image from clipboard or drag-drop | Image inserted with relative path `![](./assets/image.png)` |
| 2 | Image displays | Image renders inline in editor |
| 3 | Move `.md` file to new location | Relative path preserved, image still displays |

### 3.6 Scenario: Export to HTML

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | File > Export > HTML | Export dialog opens |
| 2 | Choose standalone or linked-assets | HTML generated accordingly |
| 3 | Export | File saved, preserves headings, lists, code, tables, images |

### 3.7 Scenario: Crash Recovery

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | Edit document without saving | Dirty state indicated (dot in title) |
| 2 | Simulate crash/force close | On restart, recovery dialog shows |
| 3 | Choose restore | Unsaved content restored |

### 3.8 Scenario: Focus Mode

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | Toggle focus mode (View menu or shortcut) | Non-active paragraphs/sections dim |
| 2 | Continue typing | Current paragraph remains highlighted |
| 3 | Navigate with arrow keys | Focus shifts, visual emphasis follows |

### 3.9 Scenario: Typewriter Mode

| Step | User Action | Expected Behavior |
|------|-------------|-------------------|
| 1 | Toggle typewriter mode | Active line centers vertically |
| 2 | Type continuously | View auto-scrolls to keep line centered |
| 3 | Navigate with mouse click | Mode temporarily disengages, re-engages on typing |

---

## 4. Functional Requirements

### 4.1 File Operations

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-001 | User can create new untitled Markdown document | Manual: Create new → verify empty buffer → type → save |
| FR-002 | User can open existing `.md` file from disk | Manual: Open file → verify content matches source |
| FR-003 | User can drag-and-drop `.md` file to open | Manual: Drag file → verify opens with correct content |
| FR-004 | User can open folder as workspace | Manual: Open folder → verify file tree in sidebar |
| FR-005 | User can save document preserving UTF-8 Markdown | Automated: Roundtrip fixture — parse → serialize → parse → compare |
| FR-006 | User can configure auto-save on/off and debounce interval | Manual: Enable → edit → wait → verify auto-save occurs |
| FR-007 | Dirty state is visually indicated (e.g., dot in title) | Manual: Edit → verify indicator → save → verify indicator clears |
| FR-008 | User can recover unsaved content after force close | Manual: Edit without save → force close → restart → verify recovery dialog |
| FR-009 | App detects external file changes and prompts reload | Manual: Edit externally → verify notification → choose reload |

### 4.2 Core Editing Experience

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-010 | Single-pane editing with live rendered Markdown | Manual: Edit any Markdown → verify rendered inline, not split preview |
| FR-011 | Headings H1-H6 render with visual hierarchy differentiation | Visual: Each level distinguishable by size/weight |
| FR-012 | Bold, italic, strikethrough, inline code render inline | Visual: Format applied immediately on typing |
| FR-013 | Links render as clickable links in editor (visual only, not navigable) | Visual: Links appear styled, not as raw Markdown |
| FR-014 | Images render inline in editor with relative path handling | Manual: Insert image → verify renders → move file → verify path still works |
| FR-015 | Ordered/unordered lists render with correct visual markers | Visual: Numbers/bullets display correctly |
| FR-016 | Enter continues list item | Automated: Press Enter → verify new item created |
| FR-017 | Enter on empty list item exits list | Automated: Empty item → Enter → verify plain paragraph |
| FR-018 | Backspace at list/heading/quote boundary behaves predictably | Automated: Backspace at boundary → verify structural behavior |
| FR-019 | Tab/Shift+Tab indent/outdent nested list items | Automated: Tab on item → verify indent; Shift+Tab → verify outdent |
| FR-020 | Task list items render with checkbox visual treatment | Visual: Checkbox displayed |
| FR-021 | Toggling task state preserves valid Markdown | Automated: Toggle → save → reopen → verify checkbox state preserved |
| FR-022 | Blockquotes render with visual distinction | Visual: Indent and styling applied |
| FR-023 | Fenced code blocks render with syntax highlighting | Visual: Language-based highlighting applied |
| FR-024 | Tables render in clear grid format | Visual: Table displays with proper alignment |
| FR-025 | Horizontal rules render as visual dividers | Visual: HR displays as line |
| FR-026 | Paste plain text works predictably | Manual: Copy text → paste → verify plain text inserted |
| FR-027 | Session-level undo/redo works for all editing operations | Manual: Undo/redo operations → verify state changes correctly |
| FR-028 | Cursor movement is predictable around all inline formatting | Automated: Cursor key tests around links, code, emphasis |
| FR-029 | Selection behavior feels natural around all constructs | Manual: Select across formatting → verify selection highlights correctly |

### 4.3 Markdown Support

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-030 | CommonMark baseline parsing supported | Automated: Parse CommonMark spec fixtures → verify pass rate |
| FR-031 | GFM extensions (tables, task lists, strikethrough, autolinks) supported | Automated: GFM fixture tests |
| FR-032 | Supported constructs survive edit-save-reopen cycles | Automated: Roundtrip fixture suite |
| FR-033 | Frontmatter preserved as text block | Manual: Add frontmatter → save → reopen → verify preserved |

### 4.4 Workspace and Navigation

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-034 | Sidebar displays workspace file tree | Manual: Open folder → verify files listed |
| FR-035 | User can create, rename, delete files/folders in sidebar | Manual: Each operation → verify file system and UI update |
| FR-036 | Sidebar refreshes on external changes | Manual: External change → verify sidebar updates |
| FR-037 | Recent files and folders persist across sessions | Manual: Open files → restart → verify recent items present |
| FR-038 | Find/Replace: find next/previous, replace one/all, case-sensitive option | Manual: Each find/replace variant works correctly |
| FR-039 | Outline/TOC panel shows document heading structure | Manual: Open doc with headings → verify TOC matches |
| FR-040 | Clicking TOC item navigates to corresponding section | Manual: Click heading → verify scroll to section |

### 4.5 Display and Themes

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-041 | Light theme renders with comfortable readability | Visual: Verify contrast, typography, spacing |
| FR-042 | Dark theme renders with comfortable readability | Visual: Verify contrast, typography, spacing |
| FR-043 | Focus mode dims non-active paragraphs | Manual: Enable → verify active paragraph highlighted |
| FR-044 | Typewriter mode keeps active line centered vertically | Manual: Enable → type → verify auto-scroll |
| FR-045 | User can adjust editor font size | Manual: Change setting → verify font size updates |
| FR-046 | User can adjust content width | Manual: Change setting → verify width updates |
| FR-047 | User can adjust line spacing | Manual: Change setting → verify spacing updates |

### 4.6 Export

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-048 | HTML export produces standalone HTML | Automated: Export → verify single file, no external dependencies |
| FR-049 | HTML export produces linked-assets HTML | Automated: Export → verify HTML + separate assets folder |
| FR-050 | HTML export preserves headings, lists, code, tables, images | Automated: Export fixture → compare output |
| FR-051 | PDF export produces readable document | Manual: Export → verify structure and layout |
| FR-052 | PDF export supports configurable page size and margins | Manual: Change settings → export → verify PDF respects settings |
| FR-053 | Export functionality behind clear interface boundary | Code review: Verify export behind service interface |

### 4.7 Preferences

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-054 | Theme preference persists across sessions | Manual: Set theme → restart → verify theme applied |
| FR-055 | Auto-save preference persists | Manual: Configure → restart → verify setting preserved |
| FR-056 | Focus mode default persists | Manual: Configure → restart → verify default applied |
| FR-057 | Typewriter mode default persists | Manual: Configure → restart → verify default applied |
| FR-058 | Editor width/font size persists | Manual: Configure → restart → verify settings applied |
| FR-059 | Export defaults persist | Manual: Configure → restart → verify defaults applied |

### 4.8 Performance

| ID | Requirement | Testability |
|----|-------------|-------------|
| FR-060 | Cold start under 2 seconds on modern laptop | Automated: Benchmark cold start time |
| FR-061 | No visible lag during typing on documents under 1 MB | Manual: Open large doc → type → verify no lag |
| FR-062 | Documents up to 5 MB remain usable | Manual: Open 5MB doc → verify usable with graceful degradation |

---

## 5. Success Criteria

### 5.1 User Experience Criteria

| Criterion | Measurable Indicator |
|------------|---------------------|
| **Seamless Live Rendering** | User can edit without mode-switching; document is readable while editing |
| **Writing Flow Uninterrupted** | User forgets they are editing Markdown; no syntax constantly visible |
| **Local-First Trust** | Files saved are valid `.md`; no data loss in normal workflows |
| **Calm UI** | Toolbar, controls, settings do not dominate writing surface |
| **High-Frequency Actions Effortless** | Lists, links, images, tables, code blocks, paste, export feel natural |

### 5.2 Functional Success Criteria

| Criterion | Measurable Indicator |
|-----------|---------------------|
| **File Operations** | New/Open/Save/Recovery all functional; no data loss |
| **Edit Fidelity** | Edit-save-reopen produces identical content |
| **Structural Editing** | Enter/Backspace/Tab behave predictably at list/quote/heading boundaries |
| **Cursor Invariants** | No cursor traps; selection feels natural around all constructs |
| **Markdown Support** | CommonMark + GFM supported constructs parse and serialize correctly |
| **Export Quality** | HTML and PDF exports match document appearance |

### 5.3 Performance Success Criteria

| Criterion | Measurable Indicator |
|-----------|---------------------|
| **Startup Time** | < 2 seconds cold start |
| **Typing Responsiveness** | No visible lag under normal editing |
| **Large Document Usability** | 5 MB documents remain usable |

### 5.4 Reliability Success Criteria

| Criterion | Measurable Indicator |
|-----------|---------------------|
| **Crash Recovery** | Unsaved content restored after force close |
| **Safe Writes** | No partial writes; atomic save strategy |
| **Crash-Free Goal** | Stable releases target >99% crash-free sessions |

---

## 6. Key Entities

### 6.1 Document Entities

| Entity | Description |
|--------|-------------|
| **Document** | Root entity containing semantic tree, source text, cursor state, selection state |
| **Block** | Base type for heading, paragraph, list, list-item, blockquote, code-block, table, HR |
| **Inline** | Base type for text, emphasis, strong, strikethrough, inline-code, link, image |
| **Cursor** | Position in source with line/column mapping |
| **Selection** | Range from anchor to head |
| **Command** | Edit action (insert, delete, format, structural) |

### 6.2 Workspace Entities

| Entity | Description |
|--------|-------------|
| **Workspace** | Root folder with file tree |
| **FileNode** | File or folder in workspace tree |
| **RecentItem** | Recently opened file or folder |
| **AssetPath** | Resolved relative path for images/assets |

### 6.3 Service Entities

| Entity | Description |
|--------|-------------|
| **DocumentService** | open/save/reload document operations |
| **EditorService** | apply commands, query selection, undo/redo |
| **WorkspaceService** | file tree operations, asset resolution |
| **ExportService** | HTML/PDF export orchestration |
| **SettingsService** | preferences persistence |
| **RecoveryService** | autosave snapshots, crash recovery |
| **ParserService** | Markdown to semantic model |
| **SerializerService** | semantic model to Markdown output |

### 6.4 UI Entities

| Entity | Description |
|--------|-------------|
| **EditorView** | Main editing canvas |
| **SidebarView** | Workspace file tree |
| **OutlineView** | Table of contents panel |
| **FindReplacePanel** | Search and replace UI |
| **Theme** | Light/dark theme tokens |
| **ExportDialog** | Export options UI |
| **PreferencesDialog** | Settings UI |

---

## 7. Implementation Milestone Mapping

| Milestone | Deliverables | Acceptance Criteria |
|-----------|--------------|---------------------|
| **M0: Bootstrap** | Repo, Cargo workspace, CI, empty shell | Contributors can build locally; CI green |
| **M1: Plain Document Loop** | New/Open/Save, text buffer, dirty state, recent files, recovery plumbing | Safe create/edit of `.md`; no data loss |
| **M2: Live Rendering Foundation** | Parser integration, render core syntax, cursor/selection mapping | Supported syntax readable while editing; fidelity passes fixtures |
| **M3: Editing Semantics** | Smart Enter/Backspace/Tab, task lists, formatting shortcuts, regression tests | Structural editing consistent; no major cursor traps |
| **M4: Authoring Essentials** | Workspace sidebar, images, code highlighting, tables, outline panel | Real documentation workflow viable |
| **M5: Calm Writing Features** | Themes, focus mode, typewriter mode, settings, find/replace | Writing-first feel achieved |
| **M6: Recovery & Export** | Autosave/recovery, HTML/PDF export, packaging | Export clean; crash recovery works; beta builds available |

---

## 8. Test Fixture Requirements

### 8.1 Markdown Parse Fixtures

```
fixtures/
  markdown/
    headings.md          # H1-H6, variations
    emphasis.md          # bold, italic, strikethrough, combinations
    links-images.md      # links, images, relative paths
    lists.md             # ordered, unordered, nested
    task-lists.md        # task list variations
    tables.md            # basic and complex tables
    code-fences.md       # fenced code with languages
    quotes.md           # blockquotes, nested
    hr.md                # horizontal rules
    frontmatter.md       # YAML frontmatter preservation
    commonmark-spec.md   # CommonMark reference tests
    gfm-spec.md          # GFM extension tests
```

### 8.2 Editor Behavior Fixtures

```
fixtures/
  editor/
    list-behaviors.md           # Enter/Backspace/Tab sequences
    cursor-across-inline.md     # Cursor movement around formatting
    selection-across-blocks.md  # Selection boundaries
    table-editing.md            # Table interaction edge cases
    task-toggle.md              # Task list state changes
```

### 8.3 Roundtrip Fixtures

```
fixtures/
  roundtrip/
    # Parse → Serialize → Parse → Compare
    simple.md
    complex-nested.md
    code-with-lang.md
    table-complex.md
```

### 8.4 Export Fixtures

```
fixtures/
  export/
    input/
      simple.md
      with-images.md
      with-tables.md
      with-code.md
    expected/
      html-simple.html
      html-with-images.html
      pdf-simple.pdf
```

### 8.5 Recovery Fixtures

```
fixtures/
  recovery/
    autosave/
    crash-scenarios/
```

---

## 9. Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-11 | Initial specification based on RustNote PRD v2.1 |

---

*Specification generated for RustNote implementation planning.*
