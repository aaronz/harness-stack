# Planning With Files Step-by-Step Prompts

Execute these prompts in sequence to implement a PRD using the Planning With Files methodology.

---

## Prerequisites

- A `PRD.md` file in your workspace root
- OpenCode CLI installed (`opencode run -m <model> "<prompt>"`)

---

## Step 1: Initialize - 初始化3文件模式

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /planning-with-files:plan 或 /plan 命令启动规划会话。

## Requirements Document
$(cat PRD.md)

## 3-File模式
这将自动创建 3 个文件:
- ./outputs/task_plan.md: 任务和进度跟踪
- ./outputs/findings.md: 研究和发现
- ./outputs/progress.md: 会话日志和测试结果

## 输出
请将 Requirements Document 中的需求写入这些文件作为持久化上下文"
```

---

## Step 2: Research & Plan - 研究与规划

```bash
opencode run -m "opencode/minimax-m2.5-free" "继续使用 3-file 模式。

## 3个文件
- ./outputs/task_plan.md
- ./outputs/findings.md
- ./outputs/progress.md

## 任务要求
基于 Requirements Document，在 task_plan.md 中创建详细的任务分解

## Requirements Document
$(cat PRD.md)

## 研究要求
使用 findings.md 存储研究内容。每 2 个操作后保存 findings

## 要求
从 Requirements Document 提取核心需求并写入任务与研究计划"
```

---

## Step 3: Implement - 执行实现

```bash
opencode run -m "opencode/minimax-m2.5-free" "继续使用 3-file 模式实现。

## 3个文件
- ./outputs/task_plan.md
- ./outputs/findings.md
- ./outputs/progress.md

## Requirements Document
$(cat PRD.md)

## 实现内容
基于 Requirements Document 提取实现范围并执行

## 进度更新
在 task_plan.md 中更新进度(checkbox)

## 错误处理
错误必须记录在 task_plan.md 以避免重复失败"
```

---

## Step 4: Verify - 验证完成

```bash
opencode run -m "opencode/minimax-m2.5-free" "使用 3-file 模式的完成检查。

## 3个文件
- ./outputs/task_plan.md
- ./outputs/findings.md
- ./outputs/progress.md

## Requirements Document
$(cat PRD.md)

## 验证要求
在 task_plan.md 中验证所有 phases 完成，并根据 Requirements Document 验证交付结果

## 输出
更新 progress.md 记录测试结果"
```

---

## Quick Reference

| Step | Output |
|------|--------|
| 1 | outputs/task_plan.md, outputs/findings.md, outputs/progress.md |
| 2 | (updated files) |
| 3 | (code changes) |
| 4 | (verified) |

---

## Custom Model

Replace model in any step:
```bash
opencode run -m "anthropic/claude-3.5-sonnet" "<prompt>"
opencode run -m "opencode/llama-3.1-70b" "<prompt>"
```