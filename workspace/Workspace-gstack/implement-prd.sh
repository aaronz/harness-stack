#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"
PRD_FILE="$WORKSPACE_DIR/../PRD.md"

MODEL="${1:-opencode/minimax-m2.5-free}"

if [ ! -f "$PRD_FILE" ]; then
    echo "Error: PRD.md not found at $PRD_FILE"
    exit 1
fi

PRD_CONTENT=$(cat "$PRD_FILE")

echo "========================================"
echo "GStack workspace - PRD Implementation"
echo "方法论: Think → Plan → Build → Review → Test → Ship → Reflect"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

OUTPUT_DIR="$SCRIPT_DIR/outputs"
mkdir -p "$OUTPUT_DIR"

DESIGN_FILE="$OUTPUT_DIR/design.md"
CEO_REVIEW="$OUTPUT_DIR/ceo-review.md"
ENG_REVIEW="$OUTPUT_DIR/eng-review.md"
REVIEW_REPORT="$OUTPUT_DIR/code-review-report.md"
SHIP_REPORT="$OUTPUT_DIR/ship-report.md"

echo "[Step 1/7] Office Hours - 需求理解..."
OFFICE_PROMPT="请使用 /office-hours 命令开始。

## Requirements Document
$PRD_CONTENT

## 流程
将提出6个强制问题来重新审视产品，挑战前提，生成多种实现方案

## 输出
请将设计文档保存到: $DESIGN_FILE"
opencode run -m "$MODEL" "$OFFICE_PROMPT"

echo ""
echo "[Step 2/7] Plan CEO Review - CEO级审查..."
CEO_PROMPT="请使用 /plan-ceo-review 命令进行CEO级审查。

## 设计文档
$DESIGN_FILE

## 审查要求
重新思考问题，找到10星产品。4种模式: Expansion, Selective Expansion, Hold Scope, Reduction。10章节审查

## 输出
请将审查结果保存到: $CEO_REVIEW"
opencode run -m "$MODEL" "$CEO_PROMPT"

echo ""
echo "[Step 3/7] Plan Eng Review - 工程审查..."
ENG_PROMPT="请使用 /plan-eng-review 命令进行工程审查。

## CEO审查
$CEO_REVIEW

## 审查要求
锁定架构、数据流、图表、边缘情况和测试。ASCII图、状态机、错误路径、测试矩阵、故障模式、安全问题

## 输出
请将工程审查结果保存到: $ENG_REVIEW"
opencode run -m "$MODEL" "$ENG_PROMPT"

echo ""
echo "[Step 4/7] Implement - 实现..."
IMPLEMENT_PROMPT="使用 gstack 的实现模式执行。

## 工程审查
$ENG_REVIEW

## Requirements Document
$PRD_CONTENT"
opencode run -m "$MODEL" "$IMPLEMENT_PROMPT"

echo ""
echo "[Step 5/7] Review - 代码审查..."
REVIEW_PROMPT="请使用 /review 命令进行代码审查。

## 实现产出
请审查Step 4的产出

## 审查要求
找到通过CI但在生产中爆发的bug。AUTO-FIXED 明显问题，FLAGS 完整性差距

## 输出
请将审查报告保存到: $REVIEW_REPORT"
opencode run -m "$MODEL" "$REVIEW_PROMPT"

echo ""
echo "[Step 6/7] QA & Ship - 测试与发布..."
SHIP_PROMPT="请使用 /qa 命令测试你的应用，找到bug并修复。然后使用 /ship 命令同步main、运行测试、审计覆盖率、推送、打开PR。

## 代码审查报告
$REVIEW_REPORT

## 要求
自动生成回归测试

## 输出
请将发布报告保存到: $SHIP_REPORT"
opencode run -m "$MODEL" "$SHIP_PROMPT"

echo ""
echo "[Step 7/7] Retro - 回顾..."
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
