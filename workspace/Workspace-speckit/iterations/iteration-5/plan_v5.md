# RustNote Implementation Plan - Iteration 5 (v5)

**Project:** RustNote - Typora-like Markdown Editor
**Version:** 5.0
**Based on:** spec_v5.md & gap-analysis.md
**Date:** 2026-04-13
**Status:** MVP Development (Iteration 5)

---

## 1. Executive Summary

Iteration-5 gap analysis reveals **3 P0 blocking issues** that were NOT adequately addressed in iteration-4. The iteration-4 claim of "90-95% MVP completion" was overstated. Core issues include incomplete TransformEngine implementation, undo/redo fidelity, and settings architecture.

**Key Finding:** MVP is approximately **75% complete** by functional requirements. P0 blocking issues must be resolved before MVP can be considered complete.

---

## 2. Priority Classification

### P0 - Blocking Issues (Must Fix - 3 Issues)

| Gap | Module | Description | Fix Required |
|-----|--------|-------------|--------------|
| G-001 | Editor | `TransformType::EnterInListItem`, `EnterInBlockQuote`, `EnterInHeading` not implemented in Rust `TransformEngine` | Complete Rust `TransformEngine::apply_*` methods for all transform types |
| G-002 | Editor | Undo/redo implementation has fidelity issues with structural edits - no regression tests | Add comprehensive undo/redo tests; verify transaction log fidelity |
| G-003 | Settings | Settings persistence uses file-based JSON instead of rusqlite as specified in PRD-10 | Replace with rusqlite-based settings service |

### P1 - High Priority (Must Address - 6 Issues)

| Gap | Module | Description |
|-----|--------|-------------|
| G-004 | Editor | Cursor mapping between TipTap DOM and source Markdown is incomplete |
| G-005 | Export | PDF export uses basic text extraction instead of proper HTML-to-PDF pipeline |
| G-006 | Parser | Parser uses `comrak` directly instead of tree-sitter for incremental parsing |
| G-007 | Buffer | `buffer` module exists but doesn't use `ropey` for efficient large document handling |
| G-008 | Editor | `Transform::Wrap` (wrapping selected text) not implemented |
| G-009 | Image | Image paste/drop relative path handling may not work correctly for subdirectory documents |

### P2 - Medium Priority (Should Address - 9 Issues)

| Gap | Module | Description |
|-----|--------|-------------|
| G-010 | Editor | Table editing uses TipTap default behavior - constrained but safe model |
| G-011 | Export | HTML export doesn't support "linked-assets mode" as per FR-031 |
| G-012 | Parser | Frontmatter parsing doesn't handle all YAML frontmatter variations |
| G-013 | Display | Focus mode implementation is basic CSS - should materially reduce distraction |
| G-014 | Display | Typewriter mode may not keep cursor at vertical center during navigation |
| G-015 | Editor | Paste behavior doesn't handle all rich text paste cases |
| G-016 | Services | Architecture doesn't expose `DocumentService`, `EditorService`, etc. per PRD-10 |
| G-017 | Recovery | Autosave uses frontend timer - not guaranteed to fire if app crashes |
| G-018 | Editor | Code fence syntax highlighting uses syntect but Shiki is specified in PRD |

### P3 - Low Priority / Deferred

| Gap | Module | Description |
|-----|--------|-------------|
| G-019 | Editor | Table cell editing safety - PRD acceptable, monitor only |
| G-020 | Security | Fuzz testing not implemented - deferred post-MVP |
| G-021 | Frontend | IME composition handling not tested - test post-MVP |

---

## 3. Implementation Strategy

### Phase 1: P0 Issue Resolution (BLOCKING - Must Complete First)

**Critical:** All P0 issues must be fully resolved before claiming MVP completion.

#### G-001: Complete TransformEngine Implementation

1. Review `TransformType` enum in `commands/editor.rs`
2. Implement `TransformEngine::apply_enter_in_list_item()` - Continue list, exit on empty
3. Implement `TransformEngine::apply_enter_in_blockquote()` - Continue quote, exit on empty
4. Implement `TransformEngine::apply_enter_in_heading()` - Create new heading or paragraph
5. Implement `TransformEngine::apply_wrap()` - Wrap selection with markers
6. Update frontend to use implemented transforms
7. Add unit tests for all transform types

#### G-002: Fix Undo/Redo Fidelity

1. Analyze current undo/redo implementation in TipTap
2. Identify structural edit handling issues
3. Add comprehensive round-trip tests (do → undo → do → undo = original)
4. Test structural edits: list manipulation, heading changes, blockquote conversion
5. Fix any fidelity issues found
6. Ensure transaction log maintains correct state

#### G-003: Switch to rusqlite Settings

1. Design rusqlite schema for settings (per PRD-10)
2. Implement `SettingsService` with rusqlite backend
3. Migrate existing settings.json data
4. Update `read_settings` and `write_settings` commands
5. Add atomic transaction support
6. Test crash recovery during settings write

### Phase 2: P1 Issue Resolution

#### G-004: Cursor Mapping
- Implement bidirectional cursor mapping between TipTap DOM and source
- Use AST-aware offset calculation to account for HTML tag insertion

#### G-005: PDF Export Quality
- Integrate proper HTML-to-PDF rendering (html2pdf, puppeteer, or webview print)
- Ensure code blocks and images render correctly

#### G-006: Tree-sitter Parser
- Implement tree-sitter-based incremental parsing wrapper around comrak

#### G-007: Ropey Buffer
- The buffer module exists but doesn't use ropey per PRD specification
- Implement ropey-based text buffer for efficient large document handling

#### G-008: Wrap Transform
- Add Wrap transform for text selection wrapping

#### G-009: Image Path Handling
- Verify and fix relative path calculation based on document location

### Phase 3: P2 Issue Resolution

1. **G-010:** Table editing - Document constraints, ensure no data loss
2. **G-011:** HTML export modes - Add linked vs inline asset export option
3. **G-012:** Frontmatter parsing - Handle complex YAML variations
4. **G-013:** Focus mode polish - Properly dim/hide non-current paragraphs
5. **G-014:** Typewriter mode - Fix scroll behavior to keep cursor centered
6. **G-015:** Paste handling - Improve rich text paste conversion
7. **G-016:** Service interfaces - Refactor to expose proper service interfaces per PRD-10
8. **G-017:** Autosave reliability - Consider Rust-side autosave with debounce
9. **G-018:** Syntax highlighter - Verify implementation matches PRD (Shiki vs syntect)

---

## 4. Success Criteria

### For Iteration-5 Completion (MVP Milestone)

**P0 Issues - ALL Must Be Resolved:**
- [ ] G-001: All `TransformType` variants implemented and working
  - Enter in list item creates new item or exits
  - Enter in blockquote continues or exits
  - Enter in heading creates new heading or paragraph
  - Wrap transform wraps selection with markers
- [ ] G-002: Undo/redo fidelity verified with round-trip tests
  - All structural edits undo correctly
  - No state corruption after undo/redo cycles
- [ ] G-003: Settings use rusqlite with atomic transactions
  - Settings persist across crashes
  - No data loss on abnormal termination

**Quality Gates:**
- [ ] All transform types have unit tests
- [ ] Undo/redo round-trip tests pass
- [ ] Settings service passes atomic transaction tests
- [ ] No P0 issues remaining

---

## 5. Files to Modify

### Backend (Rust) - P0 Issues

| File | Changes |
|------|---------|
| `src-tauri/src/commands/editor.rs` | Implement all TransformType variants |
| `src-tauri/src/editor/transforms.rs` | Complete transform engine methods |
| `src-tauri/src/model/settings.rs` | Add rusqlite backend |
| `src-tauri/src/commands/settings.rs` | Update to use rusqlite service |
| `src-tauri/src/lib.rs` | Register new settings service |
| `src-tauri/Cargo.toml` | Add rusqlite dependency |

### Backend (Rust) - P1 Issues

| File | Changes |
|------|---------|
| `src-tauri/src/buffer/mod.rs` | Implement ropey integration |
| `src-tauri/src/parser/` | Add tree-sitter wrapper |
| `src-tauri/src/commands/export.rs` | Improve PDF rendering |
| `src-tauri/src/semantic/position.rs` | Implement bidirectional cursor mapping |

### Frontend (React)

| File | Changes |
|------|---------|
| `www/src/components/TipTapEditor.jsx` | Wire new transforms, cursor mapping |
| `www/src/hooks/useTransforms.ts` | Update to use implemented transforms |
| `www/src/contexts/SettingsContext.jsx` | Map to new rusqlite backend |

### Testing

| File | Changes |
|------|---------|
| `tests/editor_transforms.rs` | Add comprehensive transform tests |
| `tests/buffer_settings_tests.rs` | Add undo/redo fidelity tests |
| `tests/integration_editor_tests.rs` | Add round-trip undo tests |

---

## 6. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| TransformEngine changes may break frontend assumptions | High | Test all transform scenarios end-to-end |
| rusqlite migration may lose existing settings | High | Backup settings.json before migration, verify data integrity |
| Cursor mapping changes may affect cursor position | Medium | Test cursor position preservation in various scenarios |
| P0 issues underestimated | High | Allocate sufficient time for testing and fixes |

---

## 7. Out of Scope for Iteration-5

The following are explicitly deferred to post-MVP or later iterations:
- Tree-sitter parser full implementation (G-006)
- Ropey buffer full implementation (G-007)
- Service interface refactoring (G-016)
- Fuzz testing (G-020)
- IME composition handling (G-021)

---

## 8. Gap-to-FR Mapping

| Gap | Related FRs | Description |
|-----|-------------|-------------|
| G-001 | FR-012, FR-014, FR-035, FR-036, FR-037 | TransformEngine incomplete |
| G-002 | FR-019 | Undo/redo fidelity |
| G-003 | FR-034 | Settings persistence architecture |
| G-004 | FR-008 | Cursor mapping |
| G-005 | FR-032 | PDF export quality |
| G-006 | FR-021 | Parser architecture |
| G-007 | NFR-003 | Buffer architecture |
| G-008 | FR-038 | Wrap transform |
| G-009 | FR-017 | Image path handling |

---

## 9. Iteration Checkpoint

```
iteration=5
phase=phase1
timestamp=1744569600
```

---

*Plan generated from Iteration-5 gap analysis - P0 issues prioritized*