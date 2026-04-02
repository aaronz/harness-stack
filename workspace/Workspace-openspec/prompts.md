# OpenSpec Step-by-Step Prompts

Execute these prompts in sequence to implement a PRD using the OpenSpec methodology.

---

## Prerequisites

- A `PRD.md` file in your workspace root
- OpenCode CLI installed (`opencode run -m <model> "<prompt>"`)

---

## Step 1: Propose - 创建需求提案

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /opsx:propose ai-ready-evaluator 命令基于 PRD.md 创建需求提案。

## Requirements Document
$(cat PRD.md)

## 输出要求
请将产出保存到: ./outputs/proposal
- proposal.md: 需求提案
- specs/: 详细规格
- design.md: 设计文档
- tasks.md: 任务清单"
```

---

## Step 2: Apply - 执行实现

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /opsx:apply 命令执行 ai-ready-evaluator 的实现。

## 提案产出参考
./outputs/proposal

## Requirements Document
$(cat PRD.md)

## 输出要求
将实现代码保存到项目目录，确保与proposal中的设计一致"
```

---

## Step 3: Archive - 归档

```bash
opencode run -m "opencode/minimax-m2.5-free" "请使用 /opsx:archive 命令归档完成的功能。

## 实现产出参考
请基于Step 2的实现产出进行归档

## 输出
完成归档，更新相关文档"
```

---

## Quick Reference

| Step | Command | Output |
|------|---------|--------|
| 1 | /opsx:propose | outputs/proposal/ (proposal.md, specs/, design.md, tasks.md) |
| 2 | /opsx:apply | (code changes) |
| 3 | /opsx:archive | (archived) |

---

## Custom Model

Replace model in any step:
```bash
opencode run -m "anthropic/claude-3.5-sonnet" "<prompt>"
opencode run -m "opencode/llama-3.1-70b" "<prompt>"
```