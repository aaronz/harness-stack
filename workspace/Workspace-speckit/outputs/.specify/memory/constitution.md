# AI-Ready Evaluator - Project Constitution

**Version**: 1.0.0  
**Ratification Date**: 2026-03-31  
**Last Amended**: 2026-04-02  

---

## 1. Project Identity

**PROJECT_NAME**: AI-Ready Evaluator  
**PROJECT_ALIAS**: AI Coding可落地性评估系统  
**TYPE**: Web Application (Full-Stack)  
**TECH_STACK**: TypeScript (Node.js + Express + React/Next.js 14), SQLite  

---

## 2. Core Mission

This system evaluates development requirements for **AI Coding implementability**—transforming the assessment focus from "can humans understand this" to "can AI generate correct code without ambiguity."

**Success Metric**: Requirements rated S/A should achieve >80% AI code acceptance on first generation.

---

## 3. Principles

### PRINCIPLE_1_NAME: Context Sufficiency

**Non-Negotiable Rules**:
- Every requirement MUST contain a business goal statement (what business problem is solved, not technical solution description)
- Domain-specific terminology MUST be defined in a glossary section
- User journey mapping MUST identify where the feature sits in the complete operation flow
- Data entity relationships MUST be explicitly documented (objects involved and their associations)

**Rationale**: AI lacks business context and generates code that deviates from actual intent when requirements lack background. Without explicit definitions, AI assumes generic patterns—often using incorrect state machines or business rules.

---

### PRINCIPLE_2_NAME: Logic Atomicity

**Non-Negotiable Rules**:
- Each requirement MUST follow the Single Responsibility Principle—do one thing only
- Requirement description length SHOULD NOT exceed 3000 tokens to stay within effective AI context window
- All dependencies (preconditions, external services, data sources) MUST be explicitly listed
- Requirements MUST be independently developable, testable, and deployable

**Rationale**: Oversized requirements cause AI to "forget" early constraints mid-generation. Implicit dependencies create deadlocks and compilation errors.

---

### PRINCIPLE_3_NAME: Boundary Definiteness

**Non-Negotiable Rules**:
- Every input parameter MUST define its valid value domain, invalid values, and null handling
- All possible states and state transition conditions MUST be enumerated (not just Happy Path)
- Concurrency scenarios MUST specify locking/competition handling strategy
- Performance boundaries (time complexity, space complexity, throughput) MUST be quantified

**Rationale**: AI defaults to Happy Path code generation. Without explicit boundary definitions, AI produces unhandled null checks, race conditions, and missing error flows.

---

### PRINCIPLE_4_NAME: Verifiability

**Non-Negotiable Rules**:
- Requirements MUST provide input/output examples including edge cases and error scenarios
- Acceptance criteria MUST be quantified ("response time <200ms" not "fast response")
- Acceptance criteria MUST be transformable into automated test assertions
- Interface contracts MUST specify preconditions, postconditions, and invariants

**Rationale**: Without examples, AI misinterprets data formats. Without quantified metrics, AI-generated code fails performance tests. Without automatable assertions, AI cannot self-verify correctness.

---

### PRINCIPLE_5_NAME: Technical Constraint Clarity

**Non-Negotiable Rules**:
- Architecture pattern (layered, microservices, event-driven) MUST be explicitly mandated
- Technology stack MUST specify exact versions (language, framework, libraries—whitelists and blacklists)
- API contracts MUST declare protocol (REST/GraphQL/gRPC) and data format (JSON Schema/Protobuf)
- Security constraints MUST state authentication method, permission model, and sensitive data handling rules

**Rationale**: AI selects deprecated APIs when versions are unspecified. AI generates unsecure endpoints without explicit security constraints. AI chooses solutions incompatible with existing architecture.

---

### PRINCIPLE_6_NAME: LLM Flexibility

**Non-Negotiable Rules**:
- System MUST support runtime LLM provider switching without code changes
- Default provider SHOULD be GPT-4o, with capability to switch to Claude, Gemini, or local models
- Local deployment mode MUST be available for data privacy requirements

**Rationale**: Users require cost/performance/regulatory flexibility. Single-provider lock-in limits adoption in enterprise and privacy-sensitive contexts.

---

### PRINCIPLE_7_NAME: Scoring Transparency

**Non-Negotiable Rules**:
- AI readiness score MUST be calculable via weighted formula with documented weights
- Complexity penalty coefficients MUST be explicitly defined per requirement type:
  - Simple (CRUD): 1.0
  - Medium (Business Logic): 0.9
  - Complex (Algorithm/Architecture): 0.7
- Risk heatmap MUST highlight requirement sections most likely to be misinterpreted by AI
- Output MUST include actionable optimization suggestions, not just scores

**Rationale**: Users must understand why a requirement received its score. Actionable feedback enables requirement improvement before AI implementation attempts.

---

## 4. Governance

### Amendment Procedure

1. Changes to principles require majority approval from core maintainers
2. Version bump follows semantic versioning:
   - MAJOR: Backward-incompatible principle removal or redefinition
   - MINOR: New principle addition or materially expanded guidance
   - PATCH: Clarifications, wording, typo fixes
3. All amendments MUST be documented in the Sync Impact Report with rationale

### Versioning Policy

- CONSTITUTION_VERSION follows semver (MAJOR.MINOR.PATCH)
- Each version bump MUST be accompanied by a changelog entry
- Deprecated principles MUST remain documented with deprecation notice for one minor version cycle

### Compliance Review Expectations

- Every new feature specification MUST pass constitution check before implementation
- Task generation MUST reference applicable principles for categorization
- Code review SHOULD verify principle-aligned documentation exists for all requirements

---

## 5. Out of Scope

This system does NOT evaluate:
- Technical feasibility (assumes technical capability exists)
- Business value or ROI
- Architecture design quality (only evaluates constraint clarity)

---

## 6. AI Limitations Acknowledgment

System MUST clearly communicate these inherent limitations:

1. **Complex Algorithm Reasoning**: Multi-step mathematical推导 or complex state machines still challenge AI
2. **Implicit Knowledge**: Undocumented legacy conventions and organizational tribal knowledge remain invisible to AI
3. **Creative UI/UX**: Subjective aesthetic judgments cannot be evaluated for AI implementability
4. **Cross-System Coordination**: Multi-subsystem integration scenarios frequently cause AI to miss coordination details

---

## 7. Evaluation Output Levels

| Level | Score Range | Expected Outcome |
|-------|-------------|------------------|
| S | 90-100 | AI can implement independently without human intervention |
| A | 75-89 | AI can implement but requires human review of critical boundaries |
| B | 60-74 | AI generates code requiring significant human modification |
| C | <60 | AI implementation not recommended—risk too high |

---

*This constitution establishes the evaluation framework for AI Coding implementability. All project artifacts MUST align with these principles.*
