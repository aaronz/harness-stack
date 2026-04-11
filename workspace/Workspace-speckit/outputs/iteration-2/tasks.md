# RustNote Implementation Task List — Iteration 2

**Plan Version:** 1.0  
**Generated:** 2026-04-11  
**Based On:** `iteration-2/implementation-plan.md`

---

## Phase 1: Setup (Project Initialization)

> Minimal - existing project structure already established

- [ ] T001 [P] Setup Cargo workspace with new module paths in `src-tauri/src/lib.rs`
- [ ] T002 [P] Verify `comrak = "0.52"` dependency resolves in `Cargo.toml`
- [ ] T003 [P] Create directory structure: `semantic/`, `editor/`, `renderer/`, `services/`

---

## Phase 2: Foundational (Blocking Prerequisites)

> These tasks block all user stories. Execute sequentially within phase.

### M2.1: Semantic Model Layer

- [ ] T010 [Story=M2.1] Create `semantic/mod.rs` with module exports
- [ ] T011 [Story=M2.1] Implement `semantic/ast.rs` — AST node wrapper around comrak
- [ ] T012 [Story=M2.1] Implement `semantic/position.rs` — Position/Range/Selection types
- [ ] T013 [Story=M2.1] Implement `semantic/transform.rs` — Source↔AST synchronization
- [ ] T014 [Story=M2.1] Refactor `parser/markdown.rs` to use comrak and output AST
- [ ] T015 [Story=M2.1] Integrate `parser/syntax.rs` with syntect for code highlighting

### M2.2: Editor Engine (Blocked by M2.1)

- [ ] T020 [Story=M2.2] Create `editor/mod.rs` with module exports
- [ ] T021 [Story=M2.2] Implement `editor/cursor.rs` — Cursor movement rules
- [ ] T022 [Story=M2.2] Implement `editor/selection.rs` — Selection handling
- [ ] T023 [Story=M2.2] Implement `editor/commands.rs` — Command types (Insert, Delete, Replace, Format)
- [ ] T024 [Story=M2.2] Implement `editor/undo.rs` — Undo/redo history system

### M2.4: Services Layer (Blocked by M2.1, runs parallel to M2.2)

- [ ] T030 [Story=M2.4] Restructure `services/mod.rs` for new architecture
- [ ] T031 [Story=M2.4] Implement `services/document.rs` — DocumentService (create, open, save, get_content, apply, get_ast, update_source)
- [ ] T032 [Story=M2.4] Implement `services/editor.rs` — EditorService (cursor, selection, transform, undo, redo)

---

## Phase 3: User Stories — Milestone 2 (Live Rendering Foundation)

### US1: View Rendered Markdown

> **Story:** As a user, I want to see live-rendered markdown so I can write without mode-switching.

- [ ] T040 [P] [Story=US1] Create `renderer/mod.rs` — Renderer module
- [ ] T041 [P] [Story=US1] Implement `renderer/state.rs` — Render state management
- [ ] T042 [P] [Story=US1] Implement `renderer/inline.rs` — Inline span rendering (emphasis, strong, code, links)
- [ ] T043 [P] [Story=US1] Implement `renderer/blocks.rs` — Block rendering (headings, lists, blockquotes, code blocks)
- [ ] T044 [Story=US1] Refactor `www/src/scripts/editor.js` — Integrate ProseMirror with Rust backend
- [ ] T045 [Story=US1] Create `www/src/styles/editor.css` — Editor-specific styles

### US2: Document Lifecycle

> **Story:** As a user, I want to create, open, and save documents so my work persists.

- [ ] T050 [Story=US2] Implement `create` command — New untitled document
- [ ] T051 [Story=US2] Implement `open` command — Open existing .md file
- [ ] T052 [Story=US2] Implement `save` command — Save with atomic write (temp + rename)
- [ ] T053 [Story=US2] Wire document commands to `editor.js` frontend

### US3: Live Sync

> **Story:** As a user, I want my edits to render instantly so I can see the result of my writing.

- [ ] T060 [Story=US3] Implement `update_source` IPC — Frontend sends edits to Rust
- [ ] T061 [Story=US3] Implement AST re-parse on source change
- [ ] T062 [Story=US3] Implement `serialize` — AST→CommonMark output
- [ ] T063 [Story=US3] Verify roundtrip fidelity (parse→serialize→parse)

---

## Phase 4: User Stories — Milestone 3 (Editing Semantics)

### US4: Smart Transforms

> **Story:** As a user, I want Enter/Backspace/Tab to behave smartly in lists and quotes so editing feels natural.

- [ ] T070 [Story=US4] Implement `editor/transforms.rs` — Transform rule engine
- [ ] T071 [Story=US4] Implement list transforms — Enter to continue/exit, Backspace to dedent
- [ ] T072 [Story=US4] Implement blockquote transforms — Enter in quote continues quote
- [ ] T073 [Story=US4] Implement heading transforms — Enter in heading creates next heading
- [ ] T074 [Story=US4] Implement task list toggle — `[ ]` ↔ `[x]` on checkbox click

### US5: Formatting Commands

> **Story:** As a user, I want to format text with keyboard shortcuts so I can write without touching the mouse.

- [ ] T080 [Story=US5] Implement bold/italic/strikethrough commands
- [ ] T081 [Story=US5] Implement inline code command
- [ ] T082 [Story=US5] Implement link insertion command
- [ ] T083 [Story=US5] Implement image insertion command

### US6: Keyboard Shortcuts

> **Story:** As a user, I want keyboard shortcuts for all actions so I can stay on the keyboard.

- [ ] T090 [Story=US6] Implement shortcut handler in `editor.js`
- [ ] T091 [Story=US6] Document all shortcuts in `www/shortcuts.md`
- [ ] T092 [Story=US6] Test all shortcuts for conflicts

### US7: Regression Tests

> **Story:** As a developer, I want automated tests for editor behaviors so I can refactor without breaking users.

- [ ] T100 [Story=US7] Create list behavior test fixtures
- [ ] T101 [Story=US7] Create cursor movement test fixtures
- [ ] T102 [Story=US7] Create selection test fixtures
- [ ] T103 [Story=US7] Create task toggle test fixtures
- [ ] T104 [Story=US7] Run full regression suite, fix failures

---

## Phase 5: Polish & Cross-Cutting Concerns

- [ ] T110 [P] Run roundtrip fidelity tests on all CommonMark + GFM constructs
- [ ] T111 [P] Verify startup time < 2s benchmark
- [ ] T112 [P] Run LSP diagnostics, fix all warnings/errors
- [ ] T113 Update `research.md` with implementation findings
- [ ] T114 Update `constitution.md` compliance check

---

## Dependency Graph

```
Phase 2 (Foundational):
  T010 ─┬─→ T011 → T012 → T013 → T014 → T015 (M2.1)
         └──────────────────────────────────┬─→ T020 → T021 → T022 → T023 → T024 (M2.2)
                                             └─→ T030 → T031 → T032 (M2.4)

Phase 3 (US1-US3, depends on Phase 2):
  T020 ──────────────────┬─→ T040 → T041 → T042 → T043 → T044 → T045 (US1)
                        └─→ T050 → T051 → T052 → T053 (US2)
                        └─→ T060 → T061 → T062 → T063 (US3)

Phase 4 (US4-US7, depends on Phase 3 + M2.2 + M2.3):
  T021,T022,T023,T024 ──┬─→ T070 → T071 → T072 → T073 → T074 (US4)
  T044                  ├─→ T080 → T081 → T082 → T083 (US5)
  T044                  ├─→ T090 → T091 → T092 (US6)
                        └─→ T100 → T101 → T102 → T103 → T104 (US7)

Phase 5 (Polish):
  T063, T074, T083, T092, T104 ─┬─→ T110 → T111 → T112 → T113 → T114
```

---

## Effort Summary

| Phase | Tasks | Estimated |
|-------|-------|----------|
| Phase 1: Setup | 3 | ~1h |
| Phase 2: Foundational | 13 | ~66h |
| Phase 3: M2 Live Rendering | 16 | ~32h |
| Phase 4: M3 Editing Semantics | 17 | ~60h |
| Phase 5: Polish | 5 | ~4h |
| **Total** | **54** | **~163h** |

---

**Task List Created:** 2026-04-11  
**Next Action:** Begin Phase 2, Task T010 (Create semantic module)
