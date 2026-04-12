# Contracts: RustNote MVP Test Plan

**Date:** 2026-04-12

---

## 1. Test Execution Contract

### TestRunner Interface

```rust
pub trait TestRunner {
    fn execute(&self, suite: &TestSuite) -> TestResult;
    fn run_benchmark(&self, benchmark: &Benchmark) -> BenchmarkResult;
}
```

### TestResult Format

```json
{
  "test_case_id": "FR-T001",
  "status": "pass|fail|skipped|error",
  "duration_ms": 42,
  "timestamp": "2026-04-12T10:30:00Z",
  "error_message": null,
  "logs": []
}
```

---

## 2. Test Fixture Contract

### Fixture File Format

```typescript
interface TestFixture {
  id: string;
  name: string;
  type: 'markdown' | 'edge-case' | 'export' | 'recovery';
  path: string;
  content?: string;  // For inline fixtures
}

// Markdown fixture: plain .md content
// Edge-case fixture: malformed/large/unicode .md content  
// Export fixture: { input: .md, expected_output: .html|.pdf }
// Recovery fixture: JSON snapshot data
```

---

## 3. CI Pipeline Contract

### Pipeline Stages

```
lint (cargo fmt, clippy, eslint) 
  → build (tauri build, npm build)
  → test (cargo test, vitest, playwright)
  → security (cargo audit, cargo-fuzz)
  → deploy (nightly/beta/stable)
```

### Required Gates

| Gate | Tool | Blocking |
|------|------|----------|
| cargo fmt --check | rustfmt | Yes |
| cargo clippy | clippy | Yes |
| cargo test | cargo test | Yes |
| npm run lint | eslint | Yes |
| npm test | vitest | Yes |
| npm run build | vite build | Yes |
| cargo audit | cargo-audit | Yes |
| cargo-fuzz | cargo-fuzz | No |
| Playwright E2E | Playwright | No |
| Visual regression | Percy | No |
| Accessibility scan | axe-core | No |

---

## 4. Benchmark Output Contract

```json
{
  "benchmark_id": "cold-start-empty",
  "metric": "time_to_ready",
  "value_ms": 1850,
  "target_ms": 2000,
  "critical_threshold_ms": 3000,
  "status": "pass",
  "sample_size": 100,
  "timestamp": "2026-04-12T10:30:00Z"
}
```

---

## 5. Export Security Contract

### HTML Export Security Requirements

```typescript
interface ExportSecurityRequirements {
  no_script_tags: boolean;        // FR-T150
  no_event_handlers: boolean;     // FR-T151
  no_external_resources: boolean;  // FR-T152
  no_javascript_urls: boolean;     // FR-T142
  no_iframe_tags: boolean;         // FR-T143
  no_style_tags: boolean;          // FR-T144
  no_data_urls: boolean;           // FR-T145
  no_base_tags: boolean;           // FR-T146
}
```

### Validation

```typescript
function validateExportSecurity(html: string): ExportValidationResult {
  return {
    valid: true,
    violations: []
  };
}
```

---

## 6. Accessibility Contract

### WCAG AA Requirements

```typescript
interface AccessibilityRequirements {
  keyboard_navigation: string[];  // FR-T160-167
  screen_reader_announcements: Record<string, string>;  // FR-T170-176
  color_contrast: {
    text: number;    // >= 4.5:1 (FR-T180)
    large_text: number;  // >= 3:1 (FR-T181)
  };
  focus_indicators: boolean;  // FR-T182
  non_color_information: boolean;  // FR-T183
}
```

### axe-core Integration

```typescript
import AxeBuilder from '@axe-core/playwright';

async function runA11yAudit(page: Page): Promise<AxeResults> {
  return await new AxeBuilder({ page })
    .withTags(['wcag2a', 'wcag2aa'])
    .analyze();
}
```

---

## 7. Platform Test Matrix Contract

```typescript
interface PlatformTestMatrix {
  platform: 'macos-intel' | 'macos-apple-silicon' | 'windows-x64' | 
            'windows-arm' | 'linux-appimage' | 'linux-deb';
  installation: 'P0' | 'P1' | 'P2';
  file_operations: 'P0' | 'P1' | 'P2';
  editor: 'P0' | 'P1' | 'P2';
  export: 'P0' | 'P1' | 'P2';
  accessibility: 'P0' | 'P1' | 'P2';
}
```
