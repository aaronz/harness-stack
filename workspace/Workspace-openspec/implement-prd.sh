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

echo "========================================"
echo "OpenSpec Workspace - PRD Implementation"
echo "方法论: /opsx:propose → /opsx:apply → /opsx:archive"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

echo "[Step 1/3] Propose - 创建需求提案..."
opencode run -m "$MODEL" "请使用 /opsx:propose ai-ready-evaluator 命令基于 PRD.md 创建需求提案。PRD内容: AI Coding可落地性评估系统(AI-Ready Evaluator)。文件位置: $PRD_FILE。输出: proposal.md, specs/, design.md, tasks.md"

echo ""
echo "[Step 2/3] Apply - 执行实现..."
opencode run -m "$MODEL" "请使用 /opsx:apply 命令执行 ai-ready-evaluator 的实现。实现内容: 评估维度模型(上下文完备性25%、逻辑原子性25%、边界明确性20%、可验证性15%、技术约束清晰度15%)、评分算法(权重×维度+复杂度惩罚)、S/A/B/C等级报告生成器"

echo ""
echo "[Step 3/3] Archive - 归档..."
opencode run -m "$MODEL" "请使用 /opsx:archive 命令归档完成的功能"

echo ""
echo "========================================"
echo "OpenSpec Workspace 实现完成!"
echo "========================================"