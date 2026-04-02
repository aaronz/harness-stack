# Superpowers Step-by-Step Prompts

Execute these prompts in sequence to implement a PRD using the Superpowers methodology.

---

## Prerequisites

- A `PRD.md` file in your workspace root
- OpenCode CLI installed (`opencode run -m <model> "<prompt>"`)

---

## Step 1: Brainstorming - 需求理解与设计

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 brainstorming skill 分析 PRD.md 中的需求。

## Requirements Document
$(cat PRD.md)

## 流程
1) 探索项目上下文
2) 提出视觉辅助(如有UI问题)
3) 提出澄清问题
4) 提出2-3个方案及权衡
5) 展示设计sections获取批准

## 输出
请将设计文档保存到: ./outputs/design.md"
```

### Manual Alternative (without bash):
```bash
# Read PRD first
cat PRD.md

# Then run:
opencode run -m "opencode/minimax-m2.5-free" "请使用 brainstorming skill 分析以下需求文档..."
```

---

## Step 2: Writing Plans - 创建实现计划

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 writing-plans skill 基于已批准的设计创建详细实现计划。

## 设计文档
$(cat outputs/design.md)

## Requirements Document
$(cat PRD.md)

## 计划要求
- 分解为2-5分钟可完成的原子任务
- 每个任务有精确文件路径、完整代码、验证步骤
- 使用 subagent-driven-development skill 进行任务分解

## 输出
请将计划保存到: ./outputs/plan.md"
```

---

## Step 3: Subagent-Driven Development - 执行实现

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 subagent-driven-development skill 执行实现计划。

## 实现计划
$(cat outputs/plan.md)

## Requirements Document
$(cat PRD.md)

## 执行要求
每个任务由fresh subagent执行，两阶段review(规范合规性→代码质量)"
```

---

## Step 4: Verification - 验证

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 verification-before-completion skill 进行最终验证。

## 实现产出
请验证Step 3的实现产出

## Requirements Document
$(cat PRD.md)

## 验证要点
- 功能完整性
- 代码质量
- 测试覆盖(80%+)

## 输出
请将验证报告保存到: ./outputs/verification-report.md"
```

---

## Step 5: Finishing - 完成开发

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 finishing-a-development-branch skill 完成开发。

## 验证报告
$(cat outputs/verification-report.md)

## 完成要求
验证测试通过，展示选项(merge/PR/keep/discard)，清理worktree"
```

---

## Quick Reference

| Step | Skill | Output File |
|------|-------|-------------|
| 1 | brainstorming | outputs/design.md |
| 2 | writing-plans | outputs/plan.md |
| 3 | subagent-driven-development | (code changes) |
| 4 | verification-before-completion | outputs/verification-report.md |
| 5 | finishing-a-development-branch | (git operations) |

---

## Custom Model

Replace model in any step:
```bash
opencode run -m "anthropic/claude-3.5-sonnet" "<prompt>"
opencode run -m "opencode/llama-3.1-70b" "<prompt>"
```