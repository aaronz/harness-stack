# RustNote - Feature Specification

## Document Metadata

| Field | Value |
|-------|-------|
| Feature Name | RustNote - WYSIWYG Markdown Editor |
| Short Name | rustnote-wysiwyg-editor |
| Version | 1.0 |
| Status | Draft |
| Created | 2026-04-11 |

---

## 1. Feature Overview

RustNote is a desktop-first, cross-platform WYSIWYG Markdown editor that delivers a Typora-like seamless live-preview writing experience while leveraging Rust for performance, stability, and maintainability. The editor targets writers, developers, technical authors, students, and knowledge workers who want a modern Markdown editor that is simpler than IDE-style editors but more polished than basic text tools.

### 1.1 Problem Statement

Users face significant frustrations with existing Markdown editors:

- Split preview and source editing interrupts writing flow
- Many editors are too heavy or too developer-centric
- Large Markdown files become slow or visually unstable
- Export quality is inconsistent across tools
- WYSIWYG behavior often breaks Markdown predictability
- File management for local Markdown projects is clumsy

### 1.2 Solution Overview

RustNote solves these problems by offering:

- A predictable live-render editing model that preserves Markdown integrity
- High performance through Rust-based architecture
- Polished desktop experience with native feel
- Professional export capabilities (HTML, PDF)
- Local-first document management

---

## 2. User Scenarios and Testing

### 2.1 Primary User Scenarios

#### Scenario 1: Technical Writer Creates Documentation

**Actor:** Sarah, a technical writer at a software company

**Goal:** Write and publish product documentation

**Flow:**

1. Sarah launches RustNote and creates a new Markdown document
2. She writes documentation with headings, tables, code blocks, and images
3. She organizes files using the workspace sidebar
4. She exports the document to HTML and PDF
5. She switches between light and dark themes

**Validation:** Document exports correctly with all formatting preserved

#### Scenario 2: Developer Writes README

**Actor:** Alex, a software developer

**Goal:** Create project README and documentation

**Flow:**

1. Alex opens a project folder as workspace
2. He edits README.md with live formatting
3. He inserts code blocks with syntax highlighting
4. He adds images and links
5. He uses keyboard shortcuts for efficient editing

**Validation:** Code blocks render with proper syntax highlighting

#### Scenario 3: Knowledge Worker Takes Notes

**Pat, a product manager)

**Goal:** Take meeting notes and organize thoughts

**Flow:**

1. Pat creates new notes using keyboard shortcuts
2. She uses task lists to track action items
3. She applies focus mode for distraction-free writing
4. She searches within her document
5. She enables auto-save for safety

**Validation:** Auto-save preserves work across crashes

### 2.2 Edge Case Scenarios

| Scenario | Description | Expected Behavior |
|----------|-------------|------------------|
| Large File Edit | Open and edit 10,000+ line Markdown file | Remains responsive with < 100ms latency |
| External File Change | File modified by external editor while open | Detect and prompt user for reload |
| Crash Recovery | App crashes with unsaved changes | Restore document from auto-save |
| Invalid Markdown | Paste content with malformed Markdown | Preserve raw content, don't corrupt |
| Missing Image | Reference image file not found | Show placeholder, don't break rendering |

---

## 3. Functional Requirements

### 3.1 Document Lifecycle Management

#### FR-001: Create New Document

**Requirement:** User can create a new untitled Markdown document

**Testable Criteria:**

- New document opens with empty content
- Default file extension is .md
- Unsaved state is visibly indicated

#### FR-002: Open Existing Document

**Requirement:** User can open a Markdown file from disk

**Testable Criteria:**

- File picker shows .md files
- Opens within 2 seconds for files under 1MB
- Content renders correctly after opening

#### FR-003: Save Document

**Requirement:** User can save document to selected file path

**Testable Criteria:**

- Save dialog allows path selection
- File saves as valid Markdown
- Unsaved indicator clears after save

#### FR-004: Auto-Save

**Requirement:** Auto-save preserves unsaved changes

**Testable Criteria:**

- Auto-save triggers at configurable interval
- Recovered content matches pre-crash state
- User can disable auto-save

#### FR-005: Recent Files

**Requirement:** App shows recent documents

**Testable Criteria:**

- Recent files appear on launch
- Clicking opens the file
- Configurable history limit

### 3.2 Live Preview Editing

#### FR-006: Live Markdown Rendering

**Requirement:** Markdown syntax renders inline while editing

**Testable Criteria:**

- Headings show as styled headings
- Bold, italic, links render visually
- Lists render with proper indentation
- Tables render in grid format
- Code blocks show with syntax highlighting

#### FR-007: Source Integrity

**Requirement:** Underlying Markdown remains valid

**Testable Criteria:**

- Can view raw Markdown source
- Serialized content is valid Markdown
- No silent corruption of structure

#### FR-008: Cursor Behavior

**Requirement:** Cursor moves intuitively through rendered content

**Testable Criteria:**

- Arrow keys move through elements predictably
- Selection follows visual boundaries
- Click to position works correctly

### 3.3 Text Editing Operations

#### FR-009: Standard Editing

**Requirement:** Basic text editing operations work

**Testable Criteria:**

- Undo/redo functions correctly
- Cut/copy/paste work as expected
- Select all selects entire document
- Multi-line selection works

#### FR-010: Markdown Shortcuts

**Requirement:** Keyboard shortcuts apply formatting

**Testable Criteria:**

- Ctrl+B makes text bold
- Ctrl+I makes text italic
-Ctrl+` makes inline code
- Shortcuts work for all core syntax

#### FR-011: Smart Editing

**Requirement:** Enter and Backspace behave smartly

**Testable Criteria:**

- Enter continues bullet lists
- Enter on empty list item exits list
- Backspace joins list items predictably

### 3.4 Markdown Syntax Support

#### FR-012: Core Syntax

**Requirement:** Support all CommonMark syntax

**Testable Criteria:**

- Headings H1-H6 render correctly
- Bold, italic, strikethrough work
- Ordered and unordered lists render
- Task lists show checkboxes
- Blockquotes display styled
- Links and images render
- Horizontal rules appear
- Tables render in grid format

#### FR-013: Code Blocks

**Requirement:** Code fences show syntax highlighting

**Testable Criteria:**

- Language detection works
- Common languages highlight
- Copy code button works

#### FR-014: Tables

**Requirement:** Tables are editable and render correctly

**Testable Criteria:**

- Table insertion works
- Cells are editable
- Row/column operations work

### 3.5 File and Workspace Management

#### FR-015: Workspace Sidebar

**Requirement:** User can open folder as workspace

**Testable Criteria:**

- Folder opens as workspace
- File tree displays Markdown files
- Can create/rename/delete files
- Folder creation works

#### FR-016: External Change Detection

**Requirement:** Detect external file changes

**Testable Criteria:**

- Change notification appears
- User can choose reload behavior

### 3.6 Navigation and Search

#### FR-017: In-Document Search

**Requirement:** Find and replace within document

**Testable Criteria:**

- Find locates text
- Find next/previous works
- Replace works
- Case-sensitive option works

### 3.7 Visual Presentation

#### FR-018: Themes

**Requirement:** Support light and dark themes

**Testable Criteria:**

- Light theme applies light colors
- Dark theme applies dark colors
- Theme switch is immediate

#### FR-019: Focus Mode

**Requirement:** Hide non-essential UI

**Testable Criteria:**

- Sidebar hides
- Toolbar minimizes
- Single section focuses

#### FR-020: Typewriter Mode

**Requirement:** Keep active line centered

**Testable Criteria:**

- Active line stays centered
- Smooth scrolling works

#### FR-021: Typography Controls

**Requirement:** Customize text appearance

**Testable Criteria:**

- Font family selectable
- Font size adjustable
- Line height configurable

### 3.8 Import and Export

#### FR-022: HTML Export

**Requirement:** Export document to HTML

**Testable Criteria:**

- HTML file generates correctly
- All formatting preserved
- Standalone file works

#### FR-023: PDF Export

**Testable Criteria:**

- PDF generates correctly
- Formatting preserved
- Images included

### 3.9 Settings

#### FR-024: Preferences

**Testable Criteria:**

- Theme setting works
- Auto-save setting works
- Font settings apply
- Export defaults configurable

### 3.10 Reliability

#### FR-025: Crash Recovery

**Testable Criteria:**

- Unsaved changes restore
- Recovery flow works
- Snapshot interval works

---

## 4. Success Criteria

### 4.1 Product Metrics

| Metric | Target |
|--------|--------|
| Weekly Active Writers | Users can create and edit documents |
| Documents per User | 5+ documents created weekly |
| Export Usage Rate | 20%+ of users export documents |
| Retention (7-day) | 60%+ return rate |
| Retention (30-day) | 40%+ return rate |

### 4.2 Quality Metrics

| Metric | Target |
|--------|--------|
| Crash-Free Session Rate | 99%+ |
| Median Startup Time | < 2 seconds |
| Typing Latency | < 50ms |
| Recovery Success Rate | 95%+ |
| Save Failure Rate | < 0.1% |

### 4.3 User Experience Metrics

| Metric | Target |
|--------|--------|
| Time to First Document | < 1 minute |
| Time to Export Complete | < 30 seconds |
| Writing Satisfaction | 4+ / 5 rating |
| Performance Satisfaction | 4+ / 5 rating |

---

## 5. Key Entities

### 5.1 Document

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique document identifier |
| title | String | Document title (from filename) |
| content | String | Raw Markdown content |
| filePath | Path | File system path |
| isDirty | Boolean | Has unsaved changes |
| lastSaved | Timestamp | Last save time |

### 5.2 Workspace

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique workspace ID |
| rootPath | Path | Root folder path |
| name | String | Workspace name |
| files | File[] | List of files |

### 5.3 Theme

| Field | Type | Description |
|-------|------|-------------|
| id | String | Theme identifier |
| name | String | Display name |
| isDark | Boolean | Dark mode flag |
| colors | ColorScheme | Theme colors |

### 5.4 ExportJob

| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Job identifier |
| documentId | UUID | Source document |
| format | Enum | HTML, PDF |
| status | Enum | Pending, Running, Complete, Failed |
| outputPath | Path | Result file path |

### 5.5 Preferences

| Field | Type | Description |
|-------|------|-------------|
| theme | String | Selected theme |
| autoSaveInterval | Integer | Seconds between saves |
| fontFamily | String | Font family name |
| fontSize | Integer | Font size in pixels |
| lineHeight | Float | Line height multiplier |

---

## 6. Assumptions

### 6.1 Technical Assumptions

- Modern hardware completes cold start in under 2 seconds
- CommonMark + GFM syntax covers 95%+ of user documents
- Tauri provides adequate cross-platform support
- Local file system access is reliable

### 6.2 User Assumptions

- Users know basic Markdown syntax
- Users have at least one folder of Markdown files
- Users prefer local storage over cloud

### 6.3 Scope Exclusions (MVP)

- Real-time collaboration
- Cloud sync
- Plugin system
- DOCX/EPUB export
- Math rendering
- Mermaid diagrams

---

## 7. Dependencies

### 7.1 External Dependencies

- Tauri framework for desktop shell
- Markdown parser ( pulldown-cmark or similar)
- Syntax highlighter (syntect or similar)

### 7.2 Internal Dependencies

- Document lifecycle depends on file I/O
- Live rendering depends on parser
- Export depends on document model

---

## 8. Open Questions

### Q1: Target User Priority

**Context:** PRD lists multiple user segments (technical writers, developers, knowledge workers, students)

**What we need to know:** Which user segment should be prioritized for MVP?

| Option | Answer | Implications |
|--------|--------|--------------|
| A | Developers | Focus on code blocks, syntax highlighting, README workflows |
| B | Technical Writers | Focus on tables, export, professional features |
| C | Knowledge Workers | Focus on simplicity, quick capture, task lists |
| D | Balanced | Equal focus on all segments |

**Recommendation:** A (Developers) - They provide early feedback and represent the core Markdown use case

---

### Q2: Table Editing Depth

**Context:** PRD mentions table editing but doesn't specify depth

**What we need to know:** How interactive should table editing be?

| Option | Answer | Implications |
|--------|--------|--------------|
| A | Basic | Insert with dialog, cell editing in source | MVP scope |
| B | Intermediate | Visual cell editing, add row/col buttons | Additional work |
| C | Full | Excel-like grid editing | Post-MVP |

**Recommendation:** B (Intermediate) - Balances effort with user value

---

### Q3: Math Support Timing

**Context:** PRD mentions math as optional MVP stretch goal

**What we need to know:** Is math support needed in MVP?

| Option | Answer | Implications |
|--------|--------|--------------|
| A | MVP | Include KaTeX rendering | Additional complexity |
| B | Post-MVP | Defer to future release | Simpler MVP |
| C | Plugin | Enable via plugin system | Requires plugin architecture |

**Recommendation:** B (Post-MVP) - Keep MVP focused on core editor experience

---

## 9. Related Artifacts

- Gap Analysis: ./gap-analysis.md
- Constitution: ./constitution.md
- Implementation Plan: ./plan.md (pending)
- Task List: ./tasks.md (pending)

---

## 10. Revision History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-11 | Initial specification from PRD |