# AI-Readiness Evaluator - Tasks

**Feature**: AI-Readiness Evaluator  
**Generated**: 2026-04-02

---

## Implementation Phases

### Phase 1: Setup (Project Initialization)

- [X] T001 Initialize Next.js 14 + TypeScript project with Express backend
- [X] T002 Configure SQLite database with better-sqlite3
- [X] T003 Set up project structure (frontend/backend separation)
- [X] T004 Configure ESLint, Prettier, and TypeScript strict mode

---

### Phase 2: Foundational (Core Infrastructure)

- [X] T005 Create database schema for Requirement, Score, RiskHighlight, OptimizationSuggestion, LLMProvider entities
- [X] T006 [P] Implement database migration system
- [X] T007 [P] Set up API error handling and validation middleware
- [X] T008 Implement LLM provider abstraction layer (interface only)

---

## Phase 3: Core Evaluation (US1 - Submit Requirements & Get Scored Assessment)

**Story Goal**: User submits a requirement and receives AI readiness score with level classification

**Independent Test Criteria**: 
- Submit requirement text via API → receive valid JSON with score (0-100) and level (S/A/B/C)
- Submit requirement via web form → receive same structured response

### Implementation Tasks

- [X] T009 [P] [US1] Create Requirement entity and repository in src/entities/requirement.ts
- [X] T010 [P] [US1] Create Score entity and repository in src/entities/score.ts
- [X] T011 [US1] Implement requirement submission endpoint POST /api/requirements in src/server/routes/requirements.ts
- [X] T012 [US1] Implement requirement validation (text required, max length)
- [X] T013 [US1] Create scoring engine with 5-dimension weighted calculation in src/services/scoring-engine.ts
- [X] T014 [US1] Implement complexity penalty logic (Simple/Medium/Complex detection)
- [X] T015 [US1] Implement level classification (S/A/B/C) in src/services/score-classifier.ts
- [X] T016 [US1] Create requirement retrieval endpoint GET /api/requirements/:id in src/server/routes/requirements.ts
- [X] T017 [US1] Build frontend requirement submission form in src/pages/index.tsx
- [X] T018 [US1] Display score result with level classification on frontend

---

## Phase 4: Risk Analysis (US2 - View Risk Heatmap & Optimization Suggestions)

**Story Goal**: User receives detailed risk highlights and actionable improvement suggestions

**Independent Test Criteria**:
- Submit vague requirement → receive risk heatmap with highlighted text spans
- Submit requirement → receive categorized suggestions (Context/Boundary/Atomicity/Verifiability)

### Implementation Tasks

- [X] T019 [P] [US2] Create RiskHighlight entity and repository in src/entities/risk-highlight.ts
- [X] T020 [P] [US2] Create OptimizationSuggestion entity and repository in src/entities/optimization-suggestion.ts
- [X] T021 [US2] Implement risk analysis service using LLM in src/services/risk-analyzer.ts
- [X] T022 [US2] Implement suggestion generation service in src/services/suggestion-generator.ts
- [X] T023 [US2] Integrate risk analysis and suggestions into scoring flow
- [X] T024 [US2] Add risk heatmap to API response
- [X] T025 [US2] Add optimization suggestions to API response
- [X] [US2] Build frontend risk display component in src/components/RiskHeatmap.tsx
- [X] [US2] Build frontend suggestions display in src/components/OptimizationSuggestions.tsx

---

## Phase 5: LLM Provider Management (US3 - Configure LLM Provider)

**Story Goal**: User can switch LLM providers at runtime without code changes

**Independent Test Criteria**:
- Change provider in settings → subsequent evaluations use new provider
- Select Ollama → data stays local, evaluation works
- Override default provider per request → that request uses specified provider

### Implementation Tasks

- [X] T026 [P] [US3] Implement LLM provider configuration in src/config/providers.ts
- [X] T027 [P] [US3] Create LLM provider adapters (OpenAI, Anthropic, Gemini, Ollama)
- [X] T028 [US3] Implement provider selection middleware in src/middleware/provider-selector.ts
- [X] T029 [US3] Add provider configuration endpoint GET/PUT /api/config/providers in src/server/routes/config.ts
- [X] T030 [US3] Implement per-request provider override via header or body
- [X] T031 [US3] Build frontend provider selector in src/components/ProviderSelector.tsx

---

## Phase 6: Score Transparency (US4 - View Detailed Score Breakdown)

**Story Goal**: User sees dimension-level scores and complexity penalty application

**Independent Test Criteria**:
- Request detailed breakdown → receive per-dimension scores (Context/Atomicity/Boundary/Verifiability/Technical)
- Request detailed breakdown → see complexity penalty coefficient and calculation

### Implementation Tasks

- [X] T032 [US4] Expand score response to include dimension-level scores
- [X] T033 [US4] Add complexity penalty details to response
- [X] T034 [US4] Create frontend score breakdown visualization in src/components/ScoreBreakdown.tsx

---

## Phase 7: Polish & Cross-Cutting Concerns

- [X] T035 Implement request logging and audit trail
- [X] T036 Add rate limiting for API endpoints
- [X] T037 Build requirements history view in frontend
- [ ] T038 Add bulk evaluation queue system
- [ ] T039 Performance optimization (caching, connection pooling)
- [X] T040 Security hardening (input sanitization, API authentication)

---

## Dependencies

```
Setup (T001-T004)
    │
    ▼
Foundational (T005-T008)
    │
    ├────────────────────────────────────────┐
    ▼                                        ▼
US1: Core Evaluation (T009-T018)    US2: Risk Analysis (T019-T025)
    │                                        │
    └──────────────┐                        │
                   ▼                        ▼
        US3: LLM Provider (T026-T031) ◄──────┘
                   │
                   ▼
        US4: Score Transparency (T032-T034)
                   │
                   ▼
            Polish (T035-T040)
```

---

## Parallel Execution Opportunities

| Phase | Parallel Tasks | Reason |
|-------|----------------|--------|
| Phase 1 | T001, T002, T003, T004 | Independent setup |
| Phase 2 | T005, T006, T007, T008 | Foundation layer |
| Phase 3 US1 | T009, T010 | Both entity/repo, no dependency |
| Phase 4 US2 | T019, T020 | Both entities, no dependency |
| Phase 5 US3 | T026, T027 | Both provider configs, no dependency |

---

## MVP Scope

**User Story 1 Only**: Core Evaluation
- T001-T004: Setup
- T005-T008: Foundational  
- T009-T018: Core Evaluation (US1)

This delivers: requirement submission → scored assessment with level classification

---

## Task Count Summary

| Phase | Task Count |
|-------|------------|
| Setup | 4 |
| Foundational | 4 |
| US1 (Core Evaluation) | 10 |
| US2 (Risk Analysis) | 7 |
| US3 (LLM Provider) | 6 |
| US4 (Score Transparency) | 3 |
| Polish | 6 |
| **Total** | **40** |

---

## Validation

All tasks follow the checklist format: `- [ ] [TaskID] [P?] [Story?] Description with file path`
