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

PRD_PATH=$(resolve_prd_path "$PRD_INPUT" "$WORKSPACE_DIR")
IMPL_DIR="$WORKSPACE_DIR/outputs"
mkdir -p "$IMPL_DIR"

log_section "Planning with Files 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "PRD: $PRD_PATH"
log "日志文件: $LOG_FILE"

log ""
log "[1/5] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "$GAP_ANALYSIS_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
log "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"

log ""
log "[2/5] 更新任务计划..."
save_checkpoint "$NEXT_ITERATION" "phase2"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "更新任务计划文档。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 现有任务计划
$IMPL_DIR/task_plan.md

## 任务
1. 基于差距分析，更新task_plan.md
2. 添加P0/P1/P2任务清单
3. 更新进度追踪

## 输出
更新后的任务计划保存到: $OUTPUTS_DIR/task_plan_v${NEXT_ITERATION}.md"

log ""
log "[3/5] 执行增量开发..."
save_checkpoint "$NEXT_ITERATION" "phase3"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "基于任务计划执行迭代开发。

## 任务计划
$OUTPUTS_DIR/task_plan_v${NEXT_ITERATION}.md

## PRD
$(cat $PRD_PATH)

## 实现目录
$IMPL_DIR/

## 任务
1. 优先实现P0任务
2. 更新进度到$IMPL_DIR/progress.md
3. 添加新发现到$IMPL_DIR/findings.md
4. 确保Build通过

## 验证
- npm run build 必须通过"

log ""
log "[4/5] 更新发现文档..."
save_checkpoint "$NEXT_ITERATION" "phase4"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "更新findings.md文档。

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务执行情况
$IMPL_DIR/progress.md

## 任务
1. 记录新发现的问题
2. 记录解决方案
3. 更新LLM Provider对比分析（如有）

## 输出
更新后的发现保存到: $OUTPUTS_DIR/findings_v${NEXT_ITERATION}.md"

log ""
log "[5/5] 生成验证报告..."
save_checkpoint "$NEXT_ITERATION" "phase5"

cd "$WORKSPACE_DIR"
opencode run --dir "$WORKSPACE_DIR" -m "$MODEL" "生成迭代验证报告。

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务计划
$OUTPUTS_DIR/task_plan_v${NEXT_ITERATION}.md

## 实现状态
检查$IMPL_DIR/目录下的代码

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
log_section "Planning with Files 迭代完成!"
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"