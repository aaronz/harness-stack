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
echo "Planning With Files workspace - PRD Implementation"
echo "方法论: 3-File Pattern (task_plan.md + findings.md + progress.md)"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

OUTPUT_DIR="$WORKSPACE_DIR/outputs"
mkdir -p "$OUTPUT_DIR"

TASK_PLAN="$OUTPUT_DIR/task_plan.md"
FINDINGS="$OUTPUT_DIR/findings.md"
PROGRESS="$OUTPUT_DIR/progress.md"

echo "[Step 1/4] Initialize - 初始化3文件模式..."
INIT_PROMPT="请使用 /planning-with-files:plan 或 /plan 命令启动规划会话。

## PRD文件
$PRD_FILE

## 项目
AI Coding可落地性评估系统

## 核心需求
- 评估维度: 上下文完备性(25%)、逻辑原子性(25%)、边界明确性(20%)、可验证性(15%)、技术约束清晰度(15%)
- 评分算法: AI就绪分 = Σ(维度得分×权重) × 复杂度惩罚系数
- 成功指标: AI代码采纳率>80%、评估效率<2分钟

## 3-File模式
这将自动创建 3 个文件:
- $TASK_PLAN: 任务和进度跟踪
- $FINDINGS: 研究和发现
- $PROGRESS: 会话日志和测试结果

## 输出
请将AI Coding可落地性评估系统需求写入这些文件作为持久化上下文"
opencode run -m "$MODEL" "$INIT_PROMPT"

echo ""
echo "[Step 2/4] Research & Plan - 研究与规划..."
RESEARCH_PROMPT="继续使用 3-file 模式。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## 任务要求
在 $TASK_PLAN 中创建详细的任务分解:
1) 评估维度模型实现
2) 评分算法
3) 报告生成器
4) 风险热力图

## 研究要求
使用 $FINDINGS 存储研究内容。每 2 个操作后保存 findings

## PRD核心要求
- 五大评估维度
- S/A/B/C等级
- 风险热力图"
opencode run -m "$MODEL" "$RESEARCH_PROMPT"

echo ""
echo "[Step 3/4] Implement - 执行实现..."
IMPLEMENT_PROMPT="继续使用 3-file 模式实现。

## 3个文件
- $TASK_PLAN
- $FINDINGS
- $PROGRESS

## 实现内容
- 评估维度模型(上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束)
- 评分算法(S/A/B/C等级)
- 报告生成器
- 风险热力图

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

## 验证要求
在 $TASK_PLAN 中验证所有 phases 完成

## PRD成功指标
- AI代码采纳率>80%
- 返工率降低50%
- 评估效率<2分钟

## 输出
更新 $PROGRESS 记录测试结果"
opencode run -m "$MODEL" "$VERIFY_PROMPT"

echo ""
echo "========================================"
echo "Planning With Files workspace 实现完成!"
echo "========================================"
