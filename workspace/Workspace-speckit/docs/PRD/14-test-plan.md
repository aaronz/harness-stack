# RustNote Test Plan

**Project:** RustNote - Typora-like Markdown Editor  
**Version:** 3.1 (MVP)  
**Last Updated:** 2026-04-11  
**Status:** Draft

---

## Table of Contents

1. [Overview](#1-overview)
2. [Test Scope](#2-test-scope)
3. [Test Environment](#3-test-environment)
4. [Unit Test Specifications](#4-unit-test-specifications)
5. [Integration Test Specifications](#5-integration-test-specifications)
6. [E2E Test Specifications](#6-e2e-test-specifications)
7. [Visual Regression Tests](#7-visual-regression-tests)
8. [Performance Test Specifications](#8-performance-test-specifications)
9. [Security Test Specifications](#9-security-test-specifications)
10. [Accessibility Test Specifications](#10-accessibility-test-specifications)
11. [Platform Test Matrix](#11-platform-test-matrix)
12. [Test Data Requirements](#12-test-data-requirements)
13. [CI/CD Pipeline](#13-cicd-pipeline)
14. [Test Execution Schedule](#14-test-execution-schedule)
15. [Risk Register](#15-risk-register)
16. [Definition of Done](#16-definition-of-done)

---

## 1. Overview

### 1.1 Purpose

This test plan defines the comprehensive testing strategy for RustNote MVP, a Typora-like WYSIWYM Markdown editor. It provides detailed test specifications, acceptance criteria, and execution guidelines to ensure product quality.

### 1.2 Test Objectives

1. **Correctness** - All Markdown constructs render and serialize correctly
2. **Fidelity** - Edit-save-reopen cycles preserve document integrity
3. **Performance** - All NFR thresholds are met
4. **Stability** - Crash-free session rate exceeds 99%
5. **Security** - No data loss, injection, or path traversal vulnerabilities
6. **Accessibility** - WCAG AA compliance for keyboard navigation and screen readers

### 1.3 Reference Documents

| Document | Location | Purpose |
|----------|----------|---------|
| Product Definition | 01-product-definition.md | Core thesis and principles |
| Product Invariants | 02-product-invariants.md | Success criteria and invariants |
| Scope & MVP | 03-scope-mvp.md | Acceptance criteria |
| Functional Requirements | 04-functional-requirements.md | FR-001 to FR-034 |
| Architecture | 05-architecture.md | NFR-001 to NFR-012 |
| Testing Strategy | 08-implementation-milestones.md | Existing testing guidelines |
| Security | 12-security.md | Security requirements |

---

## 2. Test Scope

### 2.1 In Scope

| Category | Items |
|----------|-------|
| **File Operations** | New, open, save, save-as, recent files, workspaces |
| **Editor Behaviors** | Live rendering, smart Enter/Backspace, undo/redo, keyboard shortcuts |
| **Content Types** | Headings, lists, blockquotes, code fences, tables, links, images, task lists |
| **Export** | HTML export, PDF export |
| **Recovery** | Autosave, crash recovery, external change detection |
| **UI/UX** | Themes, focus mode, typewriter mode, outline panel |
| **Platforms** | macOS (Apple Silicon + Intel), Windows (x64), Linux (AppImage/deb) |

### 2.2 Out of Scope

| Category | Reason |
|----------|--------|
| Real-time collaboration | Future feature |
| Cloud sync | Future feature |
| Plugin marketplace | Future feature |
| Mobile app | Future feature |
| AI writing features | Future feature |
| Mermaid/diagram rendering | Future feature |
| DOCX/EPUB import | Future feature |

### 2.3 Test Types

| Type | Purpose | Tool |
|------|---------|------|
| Unit | Test individual functions/modules | cargo test, vitest |
| Integration | Test module interactions | cargo test, Tauri IPC |
| E2E | Test complete user workflows | Playwright |
| Visual | Detect UI regressions | Playwright + Percy/Ark |
| Performance | Validate NFRs | custom benchmarks |
| Security | Validate sanitization, path traversal | custom + cargo-fuzz |
| Accessibility | Validate WCAG compliance | axe-core, manual testing |

---

## 3. Test Environment

### 3.1 Development Environment

| Component | Specification |
|-----------|---------------|
| OS | macOS 14+ (Apple Silicon), Windows 11, Ubuntu 22.04 |
| Rust | 1.75+ (stable) |
| Node.js | 20+ LTS |
| Tauri CLI | 2.x |
| Browser (E2E) | Chromium 120+, Firefox 120+, WebKit |

### 3.2 Test Infrastructure

```
rustnote/
├── src-tauri/
│   ├── src/
│   │   ├── parser/
│   │   ├── buffer/
│   │   ├── editor/
│   │   └── services/
│   └── tests/              # Rust integration tests
├── www/
│   ├── src/
│   │   ├── components/
│   │   └── hooks/
│   └── __tests__/          # Vitest unit tests
├── e2e/                     # Playwright E2E tests
├── fixtures/                # Test data
│   ├── markdown/           # 20+ sample .md files
│   ├── edge-cases/         # Malformed, large, unicode
│   ├── export/             # Expected outputs
│   └── recovery/           # Snapshot test cases
└── scripts/
    └── benchmark.ts        # Performance benchmarks
```

### 3.3 Test Data Classification

| Classification | Description | Examples |
|----------------|-------------|----------|
| **Safe** | Public domain, no PII | CommonMark spec examples |
| **Sensitive** | May contain user info | Recovery snapshots |
| **Boundary** | Stress test limits | 10MB documents, 10K list items |

---

## 4. Unit Test Specifications

### 4.1 Parser Module Tests

#### 4.1.1 Markdown Parsing

| Test ID | Test Case | Input | Expected Output |
|---------|-----------|-------|-----------------|
| PAR-001 | Parse headings H1-H6 | `# H1` through `###### H6` | Correct heading levels |
| PAR-002 | Parse emphasis | `**bold**`, `*italic*`, `~~strike~~` | Correct AST nodes |
| PAR-003 | Parse code spans | `` `inline` ``, ` ```block``` ` | Correct monospace rendering |
| PAR-004 | Parse links | `[text](url)` | Correct href and content |
| PAR-005 | Parse images | `![alt](src)` | Correct src and alt |
| PAR-006 | Parse lists (ul) | `- item`, `* item`, `+ item` | Correct list structure |
| PAR-007 | Parse lists (ol) | `1. item`, `a. item` | Correct ordering |
| PAR-008 | Parse task lists | `- [ ] todo`, `- [x] done` | Correct checkbox state |
| PAR-009 | Parse blockquotes | `> quote` | Correct blockquote nesting |
| PAR-010 | Parse tables (GFM) | `\| col \| col \|` | Correct cell alignment |
| PAR-011 | Parse code fences | ` ```rust\ncode\n``` ` | Correct language tag |
| PAR-012 | Parse horizontal rules | `---`, `***`, `___` | Correct HR rendering |
| PAR-013 | Parse frontmatter | `---\nkey: value\n---` | Preserved as text block |
| PAR-014 | Handle malformed input | `**unclosed bold` | Graceful degradation |
| PAR-015 | Handle deep nesting | 100+ nested structures | Parse without stack overflow |

#### 4.1.2 Serialization

| Test ID | Test Case | Input | Expected Output |
|---------|-----------|-------|-----------------|
| SER-001 | Roundtrip headings | `# H1` | `# H1` |
| SER-002 | Roundtrip emphasis | `**bold**` | `**bold**` |
| SER-003 | Roundtrip code | `` `code` `` | `` `code` `` |
| SER-004 | Roundtrip links | `[text](url)` | `[text](url)` |
| SER-005 | Roundtrip images | `![alt](src)` | `![alt](src)` |
| SER-006 | Roundtrip lists | `- item` | `- item` |
| SER-007 | Roundtrip task lists | `- [x] done` | `- [x] done` |
| SER-008 | Roundtrip blockquotes | `> quote` | `> quote` |
| SER-009 | Roundtrip tables | `\| col \|` | `\| col \|` |
| SER-010 | Roundtrip code fences | ` ```rust\ncode\n``` ` | Language tag preserved |
| SER-011 | Preserve frontmatter | `---\nkey: value\n---` | Frontmatter preserved exactly |
| SER-012 | Preserve entity escaping | `\&amp;` | `\&amp;` (not `&`) |

### 4.2 Editor Engine Tests

#### 4.2.1 Cursor Movement

| Test ID | Test Case | Scenario | Expected |
|---------|-----------|----------|----------|
| CUR-001 | Left arrow at line start | Cursor at position 0 | No movement, stay at 0 |
| CUR-002 | Right arrow at line end | Cursor at line length | No movement, stay at end |
| CUR-003 | Up arrow at doc start | Cursor at position 0 | No movement |
| CUR-004 | Down arrow at doc end | Cursor at last position | No movement |
| CUR-005 | Move across emphasis | `**bold**|` (cursor after 'l') | Correct position |
| CUR-006 | Move across links | `[text](url)|` (cursor after ')') | Correct position |
| CUR-007 | Move across code spans | `` `code`|`` (cursor after 'e') | Correct position |
| CUR-008 | Move across tables | Cell content | Jump cell-to-cell |
| CUR-009 | Move across images | `![alt](url)|` | Jump past image |
| CUR-010 | IME composition | Chinese input | No cursor jump |

#### 4.2.2 Selection

| Test ID | Test Case | Scenario | Expected |
|---------|-----------|----------|----------|
| SEL-001 | Select word | Double-click on word | Word selected |
| SEL-002 | Select line | Triple-click on line | Line selected |
| SEL-003 | Shift+arrow selection | Cursor in middle, shift+right | Extend selection |
| SEL-004 | Select all | Ctrl+A / Cmd+A | Entire doc selected |
| SEL-005 | Selection across emphasis | `**bold**|text` | Selection correct |
| SEL-006 | Selection across links | `[text](url)` | Entire link selected |
| SEL-007 | Selection across code | `` `code` `` | Entire code selected |
| SEL-008 | Selection cleared on click | Any selection, click elsewhere | Selection cleared |

#### 4.2.3 Smart Editing (Enter/Backspace)

| Test ID | Test Case | Input | Expected Output |
|---------|-----------|-------|----------------|
| ENT-001 | Enter at end of list item | `- item|`<br>`Enter` | `- item`<br>`- ` (new item) |
| ENT-002 | Enter on empty list item | `- |`<br>`Enter` | Exit list, new paragraph |
| ENT-003 | Enter in middle of list item | `- item|text`<br>`Enter` | `- item`<br>`- text` |
| ENT-004 | Enter at end of blockquote | `> quote|`<br>`Enter` | `> quote`<br>`> ` |
| ENT-005 | Enter on empty blockquote | `> |`<br>`Enter` | Exit blockquote |
| ENT-006 | Enter at heading | `# H1|`<br>`Enter` | `# H1`<br>`## ` |
| BSP-001 | Backspace at list start | `\|item`<br>`Backspace` | Exit list, join with previous |
| BSP-002 | Backspace at empty list | `- |<br>`Backspace` | Remove empty item |
| BSP-003 | Backspace at blockquote start | `>|item`<br>`Backspace` | Exit blockquote |
| BSP-004 | Backspace at heading start | `## |`<br>`Backspace` | Convert to paragraph |
| TAB-001 | Tab in list | `- item|`<br>`Tab` | `- [ ] item` (indent) |
| TAB-002 | Shift+Tab in list | `- [ ] item|`<br>`Shift+Tab` | `- item` (outdent) |

### 4.3 Buffer Tests

| Test ID | Test Case | Input | Expected |
|---------|-----------|-------|----------|
| BUF-001 | Insert at position | `abc`, insert at 1 | `a|bc` |
| BUF-002 | Delete at position | `abcd`, delete at 1 | `acd` |
| BUF-003 | Replace range | `abcd`, replace 1-2 | `aXDd` |
| BUF-004 | Insert unicode | `abc`, insert `中文` | Correct unicode handling |
| BUF-005 | Insert emoji | `abc`, insert `👍` | Correct emoji handling |
| BUF-006 | Large doc (5MB) | Insert in large file | < 100ms response |
| BUF-007 | Concurrent edits | Simulate rapid typing | No race conditions |
| BUF-008 | UTF-8 validation | Invalid UTF-8 sequence | Rejected gracefully |

### 4.4 Serializer Tests

| Test ID | Test Case | Input | Expected |
|---------|-----------|-------|----------|
| SS-001 | Empty doc | `` | `` |
| SS-002 | Plain text | `Hello world` | `Hello world` |
| SS-003 | Preserve whitespace | `  spaces  ` | `  spaces  ` |
| SS-004 | Line endings CRLF | `line1\r\nline2` | `line1\r\nline2` (on save) |
| SS-005 | Line endings LF | `line1\nline2` | `line1\nline2` (on save) |
| SS-006 | Trailing newline | `content\n` | `content\n` (preserve) |
| SS-007 | No trailing newline | `content` | `content` (no added newline) |

### 4.5 Settings Tests

| Test ID | Test Case | Input | Expected |
|---------|-----------|-------|----------|
| SET-001 | Default settings | Fresh install | Default values loaded |
| SET-002 | Save settings | Change theme | Persist to disk |
| SET-003 | Load settings | Restart app | Previous settings restored |
| SET-004 | Invalid settings | Corrupted JSON | Fall back to defaults |
| SET-005 | Missing settings key | Partial config | Use defaults for missing |

### 4.6 Recovery Tests

| Test ID | Test Case | Scenario | Expected |
|---------|-----------|----------|----------|
| REC-001 | Create snapshot | Edit document | Snapshot created |
| REC-002 | Recover from snapshot | After crash | Document restored |
| REC-003 | Discard snapshot | User chooses discard | Clean state |
| REC-004 | View backup | Recovery prompt | Show backup content |
| REC-005 | Auto-recovery | Crash during edit | Auto-restore on next launch |
| REC-006 | Stale cleanup | > 7 days old | Stale snapshots removed |
| REC-007 | Multiple snapshots | Many edits | Only latest kept |

---

## 5. Integration Test Specifications

### 5.1 File Operations

| Test ID | Test Case | Steps | Expected |
|---------|-----------|-------|----------|
| INT-F001 | Create new file | File → New | Untitled doc opens |
| INT-F002 | Save new file | New → Save → choose path | File exists at path |
| INT-F003 | Save existing file | Edit → Ctrl+S | File updated |
| INT-F004 | Save-as | File → Save As | New file created |
| INT-F005 | Open existing file | File → Open → choose file | Content loaded |
| INT-F006 | Drag-drop open | Drag .md onto window | Content loaded |
| INT-F007 | Open folder as workspace | File → Open Folder | Sidebar shows tree |
| INT-F008 | Recent files persist | Open several files, restart | Recent files shown |
| INT-F009 | File permissions denied | Open read-only file | Error message |
| INT-F010 | Unsaved changes prompt | Edit, try to close | Prompt shown |

### 5.2 Editor Integration

| Test ID | Test Case | Steps | Expected |
|---------|-----------|-------|----------|
| INT-E001 | Live render updates | Type `# Test` | H1 renders immediately |
| INT-E002 | Cursor position correct | Click after bold text | Cursor after bold |
| INT-E003 | Selection visible | Select text | Selection highlighted |
| INT-E004 | Undo restores state | Type, undo | Previous state |
| INT-E005 | Redo after undo | Type, undo, redo | State restored |
| INT-E006 | Paste plain text | Copy from Notepad, paste | Plain text inserted |
| INT-E007 | Paste rich text | Copy from browser | Converted to Markdown |
| INT-E008 | Paste image | Copy image, paste | Image inserted |
| INT-E009 | Drag-drop text | Select, drag | Text moved |
| INT-E010 | Find in document | Ctrl+F, type query | Matches highlighted |

### 5.3 Export Integration

| Test ID | Test Case | Steps | Expected |
|---------|-----------|-------|----------|
| INT-X001 | HTML export basic | Export HTML | Valid HTML file created |
| INT-X002 | HTML export self-contained | Export HTML | No external dependencies |
| INT-X003 | HTML export images | Doc with images | Images embedded |
| INT-X004 | HTML export code highlight | Doc with code | Syntax highlighted |
| INT-X005 | PDF export basic | Export PDF | Valid PDF file created |
| INT-X006 | PDF export page size | Export A4 | Correct page size |
| INT-X007 | PDF export margins | Export with margins | Margins applied |
| INT-X008 | Export overwrite prompt | Export to existing file | Confirmation prompt |
| INT-X009 | Export progress | Export large PDF | Progress shown |
| INT-X010 | Export cancel | Export → cancel | Operation cancelled |

### 5.4 Workspace Integration

| Test ID | Test Case | Steps | Expected |
|---------|-----------|-------|----------|
| INT-W001 | Create file in workspace | Right-click → New File | File created |
| INT-W002 | Rename file | Right-click → Rename | File renamed |
| INT-W003 | Delete file | Right-click → Delete | File deleted, prompt |
| INT-W004 | Create folder | Right-click → New Folder | Folder created |
| INT-W005 | Refresh workspace | External change | Sidebar refreshes |
| INT-W006 | Nested folder expand | Click folder | Children shown |
| INT-W007 | Recent folders | Open several folders | Recent folders shown |
| INT-W008 | External file change | Edit file externally | Notification shown |

### 5.5 External Change Detection

| Test ID | Test Case | Steps | Expected |
|---------|-----------|-------|----------|
| EXT-001 | File modified externally | Modify .md in VS Code | Reload prompt shown |
| EXT-002 | File deleted externally | Delete .md externally | Error shown |
| EXT-003 | File replaced externally | Replace .md externally | Reload prompt |
| EXT-004 | Keep current version | "Keep Current" chosen | No change |
| EXT-005 | Reload external version | "Reload" chosen | New content loaded |

---

## 6. E2E Test Specifications

### 6.1 Critical User Flows

#### E2E-C01: Open Existing Document and Continue Writing

```
1. Launch application
2. File → Open
3. Select existing .md file
4. Verify content loads within 500ms (hot open)
5. Click after last paragraph
6. Type new content
7. Verify live rendering
8. Save (Ctrl+S)
9. Verify save completes within 200ms
10. Reopen file
11. Verify new content persisted
```

#### E2E-C02: Create Structured Document

```
1. Launch application
2. New document
3. Type "# My Document"
4. Press Enter → auto-continue
5. Type "## Introduction"
6. Press Enter
7. Type "- First item"
8. Press Enter
9. Type "- Second item"
10. Press Enter on empty item
11. Verify list exits
12. Type "Here's a [link](https://example.com)"
13. Verify link renders
14. Save document
15. Verify valid Markdown
```

#### E2E-C03: Insert and Manage Images

```
1. Open new document
2. Paste image from clipboard
3. OR drag-drop image file
4. OR insert via menu
5. Verify image renders in editor
6. Verify relative path in source
7. Move document to different folder
8. Verify image still loads
```

#### E2E-C04: Technical Documentation with Code

```
1. Open new document
2. Type "```rust"
3. Press Enter
4. Type code content
5. Press Enter
6. Type "```"
7. Verify syntax highlighting
8. Verify language tag preserved
9. Export to HTML
10. Verify highlighting in export
```

#### E2E-C05: Export Workflow

```
1. Open document with mixed content
2. File → Export → HTML
3. Select "self-contained" option
4. Choose save location
5. Verify HTML opens correctly
6. File → Export → PDF
7. Select page size (A4)
8. Choose save location
9. Verify PDF opens correctly
10. Verify formatting matches editor
```

#### E2E-C06: Crash Recovery

```
1. Open document
2. Make significant edits (don't save)
3. Force close application (simulate crash)
4. Reopen application
5. Verify recovery prompt appears
6. Select "Restore"
7. Verify edits recovered
8. OR select "Discard"
9. Verify clean state
```

#### E2E-C07: Focus Mode Writing

```
1. Open long document (> 10 paragraphs)
2. Enable Focus Mode
3. Verify non-current paragraphs dimmed
4. Navigate to different paragraph
5. Verify current paragraph highlighted
6. Type in current paragraph
7. Verify other content remains dimmed
8. Disable Focus Mode
9. Verify all content equally visible
```

#### E2E-C08: Typewriter Mode

```
1. Open document
2. Enable Typewriter Mode
3. Navigate to middle of document
4. Verify cursor stays vertically centered
5. Scroll through document
6. Verify cursor auto-centers on active line
7. Type new content
8. Verify view adjusts to keep cursor centered
```

### 6.2 Regression Test Suite

These tests must pass before every release:

| Test ID | Priority | Description |
|---------|----------|-------------|
| REG-001 | P0 | Open/save 50 different .md files without corruption |
| REG-002 | P0 | Edit-save-reopen 20 files, verify fidelity |
| REG-003 | P0 | Undo/redo 30 operations without crash |
| REG-004 | P0 | Export 10 files to HTML, verify validity |
| REG-005 | P0 | Export 10 files to PDF, verify opens |
| REG-006 | P1 | All 20+ Markdown fixtures parse correctly |
| REG-007 | P1 | Theme toggle works on all platforms |
| REG-008 | P1 | Find/replace works with regex |
| REG-009 | P2 | 5MB document scrolls at 60 FPS |
| REG-010 | P2 | 1000-item list renders correctly |

---

## 7. Visual Regression Tests

### 7.1 Visual Checkpoints

| Test ID | Checkpoint | Validation |
|---------|------------|------------|
| VIS-001 | Dark theme | All elements visible, no contrast issues |
| VIS-002 | Light theme | All elements visible, no contrast issues |
| VIS-003 | Focus mode - dimmed paragraphs | Only current paragraph full brightness |
| VIS-004 | Focus mode - current paragraph | Correct styling applied |
| VIS-005 | Typewriter mode - cursor centering | Cursor at 40-60% viewport height |
| VIS-006 | Heading hierarchy | H1-H6 visually distinct |
| VIS-007 | List indentation | Nested lists show correct depth |
| VIS-008 | Blockquote styling | Clear visual hierarchy |
| VIS-009 | Code fence highlighting | Syntax colors visible |
| VIS-010 | Table borders | Clean grid alignment |
| VIS-011 | Link styling | Underlined, colored, distinguishable |
| VIS-012 | Image placeholder | Correct aspect ratio box |
| VIS-013 | Task list checkboxes | Visible, interactive appearance |
| VIS-014 | Outline panel | Correct heading hierarchy |
| VIS-015 | Search highlight | Yellow highlight visible |
| VIS-016 | Selection highlight | Clear visual feedback |
| VIS-017 | Error state (broken image) | Obvious but unobtrusive |
| VIS-018 | Empty state (no file) | Helpful message, not blank |
| VIS-019 | Toolbar styling | Minimal, non-intrusive |
| VIS-020 | Scrollbar styling | Subtle, matches theme |

### 7.2 Tools

| Tool | Purpose | Integration |
|------|---------|-------------|
| Playwright Screenshots | Capture baseline | CI |
| Percy/Ark | Visual diff comparison | CI |
| Manual Review | Complex UI validation | PR review |

---

## 8. Performance Test Specifications

### 8.1 Performance Thresholds (NFRs)

| Metric | Target | Critical Threshold | Test Method |
|--------|--------|-------------------|-------------|
| Cold start (empty) | < 2s | < 3s | `time_to_ready` benchmark |
| Cold start (1MB doc) | < 3s | < 5s | `time_to_render` benchmark |
| Hot file open | < 500ms | < 1s | `time_to_ready` benchmark |
| Keystroke → render | < 100ms | < 200ms | `edit_latency` benchmark |
| Save (Ctrl+S) | < 200ms | < 500ms | `save_time` benchmark |
| HTML export (10KB) | < 1s | < 2s | `export_time` benchmark |
| PDF export (10 pages) | < 5s | < 10s | `export_time` benchmark |
| Outline update | < 100ms | < 200ms | `outline_update` benchmark |
| Theme switch | < 200ms | < 500ms | `theme_switch` benchmark |
| Memory (10 docs idle) | < 300MB | < 500MB | `memory_usage` benchmark |
| Scroll (5MB doc) | 60 FPS | 30 FPS | `scroll_fps` benchmark |

### 8.2 Scalability Tests

| Document Size | Expected Behavior |
|---------------|-------------------|
| < 100KB | Full feature set, no degradation |
| 100KB - 1MB | Full feature set, debounce may increase |
| 1MB - 5MB | All features work, occasional render pauses < 200ms |
| 5MB - 10MB | Core editing works, disable live preview toggle |
| > 10MB | Warning shown, continue with degraded mode |

### 8.3 Benchmark Commands

```bash
# Rust benchmarks
cargo bench --package rustnote -- --nfo 100

# Frontend benchmarks  
npm run bench

# E2E performance
npx playwright test --project=performance
```

---

## 9. Security Test Specifications

### 9.1 Input Validation

| Test ID | Attack Vector | Input | Expected |
|---------|---------------|-------|----------|
| SEC-001 | Path traversal | `../etc/passwd` in file path | Rejected, error shown |
| SEC-002 | Null byte injection | `file\x00.txt` | Rejected |
| SEC-003 | Long path | 4096 char path | Rejected or truncated |
| SEC-004 | Invalid UTF-8 | `\xff\xfe` bytes | Rejected or sanitized |
| SEC-005 | Control chars | `\x00\x01\x02` in content | Stripped or rejected |

### 9.2 HTML Sanitization

| Test ID | Attack Vector | Input | Expected |
|---------|---------------|-------|----------|
| SEC-010 | Script injection | `<script>alert(1)</script>` | Stripped |
| SEC-011 | Event handler | `<img onerror=alert(1)>` | Stripped |
| SEC-012 | JavaScript URL | `<a href="javascript:...">` | Stripped |
| SEC-013 | Iframe injection | `<iframe src="...">` | Stripped |
| SEC-014 | Style injection | `<style>body{...}</style>` | Stripped |
| SEC-015 | Data URL XSS | `<a href="data:text/html,...">` | Stripped |
| SEC-016 | Base tag | `<base href="...">` | Stripped |

### 9.3 Export Security

| Test ID | Test Case | Expected |
|---------|-----------|----------|
| SEC-020 | HTML export - no script | Exported HTML has no `<script>` |
| SEC-021 | HTML export - no event handlers | No `on*` attributes |
| SEC-022 | HTML export - self-contained | No external resources |
| SEC-023 | PDF export - no active content | PDF is static |
| SEC-024 | Export path traversal | Cannot write outside target |

### 9.4 Fuzz Testing

| Target | Fuzzer | Duration |
|--------|--------|----------|
| Parser | cargo-fuzz | 24 hours per release |
| Serializer | cargo-fuzz | 24 hours per release |
| Deserializer | cargo-fuzz | 24 hours per release |

---

## 10. Accessibility Test Specifications

### 10.1 Keyboard Navigation

| Test ID | Feature | Key Sequence | Expected |
|---------|---------|--------------|----------|
| A11Y-001 | Navigate to editor | Tab | Focus moves to editor |
| A11Y-002 | Navigate toolbar | Tab (from editor) | Focus moves through toolbar |
| A11Y-003 | Navigate sidebar | Ctrl+Shift+S | Focus moves to sidebar |
| A11Y-004 | Open file | Ctrl+O | File dialog opens |
| A11Y-005 | Save file | Ctrl+S | File saves |
| A11Y-006 | Find | Ctrl+F | Find dialog opens |
| A11Y-007 | Close dialog | Escape | Dialog closes |
| A11Y-008 | Navigate outline | Tab (in outline panel) | Items focusable |

### 10.2 Screen Reader

| Test ID | Element | Expected Announcement |
|---------|---------|---------------------|
| A11Y-010 | Heading | "Heading level 1, [text]" |
| A11Y-011 | Link | "Link, [text], [url]" |
| A11Y-012 | Image | "Image, [alt text]" or "Image, no description" |
| A11Y-013 | List | "List, [n] items" |
| A11Y-014 | Table | "Table, [n] columns, [m] rows" |
| A11Y-015 | Code block | "Code block, language: [lang]" |
| A11Y-016 | Task list | "Checkbox, [checked/unchecked], [text]" |

### 10.3 Visual

| Test ID | Requirement | Standard |
|---------|-------------|----------|
| A11Y-020 | Color contrast (text) | 4.5:1 minimum |
| A11Y-021 | Color contrast (large text) | 3:1 minimum |
| A11Y-022 | Focus indicators | Visible on all interactive elements |
| A11Y-023 | No color-only information | Additional cues provided |

---

## 11. Platform Test Matrix

| Feature | macOS Intel | macOS Apple Silicon | Windows x64 | Windows ARM | Linux AppImage | Linux deb |
|---------|--------------|---------------------|-------------|--------------|----------------|-----------|
| **Installation** |
| Clean install | P0 | P0 | P0 | P1 | P0 | P0 |
| Upgrade install | P1 | P1 | P1 | P2 | P1 | P1 |
| Uninstall | P1 | P1 | P1 | P2 | P1 | P1 |
| **File Operations** |
| New/Open/Save | P0 | P0 | P0 | P0 | P0 | P0 |
| File dialogs | P0 | P0 | P0 | P1 | P0 | P0 |
| Drag-drop | P0 | P0 | P1 | P2 | P1 | P1 |
| **Editor** |
| Live rendering | P0 | P0 | P0 | P0 | P0 | P0 |
| Keyboard shortcuts | P0 | P0 | P0 | P0 | P0 | P0 |
| IME input | P1 | P1 | P1 | P2 | P1 | P1 |
| **Export** |
| HTML export | P0 | P0 | P0 | P0 | P0 | P0 |
| PDF export | P0 | P0 | P0 | P1 | P0 | P0 |
| **System** |
| Theme (system) | P0 | P0 | P0 | P0 | P0 | P0 |
| Notifications | P1 | P1 | P1 | P1 | P2 | P2 |
| **Accessibility** |
| VoiceOver | P0 | P0 | N/A | N/A | N/A | N/A |
| NVDA | N/A | N/A | P0 | P1 | N/A | N/A |
| Orca | N/A | N/A | N/A | N/A | P1 | P1 |

**Legend:** P0 = Must pass, P1 = Should pass, P2 = Nice to have

---

## 12. Test Data Requirements

### 12.1 Markdown Fixtures (`fixtures/markdown/`)

| File | Purpose | Coverage |
|-------|---------|----------|
| headings.md | All H1-H6 | Headings |
| emphasis.md | Bold, italic, strike | Inline formatting |
| links.md | Links, images, refs | Links |
| lists.md | UL, OL, nested | Lists |
| tasklists.md | Checkboxes | Task lists |
| blockquotes.md | Nested quotes | Blockquotes |
| codeblocks.md | Fenced code | Code blocks |
| tables.md | GFM tables | Tables |
| frontmatter.md | YAML frontmatter | Frontmatter |
| mixed.md | All elements | Full coverage |
| long.md | 1000+ paragraphs | Performance |
| wide.md | 500+ char lines | Performance |

### 12.2 Edge Cases (`fixtures/edge-cases/`)

| File | Purpose |
|-------|---------|
| malformed.md | Malformed Markdown |
| binary.md | Binary content injection |
| longlines.md | 1MB+ single lines |
| unicode.md | Chinese, Arabic, emoji |
| deep-nesting.md | 100+ list levels |
| large-table.md | 100x100 cells |

### 12.3 Export Fixtures (`fixtures/export/`)

| File | Purpose |
|-------|---------|
| minimal.html | Expected minimal HTML output |
| full.html | Expected full HTML with styling |
| code.html | Expected HTML with syntax highlighting |
| minimal.pdf | Expected minimal PDF |
| full.pdf | Expected full PDF with styling |

### 12.4 Recovery Fixtures (`fixtures/recovery/`)

| File | Purpose |
|-------|---------|
| crash-during-edit.json | Snapshot mid-edit |
| crash-after-save.json | Snapshot after save |
| stale.json | Old stale snapshot |

---

## 13. CI/CD Pipeline

### 13.1 Pipeline Stages

```
┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│   Lint      │ → │   Build     │ → │    Test     │ → │   Deploy    │
│  - rustfmt  │   │  - Tauri    │   │  - Unit     │   │  - Nightly  │
│  - clippy   │   │  - Frontend │   │  - Integr.  │   │  - Beta     │
│  - eslint   │   │             │   │  - E2E      │   │  - Stable   │
└─────────────┘   └─────────────┘   └─────────────┘   └─────────────┘
       │                                      │
       ▼                                      ▼
┌─────────────┐                        ┌─────────────┐
│  Security   │                        │  Visual     │
│  - cargo-audit│                      │  - Percy    │
│  - fuzz     │                        │  - Manual   │
└─────────────┘                        └─────────────┘
```

### 13.2 Required Gates

| Gate | Required | Blocking | Tool |
|------|----------|----------|------|
| `cargo fmt --check` | Yes | Yes | rustfmt |
| `cargo clippy` | Yes | Yes | clippy |
| `cargo test` | Yes | Yes | cargo test |
| `npm run lint` | Yes | Yes | eslint/prettier |
| `npm test` | Yes | Yes | vitest |
| `npm run build` | Yes | Yes | vite build |
| `cargo audit` | Yes | Yes | cargo-audit |
| `cargo-fuzz` | No | No | cargo-fuzz |
| Playwright E2E | No | No | Playwright |
| Visual regression | No | No | Percy |
| Accessibility scan | No | No | axe-core |

### 13.3 Pull Request Requirements

| Check | Required for Merge |
|-------|-------------------|
| All CI gates pass | Yes |
| Code reviewed | Yes |
| Tests added/updated | Yes |
| Documentation updated | If user-facing |
| No breaking changes (unless approved) | Yes |

---

## 14. Test Execution Schedule

### 14.1 Continuous

| Trigger | Tests Run |
|---------|-----------|
| Every commit | Lint, Unit tests, cargo test |
| Every PR | Full test suite |
| Every merge to main | Full test suite + deployment |

### 14.2 Release

| Phase | Tests | Duration |
|-------|-------|----------|
| RC1 | Full suite + visual regression | 2 hours |
| RC2 | Smoke tests + performance | 30 minutes |
| RC3 | Smoke tests only | 15 minutes |
| Final | Full suite + manual | 4 hours |

### 14.3 Nightly

| Test | Time | Duration |
|------|------|----------|
| Fuzz testing | 2 AM UTC | 6 hours |
| Large fixture tests | 3 AM UTC | 1 hour |
| Visual regression baseline | 4 AM UTC | 30 minutes |

---

## 15. Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| contenteditable instability | High | High | Plan migration to ProseMirror |
| Table editing complexity | High | Medium | Start with constrained model |
| Performance on large docs | Medium | High | Implement virtualization |
| Cross-platform UI consistency | Medium | Medium | Use platform-native dialogs |
| PDF export fidelity | Medium | Medium | Early integration testing |
| IME composition issues | Low | Medium | Target CJK users for testing |

---

## 16. Definition of Done

### 16.1 For Features

A feature is considered **Done** when:

- [ ] Implementation complete and merged
- [ ] Unit tests written and passing
- [ ] Integration tests written and passing
- [ ] E2E tests written and passing
- [ ] Visual regression tests (if UI changed)
- [ ] Accessibility tests passing
- [ ] Performance benchmarks meet thresholds
- [ ] Security tests passing
- [ ] Documentation updated
- [ ] No known data-loss risks
- [ ] CI pipeline green
- [ ] Code review approved

### 16.2 For Bug Fixes

A bug is considered **Done** when:

- [ ] Root cause identified
- [ ] Fix implemented
- [ ] Regression test added
- [ ] Original test now passes
- [ ] No new warnings introduced
- [ ] CI pipeline green

### 16.3 For Releases

A release is considered **Ready** when:

- [ ] All P0 tests pass
- [ ] All P1 tests pass (>90%)
- [ ] Performance thresholds met
- [ ] Security scan clean
- [ ] No known critical bugs
- [ ] Manual testing complete
- [ ] Release notes drafted
- [ ] Installers tested on all platforms

---

## Appendix A: Test ID Naming Convention

| Prefix | Category | Example |
|--------|----------|---------|
| PAR- | Parser unit tests | PAR-001 |
| SER- | Serializer unit tests | SER-001 |
| CUR- | Cursor unit tests | CUR-001 |
| SEL- | Selection unit tests | SEL-001 |
| ENT- | Enter behavior tests | ENT-001 |
| BSP- | Backspace behavior tests | BSP-001 |
| TAB- | Tab behavior tests | TAB-001 |
| BUF- | Buffer unit tests | BUF-001 |
| SS- | Serializer spec tests | SS-001 |
| SET- | Settings tests | SET-001 |
| REC- | Recovery tests | REC-001 |
| INT- | Integration tests | INT-F001 |
| E2E- | End-to-end tests | E2E-C001 |
| REG- | Regression tests | REG-001 |
| VIS- | Visual tests | VIS-001 |
| SEC- | Security tests | SEC-001 |
| A11Y- | Accessibility tests | A11Y-001 |

---

## Appendix B: Tool Versions

| Tool | Version | Notes |
|------|---------|-------|
| Rust | 1.75+ | Stable |
| Node.js | 20+ | LTS |
| Tauri CLI | 2.x | |
| Tauri | 2.x | |
| Playwright | 1.40+ | |
| Vitest | 1.0+ | |
| Percy | Latest | Visual regression |
| axe-core | Latest | Accessibility |

---

**Document History**

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-11 | Sisyphus | Initial draft |
