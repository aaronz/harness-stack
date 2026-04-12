# Constitution Update Suggestions - Iteration 3

**Project:** RustNote
**Date:** 2026-04-12
**Status:** No Constitution exists. Iteration-1 and iteration-2 proposals were never ratified.
**Reference Gap Analysis:** iterations/iteration-3/gap-analysis.md

---

## Executive Summary

The Constitution articles proposed in iteration-1 (5 articles) and iteration-2 (7 articles) were **never ratified**. If they had been, most current P1 issues would have been prevented. This iteration's analysis confirms the articles are still relevant and adds NEW articles to cover gaps that weren't anticipated.

**Key Insight:** The anti-pattern persists - "backend exists, frontend missing". The same issue that plagued iteration-1 (Focus Mode CSS without JS logic) now plagues iteration-3 (Recovery snapshots without UI prompt).

---

## Iteration-2 Articles Status

The iteration-2/constitution_updates.md proposed 7 articles. **None were ratified.**

| Article | Would Have Prevented | Status |
|---------|---------------------|--------|
| Article 1: Complete Feature Implementation | Image paste only dialog | ❌ Not Ratified |
| Article 2: P0/P1 Feature Lifecycle Guarantee | N/A - no P0s now | ❌ Not Ratified |
| Article 3: Settings-Implementation Parity | ✅ RESOLVED in iteration-3 | ❌ Not Ratified |
| Article 4: CSS-Logic Pairing | ✅ RESOLVED in iteration-3 | ❌ Not Ratified |
| Article 5: Integration Before Merge | Undo/redo shortcuts, code highlighting | ❌ Not Ratified |
| Article 6: Workspace File Operations | ✅ RESOLVED in iteration-3 | ❌ Not Ratified |
| Article 7: Editor Settings Propagation | ✅ RESOLVED in iteration-3 | ❌ Not Ratified |

**Impact of Non-Ratification:**
- Article 1 would have flagged image paste as incomplete (only dialog insert, no paste handler)
- Article 5 would have flagged undo/redo shortcuts and code highlighting as incomplete (backend exists, frontend not wired)

---

## Recommended Constitution Articles (Ratification Package)

The following articles are **proven necessary** across 3 iterations. They should be ratified together.

### Article 1: Complete Feature Implementation Rule

**Intent:** Prevent "70% done" features where UI scaffolding exists without functional backend/frontend logic.

**Text:**
> Any feature marked as implemented in iteration status must have BOTH:
> 1. Visible UI element (component, CSS class, or user-triggered control), AND
> 2. Functional backend/frontend logic that responds to user interaction
>
> Feature flags, empty event handlers, or placeholder CSS classes do not constitute "implemented" status. A feature is complete only when an average user can successfully use it end-to-end.

**Application to Current Gaps:**
- Image paste: Dialog insert exists (insertImage function), paste handler missing
- Dirty state indicator: `isDirty` exists in state, no visual indicator in UI

---

### Article 2: P0/P1 Feature Lifecycle Guarantee

**Intent:** Ensure P1 features are never marked complete until functional.

**Text:**
> P1 (high priority) features represent important user workflows that significantly impact usability. P1 features:
> 1. MUST be demonstrated working in each iteration demo if marked complete
> 2. MUST NOT be marked complete if only UI scaffolding exists without logic
> 3. MUST receive explicit deferral approval with documented justification before being moved to P2
> 4. MUST be prioritized in the next iteration if found non-functional during code review

**Status:** Iteration-3 has NO P0 issues - this article has worked.

---

### Article 3: Settings-Implementation Parity

**Intent:** Prevent settings fields from existing without corresponding implementation.

**Text:**
> Any settings field added to the Settings struct/model MUST have corresponding implementation code that reads and applies that setting within the same iteration. Adding a setting field without implementation is a draft/incomplete change.

**Status:** ✅ Correctly implemented in iteration-3. fontSize, fontFamily, lineHeight, contentWidth now applied to TipTap.

---

### Article 4: CSS-Logic Pairing

**Intent:** Prevent CSS classes from existing without corresponding JavaScript/Rust activation logic.

**Text:**
> Any CSS class added to implement a feature mode (e.g., `.focus-mode`, `.typewriter-mode`) MUST have corresponding activation/deactivation logic in the frontend JavaScript within the same iteration. CSS-only implementations are considered incomplete.

**Status:** ✅ Correctly implemented in iteration-3. Focus Mode and Typewriter Mode fully working.

---

### Article 5: Integration Before Merge

**Intent:** Ensure backend engines are connected to frontend before merge.

**Text:**
> Any backend engine or logic module MUST be integrated with its frontend interface before the PR containing it can be merged. Engine existence alone does not satisfy implementation requirements. This includes: undo/redo engines, export commands, search engines, file watchers, syntax highlighters.

**Application to Current Gaps:**
- Undo/redo: Backend engine exists (`src-tauri/src/editor/`), Ctrl+Z/Y shortcuts not wired to TipTap
- Code highlighting: Backend `highlight_code_block` exists, TipTap doesn't use it
- Image paste: Backend `insert_image` command exists, frontend paste handler missing

---

### Article 6: Workspace File Operations

**Intent:** Ensure workspace management features are complete, not just display-only.

**Text:**
> The file tree component MUST support full CRUD operations (create, rename, delete) for both files and folders. A display-only file tree is considered incomplete. Drag-and-drop file opening MUST be implemented to provide native OS integration.

**Status:** ✅ Correctly implemented in iteration-3. Sidebar.jsx has full CRUD via context menu.

---

### Article 7: Editor Settings Propagation

**Intent:** Ensure user preferences are actually applied to the editing experience.

**Text:**
> All settings that affect the editor appearance or behavior (lineHeight, fontSize, fontFamily, contentWidth, focusMode, typewriterMode) MUST be read from the Settings model and applied to the TipTap editor instance. Settings that are stored but not applied constitute incomplete implementation.

**Status:** ✅ Correctly implemented in iteration-3. TipTapEditor applies all settings.

---

## NEW Articles for Iteration-3 Gaps

The following articles address NEW patterns not covered by previous iterations.

### Article 8: Workflow Completion (NEW)

**Intent:** Ensure backend workflow steps have corresponding frontend UX.

**Text:**
> When a feature involves a multi-step workflow (backend + frontend), BOTH sides must be implemented. Backend-only workflows (saving data, generating snapshots) MUST have corresponding UI prompts, indicators, or confirmations visible to the user. A workflow that "works in the backend" but has no user-facing indication is incomplete.

**Application to Current Gap:**
- Recovery snapshots: Backend saves recovery snapshots, but no startup prompt to user when snapshot exists
- This is the iteration-3's most critical gap - the exact same pattern as iteration-1's auto-save timer

**Example Violation:**
```
Backend: recovery.rs saves snapshots ✓
Frontend: No RecoveryModal or startup prompt ✗
→ VIOLATION: Article 8
```

---

### Article 9: Keyboard Shortcut Standardization (NEW)

**Intent:** Standard keyboard shortcuts must be wired, not just defined.

**Text:**
> Any keyboard shortcut defined in the application's shortcut reference (e.g., Ctrl+N for new, Ctrl+O for open) MUST have a corresponding event handler that executes the documented action. Shortcuts that are "documented but not wired" are considered incomplete implementation and must be fixed before MVP release.

**Application to Current Gaps:**
- Ctrl+N (new document) - not wired
- Ctrl+O (open document) - not wired
- Ctrl+Shift+O (open folder) - not wired
- Ctrl+Z (undo) - not wired to TipTap
- Ctrl+Y (redo) - not wired to TipTap

---

### Article 10: State Visibility (NEW)

**Intent:** Document state indicators must be visible to users.

**Text:**
> Any document state that affects user workflow (dirty/saved, recovery available, external changes detected) MUST have a visible indicator in the UI. State that exists only in the data model but is not reflected in the UI is considered incomplete. Examples:
> - `isDirty: true` MUST show an asterisk, dot, or "(unsaved)" in the title/toolbar
> - Recovery snapshot available MUST show a prompt or indicator on startup
> - External file change detected MUST show a modal or banner

**Application to Current Gap:**
- `isDirty` exists in DocumentResult but no visual indicator in toolbar/title

---

## Gap-to-Article Mapping

| Iteration-3 Gap | Root Cause | Article That Would Prevent |
|-----------------|------------|---------------------------|
| Image paste not implemented | Frontend handler missing | Article 1 (Complete Implementation) |
| Undo/redo shortcuts not wired | Integration incomplete | Article 5 (Integration Before Merge) |
| Recovery snapshots no UI prompt | Workflow incomplete | Article 8 (Workflow Completion) - NEW |
| Dirty state not visible | State not reflected in UI | Article 10 (State Visibility) - NEW |
| Ctrl+N/O/Shift+O not wired | Shortcuts not standardized | Article 9 (Keyboard Shortcut Standardization) - NEW |
| Code highlighting not visible | Integration incomplete | Article 5 (Integration Before Merge) |

---

## Recommended Constitution Structure

```
# RustNote Constitution

## Preamble
RustNote is a Markdown editor prioritizing typora-like user experience. Development follows these principles to ensure features are complete, not just scaffolded.

## Article 1: Complete Feature Implementation Rule
## Article 2: P0/P1 Feature Lifecycle Guarantee
## Article 3: Settings-Implementation Parity
## Article 4: CSS-Logic Pairing
## Article 5: Integration Before Merge
## Article 6: Workspace File Operations
## Article 7: Editor Settings Propagation
## Article 8: Workflow Completion (NEW)
## Article 9: Keyboard Shortcut Standardization (NEW)
## Article 10: State Visibility (NEW)

## Amendment Process
Constitution amendments require:
1. Proposal in gap-analysis.md with justification
2. Review in iteration planning
3. Explicit ratification vote
4. Update to Constitution.md
```

---

## Action Items

### Immediate (This Session)

1. **Ratify Articles 1-7** - These are proven necessary across 3 iterations
2. **Ratify Articles 8-10** - These address new patterns identified in iteration-3
3. **Create `/project-root/Constitution.md`** - With all 10 articles
4. **Add Constitution validation to PR checklist** - Articles 1, 5, 8, 9, 10 should block merge if violated

### Address Current P1 Issues

Based on the articles above, the following fixes are **required**:

| P1 Issue | Required Fix | Article Invoked |
|----------|--------------|-----------------|
| Image paste | Add paste event handler in TipTapEditor | Article 1 |
| Undo/redo shortcuts | Wire Ctrl+Z/Y to TipTap's undo manager | Article 5 |
| Recovery prompt | Add RecoveryModal that checks snapshots on startup | Article 8 |
| Dirty indicator | Add visual indicator in Toolbar based on isDirty | Article 10 |
| Keyboard shortcuts | Wire Ctrl+N, Ctrl+O, Ctrl+Shift+O handlers | Article 9 |

---

## Files Referenced

- Gap Analysis: `iterations/iteration-3/gap-analysis.md`
- Previous Proposals: `iterations/iteration-2/constitution_updates.md`, `iterations/iteration-1/constitution_updates.md`
- TipTap Editor: `www/src/components/TipTapEditor.jsx`
- Toolbar: `www/src/components/Toolbar.jsx`
- Recovery Backend: `src-tauri/src/commands/recovery.rs`
- Keyboard Handling: Not centralized - needs investigation

---

## Conclusion

The iteration-3 analysis confirms that the Constitution articles proposed in iteration-1 and iteration-2 are **necessary but never sufficient alone** - they must be **ratified and enforced**. The new articles (8-10) address patterns that emerged only after implementing the core features.

**Core Principle:** A feature is not complete when its backend exists. A feature is complete when the user can successfully use it end-to-end.

---

*Constitution update suggestions generated from iteration-3 gap analysis*
