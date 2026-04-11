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
IMPL_DIR="$WORKSPACE_DIR/outputs/src"

log_section "Superpowers 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "PRD: $PRD_PATH"
log "日志文件: $LOG_FILE"

log ""
log "[1/5] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$GAP_ANALYSIS_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
log "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"

log ""
log "[2/5] Brainstorming - 需求理解..."
save_checkpoint "$NEXT_ITERATION" "phase2"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "请使用 brainstorming skill 分析差距并深化设计。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 任务
1. 基于差距分析提出澄清问题
2. 提出2-3个方案及权衡
3. 展示设计sections获取批准

## 输出
设计文档保存到: $OUTPUTS_DIR/design_v${NEXT_ITERATION}.md"

log ""
log "[3/5] Writing Plans - 创建计划..."
save_checkpoint "$NEXT_ITERATION" "phase3"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "请使用 writing-plans skill 创建详细实现计划。

## 设计文档
$OUTPUTS_DIR/design_v${NEXT_ITERATION}.md

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 计划要求
1. 分解为2-5分钟可完成的原子任务
2. 每个任务有精确文件路径
3. 使用subagent-driven-development skill进行任务分解
4. 优先P0任务

## 输出
计划保存到: $OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md"

log ""
log "[4/5] Subagent-Driven Development..."
save_checkpoint "$NEXT_ITERATION" "phase4"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "请使用 subagent-driven-development skill 执行实现。

## 实现计划
$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md

## PRD
$(cat $PRD_PATH)

## 实现目录
$IMPL_DIR/

## 任务
1. 每个任务由fresh subagent执行
2. 两阶段review(规范合规性→代码质量)
3. 优先完成P0任务
4. 确保Build通过

## 验证
- npm run build 必须通过"

log ""
log "[5/5] Verification - 验证..."
save_checkpoint "$NEXT_ITERATION" "phase5"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "请使用 verification-before-completion skill 进行最终验证。

## 实现产出
检查$IMPL_DIR/目录

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 验证要点
1. P0问题是否全部修复？
2. Build是否通过？
3. 功能是否完整？

## 输出
验证报告保存到: $OUTPUTS_DIR/verification-report.md"

log ""
log_section "Superpowers 迭代完成!"
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"