#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"
PRD_FILE="$WORKSPACE_DIR/../PRD.md"

MODEL="${1:-opencode/minimax-m2.5-free}"

# Iteration tracking
LAST_ITERATION=$(ls -d "$SCRIPT_DIR/outputs/iteration-"* 2>/dev/null | sed 's/.*iteration-//' | sort -n | tail -1 || echo "0")
NEXT_ITERATION=$((LAST_ITERATION + 1))
OUTPUT_DIR="$SCRIPT_DIR/outputs/iteration-${NEXT_ITERATION}"
mkdir -p "$OUTPUT_DIR"

if [ ! -f "$PRD_FILE" ]; then
    echo "Error: PRD.md not found at $PRD_FILE"
    exit 1
fi

PRD_CONTENT=$(cat "$PRD_FILE")

echo "========================================"
echo "Superpowers workspace - PRD Implementation"
echo "方法论: brainstorming → writing-plans → subagent-driven-development → verification → finishing"
echo "迭代: ${NEXT_ITERATION}"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

# File validation function
check_file() {
    if [ ! -f "$1" ]; then
        echo "  ❌ 文件缺失: $1"
        return 1
    fi
    if [ ! -s "$1" ] || [ $(wc -c < "$1") -lt 10 ]; then
        echo "  ❌ 文件无效（内容过少）: $1"
        return 1
    fi
    echo "  ✅ 文件存在: $1 ($(wc -c < "$1") bytes)"
    return 0
}

# Retry function for missing files
rerun_if_missing() {
    local file="$1"
    local prompt="$2"
    local max_retries=2
    local attempt=0

    while [ $attempt -lt $max_retries ]; do
        if check_file "$file"; then
            return 0
        fi
        attempt=$((attempt + 1))
        if [ $attempt -lt $max_retries ]; then
            echo "  🔄 重新生成 ($attempt/$max_retries)..."
            opencode run -m "$MODEL" "$prompt"
        fi
    done

    if ! check_file "$file"; then
        echo "  ⚠️  文件生成失败: $file"
        return 1
    fi
    return 0
}

DESIGN_FILE="$OUTPUT_DIR/design.md"
PLAN_FILE="$OUTPUT_DIR/plan.md"
VERIFY_REPORT="$OUTPUT_DIR/verification-report.md"
GAP_ANALYSIS="$OUTPUT_DIR/gap-analysis.md"

echo ""
echo "[1/5] PRD Gap Analysis - 差距分析..."
GAP_PROMPT="请分析当前实现与PRD的差距。

## 重要约束
- 禁止使用 subagent 或 task 工具 spawning 其他 agent
- 必须直接在当前 session 中完成所有分析工作

## Requirements Document
$PRD_CONTENT

## 分析维度
1. 功能完整性：PRD中描述的功能是否都已实现？
2. 接口完整性：API是否完整？CRUD是否齐全？
3. 前端完整性：PRD中描述的页面/组件是否都已实现？
4. 数据模型：PRD中的数据实体是否都已建模？
5. 配置管理：PRD中要求的配置项是否都已实现？
6. 测试覆盖：是否有必要的测试？

## 输出
将差距分析报告写入到: $GAP_ANALYSIS"
opencode run -m "$MODEL" "$GAP_PROMPT"

rerun_if_missing "$GAP_ANALYSIS" "$GAP_PROMPT"

echo ""
echo "[2/5] Brainstorming - 需求理解与设计..."
BRAINSTORM_PROMPT="请使用 brainstorming skill 分析 PRD.md 中的需求。

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 流程
1) 探索项目上下文
2) 提出视觉辅助(如有UI问题)
3) 提出澄清问题
4) 提出2-3个方案及权衡
5) 展示设计sections获取批准

## 输出
请将设计文档保存到: $DESIGN_FILE"
opencode run -m "$MODEL" "$BRAINSTORM_PROMPT"

rerun_if_missing "$DESIGN_FILE" "$BRAINSTORM_PROMPT"

echo ""
echo "[3/5] Writing Plans - 创建实现计划..."
PLANS_PROMPT="请使用 writing-plans skill 基于已批准的设计创建详细实现计划。

## 设计文档
$DESIGN_FILE

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 计划要求
- 分解为2-5分钟可完成的原子任务
- 每个任务有精确文件路径、完整代码、验证步骤
- 使用 subagent-driven-development skill 进行任务分解

## 输出
请将计划保存到: $PLAN_FILE"
opencode run -m "$MODEL" "$PLANS_PROMPT"

rerun_if_missing "$PLAN_FILE" "$PLANS_PROMPT"

echo ""
echo "[4/5] Subagent-Driven Development - 执行实现..."
SDD_PROMPT="请使用 subagent-driven-development skill 执行实现计划。

## 实现计划
$PLAN_FILE

## Requirements Document
$PRD_CONTENT

## 执行要求
每个任务由fresh subagent执行，两阶段review(规范合规性→代码质量)"
opencode run -m "$MODEL" "$SDD_PROMPT"

echo ""
echo "[5/5] Verification - 验证..."
VERIFY_PROMPT="请使用 verification-before-completion skill 进行最终验证。

## 实现产出
请验证Step 4的实现产出

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 验证要点
- 功能完整性
- 代码质量
- 测试覆盖(80%+)

## 输出
请将验证报告保存到: $VERIFY_REPORT"
opencode run -m "$MODEL" "$VERIFY_PROMPT"

rerun_if_missing "$VERIFY_REPORT" "$VERIFY_PROMPT"

echo ""
echo "========================================"
echo "Superpowers workspace 实现完成!"
echo "========================================"
echo ""
echo "输出文件:"
echo "  - Gap Analysis: $GAP_ANALYSIS"
echo "  - Design: $DESIGN_FILE"
echo "  - Plan: $PLAN_FILE"
echo "  - Verification: $VERIFY_REPORT"
