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
echo "Everything Claude Code workspace - PRD Implementation"
echo "方法论: /plan → /tdd → /code-review → /verify → /security-scan"
echo "迭代: ${NEXT_ITERATION}"
echo "模型: $MODEL"
echo "========================================"
echo "PRD: $PRD_FILE"
echo ""

PLAN_FILE="$OUTPUT_DIR/implementation-plan.md"
TDD_OUTPUT_DIR="$OUTPUT_DIR/tdd"
REVIEW_REPORT="$OUTPUT_DIR/code-review-report.md"
VERIFY_REPORT="$OUTPUT_DIR/verification-report.md"
SECURITY_REPORT="$OUTPUT_DIR/security-report.md"
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
echo "[1/6] PRD Gap Analysis - 差距分析..."
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
echo "[2/6] Plan - 创建实现计划..."
PLAN_PROMPT="请使用 /plan 命令创建实现计划。

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 输出要求
请将完整的实施计划保存到: $PLAN_FILE
确保计划包含:
- 模块划分及依赖关系
- 数据模型设计
- API接口定义
- 前端页面规划"
opencode run -m "$MODEL" "$PLAN_PROMPT"

rerun_if_missing "$PLAN_FILE" "$PLAN_PROMPT"

echo ""
echo "[3/6] TDD - 测试驱动开发..."
TDD_PROMPT="请使用 /tdd 命令执行测试驱动开发。

## 实施计划参考
$PLAN_FILE

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 输出要求
1. 先创建测试文件（RED阶段），测试必须失败
2. 实现最小代码使测试通过（GREEN阶段）
3. 重构并确保测试覆盖（REFACTOR阶段）
4. 覆盖率需达到80%+

请将TDD产出保存到: $TDD_OUTPUT_DIR"
opencode run -m "$MODEL" "$TDD_PROMPT"

echo ""
echo "[4/6] Review - 代码审查与验证..."
REVIEW_PROMPT="请使用 /code-review 命令审查代码质量。

## 实施计划
$PLAN_FILE

## TDD产出
$TDD_OUTPUT_DIR

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 审查标准
结合需求文档检查代码质量、可维护性、可测试性与性能表现

## 输出
请将审查报告保存到: $REVIEW_REPORT"
opencode run -m "$MODEL" "$REVIEW_PROMPT"

rerun_if_missing "$REVIEW_REPORT" "$REVIEW_PROMPT"

echo ""
echo "[5/6] Verify - 验证循环..."
VERIFY_PROMPT="请使用 /verify 命令运行验证循环。

## 审查报告
$REVIEW_REPORT

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 验证重点
根据需求文档验证功能正确性、质量标准与性能要求

## 输出
请将验证报告保存到: $VERIFY_REPORT"
opencode run -m "$MODEL" "$VERIFY_PROMPT"

rerun_if_missing "$VERIFY_REPORT" "$VERIFY_PROMPT"

echo ""
echo "[6/6] Security Scan - 安全审计..."
SECURITY_PROMPT="请使用 /security-scan 命令进行安全审计。

## 验证报告
$VERIFY_REPORT

## Requirements Document
$PRD_CONTENT

## 差距分析
$(cat $GAP_ANALYSIS)

## 安全检查清单
- 无硬编码 secrets
- SQL注入防护
- XSS防护
- CSRF保护
- 认证授权验证
- 速率限制

## 输出
请将安全审计报告保存到: $SECURITY_REPORT"
opencode run -m "$MODEL" "$SECURITY_PROMPT"

rerun_if_missing "$SECURITY_REPORT" "$SECURITY_PROMPT"

echo ""
echo "========================================"
echo "Everything Claude Code workspace 实现完成!"
echo "========================================"
echo ""
echo "产出文件:"
echo "  - Gap Analysis: $GAP_ANALYSIS"
echo "  - $PLAN_FILE"
echo "  - $TDD_OUTPUT_DIR/"
echo "  - $REVIEW_REPORT"
echo "  - $VERIFY_REPORT"
echo "  - $SECURITY_REPORT"
