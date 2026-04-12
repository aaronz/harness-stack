# Constitution Update Suggestions - Iteration 2

**Project:** RustNote
**Date:** 2026-04-12
**Status:** No Constitution exists. Previous iteration-1 suggestions were never implemented.
**Reference Gap Analysis:** iterations/iteration-2/gap-analysis.md

---

## Executive Summary

Iteration-1 P0 issues (Find/Replace UI, Focus Mode, Typewriter Mode, Auto-save Timer) have been **successfully resolved**. However, new P1/P2 issues emerged that require Constitution coverage to prevent recurrence:

| Priority | Issue | Status |
|----------|-------|--------|
| P1 | File tree lacks create/rename/delete | Unchanged from iteration-1 |
| P1 | Drag-and-drop file open not implemented | New P1 |
| P2 | Settings (lineHeight, fontSize) not applied to TipTap | New issue |
| P2 | Undo/redo shortcuts not wired | Unchanged from iteration-1 |
| P2 | Export UI not implemented | Unchanged from iteration-1 |

**Critical Pattern:** The iteration-1 Constitution suggestions addressed "scaffolding without implementation" but were never ratified. The same anti-pattern persists for new features.

---

## Iteration-1 Articles Status

The iteration-1/constitution_updates.md proposed 5 articles. None were implemented:

| Article | Status |
|---------|--------|
| Article 1: Complete Feature Implementation Rule | ❌ Not Ratified |
| Article 2: P0 Feature Lifecycle Guarantee | ❌ Not Ratified |
| Article 3: Settings-Implementation Parity | ❌ Not Ratified |
| Article 4: CSS-Logic Pairing | ❌ Not Ratified |
| Article 5: Integration Before Merge | ❌ Not Ratified |

**Impact:** Iteration-2 Gap Analysis confirms these articles would have helped:
- Article 1 (Complete Implementation) → Would have prevented Settings not applied to TipTap
- Article 3 (Settings-Implementation Parity) → Would have flagged `lineHeight`, `fontSize` unused
- Article 5 (Integration Before Merge) → Would have flagged export UI missing

---

## Recommended Constitution Articles

### Article 1: Complete Feature Implementation Rule

**Intent:** Prevent "70% done" features where UI scaffolding exists without functional backend/frontend logic.

**Text:**
> Any feature marked as implemented in iteration status must have BOTH:
> 1. Visible UI element (component, CSS class, or user-triggered control), AND
> 2. Functional backend/frontend logic that responds to user interaction
>
> Feature flags, empty event handlers, or placeholder CSS classes do not constitute "implemented" status. A feature is complete only when an average user can successfully use it end-to-end.

**Application to Current Gaps:**
- Settings (lineHeight, fontSize, contentWidth) → Settings fields exist, TipTap ignores them
- Export UI → Backend commands exist, frontend modal missing

---

### Article 2: P0/P1 Feature Lifecycle Guarantee

**Intent:** Ensure P1 features are never marked complete until functional.

**Text:**
> P1 (high priority) features represent important user workflows that significantly impact usability. P1 features:
> 1. MUST be demonstrated working in each iteration demo if marked complete
> 2. MUST NOT be marked complete if only UI scaffolding exists without logic
> 3. MUST receive explicit deferral approval with documented justification before being moved to P2
> 4. MUST be prioritized in the next iteration if found non-functional during code review

**Application to Current Gaps:**
- File tree CRUD → Display-only tree needs full CRUD operations
- Drag-and-drop file open → Drop zone handlers completely missing

---

### Article 3: Settings-Implementation Parity

**Intent:** Prevent settings fields from existing without corresponding implementation.

**Text:**
> Any settings field added to the Settings struct/model MUST have corresponding implementation code that reads and applies that setting within the same iteration. Adding a setting field without implementation is a draft/incomplete change. Settings include but are not limited to: lineHeight, fontSize, contentWidth, focusMode, typewriterMode.

**Application to Current Gaps:**
- `lineHeight` setting exists in Settings model but no CSS applied to TipTap
- `fontSize` setting exists in Settings model but no CSS applied to TipTap
- `contentWidth` setting exists in Settings model but no CSS applied to TipTap

---

### Article 4: CSS-Logic Pairing

**Intent:** Prevent CSS classes from existing without corresponding JavaScript/Rust activation logic.

**Text:**
> Any CSS class added to implement a feature mode (e.g., `.focus-mode`, `.typewriter-mode`) MUST have corresponding activation/deactivation logic in the frontend JavaScript within the same iteration. CSS-only implementations are considered incomplete.

**Status:** This article was correctly applied in iteration-2 - Focus Mode and Typewriter Mode are now fully implemented.

---

### Article 5: Integration Before Merge

**Intent:** Ensure backend engines are connected to frontend before merge.

**Text:**
> Any backend engine or logic module MUST be integrated with its frontend interface before the PR containing it can be merged. Engine existence alone does not satisfy implementation requirements. This includes: undo/redo engines, export commands, search engines, file watchers.

**Application to Current Gaps:**
- Export commands exist in `src-tauri/src/commands/export.rs` but no frontend modal/trigger
- Undo/redo engine exists but Ctrl+Z/Y shortcuts not connected to TipTap
- File watcher integration incomplete

---

### Article 6 (NEW): Workspace File Operations

**Intent:** Ensure workspace management features are complete, not just display-only.

**Text:**
> The file tree component MUST support full CRUD operations (create, rename, delete) for both files and folders. A display-only file tree is considered incomplete. Drag-and-drop file opening MUST be implemented to provide native OS integration.

**Application to Current Gaps:**
- File tree shows files but create/rename/delete not functional
- Drag-and-drop event handlers not implemented

---

### Article 7 (NEW): Editor Settings Propagation

**Intent:** Ensure user preferences are actually applied to the editing experience.

**Text:**
> All settings that affect the editor appearance or behavior (lineHeight, fontSize, fontFamily, contentWidth, focusMode, typewriterMode) MUST be read from the Settings model and applied to the TipTap editor instance. Settings that are stored but not applied constitute incomplete implementation.

**Application to Current Gaps:**
- `lineHeight`, `fontSize`, `fontFamily`, `contentWidth` stored but not applied to TipTap

---

## Recommended Constitution Structure

```
# RustNote Constitution

## Preamble
RustNote is a Markdown editor prioritizing typora-like user experience. Development follows these principles...

## Article 1: Complete Feature Implementation Rule
## Article 2: P0/P1 Feature Lifecycle Guarantee
## Article 3: Settings-Implementation Parity
## Article 4: CSS-Logic Pairing
## Article 5: Integration Before Merge
## Article 6: Workspace File Operations (NEW)
## Article 7: Editor Settings Propagation (NEW)

## Amendment Process
How to update this Constitution...
```

---

## Implementation Recommendations

1. **Ratify the Constitution** - Create `/project-root/Constitution.md` with Articles 1-7
2. **Add to PR Checklist** - Constitution validation should block merge if violated
3. **Address P1 Issues First** - File tree CRUD and drag-and-drop are highest visibility
4. **Wire Settings to TipTap** - lineHeight, fontSize should apply immediately when changed

---

## Files Referenced

- Gap Analysis: `iterations/iteration-2/gap-analysis.md`
- Previous Suggestions: `iterations/iteration-1/constitution_updates.md`
- Settings Model: `src-tauri/src/model/settings.rs`
- TipTap Editor: `www/src/components/TipTapEditor.jsx`
- File Tree: `www/src/components/Sidebar.jsx`
- Export Commands: `src-tauri/src/commands/export.rs`

---

*Constitution update suggestions generated from iteration-2 gap analysis*
