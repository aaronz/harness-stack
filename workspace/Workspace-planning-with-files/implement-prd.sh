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
echo "Planning With Files workspace - PRD Implementation"
echo "方法论: 3-File Pattern (task_plan.md + findings.md + progress.md)"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

OUTPUT_DIR="$SCRIPT_DIR/outputs"
mkdir -p "$OUTPUT_DIR"

TASK_PLAN="$OUTPUT_DIR/task_plan.md"
FINDINGS="$OUTPUT_DIR/findings.md"
PROGRESS="$OUTPUT_DIR/progress.md"

echo "[Step 1/4] Initialize - 初始化3文件模式..."
INIT_PROMPT="请使用 /planning-with-files:plan 或 /plan 命令启动规划会话。

## Requirements Document
$PRD_CONTENT

## 3-File模式
这将自动创建 3 个文件:
- $TASK_PLAN: 任务和进度跟踪
- $FINDINGS: 研究和发现
- $PROGRESS: 会话日志和测试结果

## 输出
请将 Requirements Document 中的需求写入这些文件作为持久化上下文"
opencode run -m "$MODEL" "$INIT_PROMPT"

echo ""
echo "[Step 2/4] Research & Plan - 研究与规划..."
RESEARCH_PROMPT="继续使用 3-file 模式。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## 任务要求
基于 Requirements Document，在 $TASK_PLAN 中创建详细的任务分解

## Requirements Document
$PRD_CONTENT

## 研究要求
使用 $FINDINGS 存储研究内容。每 2 个操作后保存 findings

## 要求
从 Requirements Document 提取核心需求并写入任务与研究计划"
opencode run -m "$MODEL" "$RESEARCH_PROMPT"

echo ""
echo "[Step 3/4] Implement - 执行实现..."
IMPLEMENT_PROMPT="继续使用 3-file 模式实现。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## Requirements Document
$PRD_CONTENT

## 实现内容
基于 Requirements Document 提取实现范围并执行

## 进度更新
在 $TASK_PLAN 中更新进度(checkbox)

## 错误处理
错误必须记录在 $TASK_PLAN 以避免重复失败"
opencode run -m "$MODEL" "$IMPLEMENT_PROMPT"

echo ""
echo "[Step 4/4] Verify - 验证完成..."
VERIFY_PROMPT="使用 3-file 模式的完成检查。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## Requirements Document
$PRD_CONTENT

## 验证要求
在 $TASK_PLAN 中验证所有 phases 完成，并根据 Requirements Document 验证交付结果

## 输出
更新 $PROGRESS 记录测试结果"
opencode run -m "$MODEL" "$VERIFY_PROMPT"

echo ""
echo "========================================"
echo "Planning With Files workspace 实现完成!"
echo "========================================"
