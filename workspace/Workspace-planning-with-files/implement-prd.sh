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
echo "Planning With Files Workspace - PRD Implementation"
echo "方法论: 3-File Pattern (task_plan.md + findings.md + progress.md)"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

echo "[Step 1/4] Initialize - 初始化3文件模式..."
opencode run -m "$MODEL" "请使用 /planning-with-files:plan 或 /plan 命令启动规划会话。这将自动创建 3 个文件: task_plan.md(任务和进度跟踪)、findings.md(研究和发现)、progress.md(会话日志和测试结果)。PRD文件: $PRD_FILE。AI Coding可落地性评估系统需求将写入这些文件作为持久化上下文"

echo ""
echo "[Step 2/4] Research & Plan - 研究与规划..."
opencode run -m "$MODEL" "继续使用 3-file 模式。在 task_plan.md 中创建详细的任务分解: 1)评估维度模型实现 2)评分算法 3)报告生成器 4)风险热力图。使用 findings.md 存储研究内容。每 2 个操作后保存 findings"

echo ""
echo "[Step 3/4] Implement - 执行实现..."
opencode run -m "$MODEL" "继续使用 3-file 模式实现。在 task_plan.md 中更新进度(checkbox)。实现: 评估维度模型(上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束)、评分算法(S/A/B/C等级)、报告生成器。错误必须记录在 task_plan.md 以避免重复失败"

echo ""
echo "[Step 4/4] Verify - 验证完成..."
opencode run -m "$MODEL" "使用 3-file 模式的完成检查。在 task_plan.md 中验证所有 phases 完成。确保满足 PRD 成功指标: AI代码采纳率>80%、返工率降低50%、评估效率<2分钟。更新 progress.md 记录测试结果"

echo ""
echo "========================================"
echo "Planning With Files Workspace 实现完成!"
echo "========================================"