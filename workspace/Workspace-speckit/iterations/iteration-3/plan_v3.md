# RustNote Implementation Plan - Iteration 3 (v3)

**Project:** RustNote - Typora-like Markdown Editor  
**Version:** 3.1  
**Plan Version:** 3  
**Date:** 2026-04-12  
**Status:** Ready for Implementation  

---

## 1. Overview

This plan addresses the remaining gaps identified in the Iteration-3 gap analysis. Implementation is at **90-95%** complete. The focus of this iteration is to resolve the remaining P1 and P2 issues to achieve MVP completion.

### 1.1 Implementation Status Summary

| Category | Status | Notes |
|----------|--------|-------|
| File Operations | ✅ 95% | Near complete |
| Document Model | ✅ 95% | Stable |
| Editor Transforms | ✅ 85% | Stable |
| Frontend Sidebar | ✅ 90% | CRUD complete |
| Frontend Outline | ✅ 95% | Navigation complete |
| Export (PDF) | ✅ 80% | Working |
| Workspace | ✅ 95% | CRUD complete |
| Frontend Editor | ✅ 90% | Near complete |
| Recovery | ⚠️ 90% | Snapshots work, UI pending |
| Settings | ✅ 98% | Near complete |

### 1.2 P0/P1/P2 Classification

**P0 - Blocking Issues:** NONE (all resolved)

**P1 - High Priority (Must Fix):**
1. Image paste not implemented (FR-017)
2. Undo/redo Ctrl+Z/Y shortcuts not wired (FR-019)
3. Recovery snapshots saved but no user prompt (FR-006)

**P2 - Medium Priority:**
4. Dirty state indicator not visible (FR-005)
5. Ctrl+N (new) shortcut not wired (FR-001)
6. Ctrl+O (open) shortcut not wired (FR-002)
7. Ctrl+Shift+O (open folder) not wired (FR-003)
8. Code syntax highlighting not visible (FR-015)
9. Link editing behavior unclear (FR-011)

**P3 - Low Priority/Deferred:**
- Frontmatter rendering (deferred post-MVP)
- Table cell editing safety (monitor post-MVP)

---

## 2. Technical Debt Summary

| Item | Description | Effort | Status |
|------|-------------|--------|--------|
| TipTap Integration | Not using all TipTap capabilities fully | Medium | In Progress |
| Editor State Sync | TipTap + Markdown conversion may lose fidelity | High | Needs Monitoring |
| Buffer Layer | No ropey-based text buffer for large documents | High | Not Started |
| Parser Layer | Using marked instead of tree-sitter as specified | Medium | Partial |
| Export Architecture | Export commands exist but PDF limited | Medium | Working |
| Test Coverage | Basic tests exist, missing integration tests | High | Needs Work |
| CSS Architecture | Design tokens partially implemented | Medium | Partial |
| State Management | React Context only, no proper state library | Low | Acceptable |
| Recovery UI | No startup prompt for crash recovery | Low | Not Started |
| Keyboard Shortcuts | Many shortcuts not wired | Low | Needs Work |

---

## 3. Implementation Priorities

### 3.1 P1 Tasks (Must Implement)

#### Task 1.1: Image Paste Support
- **FR-ID:** FR-017
- **Module:** Frontend (TipTapEditor)
- **Issue:** Only dialog-based image insertion works; users expect Cmd+V for images
- **Fix:** Handle paste events for image data in TipTapEditor
- **Backend Dependency:** Backend image commands already exist (image.rs)
- **Effort:** Medium

#### Task 1.2: Undo/Redo Keyboard Shortcuts
- **FR-ID:** FR-019
- **Module:** Frontend (TipTapEditor)
- **Issue:** TipTap has built-in undo/redo but Ctrl+Z/Y shortcuts not connected
- **Fix:** Add keyboard event handlers for undo/redo
- **Effort:** Low

#### Task 1.3: Recovery UI on Startup
- **FR-ID:** FR-006
- **Module:** Frontend (App)
- **Issue:** Backend saves snapshots, no UI prompt on startup
- **Fix:** Check for snapshots on startup, prompt user if recovery available
- **Backend Dependency:** recovery.rs already saves snapshots
- **Effort:** Medium

### 3.2 P2 Tasks

#### Task 2.1: Dirty State Indicator
- **FR-ID:** FR-005
- **Module:** Frontend (App/TitleBar)
- **Issue:** isDirty exists in state but no visual indicator
- **Fix:** Add asterisk/dot in title/toolbar when document has unsaved changes
- **Effort:** Low

#### Task 2.2: Keyboard Shortcuts (Ctrl+N, Ctrl+O, Ctrl+Shift+O)
- **FR-IDs:** FR-001, FR-002, FR-003
- **Module:** Frontend (App)
- **Issue:** Standard keyboard shortcuts not wired
- **Fix:** Add keyboard shortcut handlers for new/open/open folder
- **Effort:** Low

#### Task 2.3: Code Syntax Highlighting
- **FR-ID:** FR-015
- **Module:** Frontend (TipTapEditor)
- **Issue:** Backend has highlight_code_block, TipTap doesn't use it
- **Fix:** Integrate syntect/highlighting into TipTap code blocks
- **Backend Dependency:** Backend highlight_code_block exists
- **Effort:** Medium

#### Task 2.4: Link Editing Behavior
- **FR-ID:** FR-011
- **Module:** Frontend (TipTapEditor)
- **Issue:** Links render but click behavior undefined
- **Fix:** Define and implement link click behavior (edit vs follow)
- **Effort:** Low

### 3.3 P3 Tasks (Deferred)

#### Task 3.1: Frontmatter Special Rendering
- **FR-ID:** FR-020
- **Module:** Frontend (TipTapEditor)
- **Issue:** Frontmatter displayed without special treatment
- **Fix:** Display as collapsible block (post-MVP)
- **Effort:** Medium
- **Status:** Deferred

#### Task 3.2: Table Cell Editing Safety
- **FR-ID:** FR-016
- **Module:** Frontend (TipTapEditor)
- **Issue:** Table editing safety incomplete
- **Fix:** Implement constrained safe table editing model (post-MVP)
- **Effort:** Medium
- **Status:** Deferred

---

## 4. Architecture Notes

### 4.1 Editor State Sync
The TipTap + Markdown conversion pipeline needs monitoring:
- User edits → TipTap JSON → Markdown serialization
- Potential fidelity loss on round-trip
- Currently acceptable for MVP

### 4.2 Keyboard Shortcut Architecture
All keyboard shortcuts should be handled at the App level in a central handler:
- Avoids conflicts between components
- Consistent behavior across editor states
- Easy to audit and modify

### 4.3 Image Handling Flow
Current flow (dialog-based):
1. User clicks insert image
2. Dialog opens for path/URL
3. Backend processes image command
4. Image inserted as Markdown

New flow (paste-based):
1. User pastes image from clipboard
2. Frontend detects image data in paste event
3. Image saved to document directory via backend
4. Markdown image reference inserted

### 4.4 Recovery Flow
1. App starts
2. Frontend calls backend to check for snapshots
3. If snapshots exist, show recovery modal
4. User selects snapshot to recover
5. Backend loads snapshot and returns content

---

## 5. File Structure

### 5.1 Backend Commands (src-tauri/src/commands/)
```
commands/
├── document.rs    # Document CRUD, headings
├── editor.rs      # Transform engine, search
├── export.rs      # HTML/PDF export
├── file_tree.rs  # Workspace file operations
├── file_watcher.rs # External change detection
├── image.rs       # Image handling
├── recovery.rs    # Snapshot management
├── render.rs      # Markdown rendering
├── settings.rs    # Settings persistence
└── workspace.rs   # Workspace management
```

### 5.2 Frontend Components (www/src/)
```
components/
├── TipTapEditor.jsx     # Main editor (replaces Editor.jsx)
├── Editor.jsx           # Legacy contenteditable (to be deprecated)
├── Sidebar.jsx         # File tree with CRUD
├── OutlinePanel.jsx    # TOC with click navigation
├── SearchPanel.jsx     # Find/replace
├── Toolbar.jsx         # Mode toggles, actions
├── ExportModal.jsx     # Export dialog
├── DropZone.jsx       # Drag-and-drop handler
├── ExternalChangeModal.jsx # External change prompt
└── RecoveryModal.jsx  # NEW: Recovery prompt (to be created)

contexts/
├── DocumentContext.jsx  # Document state
├── SettingsContext.jsx # Settings state
└── SearchContext.jsx   # Search state

hooks/
├── useAutoSaveTimer.js # Auto-save timer
└── useFileWatcher.js   # File watcher integration
```

---

## 6. Dependencies

### 6.1 Backend Dependencies
- `marked` - Markdown parsing (vs tree-sitter - future)
- `syntect` - Syntax highlighting (wired to backend, needs frontend integration)
- `ropey` - Text buffer (future, for large documents)
- `serde` - Serialization
- `tauri` - Desktop framework

### 6.2 Frontend Dependencies
- `TipTap` - Rich text editor framework
- `@tiptap/extension-*` - TipTap extensions
- `react` - UI framework
- `marked` - Markdown parsing (for preview)

---

## 7. Testing Strategy

### 7.1 Unit Tests
- Transform engine tests (existing)
- Parser tests (existing)
- Settings tests (existing)
- Editor engine tests (existing)

### 7.2 Integration Tests
- Document open/save cycle
- Auto-save triggers
- Recovery flow
- Export (HTML/PDF)
- Keyboard shortcuts

### 7.3 Manual Testing Checklist
- [ ] Image paste works (Ctrl+V with image in clipboard)
- [ ] Undo/redo works (Ctrl+Z/Y)
- [ ] Recovery prompt appears on startup (if snapshot exists)
- [ ] Dirty indicator shows asterisk when unsaved
- [ ] Ctrl+N creates new document
- [ ] Ctrl+O opens file dialog
- [ ] Ctrl+Shift+O opens folder dialog
- [ ] Code blocks show syntax highlighting
- [ ] Link click behavior is consistent

---

## 8. Success Criteria

### 8.1 MVP Completion
- [ ] All P1 issues resolved
- [ ] All P2 issues resolved or formally deferred
- [ ] No blocking bugs
- [ ] Performance thresholds met

### 8.2 Performance Thresholds
| Metric | Target | Status |
|--------|--------|--------|
| Cold startup (empty doc) | < 2 seconds | ✅ Target |
| Cold startup (1MB doc) | < 3 seconds | ✅ Target |
| Hot reload (file open after first) | < 500ms | ✅ Target |
| Edit-to-render latency | < 100ms | ✅ Target |
| Save operation | < 200ms | ✅ Target |
| PDF export (10 pages) | < 5 seconds | ✅ Target |
| Memory (idle, 10 docs open) | < 300MB | ✅ Target |
| Large document (5MB) scrolling | 60 FPS | ✅ Target |

---

## 9. Risks and Mitigations

### 9.1 TipTap + Markdown Fidelity
- **Risk:** Edit-save-reopen may lose some formatting
- **Mitigation:** Monitor, establish test cases for edge cases
- **Status:** Acceptable for MVP

### 9.2 Image Paste File Size
- **Risk:** Large images pasted could cause performance issues
- **Mitigation:** Implement image compression/resizing before save
- **Status:** Future enhancement

### 9.3 Recovery Snapshot Storage
- **Risk:** Snapshots could accumulate and use disk space
- **Mitigation:** Implement cleanup of old snapshots
- **Status:** Future enhancement

---

## 10. Milestones

### Milestone 1: P1 Issues (Target: 1 day)
- [ ] Image paste implemented
- [ ] Undo/redo shortcuts wired
- [ ] Recovery UI on startup

### Milestone 2: P2 Issues (Target: 0.5 day)
- [ ] Dirty state indicator added
- [ ] Keyboard shortcuts wired (Ctrl+N, Ctrl+O, Ctrl+Shift+O)
- [ ] Code syntax highlighting integrated
- [ ] Link editing behavior defined

### Milestone 3: MVP Sign-off (Target: 0.5 day)
- [ ] All functional requirements verified
- [ ] Performance thresholds confirmed
- [ ] No blocking bugs

---

*Plan version 3 - Updated based on Iteration-3 gap analysis*
