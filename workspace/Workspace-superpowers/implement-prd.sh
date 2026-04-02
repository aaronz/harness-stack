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
echo "Superpowers workspace - PRD Implementation"
echo "方法论: brainstorming → writing-plans → subagent-driven-development → verification → finishing"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

OUTPUT_DIR="$SCRIPT_DIR/outputs"
mkdir -p "$OUTPUT_DIR"

DESIGN_FILE="$OUTPUT_DIR/design.md"
PLAN_FILE="$OUTPUT_DIR/plan.md"
VERIFY_REPORT="$OUTPUT_DIR/verification-report.md"

echo "[Step 1/5] Brainstorming - 需求理解与设计..."
BRAINSTORM_PROMPT="请使用 brainstorming skill 分析 PRD.md 中的需求。

## Requirements Document
$PRD_CONTENT

## 流程
1) 探索项目上下文
2) 提出视觉辅助(如有UI问题)
3) 提出澄清问题
4) 提出2-3个方案及权衡
5) 展示设计sections获取批准

## 输出
请将设计文档保存到: $DESIGN_FILE"
opencode run -m "$MODEL" "$BRAINSTORM_PROMPT"

echo ""
echo "[Step 2/5] Writing Plans - 创建实现计划..."
PLANS_PROMPT="请使用 writing-plans skill 基于已批准的设计创建详细实现计划。

## 设计文档
$DESIGN_FILE

## Requirements Document
$PRD_CONTENT

## 计划要求
- 分解为2-5分钟可完成的原子任务
- 每个任务有精确文件路径、完整代码、验证步骤
- 使用 subagent-driven-development skill 进行任务分解

## 输出
请将计划保存到: $PLAN_FILE"
opencode run -m "$MODEL" "$PLANS_PROMPT"

echo ""
echo "[Step 3/5] Subagent-Driven Development - 执行实现..."
SDD_PROMPT="请使用 subagent-driven-development skill 执行实现计划。

## 实现计划
$PLAN_FILE

## Requirements Document
$PRD_CONTENT

## 执行要求
每个任务由fresh subagent执行，两阶段review(规范合规性→代码质量)"
opencode run -m "$MODEL" "$SDD_PROMPT"

echo ""
echo "[Step 4/5] Verification - 验证..."
VERIFY_PROMPT="请使用 verification-before-completion skill 进行最终验证。

## 实现产出
请验证Step 3的实现产出

## Requirements Document
$PRD_CONTENT

## 验证要点
- 功能完整性
- 代码质量
- 测试覆盖(80%+)

## 输出
请将验证报告保存到: $VERIFY_REPORT"
opencode run -m "$MODEL" "$VERIFY_PROMPT"

echo ""
echo "[Step 5/5] Finishing - 完成开发..."
FINISH_PROMPT="请使用 finishing-a-development-branch skill 完成开发。

## 验证报告
$VERIFY_REPORT

## 完成要求
验证测试通过，展示选项(merge/PR/keep/discard)，清理worktree"
opencode run -m "$MODEL" "$FINISH_PROMPT"

echo ""
echo "========================================"
echo "Superpowers workspace 实现完成!"
echo "========================================"
