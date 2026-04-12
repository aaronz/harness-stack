# Research: RustNote MVP Test Plan Implementation

**Date:** 2026-04-12  
**Feature:** RustNote MVP Test Plan v3.1

---

## 1. Tauri 2.x Testing Best Practices

### Decision
Use `cargo test` for Rust unit/integration tests, Playwright for E2E, and Tauri's built-in IPC testing patterns.

### Rationale
- Rust tests run in `src-tauri/tests/` directory
- Playwright supports Tauri via `tauri://localhost` protocol
- Tauri 2.x has improved testing support with `tauri::test` macro

### Alternatives Considered
- `rusty-fork` for subprocess testing - overkill for this use case
- `mockall` for mocking - useful for command mocking

---

## 2. Playwright + Tauri Integration

### Decision
Use Playwright with `@playwright/test` for E2E testing. Current `e2e/critical/` and `e2e/regression/` structure is correct.

### Rationale
- Playwright supports desktop Tauri apps via webview
- Existing `playwright.config.ts` with project setup
- Platform coverage (Chromium, Firefox, WebKit)

### Integration Points
```typescript
// playwright.config.ts
export default defineConfig({
  testDir: './e2e',
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
    { name: 'firefox', use: { browserName: 'firefox' } },
    { name: 'webkit', use: { browserName: 'webkit' } },
  ],
});
```

---

## 3. Visual Regression Tools

### Decision
Use Playwright's built-in screenshot comparison with `toMatchSnapshot()`.

### Rationale
- No external service dependency (Percy requires account)
- Native Playwright integration
- Ark alternative is less mature

### Implementation
```typescript
test('dark theme renders correctly', async ({ page }) => {
  await page.goto('tauri://localhost');
  await page.locator('.theme-dark').click();
  expect(await page.screenshot()).toMatchSnapshot('dark-theme.png');
});
```

---

## 4. Rust Benchmarking

### Decision
Use `criterion` for stable, statistical benchmarks.

### Rationale
- Mature, widely-used crate
- Good HTML reports
- Statistical significance calculations

### Alternative
`divan` - newer, potentially faster, but less community adoption

---

## 5. cargo-fuzz Setup

### Decision
Configure `cargo-fuzz` with 24-hour runs for parser, serializer, deserializer.

### Fuzz Targets
```rust
// fuzz_targets/parser.rs
#![no_main]
libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = SemanticDocument::parse(s);
    }
});
```

### Configuration
```toml
# .cargo/fuzz.toml
[target.parsers]
runs = 86400  # 24 hours
```

---

## 6. WCAG Accessibility Testing

### Decision
Integrate `axe-core` with Playwright for automated accessibility testing.

### Rationale
- `playwright-axe` package available
- WCAG AA compliance required per spec FR-T180-183
- Complements manual testing

### Implementation
```typescript
import AxeBuilder from '@axe-core/playwright';

test('editor accessibility', async ({ page }) => {
  await page.goto('tauri://localhost');
  const results = await new AxeBuilder({ page }).analyze();
  expect(results.violations).toEqual([]);
});
```

---

## 7. Test Data Fixtures Structure

### Decision
Follow spec section 12 structure:

```
fixtures/
├── markdown/           # 11 fixture files
│   ├── headings.md
│   ├── emphasis.md
│   ├── links.md
│   ├── lists.md
│   ├── tasklists.md
│   ├── blockquotes.md
│   ├── codeblocks.md
│   ├── tables.md
│   ├── frontmatter.md
│   ├── mixed.md
│   ├── long.md
│   └── wide.md
├── edge-cases/         # 6 fixture files
│   ├── malformed.md
│   ├── binary.md
│   ├── longlines.md
│   ├── unicode.md
│   ├── deep-nesting.md
│   └── large-table.md
├── export/             # Expected outputs
│   ├── minimal.html
│   ├── full.html
│   ├── code.html
│   ├── minimal.pdf
│   └── full.pdf
└── recovery/            # Snapshot test cases
    ├── crash-during-edit.json
    ├── crash-after-save.json
    └── stale.json
```

---

## 8. Key Unknowns Resolved

| Unknown | Resolution |
|---------|-----------|
| Test framework for Rust | `cargo test` with `#[test]` macros |
| Frontend unit tests | Vitest (per spec section 2.3) |
| E2E framework | Playwright (already in use) |
| Visual regression | Playwright snapshots |
| Performance benchmarks | Criterion |
| Security fuzzing | cargo-fuzz |
| Accessibility testing | axe-core with Playwright |
| Fixtures format | .md files per spec section 12 |

---

## 9. Implementation Approach

### No NEEDS CLARIFICATION markers remain

All technical decisions have been made based on:
- Existing project infrastructure (Playwright already set up)
- Spec requirements (section 2.3 - Test Types)
- Industry best practices for Tauri/Rust apps
