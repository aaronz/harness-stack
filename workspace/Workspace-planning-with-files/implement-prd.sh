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
echo "Planning With Files workspace - PRD Implementation"
echo "方法论: 3-File Pattern (task_plan.md + findings.md + progress.md)"
echo "迭代: ${NEXT_ITERATION}"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

TASK_PLAN="$OUTPUT_DIR/task_plan.md"
FINDINGS="$OUTPUT_DIR/findings.md"
PROGRESS="$OUTPUT_DIR/progress.md"
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
echo "[1/5] PRD Gap Analysis - 差距分析..."
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
echo "[2/5] Initialize - 初始化3文件模式..."
INIT_PROMPT="请使用 /planning-with-files:plan 或 /plan 命令启动规划会话。

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 3-File模式
这将自动创建 3 个文件:
- $TASK_PLAN: 任务和进度跟踪
- $FINDINGS: 研究和发现
- $PROGRESS: 会话日志和测试结果

## 输出
请将 Requirements Document 中的需求写入这些文件作为持久化上下文"
opencode run -m "$MODEL" "$INIT_PROMPT"

rerun_if_missing "$TASK_PLAN" "$INIT_PROMPT"

echo ""
echo "[3/5] Research & Plan - 研究与规划..."
RESEARCH_PROMPT="继续使用 3-file 模式。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## 任务要求
基于 Requirements Document，在 $TASK_PLAN 中创建详细的任务分解

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 研究要求
使用 $FINDINGS 存储研究内容。每 2 个操作后保存 findings

## 要求
从 Requirements Document 提取核心需求并写入任务与研究计划"
opencode run -m "$MODEL" "$RESEARCH_PROMPT"

echo ""
echo "[4/5] Implement - 执行实现..."
IMPLEMENT_PROMPT="继续使用 3-file 模式实现。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 实现内容
基于 Requirements Document 提取实现范围并执行

## 进度更新
在 $TASK_PLAN 中更新进度(checkbox)

## 错误处理
错误必须记录在 $TASK_PLAN 以避免重复失败"
opencode run -m "$MODEL" "$IMPLEMENT_PROMPT"

echo ""
echo "[5/5] Verify - 验证完成..."
VERIFY_PROMPT="使用 3-file 模式的完成检查。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 验证要求
在 $TASK_PLAN 中验证所有 phases 完成，并根据 Requirements Document 验证交付结果

## 输出
更新 $PROGRESS 记录测试结果"
opencode run -m "$MODEL" "$VERIFY_PROMPT"

rerun_if_missing "$PROGRESS" "$VERIFY_PROMPT"

echo ""
echo "========================================"
echo "Planning With Files workspace 实现完成!"
echo "========================================"
echo ""
echo "产出文件:"
echo "  - Gap Analysis: $GAP_ANALYSIS"
echo "  - Task Plan: $TASK_PLAN"
echo "  - Findings: $FINDINGS"
echo "  - Progress: $PROGRESS"
