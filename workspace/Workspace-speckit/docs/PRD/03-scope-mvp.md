# Scope and MVP

## 8. Scope

### 8.1 MVP In Scope

* desktop app for macOS, Windows, Linux
* open, edit, save `.md` files
* open a folder as a workspace
* single-pane live Markdown editing
* rendered support for headings, emphasis, links, images, lists, task lists, code fences, quotes, horizontal rules, tables
* smart editing behavior for lists, quotes, and basic structure transitions
* in-document search and replace
* recent files and recent folders
* theme support: light and dark minimum
* focus mode
* typewriter mode
* HTML export
* PDF export
* auto-save and crash recovery
* code fence syntax highlighting
* relative asset path support
* basic outline / table-of-contents panel

### 8.2 Explicitly Out of Scope for MVP

* real-time collaboration
* cloud sync
* plugin marketplace
* mobile app
* AI writing features
* Mermaid and advanced diagrams
* full DOCX/EPUB support
* database-backed note graph
* workspace complexity comparable to IDEs

### 8.3 Stretch Goals

* frontmatter helper UI
* local document history
* math rendering
* custom CSS support
* improved paste-from-rich-text conversion

---

## 8.2 MVP Acceptance Criteria

### 8.5.1 Performance Thresholds

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Cold startup (empty doc) | < 2 seconds | Time from launch to ready-to-type |
| Cold startup (1MB doc) | < 3 seconds | Time from launch to document rendered |
| Hot reload (file open after first) | < 500ms | Second+ file open latency |
| Edit-to-render latency | < 100ms | Time from keystroke to visual update |
| Save operation | < 200ms | Time from Ctrl+S to file written |
| PDF export (10 pages) | < 5 seconds | Export start to file complete |
| Memory (idle, 10 docs open) | < 300MB | Resident memory usage |
| Large document (5MB) scrolling | 60 FPS | Smooth scroll without jank |

### 8.5.2 Functional Acceptance

**File Operations (FR-001 to FR-007)**
- [ ] New document creation and save-as work correctly
- [ ] Open existing .md files without corruption
- [ ] Workspace folder opens and displays file tree
- [ ] Recent files list persists across sessions

**Editor Behaviors (FR-008 to FR-022)**
- [ ] Live rendering matches CommonMark specification
- [ ] Smart Enter/Backspace for lists and quotes
- [ ] All keyboard shortcuts work as documented
- [ ] Undo/redo maintains document integrity

**Export (FR-030 to FR-033)**
- [ ] HTML export produces valid, self-contained HTML
- [ ] PDF export matches visual appearance
- [ ] Code fences render with syntax highlighting

**Recovery & Safety (FR-005, FR-006, FR-007)**
- [ ] Autosave triggers without data loss
- [ ] Crash recovery restores last known state
- [ ] External file changes detected and prompted

### 8.5.3 Visual Checkpoints

- [ ] Dark/light theme toggle works instantly
- [ ] Focus mode hides all chrome except current paragraph
- [ ] Typewriter mode keeps cursor at vertical center
- [ ] Outline panel shows correct heading hierarchy
- [ ] Code blocks display syntax highlighting

### 8.5.4 Accessibility Requirements

- [ ] Keyboard navigation works for all features
- [ ] Color contrast meets WCAG AA (4.5:1 for text)
- [ ] Screen reader announces document structure
- [ ] Focus indicators are visible

### 8.5.5 Platform Requirements

- [ ] Builds successfully on macOS (Apple Silicon + Intel)
- [ ] Builds successfully on Windows (x64)
- [ ] Builds successfully on Linux (AppImage/deb)
- [ ] .dmg/.msi/.AppImage installers work

---

## 9. What Must Feel Excellent in V1

For a Typora-like editor, some workflows matter disproportionately more than others.

The MVP should optimize aggressively for these:

1. opening an existing Markdown document and continuing writing immediately
2. creating a clean structured document with headings and lists
3. inserting and managing links
4. pasting content from web/docs/apps without making a mess
5. inserting images with correct relative paths
6. writing technical docs with code fences and tables
7. exporting a polished PDF or HTML file
8. recovering safely from accidental close or crash

A feature may exist and still fail if the interaction is awkward. Therefore these flows need interaction-level tests and polish.
