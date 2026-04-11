# RustNote Feature Specification

## 1. Feature Name

**RustNote** — A Writing-First, WYSIWYM Markdown Editor

---

## 2. Brief Description

RustNote is an open-source, local-first, Typora-like Markdown editor built in Rust. It provides a single-pane seamless reader-writer experience where users write in a visually formatted document while the underlying source of truth remains plain Markdown. The product prioritizes writing flow, readability, and Markdown fidelity over feature complexity.

---

## 3. Key Concepts

### Actors

| Actor | Primary Needs |
|-------|---------------|
| Writers and note-takers | Clean writing space with Markdown portability |
| Developers and technical authors | README/docs/spec editing with Git-friendly output and code-block correctness |
| Product managers and researchers | Readable drafting with headings, lists, tables, images, and export |

### Core Actions

| Category | Actions |
|----------|---------|
| **File Operations** | New file, Open file, Open folder as workspace, Save, Auto-save, Recovery |
| **Editing** | Single-pane live rendering, Heading/emphasis/list/quote/code editing, Links, Images, Tables, Task lists |
| **Navigation** | Find/replace, Outline/TOC panel, Recent files/folders |
| **Display** | Themes (light/dark), Focus mode, Typewriter mode |
| **Export** | HTML export, PDF export |

### Data

| Data Type | Description |
|-----------|-------------|
| **Source** | Raw Markdown text (UTF-8, `.md` extension) |
| **Semantic** | Parsed structures: headings, paragraphs, lists, tables, links, code blocks, images |
| **Editing** | Selection state, cursor position, undo/redo history |
| **Config** | Theme preferences, editor settings, recent items |
| **Recovery** | Autosave snapshots, crash recovery metadata |

### Constraints

| Constraint | Requirement |
|------------|-------------|
| **Platform** | Desktop (macOS, Windows, Linux) via Tauri |
| **Performance** | Cold start < 2s, no typing lag, 5MB+ documents usable |
| **Architecture** | Rust core owns correctness; frontend renders derived state |
| **Privacy** | Local-first, no network dependency in editing path |

---

## 4. User Scenarios & Testing

### Scenario 1: New Document Creation

**Trigger**: User clicks "New File" or uses keyboard shortcut  
**Flow**:
1. App creates untitled document in memory
2. User types content with live rendered preview
3. User saves to chosen location (default `.md`)

**Test Cases**:
- [ ] New file creates empty document with visible cursor
- [ ] Save dialog defaults to `.md` extension
- [ ] Saved file is valid UTF-8 Markdown

### Scenario 2: Open and Edit Existing Document

**Trigger**: User opens file via dialog, drag-drop, or recent items  
**Flow**:
1. File content loads and parses
2. Document renders in single-pane editor
3. User edits; changes reflect live
4. Dirty state indicator appears
5. User saves; dirty state clears

**Test Cases**:
- [ ] File opens within 500ms for typical documents
- [ ] Rendered content matches CommonMark+GFM spec
- [ ] Edit-save-reopen preserves all supported constructs
- [ ] Large files (5MB) remain usable with < 100ms edit latency

### Scenario 3: Structural Editing (Lists)

**Trigger**: User creates list, presses Enter, uses Backspace/Tab  
**Flow**:
1. User types `- item` → renders as bullet list
2. Press Enter → new list item appears
3. Press Enter on empty item → list exits
4. Backspace at boundary → list degrades predictably
5. Tab indents nested item; Shift+Tab outdents

**Test Cases**:
- [ ] Enter on non-empty item continues list with new marker
- [ ] Enter on empty item exits list (marker removed)
- [ ] Backspace at list start converts to paragraph
- [ ] Tab/Shift+Tab on list item indents/outdents
- [ ] Nested lists render with correct visual hierarchy
- [ ] Task lists toggle checkbox state without corruption

### Scenario 4: Link and Image Insertion

**Trigger**: User inserts link or image via UI or paste  
**Flow**:
1. User triggers insert action
2. Link dialog appears with text/target fields
3. User confirms; link renders inline
4. Relative paths resolved from document location

**Test Cases**:
- [ ] Link renders with distinguishable styling
- [ ] Click on link opens target; edit mode allows target change
- [ ] Image displays within document flow
- [ ] Broken images show error state without layout destruction
- [ ] Relative asset paths survive save/reload cycle

### Scenario 5: Focus and Typewriter Mode

**Trigger**: User toggles focus/typewriter mode  
**Flow**:
1. Focus mode dims non-active paragraphs
2. Typewriter mode centers active line vertically
3. Transitions are smooth, not jarring

**Test Cases**:
- [ ] Focus mode dims surrounding content by ≥ 30%
- [ ] Active paragraph remains fully visible
- [ ] Typewriter mode keeps active line within center 40% of viewport
- [ ] Modes can be toggled independently
- [ ] Default preferences persist across sessions

### Scenario 6: Export to HTML/PDF

**Trigger**: User triggers export via menu/command palette  
**Flow**:
1. Export dialog appears with format options
2. User selects destination and options
3. Export processes with progress indication
4. Success notification with "open file" option

**Test Cases**:
- [ ] HTML export produces standalone file with embedded styles
- [ ] HTML export produces linked-assets mode
- [ ] PDF export preserves headings, lists, tables, code blocks
- [ ] Images embed correctly in PDF
- [ ] Export errors show human-readable messages

### Scenario 7: Crash Recovery

**Trigger**: App crashes or force-closes with unsaved changes  
**Flow**:
1. On next launch, recovery prompt appears
2. User can restore, inspect, or dismiss
3. Restored content appears in new session

**Test Cases**:
- [ ] Autosave creates snapshot at configurable interval
- [ ] Recovery prompt appears when unsaved session exists
- [ ] Restored content matches pre-crash state
- [ ] Stale recovery files cleaned on successful save

---

## 5. Functional Requirements

### 5.1 File Operations

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-001 | New file creation | User can create untitled document saved as `.md` |
| FR-002 | Open file | Markdown files open and display content within 500ms for < 1MB |
| FR-003 | Open folder as workspace | Sidebar shows `.md` files; folders expandable |
| FR-004 | Save with fidelity | Saved file is valid UTF-8 Markdown; roundtrip preserves supported syntax |
| FR-005 | Auto-save | Configurable debounce; dirty state indicator visible |
| FR-006 | Crash recovery | Unsaved content restored after force close |
| FR-007 | External change detection | Prompt user to reload, keep, or compare on external change |

### 5.2 Core Editing Experience

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-008 | Single-pane live rendering | One editor canvas; no split preview as default |
| FR-009 | Heading rendering | H1-H6 visually distinct; editing markers minimal |
| FR-010 | Emphasis rendering | Bold, italic, strikethrough, inline code visually clear |
| FR-011 | Link behavior | Links appear as document content; hover shows target preview |
| FR-012 | List behavior | Enter continues list; Enter on empty exits; Backspace at boundary degrades predictably |
| FR-013 | Task list behavior | Checkbox toggles preserve Markdown task syntax |
| FR-014 | Blockquote rendering | Visual treatment clearly differentiates from body text |
| FR-015 | Code fence rendering | Syntax highlighting; language tag preserved |
| FR-016 | Table rendering | Tables display in grid format; basic cell editing works |
| FR-017 | Image rendering | Images display inline; broken images show error state |
| FR-018 | Paste behavior | Plain text pastes as plain text; rich text converts to Markdown best-effort |
| FR-019 | Undo/redo | Session-level undo/redo; structural operations participate consistently |

### 5.3 Markdown Support

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-020 | Syntax coverage | H1-H6, bold, italic, strikethrough, inline code, fenced code, lists, task lists, blockquotes, links, images, HR, tables, frontmatter |
| FR-021 | Flavor | CommonMark baseline + GFM (tables, task lists, strikethrough, autolinks) |
| FR-022 | Serialization fidelity | Edit-save-reopen cycle preserves all supported constructs |

### 5.4 Workspace and Navigation

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-023 | File tree | Create, rename, delete files/folders; refresh on external changes |
| FR-024 | Recent items | Persist and display recent files and folders |
| FR-025 | Find/replace | Find next/previous, replace one/all, case-sensitive toggle |
| FR-026 | Outline panel | Heading tree from document; click navigates to section |

### 5.5 Display and Themes

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-027 | Themes | Light and dark themes; typography tuned for readability |
| FR-028 | Focus mode | Non-active content dimmed ≥ 30% |
| FR-029 | Typewriter mode | Active line centered within center 40% of viewport |
| FR-030 | Typography settings | Font size, content width, line spacing configurable |

### 5.6 Export

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-031 | HTML export | Standalone or linked-assets mode; preserves headings, lists, code, tables, images |
| FR-032 | PDF export | Preserves structure; configurable page size and margins |
| FR-033 | Export architecture | Export behind interface boundary for future format extensibility |

### 5.7 Preferences

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-034 | Preferences persistence | Theme, auto-save, focus/typewriter defaults, editor width/font, export defaults saved and restored |

---

## 6. Success Criteria

### 6.1 User Experience Criteria

| Criterion | Measurement |
|-----------|-------------|
| **Writing flow feels uninterrupted** | User survey: ≥ 80% agree "I forget I'm editing Markdown" |
| **Readable while editing** | Visual clarity matches Typora baseline for headings, lists, quotes, code |
| **No cursor traps** | No reported cursor issues in supported editing flows |
| **No Markdown corruption** | Zero data loss incidents in supported save/reload cycles |
| **Calm UI** | Default chrome occupies ≤ 15% of viewport |
| **Fast startup** | Cold start < 2 seconds on mainstream laptop |
| **Responsive editing** | No visible lag during typing on documents < 1MB |

### 6.2 Technical Criteria

| Criterion | Measurement |
|-----------|-------------|
| **Parse fidelity** | All CommonMark+GFM supported constructs survive roundtrip |
| **Edit invariants** | Structural editing behaviors documented and regression-tested |
| **Crash-free rate** | Stable releases target > 99% crash-free sessions |
| **Export fidelity** | HTML and PDF exports match rendered view for representative documents |
| **Cross-platform** | Builds available and functional on macOS, Windows, Linux |

### 6.3 Open-Source Criteria

| Criterion | Measurement |
|-----------|-------------|
| **Contributor accessibility** | New contributor can build locally from README in ≤ 10 minutes |
| **PR reviewability** | Core PRs under 400 lines; larger PRs justified with architecture rationale |
| **Architecture documentation** | ADRs for all major decisions; repo docs cover supported Markdown behavior |

---

## 7. Key Entities

### 7.1 Core Entities

| Entity | Responsibility |
|--------|---------------|
| **Document** | In-memory semantic representation of parsed Markdown |
| **Buffer** | Raw text with dirty state tracking |
| **Workspace** | Folder-rooted context for file tree and asset resolution |
| **Selection** | Cursor position, anchor, and active range |
| **Command** | Edit action (insert, delete, transform) with undo/redo support |
| **Theme** | Color tokens, typography scale, spacing rules |
| **ExportJob** | Export request with format, options, and destination |

### 7.2 UI Entities

| Entity | Responsibility |
|--------|---------------|
| **EditorCanvas** | Main editing surface; renders Document state |
| **BlockRenderer** | Renders semantic blocks (heading, list, table, code, quote, image) |
| **InlineRenderer** | Renders inline spans (emphasis, link, code) |
| **Sidebar** | Workspace tree and recent items |
| **OutlinePanel** | TOC from document headings |
| **CommandPalette** | Fuzzy-searchable action invocation |
| **SearchBar** | Find/replace UI with match navigation |
| **PreferencesDialog** | Settings organized by user mental model |

### 7.3 Service Entities

| Entity | Responsibility |
|--------|---------------|
| **DocumentService** | Open, save, reload, serialize documents |
| **EditorService** | Execute commands, query selection, apply edits |
| **WorkspaceService** | File tree, recent items, asset paths |
| **ExportService** | HTML and PDF export orchestration |
| **SettingsService** | Persist and restore user preferences |
| **RecoveryService** | Autosave snapshots, crash recovery, stale cleanup |

### 7.4 Data Entities

| Entity | Description |
|--------|-------------|
| **MarkdownSource** | Raw UTF-8 text from `.md` file |
| **SemanticTree** | Parsed document structure (headings, blocks, spans) |
| **EditingHistory** | Undo/redo stack of Commands |
| **RecoverySnapshot** | Autosaved document state with timestamp |
| **RecentItem** | File path and last open timestamp |
| **ThemeTokens** | Design tokens for colors, spacing, typography |

---

## 8. Invariants (Non-Negotiable Experience Contracts)

| Invariant | Definition |
|-----------|------------|
| **Single-Pane** | One editor canvas is the primary experience; split preview is not default |
| **Readability** | Headings look like headings; lists look like lists; document is pleasant to read while editing |
| **Cursor** | Cursor movement is predictable; no cursor traps; selection boundaries feel natural |
| **Structure** | Enter, Backspace, Tab, Shift+Tab behave consistently for lists, quotes, nested structures |
| **Fidelity** | Editing and rendering do not silently destroy supported Markdown constructs |
| **Calmness** | Default UI is visually quiet; focus/typewriter modes are functional, not decorative |
| **Local-Trust** | Save, autosave, recovery, and external-change handling are dependable |

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

## 10. Dependencies and Boundaries

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop Shell                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐   │
│  │ App Services│  │  Settings   │  │   Recovery      │   │
│  │ (Orchestra- │  │  Service    │  │   Service       │   │
│  │   tion)     │  │             │  │                 │   │
│  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘   │
├─────────┼────────────────┼──────────────────┼─────────────┤
│  ┌──────┴──────┐  ┌──────┴──────┐  ┌────────┴────────┐   │
│  │  Workspace  │  │   Editor    │  │    Export       │   │
│  │   Service   │  │   Engine    │  │    Service      │   │
│  │ (File I/O,  │  │ (Cursor,    │  │   (HTML, PDF)   │   │
│  │  Watching)  │  │  Selection, │  │                 │   │
│  │             │  │  Commands)  │  │                 │   │
│  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘   │
├─────────┼────────────────┼──────────────────┼─────────────┤
│  ┌──────┴──────┐  ┌──────┴──────┐  ┌────────┴────────┐   │
│  │  Serializer │  │   Parser    │  │   Core Model    │   │
│  │ (MD output) │  │ (MD → Tree)  │  │ (Positions,     │   │
│  │             │  │              │  │  Ranges, Ids)   │   │
│  └─────────────┘  └─────────────┘  └─────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│                    Frontend (TypeScript/UI)                  │
│  EditorCanvas, Sidebar, Panels, Dialogs, Theme Rendering    │
└─────────────────────────────────────────────────────────────┘
```

**Rust-owned boundaries (correctness-critical)**:
- Markdown parsing and serialization
- Cursor/selection semantics
- Structural editing behavior
- Save/recovery correctness
- Export correctness

**Frontend-owned**:
- Visual rendering of document state
- Transient interaction state
- Panel and dialog presentation
- Theme application

---

*Specification Version: 1.0*  
*Source: RustNote PRD v2.1*  
*Generated for: Implementation Planning*
