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

check_file_quiet() {
    if [ ! -f "$1" ]; then
        return 1
    fi
    if [ ! -s "$1" ] || [ $(wc -c < "$1") -lt 10 ]; then
        return 1
    fi
    return 0
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

if check_file_quiet "$OUTPUTS_DIR/gap-analysis.md"; then
    log "  ⏭️  跳过Gap Analysis（已存在）"
else
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$GAP_ANALYSIS_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
    log "差距分析完成: $OUTPUTS_DIR/gap-analysis.md"
fi

log ""
log "[2/5] Brainstorming - 需求理解..."
save_checkpoint "$NEXT_ITERATION" "phase2"

if check_file_quiet "$OUTPUTS_DIR/design_v${NEXT_ITERATION}.md"; then
    log "  ⏭️  跳过Brainstorming（已存在）"
else
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
    log "Brainstorming完成: $OUTPUTS_DIR/design_v${NEXT_ITERATION}.md"
fi

log ""
log "[3/5] Writing Plans - 创建计划..."
save_checkpoint "$NEXT_ITERATION" "phase3"

if check_file_quiet "$OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md"; then
    log "  ⏭️  跳过Writing Plans（已存在）"
else
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
    log "Writing Plans完成: $OUTPUTS_DIR/plan_v${NEXT_ITERATION}.md"
fi

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

if check_file_quiet "$OUTPUTS_DIR/verification-report.md"; then
    log "  ⏭️  跳过Verification（已存在）"
else
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
    log "Verification完成: $OUTPUTS_DIR/verification-report.md"
fi

log ""
log_section "Superpowers 迭代完成!"
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"