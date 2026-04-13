# RustNote Task List - Iteration 4 (v4)

**Project:** RustNote - Typora-like Markdown Editor  
**Version:** 4.0  
**Based on:** spec_v4.md, gap-analysis.md, plan_v4.md  
**Date:** 2026-04-13  
**Priority:** P0=none, P1=critical, P2=important, P3=deferred  

---

## P0 - Blocking Issues: NONE ✅

All P0 issues have been resolved in previous iterations.

---

## P1 - High Priority

### IT-001: Link Editing Behavior
**Status:** Pending  
**Priority:** P1  
**Module:** Frontend  
**FR Reference:** FR-011  

**Task Details:**
- [ ] Create `LinkPopover.jsx` component with:
  - [ ] URL input field
  - [ ] Text input field  
  - [ ] Save/Cancel buttons
  - [ ] Delete link option
- [ ] Integrate LinkPopover with TipTap Link extension
- [ ] Define click behavior: click on link opens popover
- [ ] Define Ctrl+click behavior: follows URL in new tab
- [ ] Handle keyboard: Enter saves, Escape closes
- [ ] Position popover near the link
- [ ] Test with various link formats (URL, relative, anchor)

**Files:**
- NEW: `www/src/components/LinkPopover.jsx`
- MOD: `www/src/components/TipTapEditor.jsx`

**Verification:**
- Links are clickable and open editor popover
- Ctrl+click opens URL in new tab
- Popover positions correctly near links
- Keyboard navigation works (Enter/Escape)

---

### IT-002: Done
**Status:** Done  
**Priority:** P1  
**Module:** Frontend  
**FR Reference:** FR-015  

**Task Details:**
- [x] Verify `CodeBlockHighlight.jsx` component exists and works
- [x] Check backend `highlight_code_block` command implementation
- [x] Wire CodeBlockHighlight into TipTap code block node rendering
- [x] Ensure language detection works (js, ts, python, rust, etc.)
- [x] Test highlighting renders correctly for fenced code blocks
- [x] Verify scroll behavior for long code blocks
- [x] Check dark/light theme compatibility

**Files:**
- MOD: `www/src/components/TipTapEditor.jsx`
- MOD: `www/src/components/CodeBlockHighlight.jsx`
- CHECK: `src-tauri/src/commands/editor.rs`

**Verification:**
- Code blocks render with syntax highlighting
- Language is detected correctly
- Theme toggle maintains highlighting visibility

---

## P2 - Medium Priority

### IT-003: Done
**Status:** Done  
**Priority:** P2  
**Module:** Frontend  
**FR Reference:** FR-020  

**Task Details:**
- [x] Create `FrontmatterBlock.jsx` component
- [x] Detect frontmatter at document start (YAML `---` delimited)
- [x] Display as collapsible block with toggle
- [x] Apply syntax styling (key: value pairs)
- [x] Allow expand/collapse toggle
- [x] Parse and display metadata (title, tags, date, etc.)
- [x] Handle missing/invalid frontmatter gracefully

**Files:**
- NEW: `www/src/components/FrontmatterBlock.jsx`
- MOD: `www/src/components/TipTapEditor.jsx`

**Verification:**
- Frontmatter displays as styled collapsible block
- Toggle expands/collapses content
- Invalid frontmatter shows error state

---

### IT-004: Ropey Buffer for Large Documents
**Status:** Done
**Priority:** P2
**Module:** Backend
**FR Reference:** NFR-003

**Task Details:**
- [x] Research ropey crate
- [x] Design buffer interface
- [x] Implement ropey-based text buffer
- [x] Update editor commands to use buffer
- [x] Test with 5MB+ documents

**Files:**
- NEW: `src-tauri/src/buffer/mod.rs`
- MOD: `src-tauri/Cargo.toml` (added ropey dependency)
- MOD: `src-tauri/src/lib.rs` (added buffer module)

**Verification:**
- [x] cargo build passes
- [x] cargo test passes (94 lib tests, 207 integration tests)
- [x] Large document test (5MB+) passes

**Implementation Notes:**
- Created TextBuffer struct wrapping ropey::Rope
- O(log n) insertions and deletions
- Efficient line-based operations
- UTF-8 correct
- Full test coverage for buffer operations

---

### IT-005: Parser Migration Path
**Status:** Deferred  
**Priority:** P2 (Deferred)  
**Module:** Backend  
**FR Reference:** NFR-004  

**Task Details:**
- [ ] Research tree-sitter + comrak
- [ ] Design migration strategy
- [ ] Implement incremental parsing
- [ ] Ensure backward compatibility

**Files:**
- MOD: `src-tauri/src/parser/`

**Note:** Deferred per PRD acknowledgment (MVP uses pulldown-cmark).

---

### IT-006: ProseMirror Markdown Bridge
**Status:** Deferred  
**Priority:** P2 (Deferred)  
**Module:** Frontend  
**FR Reference:** FR-022  

**Task Details:**
- [ ] Evaluate prosemirror-markdown library
- [ ] Design bridge architecture
- [ ] Implement serialization/deserialization
- [ ] Ensure round-trip fidelity

**Files:**
- MOD: `www/src/components/TipTapEditor.jsx`
- MOD: `www/src/contexts/`

**Note:** Deferred per PRD acknowledgment (MVP uses marked.js).

---

### IT-007: Visual Regression Testing Baseline
**Status:** Pending  
**Priority:** P2  
**Module:** Testing  
**FR Reference:** Test Plan Section 7  

**Task Details:**
- [ ] Review existing Playwright visual tests
- [ ] Run screenshot tests to establish baselines
- [ ] Save baselines to `visual-baselines/` directory
- [ ] Document baseline capture process
- [ ] Add baseline verification to CI

**Files:**
- MOD: `e2e/visual/`
- NEW: `visual-baselines/` (if not exists)

**Verification:**
- All VIS-* test cases have baseline images
- Running tests produces zero unexpected differences

---

## P3 - Low Priority / Deferred

### IT-008: Table Cell Editing Safety
**Status:** Deferred  
**Priority:** P3  
**Module:** Frontend  
**FR Reference:** FR-016  

**Note:** PRD acceptable - monitor only, no action for MVP.

---

### IT-009: Fuzz Testing
**Status:** Deferred  
**Priority:** P3  
**Module:** Security  

**Note:** Deferred to post-MVP.

---

### IT-010: IME Composition Handling
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

### Core Editing ✅
- [x] FR-008: Single-pane live rendering
- [x] FR-009: Heading behavior
- [x] FR-010: Emphasis behavior
- [x] FR-012: List behavior
- [x] FR-013: Task list behavior
- [x] FR-014: Blockquote behavior
- [x] FR-017: Image behavior
- [x] FR-018: Paste behavior
- [x] FR-019: Undo/redo

### Keyboard Shortcuts ✅
- [x] Ctrl+N (new document)
- [x] Ctrl+O (open file)
- [x] Ctrl+Shift+O (open folder)
- [x] Ctrl+Z (undo)
- [x] Ctrl+Y (redo)

### UI Components ✅
- [x] Recovery modal UI (full implementation)
- [x] Dirty state indicator (* in toolbar)
- [x] External change modal

---

## Summary

| Priority | Total | Completed | Pending | Deferred |
|----------|-------|-----------|---------|----------|
| P0 | 0 | 0 | 0 | 0 |
| P1 | 2 | 0 | 2 | 0 |
| P2 | 5 | 0 | 4 | 1 (IT-004) |
| P3 | 3 | 0 | 0 | 3 |
| **Total** | **10** | **0** | **6** | **4** |

---

## Next Steps

1. **Immediate:** Start IT-001 (Link Editing) and IT-002 (Code Highlighting)
2. **After P1:** Complete IT-003 (Frontmatter) and IT-007 (Visual Baselines)
3. **Post-MVP:** IT-004, IT-005, IT-006, IT-009, IT-010

---

## Iteration Checkpoint

```
iteration=4
phase=phase1
timestamp=1776012722
```
