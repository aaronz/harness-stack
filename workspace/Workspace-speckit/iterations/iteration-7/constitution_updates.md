# Constitution Update Suggestions - Iteration 7

**Project:** RustNote
**Date:** 2026-04-14
**Status:** No Constitution exists. Articles 1-16 proposed across iterations 1-6 were never ratified.
**Reference Gap Analysis:** iterations/iteration-7/gap-analysis.md

---

## Executive Summary

Iteration-7 gap analysis identifies **1 new P0 issue** that existing Articles 1-16 do NOT fully cover:

| P0 Issue | Root Cause | Constitutional Gap |
|----------|------------|-------------------|
| G-003: No automated performance benchmark infrastructure | PRD-14 NFR thresholds defined but not enforced via automated tests | Performance NFR Enforcement not covered |

**Critical Finding:** Articles 15-16 (proposed in iteration-6) address their target gaps well — Article 15 covers bidirectional mapping completeness, Article 16 covers export output fidelity. However, Article 16 covers output *quality* but does NOT cover output *verification infrastructure*. G-002 (PDF export quality) still needs the systematic verification that Article 16 describes, and G-003 (performance benchmarks) needs automated infrastructure entirely absent from the constitution.

**This iteration proposes:**
1. **Article 17: Performance NFR Enforcement** — Automated benchmark infrastructure as a constitutional requirement
2. **Article 18: MVP Release Criteria Formalization** — Concrete P0 checklist that blocks release until met
3. **Ratification of Articles 1-16** — 6 iterations of proposals without ratification is the root cause of accumulated issues

---

## Part I: Previous Articles Status

### Articles 1-16: Proposed but NOT Ratified (iterations 1-6)

| Article | Topic | Iteration Proposed | Iteration-7 Gap Coverage | Status |
|---------|-------|-------------------|-------------------------|--------|
| Article 1 | Complete Feature Implementation | 1 | G-006 (image paths) | ❌ Not Ratified |
| Article 2 | P0/P1 Feature Lifecycle Guarantee | 1 | — | ❌ Not Ratified |
| Article 3 | Settings-Implementation Parity | 2 | G-008 (settings schema) | ❌ Not Ratified |
| Article 4 | CSS-Logic Pairing | 2 | — | ❌ Not Ratified |
| Article 5 | Integration Before Merge | 1 | G-006 (GFM parsing) | ❌ Not Ratified |
| Article 6 | Workspace File Operations | 2 | — | ❌ Not Ratified |
| Article 7 | Editor Settings Propagation | 2 | G-008 (settings schema) | ❌ Not Ratified |
| Article 8 | Workflow Completion | 3 | — | ❌ Not Ratified |
| Article 9 | Keyboard Shortcut Standardization | 3 | — | ❌ Not Ratified |
| Article 10 | State Visibility | 3 | — | ❌ Not Ratified |
| Article 11 | Interactive Element Behavior Documentation | 4 | — | ❌ Not Ratified |
| Article 12 | Engine Operation Completeness | 5 | G-004 (wrap transform) | ❌ Not Ratified |
| Article 13 | Transactional Integrity Guarantee | 5 | — | ❌ Not Ratified |
| Article 14 | Architecture Technology Conformance | 5 | G-007 (GFM parsing) | ❌ Not Ratified |
| Article 15 | Bidirectional Mapping Completeness | 6 | G-001 (cursor mapping) — PARTIAL | ❌ Not Ratified |
| Article 16 | Export Output Fidelity | 6 | G-002 (PDF export) — PARTIAL | ❌ Not Ratified |

**Total Articles Proposed:** 16 (across 6 iterations)
**Total Articles Ratified:** 0
**Iteration-7 P0 issues:** 1 NEW gap that existing articles don't fully cover

---

## Part II: Iteration-7 Resolution of Previous P0 Issues

### Successfully Fixed (Due to Articles 12-16 Awareness)

| Issue | Root Cause | Resolution | Article Invoked |
|-------|------------|------------|----------------|
| G-001 (iter5): TransformEngine incomplete | Missing variants | ✅ All Transform variants now implemented | Article 12 |
| G-002 (iter5): Parser tree-sitter | Not implemented | ✅ tree-sitter wrapper implemented | Article 14 |
| G-003 (iter5): Buffer ropey | Not implemented | ✅ Ropey buffer implemented | Article 14 |
| G-004 (iter5): Settings JSON vs rusqlite | Architecture drift | ✅ rusqlite implemented | Article 14 |
| G-005 (iter5): Undo/redo fidelity | No tests | ✅ Tests added | Article 13 |
| G-006 (iter5): Cursor mapping | Not integrated | ✅ Tests added, edge cases remain | Article 15 |
| G-007 (iter6): PDF export quality | printpdf direct use | ⚠️ Working but quality unverified | Article 16 |

**Conclusion:** Articles 12-16, while never formally ratified, WERE followed in iteration-6/7 development and fixed the P0 issues they were designed to address. This is the **5th consecutive iteration** where following unratified articles produced better outcomes than ignoring them.

---

## Part III: P0 Issue Analysis

### G-003: No Automated Performance Benchmark Infrastructure (P0 - Blocking)

**Issue:** PRD-14 Section 8.3 defines NFR thresholds (cold start < 2s, keystroke → render < 100ms, etc.) but no automated benchmark infrastructure exists to validate them. Performance is "unknown" across all metrics.

**Gap Analysis Finding:**
- PRD-14 Table in Section 8.1 defines 12 performance thresholds
- `cargo bench` directory exists with benchmark files (`benches/transforms.rs`, `benches/serialization.rs`, `benches/parsing.rs`)
- **Problem:** No CI integration, no automated pass/fail, no regression detection
- **Problem:** Benchmarks exist but are not run systematically
- **Impact:** MVP release cannot verify it meets performance requirements

**Why Existing Articles Don't Cover This:**

| Article | Why Insufficient |
|---------|------------------|
| Article 14 (Architecture Conformance) | Covers technology choices but not automated NFR enforcement |
| Article 16 (Export Output Fidelity) | Covers output *quality* but not systematic *verification infrastructure* |
| Article 13 (Transactional Integrity) | Covers correctness, not performance |
| Article 12 (Engine Completeness) | Covers functional completeness, not performance thresholds |

**Root Cause:** No constitutional requirement that "performance NFRs must be validated by automated infrastructure in CI/CD."

---

### G-001: Cursor Mapping Bidirectional Conversion — Residual Issue (P0 - Still Present)

**Issue:** `CursorMapping::dom_to_source()` has edge cases. Tests exist (`cursor_mapping_tests.rs`) but full edge case coverage for DOM→source conversion with TipTap integration is not verified.

**Why Article 15 Is Partially Effective But Not Sufficient:**
- Article 15 covers the *principle* of bidirectional completeness
- Article 15 does NOT cover the *verification* requirement — it doesn't mandate integration tests with TipTap
- G-001 persists because "tests exist" satisfies Article 15's text, but the tests don't cover TipTap integration

**Gap:** Article 15 needs an addendum or companion article requiring bidirectional mappings to be verified via **end-to-end integration tests** with the actual frontend, not just unit tests in isolation.

---

### G-002: PDF Export Quality — Residual Issue (P0 - Still Present)

**Issue:** PDF export may not correctly render complex Markdown (tables, code blocks). `printpdf` is used directly without an HTML-to-PDF pipeline.

**Why Article 16 Is Partially Effective But Not Sufficient:**
- Article 16 covers the *principle* of export output fidelity
- Article 16 does NOT cover the *infrastructure* — it doesn't mandate reference document fixtures or visual comparison tests
- G-002 persists because "export works" satisfies Article 16's text superficially, but output quality is unverified

**Gap:** Article 16 needs an addendum requiring:
- Reference document fixtures for complex Markdown
- Visual comparison tests (even basic pixel-diff or structure-diff)
- Explicit pass/fail criteria per document type

---

## Part IV: Proposed New Articles

### Article 17: Performance NFR Enforcement (NEW)

**Intent:** Ensure performance non-functional requirements (NFRs) are validated by automated infrastructure, not just claimed.

**Text:**
> Performance NFRs defined in PRD-14 are binding and MUST be validated by automated infrastructure:
>
> Specifically:
> 1. Automated benchmark scripts MUST exist for ALL performance thresholds in PRD-14 Section 8.1
> 2. Benchmarks MUST be integrated into CI/CD pipeline and run on every PR
> 3. Performance regression detection MUST block PR merge when NFR thresholds are violated
> 4. Benchmark results MUST be published (as artifacts or comments) for transparency
> 5. "Performance is acceptable" without automated measurement is insufficient — measurements are required
>
> This applies to: cold start, hot file open, keystroke→render latency, save time, export time, outline update time, theme switch time, memory usage, scroll FPS.

**Application to Current Gap:**
- G-003: Benchmark files exist but are not integrated into CI, not run automatically
- PRD-14 Section 8.3 benchmark commands exist but are not executed in CI
- All 12 NFR thresholds currently have "unknown" status — this is a constitutional violation of Article 17

**Violation Example:**
```
PRD-14 NFR: Keystroke → render < 100ms
Current state: No automated benchmark for keystroke latency
Result: "Performance is unknown" → VIOLATION: Article 17
```

**This article would have prevented:**
- 6 months of "unknown" performance status
- No automated regression detection for performance

---

### Article 18: MVP Release Criteria Formalization (NEW)

**Intent:** Create a concrete, enforceable P0 checklist that blocks MVP release until all critical issues are resolved. This prevents the pattern of "P0 issues persist across iterations without resolution."

**Text:**
> RustNote is not ready for MVP release until ALL of the following are true:
>
> **Functional Completeness:**
> 1. All FR-001 to FR-034 functional requirements pass their defined tests
> 2. All editor invariants from PRD-02 Section 7 are verified by automated tests
> 3. Bidirectional cursor mapping (DOM↔source) passes round-trip integration tests with TipTap
> 4. PDF export produces visually verified output for reference documents covering all Markdown types
>
> **Performance:**
> 5. ALL 12 NFR thresholds from PRD-14 Section 8.1 are measured and pass on every platform
> 6. No P0 or P1 issues remain open in the gap analysis
>
> **Reliability:**
> 7. Autosave and crash recovery are tested end-to-end on all platforms
> 8. External file change detection is tested on all platforms
> 9. Edit-save-reopen cycle preserves document fidelity for all Markdown constructs
>
> **Security:**
> 10. All SEC-* tests from PRD-14 Section 9 pass
> 11. No P1 or P0 security issues remain open
>
> **Process:**
> 12. Articles 1-17 of this Constitution have been formally ratified
> 13. Constitution violation rate in the release cycle is 0% for P0/P1 items
>
> "90% complete" is not an MVP release criterion. 100% on the checklist above is the criterion.

**Application to Current Gap:**
- Iteration-7 reports "90% complete" — Article 18 formalizes what "100%" actually means
- G-003 (performance), G-001 (cursor mapping), G-002 (PDF export) all block Article 18
- This prevents the iteration-to-iteration "still at 90%" pattern

**Violation Example:**
```
MVP readiness: 90% complete, 3 P0 issues open
→ VIOLATION: Article 18 (must be 100% on checklist)
```

---

## Part V: Iteration-7 Gap-to-Article Mapping

### P0 Issues in Iteration-7

| Gap | Root Cause | Article That Covers | Coverage |
|-----|------------|---------------------|----------|
| G-001: Cursor bidirectional mapping edge cases | TipTap integration not verified by Article 15 tests | Article 15 (partial) | Bidirectional principle covered; integration verification NOT covered |
| G-002: PDF export quality unverified | No visual verification infrastructure | Article 16 (partial) | Output fidelity principle covered; verification infrastructure NOT covered |
| G-003: No automated performance benchmarks | No benchmark CI integration | None existing | **NEW Article 17 needed** |

### Iteration-7 P1 Issues

| Gap | Article That Covers |
|-----|---------------------|
| G-004: Wrap transform with selection | Article 12 (Engine Completeness) |
| G-005: HTML linked-assets export | Article 16 (Export Fidelity) — partial |
| G-006: Image relative path edge cases | Article 1 (Complete Implementation) |
| G-007: GFM tables/task lists parsing | Article 14 (Architecture) |
| G-008: Settings schema verification | Article 3 (Settings Parity) + Article 7 |
| G-009: Editor.jsx deprecation | Maintenance concern — no new article needed |

### Cumulative Gap Analysis: Which Gaps Did NOT Have Covering Articles

| Iteration | Gap | Was New Article Needed? |
|-----------|-----|----------------------|
| 5 | TransformEngine incomplete | ✅ Article 12 (Engine Completeness) |
| 5 | Undo/redo fidelity | ✅ Article 13 (Transactional Integrity) |
| 5 | Settings JSON vs rusqlite | ✅ Article 14 (Architecture Conformance) |
| 6 | Cursor bidirectional mapping | ✅ Article 15 (Bidirectional Mapping) |
| 6 | Export output fidelity | ✅ Article 16 (Export Fidelity) |
| **7** | **Performance NFR enforcement** | **✅ Article 17 (NEW)** |
| **7** | **MVP release criteria** | **✅ Article 18 (NEW)** |

**Pattern:** Every iteration introduces 1-2 new constitutional gaps requiring new articles. This is expected and healthy — constitutional documents evolve with the project. What is NOT healthy is that Articles 1-16 have never been ratified, so they have no enforcement mechanism.

---

## Part VI: Ratification Package

### For Ratification: Articles 1-16 (iterations 1-6)

These 16 articles represent **6 iterations** of learning about constitutional gaps. They address patterns that repeatedly caused issues:

| Priority | Articles | Proven Necessary |
|----------|----------|-----------------|
| Must Ratify | 1-11 | Proven over 4 iterations |
| Must Ratify | 12-14 | Proven necessary by iteration-5 P0 issues (now fixed) |
| Must Ratify | 15-16 | Proven necessary by iteration-6 P0 issues (G-001, G-002 still residual but principle is right) |

### For Ratification: Articles 17-18 (iteration-7)

These address P0 issues not covered by existing articles:

| Article | Addresses | Issue Prevented |
|---------|-----------|----------------|
| Article 17 | Performance NFR Enforcement | G-003: No automated benchmark infrastructure |
| Article 18 | MVP Release Criteria Formalization | Pattern of "90% complete" indefinitely |

### Critical: Ratify Complete Constitution

After 7 iterations of proposing articles without ratification:
- 11 articles from iterations 1-4
- 3 articles (12-14) from iteration-5
- 2 articles (15-16) from iteration-6
- 2 articles (17-18) from iteration-7

**Total: 18 articles pending ratification**

This represents the most comprehensive constitutional learning in the project's history. Ratifying all 18 at once establishes a clean, enforceable baseline.

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
## Article 15: Bidirectional Mapping Completeness
## Article 16: Export Output Fidelity
## Article 17: Performance NFR Enforcement (NEW)
## Article 18: MVP Release Criteria Formalization (NEW)

## Amendment Process
Constitution amendments require:
1. Proposal in gap-analysis.md with justification
2. Review in iteration planning
3. Explicit ratification vote
4. Update to Constitution.md
```

---

## Part VIII: Iteration-7 Lessons Learned

### What Worked: Articles 15-16 Enforcement

Despite never being formally ratified, Articles 15-16 were followed:
- Cursor mapping has bidirectional tests (Article 15 awareness)
- Export fidelity is a stated concern (Article 16 awareness)
- The principle is right; the enforcement is missing

**Lesson:** Articles 15-16 are necessary but need **Article 18** to formalize the verification requirement.

### What Didn't Work: Performance NFRs Remain Unmeasured

Iteration-6 added `benches/` directory. Iteration-7 reports all performance as "unknown."
- Benchmark files exist but are not run in CI
- No regression detection
- NFR thresholds are decoration, not requirements

**Lesson:** Without Article 17 (Performance NFR Enforcement), NFRs are aspirational, not binding.

### The Root Problem: Non-Ratification Pattern

| Iteration | Articles Proposed | Articles Ratified | Result |
|-----------|------------------|-------------------|--------|
| 1 | 5 | 0 | Same patterns persisted |
| 2 | 7 | 0 | Same patterns persisted |
| 3 | 10 | 0 | Some resolved, new patterns emerged |
| 4 | 11 | 0 | Some resolved, new patterns emerged |
| 5 | 14 | 0 | P0 issues fixed by following unratified articles |
| 6 | 16 | 0 | P0 issues fixed by following unratified articles |
| 7 | 18 | 0 | Same pattern continues |

**The pattern is proven:** Articles that are followed (even without ratification) produce better outcomes. But the lack of ratification means:
- No enforcement mechanism
- No PR checklist blocking violations
- Each iteration re-discovers the same patterns

**Article 18 directly addresses this:** The MVP cannot ship until Articles 1-17 are ratified. This creates a hard deadline for ratification.

---

## Part IX: Immediate Action Items

### Fix Iteration-7 P0 Issues

| P0 Issue | Required Fix | Article Invoked |
|----------|-------------|----------------|
| G-003: No performance benchmarks | Integrate `benches/` into CI, add `npm run bench` to PR pipeline | Article 17 |
| G-001: Cursor mapping edge cases | Add TipTap integration tests for round-trip mapping | Article 15 |
| G-002: PDF export quality | Add visual verification tests with reference documents | Article 16 |

### Ratify Constitution

| Action | Required |
|--------|----------|
| Ratify Articles 1-18 | 7 iterations of gap analysis proves necessity |
| Create `Constitution.md` at project root | Formal document for enforcement |
| Add Constitution checklist to PR template | Articles 1-18 block merge if violated |

---

## Part X: Constitutional Principles Summary

Based on 7 iterations of gap analysis:

1. **Completion Means Complete** — Features must work end-to-end, not just have working parts
2. **All Operations Required** — N-1 of N operations is broken, not 99% complete
3. **Fidelity Is Non-Negotiable** — Undo/redo and export must preserve document structure
4. **Architecture Is Binding** — PRD specifications must be followed
5. **Bidirectional Mappings Must Work Both Ways** — Conversions must work in both directions
6. **Export Output Must Be Verified** — "Exports" is insufficient, must "render correctly"
7. **Performance NFRs Require Automated Validation** — "Acceptable performance" without measurement is insufficient
8. **MVP Readiness Has a Concrete Definition** — "90% complete" is not a release criterion
9. **Ratification Prevents Accumulation** — Proposed articles must be ratified or rejected with a deadline

---

## Part XI: Files Referenced

- Gap Analysis: `iterations/iteration-7/gap-analysis.md`
- Previous Proposals: `iterations/iteration-1/through/iteration-6/constitution_updates.md`
- Performance Benchmarks: `src-tauri/benches/`
- Test Plan: `docs/PRD/14-test-plan.md`
- Product Invariants: `docs/PRD/02-product-invariants.md`
- Governance: `docs/PRD/13-governance.md`
- Cursor Mapping: `src-tauri/src/semantic/position.rs`
- PDF Export: `src-tauri/src/commands/export.rs`

---

## Conclusion

Iteration-7 demonstrates a maturing project with shrinking constitutional gaps:

- Articles 12-16 (proposed in iterations 5-6) are working — previous P0 issues are resolved
- The remaining P0 issues (G-001, G-002) are residual — Article 15 and 16 need *addenda* (integration tests, visual verification) not entirely new articles
- G-003 (performance benchmarks) is genuinely new — Article 17 addresses it
- Article 18 creates the enforcement mechanism that Articles 1-16 have lacked for 6 iterations

**Root issue remains unchanged since iteration-1:** Articles are proposed but never ratified. The fix is the same as it has been for 6 iterations: **ratify the constitution**. The difference now is Article 18 makes ratification a hard MVP prerequisite.

**Recommendation:** Ratify ALL 18 articles (1-16 from previous iterations, 17-18 from this iteration) as a single ratification package with a hard deadline before MVP release. This represents 7 iterations of learning about what constitutional rules are actually necessary.

---

*Constitution update suggestions generated from iteration-7 gap analysis*
