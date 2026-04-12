# RustNote MVP Test Plan Specification

**Project:** RustNote - Typora-like WYSIWYM Markdown Editor  
**Type:** Quality Assurance Specification  
**Version:** 3.1  
**Date:** 2026-04-12  
**Status:** Draft

---

## 1. Overview

### 1.1 Purpose

This specification defines the comprehensive testing strategy and quality gates for RustNote MVP, a Typora-like WYSIWYM Markdown editor. It provides detailed test requirements, acceptance criteria, and execution guidelines to ensure product quality meets enterprise standards.

### 1.2 Quality Objectives

| Objective | Description | Target |
|-----------|-------------|--------|
| **Correctness** | All Markdown constructs render and serialize accurately | 100% spec compliance |
| **Fidelity** | Edit-save-reopen cycles preserve document integrity | Zero data corruption |
| **Performance** | Non-functional requirements are met under load | All NFR thresholds pass |
| **Stability** | Application remains reliable under adverse conditions | 99%+ crash-free sessions |
| **Security** | User data protected from loss, injection, and unauthorized access | Zero security vulnerabilities |
| **Accessibility** | Product usable by people with disabilities | Full WCAG AA compliance |

---

## 2. User Scenarios & Testing

### 2.1 Primary User Scenarios

#### Scenario 1: Document Author Creates and Edits Technical Documentation

**Actor:** Software developer writing technical documentation  
**Precondition:** Application installed and running  
**Steps:**
1. User creates a new document
2. User types structured Markdown content with headings, lists, code blocks, and links
3. User sees live rendering of all content as they type
4. User saves the document
5. User reopens the document and verifies content is unchanged

**Expected Outcome:** Document content is preserved exactly through save-reload cycle with correct rendering at each step.

---

#### Scenario 2: Document Author Exports Content for Distribution

**Actor:** Technical writer preparing documentation for publishing  
**Precondition:** Document with mixed content (text, code, images) is open  
**Steps:**
1. User exports document to HTML (self-contained)
2. User exports same document to PDF
3. User verifies both exports render correctly and are self-contained

**Expected Outcome:** Exports are valid, self-contained, and visually match the editor's rendering.

---

#### Scenario 3: Writer Uses Focus Mode for Distraction-Free Editing

**Actor:** Technical writer requiring focused writing environment  
**Precondition:** Document with 10+ paragraphs is open  
**Steps:**
1. User enables Focus Mode
2. User observes that only current paragraph is fully visible
3. User navigates between paragraphs
4. User types new content while in Focus Mode
5. User disables Focus Mode

**Expected Outcome:** Focus Mode properly dims non-active content and keeps cursor centered vertically (Typewriter Mode).

---

#### Scenario 4: Application Recovers Work After Unexpected Shutdown

**Actor:** User with unsaved changes experiencing a crash  
**Precondition:** User has made significant edits without saving  
**Steps:**
1. Application crashes or is force-closed
2. User relaunches application
3. User is presented with recovery option
4. User chooses to restore or discard changes

**Expected Outcome:** User can recover unsaved work, or cleanly start fresh if they choose to discard.

---

#### Scenario 5: User with Accessibility Needs Navigates and Edits Document

**Actor:** User relying on keyboard navigation and screen reader  
**Precondition:** Application running with accessibility tools active  
**Steps:**
1. User tabs through the interface
2. User opens a file using keyboard shortcuts
3. User navigates through document structure using arrow keys
4. User edits content and saves using keyboard shortcuts

**Expected Outcome:** All functionality accessible via keyboard and properly announced by screen readers.

---

### 2.2 Testing Scope

#### In Scope

| Category | Items |
|----------|-------|
| **File Operations** | New, open, save, save-as, recent files, workspaces |
| **Editor Behaviors** | Live rendering, smart Enter/Backspace, undo/redo, keyboard shortcuts |
| **Content Types** | Headings, lists, blockquotes, code fences, tables, links, images, task lists |
| **Export** | HTML export, PDF export |
| **Recovery** | Autosave, crash recovery, external change detection |
| **UI/UX** | Themes, focus mode, typewriter mode, outline panel |
| **Platforms** | macOS (Apple Silicon + Intel), Windows (x64), Linux (AppImage/deb) |

#### Out of Scope (Future Features)

| Category | Reason |
|----------|--------|
| Real-time collaboration | Future roadmap item |
| Cloud sync | Future roadmap item |
| Plugin marketplace | Future roadmap item |
| Mobile application | Future roadmap item |
| AI writing features | Future roadmap item |
| Mermaid/diagram rendering | Future roadmap item |
| DOCX/EPUB import | Future roadmap item |

---

## 3. Functional Requirements

### 3.1 Unit Test Requirements

#### Parser Module

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T001 | Markdown parsing accuracy | All Markdown constructs (headings H1-H6, emphasis, code spans, links, images, lists, blockquotes, tables, code fences, horizontal rules, frontmatter) parse correctly to AST |
| FR-T002 | Serialization fidelity | Roundtrip parsing-serialization produces byte-for-byte identical output for all Markdown constructs |
| FR-T003 | Malformed input handling | Graceful degradation when parsing malformed input (unclosed emphasis, invalid UTF-8) |
| FR-T004 | Deep nesting support | Parser handles 100+ levels of nested structures without stack overflow |

#### Editor Engine

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T010 | Cursor movement accuracy | Cursor correctly moves within and across all inline and block elements (emphasis, links, code spans, tables, images) |
| FR-T011 | Selection behavior | Double-click selects word, triple-click selects line, shift+arrow extends selection correctly |
| FR-T012 | Smart Enter behavior | Enter key contextually continues lists, blockquotes, headings with proper indentation |
| FR-T013 | Smart Backspace behavior | Backspace at structural boundaries (list start, blockquote start) exits the structure appropriately |
| FR-T014 | Tab/Shift+Tab in lists | Tab indents list items, Shift+Tab outdents list items |
| FR-T015 | IME composition support | Chinese/IME input does not cause cursor jumping during composition |

#### Buffer/Content Management

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T020 | Insert/delete operations | Insert and delete at any position maintains document integrity |
| FR-T021 | Unicode content handling | Full Unicode support including CJK characters and emoji |
| FR-T022 | Large document performance | 5MB document operations complete in under 100ms |
| FR-T023 | Concurrent edit safety | No race conditions during rapid typing |
| FR-T024 | UTF-8 validation | Invalid UTF-8 sequences are rejected gracefully |

#### Settings & Recovery

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T030 | Settings persistence | User settings persist across application restarts |
| FR-T031 | Settings corruption handling | Corrupted settings file falls back to defaults |
| FR-T032 | Snapshot creation | Document snapshots created automatically during editing |
| FR-T033 | Recovery from snapshot | Documents can be fully restored from snapshots after crash |
| FR-T034 | Stale snapshot cleanup | Snapshots older than 7 days are automatically removed |

---

### 3.2 Integration Test Requirements

#### File Operations

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T040 | New document creation | New document opens with untitled state |
| FR-T041 | Save to path | Files save correctly to user-selected path |
| FR-T042 | Save-as operation | Save-as creates new file at specified location |
| FR-T043 | Open existing file | Existing .md files load correctly with content displayed |
| FR-T044 | Drag-drop file open | Dragging .md file onto window opens it |
| FR-T045 | Workspace folder open | Folder opens as workspace with file tree in sidebar |
| FR-T046 | Recent files persistence | Recently opened files persist across restarts |
| FR-T047 | Read-only file handling | Attempting to open read-only file shows appropriate error |
| FR-T048 | Unsaved changes prompt | Closing with unsaved changes prompts user |

#### Editor Integration

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T050 | Live render updates | Markdown renders immediately as user types |
| FR-T051 | Cursor position accuracy | Cursor position is visually correct after clicking |
| FR-T052 | Undo/redo functionality | Undo restores previous state, redo restores undone state |
| FR-T053 | Paste plain text | Pasting from plain text sources inserts plain text |
| FR-T054 | Paste rich text | Pasting from rich text sources converts to Markdown |
| FR-T055 | Find in document | Ctrl+F finds and highlights matches in document |

#### Export Integration

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T060 | HTML export validity | Exported HTML is valid and renders correctly |
| FR-T061 | HTML export self-containment | Exported HTML has no external dependencies |
| FR-T062 | HTML export with images | Documents with images export with embedded images |
| FR-T063 | HTML export code highlighting | Code blocks in exports have syntax highlighting |
| FR-T064 | PDF export validity | Exported PDF is valid and opens correctly |
| FR-T065 | PDF export page sizing | PDF respects selected page size (e.g., A4) |
| FR-T066 | Export overwrite prompt | Exporting to existing file prompts for confirmation |
| FR-T067 | Export cancellation | User can cancel export operation in progress |

#### Workspace Integration

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T070 | Create file in workspace | Right-click context menu creates new file |
| FR-T071 | Rename file in workspace | Right-click context menu renames file |
| FR-T072 | Delete file from workspace | Right-click context menu deletes file with confirmation |
| FR-T073 | External change detection | Sidebar refreshes when files change externally |
| FR-T074 | Nested folder navigation | Folders expand to show children in sidebar |

#### External Change Detection

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T080 | External file modification | Prompt shown when open file is modified externally |
| FR-T081 | External file deletion | Error shown when open file is deleted externally |
| FR-T082 | Keep current version | User can keep their version when conflict detected |
| FR-T083 | Reload external version | User can reload when file changed externally |

---

### 3.3 End-to-End Test Requirements

#### Critical User Flows

| ID | Flow | Acceptance Criteria |
|----|------|---------------------|
| FR-T090 | Open and edit existing document | User can open existing .md file, edit, save, and verify persistence |
| FR-T091 | Create structured document | User can create document with headings, lists, links, code blocks |
| FR-T092 | Insert and manage images | User can insert images via clipboard, drag-drop, or menu; images render correctly |
| FR-T093 | Technical documentation with code | User can write and export code-heavy documentation with syntax highlighting |
| FR-T094 | Export workflow | User can export to HTML and PDF with correct formatting |
| FR-T095 | Crash recovery | User can recover unsaved work after simulated crash |
| FR-T096 | Focus mode writing | User can write with distraction-free focus mode enabled |
| FR-T097 | Typewriter mode | Cursor stays vertically centered while typing |

#### Regression Coverage

| ID | Description | Priority |
|----|-------------|----------|
| FR-T100 | Open/save 50 different .md files without corruption | P0 |
| FR-T101 | Edit-save-reopen 20 files with fidelity verification | P0 |
| FR-T102 | Undo/redo 30 operations without crash | P0 |
| FR-T103 | Export 10 files to HTML with validity verification | P0 |
| FR-T104 | Export 10 files to PDF with open verification | P0 |
| FR-T105 | All 20+ Markdown fixtures parse correctly | P1 |
| FR-T106 | Theme toggle works on all platforms | P1 |
| FR-T107 | Find/replace works with regex | P1 |
| FR-T108 | 5MB document scrolls at 60 FPS | P2 |
| FR-T109 | 1000-item list renders correctly | P2 |

---

### 3.4 Visual Regression Requirements

| ID | Checkpoint | Acceptance Criteria |
|----|------------|---------------------|
| FR-T110 | Dark theme rendering | All elements visible with proper contrast ratios |
| FR-T111 | Light theme rendering | All elements visible with proper contrast ratios |
| FR-T112 | Focus mode styling | Only current paragraph fully bright, others dimmed |
| FR-T113 | Typewriter mode centering | Cursor at 40-60% viewport height |
| FR-T114 | Heading hierarchy | H1-H6 are visually distinct |
| FR-T115 | List indentation | Nested lists show correct visual depth |
| FR-T116 | Blockquote styling | Clear visual hierarchy for nested quotes |
| FR-T117 | Code fence highlighting | Syntax colors clearly visible |
| FR-T118 | Table alignment | Clean grid with proper cell alignment |
| FR-T119 | Link styling | Links clearly distinguishable (underlined, colored) |
| FR-T120 | Image placeholder | Correct aspect ratio displayed |
| FR-T121 | Task list checkboxes | Visible and interactive appearance |
| FR-T122 | Outline panel | Correct heading hierarchy displayed |
| FR-T123 | Search highlight | Yellow highlight clearly visible |
| FR-T124 | Selection highlight | Clear visual feedback for selection |
| FR-T125 | Empty state | Helpful message displayed, not blank |
| FR-T126 | Toolbar styling | Minimal and non-intrusive |
| FR-T127 | Scrollbar styling | Subtle and matches theme |

---

### 3.5 Performance Requirements

#### Performance Thresholds

| Metric | Target | Critical Threshold | Measurement Method |
|--------|--------|-------------------|-------------------|
| Cold start (empty) | < 2 seconds | < 3 seconds | Time from launch to ready |
| Cold start (1MB doc) | < 3 seconds | < 5 seconds | Time from launch to render |
| Hot file open | < 500ms | < 1 second | Time to ready after file selected |
| Keystroke to render | < 100ms | < 200ms | Input latency measurement |
| Save operation | < 200ms | < 500ms | Ctrl+S to completion |
| HTML export (10KB) | < 1 second | < 2 seconds | Export duration |
| PDF export (10 pages) | < 5 seconds | < 10 seconds | Export duration |
| Outline update | < 100ms | < 200ms | Sidebar refresh time |
| Theme switch | < 200ms | < 500ms | Full theme application |
| Memory (10 docs idle) | < 300MB | < 500MB | Resident memory usage |
| Scroll (5MB doc) | 60 FPS | 30 FPS | Frame rate during scroll |

#### Scalability Requirements

| Document Size | Expected Behavior |
|---------------|-------------------|
| < 100KB | Full feature set, no degradation |
| 100KB - 1MB | Full feature set, occasional debounce |
| 1MB - 5MB | All features work, occasional pauses < 200ms |
| 5MB - 10MB | Core editing works, live preview toggle disabled |
| > 10MB | Warning shown, degraded mode continues |

---

### 3.6 Security Requirements

#### Input Validation

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T130 | Path traversal prevention | `../etc/passwd` in file paths rejected with error |
| FR-T131 | Null byte injection prevention | Null bytes in filenames rejected |
| FR-T132 | Long path handling | Paths > 4096 chars rejected or truncated safely |
| FR-T133 | Invalid UTF-8 handling | Invalid UTF-8 bytes rejected or sanitized |
| FR-T134 | Control character handling | Control characters stripped or rejected |

#### HTML Sanitization

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T140 | Script injection prevention | `<script>` tags stripped from exports |
| FR-T141 | Event handler prevention | `on*` event handlers stripped |
| FR-T142 | JavaScript URL prevention | `javascript:` URLs stripped |
| FR-T143 | Iframe injection prevention | `<iframe>` tags stripped |
| FR-T144 | Style injection prevention | `<style>` tags stripped |
| FR-T145 | Data URL XSS prevention | `data:` URLs in href stripped |
| FR-T146 | Base tag prevention | `<base>` tags stripped |

#### Export Security

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T150 | HTML export contains no scripts | Exported HTML has zero `<script>` elements |
| FR-T151 | HTML export has no event handlers | No `on*` attributes in exported HTML |
| FR-T152 | HTML export is self-contained | No external resource references |
| FR-T153 | PDF export has no active content | PDF is static, no JavaScript |
| FR-T154 | Export path traversal prevention | Cannot write files outside target directory |

---

### 3.7 Accessibility Requirements

#### Keyboard Navigation

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-T160 | Editor focus | Tab moves focus to editor |
| FR-T161 | Toolbar navigation | Tab from editor moves through toolbar |
| FR-T162 | Sidebar focus | Ctrl+Shift+S moves focus to sidebar |
| FR-T163 | File operations | Ctrl+O opens file dialog |
| FR-T164 | Save operation | Ctrl+S saves file |
| FR-T165 | Find dialog | Ctrl+F opens find dialog |
| FR-T166 | Dialog dismissal | Escape closes dialogs |
| FR-T167 | Outline navigation | Tab in outline panel focuses items |

#### Screen Reader Support

| ID | Element | Expected Announcement |
|----|---------|---------------------|
| FR-T170 | Heading | "Heading level [n], [text]" |
| FR-T171 | Link | "Link, [text], [url]" |
| FR-T172 | Image | "Image, [alt text]" or "Image, no description" |
| FR-T173 | List | "List, [n] items" |
| FR-T174 | Table | "Table, [n] columns, [m] rows" |
| FR-T175 | Code block | "Code block, language: [lang]" |
| FR-T176 | Task list item | "Checkbox, [checked/unchecked], [text]" |

#### Visual Accessibility

| ID | Requirement | Standard |
|----|-------------|----------|
| FR-T180 | Text contrast | 4.5:1 minimum ratio |
| FR-T181 | Large text contrast | 3:1 minimum ratio |
| FR-T182 | Focus indicators | Visible on all interactive elements |
| FR-T183 | Non-color information | Additional cues beyond color provided |

---

## 4. Success Criteria

### 4.1 Quality Metrics

| Criterion | Metric | Target |
|-----------|--------|--------|
| **Test Coverage** | Parser unit test coverage | 100% of Markdown constructs |
| **Fidelity Rate** | Edit-save-reopen fidelity | 100% of documents preserve content exactly |
| **Crash Rate** | Crash-free sessions | > 99% |
| **Performance Pass Rate** | NFR threshold compliance | 100% of metrics meet target |
| **Security Vulnerabilities** | Known vulnerabilities | 0 critical/high severity |
| **Accessibility Compliance** | WCAG AA criteria met | 100% |
| **Regression Rate** | P0 regression tests passing | 100% per release |

### 4.2 Platform Coverage

| Platform | Installation | File Operations | Editor | Export | Accessibility |
|----------|-------------|-----------------|--------|--------|---------------|
| macOS Intel | P0 | P0 | P0 | P0 | P0 (VoiceOver) |
| macOS Apple Silicon | P0 | P0 | P0 | P0 | P0 (VoiceOver) |
| Windows x64 | P0 | P0 | P0 | P0 | P0 (NVDA) |
| Windows ARM | P1 | P0 | P0 | P1 | P1 (NVDA) |
| Linux AppImage | P0 | P0 | P0 | P0 | P1 (Orca) |
| Linux deb | P0 | P0 | P0 | P0 | P1 (Orca) |

**Priority Legend:** P0 = Must pass, P1 = Should pass, P2 = Nice to have

---

## 5. Key Entities

### 5.1 Test Artifacts

| Entity | Description |
|--------|-------------|
| **Unit Test** | Individual function/module test in Rust (cargo test) or JavaScript (vitest) |
| **Integration Test** | Multi-module interaction test via Tauri IPC |
| **E2E Test** | Complete user workflow test via Playwright |
| **Visual Test** | UI regression detection via Playwright + Percy/Ark |
| **Performance Benchmark** | Custom benchmark measuring NFR compliance |
| **Security Test** | Input validation and sanitization test |

### 5.2 Test Data

| Entity | Description |
|--------|-------------|
| **Markdown Fixture** | Sample .md file for parsing/serialization testing |
| **Edge Case Fixture** | Malformed, large, or unicode content for stress testing |
| **Export Fixture** | Expected output files for export validation |
| **Recovery Fixture** | Snapshot data for recovery testing |

### 5.3 Test Infrastructure

| Entity | Description |
|--------|-------------|
| **CI Pipeline** | Automated lint, build, test, and deploy stages |
| **Test Environment** | Dev environment specs (OS, Rust 1.75+, Node.js 20+, Tauri 2.x) |
| **Fuzz Testing** | 24-hour cargo-fuzz runs for parser, serializer, deserializer |

---

## 6. Assumptions

1. **Testing tools availability:** Playwright, vitest, cargo-fuzz, and axe-core are available and compatible with specified versions
2. **Platform accessibility tools:** VoiceOver (macOS), NVDA (Windows), and Orca (Linux) are available for accessibility testing
3. **Performance measurement:** Benchmark infrastructure can accurately measure keystroke latency, FPS, and memory usage
4. **Fuzz testing duration:** 24-hour fuzz runs are sufficient to find major parser/serializer bugs
5. **Visual regression scope:** Visual tests cover all major UI states and themes

---

## 7. Dependencies

| Dependency | Document | Purpose |
|------------|----------|---------|
| Product Definition | 01-product-definition.md | Core thesis and principles |
| Product Invariants | 02-product-invariants.md | Success criteria and invariants |
| Scope & MVP | 03-scope-mvp.md | Acceptance criteria |
| Functional Requirements | 04-functional-requirements.md | FR-001 to FR-034 |
| Architecture | 05-architecture.md | NFR-001 to NFR-012 |
| Implementation Milestones | 08-implementation-milestones.md | Testing guidelines |
| Security | 12-security.md | Security requirements |

---

## 8. Definition of Done

### 8.1 For Test Implementation

A test is considered **Done** when:

- [ ] Test code written and passing
- [ ] Test data/fixtures created
- [ ] Test integrated into CI pipeline
- [ ] No known false positives
- [ ] Documentation updated

### 8.2 For Release

A release is considered **Ready** when:

- [ ] All P0 tests pass
- [ ] All P1 tests pass (>90%)
- [ ] Performance thresholds met
- [ ] Security scan clean
- [ ] No known critical bugs
- [ ] Manual testing complete
- [ ] Installers tested on all platforms
