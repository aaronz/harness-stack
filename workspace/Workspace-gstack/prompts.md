# GStack Step-by-Step Prompts

Execute these prompts in sequence to implement a PRD using the GStack methodology.

---

## Prerequisites

- A `PRD.md` file in your workspace root
- OpenCode CLI installed (`opencode run -m <model> "<prompt>"`)

---

## Step 1: Office Hours - 需求理解

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /office-hours 命令开始。

## Requirements Document
$(cat PRD.md)

## 流程
将提出6个强制问题来重新审视产品，挑战前提，生成多种实现方案

## 输出
请将设计文档保存到: ./outputs/design.md"
```

---

## Step 2: Plan CEO Review - CEO级审查

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /plan-ceo-review 命令进行CEO级审查。

## 设计文档
$(cat outputs/design.md)

## 审查要求
重新思考问题，找到10星产品。4种模式: Expansion, Selective Expansion, Hold Scope, Reduction。10章节审查

## 输出
请将审查结果保存到: ./outputs/ceo-review.md"
```

---

## Step 3: Plan Eng Review - 工程审查

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /plan-eng-review 命令进行工程审查。

## CEO审查
$(cat outputs/ceo-review.md)

## 审查要求
锁定架构、数据流、图表、边缘情况和测试。ASCII图、状态机、错误路径、测试矩阵、故障模式、安全问题

## 输出
请将工程审查结果保存到: ./outputs/eng-review.md"
```

---

## Step 4: Implement - 实现

```bash
opencode run -m "opencode/minimax-m2.5-free" "使用 gstack 的实现模式执行。

## 工程审查
$(cat outputs/eng-review.md)

## Requirements Document
$(cat PRD.md)"
```

---

## Step 5: Review - 代码审查

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /review 命令进行代码审查。

## 实现产出
请审查Step 4的产出

## 审查要求
找到通过CI但在生产中爆发的bug。AUTO-FIXED 明显问题，FLAGS 完整性差距

## 输出
请将审查报告保存到: ./outputs/code-review-report.md"
```

---

## Step 6: QA & Ship - 测试与发布

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /qa 命令测试你的应用，找到bug并修复。然后使用 /ship 命令同步main、运行测试、审计覆盖率、推送、打开PR。

## 代码审查报告
$(cat outputs/code-review-report.md)

## 要求
自动生成回归测试

## 输出
请将发布报告保存到: ./outputs/ship-report.md"
```

---

## Step 7: Retro - 回顾

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /retro 命令进行回顾。

## 发布报告
$(cat outputs/ship-report.md)

## 要求
每周总结: 人员分解、航运 streaks、测试健康趋势、增长机会"
```

---

## Quick Reference

| Step | Command | Output |
|------|---------|--------|
| 1 | /office-hours | outputs/design.md |
| 2 | /plan-ceo-review | outputs/ceo-review.md |
| 3 | /plan-eng-review | outputs/eng-review.md |
| 4 | (implementation) | (code changes) |
| 5 | /review | outputs/code-review-report.md |
| 6 | /qa + /ship | outputs/ship-report.md |
| 7 | /retro | (summary) |

---

## Custom Model

Replace model in any step:
```bash
opencode run -m "anthropic/claude-3.5-sonnet" "<prompt>"
opencode run -m "opencode/llama-3.1-70b" "<prompt>"
```