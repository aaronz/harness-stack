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
echo "Everything Claude Code workspace - PRD Implementation"
echo "方法论: /plan → /tdd → /code-review → /verify → /security-scan"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

OUTPUT_DIR="$WORKSPACE_DIR/outputs"
mkdir -p "$OUTPUT_DIR"

PLAN_FILE="$OUTPUT_DIR/implementation-plan.md"
TDD_OUTPUT_DIR="$OUTPUT_DIR/tdd"
REVIEW_REPORT="$OUTPUT_DIR/code-review-report.md"
VERIFY_REPORT="$OUTPUT_DIR/verification-report.md"
SECURITY_REPORT="$OUTPUT_DIR/security-report.md"

echo "[Step 1/5] Plan - 创建实现计划..."
PLAN_PROMPT="请使用 /plan 命令创建 AI Coding可落地性评估系统的实现计划。

## PRD文件参考
$PRD_FILE

## 系统要求（从PRD提取的核心要求）
1. **评估维度模型**: 上下文完备性(25%)、逻辑原子性(25%)、边界明确性(20%)、可验证性(15%)、技术约束清晰度(15%)
2. **评分算法**: AI就绪分 = Σ(维度得分 × 维度权重) × 复杂度惩罚系数
3. **核心模块: 评估引擎、评分计算器、报告生成器**
4. **成功指标**:
   - AI代码采纳率 > 80%
   - 评估效率 < 2分钟

## 输出要求
请将完整的实施计划保存到: $PLAN_FILE
确保计划包含:
- 模块划分及依赖关系
- 数据模型设计
- API接口定义
- 前端页面规划"
opencode run -m "$MODEL" "$PLAN_PROMPT"

echo ""
echo "[Step 2/5] TDD - 测试驱动开发..."
TDD_PROMPT="请使用 /tdd 命令执行测试驱动开发。

## 实施计划参考
$PLAN_FILE

## PRD核心要求
- 评估维度: 上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束清晰度
- 评分计算: 加权求和 + 复杂度惩罚
- 成功指标: AI代码采纳率>80%、评估效率<2分钟

## 输出要求
1. 先创建测试文件（RED阶段），测试必须失败
2. 实现最小代码使测试通过（GREEN阶段）
3. 重构并确保测试覆盖（REFACTOR阶段）
4. 覆盖率需达到80%+

请将TDD产出保存到: $TDD_OUTPUT_DIR"
opencode run -m "$MODEL" "$TDD_PROMPT"

echo ""
echo "[Step 3/5] Review - 代码审查与验证..."
REVIEW_PROMPT="请使用 /code-review 命令审查代码质量。

## 实施计划
$PLAN_FILE

## TDD产出
$TDD_OUTPUT_DIR

## 审查标准（来自PRD成功指标）
- AI代码采纳率 > 80%（代码需符合可测试性、可维护性）
- 评估效率 < 2分钟（性能优化检查）

## 输出
请将审查报告保存到: $REVIEW_REPORT"
opencode run -m "$MODEL" "$REVIEW_PROMPT"

echo ""
echo "[Step 4/5] Verify - 验证循环..."
VERIFY_PROMPT="请使用 /verify 命令运行验证循环。

## 审查报告
$REVIEW_REPORT

## PRD成功指标验证
1. **AI代码采纳率>80%**: 代码质量是否足够清晰、无歧义
2. **评估效率<2分钟**: 性能是否满足要求

## 输出
请将验证报告保存到: $VERIFY_REPORT"
opencode run -m "$MODEL" "$VERIFY_PROMPT"

echo ""
echo "[Step 5/5] Security Scan - 安全审计..."
SECURITY_PROMPT="请使用 /security-scan 命令进行安全审计。

## 验证报告
$VERIFY_REPORT

## PRD要求的安全检查
- 无硬编码 secrets
- SQL注入防护
- XSS防护
- CSRF保护
- 认证授权验证
- 速率限制

## 输出
请将安全审计报告保存到: $SECURITY_REPORT"
opencode run -m "$MODEL" "$SECURITY_PROMPT"

echo ""
echo "========================================"
echo "Everything Claude Code workspace 实现完成!"
echo "========================================"
echo ""
echo "产出文件:"
echo "  - $PLAN_FILE"
echo "  - $TDD_OUTPUT_DIR/"
echo "  - $REVIEW_REPORT"
echo "  - $VERIFY_REPORT"
echo "  - $SECURITY_REPORT"
