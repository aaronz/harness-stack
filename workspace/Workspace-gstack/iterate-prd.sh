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
IMPL_DIR="$WORKSPACE_DIR/outputs/worktrees/ai-ready-evaluator"

log_section "GStack 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "PRD: $PRD_PATH"
log "日志文件: $LOG_FILE"

log ""
log "[1/6] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

opencode run -m "$MODEL" "$GAP_ANALYSIS_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
log "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"

log ""
log "[2/6] 生成增量文档..."
save_checkpoint "$NEXT_ITERATION" "phase2"

INCREMENT=$(cat << 'INCEOF'
# GStack 迭代增量文档 v3.0

## 一、迭代目标
[基于差距分析的核心目标]

## 二、待实现功能
### P0
- [ ] [功能]

### P1
- [ ] [功能]

## 三、技术债务
- [ ] [债务项] - [修复建议]

## 四、验收标准
1. [具体条件]
INCEOF
)

opencode run -m "$MODEL" "$(echo "$INCREMENT"; cat "$OUTPUTS_DIR/gap-analysis.md")" > "$OUTPUTS_DIR/increment.md"
log "增量文档完成: $OUTPUTS_DIR/increment.md"

log ""
log "[3/6] Office Hours - 需求理解..."
save_checkpoint "$NEXT_ITERATION" "phase3"

opencode run -m "$MODEL" "使用 /office-hours 命令进行需求理解深化。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 要求
重点关注差距分析中识别的P0问题，重新审视产品设计。

## 输出
更新设计文档到: ./outputs/iteration-${NEXT_ITERATION}/design-v${NEXT_ITERATION}.md"

log ""
log "[4/6] CEO Review + Eng Review..."
save_checkpoint "$NEXT_ITERATION" "phase4"

opencode run -m "$MODEL" "使用 /plan-ceo-review 和 /plan-eng-review 命令进行审查。

## 增量文档
$(cat "$OUTPUTS_DIR/increment.md")

## 现有设计
./outputs/design.md

## 审查重点
1. 架构调整是否合理？
2. 数据持久化方案
3. 多Provider支持方案

## 输出
审查结果保存到: ./outputs/iteration-${NEXT_ITERATION}/review-v${NEXT_ITERATION}.md"

log ""
log "[5/6] 执行实现..."
save_checkpoint "$NEXT_ITERATION" "phase5"

opencode run -m "$MODEL" "使用 gstack 的实现模式执行迭代开发。

## 增量文档
$(cat "$OUTPUTS_DIR/increment.md")

## 审查结果
./outputs/iteration-${NEXT_ITERATION}/review-v${NEXT_ITERATION}.md

## 目录
$IMPL_DIR

## 任务
1. 优先实现P0功能
2. 确保核心功能完整
3. 保持代码风格一致
4. 添加必要的错误处理

## 验证
- npm run build 必须通过"

log ""
log "[6/6] 验证与QA..."
save_checkpoint "$NEXT_ITERATION" "phase6"

opencode run -m "$MODEL" "使用 /review 和 /qa 命令进行验证。

## 代码审查
./outputs/iteration-${NEXT_ITERATION}/review-v${NEXT_ITERATION}.md

## 要求
1. 找到所有P0问题的修复
2. 确认build通过
3. 生成验证报告

## 输出
验证报告保存到: ./outputs/iteration-${NEXT_ITERATION}/verification-report.md"

log ""
log_section "GStack 迭代完成!"
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"
