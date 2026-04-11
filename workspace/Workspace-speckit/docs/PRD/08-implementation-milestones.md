# Testing Strategy

## 21. Testing Strategy

### 20.1 Unit Tests

Cover:
* parser behavior
* cursor movement
* selection logic
* editing commands
* undo/redo
* serializer
* settings and recovery

### 20.2 Editing Invariant Regression Tests

This is a key Typora-like requirement.

Maintain fixture-based tests for scenarios such as:
* Enter on list item
* Enter on empty list item
* Backspace at heading/list/quote boundaries
* link text edits
* cursor motion across inline code and emphasis
* toggling task lists
* table edit edge cases

### 20.3 Fixture Tests

Use real Markdown fixtures for:
* parse expectations
* roundtrip save behavior
* export regressions
* tricky documents from real-world authoring

### 20.4 Snapshot Tests

Useful for:
* HTML export
* semantic tree output
* theme token outputs

### 20.5 Integration Tests

Test:
* open/edit/save flows
* open folder and navigate files
* external file changes
* recovery flows
* export command flows

### 20.6 Performance Benchmarks

Track:
* startup time
* parse time
* edit latency
* serialization time
* export time

---

## 21.7 Testing Coverage Matrix

| Feature | Unit | Integration | E2E | Visual | Owner |
|---------|------|------------|-----|--------|-------|
| **File Operations** ||||||
| New/Open/Save | ✅ Rust parser | ✅ Tauri IPC | ✅ | ❌ | @rust-team |
| Workspace tree | ✅ FS ops | ✅ | ✅ | ❌ | @rust-team |
| Recent files | ✅ Settings | ✅ | ✅ | ❌ | @rust-team |
| **Editor** ||||||
| Markdown parsing | ✅ comrak/pulldown | ❌ | ✅ | ✅ | @rust-team |
| Live rendering | ❌ | ✅ | ✅ | ✅ | @frontend |
| Smart Enter/Backspace | ✅ transforms | ✅ | ✅ | ❌ | @rust-team |
| Cursor/selection | ✅ cursor | ✅ | ✅ | ✅ | @rust-team |
| Undo/redo | ✅ undo stack | ✅ | ✅ | ❌ | @rust-team |
| Keyboard shortcuts | ❌ | ✅ | ✅ | ✅ | @frontend |
| **Content** ||||||
| Headings/lists/quotes | ✅ parser | ✅ | ✅ | ✅ | @rust-team |
| Code fences | ✅ highlighting | ✅ | ✅ | ✅ | @rust-team |
| Tables | ✅ parser | ✅ | ✅ | ❌ | @rust-team |
| Images | ✅ path resolve | ✅ | ✅ | ✅ | @rust-team |
| Links | ✅ parser | ✅ | ✅ | ✅ | @rust-team |
| Task lists | ✅ parser | ✅ | ✅ | ❌ | @rust-team |
| **Export** ||||||
| HTML export | ✅ sanitizer | ✅ | ✅ | ✅ | @rust-team |
| PDF export | ✅ | ✅ | ✅ | ✅ | @rust-team |
| **Recovery** ||||||
| Autosave | ✅ timer | ✅ | ✅ | ❌ | @rust-team |
| Crash recovery | ✅ | ✅ | ✅ | ❌ | @rust-team |
| External change detect | ✅ watcher | ✅ | ✅ | ❌ | @rust-team |
| **UI/UX** ||||||
| Theme toggle | ❌ | ✅ | ✅ | ✅ | @frontend |
| Focus mode | ❌ | ✅ | ✅ | ✅ | @frontend |
| Typewriter mode | ❌ | ✅ | ✅ | ✅ | @frontend |
| Outline panel | ❌ | ✅ | ✅ | ✅ | @frontend |
| Find/replace | ✅ | ✅ | ✅ | ❌ | @frontend |

### CI Gates

| Gate | Required | Tool |
|------|----------|------|
| `cargo test` | ✅ All passing | cargo test |
| `cargo clippy` | ✅ Zero warnings | clippy |
| `cargo fmt --check` | ✅ | rustfmt |
| `npm test` | ✅ All passing | vitest |
| `npm run build` | ✅ | vite build |
| Visual regression | ⚠️ PR review | Percy/Playwright |
| Accessibility scan | ⚠️ PR review | axe-core |

### Test Data Requirements

* `fixtures/markdown/` - 20+ sample .md files covering all syntax
* `fixtures/edge-cases/` - Malformed docs, large docs, unicode
* `fixtures/export/` - Expected HTML/PDF outputs for regression
* `fixtures/recovery/` - Snapshot test cases

---

## 22. Definition of Done

A feature is done only when:
* implementation is merged
* tests added or updated
* supported behavior documented if relevant
* no known major data-loss risk remains
* acceptance criteria are met
* CI passes
