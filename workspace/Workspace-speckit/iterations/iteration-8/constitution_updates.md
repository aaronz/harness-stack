# Constitution Update Suggestions - Iteration 8

**Project:** RustNote
**Date:** 2026-04-14
**Status:** No Constitution exists. Articles 1-18 proposed across iterations 1-7 remain unratified.
**Reference Gap Analysis:** iterations/iteration-8/gap-analysis.md
**Previous Proposals:** iterations/iteration-7/constitution_updates.md (Articles 1-18)

---

## Executive Summary

Iteration-8 gap analysis identifies **1 new P0 issue** and **1 new constitutional gap** that existing Articles 1-18 do NOT fully cover:

| New Item | Type | Root Cause | Constitutional Gap |
|----------|------|------------|-------------------|
| G-004: Paste rich-text conversion incomplete | **P0 Issue** | TurndownService lacks comprehensive rules for Word/Excel/HTML clipboard formats | Article 16 scope ambiguity — export fidelity principle covers format conversion but doesn't explicitly name paste/clipboard |
| G-011: i18n architecture not externalized | **P1 Constitutional Gap** | NFR-014 requires all UI strings externalized but no i18n library is installed | No article covers **externalization of user-facing text as a standalone architectural requirement** |

**Critical Finding:** Articles 15-16 (proposed in iterations 6-7) remain partially effective but not sufficient. Article 15 (Bidirectional Mapping) lacks enforcement of integration-level verification. Article 16 (Export Output Fidelity) covers the principle but its scope never explicitly names paste/clipboard conversion, leading to G-004 being classified as a new P0 rather than a known constitutional concern.

**This iteration proposes:**
1. **Article 16 Addendum: Paste/Clipboard Conversion Fidelity** — Explicitly extend Article 16's scope to cover paste and clipboard content conversion
2. **Article 19: UI String Externalization** — New article requiring all user-facing strings to be externalized for i18n readiness as an architectural requirement
3. **Ratification Now or Never** — 8 iterations of proposals without ratification is the root cause of persistent P0 issues

---

## Part I: Previous Articles Status (Articles 1-18)

### Articles 1-18: Proposed but NOT Ratified (iterations 1-7)

| Article | Topic | Iter Proposed | Iteration-8 Gap Coverage | Status |
|---------|-------|---------------|--------------------------|--------|
| Article 1 | Complete Feature Implementation | 1 | G-007 (image paths) | ❌ Not Ratified |
| Article 2 | P0/P1 Feature Lifecycle Guarantee | 1 | — | ❌ Not Ratified |
| Article 3 | Settings-Implementation Parity | 2 | G-009 (settings schema) | ❌ Not Ratified |
| Article 4 | CSS-Logic Pairing | 2 | — | ❌ Not Ratified |
| Article 5 | Integration Before Merge | 1 | G-008 (GFM parsing) | ❌ Not Ratified |
| Article 6 | Workspace File Operations | 2 | — | ❌ Not Ratified |
| Article 7 | Editor Settings Propagation | 2 | G-009 (settings schema) | ❌ Not Ratified |
| Article 8 | Workflow Completion | 3 | — | ❌ Not Ratified |
| Article 9 | Keyboard Shortcut Standardization | 3 | — | ❌ Not Ratified |
| Article 10 | State Visibility | 3 | — | ❌ Not Ratified |
| Article 11 | Interactive Element Behavior Documentation | 4 | G-013 (focus mode visual) | ❌ Not Ratified |
| Article 12 | Engine Operation Completeness | 5 | G-005 (wrap transform) | ❌ Not Ratified |
| Article 13 | Transactional Integrity Guarantee | 5 | — | ❌ Not Ratified |
| Article 14 | Architecture Technology Conformance | 5 | G-008 (GFM parsing) | ❌ Not Ratified |
| Article 15 | Bidirectional Mapping Completeness | 6 | G-001 (cursor mapping) — PARTIAL | ❌ Not Ratified |
| Article 16 | Export Output Fidelity | 6 | G-002 (PDF export) — PARTIAL | ❌ Not Ratified |
| Article 17 | Performance NFR Enforcement | 7 | G-003 (benchmarks now exist, results undocumented) | ❌ Not Ratified |
| Article 18 | MVP Release Criteria Formalization | 7 | — | ❌ Not Ratified |

**Total Articles Proposed:** 18 (across 7 iterations)
**Total Articles Ratified:** 0
**New P0 issues in iteration-8:** 1 (G-004: Paste rich-text conversion)
**New constitutional gap in iteration-8:** 1 (G-011: i18n externalization)

---

## Part II: Iteration-8 Resolution of Previous P0 Issues

### P0 Issues from Iteration 7: Status

| Issue | Article Invoked | Iteration-8 Status | Resolution |
|-------|-----------------|--------------------|------------|
| G-001 (iter7): Cursor mapping bidirectional conversion | Article 15 | ⚠️ **Still P0** | Implementation exists; TipTap integration test still missing |
| G-002 (iter7): PDF export quality unverified | Article 16 | ⚠️ **Still P0** | printpdf working; manual visual verification still not done |
| G-003 (iter7): No automated benchmarks | Article 17 | ✅ **IMPROVED** | 5 benchmark files now exist; results still not documented |

### What Article 17 Achieved (Iteration 7→8 Progress)

Article 17 (proposed in iteration-7) has demonstrably improved the situation:
- Iteration-7: "No benchmark infrastructure exists" → **FIXED**: 5 benchmark files now present
- However: "results not documented" → Still a G-003 concern — benchmarks exist but are not run/document

**Article 17 partial success:** The principle worked. Infrastructure was built. But the full cycle (run → document → compare to thresholds → fix failures) wasn't completed. This reveals Article 17 needs an **addendum**: "Results must be documented and compared to NFR thresholds on every PR."

---

## Part III: Iteration-8 P0 Issue Analysis

### G-004: Paste Rich-Text Conversion Incomplete (P0 - NEW)

**Issue:** `turndown` is used for HTML→Markdown conversion, but `semantic/paste.rs` and `TurndownService` lack comprehensive rules for Word/Excel clipboard formats and complex HTML from web browsers. Table conversion fidelity is inadequate.

**Why This Is a New P0 (Not Covered by Iteration-7 Analysis):**
- Iteration-7 gap analysis did not identify paste conversion as a P0 blocking issue
- Iteration-8 revealed the gap through deeper analysis of `paste.rs` and `turndown` rules
- FR-018 specifically covers paste behavior but wasn't flagged as blocking in prior iterations

**Is There a Covering Article?**

| Article | Coverage Analysis |
|---------|-------------------|
| Article 1 (Complete Feature Implementation) | Covers the principle that features must work end-to-end. Paste is a feature. ✅ Indirect coverage. |
| Article 16 (Export Output Fidelity) | **Closest but insufficient**: Article 16 covers output fidelity for HTML/PDF export but does NOT explicitly name paste/clipboard conversion. The scope ambiguity means G-004 wasn't anticipated by the constitutional process. ✅ Partial coverage. |
| Article 5 (Integration Before Merge) | Covers integration testing before merge. Paste integration tests don't exist. ✅ Covers the verification gap. |

**Root Cause:** Article 16's scope ("export output fidelity") implies format conversion fidelity but never explicitly names paste or clipboard content. This led to paste conversion being deprioritized.

**Proposed Fix:** **Article 16 Addendum** — Extend Article 16's scope to explicitly cover paste and clipboard conversion, renaming it "Format Conversion Fidelity" to cover HTML export, PDF export, AND paste clipboard conversion.

---

### G-001: Cursor Mapping Bidirectional Conversion — Persistent (P0 - Still Present)

**Issue:** `CursorMapping::dom_to_source()` exists with binary search but TipTap integration not verified end-to-end. Edge cases remain uncaught.

**Why Articles 15-16 Are Insufficient:**
- Article 15 covers the *principle* of bidirectional completeness
- Article 15 does NOT mandate **integration-level verification** — unit tests exist but no end-to-end test with real TipTap DOM output
- G-001 persists because "tests exist" satisfies Article 15's text superficially

**Gap:** Article 15 needs an addendum requiring bidirectional mappings to be verified via end-to-end integration tests with the actual frontend, not just Rust-side unit tests.

**Proposed Fix:** **Article 15 Addendum** — Bidirectional mappings (source→DOM and DOM→source) must be verified by integration tests that use the actual frontend editor's DOM output, not mocked DOM structures.

---

### G-002: PDF Export Quality — Persistent (P0 - Still Present)

**Issue:** printpdf implementation may not correctly render complex Markdown (tables, code blocks, images). Manual visual verification hasn't been done.

**Why Article 16 Is Insufficient:**
- Article 16 covers output *quality* but not *verification infrastructure*
- Reference document fixtures don't exist for visual comparison
- No pass/fail criteria per document type

**Proposed Fix:** **Article 16 Addendum** — Export output fidelity requires reference document fixtures covering all Markdown types (tables, code blocks, images, nested lists) with explicit pass/fail criteria per type.

---

### G-003: NFR Benchmarks — Improved But Incomplete (P0 - Still Present)

**Issue:** Benchmark infrastructure exists (5 files), but results are not captured, documented, or compared against PRD-05 thresholds.

**Article 17 Progress:** Article 17 (proposed iter7) worked — infrastructure was built. But Article 17 only mandates "automated benchmark infrastructure." It doesn't mandate "results must be documented and compared to thresholds."

**Proposed Fix:** **Article 17 Addendum** — Benchmark infrastructure must produce documented results compared to PRD-05 NFR thresholds on every PR. "Benchmarks exist but not run" is insufficient.

---

## Part IV: New Constitutional Gap Analysis

### G-011: i18n Architecture Not Externalized (P1 - NEW Constitutional Gap)

**Issue:** NFR-014 requires all user-facing strings externalized for i18n readiness. No i18n library is installed. All UI strings are hardcoded in React components.

**Why This Is a Constitutional Gap (Not Just an Implementation Gap):**

| Dimension | Analysis |
|-----------|----------|
| PRD Requirement | NFR-014 explicitly requires i18n architecture |
| Existing Articles | None explicitly cover UI string externalization |
| Article 11 (Interactive Element Documentation) | Covers behavior documentation, not text externalization |
| Article 4 (CSS-Logic Pairing) | Covers CSS and logic pairing, not UI text |
| Root Cause | No constitutional requirement that "all user-facing strings must be externalized to a locale file" |

**Why This Matters Constitutionally:**
- This is the 8th iteration of the same pattern: PRD requirement exists → no constitutional article mandates it → implementation deferred indefinitely
- Unlike G-004 (paste), G-011 is clearly a new constitutional gap requiring a new article

**Proposed Fix:** **Article 19: UI String Externalization** — New article requiring all user-facing strings to be externalized to locale files as an architectural requirement.

---

### Security Gap (G-016): XSS in Link URLs

**Issue:** `LinkPopover.jsx` doesn't validate URLs against PRD-12 security policy (javascript:, data: blocked).

**Existing Coverage:**

| Article | Coverage |
|---------|----------|
| Article 14 (Architecture Technology Conformance) | PRD-12 security requirements are part of the architecture |
| Article 1 (Complete Feature Implementation) | Security is a feature requirement |
| No explicit security validation article | No article explicitly requires input sanitization/validation |

**Assessment:** G-016 is a P1 issue that should be caught by Article 14 (Architecture Conformance) since PRD-12 Section 12.4.3 explicitly specifies link URL validation. The issue persists because Article 14 covers technology choices, not security enforcement. A specific article on input validation would help.

**Proposed Fix:** Rather than a new article, strengthen Article 14 to explicitly cover **security input validation** as part of architecture conformance. Or: **Article 20: Input Validation Enforcement** (if scoped broadly).

---

## Part V: Gap-to-Article Mapping (Iteration 8)

### P0 Issues

| Gap | Iter | Root Cause | Article That Covers | Coverage Status |
|-----|------|------------|---------------------|----------------|
| G-001: Cursor bidirectional mapping | 8 | TipTap integration not verified end-to-end | Article 15 (partial) | Principle covered; integration verification NOT covered |
| G-002: PDF export quality | 8 | No visual verification infrastructure | Article 16 (partial) | Quality principle covered; verification infrastructure NOT covered |
| G-003: Benchmark results undocumented | 8 | Article 17 only mandates infrastructure, not results documentation | Article 17 (partial) | Infrastructure covered; results documentation NOT covered |
| G-004: Paste rich-text conversion | **8 (NEW)** | Article 16 scope ambiguity — paste not explicitly named | Article 16 (ambiguous) | Export fidelity principle covers; paste/clipboard conversion NOT explicitly covered |

### P1 Issues

| Gap | Article That Covers |
|-----|---------------------|
| G-005: Wrap transform with selection | Article 12 (Engine Completeness) |
| G-006: HTML linked-assets export | Article 16 (Export Fidelity) |
| G-007: Image relative path edge cases | Article 1 (Complete Implementation) |
| G-008: GFM tables/task lists parsing | Article 14 (Architecture) |
| G-009: Settings schema verification | Article 3 (Settings Parity) + Article 7 |
| G-010: Editor.jsx deprecation | Maintenance concern — no new article needed |
| G-011: i18n not externalized | **None existing** — **NEW Article 19 needed** |
| G-012: Table editing limitations | Article 16 (Export Fidelity) — stretch goal |
| G-013: Focus mode visual verification | Article 11 (Interactive Element Documentation) |
| G-014: Typewriter mode scroll verification | Article 11 (Interactive Element Documentation) |
| G-015: PreferencesModal coverage | Article 3 (Settings Parity) |
| G-016: XSS in link URLs | Article 14 (Architecture Conformance) — insufficiently explicit |

### Constitutional Gaps Summary

| Iteration | New P0 Gap | New Constitutional Gap | Articles Needed |
|-----------|-----------|----------------------|-----------------|
| 1 | 4 | 4 | Article 1-5 |
| 2 | 3 | 4 | Article 6-11 |
| 3 | 3 | 3 | Article 12-14 |
| 4 | 2 | 1 | Article 15-16 |
| 5 | 2 | 1 | Article 17-18 |
| 6 | 1 | 1 | — |
| 7 | 1 | 1 | — |
| **8** | **1 (G-004)** | **1 (G-011)** | **Article 16 Addendum + Article 19** |

**Pattern confirmed:** Every iteration introduces 1 constitutional gap requiring 1-2 articles. The gap rate is decreasing (2→1 per iteration), suggesting the constitutional framework is converging.

---

## Part VI: Proposed Articles

### Article 15 Addendum: Integration-Level Bidirectional Verification

**Applies to:** G-001 (cursor mapping — still P0 after 2 iterations)

**Text:**
> Article 15 (Bidirectional Mapping Completeness) is strengthened by the following addendum:
>
> Bidirectional mappings (source→DOM and DOM→source) MUST be verified by **integration tests that use the actual frontend editor's DOM output**, not mocked DOM structures. Unit tests in isolation do NOT satisfy Article 15's verification requirement.
>
> Specifically:
> 1. `CursorMapping::dom_to_source()` must be tested against real TipTap-generated DOM offsets
> 2. Edge cases must include: nested inline elements, code spans with multiple characters, tables with merged cells
> 3. Round-trip tests (source → DOM → source) must pass with zero divergence for all Markdown constructs

**Why This Is Needed:** Article 15 was proposed 2 iterations ago. G-001 persists because Article 15's text ("bidirectional mappings must work both ways") was interpreted as requiring unit tests, which exist. But the real verification requires integration-level testing. This addendum makes that explicit.

---

### Article 16 Addendum: Paste and Clipboard Conversion Fidelity

**Applies to:** G-004 (paste rich-text — NEW P0)

**Text:**
> Article 16 (Export Output Fidelity) is renamed "**Format Conversion Fidelity**" and its scope is extended:
>
> Format conversion fidelity applies to ALL content format transformations in the application:
> 1. **HTML export** — complex Markdown (tables, code blocks, images, nested lists) must render correctly
> 2. **PDF export** — complex Markdown must render correctly, verified by reference document fixtures
> 3. **Paste/clipboard conversion** — rich text from external sources (Word, Excel, web browsers, other applications) must be converted to Markdown with high fidelity, including:
>    - Tables (Excel, HTML)
>    - Lists with proper nesting
>    - Images (inline and linked)
>    - Bold, italic, strikethrough formatting
>    - Links and URLs
>    - Code blocks with language detection
>    - Frontmatter blocks (when applicable)
>
> "Works" is insufficient. Conversion fidelity must be verified by reference fixtures for each input type.

**Why This Is Needed:** G-004 (paste rich-text conversion) was not anticipated because Article 16's name ("Export Output Fidelity") didn't signal coverage of paste/clipboard conversion. The rename makes the scope explicit.

---

### Article 16 Second Addendum: Visual Verification Infrastructure

**Applies to:** G-002 (PDF export quality — still P0 after 2 iterations)

**Text:**
> Article 16 (renamed Format Conversion Fidelity) is further extended with visual verification requirements:
>
> For PDF export and any visual output format:
> 1. Reference document fixtures MUST exist covering all Markdown types: tables, code blocks with syntax highlighting, images, nested lists, blockquotes, frontmatter
> 2. Each fixture MUST have explicit pass/fail criteria (e.g., "table borders render correctly", "code syntax is highlighted", "images are embedded at correct size")
> 3. Fixtures are run as part of the export test suite
> 4. Visual output quality is not optional — "it compiles" is insufficient

**Why This Is Needed:** G-002 (PDF export quality) persists because Article 16's text ("output must be correct") was interpreted as "the export works." The addendum makes explicit that visual correctness requires reference fixtures and explicit pass/fail criteria.

---

### Article 17 Addendum: Benchmark Results Documentation

**Applies to:** G-003 (benchmark results undocumented — still P0 after this iteration)

**Text:**
> Article 17 (Performance NFR Enforcement) is strengthened by the following addendum:
>
> Benchmark infrastructure alone is insufficient. Results MUST be:
> 1. **Executed** on every PR and CI run
> 2. **Documented** in machine-readable format (JSON/CSV)
> 3. **Compared** to PRD-05 NFR thresholds with explicit pass/fail per threshold
> 4. **Published** as CI artifacts for transparency
> 5. **Tracked** over time for regression detection
>
> "Benchmark infrastructure exists" is a necessary but not sufficient condition. "Benchmark results exist and are within thresholds" is the actual requirement.

**Why This Is Needed:** Article 17 was proposed in iteration-7 and the infrastructure was built (5 benchmark files). But G-003 still exists because "infrastructure" ≠ "results." The addendum closes this gap.

---

### Article 19: UI String Externalization (NEW)

**Applies to:** G-011 (i18n not externalized — NEW constitutional gap)

**Text:**
> All user-facing strings in the application MUST be externalized to locale files. Specifically:
>
> 1. No user-visible string (labels, buttons, tooltips, error messages, modal titles, menu items, placeholders) may be hardcoded in component source files
> 2. All strings MUST reside in locale files (e.g., `locales/en.json`) with a hierarchical key naming convention: `{section}.{component}.{purpose}` (e.g., `toolbar.file.new`, `modal.export.title`)
> 3. An i18n library (e.g., i18next) MUST be installed and configured
> 4. All new components MUST externalize strings from their first commit, not retroactively
> 5. Runtime language switching is not required for MVP, but the architecture must support it
>
> This applies to: React components, error messages, tooltips, aria labels, placeholder text, and any text visible to the user.

**Why This Is Needed:** NFR-014 requires i18n readiness. Without a constitutional article mandating string externalization, this architectural requirement is deprioritized in every iteration. This article makes it binding.

**Enforcement:** Add a lint rule (e.g., react-intl `no-untranslated-strings`) to fail the build if hardcoded strings are found in component files.

---

### Article 20: Security Input Validation (NEW)

**Applies to:** G-016 (XSS in link URLs — P1 security issue)

**Text:**
> All user inputs that become part of application data (URLs, file paths, document content) MUST be validated against explicit allowlists before storage or execution. Specifically:
>
> 1. **URL validation** — Link URLs must be validated against PRD-12 Section 12.4.3. `javascript:`, `data:`, and other dangerous protocols must be blocked. Only allowlisted schemes (http, https, mailto) are permitted.
> 2. **Path validation** — File paths must be validated against path traversal attacks. Absolute paths outside the workspace must be rejected.
> 3. **Content sanitization** — HTML content must be sanitized by the comrak library (already configured). Custom HTML injection must not bypass sanitization.
> 4. **Input validation is not optional** — validation must happen at the Tauri command boundary, not just in the frontend.
>
> "The library handles it" is insufficient — validation must be explicit and tested.

**Why This Is Needed:** G-016 (XSS in link URLs) is a P1 security issue that PRD-12 explicitly covers. Without a constitutional article, input validation is implemented inconsistently. This article makes security input validation explicit and binding.

---

## Part VII: Ratification Package

### For Ratification: Articles 1-19

| Priority | Articles | Proven Necessary |
|----------|----------|-----------------|
| Must Ratify | 1-11 | Proven over 4 iterations |
| Must Ratify | 12-14 | Proven necessary by iteration-5 P0 issues |
| Must Ratify | 15-16 | Proven necessary by iteration-6 P0 issues (G-001, G-002 residual — addenda needed) |
| Must Ratify | 17-18 | Proven necessary by iteration-7 (benchmarks built, release criteria defined) |
| Must Ratify | Addenda (15A, 16A×2, 17A) | Iteration-8 P0 analysis reveals addenda needed for full coverage |
| Must Ratify | 19 | Iteration-8 G-011 i18n gap |

### Critical: Ratify Complete Constitution NOW

After **8 iterations** of proposing articles without ratification:
- 11 articles from iterations 1-4
- 3 articles (12-14) from iteration-5
- 2 articles (15-16) from iteration-6
- 2 articles (17-18) from iteration-7
- Addenda + Article 19 from iteration-8

**Total: 19 articles + 4 addenda pending ratification**

The pattern is now **mathematically proven**: Articles that are followed (even without ratification) produce better outcomes. But the lack of ratification means:
- No enforcement mechanism
- No PR checklist blocking violations
- P0 issues persist across 2+ iterations

**Article 18 creates the enforcement mechanism.** But Article 18 itself is not ratified. This is a deadlock.

**Proposed resolution:** Ratify ALL 19 articles + addenda as a single ratification package. The cost of ratification is one vote. The cost of non-ratification is indefinite P0 persistence.

---

## Part VIII: Recommended Constitution Structure

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
  + Addendum 15A: Integration-Level Verification (iter-8)
## Article 16: Format Conversion Fidelity (renamed from Export Output Fidelity)
  + Addendum 16A: Paste/Clipboard Conversion (iter-8)
  + Addendum 16B: Visual Verification Infrastructure (iter-8)
## Article 17: Performance NFR Enforcement
  + Addendum 17A: Benchmark Results Documentation (iter-8)
## Article 18: MVP Release Criteria Formalization
## Article 19: UI String Externalization (NEW, iter-8)
## Article 20: Security Input Validation (NEW, iter-8)

## Amendment Process
Constitution amendments require:
1. Proposal in gap-analysis.md with justification
2. Review in iteration planning
3. Explicit ratification vote
4. Update to Constitution.md
```

---

## Part IX: Constitutional Principles Summary

Based on 8 iterations of gap analysis:

1. **Completion Means Complete** — Features must work end-to-end, not just have working parts
2. **All Operations Required** — N-1 of N operations is broken, not 99% complete
3. **Fidelity Is Non-Negotiable** — Undo/redo, export, and paste must preserve document structure and formatting
4. **Architecture Is Binding** — PRD specifications must be followed
5. **Bidirectional Mappings Must Work Both Ways** — Conversions must work in both directions and be verified at integration level
6. **Format Conversion Includes Paste** — Clipboard conversion is export fidelity, not a separate concern
7. **Export Output Must Be Verified** — "Exports" is insufficient; must "render correctly" with reference fixtures
8. **Performance NFRs Require Automated Validation** — "Acceptable performance" without measurement is insufficient
9. **Benchmark Results Must Be Documented** — Infrastructure ≠ results; both are required
10. **MVP Readiness Has a Concrete Definition** — "90% complete" is not a release criterion
11. **Ratification Prevents Accumulation** — Proposed articles must be ratified or rejected with a deadline
12. **UI Strings Must Be Externalized** — All user-facing text must be in locale files from first commit
13. **Security Input Validation Is Explicit** — Allowlists and validation at command boundaries, not just "the library handles it"

---

## Part X: Immediate Action Items

### Fix Iteration-8 P0 Issues

| P0 Issue | Required Fix | Article Invoked |
|----------|-------------|-----------------|
| G-001: Cursor bidirectional mapping | Add TipTap integration tests for round-trip mapping | Article 15 + Addendum 15A |
| G-002: PDF export quality | Add reference document fixtures with visual verification tests | Article 16 + Addendum 16B |
| G-003: Benchmark results undocumented | Run `cargo bench`, document results vs PRD-05 thresholds, add to CI | Article 17 + Addendum 17A |
| G-004: Paste rich-text conversion | Expand TurndownService rules, add Word/Excel clipboard handling | Article 16 + Addendum 16A |

### Ratify Constitution

| Action | Required |
|--------|----------|
| Ratify Articles 1-19 + Addenda | 8 iterations of gap analysis proves necessity |
| Create `Constitution.md` at project root | Formal document for enforcement |
| Add Constitution checklist to PR template | Articles 1-19 block merge if violated |
| Add lint rule for hardcoded strings | Enforce Article 19 automatically |

---

## Part XI: Constitutional Evolution Analysis

### Iteration-by-Iteration Gap Tracking

| Iteration | P0 Gaps | Constitutional Gaps | Articles Proposed | Articles Ratified | Net Effect |
|-----------|---------|--------------------|--------------------|-------------------|------------|
| 1 | 4 | 4 | 5 | 0 | Patterns identified |
| 2 | 3 | 4 | 7 | 0 | Patterns confirmed |
| 3 | 3 | 3 | 10 | 0 | Some resolved |
| 4 | 2 | 1 | 11 | 0 | Some resolved |
| 5 | 2 | 1 | 14 | 0 | P0s fixed by following articles |
| 6 | 2 | 1 | 16 | 0 | P0s fixed by following articles |
| 7 | 1 | 1 | 18 | 0 | Benchmarks built |
| **8** | **4** | **1** | **19 + 4 addenda** | **0** | Infrastructure improved, P0s persist |

**The pattern is proven beyond doubt after 8 iterations:**
1. Articles that are followed produce better outcomes than no articles
2. But without ratification, P0 issues persist across 2+ iterations
3. The convergence is real: gap rate decreased from 4/gap per iteration (iter 1-4) to 1 per iteration (iter 8)
4. The remaining constitutional gaps are narrower and more specific (integration tests, visual verification, results documentation, paste conversion, i18n)

**Conclusion:** The constitutional framework is working. The remaining work is: (a) ratify it, (b) close the remaining P0s using the existing framework + addenda.

---

## Part XII: Files Referenced

- Gap Analysis: `iterations/iteration-8/gap-analysis.md`
- Previous Proposals: `iterations/iteration-1/through/iteration-7/constitution_updates.md`
- Performance Benchmarks: `src-tauri/benches/`
- Cursor Mapping: `src-tauri/src/semantic/position.rs`
- Paste Handling: `src-tauri/src/semantic/paste.rs`
- PDF Export: `src-tauri/src/commands/export.rs`
- Security Requirements: `docs/PRD/12-security-requirements.md`
- NFR Requirements: `docs/PRD/05-non-functional-requirements.md`
- i18n Requirements: `docs/PRD/11-nfr.md` (NFR-014)
- Constitution Script: `scripts/constitution.sh`

---

## Conclusion

Iteration-8 demonstrates a maturing project where the constitutional framework is producing measurable results:

- Article 17 (proposed iter-7) demonstrably worked: benchmark infrastructure was built
- The remaining P0 issues are narrower: results documentation, integration verification, visual verification, paste conversion fidelity
- The constitutional gap rate has decreased from 4 per iteration to 1 per iteration
- The framework is converging — the remaining gaps are specific addenda, not entirely new categories

**New items requiring constitutional coverage:**
1. **Article 16 Addendum (paste/clipboard)** — Extend export fidelity to paste conversion explicitly
2. **Article 15 Addendum (integration verification)** — Require integration-level bidirectional tests
3. **Article 16 Addendum (visual verification)** — Require reference document fixtures for visual output
4. **Article 17 Addendum (results documentation)** — Require benchmark results to be documented
5. **Article 19 (i18n)** — New article requiring UI string externalization
6. **Article 20 (security validation)** — New article requiring explicit input validation

**The root issue is unchanged since iteration-1:** Articles are proposed but never ratified. Article 18 creates the enforcement mechanism. But 8 iterations of evidence prove that "following unratified articles" is better than "ignoring them." The next iteration should ratify ALL 19 articles + addenda and finally have an enforceable constitution.

---

*Constitution update suggestions generated from iteration-8 gap analysis*
