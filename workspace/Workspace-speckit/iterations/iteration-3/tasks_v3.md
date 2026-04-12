# RustNote Task List - Iteration 3 (v3)

**Project:** RustNote - Typora-like Markdown Editor  
**Version:** 3.1  
**Task List Version:** 3  
**Date:** 2026-04-12  
**Status:** Ready for Implementation  

---

## Priority Legend
- **P0:** Blocking (Must Fix) - NONE
- **P1:** High Priority (Must Implement)
- **P2:** Medium Priority
- **P3:** Low Priority / Deferred

---

## P1 Tasks (High Priority - Must Implement)

### Task 1.1: Image Paste Support
- **Task ID:** T-001
- **FR-ID:** FR-017
- **Module:** Frontend (TipTapEditor)
- **Priority:** P1
- **Estimated Effort:** Medium
- **Dependencies:** Backend image.rs (already exists)

**Description:**
Implement clipboard image paste functionality in TipTapEditor. Currently only dialog-based image insertion works.

**Acceptance Criteria:**
- [ ] User can paste image from clipboard using Ctrl+V
- [ ] Pasted image is saved to document directory via backend
- [ ] Markdown image reference is inserted at cursor position
- [ ] Image is rendered correctly in editor

**Implementation Steps:**
1. Add paste event handler to TipTapEditor
2. Detect image data (image/png, image/jpeg) in paste event
3. Extract image blob from clipboard
4. Call backend image.save command to save image to disk
5. Insert Markdown image syntax at cursor position
6. Verify rendering in editor

**File:** `www/src/components/TipTapEditor.jsx`

---

### Task 1.2: Undo/Redo Keyboard Shortcuts
- **Task ID:** T-002
- **FR-ID:** FR-019
- **Module:** Frontend (TipTapEditor)
- **Priority:** P1
- **Estimated Effort:** Low
- **Dependencies:** None

**Description:**
Wire Ctrl+Z (undo) and Ctrl+Y (redo) keyboard shortcuts to TipTap's built-in undo/redo functionality.

**Acceptance Criteria:**
- [ ] Ctrl+Z triggers undo
- [ ] Ctrl+Y triggers redo
- [ ] Undo/redo works across all editing operations
- [ ] No conflicts with other handlers

**Implementation Steps:**
1. Add keyboard event listener to TipTapEditor
2. Handle Ctrl+Z for undo, Ctrl+Y for redo
3. Test undo/redo across different editing operations
4. Verify no conflicts with browser defaults

**File:** `www/src/components/TipTapEditor.jsx`

---

### Task 1.3: Recovery UI on Startup
- **Task ID:** T-003
- **FR-ID:** FR-006
- **Module:** Frontend (App)
- **Priority:** P1
- **Estimated Effort:** Medium
- **Dependencies:** Backend recovery.rs (already saves snapshots)

**Description:**
Add startup recovery prompt when crash recovery snapshots exist. Currently snapshots are saved but no UI prompts the user.

**Acceptance Criteria:**
- [ ] App checks for snapshots on startup
- [ ] If snapshots exist, recovery modal is shown
- [ ] User can choose to recover a snapshot or start fresh
- [ ] Recovered content loads in editor

**Implementation Steps:**
1. Create RecoveryModal.jsx component
2. On app startup, call backend to check for snapshots
3. If snapshots exist, show RecoveryModal before main editor
4. Display list of available snapshots with timestamps
5. On selection, load snapshot content into editor
6. On dismiss, start with empty/new document

**Files:**
- `www/src/components/RecoveryModal.jsx` (new)
- `www/src/App.jsx` (add startup check)

---

## P2 Tasks (Medium Priority)

### Task 2.1: Dirty State Indicator
- **Task ID:** T-004
- **FR-ID:** FR-005
- **Module:** Frontend (App/TitleBar)
- **Priority:** P2
- **Estimated Effort:** Low
- **Dependencies:** None

**Description:**
Add visual indicator when document has unsaved changes. isDirty state already exists but no visual feedback.

**Acceptance Criteria:**
- [ ] Asterisk (*) appears in title when document is dirty
- [ ] Asterisk disappears after save
- [ ] Indicator visible in both light and dark themes

**Implementation Steps:**
1. Modify title display in App component
2. Add asterisk prefix when document.isDirty is true
3. Style indicator appropriately for themes

**File:** `www/src/App.jsx`

---

### Task 2.2: Keyboard Shortcut - New Document (Ctrl+N)
- **Task ID:** T-005
- **FR-ID:** FR-001
- **Module:** Frontend (App)
- **Priority:** P2
- **Estimated Effort:** Low
- **Dependencies:** None

**Description:**
Wire Ctrl+N keyboard shortcut to create new document.

**Acceptance Criteria:**
- [ ] Ctrl+N creates new untitled document
- [ ] If current document is dirty, prompt for save first

**Implementation Steps:**
1. Add keyboard event handler in App component
2. Handle Ctrl+N event
3. Check for dirty state and prompt if needed
4. Reset document state to new untitled document

**File:** `www/src/App.jsx`

---

### Task 2.3: Keyboard Shortcut - Open File (Ctrl+O)
- **Task ID:** T-006
- **FR-ID:** FR-002
- **Module:** Frontend (App)
- **Priority:** P2
- **Estimated Effort:** Low
- **Dependencies:** None

**Description:**
Wire Ctrl+O keyboard shortcut to open file dialog.

**Acceptance Criteria:**
- [ ] Ctrl+O opens native file dialog
- [ ] Selected file loads in editor
- [ ] If current document is dirty, prompt for save first

**Implementation Steps:**
1. Add keyboard event handler in App component
2. Handle Ctrl+O event
3. Check for dirty state and prompt if needed
4. Call backend open_file command
5. Load selected file into editor

**File:** `www/src/App.jsx`

---

### Task 2.4: Keyboard Shortcut - Open Folder (Ctrl+Shift+O)
- **Task ID:** T-007
- **FR-ID:** FR-003
- **Module:** Frontend (App)
- **Priority:** P2
- **Estimated Effort:** Low
- **Dependencies:** None

**Description:**
Wire Ctrl+Shift+O keyboard shortcut to open workspace folder.

**Acceptance Criteria:**
- [ ] Ctrl+Shift+O opens folder selection dialog
- [ ] Selected folder becomes workspace
- [ ] File tree populates in sidebar

**Implementation Steps:**
1. Add keyboard event handler in App component
2. Handle Ctrl+Shift+O event
3. Call backend open_folder command
4. Update workspace state and sidebar

**File:** `www/src/App.jsx`

---

### Task 2.5: Code Syntax Highlighting
- **Task ID:** T-008
- **FR-ID:** FR-015
- **Module:** Frontend (TipTapEditor)
- **Priority:** P2
- **Estimated Effort:** Medium
- **Dependencies:** Backend highlight_code_block exists

**Description:**
Integrate backend syntax highlighting into TipTap code blocks. Backend has highlight_code_block but TipTap doesn't use it.

**Acceptance Criteria:**
- [ ] Code blocks render with syntax highlighting
- [ ] Multiple languages supported
- [ ] Highlighting matches VS Code-like theme

**Implementation Steps:**
1. Create CodeBlock extension for TipTap
2. Call backend highlight_code_block command
3. Apply HTML highlighting to code block content
4. Support language detection
5. Style highlighted code appropriately

**Files:**
- `www/src/components/TipTapEditor.jsx`
- `www/src/extensions/CodeBlockHighlight.jsx` (new)

---

### Task 2.6: Link Editing Behavior
- **Task ID:** T-009
- **FR-ID:** FR-011
- **Module:** Frontend (TipTapEditor)
- **Priority:** P2
- **Estimated Effort:** Low
- **Dependencies:** None

**Description:**
Define and implement consistent link click behavior. Currently links render but click behavior is undefined.

**Acceptance Criteria:**
- [ ] Click on link focuses for editing (not following)
- [ ] Double-click opens link editor dialog
- [ ] External links (http://) open in browser

**Implementation Steps:**
1. Define link click behavior spec
2. Implement single-click to focus for editing
3. Implement double-click to open edit dialog
4. Handle external links (http://) to open in browser

**File:** `www/src/components/TipTapEditor.jsx`

---

## P3 Tasks (Deferred - Post-MVP)

### Task 3.1: Frontmatter Special Rendering
- **Task ID:** T-010
- **FR-ID:** FR-020
- **Module:** Frontend (TipTapEditor)
- **Priority:** P3
- **Estimated Effort:** Medium
- **Status:** Deferred to post-MVP

**Description:**
Display frontmatter as collapsible block at document start.

**Acceptance Criteria:**
- [ ] Frontmatter section collapsible
- [ ] Styled distinctly from body content
- [ ] Edit mode for frontmatter fields

**Implementation Steps:** (Deferred)
1. Detect frontmatter at document start
2. Render as collapsible section
3. Style with distinct appearance
4. Provide edit mode for frontmatter

---

### Task 3.2: Table Cell Editing Safety
- **Task ID:** T-011
- **FR-ID:** FR-016
- **Module:** Frontend (TipTapEditor)
- **Priority:** P3
- **Estimated Effort:** Medium
- **Status:** Deferred to post-MVP

**Description:**
Implement constrained safe table editing model to prevent table structure corruption.

**Acceptance Criteria:**
- [ ] Tab navigates between cells
- [ ] Enter does not create new rows incorrectly
- [ ] Backspace at cell start does not delete row

**Implementation Steps:** (Deferred)
1. Create custom table cell behavior
2. Handle Tab/Enter/Backspace specifically for tables
3. Prevent structure corruption

---

## Technical Debt Tasks (Ongoing)

### Task 4.1: TipTap Integration Completeness
- **Task ID:** T-012
- **Module:** Frontend
- **Effort:** Medium
- **Status:** In Progress

**Description:**
Not using all TipTap capabilities fully. Many extensions available but not utilized.

**Actions:**
- Review available TipTap extensions
- Implement missing valuable features
- Improve editor capabilities

---

### Task 4.2: Editor State Sync Monitoring
- **Task ID:** T-013
- **Module:** Frontend
- **Effort:** High
- **Status:** Needs Monitoring

**Description:**
TipTap + Markdown conversion may lose fidelity on round-trip.

**Actions:**
- Establish edge case tests
- Monitor for reported fidelity issues
- Consider alternative approaches post-MVP

---

### Task 4.3: Keyboard Shortcut Centralization
- **Task ID:** T-014
- **Module:** Frontend
- **Effort:** Low
- **Status:** Needs Work

**Description:**
Many keyboard shortcuts not wired. Need centralized shortcut handler.

**Actions:**
- Create centralized keyboard handler
- Document all shortcuts
- Implement missing shortcuts

---

## Task Summary

| Task ID | Task | Priority | Effort | Status |
|---------|------|----------|--------|--------|
| T-001 | Image Paste Support | P1 | Medium | Done |
| T-002 | Undo/Redo Shortcuts | P1 | Low | Pending |
| T-003 | Recovery UI on Startup | P1 | Medium | Pending |
| T-004 | Dirty State Indicator | P2 | Low | Pending |
| T-005 | Ctrl+N (New) Shortcut | P2 | Low | Pending |
| T-006 | Ctrl+O (Open) Shortcut | P2 | Low | Pending |
| T-007 | Ctrl+Shift+O (Folder) Shortcut | P2 | Low | Pending |
| T-008 | Code Syntax Highlighting | P2 | Medium | Pending |
| T-009 | Link Editing Behavior | P2 | Low | Pending |
| T-010 | Frontmatter Rendering | P3 | Medium | Deferred |
| T-011 | Table Cell Editing Safety | P3 | Medium | Deferred |
| T-012 | TipTap Integration | Tech Debt | Medium | In Progress |
| T-013 | Editor State Sync | Tech Debt | High | Monitoring |
| T-014 | Shortcut Centralization | Tech Debt | Low | Needs Work |

---

## Dependencies Graph

```
T-001 (Image Paste) ──────► image.rs (Backend) ✅
T-008 (Syntax Highlight) ──► highlight_code_block (Backend) ✅
T-003 (Recovery UI) ──────► recovery.rs (Backend) ✅

T-005, T-006, T-007 (Shortcuts) ──► App.jsx
T-002 (Undo/Redo) ─────────► TipTapEditor.jsx
T-004 (Dirty Indicator) ──► App.jsx
T-009 (Link Behavior) ─────► TipTapEditor.jsx
```

---

## Iteration 3 Task Completion Checklist

### Before Starting
- [ ] Read plan_v3.md thoroughly
- [ ] Understand P1 priorities
- [ ] Set up development environment

### P1 Completion (Target: 1 day)
- [x] T-001 Image Paste - Complete
- [ ] T-002 Undo/Redo - Complete
- [ ] T-003 Recovery UI - Complete

### P2 Completion (Target: 0.5 day)
- [ ] T-004 Dirty Indicator - Complete
- [ ] T-005 Ctrl+N - Complete
- [ ] T-006 Ctrl+O - Complete
- [ ] T-007 Ctrl+Shift+O - Complete
- [ ] T-008 Syntax Highlighting - Complete
- [ ] T-009 Link Behavior - Complete

### Post-Implementation
- [ ] Run all existing tests
- [ ] Manual testing checklist
- [ ] Performance verification
- [ ] Update checkpoint

---

*Task list version 3 - Updated based on Iteration-3 gap analysis*
