#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

DEFAULT_MODEL="opencode/minimax-m2.5-free"
RESUME_ITERATION=""
MODEL="$DEFAULT_MODEL"
PRD_INPUT=""
VERBOSE="false"
USE_SUBAGENTS="false"
WORKSPACE_DIR="$SCRIPT_DIR"
OUTPUTS_DIR=""

CONSTRAINTS_NO_SUBAGENT='## 重要约束
- 禁止使用 subagent 或 task 工具 spawning 其他 agent
- 禁止将工作委托给其他 agent
- 必须直接在当前 session 中完成所有分析/实现工作
- 只使用 Read、Write、Edit、Grep、LSP、Bash 等直接工具

'

parse_args() {
    RESUME_ITERATION=""
    MODEL="$DEFAULT_MODEL"
    PRD_INPUT=""
    VERBOSE="false"
    USE_SUBAGENTS="false"

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --resume|-R)
                RESUME_ITERATION="$2"
                shift 2
                ;;
            --model|-m)
                MODEL="$2"
                shift 2
                ;;
            --prd|-p)
                PRD_INPUT="$2"
                shift 2
                ;;
            --verbose|-v)
                VERBOSE="true"
                shift
                ;;
            --use-subagents)
                USE_SUBAGENTS="true"
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [options]"
                echo "Options:"
                echo "  --resume, -R <N>    Resume from iteration N"
                echo "  --model, -m <M>     Set model (default: $DEFAULT_MODEL)"
                echo "  --prd, -p <P>       PRD file or directory"
                echo "  --verbose, -v       Enable verbose output"
                echo "  --use-subagents     Allow subagent spawning (default: disabled)"
                exit 0
                ;;
            *)
                shift
                ;;
        esac
    done
}

log() {
    local timestamp
    timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local message="[$timestamp] $1"
    echo "$message"
    if [ -n "$LOG_FILE" ]; then
        echo "$message" >> "$LOG_FILE"
    fi
}

log_section() {
    echo "=============================================="
    echo "$1"
    echo "=============================================="
}

save_checkpoint() {
    local iteration="$1"
    local phase="$2"
    local checkpoint_file="$OUTPUTS_DIR/.checkpoint"
    
    mkdir -p "$OUTPUTS_DIR"
    cat > "$checkpoint_file" << EOF
iteration=$iteration
phase=$phase
timestamp=$(date '+%Y-%m-%d %H:%M:%S')
EOF
    log "Checkpoint saved: iteration=$iteration, phase=$phase"
}

resolve_prd_path() {
    local prd_input="$1"
    local workspace="$2"
    
    if [ -n "$prd_input" ]; then
        if [ -d "$prd_input" ]; then
            echo "$prd_input"
        elif [ -f "$prd_input" ]; then
            echo "$prd_input"
        else
            echo "Error: PRD path does not exist: $prd_input"
            exit 1
        fi
    else
        echo "$workspace/PRD.md"
    fi
}

parse_args "$@"
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

LOG_FILE="$SESSION_LOG_DIR/iteration-${NEXT_ITERATION}_$(date +%Y%m%d_%H%M%S).log"

PRD_FILE=$(resolve_prd_path "$PRD_INPUT" "$WORKSPACE_DIR")

log_section "Spec Kit 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "PRD: $PRD_FILE"
log "子代理: $([ "$USE_SUBAGENTS" = "true" ] && echo "启用" || echo "禁用")"
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

check_file_quiet() {
    if [ ! -f "$1" ]; then
        return 1
    fi
    if [ ! -s "$1" ] || [ $(wc -c < "$1") -lt 10 ]; then
        return 1
    fi
    return 0
}

build_prompt() {
    local prompt_body="$1"
    if [ "$USE_SUBAGENTS" = "false" ]; then
        echo "${CONSTRAINTS_NO_SUBAGENT}${prompt_body}"
    else
        echo "${prompt_body}"
    fi
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
            cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$prompt"
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

if check_file_quiet "$GAP_ANALYSIS"; then
    log "  ⏭️  跳过Gap Analysis（已存在）"
else
    GAP_PROMPT=$(build_prompt "请分析当前实现与PRD的差距。

## Requirements Document
$PRD_CONTENT

## 输出
将差距分析报告写入到: $GAP_ANALYSIS")

    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$GAP_PROMPT"
    rerun_if_missing "$GAP_ANALYSIS" "$GAP_PROMPT"
fi

log ""
log "[2/6] Constitution - 建立项目原则..."
save_checkpoint "$NEXT_ITERATION" "phase2"

if check_file_quiet "$CONSTITUTION_FILE"; then
    log "  ⏭️  跳过Constitution（已存在）"
else
    CONSTITUTION_PROMPT=$(build_prompt "You are creating a project constitution.

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
Save the final constitution to: $CONSTITUTION_FILE")

    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$CONSTITUTION_PROMPT"
    rerun_if_missing "$CONSTITUTION_FILE" "$CONSTITUTION_PROMPT"
fi

log ""
log "[3/6] Specify - 定义需求规范..."
save_checkpoint "$NEXT_ITERATION" "phase3"

if check_file_quiet "$SPEC_FILE"; then
    log "  ⏭️  跳过Specify（已存在）"
else
    SPECIFY_PROMPT=$(build_prompt "You are creating a feature specification.

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
Save the specification to: $SPEC_FILE")

    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$SPECIFY_PROMPT"
    rerun_if_missing "$SPEC_FILE" "$SPECIFY_PROMPT"
fi

log ""
log "[4/6] Plan - 创建技术实现计划..."
save_checkpoint "$NEXT_ITERATION" "phase4"

if check_file_quiet "$PLAN_FILE"; then
    log "  ⏭️  跳过Plan（已存在）"
else
    PLAN_PROMPT=$(build_prompt "You are creating a technical implementation plan.

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
Save the plan to: $PLAN_FILE")

    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$PLAN_PROMPT"
    rerun_if_missing "$PLAN_FILE" "$PLAN_PROMPT"
fi

log ""
log "[5/6] Tasks - 生成任务清单..."
save_checkpoint "$NEXT_ITERATION" "phase5"

if check_file_quiet "$TASKS_FILE"; then
    log "  ⏭️  跳过Tasks（已存在）"
else
    TASKS_PROMPT=$(build_prompt "You are generating an actionable task list.

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
Save the task list to: $TASKS_FILE")

    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$TASKS_PROMPT"
    rerun_if_missing "$TASKS_FILE" "$TASKS_PROMPT"
fi

log ""
log "[6/6] Implement - 执行实现..."
save_checkpoint "$NEXT_ITERATION" "phase6"

IMPLEMENT_PROMPT=$(build_prompt "You are implementing a project based on the task list.

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
Implement all code files according to the task list. Create the complete working application.")

cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$IMPLEMENT_PROMPT"

log ""
log "[6/6] 更新任务状态..."
save_checkpoint "$NEXT_ITERATION" "phase6b"

TASK_STATUS_FILE="$OUTPUTS_DIR/task_status.txt"
PROMPT=$(build_prompt "分析任务完成情况并输出状态报告。

## 任务列表
$(cat "$TASKS_FILE")

## 任务
1. 读取任务列表中所有任务的状态
2. 统计已完成和剩余任务
3. 输出状态报告到文件

## 输出格式
将以下格式的状态报告写入到: $TASK_STATUS_FILE
\`\`\`
COMPLETED_TASKS=<逗号分隔的任务ID列表>
REMAINING_TASKS=<逗号分隔的任务ID列表>
P0_REMAINING=<数字：P0剩余数量>
P1_REMAINING=<数字：P1剩余数量>
TOTAL_PROGRESS=<已完成数>/<总数>
\`\`\`

如果所有P0任务已完成，输出: READY_FOR_VERIFICATION=true
如果还有P0任务未完成，输出: READY_FOR_VERIFICATION=false")
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$PROMPT" 2>/dev/null || true

if [ -f "$TASK_STATUS_FILE" ]; then
    log "任务状态已更新:"
    cat "$TASK_STATUS_FILE" | while read line; do log "  $line"; done
    
    READY_FOR_VERIFY=$(grep "READY_FOR_VERIFICATION=" "$TASK_STATUS_FILE" 2>/dev/null | cut -d= -f2)
    if [ "$READY_FOR_VERIFY" = "true" ]; then
        log "  ✅ 所有P0任务完成，可以进入验证阶段"
    else
        log "  ⚠️  仍有P0任务未完成，建议继续实现"
    fi
else
    log "  ⚠️  任务状态文件未生成"
fi

log ""
log_section "Spec Kit 迭代完成!"
log "产出文件:"
log "  - Gap Analysis: $GAP_ANALYSIS"
log "  - Constitution: $CONSTITUTION_FILE"
log "  - Specification: $SPEC_FILE"
log "  - Implementation Plan: $PLAN_FILE"
log "  - Task List: $TASKS_FILE"
log "  - Task Status: $TASK_STATUS_FILE"
log "日志保存于: $LOG_FILE"