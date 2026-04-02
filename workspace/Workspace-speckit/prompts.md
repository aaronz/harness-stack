# SpecKit Step-by-Step Prompts

Execute these prompts in sequence to implement a PRD using the SpecKit methodology.

---

## Prerequisites

- A `PRD.md` file in your workspace root
- OpenCode CLI installed (`opencode run -m <model> "<prompt>"`)
- Directory structure: `.specify/memory/`, `.specify/templates/`, `.specify/specs/`

---

## Step 1: Constitution - 建立项目原则

```bash
opencode run -m "opencode/minimax-m2.5-free" "You are creating a project constitution.

## Task
Update the project constitution at \`.specify/memory/constitution.md\`. This file is a TEMPLATE containing placeholder tokens in square brackets (e.g. \`[PROJECT_NAME]\`, \`[PRINCIPLE_1_NAME]\`). Your job is to (a) collect/derive concrete values, (b) fill the template precisely, and (c) propagate any amendments across dependent artifacts.

## Requirements Document
$(cat PRD.md)

## Execution Steps
1. Load the existing constitution at \`.specify/memory/constitution.md\`
2. Analyze the requirements document to understand the project
3. Fill all placeholder tokens with concrete values derived from the requirements
4. Define principles appropriate for this specific project based on what the requirements demand
5. Ensure each Principle section has: succinct name, non-negotiable rules, explicit rationale
6. Write the completed constitution to \`.specify/memory/constitution.md\`

## Output
Save the final constitution to: ./outputs/constitution.md"
```

---

## Step 2: Specify - 定义需求规范

```bash
opencode run -m "opencode/minimax-m2.5-free" "You are creating a feature specification.

## Task
Create a detailed specification based on the requirements document, focusing on WHAT users need and WHY (not HOW to implement).

## Requirements Document
$(cat PRD.md)

## Constitution
$(cat .specify/memory/constitution.md 2>/dev/null || echo "Not yet created")

## Execution Steps
1. Generate a concise short name for the feature based on the requirements
2. Parse the requirements to extract key concepts: actors, actions, data, constraints
3. Create User Scenarios & Testing section
4. Generate Functional Requirements (each must be testable)
5. Define Success Criteria (measurable, technology-agnostic)
6. Identify Key Entities involved

## Output
Save the specification to: ./outputs/spec.md"
```

---

## Step 3: Plan - 创建技术实现计划

```bash
opencode run -m "opencode/minimax-m2.5-free" "You are creating a technical implementation plan.

## Task
Create an implementation plan following the plan template structure.

## Specification
$(cat outputs/spec.md 2>/dev/null || echo "Specification not yet created")

## Constitution
$(cat .specify/memory/constitution.md 2>/dev/null || echo "Constitution not yet created")

## Plan Content Required
1. Technical Context (infer appropriate tech stack from requirements)
2. Constitution Check (verify alignment with principles)
3. Phase 0: Research (resolve unknowns)
4. Phase 1: Design (data model, contracts, quickstart)
5. Phase 2: Implementation breakdown
6. File structure and module organization

## Output
Save the plan to: ./outputs/plan.md"
```

---

## Step 4: Tasks - 生成任务清单

```bash
opencode run -m "opencode/minimax-m2.5-free" "You are generating an actionable task list.

## Task
Create a detailed, dependency-ordered task list from the implementation plan.

## Implementation Plan
$(cat outputs/plan.md 2>/dev/null || echo "Plan not yet created")

## Task Generation Rules
1. Organize by user story to enable independent implementation and testing
2. Use strict checklist format: \`- [ ] [TaskID] [P?] [Story?] Description with file path\`
3. Task IDs: Sequential (T001, T002, T003...)
4. [P] marker: Include ONLY if task is parallelizable (different files, no dependencies)
5. [Story] label: Format [US1], [US2], etc. for user story phase tasks

## Phase Structure
- Phase 1: Setup (project initialization)
- Phase 2: Foundational (blocking prerequisites)
- Phase 3+: User Stories in priority order
- Final Phase: Polish & Cross-Cutting Concerns

## Output
Save the task list to: ./outputs/tasks.md"
```

---

## Step 5: Implement - 执行实现

```bash
opencode run -m "opencode/minimax-m2.5-free" "You are implementing a project based on the task list.

## Task
Execute all tasks from the task list to build the complete application.

## Task List
$(cat outputs/tasks.md 2>/dev/null || echo "Tasks not yet created")

## Implementation Plan
$(cat outputs/plan.md 2>/dev/null || echo "Plan not yet created")

## Execution Rules
1. Complete each phase before moving to the next
2. Respect dependencies - sequential tasks in order, parallel tasks [P] can run together
3. Mark completed tasks as [X] in the tasks file
4. Report progress after each completed task
5. Halt execution if any non-parallel task fails

## Output
Implement all code files according to the task list. Create the complete working application."
```

---

## Quick Reference

| Step | Output File |
|------|-------------|
| 1 | outputs/constitution.md |
| 2 | outputs/spec.md |
| 3 | outputs/plan.md |
| 4 | outputs/tasks.md |
| 5 | (code changes) |

---

## Custom Model

Replace model in any step:
```bash
opencode run -m "anthropic/claude-3.5-sonnet" "<prompt>"
opencode run -m "opencode/llama-3.1-70b" "<prompt>"
```