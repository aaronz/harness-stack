# Quickstart: Implementing RustNote MVP Test Plan

**Date:** 2026-04-12

---

## 1. Project Structure

```
rustnote/
├── src-tauri/
│   ├── src/
│   │   ├── parser/
│   │   ├── buffer/
│   │   ├── editor/
│   │   └── services/
│   └── tests/
│       └── editor_transforms.rs  ← Add unit tests here
├── www/
│   ├── src/
│   │   └── components/
│   └── __tests__/               ← Create for Vitest
├── e2e/
│   ├── critical/                ← E2E critical flows
│   └── regression/              ← E2E regression suite
├── fixtures/
│   ├── markdown/                ← Section 12.1
│   ├── edge-cases/              ← Section 12.2
│   ├── export/                  ← Section 12.3
│   └── recovery/                ← Section 12.4
└── scripts/
    └── benchmark.ts             ← Performance benchmarks
```

---

## 2. Running Tests

### Rust Unit Tests

```bash
cd rustnote
cargo test
cargo test --package rustnote_lib
cargo bench  # Run benchmarks
```

### Frontend Unit Tests (Vitest)

```bash
cd rustnote/www
npm test
npm run test:watch
```

### E2E Tests (Playwright)

```bash
cd rustnote
npx playwright test
npx playwright test --project=chromium
npx playwright test e2e/critical/
npx playwright test e2e/regression/
```

### Visual Regression

```bash
npx playwright test --project=visual
npx playwright screenshot
```

### Accessibility

```bash
npx playwright test a11y/
```

---

## 3. Adding New Tests

### Rust Unit Test

```rust
// src-tauri/tests/editor_transforms.rs

#[test]
fn test_parse_headings_h1_h6() {
    let cases = vec![
        ("# H1", 1),
        ("## H2", 2),
        ("### H3", 3),
        ("#### H4", 4),
        ("##### H5", 5),
        ("###### H6", 6),
    ];
    
    for (source, expected_level) in cases {
        let doc = SemanticDocument::parse(source);
        assert_eq!(doc.headings[0].level, expected_level);
    }
}
```

### Playwright E2E Test

```typescript
// e2e/critical/open-edit.spec.ts

test('E2E-C01: Open and edit existing document', async ({ page }) => {
  await page.goto('tauri://localhost');
  
  // Open file
  await page.click('[data-testid="menu-file"]');
  await page.click('[data-testid="menu-file-open"]');
  
  // Verify loads in 500ms
  const start = Date.now();
  await page.locator('.editor').waitFor({ state: 'visible' });
  expect(Date.now() - start).toBeLessThan(500);
  
  // Edit and save
  await page.keyboard.type('## New Content');
  await page.keyboard.press('Control+s');
  
  // Reopen and verify
  // ...
});
```

### Vitest Component Test

```typescript
// www/__tests__/Editor.test.tsx

import { test, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Editor } from '../src/components/Editor';

test('renders markdown heading', () => {
  render(<Editor content="# Hello" />);
  expect(screen.getByRole('heading', { level: 1 })).toHaveTextContent('Hello');
});
```

---

## 4. Test Fixtures

### Creating Markdown Fixtures

```bash
# fixtures/markdown/headings.md
# H1
## H2
### H3
#### H4
##### H5
###### H6
```

### Creating Edge Case Fixtures

```bash
# fixtures/edge-cases/unicode.md
中文测试
العربية
日本語
한국어
👍 🚀 💯
```

### Creating Export Fixtures

```html
<!-- fixtures/export/expected-minimal.html -->
<!DOCTYPE html>
<html>
<head><title>Minimal</title></head>
<body>
<h1>Heading</h1>
<p>Paragraph</p>
</body>
</html>
```

---

## 5. CI Integration

### GitHub Actions (example)

```yaml
# .github/workflows/test.yml
- name: Run unit tests
  run: cargo test
  
- name: Run E2E tests
  run: npx playwright test
  
- name: Run benchmarks
  run: cargo bench -- --nfo 100
```

---

## 6. Performance Benchmarking

### Rust Criterion

```rust
// benches/parsing.rs

use criterion::{black_box, criterion_group, Criterion};

fn bench_parse_large_doc(c: &mut Criterion) {
    let content = std::fs::read_to_string("fixtures/large.md").unwrap();
    
    c.bench_function("parse_1mb_doc", |b| {
        b.iter(|| {
            SemanticDocument::parse(black_box(&content))
        })
    });
}

criterion_group!(benches, bench_parse_large_doc);
```

### Playwright Performance Test

```typescript
// e2e/performance/keystroke.spec.ts

test('keystroke to render < 100ms', async ({ page }) => {
  await page.goto('tauri://localhost');
  
  const measurements: number[] = [];
  
  for (let i = 0; i < 100; i++) {
    const start = performance.now();
    await page.keyboard.type('x');
    await page.waitForTimeout(0);
    measurements.push(performance.now() - start);
  }
  
  const avg = measurements.reduce((a, b) => a + b) / measurements.length;
  expect(avg).toBeLessThan(100);
});
```

---

## 7. Debugging Failed Tests

```bash
# Run single test with output
cargo test test_name -- --nocapture

# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Playwright debug mode
PWDEBUG=1 npx playwright test

# View Playwright report
npx playwright show-report
```
