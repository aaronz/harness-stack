# RustNote Implementation Plan - Iteration 6 (v6)

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 6.0
**Based on:** spec_v6.md & gap-analysis.md
**Date:** 2026-04-14
**Status:** MVP Development (Iteration 6)

---

## 1. Executive Summary

Iteration-6 gap analysis shows significant progress from Iteration-5. Major architectural improvements were completed including rusqlite settings persistence, tree-sitter incremental parsing, ropey-based buffer, and transform engine completion.

**Key Finding:** MVP is approximately **85% complete** by functional requirements. Remaining P0 blocking issues must be resolved before MVP can be considered complete.

### What Was Fixed in Iteration-6

| Area | Iteration-5 | Iteration-6 | Change |
|------|------------|-------------|--------|
| Settings Persistence | ❌ JSON file | ✅ rusqlite | **FIXED** |
| TransformEngine | ❌ Incomplete | ✅ Complete | **FIXED** |
| Tree-sitter Parser | ❌ Not implemented | ✅ Implemented | **FIXED** |
| Ropey Buffer | ❌ Not using ropey | ✅ Implemented | **FIXED** |
| Undo/Redo Tests | ❌ Missing | ✅ Added | **FIXED** |
| Cursor Mapping Tests | ❌ Missing | ✅ Added | **FIXED** |
| Focus Mode Tests | ❌ Missing | ✅ Added | **FIXED** |

---

## 2. Priority Classification

### P0 - Blocking Issues (Must Fix - 2 Issues)

| Gap | Module | Description | Fix Required |
|-----|--------|-------------|--------------|
| G-001 | Editor | Cursor mapping bidirectional conversion (DOM→source) not fully implemented - `build_cursor_mapping()` creates source-to-DOM mapping but `CursorMapping::dom_to_source()` may have edge cases | Complete bidirectional cursor mapping implementation with comprehensive tests |
| G-002 | Export | PDF export `printpdf` implementation may not properly render complex Markdown (tables, code blocks with syntax highlighting) | Verify PDF output quality and integrate proper HTML-to-PDF pipeline |

### P1 - High Priority (Must Address - 6 Issues)

| Gap | Module | Description |
|-----|--------|-------------|
| G-003 | Editor | `Transform::Wrap` with selection - the transform engine supports it but integration with TipTap editor may be incomplete |
| G-004 | Export | HTML export doesn't support "linked-assets mode" as per FR-031 |
| G-005 | Image | Image relative path handling for subdirectory documents may have edge cases |
| G-006 | Parser | tree-sitter parser uses `tree-sitter-markdown` but GFM tables and task lists may need special handling |
| G-007 | Settings | `Settings` model may have nested `editor` struct vs flat structure per PRD-09 |
| G-008 | Frontend | `Editor.jsx` and `TipTapEditor.jsx` - unclear distinction, potential duplication |

### P2 - Medium Priority (Should Address - 9 Issues)

| Gap | Module | Description |
|-----|--------|-------------|
| G-009 | Editor | Table editing uses TipTap default behavior - constrained but safe model |
| G-010 | Display | Focus mode implementation needs verification - tests exist but actual visual dimming behavior in browser |
| G-011 | Display | Typewriter mode scroll behavior may not keep cursor at vertical center during navigation |
| G-012 | Editor | Paste behavior converts to Markdown on best-effort but doesn't handle all rich text paste cases |
| G-013 | Frontend | Preferences UI is limited - no dedicated settings panel beyond toolbar buttons |
| G-014 | Services | Architecture has services but service interfaces may need verification |
| G-015 | Autosave | Autosave uses frontend timer + Rust service - verify crash recovery still works if app crashes before timer fires |

---

## 3. Implementation Strategy

### Phase 1: P0 Issue Resolution (BLOCKING - Must Complete First)

**Critical:** All P0 issues must be fully resolved before claiming MVP completion.

#### G-001: Complete Cursor Mapping Bidirectional Conversion

1. Review `CursorMapping` implementation in `semantic/position.rs`
2. Analyze `build_cursor_mapping()` source-to-DOM mapping logic
3. Review `dom_to_source()` implementation for edge cases
4. Add comprehensive edge case tests:
   - Nested formatting (bold inside italic, code inside link)
   - Multi-byte characters (Unicode)
   - CRLF vs LF line endings
   - HTML entity encoding (&amp;, &lt;, &gt;)
   - Empty nodes and zero-width nodes
5. Fix any edge case failures
6. Verify round-trip accuracy (source → DOM → source)

#### G-002: Verify and Fix PDF Export Quality

1. Test current PDF export with complex Markdown structures:
   - Tables with borders and alignment
   - Code blocks with syntax highlighting
   - Images with various sizes
   - Nested blockquotes
   - Task lists with checkboxes
2. Compare output to HTML rendering fidelity
3. If quality issues found, integrate proper HTML-to-PDF pipeline:
   - Option A: webview print (native)
   - Option B: html2pdf or similar crate
   - Option C: temp HTML file + system print dialog
4. Verify code block syntax highlighting in PDF
5. Verify image rendering in PDF

### Phase 2: P1 Issue Resolution

#### G-003: Wrap Transform Integration Verification
- Verify wrap transform works end-to-end with TipTap selection
- Test with various markers: bold, italic, code, link

#### G-004: HTML Export Linked-Assets Mode
- Implement linked vs inline asset export option per FR-031

#### G-005: Image Path Testing
- Add tests for relative path handling in subdirectory documents
- Verify edge cases: URL-encoded paths, cross-platform

#### G-006: GFM Parser Verification
- Verify tree-sitter correctly parses GFM tables and task lists
- Add tests for GFM extension nodes

#### G-007: Settings Schema Verification
- Verify Settings model matches PRD-09 flat structure
- Check for nested `editor` struct vs flat structure

#### G-008: Editor Consolidation
- Clarify relationship between `Editor.jsx` and `TipTapEditor.jsx`
- Consolidate or document the distinction

### Phase 3: P2 Issue Resolution

1. **G-009:** Document table editing constraints
2. **G-010:** Verify CSS-based focus mode properly dims non-current paragraphs
3. **G-011:** Verify scroll behavior and fix if needed
4. **G-012:** Improve rich text paste conversion
5. **G-013:** Consider adding dedicated settings panel
6. **G-014:** Verify service trait implementations match PRD-10
7. **G-015:** Consider Rust-side autosave with debounce as backup

---

## 4. Success Criteria

### For Iteration-6 Completion (MVP Milestone)

**P0 Issues - ALL Must Be Resolved:**
- [ ] G-001: Cursor mapping bidirectional conversion works for all edge cases
  - dom_to_source() handles nested formatting
  - dom_to_source() handles Unicode correctly
  - dom_to_source() handles CRLF/LF correctly
  - dom_to_source() handles HTML entities
  - Round-trip accuracy verified
- [ ] G-002: PDF export quality verified
  - Tables render correctly with borders
  - Code blocks have syntax highlighting
  - Images render at correct sizes
  - Multi-page documents work correctly

**Quality Gates:**
- [ ] All P0 edge case tests pass
- [ ] PDF visual verification completed
- [ ] No P0 issues remaining

---

## 5. Files to Modify

### Backend (Rust) - P0 Issues

| File | Changes |
|------|---------|
| `src-tauri/src/semantic/position.rs` | Complete bidirectional cursor mapping |
| `src-tauri/src/commands/export.rs` | Improve PDF rendering pipeline |
| `src-tauri/src/buffer/mod.rs` | Ensure ropey integration complete |

### Backend (Rust) - P1 Issues

| File | Changes |
|------|---------|
| `src-tauri/src/commands/editor.rs` | Verify wrap transform integration |
| `src-tauri/src/commands/export.rs` | Add linked-assets mode |
| `src-tauri/src/parser/tree_sitter.rs` | Verify GFM parsing |
| `src-tauri/src/model/settings.rs` | Verify flat structure |
| `src-tauri/src/services/settings.rs` | Verify trait implementation |

### Frontend (React)

| File | Changes |
|------|---------|
| `www/src/components/TipTapEditor.jsx` | Verify wrap transform, cursor mapping |
| `www/src/components/Editor.jsx` | Clarify relationship with TipTapEditor |
| `www/src/components/ExportModal.jsx` | Add linked-assets option |

### Testing

| File | Changes |
|------|---------|
| `tests/cursor_mapping_tests.rs` | Add more edge case tests |
| `tests/pdf_export_tests.rs` | Add visual verification tests |
| `tests/tree_sitter_parser_tests.rs` | Add GFM tests |

---

## 6. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Cursor mapping edge cases may be complex | Medium | Test thoroughly with real documents |
| PDF rendering libraries may have limitations | Medium | Evaluate multiple options before implementation |
| GFM parser verification may reveal issues | Low | Add tests and fix if found |

---

## 7. Out of Scope for Iteration-6

The following are explicitly deferred to post-MVP or later iterations:
- Full service interface refactoring (G-014)
- Fuzz testing (post-MVP)
- IME composition handling (post-MVP)
- Visual regression testing baseline update (TD-004)

---

## 8. Gap-to-FR Mapping

| Gap | Related FRs | Description |
|-----|-------------|-------------|
| G-001 | FR-008 | Cursor mapping bidirectional conversion |
| G-002 | FR-032 | PDF export quality |
| G-003 | FR-038 | Wrap transform with selection |
| G-004 | FR-031 | HTML export linked-assets mode |
| G-005 | FR-017 | Image relative path handling |
| G-006 | FR-021 | tree-sitter GFM parsing |
| G-007 | FR-034 | Settings schema verification |
| G-008 | FR-008 | Editor.jsx/TipTapEditor.jsx consolidation |

---

## 9. Iteration Checkpoint

```
iteration=6
phase=phase1
timestamp=1744617600
```

---

*Plan generated from Iteration-6 gap analysis - P0 issues prioritized*