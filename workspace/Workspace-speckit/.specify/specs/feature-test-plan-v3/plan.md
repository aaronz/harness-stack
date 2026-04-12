# RustNote MVP Test Plan Implementation Plan

**Feature:** RustNote MVP Test Plan v3.1  
**Based on:** Spec v3.1 (2026-04-12)  
**Status:** Planning Phase  
**Branch:** `feature/test-plan-v3`

---

## Phase 0: Technical Context

### What This Feature Is

This is a **Test Plan Specification** - a comprehensive QA document that defines:
- Unit test requirements (parser, editor engine, buffer, settings, recovery)
- Integration test requirements (file ops, editor, export, workspace)
- E2E test requirements (critical user flows, regression suite)
- Visual regression requirements (themes, modes, components)
- Performance benchmarks (NFR thresholds)
- Security test requirements (input validation, sanitization)
- Accessibility test requirements (keyboard nav, screen reader)

**Implementation = Writing the actual test code and infrastructure**

### Technical Context

| Component | Current State | Needed |
|-----------|--------------|--------|
| **Rust Unit Tests** | `src-tauri/tests/editor_transforms.rs` - basic transform tests | Expand to full FR-T001 to FR-T034 coverage |
| **Vitest (Frontend)** | No `www/__tests__/` directory | Create and add component tests |
| **Playwright E2E** | `e2e/critical/` and `e2e/regression/` exist | Expand to FR-T090 to FR-T109 |
| **Visual Regression** | No Percy/Ark setup | Integrate visual diff tooling |
| **Performance Benchmarks** | No benchmark infrastructure | Add Rust benchmarks + Playwright perf tests |
| **Security Testing** | No dedicated security tests | Add cargo-fuzz integration |
| **Accessibility Testing** | No axe-core integration | Add a11y tests |
| **Test Fixtures** | `fixtures/` with markdown samples | Expand to full spec coverage |

### Dependencies

- **Rust 1.75+** with `cargo test`
- **Node.js 20+** with vitest
- **Playwright 1.40+** for E2E
- **cargo-fuzz** for security testing
- **axe-core** for accessibility

---

## Phase 1: Research

### Research Tasks

1. **Tauri 2.x Testing Best Practices** - How to test Tauri commands and IPC
2. **Playwright + Tauri Integration** - E2E testing patterns for Tauri apps
3. **Visual Regression with Playwright** - Percy vs Ark vs native screenshot diff
4. **Rust Benchmarking** - `criterion` vs `divan` for NFR validation
5. **cargo-fuzz Setup** - Parser/serializer fuzzing configuration
6. **WCAG Testing with axe-core** - Automated accessibility validation

### Output: `research.md`

---

## Phase 2: Data Model

### Test Entities

| Entity | Fields | Relationships |
|--------|--------|---------------|
| **TestSuite** | id, name, type (unit/integration/e2e), target_module | contains TestCase |
| **TestCase** | id, description, input, expected, acceptance_criteria | belongs to TestSuite |
| **TestFixture** | id, name, type (markdown/edge-case/export/recovery), path | used by TestCase |
| **Benchmark** | id, name, metric, target, critical_threshold | measures PerformanceMetric |
| **TestResult** | test_case_id, status, duration, timestamp, logs | references TestCase |

### Test Type Hierarchy

```
TestPlan
├── Unit Tests
│   ├── Parser Tests (FR-T001-004)
│   ├── Editor Engine Tests (FR-T010-015)
│   ├── Buffer Tests (FR-T020-024)
│   └── Settings/Recovery Tests (FR-T030-034)
├── Integration Tests
│   ├── File Operations (FR-T040-048)
│   ├── Editor Integration (FR-T050-055)
│   ├── Export Integration (FR-T060-067)
│   ├── Workspace Integration (FR-T070-074)
│   └── External Change Detection (FR-T080-083)
├── E2E Tests
│   ├── Critical Flows (FR-T090-097)
│   └── Regression Suite (FR-T100-109)
├── Visual Regression (FR-T110-127)
├── Performance Benchmarks (FR-T130+)
├── Security Tests (FR-T130-154)
└── Accessibility Tests (FR-T160-183)
```

---

## Phase 3: Interface Contracts

### Test Execution Contract

```
TestRunner.execute(suite: TestSuite) -> TestResult
TestRunner.runBenchmark(benchmark: Benchmark) -> BenchmarkResult
TestFixture.load(name: string) -> FixtureData
TestReporter.generateReport(results: TestResult[]) -> Report
```

### CI Pipeline Contract

```
Pipeline: lint → build → test → deploy
  ├── Lint: cargo fmt, clippy, eslint
  ├── Build: tauri build, npm run build
  ├── Test: cargo test, vitest, playwright
  └── Security: cargo audit, cargo-fuzz
```

### Test Data Contract

```
FixtureFormat: .md files in fixtures/markdown/, fixtures/edge-cases/, etc.
BenchmarkOutput: JSON with metric, value, threshold, status
TestReport: JSON with suite, results, duration, coverage
```

---

## Phase 4: Implementation Plan

### Priority Order

#### P0 - Critical Test Infrastructure (Week 1)

| Task | Tests Covered | Effort |
|------|--------------|--------|
| Expand Rust unit test coverage | FR-T001-004, FR-T010-015, FR-T020-024, FR-T030-034 | 3 days |
| Set up Vitest in www/ | Component tests for UI | 1 day |
| Add Playwright E2E for critical flows | FR-T090-097 | 2 days |
| Create test fixtures directory structure | All fixtures in spec section 12 | 1 day |

#### P1 - Integration & Regression (Week 2)

| Task | Tests Covered | Effort |
|------|--------------|--------|
| Integration tests for file operations | FR-T040-048 | 1 day |
| Integration tests for export | FR-T060-067 | 1 day |
| Integration tests for workspace | FR-T070-074 | 1 day |
| Regression test suite | FR-T100-109 | 2 days |

#### P2 - Specialized Testing (Week 3)

| Task | Tests Covered | Effort |
|------|--------------|--------|
| Visual regression setup | FR-T110-127 | 2 days |
| Performance benchmarks | Section 8 NFRs | 2 days |
| Security tests (input validation) | FR-T130-134 | 1 day |
| Security tests (HTML sanitization) | FR-T140-146, FR-T150-154 | 1 day |
| Accessibility tests | FR-T160-183 | 1 day |

#### P3 - CI/CD Integration (Week 4)

| Task | Purpose | Effort |
|------|---------|--------|
| Add all tests to CI pipeline | gates in section 13 | 1 day |
| Set up cargo-fuzz for parser | 24hr fuzz runs | 0.5 day |
| Add visual regression to CI | Percy/Ark integration | 0.5 day |

---

## Success Criteria

- [ ] All FR-T001 through FR-T183 have corresponding test code
- [ ] Test fixtures cover all spec section 12 requirements
- [ ] CI pipeline gates pass (section 13.2)
- [ ] Performance benchmarks measure all NFRs in section 8.1
- [ ] cargo-fuzz runs configured for parser, serializer, deserializer
- [ ] Accessibility tests run with axe-core

---

*Plan created: 2026-04-12*
