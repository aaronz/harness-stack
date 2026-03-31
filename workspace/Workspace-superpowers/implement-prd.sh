#!/bin/bash
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WORKSPACE_DIR="$(dirname "$SCRIPT_DIR")"
PRD_FILE="$WORKSPACE_DIR/../PRD.md"

MODEL="${1:-minimax-m2.5-free}"

if [ ! -f "$PRD_FILE" ]; then
    echo "Error: PRD.md not found at $PRD_FILE"
    exit 1
fi

echo "========================================"
echo "Superpowers Workspace - PRD Implementation"
echo "方法论: brainstorming → writing-plans → subagent-driven-development → verification → finishing"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

echo "[Step 1/5] Brainstorming - 需求理解与设计..."
opencode run -m "$MODEL" "请使用 brainstorming skill 分析 PRD.md 中的 AI Coding可落地性评估系统需求。文件位置: $PRD_FILE。流程: 1)探索项目上下文 2)提出视觉辅助(如有UI问题) 3)提出澄清问题 4)提出2-3个方案及权衡 5)展示设计sections获取批准。输出: design doc 保存到 docs/superpowers/specs/"

echo ""
echo "[Step 2/5] Writing Plans - 创建实现计划..."
opencode run -m "$MODEL" "请使用 writing-plans skill 基于已批准的设计创建详细实现计划。计划需分解为2-5分钟可完成的原子任务，每个任务有精确文件路径、完整代码、验证步骤。使用 subagent-driven-development skill 进行任务分解"

echo ""
echo "[Step 3/5] Subagent-Driven Development - 执行实现..."
opencode run -m "$MODEL" "请使用 subagent-driven-development skill 执行实现计划。每个任务由fresh subagent执行，两阶段review(规范合规性→代码质量)。实现内容: 评估维度模型、评分算法、S/A/B/C报告生成器、风险热力图"

echo ""
echo "[Step 4/5] Verification - 验证..."
opencode run -m "$MODEL" "请使用 verification-before-completion skill 进行最终验证。验证要点: 功能完整性、代码质量、测试覆盖(80%+)。确保PRD中的成功指标满足: AI代码采纳率>80%、返工率降低50%、评估效率<2分钟"

echo ""
echo "[Step 5/5] Finishing - 完成开发..."
opencode run -m "$MODEL" "请使用 finishing-a-development-branch skill 完成开发。验证测试通过，展示选项(merge/PR/keep/discard)，清理worktree"

echo ""
echo "========================================"
echo "Superpowers Workspace 实现完成!"
echo "========================================"