# Data Model: RustNote MVP Test Plan

**Date:** 2026-04-12

---

## 1. Test Entities

### TestSuite

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique identifier (e.g., "parser-unit", "file-integration") |
| name | string | Human-readable name |
| type | enum | unit, integration, e2e, visual, performance, security, accessibility |
| target_module | string | Module being tested (parser, editor, buffer, etc.) |
| test_cases | TestCase[] | Collection of test cases in this suite |

### TestCase

| Field | Type | Description |
|-------|------|-------------|
| id | string | FR-T identifier (e.g., "FR-T001") |
| description | string | Human-readable test description |
| input | any | Test input data or scenario |
| expected | any | Expected output or behavior |
| acceptance_criteria | string | Pass/fail criteria |
| priority | enum | P0, P1, P2 |
| fixture_dependencies | string[] | Required test fixtures |

### TestFixture

| Field | Type | Description |
|-------|------|-------------|
| id | string | Unique fixture identifier |
| name | string | Human-readable name |
| type | enum | markdown, edge-case, export, recovery |
| path | string | File path to fixture |
| content | string | Fixture content (for inline fixtures) |

### Benchmark

| Field | Type | Description |
|-------|------|-------------|
| id | string | Benchmark identifier |
| name | string | Human-readable name |
| metric | string | What is being measured |
| target | number | Desired value |
| critical_threshold | number | Maximum acceptable value |
| measurement_method | string | How to measure |

### TestResult

| Field | Type | Description |
|-------|------|-------------|
| test_case_id | string | Reference to TestCase |
| status | enum | pass, fail, skipped, error |
| duration_ms | number | Execution time |
| timestamp | datetime | When test ran |
| error_message | string? | Error details if failed |

---

## 2. Test Hierarchy

```
TestPlan
├── Unit Test Suites
│   ├── ParserUnitTests (FR-T001-004)
│   │   ├── test_parse_headings
│   │   ├── test_parse_emphasis
│   │   ├── test_serialization
│   │   └── test_malformed_input
│   ├── EditorEngineTests (FR-T010-015)
│   │   ├── test_cursor_movement
│   │   ├── test_selection
│   │   ├── test_smart_enter
│   │   └── test_ime_composition
│   ├── BufferTests (FR-T020-024)
│   │   ├── test_insert_delete
│   │   ├── test_unicode_handling
│   │   └── test_large_doc_performance
│   └── SettingsRecoveryTests (FR-T030-034)
│       ├── test_settings_persistence
│       └── test_snapshot_recovery
├── Integration Test Suites
│   ├── FileOperationsTests (FR-T040-048)
│   ├── EditorIntegrationTests (FR-T050-055)
│   ├── ExportIntegrationTests (FR-T060-067)
│   ├── WorkspaceIntegrationTests (FR-T070-074)
│   └── ExternalChangeTests (FR-T080-083)
├── E2E Test Suites
│   ├── CriticalFlowsTests (FR-T090-097)
│   │   ├── test_open_and_edit_document
│   │   ├── test_create_structured_document
│   │   └── test_crash_recovery
│   └── RegressionTests (FR-T100-109)
├── VisualRegressionTests (FR-T110-127)
├── PerformanceBenchmarkSuites (Section 8)
├── SecurityTestSuites (FR-T130-154)
└── AccessibilityTestSuites (FR-T160-183)
```

---

## 3. State Transitions

### Test Lifecycle States

```
Created → Ready → Running → Completed
                   ↓
                 Failed
                   ↓
              Debugging → Ready
```

### Test Execution State Machine

```
IDLE → RUNNING → PASSED
           ↓
         FAILED → ANALYZING → FIXING → RE_RUNNING
           ↓
         SKIPPED
```

---

## 4. Validation Rules

| Entity | Rule |
|--------|------|
| TestCase.id | Must match FR-T### pattern |
| TestCase.priority | Must be P0, P1, or P2 |
| Benchmark.target | Must be less than critical_threshold |
| TestResult.status | Must be valid enum value |
| TestFixture.path | Must exist when type is file |

---

## 5. Relationships

- TestSuite **contains** many TestCases
- TestCase **references** many TestFixtures
- TestCase **produces** many TestResults
- Benchmark **measures** PerformanceMetric
