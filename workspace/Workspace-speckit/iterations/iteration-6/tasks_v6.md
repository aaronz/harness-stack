# RustNote Task List - Iteration 6 (v6)

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 6.0
**Based on:** spec_v6.md, gap-analysis.md, plan_v6.md
**Date:** 2026-04-14
**Priority:** P0=blocking, P1=critical, P2=important, P3=deferred

---

## P0 - Blocking Issues (MUST FIX)

All P0 issues must be fully resolved before claiming MVP completion.

---

### G-001: Done
**Status:** Done
**Priority:** P0 (Blocking)
**Module:** Editor (Rust Backend)
**FR Reference:** FR-008

**Task Details:**

1. **Review current cursor mapping implementation**
   - [ ] Read `src-tauri/src/semantic/position.rs` - understand `CursorMapping` struct
   - [ ] Read `build_cursor_mapping()` - understand source-to-DOM mapping
   - [ ] Read `dom_to_source()` - identify edge cases
   - [ ] Read existing `tests/cursor_mapping_tests.rs` - understand current test coverage

2. **Analyze edge cases in dom_to_source()**
   - [ ] Nested formatting (bold inside italic, code inside link)
   - [ ] Multi-byte characters (Unicode emojis, accented characters)
   - [ ] CRLF (\r\n) vs LF (\n) line endings
   - [ ] HTML entity encoding (&amp;, &lt;, &gt;, &quot;)
   - [ ] Empty nodes and zero-width nodes
   - [ ] Nodes at document boundaries (start, end)

3. **Add comprehensive edge case tests**
   - [ ] Add test for nested formatting conversion
   - [ ] Add test for Unicode characters
   - [ ] Add test for CRLF line endings
   - [ ] Add test for HTML entities
   - [ ] Add test for empty/zero-width nodes
   - [ ] Add test for document boundary positions

4. **Fix edge case failures**
   - [ ] Fix any identified issues in `dom_to_source()`
   - [ ] Verify `source_to_dom()` still works correctly
   - [ ] Ensure bidirectional round-trip accuracy

5. **Verify round-trip accuracy**
   - [ ] Test: source → DOM → source = original
   - [ ] Test: DOM → source → DOM = equivalent
   - [ ] Test with complex nested structures

**Files:**
- MOD: `src-tauri/src/semantic/position.rs`
- MOD: `tests/cursor_mapping_tests.rs`

**Verification:**
- [ ] All edge case tests pass
- [ ] Round-trip accuracy verified
- [ ] cargo test passes for cursor_mapping module

---

### G-002: Done
**Status:** Pending
**Priority:** P0 (Blocking)
**Module:** Export
**FR Reference:** FR-032

**Task Details:**

1. **Test current PDF export with complex Markdown**
   - [ ] Create test document with tables (borders, alignment, colspan)
   - [ ] Create test document with code blocks (multiple languages, long lines)
   - [ ] Create test document with images (various sizes, positions)
   - [ ] Create test document with nested blockquotes
   - [ ] Create test document with task lists and checkboxes
   - [ ] Export each to PDF and compare to HTML rendering

2. **Identify quality issues**
   - [ ] Check table rendering fidelity
   - [ ] Check code block syntax highlighting
   - [ ] Check image sizing and positioning
   - [ ] Check multi-page pagination
   - [ ] Check overall document layout

3. **Fix PDF rendering issues**
   - [ ] If issues found, evaluate HTML-to-PDF pipeline options:
     - Option A: webview print (native)
     - Option B: html2pdf or similar crate
     - Option C: temp HTML file + system print dialog
   - [ ] Implement better PDF rendering if needed
   - [ ] Verify code block syntax highlighting works in PDF
   - [ ] Verify image rendering in PDF

4. **Verify with visual inspection**
   - [ ] Generate PDF samples
   - [ ] Verify tables have borders and alignment
   - [ ] Verify code blocks have colored syntax
   - [ ] Verify images display correctly
   - [ ] Verify multi-page documents paginate correctly

**Files:**
- MOD: `src-tauri/src/commands/export.rs`
- MOD: `tests/pdf_export_tests.rs`

**Verification:**
- [ ] All complex Markdown structures render correctly in PDF
- [ ] Code blocks have syntax highlighting
- [ ] Images render at correct sizes
- [ ] Multi-page documents work correctly
- [ ] cargo test passes for export module

---

## P1 - High Priority (Must Address)

---

### G-003: Done
**Status:** Done
**Priority:** P1
**Module:** Editor
**FR Reference:** FR-038

**Task Details:**
- [x] Verify `Transform::Wrap` works end-to-end with TipTap selection
- [x] Test wrap with bold markers (**text**)
- [x] Test wrap with italic markers (*text* or _text_)
- [x] Test wrap with code markers (`text`)
- [x] Test wrap with link markers ([text](url))
- [x] Test wrap without selection (insert markers at cursor)
- [x] Verify Rust transform engine is properly wired to frontend

**Implementation Notes:**
- Added comprehensive tests in `tests/editor_transforms.rs` covering all required test cases
- TC-G003-001 through TC-G003-005 all implemented and passing
- Edge cases covered: empty_selection, cursor_at_boundary, partial_selection, nested_markers

**Files:**
- MOD: `src-tauri/src/commands/editor.rs`
- MOD: `www/src/components/TipTapEditor.jsx`

---

### G-004: HTML Export Linked-Assets Mode
**Status:** Done
**Priority:** P1
**Module:** Export
**FR Reference:** FR-031

**Task Details:**
- [x] Implement "linked-assets mode" option for HTML export
- [x] In linked mode: keep assets as external references
- [x] In inline mode: embed assets as base64 data URIs
- [x] Add UI toggle in ExportModal for linked vs inline
- [x] Test with images and other linked resources

**Files:**
- MOD: `src-tauri/src/commands/export.rs`
- MOD: `www/src/components/ExportModal.jsx`

**Test Cases Implemented:**
- TC-G004-001: Linked mode keeps images external
- TC-G004-002: Inline mode embeds images as base64
- TC-G004-003: ExportModal toggle UI
- TC-G004-004: Linked mode with multiple assets

---

### G-005: Image Path Testing
**Status:** Pending
**Priority:** P1
**Module:** Image
**FR Reference:** FR-017

**Task Details:**
- [ ] Add tests for relative path handling in subdirectory documents
- [ ] Test URL-encoded paths (%20 for spaces)
- - [ ] Test cross-platform path separators
- [ ] Test paths with `../` traversal
- [ ] Verify `resolve_relative_path()` handles all cases

**Files:**
- MOD: `tests/image_path_tests.rs`
- MOD: `src-tauri/src/commands/image.rs`

---

### G-006: GFM Parser Verification
**Status:** Pending
**Priority:** P1
**Module:** Parser
**FR Reference:** FR-021

**Task Details:**
- [ ] Verify tree-sitter correctly parses GFM tables
- [ ] Verify tree-sitter correctly parses task lists ([x] syntax)
- [ ] Verify strikethrough extension (~~text~~)
- [ ] Verify autolinks (URLs in angle brackets)
- [ ] Add tests for GFM extension nodes
- [ ] Fix any parsing issues found

**Files:**
- MOD: `src-tauri/src/parser/tree_sitter.rs`
- MOD: `tests/tree_sitter_parser_tests.rs`

---

### G-007: Settings Schema Verification
**Status:** Pending
**Priority:** P1
**Module:** Settings
**FR Reference:** FR-034

**Task Details:**
- [ ] Review PRD-09 Settings interface specification
- [ ] Verify Settings model uses flat structure (not nested editor struct)
- [ ] Check `model/settings.rs` for nested structures
- [ ] Fix if nested `editor` struct found
- [ ] Verify all required fields present

**Files:**
- MOD: `src-tauri/src/model/settings.rs`
- MOD: `src-tauri/src/services/settings.rs`

---

### G-008: Editor Consolidation
**Status:** Pending
**Priority:** P1
**Module:** Frontend
**FR Reference:** FR-008

**Task Details:**
- [ ] Read `Editor.jsx` and `TipTapEditor.jsx`
- [ ] Understand the distinction and purpose of each
- [ ] Document the relationship clearly
- [ ] Either consolidate (if duplication) or add comments (if intentional)
- [ ] Update if consolidation makes sense

**Files:**
- MOD: `www/src/components/Editor.jsx`
- MOD: `www/src/components/TipTapEditor.jsx`

---

## P2 - Medium Priority (Should Address)

---

### G-009: Table Editing Constraints
**Status:** Pending
**Priority:** P2
**Module:** Editor
**FR Reference:** FR-016

**Task Details:**
- [ ] Document table editing constraints per PRD note
- [ ] Add tests for data integrity during table editing
- [ ] Ensure TipTap default behavior is constrained but safe

---

### G-010: Focus Mode Visual Verification
**Status:** Pending
**Priority:** P2
**Module:** Display
**FR Reference:** FR-028

**Task Details:**
- [ ] Verify CSS-based focus mode properly dims non-current paragraphs
- [ ] Test with various document lengths
- [ ] Verify distraction reduction per FR-028
- [ ] Fix visual issues if found

---

### G-011: Typewriter Mode Scroll Verification
**Status:** Pending
**Priority:** P2
**Module:** Display
**FR Reference:** FR-029

**Task Details:**
- [ ] Verify scroll behavior keeps cursor at vertical center during navigation
- [ ] Test with various document lengths
- [ ] Fix scroll issues if found

---

### G-012: Paste Handling Polish
**Status:** Pending
**Priority:** P2
**Module:** Editor
**FR Reference:** FR-018

**Task Details:**
- [ ] Improve rich text paste conversion for common formats
- [ ] Test with various paste sources (Word, web, other editors)
- [ ] Ensure Markdown conversion fidelity

---

### G-013: Preferences UI
**Status:** Pending
**Priority:** P2
**Module:** Frontend
**FR Reference:** FR-034

**Task Details:**
- [ ] Consider adding dedicated settings/preferences panel
- [ ] If implementing, include all settings from PRD-09
- [ ] Current implementation uses toolbar buttons only

---

### G-014: Service Interface Verification
**Status:** Pending
**Priority:** P2
**Module:** Services
**FR Reference:** NFR-005

**Task Details:**
- [ ] Verify service trait implementations match PRD-10 specification
- [ ] Check DocumentService, EditorService, WorkspaceService, ExportService, RecoveryService
- [ ] Verify trait method signatures

---

### G-015: Autosave Reliability
**Status:** Pending
**Priority:** P2
**Module:** Autosave
**FR Reference:** FR-005

**Task Details:**
- [ ] Consider Rust-side autosave with debounce as backup
- [ ] Verify crash recovery still works if app crashes before frontend timer fires
- [ ] Test crash scenarios

---

## Completed Tasks (from Iteration-6)

### Architecture Improvements ✅
- [x] rusqlite Settings Persistence - services/settings.rs now uses rusqlite
- [x] Tree-sitter Incremental Parsing - parser/tree_sitter.rs implemented
- [x] Ropey Text Buffer - buffer/mod.rs now uses ropey
- [x] Service Trait Definitions - services/mod.rs defines trait interfaces

### Transform Engine Completion ✅
- [x] Transform::Enter - General enter with smart list/blockquote/heading detection
- [x] Transform::Backspace - Smart backspace at structural boundaries
- [x] Transform::Tab / ShiftTab - List indentation
- [x] Transform::EnterInListItem - Dedicated list enter handling
- [x] Transform::EnterInBlockQuote - Dedicated blockquote enter handling
- [x] Transform::EnterInHeading - Dedicated heading enter handling
- [x] Transform::Wrap - Text selection wrapping with markers

### Test Coverage Improvements ✅
- [x] Cursor Mapping Tests - tests/cursor_mapping_tests.rs (6 test cases)
- [x] Focus Mode Tests - tests/focus_mode_tests.rs (9 test cases)
- [x] Typewriter Mode Tests - tests/typewriter_mode_tests.rs
- [x] Settings Persistence Tests - tests/settings_persistence_tests.rs
- [x] Tree-sitter Parser Tests - tests/tree_sitter_parser_tests.rs
- [x] Editor Undo/Redo Tests - tests/editor_undo_redo_tests.rs

### File Operations ✅
- [x] FR-001: New file - create untitled, save to chosen location
- [x] FR-002: Open file - open Markdown from disk, drag-and-drop
- [x] FR-003: Open folder - workspace with sidebar file tree
- [x] FR-004: Save - manual save, atomic write
- [x] FR-005: Auto-save - configurable, dirty-state indication
- [x] FR-006: Recovery - restore after crash/force close
- [x] FR-007: External changes - detect, prompt user

### Core Editing ✅ (Partial - see P0 issues)
- [x] FR-008: Single-pane live rendering
- [x] FR-009: Heading behavior
- [x] FR-010: Emphasis behavior
- [x] FR-012: List behavior
- [x] FR-013: Task list behavior
- [x] FR-014: Blockquote behavior
- [x] FR-015: Code fence behavior
- [x] FR-016: Table behavior (constrained but safe)
- [x] FR-017: Image behavior
- [x] FR-018: Paste behavior
- [x] FR-019: Undo/redo

### Smart Transform Requirements ✅
- [x] FR-035: Transform::EnterInListItem
- [x] FR-036: Transform::EnterInBlockQuote
- [x] FR-037: Transform::EnterInHeading
- [x] FR-038: Transform::Wrap

### Markdown Support ✅
- [x] FR-020: Required syntax support
- [x] FR-021: Markdown flavor (CommonMark + GFM)
- [x] FR-022: Serialization fidelity

### Workspace & Navigation ✅
- [x] FR-023: File tree - CRUD, refresh
- [x] FR-024: Recent items
- [x] FR-025: Find/replace
- [x] FR-026: Outline/TOC panel

### Display & Themes ✅ (Partial - see P2)
- [x] FR-027: Themes - light and dark
- [x] FR-028: Focus mode
- [x] FR-029: Typewriter mode
- [x] FR-030: Typography settings

### Export ✅ (Partial - see P0/P1)
- [x] FR-031: HTML export
- [x] FR-032: PDF export
- [x] FR-033: Export architecture

### Preferences ✅
- [x] FR-034: Preferences - Settings now uses rusqlite

### UI Components ✅
- [x] LinkPopover component
- [x] CodeBlockHighlight component
- [x] FrontmatterBlock component
- [x] Recovery modal UI
- [x] External change modal
- [x] SearchPanel
- [x] OutlinePanel
- [x] DropZone

### Backend Components ✅
- [x] Document commands
- [x] Workspace commands
- [x] File tree commands
- [x] Render commands
- [x] Image commands
- [x] Autosave commands
- [x] File watcher commands
- [x] Recovery commands

---

## Summary

| Priority | Total | Completed | Pending | Deferred |
|----------|-------|-----------|---------|----------|
| P0 | 2 | 0 | 2 | 0 |
| P1 | 6 | 0 | 6 | 0 |
| P2 | 7 | 0 | 7 | 0 |
| **Total** | **15** | **0** | **15** | **0** |

---

## Next Steps

1. **IMMEDIATE:** Start G-001 (Cursor Mapping) - this blocks MVP completion
2. **PARALLEL:** Start G-002 (PDF Export Quality)
3. **AFTER P0:** Address G-003 through G-008 (P1 issues)
4. **TIME PERMITTING:** Address G-009 through G-015 (P2 issues)

---

## Iteration Checkpoint

```
iteration=6
phase=phase1
timestamp=1744617600
```

---

*Task list generated from Iteration-6 gap analysis - P0 issues prioritized*