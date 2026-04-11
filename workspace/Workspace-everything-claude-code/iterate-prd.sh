#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../lib/common.sh"

parse_args "$@"

WORKSPACE_DIR="$SCRIPT_DIR"
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
log "PRD: $PRD_PATH"
log "日志文件: $LOG_FILE"

log ""
log "[1/5] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

cd "$WORKSPACE_DIR"
opencode run -m "$MODEL" "$GAP_ANALYSIS_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
log "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"

log ""
log "[2/5] Plan - 创建计划..."
save_checkpoint "$NEXT_ITERATION" "phase2"

cd "$WORKSPACE_DIR"
opencode run -m "$MODEL" "使用 /plan 命令创建实现计划。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 基于差距分析创建详细计划
2. 优先P0任务
3. 明确文件路径和依赖

## 输出
计划保存到: $OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md"

log ""
log "[3/5] Execute - 执行实现..."
save_checkpoint "$NEXT_ITERATION" "phase3"

cd "$WORKSPACE_DIR"
opencode run -m "$MODEL" "执行实现。

## 计划
$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md

## PRD
$(cat $PRD_PATH)

## 实现目录
$IMPL_DIR/

## 任务
1. 按计划执行实现
2. 优先完成P0任务
3. 确保Build通过

## 验证
- npm run build 必须通过"

log ""
log "[4/5] TDD - 测试驱动（如需要）..."
save_checkpoint "$NEXT_ITERATION" "phase4"

cd "$WORKSPACE_DIR"
opencode run -m "$MODEL" "使用 /tdd 命令为P0功能编写测试。

## 计划
$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 为P0功能编写单元测试
2. 确保测试通过
3. 如无测试必要，跳过此步骤"

log ""
log "[5/5] Verify & Review - 验证..."
save_checkpoint "$NEXT_ITERATION" "phase5"

cd "$WORKSPACE_DIR"
opencode run -m "$MODEL" "使用 /verify 和 /code-review 命令进行验证。

## 计划
$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md

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
验证报告保存到: $OUTPUTS_DIR/verification-report.md"

log ""
log_section "Everything Claude Code 迭代完成!"
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"