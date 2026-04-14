# Constitution Update Suggestions - Iteration 6

**Project:** RustNote
**Date:** 2026-04-14
**Status:** No Constitution exists. Articles 1-14 proposed across iterations 1-5 were never ratified.
**Reference Gap Analysis:** iterations/iteration-6/gap-analysis.md

---

## Executive Summary

Iteration-6 gap analysis identifies **2 new P0 issues** that reveal constitutional gaps NOT covered by existing proposed Articles 1-14:

| P0 Issue | Root Cause | Constitutional Gap |
|----------|------------|-------------------|
| G-001: Cursor mapping bidirectional conversion incomplete | `dom_to_source()` has edge cases | Bidirectional Mapping not explicitly covered |
| G-002: PDF export quality for complex Markdown | `printpdf` may not render tables/code blocks correctly | Export Output Fidelity not covered |

**Critical Finding:** Iteration-6 resolved several iteration-5 P0 issues (rusqlite, tree-sitter, ropey, TransformEngine completeness) due to Articles 12-14 enforcement. However, the 2 remaining P0 issues reveal NEW constitutional gaps:

1. **Bidirectional Mapping Completeness** (Article 15) - When converting positions between DOM↔source, BOTH directions must work with full edge case coverage
2. **Export Output Fidelity** (Article 16) - When exporting to PDF/HTML, output must faithfully represent document structure

---

## Part I: Previous Articles Status

### Articles 1-14: Proposed but NOT Ratified (iterations 1-5)

| Article | Topic | Iteration Proposed | Status |
|---------|-------|-------------------|--------|
| Article 1 | Complete Feature Implementation | 1 | ❌ Not Ratified |
| Article 2 | P0/P1 Feature Lifecycle Guarantee | 1 | ❌ Not Ratified |
| Article 3 | Settings-Implementation Parity | 2 | ❌ Not Ratified |
| Article 4 | CSS-Logic Pairing | 2 | ❌ Not Ratified |
| Article 5 | Integration Before Merge | 3 | ❌ Not Ratified |
| Article 6 | Workspace File Operations | 3 | ❌ Not Ratified |
| Article 7 | Editor Settings Propagation | 3 | ❌ Not Ratified |
| Article 8 | Workflow Completion | 4 | ❌ Not Ratified |
| Article 9 | Keyboard Shortcut Standardization | 4 | ❌ Not Ratified |
| Article 10 | State Visibility | 4 | ❌ Not Ratified |
| Article 11 | Interactive Element Behavior Documentation | 4 | ❌ Not Ratified |
| Article 12 | Engine Operation Completeness | 5 | ❌ Not Ratified |
| Article 13 | Transactional Integrity Guarantee | 5 | ❌ Not Ratified |
| Article 14 | Architecture Technology Conformance | 5 | ❌ Not Ratified |

**Total Articles Proposed:** 14 (across 5 iterations)
**Total Articles Ratified:** 0
**P0 issues in iteration-6:** 2 NEW issues that existing articles don't fully cover

---

## Part II: Iteration-6 Resolution of Iteration-5 P0 Issues

### Successfully Fixed (Due to Article 12-14 Enforcement)

| Issue | Root Cause | Resolution |
|-------|------------|------------|
| G-003 (iter5): Settings JSON vs rusqlite | Architecture drift | ✅ Now uses rusqlite (Article 14 enforced) |
| G-001 (iter5): TransformEngine incomplete | Missing variants | ✅ All Transform variants now implemented (Article 12) |
| G-006 (iter5): Parser tree-sitter | Not implemented | ✅ tree-sitter wrapper implemented (Article 14) |
| G-007 (iter5): Buffer ropey | Not implemented | ✅ Ropey buffer implemented (Article 14) |
| G-002 (iter5): Undo/redo fidelity | No tests | ✅ Tests added (Article 13 awareness) |

**Conclusion:** Articles 12-14, while never formally ratified, WERE enforced in iteration-6 development and FIXED the P0 issues they were designed to address. This proves the articles are effective if followed.

---

## Part III: P0 Issue Analysis

### G-001: Cursor Mapping Bidirectional Conversion (P0 - Blocking)

**Issue:** `CursorMapping::build_cursor_mapping()` creates source-to-DOM mapping successfully, but `CursorMapping::dom_to_source()` may have edge cases where DOM position cannot be correctly converted back to source position.

**Gap Analysis Finding:**
- `semantic/position.rs` implements bidirectional cursor mapping
- `build_cursor_mapping()` (source→DOM) works correctly
- `dom_to_source()` (DOM→source) has edge cases with:
  - Nested inline elements (links within emphasis)
  - Code spans at block boundaries
  - Empty elements
  - Mixed bidirectional content (RTL within LTR)

**Why Existing Articles Don't Cover This:**

| Article | Why Insufficient |
|---------|------------------|
| Article 5 (Integration Before Merge) | Covers API integration but not bidirectional mapping contract |
| Article 12 (Engine Operation Completeness) | Covers "all operations must be implemented" but G-001 IS implemented - it just has edge cases |
| Article 13 (Transactional Integrity) | Covers undo/redo fidelity, not position mapping fidelity |

**Root Cause:** No constitutional requirement that "bidirectional mappings must maintain fidelity in both directions with edge case coverage."

---

### G-002: PDF Export Quality for Complex Markdown (P0 - Blocking)

**Issue:** PDF export `printpdf` implementation may not properly render complex Markdown structures (tables, code blocks with syntax highlighting).

**Gap Analysis Finding:**
- HTML export works correctly
- PDF export uses `printpdf` crate directly
- **Problem:** Tables may not render with proper borders/alignment
- **Problem:** Code blocks may lose syntax highlighting
- **Problem:** Complex nested structures may reflow incorrectly

**Why Existing Articles Don't Cover This:**

| Article | Why Insufficient |
|---------|------------------|
| Article 5 (Integration Before Merge) | Mentions export must work but doesn't specify output quality |
| Article 13 (Transactional Integrity) | Covers undo/redo fidelity, not export output fidelity |
| Article 14 (Architecture Conformance) | Covers technology choice, not output quality verification |

**Root Cause:** No constitutional requirement that "export output must faithfully represent document structure and styling."

---

## Part IV: Proposed New Articles

### Article 15: Bidirectional Mapping Completeness (NEW)

**Intent:** Prevent incomplete bidirectional conversions where one direction works but the other has edge cases.

**Text:**
> When a module implements bidirectional conversion between two representations (such as DOM↔source, screen↔buffer, or index↔position), ALL conversion directions MUST be complete and edge cases MUST be handled:
>
> Specifically:
> 1. If `a_to_b()` exists, `b_to_a()` MUST also exist and work correctly
> 2. Round-trip conversion (a→b→a) MUST produce identical result to original
> 3. Edge cases (empty values, nested structures, boundary conditions) MUST be handled in BOTH directions
> 4. Missing edge case handling in one direction constitutes incomplete implementation, not partial implementation
>
> This applies to: cursor mapping, position indexing, coordinate systems, and any paired conversion functions.

**Application to Current Gap:**
- G-001: `CursorMapping` has `source_to_dom()` working but `dom_to_source()` has edge cases
- Round-trip tests (source→dom→source) MUST pass for ALL positions
- Edge cases MUST be documented and tested

**Violation Example:**
```
build_cursor_mapping() → source_to_dom() works at position 100
dom_to_source() → fails at position 100 when DOM has nested elements
→ VIOLATION: Article 15 (bidirectional completeness)
```

---

### Article 16: Export Output Fidelity (NEW)

**Intent:** Ensure export output faithfully represents the document's visual appearance and structure.

**Text:**
> When exporting to a format (PDF, HTML, DOCX), the output MUST faithfully represent the document's visual appearance:
>
> Specifically:
> 1. Document structure (headings, lists, tables, quotes) MUST render with correct visual hierarchy
> 2. Inline formatting (emphasis, links, code, images) MUST render correctly
> 3. Syntax highlighting MUST be preserved in code blocks
> 4. Complex Markdown (tables with borders, nested structures) MUST render correctly
> 5. Export output MUST be verified against reference documents representing common Markdown patterns
> 6. "Exports successfully" without "renders correctly" is insufficient
>
> Verification requires: visual comparison tests, reference document fixtures, and explicit pass/fail criteria.

**Application to Current Gap:**
- G-002: PDF export may not correctly render tables and code blocks
- Visual verification tests MUST be added for complex Markdown
- Reference documents representing complex Markdown MUST be tested

**Violation Example:**
```
HTML export: table with proper borders ✅
PDF export: table borders missing ❌
→ VIOLATION: Article 16 (export output fidelity)
```

---

## Part V: Iteration-6 Gap-to-Article Mapping

### P0 Issues in Iteration-6

| Gap | Article That Covers | Article That Would Have Prevented |
|-----|---------------------|----------------------------------|
| G-001: Cursor bidirectional mapping | Article 12 (partial - if treated as operation) | Article 15 (NEW): Bidirectional Mapping Completeness |
| G-002: PDF export quality | None existing | Article 16 (NEW): Export Output Fidelity |

### Iteration-6 P1 Issues

| Gap | Article That Covers | Article That Would Have Prevented |
|-----|---------------------|----------------------------------|
| G-003: Wrap transform with selection | Article 12 (Engine Completeness) | - |
| G-004: Linked-assets HTML export | Article 1 (Complete Feature) | Article 16 (Export Fidelity) - partial |
| G-005: Image relative paths | Article 1 (Complete Feature) | - |
| G-006: GFM tables/task lists parsing | Article 14 (Architecture Conformance) | - |
| G-007: Settings nested vs flat schema | Article 3 (Settings Parity) | - |
| G-008: Editor.jsx/TipTapEditor.jsx duplication | Article 4 (CSS-Logic Pairing) | - |

---

## Part VI: Ratification Package

### For Ratification: Articles 1-16 (all articles proposed across all iterations)

These 16 articles represent 6 iterations of learning about constitutional gaps:

| Priority | Articles | Proven Necessary |
|----------|----------|-----------------|
| Must Ratify | 1-11 | Proven over 4 iterations |
| Must Ratify | 12-14 | Proven necessary by iteration-5 P0 issues (now fixed) |
| Must Ratify | 15-16 | Proven necessary by iteration-6 P0 issues |

### Critical: Ratify Complete Constitution

After 6 iterations of proposing articles without ratification:
- 11 articles from iterations 1-4
- 3 articles (12-14) from iteration-5
- 2 articles (15-16) from iteration-6

**Total: 16 articles pending ratification**

---

## Part VII: Recommended Constitution Structure

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
## Article 12: Engine Operation Completeness
## Article 13: Transactional Integrity Guarantee
## Article 14: Architecture Technology Conformance
## Article 15: Bidirectional Mapping Completeness (NEW)
## Article 16: Export Output Fidelity (NEW)

## Amendment Process
Constitution amendments require:
1. Proposal in gap-analysis.md with justification
2. Review in iteration planning
3. Explicit ratification vote
4. Update to Constitution.md
```

---

## Part VIII: Iteration-6 Lessons Learned

### What Worked: Article 12-14 Enforcement

Despite never being formally ratified, iteration-6 development followed Articles 12-14 principles:
- rusqlite implemented (Article 14)
- tree-sitter implemented (Article 14)
- ropey implemented (Article 14)
- All Transform variants implemented (Article 12)
- Undo/redo tests added (Article 13 awareness)

**Lesson:** Articles 12-14 are effective when followed.

### What Didn't Work: Missing Bidirectional & Export Articles

Iteration-6 still has P0 issues because Articles 15-16 don't exist:
- Cursor mapping has edge cases in one direction (Article 15 needed)
- PDF export quality not verified (Article 16 needed)

**Lesson:** Constitutional gaps accumulate when articles are proposed but not ratified.

---

## Part IX: Immediate Action Items

### Fix Iteration-6 P0 Issues

| P0 Issue | Required Fix | Article Invoked |
|----------|-------------|-----------------|
| G-001: Cursor bidirectional | Add round-trip tests, fix `dom_to_source()` edge cases | Article 15 |
| G-002: PDF export quality | Visual verification tests, fix table/code rendering | Article 16 |

### Ratify Constitution

| Action | Required |
|--------|----------|
| Ratify Articles 1-16 | 6 iterations of gap analysis proves necessity |

---

## Part X: Constitutional Principles Summary

Based on 6 iterations of gap analysis:

1. **Completion Means Complete** - Features must work end-to-end, not just have working parts
2. **All Operations Required** - N-1 of N operations is broken, not 99% complete
3. **Fidelity Is Non-Negotiable** - Undo/redo and export must preserve document structure
4. **Architecture Is Binding** - PRD specifications must be followed
5. **Bidirectional Mappings Must Work Both Ways** - Conversions must work in both directions
6. **Export Output Must Be Verified** - "Exports" is insufficient, must "render correctly"
7. **Ratification Prevents Accumulation** - Proposed articles must be ratified or rejected

---

## Part XI: Files Referenced

- Gap Analysis: `iterations/iteration-6/gap-analysis.md`
- Previous Proposals: `iterations/iteration-1/through/iteration-5/constitution_updates.md`
- Cursor Mapping: `src-tauri/src/semantic/position.rs`
- PDF Export: `src-tauri/src/commands/export.rs`
- Product Invariants: `docs/PRD/02-product-invariants.md`

---

## Conclusion

Iteration-6 demonstrates that:
1. Articles 12-14 (proposed in iteration-5) ARE effective when followed - they fixed iteration-5 P0 issues
2. Articles 15-16 are needed for iteration-6 P0 issues - bidirectional mapping and export fidelity
3. Articles continue to accumulate because none have been ratified

**Recommendation:** Ratify ALL 16 articles (1-16) as a single ratification package. This represents 6 iterations of learning about what constitutional rules are actually necessary to prevent P0 issues.

---

*Constitution update suggestions generated from iteration-6 gap analysis*
