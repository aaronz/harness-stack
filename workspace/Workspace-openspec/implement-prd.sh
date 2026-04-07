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
echo "OpenSpec workspace - PRD Implementation"
echo "方法论: /opsx:propose → /opsx:apply → /opsx:archive"
echo "迭代: ${NEXT_ITERATION}"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

PROPOSAL_DIR="$OUTPUT_DIR/proposal"
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
echo "[1/3] PRD Gap Analysis - 差距分析..."
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
echo "[2/3] Propose - 创建需求提案..."
PROPOSE_PROMPT="请使用 /opsx:propose ai-ready-evaluator 命令基于 PRD.md 创建需求提案。

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 输出要求
请将产出保存到: $PROPOSAL_DIR
- proposal.md: 需求提案
- specs/: 详细规格
- design.md: 设计文档
- tasks.md: 任务清单"
opencode run -m "$MODEL" "$PROPOSE_PROMPT"

rerun_if_missing "$PROPOSAL_DIR/proposal.md" "$PROPOSE_PROMPT"

echo ""
echo "[3/3] Apply - 执行实现 + Archive..."
APPLY_PROMPT="请使用 /opsx:apply 命令执行 ai-ready-evaluator 的实现。

## 提案产出参考
$PROPOSAL_DIR

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 输出要求
将实现代码保存到项目目录，确保与proposal中的设计一致"
opencode run -m "$MODEL" "$APPLY_PROMPT"

echo ""
echo "========================================"
echo "OpenSpec workspace 实现完成!"
echo "========================================"
echo ""
echo "产出文件:"
echo "  - Gap Analysis: $GAP_ANALYSIS"
echo "  - Proposal: $PROPOSAL_DIR/"
