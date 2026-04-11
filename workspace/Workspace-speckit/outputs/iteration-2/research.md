# RustNote Research — Iteration 2

**Date:** 2026-04-11  
**Status:** Complete

---

## 1. Research Summary

### 1.1 Tauri Editor Patterns (Agent: bg_8b81e27c)

**Key Findings:**

1. **Rendering Strategy:** Hybrid approach recommended
   - Primary: ProseMirror for WYSIWYG editing (mature, robust cursor/selection)
   - Source mode: CodeMirror 6 for Markdown source editing
   - Alternative: Muya (used by MarkText)

2. **Rust-Frontend Communication:**
   - Use `invoke()` for commands returning values
   - Use events for push notifications from Rust → frontend
   - Standard Tauri IPC pattern confirmed

3. **Real-World Examples:**
   - MarkText: Uses Muya for WYSIWYG + source modes
   - Vditor: Three modes (wysiwyg/ir/sv), Typora-like live rendering
   - Milkdown: ProseMirror-based, layered architecture

**Evidence:**
- ProseMirror README: https://github.com/ProseMirror/prosemirror/blob/master/README.md
- Tauri invoke pattern: https://github.com/tauri-apps/tauri/blob/dev/examples/commands/main.rs
- MarkText Muya integration: https://github.com/marktext/marktext/blob/11c8cc1e/src/renderer/components/editorWithTabs/editor.vue

---

### 1.2 Rust Markdown Crates (Agent: bg_983a82f5)

**Key Findings:**

1. **pulldown-cmark:**
   - Event-based parser (not AST by default)
   - GFM support via feature flags: ENABLE_TABLES, ENABLE_STRIKETHROUGH, ENABLE_TASKLISTS
   - Fast, minimal, but requires custom AST layer

2. **comrak:** ⭐ RECOMMENDED
   - Full AST model (AstNode, NodeValue)
   - Parse → mutate AST → format back to HTML/CommonMark
   - Best for WYSIWYM editing due to explicit AST
   - Active maintenance (2026)

3. **markdown-rs:**
   - AST-based using mdast standard
   - Cross-language MD AST compatibility
   - Different ecosystem from Rust-dominant tooling

4. **syntect Integration:**
   - mdBook uses syntect for code highlighting
   - Build-time highlighting pattern proven
   - PR examples: https://github.com/rust-lang/mdBook/pull/1494

**Decision:** Use **comrak** for AST-based editing pipeline

**Evidence:**
- comrak AST types: https://github.com/kivikakk/comrak/blob/master/src/nodes.rs
- comrak parse_document example: https://github.com/kivikakk/comrak/blob/master/examples/update-readme.rs
- pulldown-cmark options: https://github.com/raphlinus/pulldown-cmark/blob/main/pulldown_cmark/lib.rs#L646-L671

---

### 1.3 ProseMirror Patterns (Agent: bg_a4a4fc9c)

**Key Findings:**

1. **Schema Design:**
   - ProseMirror schemas define document model (nodes/marks)
   - prosemirror-markdown provides MarkdownParser/MarkdownSerializer bridge
   - Example: https://github.com/ProseMirror/prosemirror-markdown/blob/master/src/index.ts

2. **Live Rendering Pattern:**
   - Milkdown demonstrates layered approach:
     - Parser: Markdown → ProseMirror doc
     - State: ProseMirror state management
     - View: ProseMirror view for rendering
     - NodeViews: Custom rendering for complex blocks
   - Evidence: https://github.com/Milkdown/milkdown/blob/3d6590d0/packages/core/src/editor/editor.ts

3. **Transform Rules:**
   - Keymap plugin handles Enter/Backspace/Tab
   - Input rules for smart quotes, dashes
   - prosemirror-example-setup shows wiring
   - Evidence: https://github.com/ProseMirror/prosemirror-example-setup/blob/master/src/index.ts

4. **Cursor Position Mapping:**
   - ProseMirror uses Mapping concept for position tracking
   - StepMap maps positions through transforms
   - Critical for WYSIWYM cursor consistency
   - Evidence: https://github.com/ProseMirror/prosemirror-transform/blob/master/src/README.md

---

## 2. Architecture Decisions

### 2.1 Frontend Rendering Strategy

| Option | Pros | Cons | Decision |
|--------|------|------|----------|
| **ProseMirror** | Robust cursor/selection, mature ecosystem | Learning curve | ✅ **SELECTED** |
| Muya | Used by MarkText, battle-tested | Less modular | Alternative |
| CodeMirror 6 | Great source mode | Not WYSIWYG | Source mode only |
| Custom contenteditable | Full control | Cursor bugs, maintenance burden | ❌ Rejected |

**Rationale:** ProseMirror provides battle-tested editing semantics with explicit cursor/selection handling. Its Markdown bridge enables round-tripping between Markdown and ProseMirror doc.

### 2.2 Rust Markdown Parser

| Crate | AST | GFM | Roundtrip | Decision |
|-------|-----|-----|-----------|----------|
| **comrak** | ✅ Full | ✅ | ✅ format_commonmark | ✅ **SELECTED** |
| pulldown-cmark | ❌ Event stream | ✅ | ⚠️ Manual | Build-time only |
| markdown-rs | ✅ mdast | ✅ | ✅ | Alternative |

**Rationale:** comrak provides explicit AST (AstNode/NodeValue) ideal for WYSIWYM editing with parse → mutate → serialize workflow.

### 2.3 Code Highlighting

| Approach | When | Tool | Decision |
|----------|------|------|----------|
| Build-time | Render/export | syntect | ✅ **SELECTED** |
| Client-side | Live editing | highlight.js | Not needed |

**Rationale:** syntect is already integrated (see gap-analysis). Use for HTML/PDF export; live editing can use lightweight highlighting.

---

## 3. Consolidated Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Frontend (TypeScript)                   │
│  ┌─────────────────┐  ┌─────────────────────────────────┐ │
│  │  ProseMirror    │  │  MarkdownParser/MarkdownSerializer│ │
│  │  EditorView     │◄─┤  (prosemirror-markdown)          │ │
│  │  (WYSIWYG)      │  └─────────────────────────────────┘ │
│  └─────────────────┘                                       │
│           │                                                │
│           │ Tauri IPC (invoke/events)                      │
└───────────┼────────────────────────────────────────────────┘
            │
┌───────────▼────────────────────────────────────────────────┐
│                     Rust Backend                            │
│  ┌─────────────────┐  ┌─────────────────────────────────┐  │
│  │  comrak         │  │  semantic/                       │  │
│  │  (AST parsing)  │──┤  • ast.rs (Rust AST nodes)      │  │
│  │                 │  │  • position.rs (cursor mapping) │  │
│  └─────────────────┘  └─────────────────────────────────┘  │
│  ┌─────────────────┐  ┌─────────────────────────────────┐  │
│  │  syntect        │  │  editor/                         │  │
│  │  (highlighting) │  │  • cursor.rs (movement rules)    │  │
│  │                 │  │  • transforms.rs (Enter/Backspace)│  │
│  └─────────────────┘  └─────────────────────────────────┘  │
└────────────────────────────────────────────────────────────┘
```

---

## 4. Implementation Implications

### 4.1 Frontend Changes (editor.js)

**Current State:** Raw contenteditable div
**Target State:** ProseMirror-based editor

**Migration Path:**
1. Add ProseMirror via npm/CDN
2. Create schema for Markdown constructs
3. Integrate markdownParser from prosemirror-markdown
4. Wire up editor with keymap and input rules
5. Custom NodeViews for code blocks, tables

### 4.2 Backend Changes (Rust)

**Current State:** pulldown-cmark with HTML output
**Target State:** comrak with AST

**Migration Path:**
1. Replace pulldown-cmark with comrak in Cargo.toml
2. Create semantic/ module with AST types
3. Add position.rs for cursor mapping
4. Update parser/markdown.rs to use comrak

### 4.3 Integration Points

| Frontend | Backend | Protocol |
|----------|---------|----------|
| ProseMirror state | comrak AST | Invoke for parse/serialize |
| Cursor position | semantic/position | Invoke for position mapping |
| Export HTML | export service | Invoke with AST |

---

## 5. Next Steps

1. **Update Cargo dependencies:** Replace pulldown-cmark with comrak
2. **Add ProseMirror:** Include via CDN or npm in www/
3. **Create Rust semantic layer:** AST types + position mapping
4. **Wire frontend to backend:** Tauri invoke for AST operations
5. **Implement NodeViews:** Custom rendering for blocks

---

## 6. References

| Source | URL |
|--------|-----|
| ProseMirror core | https://github.com/ProseMirror/prosemirror |
| prosemirror-markdown | https://github.com/ProseMirror/prosemirror-markdown |
| Milkdown architecture | https://github.com/Milkdown/milkdown |
| comrak | https://github.com/kivikakk/comrak |
| Tauri IPC | https://github.com/tauri-apps/tauri |
| Vditor (Typora-like) | https://github.com/Vanessa219/vditor |
| syntect mdBook integration | https://github.com/rust-lang/mdBook/pull/1494 |
