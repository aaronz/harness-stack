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

LOG_FILE="$SESSION_LOG_DIR/iteration-${NEXT_ITERATION}_$(date +%Y%m%d_%H%M%S).log"

PRD_PATH=$(resolve_prd_path "$PRD_INPUT" "$WORKSPACE_DIR")
PROPOSAL_DIR="$WORKSPACE_DIR/outputs/proposal"
mkdir -p "$PROPOSAL_DIR"

log_section "OpenSpec 迭代开发 v3.0"
log "工作目录: $WORKSPACE_DIR"
log "迭代目录: $OUTPUTS_DIR"
log "模型: $MODEL"
log "日志文件: $LOG_FILE"

log "PRD路径: $PRD_PATH"

mkdir -p "$OUTPUTS_DIR"

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
log "[2/5] 生成增量文档..."
save_checkpoint "$NEXT_ITERATION" "phase2"

if check_file_quiet "$OUTPUTS_DIR/increment.md"; then
    log "  ⏭️  跳过Increment（已存在）"
else
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

    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$(echo "$INCREMENT"; echo ""; echo "## 差距分析结果"; cat "$OUTPUTS_DIR/gap-analysis.md")" > "$OUTPUTS_DIR/increment.md"
    log "增量文档完成: $OUTPUTS_DIR/increment.md"
fi

log ""
log "[3/5] 执行迭代开发..."
save_checkpoint "$NEXT_ITERATION" "phase3"

ITERATION_TASK=$(cat << 'ITEOF'
基于增量文档，执行第二轮开发：

## 增量文档位置
$OUTPUTS_DIR/increment.md

## PRD位置
$WORKSPACE_DIR/PRD.md

## 实现目录
$PROPOSAL_DIR/

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
完成后，更新 $OUTPUTS_DIR/increment.md，标记已完成的任务。
ITEOF
)

cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$(eval echo \"$ITERATION_TASK\")"
log "迭代开发完成"

log ""
log "[4/5] 生成验证报告..."
save_checkpoint "$NEXT_ITERATION" "phase4"

if check_file_quiet "$OUTPUTS_DIR/verification-report.md"; then
    log "  ⏭️  跳过Verification（已存在）"
else
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

    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$(echo "$VERIFY"; echo ""; echo "## 当前实现"; find "$PROPOSAL_DIR" -name "*.ts" -o -name "*.tsx" 2>/dev/null | head -20)" > "$OUTPUTS_DIR/verification-report.md"
    log "验证报告完成: $OUTPUTS_DIR/verification-report.md"
fi

log ""
log "=============================================="
log "OpenSpec 迭代完成!"
log "=============================================="
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"