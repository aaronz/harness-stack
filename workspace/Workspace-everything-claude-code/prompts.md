# Everything Claude Code Step-by-Step Prompts

Execute these prompts in sequence to implement a PRD using the Everything Claude Code methodology.

---

## Prerequisites

- A `PRD.md` file in your workspace root
- OpenCode CLI installed (`opencode run -m <model> "<prompt>"`)

---

## Step 1: Plan - 创建实现计划

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /plan 命令创建实现计划。

## Requirements Document
$(cat PRD.md)

## 输出要求
请将完整的实施计划保存到: ./outputs/implementation-plan.md
确保计划包含:
- 模块划分及依赖关系
- 数据模型设计
- API接口定义
- 前端页面规划"
```

---

## Step 2: TDD - 测试驱动开发

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /tdd 命令执行测试驱动开发。

## 实施计划参考
$(cat outputs/implementation-plan.md)

## Requirements Document
$(cat PRD.md)

## 输出要求
1. 先创建测试文件（RED阶段），测试必须失败
2. 实现最小代码使测试通过（GREEN阶段）
3. 重构并确保测试覆盖（REFACTOR阶段）
4. 覆盖率需达到80%+

请将TDD产出保存到: ./outputs/tdd/"
```

---

## Step 3: Review - 代码审查与验证

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /code-review 命令审查代码质量。

## 实施计划
$(cat outputs/implementation-plan.md)

## TDD产出
./outputs/tdd/

## Requirements Document
$(cat PRD.md)

## 审查标准
结合需求文档检查代码质量、可维护性、可测试性与性能表现

## 输出
请将审查报告保存到: ./outputs/code-review-report.md"
```

---

## Step 4: Verify - 验证循环

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /verify 命令运行验证循环。

## 审查报告
$(cat outputs/code-review-report.md)

## Requirements Document
$(cat PRD.md)

## 验证重点
根据需求文档验证功能正确性、质量标准与性能要求

## 输出
请将验证报告保存到: ./outputs/verification-report.md"
```

---

## Step 5: Security Scan - 安全审计

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /security-scan 命令进行安全审计。

## 验证报告
$(cat outputs/verification-report.md)

## Requirements Document
$(cat PRD.md)

## 安全检查清单
- 无硬编码 secrets
- SQL注入防护
- XSS防护
- CSRF保护
- 认证授权验证
- 速率限制

## 输出
请将安全审计报告保存到: ./outputs/security-report.md"
```

---

## Quick Reference

| Step | Command | Output |
|------|---------|--------|
| 1 | /plan | outputs/implementation-plan.md |
| 2 | /tdd | outputs/tdd/ |
| 3 | /code-review | outputs/code-review-report.md |
| 4 | /verify | outputs/verification-report.md |
| 5 | /security-scan | outputs/security-report.md |

---

## Custom Model

Replace model in any step:
```bash
opencode run -m "anthropic/claude-3.5-sonnet" "<prompt>"
opencode run -m "opencode/llama-3.1-70b" "<prompt>"
```