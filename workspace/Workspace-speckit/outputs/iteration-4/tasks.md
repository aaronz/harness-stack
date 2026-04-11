# RustNote Iteration 4 — Task List

**Generated from:** Implementation Plan v1.0  
**Date:** 2026-04-11  
**Total Tasks:** 29  
**Estimated Effort:** 146h

---

## Phase 1: Setup

- [ ] T001 [P0] [Setup] Initialize iteration-4 development branch  
  **File:** N/A  
  **Description:** Create branch from current state, set up development environment  
  **Effort:** 2h  
  **Dependencies:** None

---

## Phase 2: Foundational Backend (Prerequisites)

- [ ] T002 [P0] [Backend] Implement render_for_editor command with cursor mapping  
  **File:** `src-tauri/src/commands/render.rs`  
  **Description:** Add `render_for_editor(markdown, cursor_offset)` returning EditorRenderResult with html, cursor_mapping, active_paragraph, headings  
  **Effort:** 8h  
  **Dependencies:** None  
  **Key Types:** `EditorRenderResult`, `CursorMapping`

- [ ] T003 [P0] [Backend] Add CursorMapping types and offset-to-DOM position methods  
  **File:** `src-tauri/src/semantic/position.rs`  
  **Description:** Add CursorMapping struct with source_offset, dom_offset, line, column. Implement source offset to DOM position mapping  
  **Effort:** 8h  
  **Dependencies:** None

- [ ] T004 [P0] [Backend] Implement get_paragraph_at method for active paragraph tracking  
  **File:** `src-tauri/src/semantic/ast.rs`  
  **Description:** Add `get_paragraph_at(offset) -> usize` returning paragraph index at cursor position  
  **Effort:** 4h  
  **Dependencies:** None

- [ ] T005 [P0] [Backend] Implement get_headings_with_positions for TOC extraction  
  **File:** `src-tauri/src/semantic/ast.rs`  
  **Description:** Add `get_headings_with_positions()` returning Vec<(heading_text, level, position)> for TOC panel  
  **Effort:** 4h  
  **Dependencies:** None

---

## Phase 3: Frontend Foundation

- [ ] T006 [P0] [F1] Architecture Redesign — Replace split view with single contenteditable  
  **File:** `www/src/scripts/editor.js`  
  **Description:** Replace textarea + preview div split with single contenteditable container. Remove old split-view logic. Set up editor state object with content, semanticDoc, cursorOffset, selectionRange  
  **Effort:** 16h  
  **Dependencies:** T002 (backend render command)

- [ ] T007 [P0] [F2] Decoration System — Add span-based markdown decorations  
  **File:** `www/src/scripts/editor.js`  
  **Description:** Implement span decoration approach: markdown syntax (**, __, `, #) wrapped in spans with CSS classes that hide visual rendering while maintaining source fidelity  
  **Effort:** 12h  
  **Dependencies:** T006

- [ ] T008 [P0] [F7] Live Sync — Sync on keystroke with debounce  
  **File:** `www/src/scripts/editor.js`  
  **Description:** On every keystroke, debounce 50ms then invoke update_source. Get rendered HTML + cursor mapping from backend. Apply decorations without losing cursor  
  **Effort:** 4h  
  **Dependencies:** T006

---

## Phase 4: Cursor & Rendering Integration

- [ ] T009 [P0] [F3] Cursor Position Mapping — Wire backend cursor mapping to frontend  
  **File:** `www/src/scripts/editor.js` + `src-tauri/src/commands/render.rs`  
  **Description:** Use CursorMapping from backend to translate source offsets to DOM positions. Track cursor in source coordinates, sync visual cursor position via mapping  
  **Effort:** 12h  
  **Dependencies:** T002, T003, T004, T006

- [ ] T010 [P0] [F4] Inline Rendering — Render markdown inline in contenteditable  
  **File:** `www/src/scripts/editor.js`  
  **Description:** Parse rendered HTML from backend, apply as innerHTML with decoration spans. Handle block elements (p, h1-h6, ul, ol, blockquote, pre) vs inline elements (em, strong, code, link)  
  **Effort:** 16h  
  **Dependencies:** T007, T009

- [ ] T011 [P0] [F5] Keyboard Shortcuts — Connect all shortcuts to Rust transforms  
  **File:** `www/src/scripts/editor.js`  
  **Description:** Wire Enter, Backspace, Tab, ShiftTab to Rust TransformEngine via invoke. Handle smart quotes, lists, headings shortcuts. Update content after transform  
  **Effort:** 8h  
  **Dependencies:** T009

- [ ] T012 [P0] [F6] Undo/Redo Integration — Wire undo/redo to Rust UndoManager  
  **File:** `www/src/scripts/editor.js`  
  **Description:** Connect Ctrl/Cmd+Z and Ctrl/Cmd+Shift+Z to Rust undo/redo commands. Maintain undo stack in sync with Rust backend  
  **Effort:** 4h  
  **Dependencies:** T006

- [ ] T013 [P0] [F8] Code Block Highlighting — Integrate syntect highlighting in editor  
  **File:** `www/src/scripts/editor.js` + `src-tauri/src/renderer/`  
  **Description:** When rendering code blocks (```), apply syntect-generated HTML with syntax highlighting classes. Style with theme CSS  
  **Effort:** 8h  
  **Dependencies:** T010

---

## Phase 5: Focus Mode

- [ ] T014 [P0] [FM1] Focus Mode State — Add focusMode state and toggle  
  **File:** `www/src/scripts/editor.js`  
  **Description:** Add focusMode boolean to editor state. Add toggleFocusMode() function. Listen for Ctrl/Cmd+Shift+F shortcut  
  **Effort:** 2h  
  **Dependencies:** T006

- [ ] T015 [P0] [FM2] Dim Non-Active CSS — CSS for dimmed paragraphs  
  **File:** `www/src/styles/editor.css`  
  **Description:** Add .focus-mode .paragraph:not(.active) { opacity: 0.3 } style. Add transitions for smooth dimming. Style .active paragraph normally  
  **Effort:** 4h  
  **Dependencies:** T014

- [ ] T016 [P0] [FM3] Active Paragraph Tracking — Update on cursor move  
  **File:** `www/src/scripts/editor.js`  
  **Description:** On cursor move (via T009), find paragraph at cursor position. Add .active class to current paragraph, remove from others. Use active_paragraph from backend or calculate from cursor mapping  
  **Effort:** 4h  
  **Dependencies:** T009, T014

- [ ] T017 [P0] [FM4] Toggle UI — Menu and keyboard shortcut for focus mode  
  **File:** `www/src/scripts/app.js`  
  **Description:** Add View > Focus Mode menu item. Wire Ctrl/Cmd+Shift+F to toggleFocusMode(). Show indicator when focus mode active  
  **Effort:** 2h  
  **Dependencies:** T014

---

## Phase 6: Typewriter Mode

- [ ] T018 [P0] [TM1] Typewriter Mode State — Add typewriterMode state  
  **File:** `www/src/scripts/editor.js`  
  **Description:** Add typewriterMode boolean to editor state. Add toggleTypewriterMode() function. Listen for Ctrl/Cmd+Shift+T shortcut  
  **Effort:** 2h  
  **Dependencies:** T006

- [ ] T019 [P0] [TM2] Scroll to Center — Scroll active line to vertical center on move  
  **File:** `www/src/scripts/editor.js`  
  **Description:** On cursor move, calculate position of active line. Use scrollIntoView({ block: 'center', behavior: 'smooth' }) to center. Disable temporarily when user scrolls manually  
  **Effort:** 6h  
  **Dependencies:** T018

- [ ] T020 [P0] [TM3] Smooth Scroll CSS — CSS scroll-behavior styling  
  **File:** `www/src/styles/editor.css`  
  **Description:** Add html { scroll-behavior: smooth; } for native smooth scrolling. Ensure editor container has proper overflow and height  
  **Effort:** 2h  
  **Dependencies:** T018

- [ ] T021 [P0] [TM4] Toggle UI — Menu and keyboard shortcut for typewriter mode  
  **File:** `www/src/scripts/app.js`  
  **Description:** Add View > Typewriter Mode menu item. Wire Ctrl/Cmd+Shift+T to toggleTypewriterMode(). Show indicator when typewriter mode active  
  **Effort:** 2h  
  **Dependencies:** T018

---

## Phase 7: Outline/TOC Panel

- [ ] T022 [P0] [TOC1] TOC Extraction — Get headings from backend  
  **File:** `www/src/scripts/outline.js`  
  **Description:** On document load and change, invoke backend to get headings with positions. Store heading list in state. Debounce heavy operations  
  **Effort:** 4h  
  **Dependencies:** T005

- [ ] T023 [P0] [TOC2] TOC Panel UI — Render heading list  
  **File:** `www/src/scripts/outline.js` + `www/src/styles/editor.css`  
  **Description:** Create sidebar panel with heading list. Indent by heading level. Style like typical markdown editor TOC (Notion, Typora style)  
  **Effort:** 8h  
  **Dependencies:** T022

- [ ] T024 [P0] [TOC3] Click Navigation — Scroll to heading on click  
  **File:** `www/src/scripts/outline.js`  
  **Description:** On heading click, get heading position from backend. Scroll to that position in editor. Use smooth scroll. Highlight current heading in TOC  
  **Effort:** 4h  
  **Dependencies:** T023

- [ ] T025 [P0] [TOC4] Panel Toggle — Show/hide TOC panel  
  **File:** `www/src/scripts/app.js`  
  **Description:** Add View > Outline menu item. Wire Ctrl/Cmd+Shift+O to toggle outline panel. Persist toggle state in settings  
  **Effort:** 2h  
  **Dependencies:** T022

---

## Phase 8: Integration & Polish

- [ ] T026 [P0] [Integration] End-to-end integration testing  
  **File:** `rustnote/tests/`  
  **Description:** Test full editing flow: keystroke → Rust parse → render → display → cursor sync. Verify roundtrip fidelity (edit-save-reopen)  
  **Effort:** 8h  
  **Dependencies:** T010, T011, T012

- [ ] T027 [P0] [Polish] Performance benchmarking and optimization  
  **File:** N/A  
  **Description:** Benchmark typing latency (<16ms target), cursor move latency (<50ms), mode toggle (<100ms). Optimize debounce, reduce unnecessary re-renders  
  **Effort:** 8h  
  **Dependencies:** T026

- [ ] T028 [P0] [Polish] Theme integration verification  
  **File:** `www/src/styles/theme-*.css`  
  **Description:** Verify light/dark themes work with new WYSIWYM editor. Check code highlighting, focus mode dimming, TOC panel styling  
  **Effort:** 4h  
  **Dependencies:** T010, T015, T023

- [ ] T029 [P0] [Polish] Visual QA and acceptance criteria verification  
  **File:** N/A  
  **Description:** Verify all acceptance criteria from plan section 7.1: WYSIWYM editor, cursor mapping, smart transforms, focus mode, typewriter mode, TOC, themes, undo/redo  
  **Effort:** 4h  
  **Dependencies:** T027, T028

---

## Task Summary

| Phase | Tasks | Total Effort |
|-------|-------|--------------|
| Phase 1: Setup | T001 | 2h |
| Phase 2: Foundational Backend | T002-T005 | 24h |
| Phase 3: Frontend Foundation | T006-T008 | 32h |
| Phase 4: Cursor & Rendering | T009-T013 | 48h |
| Phase 5: Focus Mode | T014-T017 | 12h |
| Phase 6: Typewriter Mode | T018-T021 | 12h |
| Phase 7: Outline/TOC | T022-T025 | 18h |
| Phase 8: Integration & Polish | T026-T029 | 24h |
| **Total** | **29 tasks** | **146h** |

---

## Dependency Graph

```
T001 (Setup)
    │
    ▼
T002 ───────────────────────────────────────────────────────┐
T003 ───────────────────────────────────────────────────────┤
T004 ───────────────────────────────────────────────────────┤
T005 ───────────────────────────────────────────────────────┤
    │                                                        │
    ▼                                                        ▼
T006 (F1) ─────────────────────────────────────────────────┐ │
    │ ──▶ T007 (F2) ──▶ T010 (F4) ──▶ T013 (F8)           │ │
    │                                     ▲                │ │
    │                                     │                │ │
T008 (F7)                                 │                │ │
    │                                    T009 (F3) ──▶ T011 (F5) ──▶ T012 (F6)
    │                                       ▲               │
    │                                       │                │
    ├───────────────────────────────────────┼────────────────┤
    │                                       │                │
T014 (FM1) ──▶ T015 (FM2) ──▶ T016 (FM3) ──▶ T017 (FM4)     │
    │                                                       │
T018 (TM1) ──▶ T019 (TM2) ──▶ T020 (TM3) ──▶ T021 (TM4)     │
    │                                                       │
T022 (TOC1) ──▶ T023 (TOC2) ──▶ T024 (TOC3) ──▶ T025 (TOC4) │
    │                                                       │
    └───────────────────────────────────────────────────────┘
                              │
                              ▼
                    T026 (Integration)
                              │
                              ▼
                    T027 (Performance)
                              │
                              ▼
                    T028 (Theme Polish)
                              │
                              ▼
                    T029 (Visual QA)
```

---

## Parallelization Opportunities

| Tasks | Can Run In Parallel | Reason |
|-------|---------------------|--------|
| T002-T005 | YES | Independent backend additions |
| T014-T017 | YES | Focus mode components independent after T006 |
| T018-T021 | YES | Typewriter mode components independent after T006 |
| T022-T025 | YES | TOC components independent after T005 |
| T026-T029 | NO | Must follow sequential integration |

---

## Priority Markers

- **[P0]** — Critical path, must complete
- **[P1]** — Important but may be parallelized

All tasks marked P0 as all are required for iteration 4 acceptance criteria.

---

**Last Updated:** 2026-04-11  
**Task List Version:** 1.0
