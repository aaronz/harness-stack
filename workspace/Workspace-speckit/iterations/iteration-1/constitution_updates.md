# Constitution Update Suggestions

**Project:** RustNote  
**Date:** 2026-04-11  
**Status:** No existing Constitution - suggestions for new Constitution creation

---

## Executive Summary

No Constitution file exists in this project. The gap analysis identifies 4 P0 issues that a Constitution should prevent from being deferred or de-scoped in future iterations. These suggestions propose Constitution articles that would ensure P0 features are never left non-functional when CSS/implementation scaffolding exists.

---

## Gap Analysis: P0 Issues Summary

| P0 Issue | Current State | Problem |
|----------|--------------|---------|
| Find/Replace UI | Not implemented | No search panel or Ctrl+F/Ctrl+H shortcuts |
| Focus Mode | CSS exists, logic missing | `.focus-mode` class present but no paragraph de-emphasis |
| Typewriter Mode | CSS exists, logic missing | `.typewriter-mode` class present but no cursor centering |
| Auto-save Timer | Settings exist, timer missing | Debounced auto-save never triggers |

**Critical Pattern:** All four P0 issues share a common anti-pattern — **scaffolding without implementation**. CSS classes or settings fields exist, but the corresponding logic was never completed.

---

## Proposed Constitution Articles

### Article 1: Complete Feature Implementation Rule

**Intent:** Prevent "70% done" features where UI scaffolding exists without functional backend/frontend logic.

**Text Proposal:**
> Any feature marked as implemented in iteration status must have BOTH:
> 1. Visible UI element (component, CSS class, or user-triggered control), AND
> 2. Functional backend/frontend logic that responds to user interaction
>
> Feature flags, empty event handlers, or placeholder CSS classes do not constitute "implemented" status. A feature is complete only when an average user can successfully use it end-to-end.

**Application to P0 Issues:**
- Find/Replace: Needs UI panel + keyboard shortcuts + search logic (all missing)
- Focus/Typewriter modes: Need CSS (exists) + JavaScript activation logic (missing)
- Auto-save: Needs settings field (exists) + timer trigger (missing)

---

### Article 2: P0 Feature Lifecycle Guarantee

**Intent:** Ensure P0 features are never marked complete until functional, and never deferred without explicit stakeholder approval.

**Text Proposal:**
> P0 (blocking) features represent core user workflows without which the application cannot be considered functional. P0 features:
> 1. MUST be demonstrated working in each iteration demo
> 2. MUST NOT be merged to main branch if non-functional
> 3. MUST receive explicit deferral approval with documented justification before being moved to P1/P2
> 4. MUST be prioritized in the next iteration if found non-functional during code review

**Application to RustNote P0 Issues:**
These 4 issues should have blocked the iteration 6 release:
- Find/Replace UI
- Focus mode logic
- Typewriter mode logic
- Auto-save timer

---

### Article 3: Settings-Implementation Parity

**Intent:** Prevent settings fields from existing without corresponding implementation.

**Text Proposal:**
> Any settings field added to the Settings struct/model MUST have corresponding implementation code that reads and applies that setting within the same iteration. Adding a setting field without implementation is a draft/incomplete change.

**Application to Auto-save Issue:**
- `auto_save: bool` exists in Settings
- `auto_save_interval: u64` exists in Settings
- Timer logic that respects these settings does NOT exist

---

### Article 4: CSS-Logic Pairing

**Intent:** Prevent CSS classes from existing without corresponding JavaScript/Rust activation logic.

**Text Proposal:**
> Any CSS class added to implement a feature mode (e.g., `.focus-mode`, `.typewriter-mode`) MUST have corresponding activation/deactivation logic in the frontend JavaScript within the same iteration. CSS-only implementations are considered incomplete.

**Application to Focus/Typewriter Issues:**
- `.focus-mode` CSS exists in editor styles
- `.typewriter-mode` CSS exists in editor styles
- No JavaScript adds these classes based on user toggle

---

### Article 5: Integration Before Merge

**Intent:** Ensure backend engines are connected to frontend before merge.

**Text Proposal:**
> Any backend engine or logic module (e.g., undo/redo engine, find/replace engine) MUST be integrated with its frontend interface before the PR containing it can be merged. Engine existence alone does not satisfy implementation requirements.

**Application to Undo/Redo Issue:**
- Undo engine exists in `src-tauri/src/editor/undo.rs`
- No frontend integration (Ctrl+Z/Ctrl+Y handlers not connected)

---

## Recommendations

### Immediate Constitution Creation

1. Create `/project-root/Constitution.md` with the 5 articles above
2. Add Constitution validation to the PR review checklist
3. Flag iteration-6 completion as conditional on resolving these 4 P0 issues

### Suggested Constitution Structure

```
# RustNote Constitution

## Preamble
Core principles guiding RustNote development...

## Article 1: Complete Feature Implementation
## Article 2: P0 Feature Lifecycle Guarantee  
## Article 3: Settings-Implementation Parity
## Article 4: CSS-Logic Pairing
## Article 5: Integration Before Merge

## Amendment Process
How to update this Constitution...
```

---

## Files Referenced

- Gap Analysis: `iterations/iteration-1/gap-analysis.md`
- Settings Model: `src-tauri/src/model/settings.rs`
- Editor CSS: `www/src/styles/editor.css`
- Undo Engine: `src-tauri/src/editor/undo.rs`

---

*Constitution update suggestions generated from iteration-1 gap analysis*
