#!/bin/bash
# OpenSpec 迭代脚本 v3.0 - 对比实现与PRD差距，输出增量文档，执行迭代开发

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../lib/common.sh"

parse_args "$@"

WORKSPACE_DIR="$SCRIPT_DIR"
SESSION_LOG_DIR="$WORKSPACE_DIR/sessions"
mkdir -p "$SESSION_LOG_DIR"

setup_iteration_output "$WORKSPACE_DIR" "openspec"

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

log_section "OpenSpec 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "日志文件: $LOG_FILE"

PRD_PATH=$(resolve_prd_path "$PRD_INPUT" "$WORKSPACE_DIR")
log "PRD路径: $PRD_PATH"

mkdir -p "$OUTPUTS_DIR"

log ""
log "[1/5] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

opencode run -m "$MODEL" "$GAP_ANALYSIS_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
log "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"

log ""
log "[2/5] 生成增量文档..."
save_checkpoint "$NEXT_ITERATION" "phase2"

INCREMENT=$(cat << 'INCEOF'
基于差距分析，生成第二轮迭代的增量文档：

## 输出格式
# AI-Ready Evaluator 迭代增量文档 v2.0

## 一、迭代目标
[基于差距分析，列出本轮迭代的核心目标]

## 二、待实现功能清单
### P0 (必须实现)
- [ ] [功能名称] - [差距描述] - [预估工时]

### P1 (应该实现)
- [ ] [功能名称] - [差距描述] - [预估工时]

### P2 (可以实现)
- [ ] [功能名称] - [差距描述] - [预估工时]

## 三、技术债务清单
- [ ] [债务项] - [原因] - [修复建议]

## 四、验收标准
1. [具体可验证的验收条件]

## 五、风险与依赖
- [已识别的风险] - [应对策略]
INCEOF
)

opencode run -m "$MODEL" "$(echo "$INCREMENT"; echo ""; echo "## 差距分析结果"; cat "$OUTPUTS_DIR/gap-analysis.md")" > "$OUTPUTS_DIR/increment.md"
log "增量文档完成: $OUTPUTS_DIR/increment.md"

log ""
log "[3/5] 执行迭代开发..."
save_checkpoint "$NEXT_ITERATION" "phase3"

ITERATION_TASK=$(cat << 'ITEOF'
基于增量文档，执行第二轮开发：

## 增量文档位置
./outputs/iteration-N/increment.md

## PRD位置
./PRD.md

## 实现目录
./outputs/proposal/

## 任务
1. 优先实现P0级别的功能
2. 确保所有核心功能完整
3. 保持代码风格一致
4. 更新相应的文档

## 验证要求
- 代码必须通过 TypeScript 编译
- 必须有基本的错误处理
- 遵循原有架构模式

## 输出
完成后，更新 ./outputs/iteration-N/increment.md，标记已完成的任务。
ITEOF
)

opencode run -m "$MODEL" "$ITERATION_TASK"
log "迭代开发完成"

log ""
log "[4/5] 生成验证报告..."
save_checkpoint "$NEXT_ITERATION" "phase4"

VERIFY=$(cat << 'VEOF'
验证第二轮迭代的产出：

## 验证清单
1. 检查所有P0功能是否已实现
2. 对比 PRD.md 的完整度
3. 检查代码质量
4. 生成验证报告

## 输出格式
# 迭代验证报告

## 实现状态
| 功能 | 状态 | 备注 |
|------|------|------|

## PRD完整度
[计算当前实现与PRD的匹配度百分比]

## 遗留问题
- [ ] [问题] - [优先级] - [建议]

## 下一步建议
1. [具体建议]
VEOF
)

opencode run -m "$MODEL" "$(echo "$VEOF"; echo ""; echo "## 当前实现"; find ./outputs/proposal/packages -name "*.ts" -o -name "*.tsx" | head -20)" > "$OUTPUTS_DIR/verification-report.md"
log "验证报告完成: $OUTPUTS_DIR/verification-report.md"

log ""
log "=============================================="
log "OpenSpec 迭代完成!"
log "=============================================="
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"