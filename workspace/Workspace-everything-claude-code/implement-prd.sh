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
echo "Everything Claude Code Workspace - PRD Implementation"
echo "方法论: /plan → /tdd → /code-review → /verify → /security-scan"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

echo "[Step 1/4] Plan - 创建实现计划..."
opencode run -m "$MODEL" "请使用 /plan 命令创建 AI Coding可落地性评估系统的实现计划。PRD文件: $PRD_FILE。使用 planner agent 分解功能: 评估维度模型(上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束清晰度)、评分算法、报告生成器。输出: implementation blueprint"

echo ""
echo "[Step 2/4] TDD - 测试驱动开发..."
opencode run -m "$MODEL" "请使用 /tdd 命令执行测试驱动开发。使用 tdd-guide agent 强制 RED-GREEN-REFACTOR 循环。实现内容: 评估引擎模块、评分计算器、报告生成器。每个功能先写失败测试，再写最小实现。覆盖率需达到80%+"

echo ""
echo "[Step 3/4] Review & Verify - 代码审查与验证..."
opencode run -m "$MODEL" "请使用 /code-review 命令审查代码质量。然后使用 /verify 命令运行验证循环(build、test、lint、typecheck、security)。确保满足PRD成功指标: AI代码采纳率>80%、评估效率<2分钟"

echo ""
echo "[Step 4/4] Security Scan - 安全审计..."
opencode run -m "$MODEL" "请使用 /security-scan 命令进行安全审计。检查: 无硬编码 secrets、SQL注入防护、XSS防护、CSRF保护、认证授权验证、速率限制。确保生产就绪"

echo ""
echo "========================================"
echo "Everything Claude Code Workspace 实现完成!"
echo "========================================"