# RustNote Feature Specification

## 1. Feature Name

**RustNote** — A Writing-First, WYSIWYM Markdown Editor

---

## 2. Key Concepts

### 2.1 Actors

| Actor | Primary Jobs |
|-------|-------------|
| **Writer / Note-taker** | Create and edit documents in a calm, distraction-free environment with Markdown portability |
| **Developer / Technical Author** | Edit README/docs/specs with code fences, tables, and Git-friendly output |
| **Product Manager / Researcher** | Draft documents with headings, lists, tables, images, and export to polished formats |

### 2.2 Core Actions

| Action Category | Key Operations |
|----------------|----------------|
| **File Operations** | New file, Open file, Open folder as workspace, Save, Auto-save, Crash recovery, Handle external changes |
| **Editing** | Single-pane live rendering, Smart Enter/Backspace/Tab for lists/quotes, Inline formatting (bold/italic/strikethrough/code), Undo/redo |
| **Content Blocks** | Headings, Lists (ordered/unordered/task), Blockquotes, Code fences, Tables, Images, Links, Horizontal rules |
| **Navigation** | File tree, Recent items, Find/replace, Outline/TOC panel |
| **Display** | Themes (light/dark), Focus mode, Typewriter mode, Font/width settings |
| **Export** | HTML export, PDF export |

### 2.3 Key Data

| Data Entity | Description |
|-------------|-------------|
| **Document** | Raw Markdown text with semantic structure overlay |
| **Workspace** | Folder-based workspace with file tree |
| **Settings** | Theme, auto-save, focus/typewriter defaults, editor width, export defaults |
| **Recovery Snapshot** | Autosaved content for crash recovery |
| **Recent Items** | Persisted list of recently opened files and folders |

### 2.4 Constraints

| Constraint | Implication |
|------------|-------------|
| **Local-first** | No network dependency in core editing path |
| **Cross-platform** | macOS, Windows, Linux support required |
| **Tauri V1 shell** | Desktop packaging and OS integration via Tauri |
| **Rust core ownership** | Parsing, editing semantics, serialization, recovery, export logic in Rust |
| **Markdown fidelity** | Supported constructs must survive edit-save-reopen cycles |

---

## 3. User Scenarios & Testing

### 3.1 Scenario: Opening an Existing Document

**User Story**: As a writer, I open an existing `.md` file and immediately continue writing.

**Testable Steps**:
1. Launch app → File picker or drag-drop `.md` file → Document loads with live rendering
2. Cursor appears at last position or top of document
3. User types → Changes appear in rendered form immediately
4. User saves → File on disk contains valid Markdown

**Acceptance Criteria**: Supported Markdown preserves appearance after save/reopen.

### 3.2 Scenario: Creating a Structured Document

**User Story**: As a developer, I create a README with headings, bullet lists, and a code fence.

**Testable Steps**:
1. New file → Empty document with placeholder or cursor
2. Type `# Main Title` → Renders as H1 heading
3. Type `## Section` → Renders as H2
4. Type `- item 1` then Enter → New list item appears
5. Type `- item 2` then Enter on empty → List exits
6. Type ``` then Enter → Code fence begins with syntax highlighting

**Acceptance Criteria**: Structural editing feels consistent; no cursor traps.

### 3.3 Scenario: Managing Links and Images

**User Story**: As a product manager, I insert links and images with correct relative paths.

**Testable Steps**:
1. Type `[text](url)` → Renders as clickable link
2. Insert image via picker/drag/paste → Image renders with relative path
3. Image at `../assets/photo.png` → Renders correctly when document is saved and reopened
4. Broken image path → Shows clear but unobtrusive error state

**Acceptance Criteria**: Asset paths remain correct; broken assets are visually indicated.

### 3.4 Scenario: Task List Management

**User Story**: As a writer, I create and toggle task list items.

**Testable Steps**:
1. Type `- [ ] Task item` → Renders with unchecked checkbox
2. Click checkbox or use shortcut → Toggles to `- [x]`
3. Toggle preserves valid Markdown syntax
4. Reopen file → Checkbox state persists

**Acceptance Criteria**: Task state persists correctly in Markdown source.

### 3.5 Scenario: Focus Mode Writing

**User Story**: As a writer in flow state, I use focus mode to reduce distractions.

**Testable Steps**:
1. Toggle focus mode → Non-current paragraphs/sections dim or de-emphasize
2. Navigate document → Focus follows cursor
3. Typewriter mode enabled → Active line/paragraph stays vertically centered
4. Both modes remain smooth during navigation and editing

**Acceptance Criteria**: Focus/typewriter modes materially reduce visual distraction without lag.

### 3.6 Scenario: Export to HTML/PDF

**User Story**: As a technical author, I export a document to share or print.

**Testable Steps**:
1. Export dialog → Select HTML or PDF format
2. HTML: Standalone or linked-assets mode
3. PDF: Preserves structure, readable layout, configurable page size/margins
4. Open exported file → Headings, lists, code blocks, tables, images render correctly

**Acceptance Criteria**: Export produces production-usable output for ordinary documents.

### 3.7 Scenario: Crash Recovery

**User Story**: As a user, my app crashes but my work is recoverable.

**Testable Steps**:
1. Edit document → Unsaved changes accumulate
2. App crashes or force-closes → Recovery snapshot persisted
3. Reopen app → Recovery prompt appears
4. User restores → Unsaved content is recovered

**Acceptance Criteria**: No normal-flow data loss; recovery works in tested scenarios.

---

## 4. Functional Requirements

### 4.1 File Operations

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-001 | New file creates untitled document | Untitled doc opens; save dialog suggests `.md` extension |
| FR-002 | Open file via picker or drag-drop | File content loads with live rendering |
| FR-003 | Open folder as workspace | Sidebar shows Markdown files in tree |
| FR-004 | Manual save preserves valid UTF-8 Markdown | Roundtrip parse-serialize produces identical output for supported constructs |
| FR-005 | Auto-save configurable on/off with debounce | Dirty state indicator visible; file updated after debounce interval |
| FR-006 | Crash recovery restores unsaved content | Recovery prompt appears; content restored correctly |
| FR-007 | External change detection | User can reload, preserve current buffer, or defer decision |

### 4.2 Core Editing Experience

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-008 | Single-pane live rendering | One editing canvas; no mode switch to view formatted output |
| FR-009 | Headings visually differentiated by level | H1-H6 render with distinct sizing/weight while editing |
| FR-010 | Emphasis renders inline with natural cursor/selection | Cursor moves predictably around bold/italic/code spans |
| FR-011 | Links visually appear as links while editable | Link text and target editable without raw syntax confusion |
| FR-012 | Smart list behavior | Enter continues list; Enter on empty item exits; Backspace at boundaries degrades predictably |
| FR-013 | Task list checkboxes toggle correctly | `- [ ]` ↔ `- [x]` toggle preserves Markdown |
| FR-014 | Blockquotes render visually with easy entry/exit | `>` prefix visible; Enter continues quote; Backspace at start exits |
| FR-015 | Code fences render with syntax highlighting | Language tag preserved; highlighting applied |
| FR-016 | Tables render clearly | Table editing prioritizes correctness; safe constrained editing model |
| FR-017 | Images insert with relative paths | Picker/drag/paste supported; broken images show error state |
| FR-018 | Paste behavior is predictable | Plain text pastes as text; rich text converts to Markdown on best-effort |
| FR-019 | Undo/redo works for all editing operations | Session-level undo/redo; structural operations participate consistently |

### 4.3 Markdown Support

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-020 | Required syntax: H1-H6, bold, italic, strikethrough, inline code, fenced code, ordered/unordered/task lists, blockquotes, links, images, HR, tables, frontmatter | Each construct renders and roundtrips correctly |
| FR-021 | Markdown flavor: CommonMark baseline + GFM (tables, task lists, strikethrough, autolinks) | Parsed and rendered per flavor specification |
| FR-022 | Serialization fidelity | Supported constructs survive edit-save-reopen cycle |

### 4.4 Workspace and Navigation

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-023 | File tree supports create/rename/delete | Operations update tree and disk correctly |
| FR-024 | Recent items persist locally | Recently opened files/folders appear in UI |
| FR-025 | Find/replace with next/previous, replace one/all, case-sensitive option | Search highlights matches; replace updates document |
| FR-026 | Outline/TOC panel from document headings | Click navigates to section; panel hide/show works |

### 4.5 Display and Themes

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-027 | Light and dark themes | Themes apply correctly; typography readable |
| FR-028 | Focus mode dims non-current content | Materially reduces distraction; toggle works |
| FR-029 | Typewriter mode centers active line | Active line stays centered during editing |
| FR-030 | Content width and typography settings | Font size, content width, line spacing adjustable |

### 4.6 Export

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-031 | HTML export with standalone or linked-assets mode | Output preserves document structure |
| FR-032 | PDF export with configurable page size/margins | Output is readable and properly paginated |
| FR-033 | Export architecture behind interface boundary | New export formats can be added without core changes |

### 4.7 Preferences

| ID | Requirement | Testable Criterion |
|----|-------------|-------------------|
| FR-034 | Persisted preferences for theme, auto-save, focus/typewriter defaults, editor width/font, export defaults | Settings persist across app restarts |

---

## 5. Success Criteria

### 5.1 User Experience Criteria

| Criterion | Measure |
|-----------|---------|
| **Writing feels uninterrupted** | User feedback: "I forget I'm editing Markdown" |
| **Smoothness vs raw editing** | User feedback: "Smoother than editing raw `.md` in a code editor" |
| **File fidelity** | User feedback: "Does not break my files" — zero data loss reports for supported constructs |
| **Authoring ergonomics** | Images, tables, lists, export work without friction |
| **Calmness** | User feedback: "Calm enough for long-form writing" |

### 5.2 Technical Criteria

| Criterion | Target |
|-----------|--------|
| **Startup time** | Cold start under 2 seconds on mainstream modern laptop |
| **Typing responsiveness** | No visible lag during ordinary editing on typical documents under 1 MB |
| **Large document usability** | 5 MB documents remain usable with graceful degradation |
| **Crash-free rate** | Stable releases target above 99% crash-free sessions |
| **Safe writes** | Partial writes avoided via temp-file replacement strategy |

### 5.3 Open-Source Maintainability Criteria

| Criterion | Measure |
|-----------|---------|
| **Contributor onboarding** | Contributors can understand repo quickly from docs |
| **Editing bug reproducibility** | Editing bugs are reproducible via regression tests |
| **Cross-platform stability** | Releases stable across macOS, Windows, Linux |

---

## 6. Key Entities

### 6.1 Domain Entities

| Entity | Responsibility |
|--------|----------------|
| **Document** | Represents the Markdown document with semantic structure |
| **Buffer** | In-memory text representation with dirty state tracking |
| **Selection** | Cursor position and selection range |
| **Workspace** | Folder-based context with file tree |
| **FileNode** | File or folder in workspace tree |
| **Outline** | Heading structure extracted from document |
| **ExportJob** | Export request with format and options |
| **RecoverySnapshot** | Autosaved state for crash recovery |
| **Settings** | User preferences and configuration |

### 6.2 Service Entities (Public Boundaries)

| Service | Operations |
|---------|-----------|
| **DocumentService** | open, save, reload, get_content, get_outline |
| **EditorService** | apply_command, get_selection, set_selection, undo, redo |
| **WorkspaceService** | open_folder, list_files, create_file, rename_file, delete_file |
| **ExportService** | export_html, export_pdf |
| **SettingsService** | get_settings, update_settings |
| **RecoveryService** | create_snapshot, restore_snapshot, list_snapshots |

### 6.3 Editor Commands

| Command Category | Examples |
|-----------------|---------|
| **Insert** | InsertText, InsertHeading, InsertList, InsertCodeBlock, InsertTable, InsertImage, InsertLink |
| **Delete** | DeleteBackward, DeleteForward, DeleteLine |
| **Transform** | ToggleBold, ToggleItalic, ToggleStrikethrough, ToggleTask, Indent, Outdent |
| **Navigation** | MoveCursor, MoveSelection, ScrollTo |
| **Structural** | EnterList, ExitList, EnterQuote, ExitQuote, EnterCodeBlock, ExitCodeBlock |

---

## 7. Architecture Boundaries

### 7.1 Rust-Owned Logic (Non-Negotiable)

- Markdown parsing and semantic representation
- Cursor and selection semantics
- Structural editing behavior
- Serialization and roundtrip fidelity
- Save, auto-save, and crash recovery
- Export correctness

### 7.2 UI Layer Responsibility

- Render derived state from Rust services
- Dispatch user commands to Rust services
- Handle focus/typewriter presentation
- Apply theme tokens

### 7.3 Public API Style

Narrow service interfaces preferred over exposing unstable internal types:
- Synchronous queries for state
- Command dispatch for mutations
- Event notification for external changes

---

## 8. Out-of-Scope for MVP

- Real-time collaboration
- Cloud sync
- Plugin marketplace
- Mobile app
- AI writing features
- Mermaid and advanced diagrams
- Full DOCX/EPUB support
- Database-backed note graph
- Workspace complexity comparable to IDEs
