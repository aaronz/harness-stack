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

PRD_PATH=$(resolve_prd_path "$PRD_INPUT" "$WORKSPACE_DIR")
IMPL_DIR="$WORKSPACE_DIR/outputs/src"
mkdir -p "$IMPL_DIR"

log_section "Everything Claude Code 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "子代理: $([ "$USE_SUBAGENTS" = "true" ] && echo "启用" || echo "禁用")"
log "PRD: $PRD_PATH"
log "日志文件: $LOG_FILE"

log ""
log "[1/5] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

if check_file_quiet "$OUTPUTS_DIR/gap-analysis.md"; then
    log "  ⏭️  跳过Gap Analysis（已存在）"
else
    PROMPT=$(build_prompt "分析当前实现与PRD的差距。

## 任务
1. 读取当前实现目录结构
2. 读取PRD.md识别核心功能需求
3. 对比实现与PRD的差距

## 分析维度
1. 功能完整性：PRD中描述的功能是否都已实现？
2. 接口完整性：API是否完整？CRUD是否齐全？
3. 前端完整性：PRD中描述的页面/组件是否都已实现？
4. 数据模型：PRD中的数据实体是否都已建模？
5. 配置管理：PRD中要求的配置项是否都已实现？
6. 测试覆盖：是否有必要的测试？

## 输出
将差距分析报告写入到: $OUTPUTS_DIR/gap-analysis.md")
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
    log "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"
fi

log ""
log "[2/5] Plan - 创建计划..."
save_checkpoint "$NEXT_ITERATION" "phase2"

if check_file_quiet "$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md"; then
    log "  ⏭️  跳过Plan（已存在）"
else
    PROMPT=$(build_prompt "使用 /plan 命令创建实现计划。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 基于差距分析创建详细计划
2. 优先P0任务
3. 明确文件路径和依赖

## 输出
计划保存到: $OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md")
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$PROMPT" > "$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md"
    log "Plan完成: $OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md"
fi

log ""
log "[3/5] Execute - 执行实现..."
save_checkpoint "$NEXT_ITERATION" "phase3"

PROMPT=$(build_prompt "执行实现。

## 计划
$(cat "$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md")

## PRD
$(cat $PRD_PATH)

## 实现目录
$IMPL_DIR/

## 任务
1. 按计划执行实现
2. 优先完成P0任务
3. 确保Build通过

## 验证
- npm run build 必须通过")
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$PROMPT"
log "执行实现完成"

log ""
log "[3/5b] 更新任务状态..."
save_checkpoint "$NEXT_ITERATION" "phase3b"

TASK_STATUS_FILE="$OUTPUTS_DIR/task_status.txt"
PROMPT=$(build_prompt "分析任务完成情况并输出状态报告。

## 计划
$(cat "$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md")

## 任务
1. 分析计划中标记的已完成任务
2. 统计剩余P0/P1/P2任务
3. 输出状态报告到文件

## 输出格式
将以下格式的状态报告写入到: $TASK_STATUS_FILE
```
COMPLETED_TASKS=<逗号分隔的任务描述>
REMAINING_TASKS=<逗号分隔的任务描述>
P0_REMAINING=<数字：P0剩余数量>
P1_REMAINING=<数字：P1剩余数量>
TOTAL_PROGRESS=<已完成数>/<总数>
```

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
log "[4/5] TDD - 测试驱动（如需要）..."
save_checkpoint "$NEXT_ITERATION" "phase4"

PROMPT=$(build_prompt "使用 /tdd 命令为P0功能编写测试。

## 计划
$(cat "$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md")

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 为P0功能编写单元测试
2. 确保测试通过
3. 如无测试必要，跳过此步骤")
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$PROMPT"
log "TDD完成"

log ""
log "[6/6] Verify & Review - 验证..."
save_checkpoint "$NEXT_ITERATION" "phase6"

if check_file_quiet "$OUTPUTS_DIR/verification-report.md"; then
    log "  ⏭️  跳过Verification（已存在）"
else
    PROMPT=$(build_prompt "使用 /verify 和 /code-review 命令进行验证。

## 计划
$(cat "$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md")

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 实现状态
检查$IMPL_DIR/目录

## 输出格式
# 迭代验证报告

## P0问题状态
| 问题 | 状态 | 备注 |

## PRD完整度
[百分比]

## 遗留问题
## 下一步建议

## 输出
验证报告保存到: $OUTPUTS_DIR/verification-report.md")
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$PROMPT" > "$OUTPUTS_DIR/verification-report.md"
    log "Verification完成: $OUTPUTS_DIR/verification-report.md"
fi

log ""
log_section "Everything Claude Code 迭代完成!"
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"