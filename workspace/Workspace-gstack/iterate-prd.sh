#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

DEFAULT_MODEL="opencode/minimax-m2.5-free"
RESUME_ITERATION=""
MODEL="$DEFAULT_MODEL"
PRD_INPUT=""
VERBOSE="false"
WORKSPACE_DIR="$SCRIPT_DIR"
OUTPUTS_DIR=""

parse_args() {
    RESUME_ITERATION=""
    MODEL="$DEFAULT_MODEL"
    PRD_INPUT=""
    VERBOSE="false"

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
            --help|-h)
                echo "Usage: $0 [options]"
                echo "Options:"
                echo "  --resume, -R <N>    Resume from iteration N"
                echo "  --model, -m <M>     Set model (default: $DEFAULT_MODEL)"
                echo "  --prd, -p <P>       PRD file or directory"
                echo "  --verbose, -v       Enable verbose output"
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
    echo "[$timestamp] $1"
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

GAP_ANALYSIS_PROMPT='分析当前实现与PRD的差距：

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

## 通用差距识别
- 缺失的功能模块
- 不完整的实现
- 未连接的模块
- 硬编码/魔法数字
- 错误处理缺失
- 类型定义缺失

## 输出格式
# 差距分析报告

## 差距列表
| 差距项 | 严重程度 | 模块 | 修复建议 |

## P0问题（必须修复）
...

## P1问题（应该修复）
...

## P2问题（可以修复）
...

## 技术债务
...'

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
IMPL_DIR="$WORKSPACE_DIR/outputs/worktrees/ai-ready-evaluator"
mkdir -p "$IMPL_DIR"

log_section "GStack 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "PRD: $PRD_PATH"
log "日志文件: $LOG_FILE"

log ""
log "[1/6] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$GAP_ANALYSIS_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
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

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$(echo "$INCREMENT"; cat "$OUTPUTS_DIR/gap-analysis.md")" > "$OUTPUTS_DIR/increment.md"
log "增量文档完成: $OUTPUTS_DIR/increment.md"

log ""
log "[3/6] Office Hours - 需求理解..."
save_checkpoint "$NEXT_ITERATION" "phase3"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "使用 /office-hours 命令进行需求理解深化。

## PRD
$(cat $PRD_PATH)

## 差距分析
$(cat $OUTPUTS_DIR/gap-analysis.md)

## 要求
重点关注差距分析中识别的P0问题，重新审视产品设计。

## 输出
更新设计文档到: $OUTPUTS_DIR/design-v${NEXT_ITERATION}.md"

log ""
log "[4/6] CEO Review + Eng Review..."
save_checkpoint "$NEXT_ITERATION" "phase4"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "使用 /plan-ceo-review 和 /plan-eng-review 命令进行审查。

## 增量文档
$(cat "$OUTPUTS_DIR/increment.md")

## 现有设计
$WORKSPACE_DIR/outputs/design.md

## 审查重点
1. 架构调整是否合理？
2. 数据持久化方案
3. 多Provider支持方案

## 输出
审查结果保存到: $OUTPUTS_DIR/review-v${NEXT_ITERATION}.md"

log ""
log "[5/6] 执行实现..."
save_checkpoint "$NEXT_ITERATION" "phase5"

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "使用 gstack 的实现模式执行迭代开发。

## 增量文档
$(cat "$OUTPUTS_DIR/increment.md")

## 审查结果
$OUTPUTS_DIR/review-v${NEXT_ITERATION}.md

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

cd "$WORKSPACE_DIR"
cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "使用 /review 和 /qa 命令进行验证。

## 代码审查
$OUTPUTS_DIR/review-v${NEXT_ITERATION}.md

## 要求
1. 找到所有P0问题的修复
2. 确认build通过
3. 生成验证报告

## 输出
验证报告保存到: $OUTPUTS_DIR/verification-report.md"

log ""
log_section "GStack 迭代完成!"
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"