# RustNote Task List - Iteration 5 (v5)

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 5.0
**Based on:** spec_v5.md, gap-analysis.md, plan_v5.md
**Date:** 2026-04-13
**Priority:** P0=blocking, P1=critical, P2=important, P3=deferred

---

## P0 - Blocking Issues (MUST FIX)

All P0 issues must be fully resolved before claiming MVP completion.

---

### G-001: Done
**Status:** Pending
**Priority:** P0 (Blocking)
**Module:** Editor (Rust Backend)
**FR Reference:** FR-012, FR-014, FR-035, FR-036, FR-037, FR-038

**Task Details:**

1. **Review TransformType enum**
   - [ ] Read `src-tauri/src/commands/editor.rs` - find `TransformType` enum
   - [ ] Read `src-tauri/src/editor/transforms.rs` - find current implementations
   - [ ] Read `www/src/hooks/useTransforms.ts` - understand expected behavior

2. **Implement EnterInListItem transform**
   - [ ] Create `TransformEngine::apply_enter_in_list_item()` method
   - [ ] Logic: If list item is empty, exit list and create paragraph
   - [ ] Logic: If list item has content, create new list item below
   - [ ] Handle nested lists correctly
   - [ ] Return transformed Markdown source

3. **Implement EnterInBlockQuote transform**
   - [ ] Create `TransformEngine::apply_enter_in_blockquote()` method
   - [ ] Logic: If blockquote is empty, exit blockquote and create paragraph
   - [ ] Logic: If blockquote has content, continue with new line in quote
   - [ ] Handle nested blockquotes correctly
   - [ ] Return transformed Markdown source

4. **Implement EnterInHeading transform**
   - [ ] Create `TransformEngine::apply_enter_in_heading()` method
   - [ ] Logic: If heading is empty, convert to paragraph
   - [ ] Logic: If heading has content, create new heading of same level
   - [ ] Handle Setext heading syntax (=== or ---) correctly
   - [ ] Return transformed Markdown source

5. **Implement Wrap transform**
   - [ ] Create `TransformEngine::apply_wrap()` method
   - [ ] Accept `before` and `after` strings (e.g., `**` for bold)
   - [ ] Wrap selected text with markers
   - [ ] Handle no selection gracefully (insert markers at cursor)
   - [ ] Return transformed Markdown source

6. **Wire frontend to use implemented transforms**
   - [ ] Update `useTransforms.ts` to call Rust backend
   - [ ] Remove frontend-only transform implementations
   - [ ] Test all transform scenarios end-to-end

7. **Add unit tests**
   - [ ] Add test for EnterInListItem (empty and with content)
   - [ ] Add test for EnterInBlockQuote (empty and with content)
   - [ ] Add test for EnterInHeading (empty and with content)
   - [ ] Add test for Wrap (with selection, without selection)
   - [ ] Add test for nested structures

**Files:**
- MOD: `src-tauri/src/commands/editor.rs`
- MOD: `src-tauri/src/editor/transforms.rs`
- MOD: `www/src/hooks/useTransforms.ts`
- MOD: `www/src/components/TipTapEditor.jsx`
- NEW: `tests/editor_transforms.rs` (add comprehensive tests)

**Verification:**
- [ ] All TransformType variants implemented in Rust
- [ ] Pressing Enter in list item creates new item or exits correctly
- [ ] Pressing Enter in blockquote continues or exits correctly
- [ ] Pressing Enter in heading creates new heading or paragraph
- [ ] Wrap transform works with selections
- [ ] cargo test passes for transform module

---

### G-002: Done
**Status:** Pending
**Priority:** P0 (Blocking)
**Module:** Editor (Frontend + Backend)
**FR Reference:** FR-019

**Task Details:**

1. **Analyze current undo/redo implementation**
   - [ ] Read TipTapEditor.jsx - understand current undo/redo setup
   - [ ] Read backend transaction log if any
   - [ ] Identify structural edit handling issues

2. **Add comprehensive round-trip tests**
   - [ ] Create test: do operation → undo → do again → undo = original state
   - [ ] Test list manipulation (add/remove/reorder items)
   - [ ] Test heading level changes
   - [ ] Test blockquote conversion
   - [ ] Test nested structures

3. **Fix structural edit handling issues**
   - [ ] Identify any state corruption after undo/redo
   - [ ] Fix transaction log to maintain correct state
   - [ ] Ensure structural edits are atomic

4. **Verify transaction log fidelity**
   - [ ] Test multiple rapid undo/redo operations
   - [ ] Test undo/redo across save boundaries
   - [ ] Test undo/redo after document close/reopen

**Files:**
- MOD: `www/src/components/TipTapEditor.jsx`
- MOD: `tests/integration_editor_tests.rs`
- NEW: `tests/editor_undo_redo_tests.rs`

**Verification:**
- [ ] Round-trip tests pass (do → undo → do → undo = original)
- [ ] Structural edits undo correctly
- [ ] No state corruption after multiple undo/redo cycles
- [ ] Undo/redo works after save and reopen

---

### G-003: Done
**Status:** Pending
**Priority:** P0 (Blocking)
**Module:** Settings (Rust Backend)
**FR Reference:** FR-034

**Task Details:**

1. **Design rusqlite schema**
   - [ ] Review PRD-10 for settings schema requirements
   - [ ] Design tables: settings, recent_files, workspace_state
   - [ ] Define indexes for frequent queries

2. **Implement SettingsService with rusqlite**
   - [ ] Create `src-tauri/src/services/settings.rs`
   - [ ] Implement init_database() - create tables if not exist
   - [ ] Implement read_settings() - load from database
   - [ ] Implement write_settings() - save to database with transaction
   - [ ] Implement atomic update with rollback on failure

3. **Migrate existing settings.json data**
   - [ ] Read existing `settings.json` if exists
   - [ ] Migrate data to rusqlite database
   - [ ] Backup original settings.json
   - [ ] Verify migration integrity

4. **Update commands to use rusqlite service**
   - [ ] Update `src-tauri/src/commands/settings.rs`
   - [ ] Update `src-tauri/src/commands/mod.rs` to register service
   - [ ] Update `src-tauri/src/lib.rs`

5. **Test crash recovery**
   - [ ] Test settings write during crash simulation
   - [ ] Verify atomic transaction rollback
   - [ ] Test settings read after abnormal termination

**Files:**
- NEW: `src-tauri/src/services/settings.rs` (or mod.rs in services/)
- MOD: `src-tauri/src/commands/settings.rs`
- MOD: `src-tauri/src/commands/mod.rs`
- MOD: `src-tauri/src/lib.rs`
- MOD: `src-tauri/Cargo.toml` (add rusqlite, serde_rusqlite)
- MOD: `www/src/contexts/SettingsContext.jsx` (map types)
- NEW: `tests/settings_persistence_tests.rs`

**Verification:**
- [ ] Settings persist correctly in rusqlite database
- [ ] Atomic transactions prevent data corruption on crash
- [ ] Existing settings.json migrates correctly
- [ ] cargo test passes for settings module

---

## P1 - High Priority (Must Address)

---

### G-004: Cursor Mapping
**Status:** Pending
**Priority:** P1
**Module:** Editor (Rust + Frontend)
**FR Reference:** FR-008

**Task Details:**
- [ ] Implement bidirectional cursor mapping with AST-aware offset calculation
- [ ] Account for HTML tag insertion in mapping
- [ ] Test cursor position preservation in various scenarios

**Files:**
- MOD: `src-tauri/src/semantic/position.rs`
- MOD: `www/src/components/TipTapEditor.jsx`

---

### G-005: PDF Export Quality
**Status:** Pending
**Priority:** P1
**Module:** Export
**FR Reference:** FR-032

**Task Details:**
- [ ] Integrate proper HTML-to-PDF rendering (html2pdf, puppeteer, or webview print)
- [ ] Ensure code blocks render with highlighting
- [ ] Ensure images render correctly
- [ ] Test multi-page documents

**Files:**
- MOD: `src-tauri/src/commands/export.rs`
- MOD: `www/src/components/ExportModal.jsx`

---

### G-006: Tree-sitter Parser
**Status:** Pending
**Priority:** P1
**Module:** Parser
**FR Reference:** FR-021

**Task Details:**
- [ ] Research tree-sitter + comrak integration
- [ ] Implement tree-sitter-based incremental parsing wrapper
- [ ] Ensure backward compatibility with existing parsing
- [ ] Test with large documents

**Files:**
- MOD: `src-tauri/src/parser/`
- MOD: `src-tauri/Cargo.toml`

---

### G-007: Ropey Buffer
**Status:** Pending
**Priority:** P1
**Module:** Buffer
**FR Reference:** NFR-003

**Task Details:**
- [ ] Review current buffer module implementation
- [ ] Implement ropey-based text buffer per PRD specification
- [ ] Update editor commands to use buffer
- [ ] Test with 5MB+ documents

**Files:**
- MOD: `src-tauri/src/buffer/mod.rs`
- MOD: `src-tauri/Cargo.toml`

---

### G-008: Wrap Transform
**Status:** Pending (covered by G-001)
**Priority:** P1
**Module:** Editor
**FR Reference:** FR-038

**Task Details:**
- [ ] Part of G-001 TransformEngine implementation

---

### G-009: Image Path Handling
**Status:** Pending
**Priority:** P1
**Module:** Image
**FR Reference:** FR-017

**Task Details:**
- [ ] Verify relative path calculation based on document location
- [ ] Test with documents in subdirectories
- [ ] Fix any path resolution issues

**Files:**
- MOD: `src-tauri/src/commands/image.rs`
- MOD: `www/src/components/TipTapEditor.jsx`

---

## P2 - Medium Priority (Should Address)

---

### G-010: Table Editing
**Status:** Pending
**Priority:** P2
**Module:** Editor
**FR Reference:** FR-016

**Task Details:**
- [ ] Document table editing constraints
- [ ] Ensure no data loss with TipTap default behavior
- [ ] Consider adding custom cell-model editing if time permits

---

### G-011: HTML Export Modes
**Status:** Pending
**Priority:** P2
**Module:** Export
**FR Reference:** FR-031

**Task Details:**
- [ ] Add option for linked vs inline asset export
- [ ] Implement "linked-assets mode" per FR-031
- [ ] Test with images and other assets

---

### G-012: Frontmatter Parsing
**Status:** Pending
**Priority:** P2
**Module:** Parser
**FR Reference:** FR-020

**Task Details:**
- [ ] Improve frontmatter parsing to handle complex YAML
- [ ] Test with various frontmatter formats
- [ ] Handle missing/invalid frontmatter gracefully

---

### G-013: Focus Mode Quality
**Status:** Pending
**Priority:** P2
**Module:** Display
**FR Reference:** FR-028

**Task Details:**
- [ ] Enhance focus mode to properly dim/hide non-current paragraphs
- [ ] Test with various document lengths
- [ ] Verify distraction reduction per FR-028

---

### G-014: Typewriter Mode Quality
**Status:** Pending
**Priority:** P2
**Module:** Display
**FR Reference:** FR-029

**Task Details:**
- [ ] Fix typewriter mode scroll behavior
- [ ] Verify cursor stays at vertical center during navigation
- [ ] Test with various document lengths

---

### G-015: Paste Handling
**Status:** Pending
**Priority:** P2
**Module:** Editor
**FR Reference:** FR-018

**Task Details:**
- [ ] Improve paste handling for common rich text formats
- [ ] Test with various paste sources
- [ ] Ensure Markdown conversion fidelity

---

### G-016: Service Interfaces
**Status:** Pending
**Priority:** P2
**Module:** Services
**FR Reference:** NFR-005

**Task Details:**
- [ ] Refactor to expose DocumentService, EditorService, etc.
- [ ] Align with PRD-10 architecture
- [ ] Add proper interface boundaries

---

### G-017: Autosave Reliability
**Status:** Pending
**Priority:** P2
**Module:** Recovery
**FR Reference:** FR-005

**Task Details:**
- [ ] Consider Rust-side autosave with debounce
- [ ] Ensure frontend timer fires reliably
- [ ] Test crash recovery scenarios

---

### G-018: Syntax Highlighter
**Status:** Pending
**Priority:** P2
**Module:** Editor
**FR Reference:** FR-015

**Task Details:**
- [ ] Verify SyntaxHighlighter implementation matches PRD (Shiki vs syntect)
- [ ] Document any deviations from PRD specification
- [ ] Consider Shiki integration if required by PRD

---

## P3 - Low Priority / Deferred

---

### G-019: Table Cell Editing Safety
**Status:** Deferred
**Priority:** P3
**Module:** Editor
**FR Reference:** FR-016

**Note:** PRD acceptable - monitor only, no action for MVP.

---

### G-020: Fuzz Testing
**Status:** Deferred
**Priority:** P3
**Module:** Security

**Note:** Deferred to post-MVP.

---

### G-021: IME Composition Handling
**Status:** Deferred
**Priority:** P3
**Module:** Frontend

**Note:** Test post-MVP.

---

## Completed Tasks (from previous iterations)

### File Operations ✅
- [x] FR-001: New file - create untitled, save to chosen location
- [x] FR-002: Open file - open Markdown from disk, drag-and-drop
- [x] FR-003: Open folder - workspace with sidebar file tree
- [x] FR-004: Save - manual save, atomic write
- [x] FR-005: Auto-save - configurable, dirty-state indication
- [x] FR-006: Recovery - restore after crash/force close
- [x] FR-007: External changes - detect, prompt user

### Core Editing ✅ (Partial - see P0 issues)
- [x] FR-008: Single-pane live rendering (partially)
- [x] FR-009: Heading behavior
- [x] FR-010: Emphasis behavior
- [x] FR-013: Task list behavior
- [x] FR-017: Image behavior (partial)
- [x] FR-018: Paste behavior (partial)

### UI Components ✅
- [x] LinkPopover component
- [x] CodeBlockHighlight component
- [x] FrontmatterBlock component
- [x] Recovery modal UI
- [x] Dirty state indicator
- [x] External change modal
- [x] SearchPanel
- [x] OutlinePanel

### Backend Components ✅ (Partial - see P0 issues)
- [x] Document commands
- [x] Workspace commands
- [x] File tree commands
- [x] Export commands (partial)
- [x] Recovery commands
- [x] Render commands
- [x] Image commands

---

## Summary

| Priority | Total | Completed | Pending | Deferred |
|----------|-------|-----------|---------|----------|
| P0 | 3 | 0 | 3 | 0 |
| P1 | 6 | 0 | 6 | 0 |
| P2 | 9 | 0 | 9 | 0 |
| P3 | 3 | 0 | 0 | 3 |
| **Total** | **21** | **0** | **18** | **3** |

---

## Next Steps

1. **IMMEDIATE:** Start G-001 (TransformEngine) - this blocks MVP completion
2. **PARALLEL:** Start G-002 (Undo/Redo) and G-003 (rusqlite Settings) 
3. **AFTER P0:** Address G-004 through G-009 (P1 issues)
4. **TIME PERMITTING:** Address G-010 through G-018 (P2 issues)
5. **POST-MVP:** G-019, G-020, G-021

---

## Iteration Checkpoint

```
iteration=5
phase=phase1
timestamp=1744569600
```

---

*Task list generated from Iteration-5 gap analysis - P0 issues prioritized*