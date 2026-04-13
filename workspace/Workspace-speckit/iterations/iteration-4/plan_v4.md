# RustNote Implementation Plan - Iteration 4 (v4)

**Project:** RustNote - Typora-like Markdown Editor  
**Version:** 4.0  
**Based on:** spec_v4.md & gap-analysis.md  
**Date:** 2026-04-13  
**Status:** MVP Development (Iteration 4)  

---

## 1. Executive Summary

Iteration-4 gap analysis shows **90-95% MVP completion**. All P0 blocking issues have been resolved. The remaining work focuses on P1 high-priority items and P2 medium-priority items that improve user experience.

**Key Achievement:** No P0 issues remain. Core MVP functionality is working.

---

## 2. Priority Classification

### P0 - Blocking Issues: **NONE** ✅
All P0 issues from previous iterations have been resolved.

### P1 - High Priority (Must Address)
| Issue | Module | Description |
|-------|--------|-------------|
| IT-001 | Frontend | Link editing behavior undefined - click opens link editor |
| IT-002 | Frontend | Code syntax highlighting not integrated with TipTap |

### P2 - Medium Priority (Should Address)
| Issue | Module | Description |
|-------|--------|-------------|
| IT-003 | Frontend | Frontmatter rendered as plain text |
| IT-004 | Backend | No ropey-based buffer for large documents |
| IT-005 | Backend | Parser migration path not started |
| IT-006 | Frontend | prosemirror-markdown bridge not implemented |
| IT-007 | Testing | No visual regression testing baseline |

### P3 - Low Priority (Nice to Have)
| Issue | Module | Description |
|-------|--------|-------------|
| IT-008 | Frontend | Table cell editing safety |
| IT-009 | Security | Fuzz testing not implemented |
| IT-010 | Frontend | IME composition handling not tested |

---

## 3. Implementation Strategy

### Phase 1: P1 Issue Resolution
1. **IT-001: Link Editing Behavior**
   - Create `LinkPopover` component for inline link editing
   - Define: click = open editor, Ctrl+click = follow URL
   - Integrate with TipTap Link extension

2. **IT-002: Code Syntax Highlighting Integration**
   - Wire `CodeBlockHighlight` component into TipTap code block nodes
   - Verify backend `highlight_code_block` command is called
   - Test syntax highlighting renders correctly

### Phase 2: P2 Issue Resolution
3. **IT-003: Frontmatter UI**
   - Create `FrontmatterBlock` component
   - Display as collapsible block with YAML syntax styling

4. **IT-007: Visual Regression Baselines**
   - Run Playwright screenshot tests
   - Establish baselines in `visual-baselines/` directory

### Phase 3: P2/P3 Post-MVP Work
5. **IT-004: Ropey Buffer** - Deferred (significant architecture change)
6. **IT-005: Parser Migration** - Deferred per PRD acknowledgment
7. **IT-006: ProseMirror Markdown Bridge** - Deferred per PRD acknowledgment

---

## 4. Success Criteria

### For Iteration-4 Completion
- [ ] IT-001: Link click opens editor popover, Ctrl+click follows URL
- [ ] IT-002: Code blocks render with syntax highlighting in TipTap
- [ ] IT-003: Frontmatter displays as collapsible styled block
- [ ] IT-007: Visual baselines established for all VIS-* test cases

### Quality Gates
- [ ] All E2E tests pass
- [ ] No new lint errors introduced
- [ ] No P0/P1 issues remaining

---

## 5. Out of Scope for Iteration-4

The following are explicitly deferred to post-MVP:
- Ropey text buffer implementation (IT-004)
- Tree-sitter parser migration (IT-005)
- ProseMirror markdown bridge (IT-006)
- Fuzz testing (IT-009)
- IME composition testing (IT-010)

---

## 6. Files to Modify

### Frontend (React)
| File | Changes |
|------|---------|
| `www/src/components/LinkPopover.jsx` | NEW - Link editor popover |
| `www/src/components/TipTapEditor.jsx` | Wire LinkPopover, CodeBlockHighlight |
| `www/src/components/CodeBlockHighlight.jsx` | Integration with TipTap |
| `www/src/components/FrontmatterBlock.jsx` | NEW - Frontmatter display |
| `www/src/App.jsx` | LinkPopover integration |

### Backend (Rust) - Minimal changes
| File | Changes |
|------|---------|
| `src-tauri/src/commands/editor.rs` | May need link/highlight commands |
| `src-tauri/src/parser/` | No changes for P1 items |

### Testing
| File | Changes |
|------|---------|
| `e2e/visual/` | Update to capture baselines |
| `www/src/components/` | Component updates as needed |

---

## 7. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| Link editing may conflict with browser default | Medium | Explicit event handling with preventDefault |
| Code highlighting performance | Low | Lazy-load highlighting, cache results |
| Frontmatter parsing edge cases | Low | Use existing YAML parsing library |

---

## 8. Iteration Checkpoint

```
iteration=4
phase=phase1
timestamp=1776012722
```
