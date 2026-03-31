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
echo "Spec Kit Workspace - PRD Implementation"
echo "方法论: /speckit.constitution → /speckit.specify → /speckit.plan → /speckit.tasks → /speckit.implement"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

echo "[Step 1/5] Constitution - 建立项目原则..."
opencode run -m "$MODEL" "请使用 /speckit.constitution 命令创建项目开发原则。原则应聚焦于: 代码质量标准、测试覆盖率要求(80%+)、用户体验一致性、性能要求。确保这些原则指导所有后续技术决策。"

echo ""
echo "[Step 2/5] Specify - 定义需求规范..."
opencode run -m "$MODEL" "请使用 /speckit.specify 命令基于 PRD.md 定义需求规范。PRD内容: AI Coding可落地性评估系统。核心要点: 五大评估维度(上下文完备性25%、逻辑原子性25%、边界明确性20%、可验证性15%、技术约束清晰度15%)、评分算法、AI就绪等级(S/A/B/C级)"

echo ""
echo "[Step 3/5] Plan - 创建技术实现计划..."
opencode run -m "$MODEL" "请使用 /speckit.plan 命令创建技术实现计划。技术栈: 根据现有项目选择。架构: 模块化设计。计划应包含: 评估引擎模块、评分计算器、报告生成器、数据模型设计"

echo ""
echo "[Step 4/5] Tasks - 生成任务清单..."
opencode run -m "$MODEL" "请使用 /speckit.tasks 命令生成可执行的任务清单。从技术计划中分解出具体的开发任务，每个任务应有明确的验收标准"

echo ""
echo "[Step 5/5] Implement - 执行实现..."
opencode run -m "$MODEL" "请使用 /speckit.implement 命令执行所有任务。实现内容: 评估维度模型、评分算法、S/A/B/C等级报告生成器、风险热力图功能"

echo ""
echo "========================================"
echo "Spec Kit Workspace 实现完成!"
echo "========================================"