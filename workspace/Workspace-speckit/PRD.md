# RustNote PRD

**Project:** Typora-like Markdown Editor in Rust  
**Version:** 3.1  
**Tech Stack:** Tauri v2 + React + Rust  
**License:** MIT OR Apache-2.0

---

## Quick Links

The full PRD is split into wiki-style documents in `docs/PRD/`:

| Document | Description |
|----------|-------------|
| [01-product-definition](docs/PRD/01-product-definition.md) | Executive definition, product thesis, principles |
| [02-product-invariants](docs/PRD/02-product-invariants.md) | Invariants, success definition |
| [03-scope-mvp](docs/PRD/03-scope-mvp.md) | MVP scope, acceptance criteria |
| [04-functional-requirements](docs/PRD/04-functional-requirements.md) | FR-001 to FR-034 |
| [05-architecture](docs/PRD/05-architecture.md) | Non-functional requirements, architecture |
| [06-frontend-design](docs/PRD/06-frontend-design.md) | Frontend design goals, product principles |
| [07-technical-stack](docs/PRD/07-technical-stack.md) | Engineering standards, frontend stack |
| [08-implementation-milestones](docs/PRD/08-implementation-milestones.md) | Testing strategy, milestones |
| [09-api-contracts](docs/PRD/09-api-contracts.md) | Frontend ↔ Backend IPC API |
| [10-rust-crate-design](docs/PRD/10-rust-crate-design.md) | Repository structure, crate responsibilities |
| [11-ux-requirements](docs/PRD/11-ux-requirements.md) | Accessibility, error states |
| [12-security](docs/PRD/12-security.md) | Security, threat model |
| [13-governance](docs/PRD/13-governance.md) | Release engineering, governance |
| [14-test-plan](docs/PRD/14-test-plan.md) | Comprehensive test specifications |
| [CHANGELOG](docs/PRD/CHANGELOG.md) | Version history |

---

## Core Product Definition

**RustNote** is a writing-first, WYSIWYM Markdown editor that eliminates the cognitive gap between editing Markdown source and reading formatted content, while preserving plain Markdown as the durable source of truth.

### Key Principles

1. **Write first** - The user should feel like they are writing a document, not managing syntax.
2. **Markdown is the source of truth** - Everything saved must remain valid, predictable Markdown.
3. **Single-pane live rendering** - Not split preview; users should never need to mode-switch.
4. **Rust owns correctness** - Parsing, editing semantics, serialization, recovery, and export-critical logic belong in Rust-owned boundaries.
5. **Local-first always** - No telemetry, all data stays on user's local filesystem.
6. **Calm, distraction-free** - Minimal chrome, strong typography, focus-friendly modes.

### MVP Scope

- Desktop app for macOS, Windows, Linux
- Open, edit, save `.md` files
- Single-pane live Markdown editing
- Smart editing behavior for lists, quotes, headings
- Focus mode, typewriter mode
- HTML export, PDF export
- Auto-save and crash recovery
- Theme support (light and dark)
- Workspace sidebar, outline panel
- Find/replace

### Architecture

**Production Stack:**
- Tauri v2 shell
- Tiptap/ProseMirror editor
- prosemirror-markdown bridge
- Rust core (comrak, ropey, tree-sitter, rusqlite)
- Shiki + syntect for highlighting

**Current MVP Stack:**
- Tauri v2 shell
- React 18 + contenteditable
- Rust services (pulldown-cmark)
- Tailwind CSS

### Repository

```
rustnote/
├── docs/PRD/           # Full PRD documentation (wiki style)
├── src-tauri/         # Tauri app + Rust backend
└── www/               # React frontend
```
