#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

DEFAULT_MODEL="opencode/minimax-m2.5-free"
RESUME_ITERATION=""
MODEL="$DEFAULT_MODEL"
PRD_INPUT=""
VERBOSE="false"
USE_SUBAGENTS="false"
MAX_IMPLEMENTATION_ROUNDS=3
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
            --rounds|-r)
                MAX_IMPLEMENTATION_ROUNDS="$2"
                shift 2
                ;;
            --help|-h)
                echo "Usage: $0 [options]"
                echo "Options:"
                echo "  --resume, -R <N>    Resume from iteration N"
                echo "  --model, -m <M>     Set model (default: $DEFAULT_MODEL)"
                echo "  --prd, -p <P>       PRD file or directory"
                echo "  --verbose, -v       Enable verbose output"
                echo "  --use-subagents     Allow subagent spawning (default: disabled)"
                echo "  --rounds, -r <N>    Max implementation rounds (default: 3)"
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
log "子代理: $([ "$USE_SUBAGENTS" = "true" ] && echo "启用" || echo "禁用")"
log "日志文件: $LOG_FILE"

log "PRD路径: $PRD_PATH"

mkdir -p "$OUTPUTS_DIR"

log ""
log "[1/5] 执行PRD差距分析..."
save_checkpoint "$NEXT_ITERATION" "phase1"

if check_file_quiet "$OUTPUTS_DIR/gap-analysis.md"; then
    log "  ⏭️  跳过Gap Analysis（已存在）"
else
    GAP_PROMPT=$(build_prompt "分析当前实现与PRD的差距。

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
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$GAP_PROMPT" > "$OUTPUTS_DIR/gap-analysis.md"
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

    FULL_PROMPT=$(build_prompt "$(echo "$INCREMENT"; echo ""; echo "## 差距分析结果"; cat "$OUTPUTS_DIR/gap-analysis.md")")
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$FULL_PROMPT" > "$OUTPUTS_DIR/increment.md"
    log "增量文档完成: $OUTPUTS_DIR/increment.md"
fi

impl_round=0
READY_FOR_VERIFY="false"
while [ "$READY_FOR_VERIFY" != "true" ] && [ $impl_round -lt $MAX_IMPLEMENTATION_ROUNDS ]; do
    impl_round=$((impl_round + 1))
    log ""
    log "[3/5] 执行迭代开发... (第${impl_round}轮)"
    save_checkpoint "$NEXT_ITERATION" "phase3-impl${impl_round}"

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

    FULL_PROMPT=$(build_prompt "$(eval echo \"$ITERATION_TASK\")")
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$FULL_PROMPT"
    log "迭代开发完成"

    log ""
    log "[4/5] 更新任务状态..."
    save_checkpoint "$NEXT_ITERATION" "phase4"

    TASK_STATUS_FILE="$OUTPUTS_DIR/task_status.txt"
    PROMPT=$(build_prompt "分析增量任务完成情况并输出状态报告。

## 增量文档
$(cat "$OUTPUTS_DIR/increment.md")

## 任务
1. 分析增量文档中标记的已完成任务
2. 统计剩余P0/P1/P2任务
3. 输出状态报告到文件

## 输出格式
将以下格式的状态报告写入到: $TASK_STATUS_FILE
\`\`\`
COMPLETED_TASKS=<逗号分隔的任务ID或描述>
REMAINING_TASKS=<逗号分隔的任务ID或描述>
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
            break
        fi
    else
        log "  ⚠️  任务状态文件未生成"
    fi

    if [ $impl_round -ge $MAX_IMPLEMENTATION_ROUNDS ]; then
        log "  ⚠️  达到最大轮次限制，进入验证阶段"
        break
    fi

    log "  🔄 仍有P0任务未完成，继续第$((impl_round+1))轮实现..."
done

log ""
log "[5/5] 生成验证报告..."
save_checkpoint "$NEXT_ITERATION" "phase5"

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

    FULL_PROMPT=$(build_prompt "$(echo "$VERIFY"; echo ""; echo "## 当前实现"; find "$PROPOSAL_DIR" -name "*.ts" -o -name "*.tsx" 2>/dev/null | head -20)")
    cd "$WORKSPACE_DIR" && opencode run -m "$MODEL" "$FULL_PROMPT" > "$OUTPUTS_DIR/verification-report.md"
    log "验证报告完成: $OUTPUTS_DIR/verification-report.md"
fi

log ""
log "=============================================="
log "OpenSpec 迭代完成!"
log "=============================================="
log "迭代目录: $OUTPUTS_DIR"
log "日志保存于: $LOG_FILE"