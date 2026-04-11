#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../lib/common.sh"

parse_args "$@"

WORKSPACE_DIR="$SCRIPT_DIR"
export WORKSPACE_DIR
SESSION_LOG_DIR="$WORKSPACE_DIR/sessions"
mkdir -p "$SESSION_LOG_DIR"

if [ -n "$RESUME_ITERATION" ]; then
    NEXT_ITERATION="$RESUME_ITERATION"
    OUTPUTS_DIR="$WORKSPACE_DIR/outputs/iteration-${NEXT_ITERATION}"
    if [ ! -d "$OUTPUTS_DIR" ]; then
        echo "Error: Resume iteration $OUTPUTS_DIR does not exist"
        exit 1
    fi
else
    LAST_ITERATION=$(ls -d "$WORKSPACE_DIR/outputs/iteration-"* 2>/dev/null | sed 's/.*iteration-//' | sort -n | tail -1 || echo "0")
    NEXT_ITERATION=$((LAST_ITERATION + 1))
    OUTPUTS_DIR="$WORKSPACE_DIR/outputs/iteration-${NEXT_ITERATION}"
    mkdir -p "$OUTPUTS_DIR"
fi

if [ -z "$LOG_FILE" ]; then
    LOG_FILE="$SESSION_LOG_DIR/iteration-${NEXT_ITERATION}_$(date +%Y%m%d_%H%M%S).log"
fi

PRD_FILE=$(resolve_prd_path "$PRD_INPUT" "$WORKSPACE_DIR")

log_section "Spec Kit 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "PRD: $PRD_FILE"
log "日志文件: $LOG_FILE"

PRD_CONTENT=$(cat "$PRD_FILE")

CONSTITUTION_FILE="$OUTPUTS_DIR/constitution.md"
SPEC_FILE="$OUTPUTS_DIR/spec.md"
PLAN_FILE="$OUTPUTS_DIR/plan.md"
TASKS_FILE="$OUTPUTS_DIR/tasks.md"
GAP_ANALYSIS="$OUTPUTS_DIR/gap-analysis.md"

check_file() {
    if [ ! -f "$1" ]; then
        log "  ❌ 文件缺失: $1"
        return 1
    fi
    if [ ! -s "$1" ] || [ $(wc -c < "$1") -lt 10 ]; then
        log "  ❌ 文件无效（内容过少）: $1"
        return 1
    fi
    log "  ✅ 文件存在: $1 ($(wc -c < "$1") bytes)"
    return 0
}

rerun_if_missing() {
    local file="$1"
    local prompt="$2"
    local max_retries=2
    local attempt=0

    while [ $attempt -lt $max_retries ]; do
        if check_file "$file"; then
            return 0
        fi
        attempt=$((attempt + 1))
        if [ $attempt -lt $max_retries ]; then
            log "  🔄 重新生成 ($attempt/$max_retries)..."
            cd "$WORKSPACE_DIR"
            opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$prompt"
        fi
    done

    if ! check_file "$file"; then
        log "  ⚠️  文件生成失败: $file"
        return 1
    fi
    return 0
}

mkdir -p "$WORKSPACE_DIR/.specify/memory"
mkdir -p "$WORKSPACE_DIR/.specify/templates"
mkdir -p "$WORKSPACE_DIR/.specify/specs"

if [ ! -f "$WORKSPACE_DIR/.specify/memory/constitution.md" ]; then
    if [ -f "$WORKSPACE_DIR/.specify/templates/constitution-template.md" ]; then
        cp "$WORKSPACE_DIR/.specify/templates/constitution-template.md" "$WORKSPACE_DIR/.specify/memory/constitution.md"
    fi
fi

log ""
log "[1/6] PRD Gap Analysis - 差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

GAP_PROMPT="请分析当前实现与PRD的差距。

## 重要约束
- 禁止使用 subagent 或 task 工具 spawning 其他 agent
- 必须直接在当前 session 中完成所有分析工作

## Requirements Document
$PRD_CONTENT

## 输出
将差距分析报告写入到: $GAP_ANALYSIS"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$GAP_PROMPT"
rerun_if_missing "$GAP_ANALYSIS" "$GAP_PROMPT"

log ""
log "[2/6] Constitution - 建立项目原则..."
save_checkpoint "$NEXT_ITERATION" "phase2"

CONSTITUTION_PROMPT="You are creating a project constitution.

## Task
Update the project constitution at \`$WORKSPACE_DIR/.specify/memory/constitution.md\`. This file is a TEMPLATE containing placeholder tokens in square brackets (e.g. \`[PROJECT_NAME]\`, \`[PRINCIPLE_1_NAME]\`). Your job is to (a) collect/derive concrete values, (b) fill the template precisely, and (c) propagate any amendments across dependent artifacts.

## Requirements Document
$PRD_CONTENT

## Execution Steps
1. Load the existing constitution at \`$WORKSPACE_DIR/.specify/memory/constitution.md\`
2. Analyze the requirements document to understand the project
3. Fill all placeholder tokens with concrete values derived from the requirements
4. Define principles appropriate for this specific project based on what the requirements demand
5. Ensure each Principle section has: succinct name, non-negotiable rules, explicit rationale
6. Write the completed constitution to \`$CONSTITUTION_FILE\`

## Output
Save the final constitution to: $CONSTITUTION_FILE"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$CONSTITUTION_PROMPT"
rerun_if_missing "$CONSTITUTION_FILE" "$CONSTITUTION_PROMPT"

log ""
log "[3/6] Specify - 定义需求规范..."
save_checkpoint "$NEXT_ITERATION" "phase3"

SPECIFY_PROMPT="You are creating a feature specification.

## Task
Create a detailed specification based on the requirements document, focusing on WHAT users need and WHY (not HOW to implement).

## Requirements Document
$PRD_CONTENT

## Constitution
$(cat "$WORKSPACE_DIR/.specify/memory/constitution.md" 2>/dev/null || echo "Not yet created")

## Execution Steps
1. Generate a concise short name for the feature based on the requirements
2. Parse the requirements to extract key concepts: actors, actions, data, constraints
3. Create User Scenarios & Testing section
4. Generate Functional Requirements (each must be testable)
5. Define Success Criteria (measurable, technology-agnostic)
6. Identify Key Entities involved

## Output
Save the specification to: $SPEC_FILE"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$SPECIFY_PROMPT"
rerun_if_missing "$SPEC_FILE" "$SPECIFY_PROMPT"

log ""
log "[4/6] Plan - 创建技术实现计划..."
save_checkpoint "$NEXT_ITERATION" "phase4"

PLAN_PROMPT="You are creating a technical implementation plan.

## Task
Create an implementation plan following the plan template structure.

## Specification
$(cat "$SPEC_FILE" 2>/dev/null || echo "Specification not yet created")

## Constitution
$(cat "$WORKSPACE_DIR/.specify/memory/constitution.md" 2>/dev/null || echo "Constitution not yet created")

## Plan Content Required
1. Technical Context (infer appropriate tech stack from requirements)
2. Constitution Check (verify alignment with principles)
3. Phase 0: Research (resolve unknowns)
4. Phase 1: Design (data model, contracts, quickstart)
5. Phase 2: Implementation breakdown
6. File structure and module organization

## Output
Save the plan to: $PLAN_FILE"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$PLAN_PROMPT"
rerun_if_missing "$PLAN_FILE" "$PLAN_PROMPT"

log ""
log "[5/6] Tasks - 生成任务清单..."
save_checkpoint "$NEXT_ITERATION" "phase5"

TASKS_PROMPT="You are generating an actionable task list.

## Task
Create a detailed, dependency-ordered task list from the implementation plan.

## Implementation Plan
$(cat "$PLAN_FILE" 2>/dev/null || echo "Plan not yet created")

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
Save the task list to: $TASKS_FILE"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$TASKS_PROMPT"
rerun_if_missing "$TASKS_FILE" "$TASKS_PROMPT"

log ""
log "[6/6] Implement - 执行实现..."
save_checkpoint "$NEXT_ITERATION" "phase6"

IMPLEMENT_PROMPT="You are implementing a project based on the task list.

## Task
Execute all tasks from the task list to build the complete application.

## Task List
$(cat "$TASKS_FILE" 2>/dev/null || echo "Tasks not yet created")

## Implementation Plan
$(cat "$PLAN_FILE" 2>/dev/null || echo "Plan not yet created")

## 差距分析
$(cat "$GAP_ANALYSIS")

## Execution Rules
1. Complete each phase before moving to the next
2. Respect dependencies - sequential tasks in order, parallel tasks [P] can run together
3. Mark completed tasks as [X] in the tasks file
4. Report progress after each completed task
5. Halt execution if any non-parallel task fails

## Output
Implement all code files according to the task list. Create the complete working application."

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$IMPLEMENT_PROMPT"

log ""
log_section "Spec Kit 迭代完成!"
log "产出文件:"
log "  - Gap Analysis: $GAP_ANALYSIS"
log "  - Constitution: $CONSTITUTION_FILE"
log "  - Specification: $SPEC_FILE"
log "  - Implementation Plan: $PLAN_FILE"
log "  - Task List: $TASKS_FILE"
log "日志保存于: $LOG_FILE"