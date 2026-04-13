# Constitution Update Suggestions - Iteration 4

**Project:** RustNote
**Date:** 2026-04-13
**Status:** No Constitution exists. Articles 1-10 proposed in iterations 1-3 were never ratified.
**Reference Gap Analysis:** iterations/iteration-4/gap-analysis.md

---

## Executive Summary

Iteration-4 demonstrates significant improvement: **ALL previous P0/P1 issues have been resolved**. The MVP is approximately 90-95% complete with only 2 remaining P1 issues.

**However**, these P1 issues reveal a critical flaw: Articles 1-10 were **proposed but never ratified** in previous iterations. Had they been ratified, current issues would have been prevented:

| Issue | Would Have Been Prevented By | Status |
|-------|------------------------------|--------|
| Code highlighting not wired | Article 5 (Integration Before Merge) | ❌ Not Ratified |
| Link editing undefined | Article ?? (UI Behavior Documentation) | ❌ Does Not Exist |

**This iteration proposes:**
1. Ratification of Articles 1-10 from iteration-3 (proven necessary over 3 iterations)
2. NEW Article 11: Interactive Element Behavior Documentation
3. Ratification is **critical** - MVP is at risk if these patterns continue

---

## Part I: Previous Articles Status

### Iteration-3 Articles (1-10) - Proposed but NOT Ratified

| Article | Topic | Would Have Prevented | Status |
|---------|-------|---------------------|--------|
| Article 1 | Complete Feature Implementation | Image paste only dialog | ❌ Not Ratified |
| Article 2 | P0/P1 Feature Lifecycle Guarantee | P1 marking abuse | ❌ Not Ratified |
| Article 3 | Settings-Implementation Parity | ✅ RESOLVED iteration-3 | ❌ Not Ratified |
| Article 4 | CSS-Logic Pairing | ✅ RESOLVED iteration-3 | ❌ Not Ratified |
| Article 5 | Integration Before Merge | Code highlighting, undo/redo | ❌ Not Ratified |
| Article 6 | Workspace File Operations | ✅ RESOLVED iteration-3 | ❌ Not Ratified |
| Article 7 | Editor Settings Propagation | ✅ RESOLVED iteration-3 | ❌ Not Ratified |
| Article 8 | Workflow Completion | Recovery UI | ✅ RESOLVED iteration-4 |
| Article 9 | Keyboard Shortcut Standardization | Ctrl+N/O shortcuts | ✅ RESOLVED iteration-4 |
| Article 10 | State Visibility | Dirty indicator | ✅ RESOLVED iteration-4 |

**Note:** Articles 8, 9, 10 were implemented in iteration-4 WITHOUT ratification - the issues were obvious enough to fix directly. Articles 3, 4, 6, 7 were also resolved. But Articles 1, 2, 5 remain unratified and their patterns still cause issues.

---

## Part II: Iteration-4 Gap Analysis

### Resolved Issues (Iteration-4 Wins)

The following issues from iteration-3 were successfully fixed:

| Issue | Resolution | Article That Should Have Prevented |
|-------|------------|----------------------------------|
| Image paste not implemented | TipTapEditor.jsx handlePaste | Article 1 |
| Undo/redo Ctrl+Z/Y not wired | handleKeyDown now handles Ctrl+Z/Y | Article 5 |
| Recovery snapshots no UI | RecoveryModal.jsx fully implemented | Article 8 |
| Dirty state not visible | Toolbar shows asterisk | Article 10 |
| Ctrl+N shortcut not wired | App.jsx handles Ctrl+N | Article 9 |
| Ctrl+O shortcut not wired | App.jsx handles Ctrl+O | Article 9 |
| Ctrl+Shift+O not wired | App.jsx handles Ctrl+Shift+O | Article 9 |

### Remaining P1 Issues (Iteration-4 Losses)

**1. Code syntax highlighting not integrated (FR-015)**
- Backend `highlight_code_block` command exists with syntect
- Frontend CodeBlockHighlight component exists
- **Not wired together** - code blocks render without highlighting
- This is EXACTLY the pattern Article 5 was meant to prevent
- **Article 5 was not ratified, so it could not be enforced**

**2. Link editing behavior undefined (FR-011)**
- FR-011 requires "link editing without raw-syntax confusion"
- Current: Links render as clickable links but click behavior is browser default
- No documentation of: click-to-edit vs Ctrl+click-to-follow
- No LinkPopover or edit behavior implemented
- **No existing article covers UI behavior documentation for interactive elements**

---

## Part III: Proposed Constitution Articles

### For Ratification: Articles 1-10 (from iteration-3)

These articles are **proven necessary** across 3+ iterations. They address patterns that repeatedly cause issues:

#### Article 1: Complete Feature Implementation Rule

**Intent:** Prevent "70% done" features where UI scaffolding exists without functional backend/frontend logic.

**Text:**
> Any feature marked as implemented in iteration status must have BOTH:
> 1. Visible UI element (component, CSS class, or user-triggered control), AND
> 2. Functional backend/frontend logic that responds to user interaction
>
> Feature flags, empty event handlers, or placeholder CSS classes do not constitute "implemented" status. A feature is complete only when an average user can successfully use it end-to-end.

**Application to Current Gap:**
- CodeBlockHighlight component exists, not wired to TipTap rendering

---

#### Article 2: P0/P1 Feature Lifecycle Guarantee

**Intent:** Ensure P1 features are never marked complete until functional.

**Text:**
> P1 (high priority) features represent important user workflows that significantly impact usability. P1 features:
> 1. MUST be demonstrated working in each iteration demo if marked complete
> 2. MUST NOT be marked complete if only UI scaffolding exists without logic
> 3. MUST receive explicit deferral approval with documented justification before being moved to P2
> 4. MUST be prioritized in the next iteration if found non-functional during code review

---

#### Article 3: Settings-Implementation Parity

**Intent:** Prevent settings fields from existing without corresponding implementation.

**Text:**
> Any settings field added to the Settings struct/model MUST have corresponding implementation code that reads and applies that setting within the same iteration. Adding a setting field without implementation is a draft/incomplete change.

**Status:** ✅ Correctly implemented in iteration-3/4.

---

#### Article 4: CSS-Logic Pairing

**Intent:** Prevent CSS classes from existing without corresponding JavaScript/Rust activation logic.

**Text:**
> Any CSS class added to implement a feature mode (e.g., `.focus-mode`, `.typewriter-mode`) MUST have corresponding activation/deactivation logic in the frontend JavaScript within the same iteration. CSS-only implementations are considered incomplete.

**Status:** ✅ Correctly implemented in iteration-3/4.

---

#### Article 5: Integration Before Merge

**Intent:** Ensure backend engines are connected to frontend before merge.

**Text:**
> Any backend engine or logic module MUST be integrated with its frontend interface before the PR containing it can be merged. Engine existence alone does not satisfy implementation requirements. This includes: undo/redo engines, export commands, search engines, file watchers, syntax highlighters, and any command exposed via Tauri IPC.

**Application to Current Gap:**
- Code highlighting: Backend `highlight_code_block` exists, TipTap doesn't call it

**Enforcement:** This article should block PR merge via checklist.

---

#### Article 6: Workspace File Operations

**Intent:** Ensure workspace management features are complete, not just display-only.

**Text:**
> The file tree component MUST support full CRUD operations (create, rename, delete) for both files and folders. A display-only file tree is considered incomplete. Drag-and-drop file opening MUST be implemented to provide native OS integration.

**Status:** ✅ Correctly implemented in iteration-3/4.

---

#### Article 7: Editor Settings Propagation

**Intent:** Ensure user preferences are actually applied to the editing experience.

**Text:**
> All settings that affect the editor appearance or behavior (lineHeight, fontSize, fontFamily, contentWidth, focusMode, typewriterMode) MUST be read from the Settings model and applied to the TipTap editor instance. Settings that are stored but not applied constitute incomplete implementation.

**Status:** ✅ Correctly implemented in iteration-3/4.

---

#### Article 8: Workflow Completion

**Intent:** Ensure backend workflow steps have corresponding frontend UX.

**Text:**
> When a feature involves a multi-step workflow (backend + frontend), BOTH sides must be implemented. Backend-only workflows (saving data, generating snapshots) MUST have corresponding UI prompts, indicators, or confirmations visible to the user. A workflow that "works in the backend" but has no user-facing indication is incomplete.

**Status:** ✅ Correctly implemented in iteration-4.

---

#### Article 9: Keyboard Shortcut Standardization

**Intent:** Standard keyboard shortcuts must be wired, not just defined.

**Text:**
> Any keyboard shortcut defined in the application's shortcut reference (e.g., Ctrl+N for new, Ctrl+O for open) MUST have a corresponding event handler that executes the documented action. Shortcuts that are "documented but not wired" are considered incomplete implementation and must be fixed before MVP release.

**Status:** ✅ Correctly implemented in iteration-4.

---

#### Article 10: State Visibility

**Intent:** Document state indicators must be visible to users.

**Text:**
> Any document state that affects user workflow (dirty/saved, recovery available, external changes detected) MUST have a visible indicator in the UI. State that exists only in the data model but is not reflected in the UI is considered incomplete.

**Status:** ✅ Correctly implemented in iteration-4.

---

### NEW Article 11: Interactive Element Behavior Documentation

**Intent:** Prevent ambiguous click behavior for interactive elements (links, buttons, etc.)

**Text:**
> All interactive elements that respond to click or modifier-key interactions MUST have explicitly documented behavior. This includes:
> 1. Link elements: MUST document whether click opens editor, follows URL, or triggers popover
> 2. If modifier keys affect behavior (e.g., Ctrl+click follows link, click edits), this MUST be documented
> 3. Default browser behaviors on interactive elements MUST be explicitly overridden or confirmed as intentional
>
> "Works like browser default" is not acceptable documentation - the developer must explicitly choose and document each behavior.

**Application to Current Gap:**
- FR-011 (Link behavior): Currently undefined - click opens link in browser vs opens editor
- Need LinkPopover component with documented behavior

**ExampleViolation:**
```
Link renders as <a href="..."> but click behavior is browser default
→ User clicks expecting edit, browser opens URL
→ VIOLATION: Article 11
```

---

## Part IV: Gap-to-Article Mapping

| Iteration-4 Gap | Root Cause | Article That Would Prevent |
|-----------------|------------|---------------------------|
| Code highlighting not wired | Backend/frontend integration | Article 5 (Integration Before Merge) |
| Link editing undefined | No behavior documentation | Article 11 (NEW - Interactive Element Behavior) |
| Frontmatter as plain text | Content rendering gap | Article 1 (partial) |
| XSS in link URLs | Security not explicit | Article 11 (behavior override prevents default) |

---

## Part V: Recommended Constitution Structure

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
## Article 8: Workflow Completion
## Article 9: Keyboard Shortcut Standardization
## Article 10: State Visibility
## Article 11: Interactive Element Behavior Documentation (NEW)

## Amendment Process
Constitution amendments require:
1. Proposal in gap-analysis.md with justification
2. Review in iteration planning
3. Explicit ratification vote
4. Update to Constitution.md
```

---

## Part VI: Action Items

### Critical: Ratify Articles 1-10

Articles 1-10 have been proposed across 3 iterations. They address patterns that **repeatedly** cause issues:
- Article 1: Image paste, CodeBlockHighlight scaffolding
- Article 5: Code highlighting, undo/redo shortcuts
- Articles 8-10: Recovery UI, keyboard shortcuts, dirty indicator (already fixed but Article proves value)

### Immediate: Address Current P1 Issues

| P1 Issue | Required Fix | Article Invoked |
|----------|--------------|-----------------|
| Code highlighting | Wire CodeBlockHighlight to TipTap code block rendering | Article 5 |
| Link editing | Add LinkPopover with documented click/Ctrl+click behavior | Article 11 |

### Security: Address XSS in Link URLs

The gap analysis mentions XSS in link URLs as a P1 security issue. This relates to Article 11 - when we document and override link behavior, we can also sanitize URLs.

---

## Part VII: Files Referenced

- Gap Analysis: `iterations/iteration-4/gap-analysis.md`
- Previous Proposals: `iterations/iteration-3/constitution_updates.md`
- TipTap Editor: `www/src/components/TipTapEditor.jsx`
- CodeBlockHighlight: `www/src/components/CodeBlockHighlight.jsx`
- Toolbar: `www/src/components/Toolbar.jsx`
- Keyboard Handling: `www/src/App.jsx`

---

## Conclusion

Iteration-4 proves that **when obvious issues are addressed without constitutional guidance, they get fixed**. However, subtle issues like code highlighting integration and link behavior documentation persist because:

1. Articles 1-10 were proposed but never ratified - no enforcement mechanism
2. No article covers **documenting interactive element behavior**

**Recommendation:** Ratify Articles 1-10 (proven over 3 iterations) AND add Article 11 to cover the interactive element behavior gap.

**Core Principle:** A feature is not complete when its backend exists. A feature is complete when the user can successfully use it end-to-end. And an interactive element is not safe when its behavior is undefined.

---

*Constitution update suggestions generated from iteration-4 gap analysis*
