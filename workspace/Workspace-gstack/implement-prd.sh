#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"
PRD_FILE="$WORKSPACE_DIR/../PRD.md"

MODEL="${1:-opencode/minimax-m2.5-free}"

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
echo "GStack workspace - PRD Implementation"
echo "方法论: Think → Plan → Build → Review → Test → Ship → Reflect"
echo "迭代: ${NEXT_ITERATION}"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

DESIGN_FILE="$OUTPUT_DIR/design.md"
CEO_REVIEW="$OUTPUT_DIR/ceo-review.md"
ENG_REVIEW="$OUTPUT_DIR/eng-review.md"
REVIEW_REPORT="$OUTPUT_DIR/code-review-report.md"
SHIP_REPORT="$OUTPUT_DIR/ship-report.md"
GAP_ANALYSIS="$OUTPUT_DIR/gap-analysis.md"

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

echo ""
echo "[1/8] PRD Gap Analysis - 差距分析..."
GAP_PROMPT="请分析当前实现与PRD的差距。

## 重要约束
- 禁止使用 subagent 或 task 工具 spawning 其他 agent
- 必须直接在当前 session 中完成所有分析工作

## Requirements Document
$PRD_CONTENT

## 输出
将差距分析报告写入到: $GAP_ANALYSIS"
opencode run -m "$MODEL" "$GAP_PROMPT"

rerun_if_missing "$GAP_ANALYSIS" "$GAP_PROMPT"

echo ""
echo "[2/8] Office Hours - 需求理解..."
OFFICE_PROMPT="请使用 /office-hours 命令开始。

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 流程
将提出6个强制问题来重新审视产品，挑战前提，生成多种实现方案

## 输出
请将设计文档保存到: $DESIGN_FILE"
opencode run -m "$MODEL" "$OFFICE_PROMPT"

rerun_if_missing "$DESIGN_FILE" "$OFFICE_PROMPT"

echo ""
echo "[3/8] Plan CEO Review - CEO级审查..."
CEO_PROMPT="请使用 /plan-ceo-review 命令进行CEO级审查。

## 设计文档
$DESIGN_FILE

## 差距分析
$(cat $GAP_ANALYSIS)

## 审查要求
重新思考问题，找到10星产品。4种模式: Expansion, Selective Expansion, Hold Scope, Reduction。10章节审查

## 输出
请将审查结果保存到: $CEO_REVIEW"
opencode run -m "$MODEL" "$CEO_PROMPT"

rerun_if_missing "$CEO_REVIEW" "$CEO_PROMPT"

echo ""
echo "[4/8] Plan Eng Review - 工程审查..."
ENG_PROMPT="请使用 /plan-eng-review 命令进行工程审查。

## CEO审查
$CEO_REVIEW

## 审查要求
锁定架构、数据流、图表、边缘情况和测试。ASCII图、状态机、错误路径、测试矩阵、故障模式、安全问题

## 输出
请将工程审查结果保存到: $ENG_REVIEW"
opencode run -m "$MODEL" "$ENG_PROMPT"

rerun_if_missing "$ENG_REVIEW" "$ENG_PROMPT"

echo ""
echo "[5/8] Implement - 实现..."
IMPLEMENT_PROMPT="使用 gstack 的实现模式执行。

## 工程审查
$ENG_REVIEW

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)"
opencode run -m "$MODEL" "$IMPLEMENT_PROMPT"

echo ""
echo "[6/8] Review - 代码审查..."
REVIEW_PROMPT="请使用 /review 命令进行代码审查。

## 实现产出
请审查Step 5的产出

## 差距分析
$(cat $GAP_ANALYSIS)

## 审查要求
找到通过CI但在生产中爆发的bug。AUTO-FIXED 明显问题，FLAGS 完整性差距

## 输出
请将审查报告保存到: $REVIEW_REPORT"
opencode run -m "$MODEL" "$REVIEW_PROMPT"

rerun_if_missing "$REVIEW_REPORT" "$REVIEW_PROMPT"

echo ""
echo "[7/8] QA & Ship - 测试与发布..."
SHIP_PROMPT="请使用 /qa 命令测试你的应用，找到bug并修复。然后使用 /ship 命令同步main、运行测试、审计覆盖率、推送、打开PR。

## 代码审查报告
$REVIEW_REPORT

## 差距分析
$(cat $GAP_ANALYSIS)

## 要求
自动生成回归测试

## 输出
请将发布报告保存到: $SHIP_REPORT"
opencode run -m "$MODEL" "$SHIP_PROMPT"

rerun_if_missing "$SHIP_REPORT" "$SHIP_PROMPT"

echo ""
echo "[8/8] Retro - 回顾..."
RETRO_PROMPT="请使用 /retro 命令进行回顾。

## 发布报告
$SHIP_REPORT

## 要求
每周总结: 人员分解、航运 streaks、测试健康趋势、增长机会"
opencode run -m "$MODEL" "$RETRO_PROMPT"

echo ""
echo "========================================"
echo "GStack workspace 实现完成!"
echo "========================================"
echo ""
echo "产出文件:"
echo "  - Gap Analysis: $GAP_ANALYSIS"
echo "  - Design: $DESIGN_FILE"
echo "  - CEO Review: $CEO_REVIEW"
echo "  - Eng Review: $ENG_REVIEW"
echo "  - Code Review: $REVIEW_REPORT"
echo "  - Ship Report: $SHIP_REPORT"
