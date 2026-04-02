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
echo "OpenSpec workspace - PRD Implementation"
echo "方法论: /opsx:propose → /opsx:apply → /opsx:archive"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

OUTPUT_DIR="$SCRIPT_DIR/outputs"
mkdir -p "$OUTPUT_DIR"

PROPOSAL_DIR="$OUTPUT_DIR/proposal"

echo "[Step 1/3] Propose - 创建需求提案..."
PROPOSE_PROMPT="请使用 /opsx:propose ai-ready-evaluator 命令基于 PRD.md 创建需求提案。

## Requirements Document
$PRD_CONTENT

## 输出要求
请将产出保存到: $PROPOSAL_DIR
- proposal.md: 需求提案
- specs/: 详细规格
- design.md: 设计文档
- tasks.md: 任务清单"
opencode run -m "$MODEL" "$PROPOSE_PROMPT"

echo ""
echo "[Step 2/3] Apply - 执行实现..."
APPLY_PROMPT="请使用 /opsx:apply 命令执行 ai-ready-evaluator 的实现。

## 提案产出参考
$PROPOSAL_DIR

## Requirements Document
$PRD_CONTENT

## 输出要求
将实现代码保存到项目目录，确保与proposal中的设计一致"
opencode run -m "$MODEL" "$APPLY_PROMPT"

echo ""
echo "[Step 3/3] Archive - 归档..."
ARCHIVE_PROMPT="请使用 /opsx:archive 命令归档完成的功能。

## 实现产出参考
请基于Step 2的实现产出进行归档

## 输出
完成归档，更新相关文档"
opencode run -m "$MODEL" "$ARCHIVE_PROMPT"

echo ""
echo "========================================"
echo "OpenSpec workspace 实现完成!"
echo "========================================"
