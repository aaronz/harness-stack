# Constitution Update Suggestions - Iteration 5

**Project:** RustNote
**Date:** 2026-04-13
**Status:** No Constitution exists. Articles 1-11 proposed across iterations 1-4 were never ratified.
**Reference Gap Analysis:** iterations/iteration-5/gap-analysis.md

---

## Executive Summary

Iteration-5 gap analysis identifies **3 new P0 issues** that reveal constitutional gaps NOT covered by existing proposed Articles 1-11:

| P0 Issue | Root Cause | Constitutional Gap |
|----------|------------|-------------------|
| G-001: TransformEngine incomplete | Not all TransformType variants implemented | Engine Completeness not covered |
| G-002: Undo/redo fidelity issues | Structural edits not handled correctly | Transactional Integrity not covered |
| G-003: Settings uses JSON not rusqlite | PRD-10 technology spec not followed | Architecture Conformance not covered |

**Critical Finding:** Articles 1-11 have been proposed for 4 iterations but never ratified. This iteration adds 3 MORE articles addressing patterns that the existing articles don't cover:

1. **Engine Operation Completeness** (Article 12) - ALL variants in an engine must be implemented
2. **Transactional Integrity Guarantee** (Article 13) - undo/redo must maintain structural fidelity
3. **Architecture Technology Conformance** (Article 14) - PRD technology specifications must be followed

---

## Part I: Previous Articles Status

### Articles 1-11: Proposed but NOT Ratified (iterations 1-4)

| Article | Topic | Would Have Prevented | Status |
|---------|-------|---------------------|--------|
| Article 1 | Complete Feature Implementation | General scaffolding issues | ❌ Not Ratified |
| Article 2 | P0/P1 Feature Lifecycle Guarantee | Lifecycle issues | ❌ Not Ratified |
| Article 3 | Settings-Implementation Parity | Settings field/implementation gaps | ❌ Not Ratified |
| Article 4 | CSS-Logic Pairing | CSS without JS logic | ❌ Not Ratified |
| Article 5 | Integration Before Merge | Backend/frontend wiring | ❌ Not Ratified |
| Article 6 | Workspace File Operations | File tree CRUD | ❌ Not Ratified |
| Article 7 | Editor Settings Propagation | Settings not applied | ❌ Not Ratified |
| Article 8 | Workflow Completion | Backend-only workflows | ❌ Not Ratified |
| Article 9 | Keyboard Shortcut Standardization | Shortcuts not wired | ❌ Not Ratified |
| Article 10 | State Visibility | State indicators missing | ❌ Not Ratified |
| Article 11 | Interactive Element Behavior Documentation | Undefined click behaviors | ❌ Not Ratified |

**Total Articles Proposed:** 11 (across 4 iterations)
**Total Articles Ratified:** 0
**P0 issues in iteration-5:** 3 NEW issues that existing articles don't cover

---

## Part II: P0 Issue Analysis

### G-001: TransformEngine Incomplete (P0 - Blocking)

**Issue:** `TransformType::EnterInListItem`, `EnterInBlockQuote`, `EnterInHeading` not implemented in Rust `TransformEngine`. Frontend has switch cases but Rust backend returns error/unimplemented.

**Gap Analysis Finding:**
- Frontend `useTransforms.ts` defines all transform types including `EnterInListItem`, `EnterInBlockQuote`, `EnterInHeading`
- Backend `transforms.rs` has `TransformEngine::apply()` method but returns error for unimplemented variants
- User experience: Pressing Enter in a list item does NOT create a new list item - the transform silently fails

**Why Existing Articles Don't Cover This:**

| Article | Why Insufficient |
|---------|------------------|
| Article 1 (Complete Feature Implementation) | Covers "70% done" features but doesn't explicitly require ALL variants of an enum/operation set to be implemented |
| Article 5 (Integration Before Merge) | Covers integration but TRANSFORM ENGINE IS integrated - it just doesn't implement all operations |

**Root Cause:** No constitutional requirement that "when a module exposes N operations, ALL N operations must be implemented."

---

### G-002: Undo/Redo Fidelity Issues (P0 - Blocking)

**Issue:** Undo/redo implementation exists but has fidelity issues with structural edits. No regression tests for round-trip undo.

**Gap Analysis Finding:**
- Frontend TipTap has undo/redo via `editor.commands.unsetHistory()` and `editor.commands commands.chain().run()`
- Backend may have separate transaction log
- **Problem:** When undoing structural edits (adding/removing list items, changing heading levels), the result may not match the original state

**Why Existing Articles Don't Cover This:**

| Article | Why Insufficient |
|---------|------------------|
| Article 5 (Integration Before Merge) | Mentions "undo/redo engines" must be integrated but doesn't specify INTEGRITY requirements |
| Article 8 (Workflow Completion) | Covers workflow completion not structural fidelity |

**Root Cause:** No constitutional requirement for "transactional integrity" - undo/redo must faithfully reverse operations preserving document structure.

---

### G-003: Settings Uses JSON Not rusqlite (P0 - Blocking)

**Issue:** Settings persistence uses file-based JSON (`settings.json`) instead of rusqlite as specified in PRD-10.

**Gap Analysis Finding:**
- Current: `src-tauri/src/model/settings.rs` reads/writes `settings.json` file
- Required by PRD-10: rusqlite-based persistence
- **Problem:** JSON file is prone to corruption on crash; no atomic writes, no transactions

**Why Existing Articles Don't Cover This:**

| Article | Why Insufficient |
|---------|------------------|
| Article 3 (Settings-Implementation Parity) | Requires settings fields to have implementation but doesn't specify WHICH technology |
| Article 7 (Editor Settings Propagation) | Requires settings to be applied but doesn't specify persistence technology |

**Root Cause:** No constitutional requirement that "technology specifications in PRD must be followed."

---

## Part III: Proposed New Articles

### Article 12: Engine Operation Completeness (NEW)

**Intent:** Prevent "partially implemented engines" where some operations are functional while others return errors or no-op.

**Text:**
> When a module or engine exposes a set of operations (via enum variants, trait methods, command definitions, or API endpoints), ALL specified operations MUST be implemented with functional behavior. Partial implementation where some operations return errors, unimplemented errors, or no-op while others work correctly constitutes incomplete implementation.
>
> Specifically:
> 1. Enum variants used in switch/case statements MUST have implementations for ALL variants
> 2. Trait methods MUST have implementations for ALL declared methods
> 3. Command handlers MUST handle ALL documented command variants
> 4. "Unimplemented" or "TODO" in production code for exposed operations is a constitutional violation

**Application to Current Gap:**
- `TransformType` enum has variants `EnterInListItem`, `EnterInBlockQuote`, `EnterInHeading` that return errors
- These MUST be implemented before iteration-5 can be considered complete

**Violation Example:**
```
TransformEngine::apply(TransformType::EnterInListItem) → Err("Unimplemented")
→ VIOLATION: Article 12
```

---

### Article 13: Transactional Integrity Guarantee (NEW)

**Intent:** Ensure undo/redo operations maintain perfect structural fidelity - undoing an edit must produce byte-for-byte identical document state.

**Text:**
> Any editor that supports undo/redo MUST maintain transactional integrity:
> 1. Undo operations MUST produce document state that is byte-for-byte identical to the state BEFORE the operation being undone
> 2. Round-trip test (do operation → undo → do again → undo) MUST produce identical state to original
> 3. Structural edits (list manipulation, heading level changes, block quote conversion) are NOT exempt from fidelity requirements
> 4. Undo/redo that "mostly works" but fails on structural edits is considered BROKEN

**Application to Current Gap:**
- G-002: Undo/redo fidelity issues with structural edits must be fixed
- Round-trip tests MUST be added before declaring undo/redo functional

**Violation Example:**
```
Document: "# Heading"
User changes to "## Heading"
Undo
Result: "# Heading" but cursor position differs
→ VIOLATION: Article 13 (fidelity not preserved)
```

---

### Article 14: Architecture Technology Conformance (NEW)

**Intent:** Ensure implementations follow technology specifications defined in PRD/architecture documents.

**Text:**
> When the PRD or architecture document specifies a particular technology, library, or approach (e.g., "use rusqlite for settings persistence", "use ropey for buffer"), that specification is binding. Substituting equivalent functionality via different technology requires explicit PRD amendment, not silent substitution.
>
> Specifically:
> 1. PRD technology specifications MUST be followed unless explicitly amended
> 2. Substituting JSON for rusqlite, or manual memory management for ropey, constitutes architecture drift
> 3. Technology drift MUST be flagged in gap analysis and rectified or formally amended
> 4. "Works well enough" is not justification for architecture drift

**Application to Current Gap:**
- G-003: PRD-10 specifies rusqlite for settings, implementation uses JSON file
- This MUST be rectified to use rusqlite as specified, OR PRD-10 must be formally amended

**Violation Example:**
```
PRD-10: "Settings stored in rusqlite database"
Implementation: "settings.json file"
→ VIOLATION: Article 14 (architecture drift)
```

---

## Part IV: Ratification Package

### For Ratification: Articles 1-11 (from iterations 1-4)

These 11 articles have been proposed across 4 iterations. They address proven patterns that cause repeated issues:

| Article | Proven Necessary | Issue Prevented |
|---------|-----------------|----------------|
| Article 1 | 4 iterations | General scaffolding without logic |
| Article 2 | 4 iterations | P0/P1 lifecycle abuse |
| Article 3 | 3 iterations | Settings fields without implementation |
| Article 4 | 3 iterations | CSS classes without JS logic |
| Article 5 | 4 iterations | Backend not integrated |
| Article 6 | 2 iterations | File tree display-only |
| Article 7 | 2 iterations | Settings not applied |
| Article 8 | 2 iterations | Backend-only workflows |
| Article 9 | 2 iterations | Shortcuts not wired |
| Article 10 | 2 iterations | State indicators missing |
| Article 11 | 1 iteration | Interactive element behavior undefined |

### For Ratification: Articles 12-14 (from iteration-5)

These 3 new articles address P0 issues not covered by existing articles:

| Article | Addresses | Issue Prevented |
|---------|-----------|----------------|
| Article 12 | Engine Operation Completeness | G-001: TransformEngine incomplete |
| Article 13 | Transactional Integrity Guarantee | G-002: Undo/redo fidelity |
| Article 14 | Architecture Technology Conformance | G-003: JSON instead of rusqlite |

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
## Article 11: Interactive Element Behavior Documentation
## Article 12: Engine Operation Completeness (NEW)
## Article 13: Transactional Integrity Guarantee (NEW)
## Article 14: Architecture Technology Conformance (NEW)

## Amendment Process
Constitution amendments require:
1. Proposal in gap-analysis.md with justification
2. Review in iteration planning
3. Explicit ratification vote
4. Update to Constitution.md
```

---

## Part VI: Gap-to-Article Mapping

### Iteration-5 P0 Issues

| Gap | Article That Covers | Article That Would Have Prevented |
|-----|---------------------|----------------------------------|
| G-001: TransformEngine incomplete | None existing | Article 12 (NEW): Engine Operation Completeness |
| G-002: Undo/redo fidelity | Article 5 (partial) | Article 13 (NEW): Transactional Integrity Guarantee |
| G-003: JSON instead of rusqlite | Article 3 (partial) | Article 14 (NEW): Architecture Technology Conformance |

### All Known Gaps Covered

| Gap ID | Iteration Introduced | Gap | Covering Article |
|--------|--------------------|-----|-----------------|
| G-001 | 5 | TransformEngine incomplete | Article 12 (NEW) |
| G-002 | 5 | Undo/redo fidelity | Article 13 (NEW) |
| G-003 | 5 | Settings JSON vs rusqlite | Article 14 (NEW) |
| G-004 | 5 | Cursor mapping incomplete | Article 5 (Integration) - partial |
| G-005 | 5 | PDF export quality | Article 5 (Integration) - partial |
| G-006 | 5 | Parser tree-sitter | Article 14 (Architecture) - direct |
| G-007 | 5 | Buffer ropey | Article 14 (Architecture) - direct |
| G-008 | 5 | Wrap transform | Article 12 (Engine Completeness) |
| G-009 | 5 | Image relative paths | Article 1 (Complete Implementation) |

---

## Part VII: Action Items

### Critical: Ratify Complete Constitution

**Articles to Ratify:** 1-14 (all articles proposed across all iterations)

| Priority | Articles | Proven Necessary |
|----------|----------|-----------------|
| Must Ratify | 1-11 | Proven over 4 iterations |
| Must Ratify | 12-14 | Proven necessary by iteration-5 P0 issues |

### Immediate: Address Iteration-5 P0 Issues

| P0 Issue | Required Fix | Article Invoked |
|----------|-------------|-----------------|
| G-001: TransformEngine | Implement EnterInListItem, EnterInBlockQuote, EnterInHeading | Article 12 |
| G-002: Undo/redo fidelity | Add round-trip tests, fix structural edit handling | Article 13 |
| G-003: JSON settings | Replace with rusqlite per PRD-10 | Article 14 |

### Short-term: Address Remaining P1 Issues

| P1 Issue | Required Fix | Article Invoked |
|----------|-------------|-----------------|
| G-004: Cursor mapping | Implement bidirectional cursor mapping | Article 5 |
| G-006: Parser tree-sitter | Add tree-sitter wrapper per PRD-10 | Article 14 |
| G-007: Buffer ropey | Implement ropey per PRD-10 | Article 14 |
| G-008: Wrap transform | Add Wrap transform | Article 12 |

---

## Part VIII: Constitutional Principles

Based on 5 iterations of gap analysis, the following principles have proven necessary:

### Principle 1: Completion Means Complete
A feature is not complete when its backend exists or its UI renders. A feature is complete when a user can successfully use it end-to-end without error.

### Principle 2: All Operations Are Required
When a module exposes N operations, implementing N-1 is not 99% complete - it is broken. Users encounter the missing operation and experience failure.

### Principle 3: Fidelity Is Non-Negotiable
Undo/redo that "mostly works" is not acceptable. Structural edits must be as reversible as text edits. Data integrity is not optional.

### Principle 4: Architecture Is Binding
PRD specifications exist to ensure coherent architecture. Substituting technology without formal amendment is architecture drift that accumulates technical debt.

### Principle 5: Ratification Prevents Accumulation
Articles proposed but not ratified accumulate. Each iteration adds more proposed articles without enforcement. After 5 iterations, 14 articles are pending ratification. This pattern itself is a constitutional failure.

---

## Part IX: Files Referenced

- Gap Analysis: `iterations/iteration-5/gap-analysis.md`
- Previous Proposals: `iterations/iteration-1/through/iteration-4/constitution_updates.md`
- Transform Engine: `src-tauri/src/editor/transforms.rs`
- Settings Model: `src-tauri/src/model/settings.rs`
- TipTap Editor: `www/src/components/TipTapEditor.jsx`
- Transform Hooks: `www/src/hooks/useTransforms.ts`

---

## Conclusion

Iteration-5 reveals that the constitutional gaps are not just about enforcing existing articles - the existing articles don't even cover the patterns that cause P0 issues:

1. **Article 12** (Engine Operation Completeness) - Required because TransformEngine has unhandled variants
2. **Article 13** (Transactional Integrity) - Required because undo/redo fidelity wasn't specified
3. **Article 14** (Architecture Conformance) - Required because JSON was substituted for rusqlite

**The fundamental problem:** Articles have been proposed for 4 iterations but never ratified. If Articles 1-5 had been ratified in iteration-1, would we now have Articles 12-14 issues? Possibly not - the systematic enforcement would have caught these patterns earlier.

**Recommendation:** Ratify ALL 14 articles (1-11 from previous iterations, 12-14 from this iteration) as a single ratification package. This represents 5 iterations of learning about what constitutional rules are actually necessary.

---

*Constitution update suggestions generated from iteration-5 gap analysis*
