# My Haness - AI Coding Workspace Framework

基于 opencode CLI 实现 PRD.md 需求的多方法论实现框架。

## 目录结构

```
myhaness/
├── PRD.md                          # 需求文档
├── setup-projects.sh/ps1           # 初始化项目脚本
├── run-all-workspaces.sh/ps1       # 批量运行所有 workspace
├── skill-source/                   # 克隆的源码仓库
│   ├── openspec/
│   ├── speckit/
│   ├── superpowers/
│   ├── everything-claude-code/
│   ├── planning-with-files/
│   └── gstack/
└── workspace/                      # 工作区
    ├── Workspace-openspec/
    ├── Workspace-speckit/
    ├── Workspace-superpowers/
    ├── Workspace-everything-claude-code/
    ├── Workspace-planning-with-files/
    └── Workspace-gstack/
```

## 方法论对比

| Workspace | 方法论 | 核心命令/技能 |
|-----------|--------|--------------|
| **openspec** | Propose → Apply → Archive | `/opsx:propose`, `/opsx:apply`, `/opsx:archive` |
| **speckit** | SDD Workflow | `/speckit.constitution`, `/speckit.specify`, `/speckit.plan`, `/speckit.tasks`, `/speckit.implement` |
| **superpowers** | Full Development Cycle | `brainstorming`, `writing-plans`, `subagent-driven-development`, `verification-before-completion`, `finishing-a-development-branch` |
| **everything-claude-code** | Agent Commands | `/plan`, `/tdd`, `/code-review`, `/verify`, `/security-scan` |
| **planning-with-files** | 3-File Pattern | `task_plan.md` + `findings.md` + `progress.md` |
| **gstack** | Sprint Workflow | `/office-hours`, `/plan-ceo-review`, `/plan-eng-review`, `/review`, `/qa`, `/ship`, `/retro` |

## 快速开始

### 1. 初始化项目

```bash
# Bash
./setup-projects.sh

# PowerShell
.\setup-projects.ps1
```

### 2. 运行单个 Workspace

```bash
# Bash
cd workspace/Workspace-openspec
./implement-prd.sh

# 指定模型
./implement-prd.sh openai/gpt-4.5

# PowerShell
cd workspace\Workspace-openspec
.\implement-prd.ps1
.\implement-prd.ps1 openai/gpt-4.5
```

### 3. 运行所有 Workspaces

```bash
# Bash
./run-all-workspaces.sh

# 指定模型
./run-all-workspaces.sh openai/gpt-4.5

# PowerShell
.\run-all-workspaces.ps1
.\run-all-workspaces.ps1 openai/gpt-4.5
```

## 脚本说明

### 实现脚本

每个 workspace 包含 `implement-prd.sh` 和 `implement-prd.ps1`，基于该 workspace 的方法论执行需求实现。

**参数:**
- `$1` - 模型名称 (默认: `minimax-m2.5-free`)

**示例:**
```bash
./implement-prd.sh minimax-m2.5-free
```

### 批量运行脚本

`run-all-workspaces.sh/ps1` 依次运行所有 workspace 实现脚本。

**参数:**
- `$1` - 模型名称 (默认: `minimax-m2.5-free`)

### 初始化脚本

`setup-projects.sh/ps1` 克隆所有源码仓库并创建 workspace 结构。

**功能:**
- 克隆 6 个源码仓库到 `skill-source/`
- 复制 skills、agents、commands 到各 workspace `.opencode/` 目录
- 初始化 git 仓库

## Workspace 详细

### OpenSpec

```
方法论: /opsx:propose → /opsx:apply → /opsx:archive
步骤: 3
```

创建需求提案 → 执行实现 → 归档完成

### Spec Kit

```
方法论: /speckit.constitution → /speckit.specify → /speckit.plan → /speckit.tasks → /speckit.implement
步骤: 5
```

建立项目原则 → 定义需求规范 → 创建技术计划 → 生成任务清单 → 执行实现

### Superpowers

```
方法论: brainstorming → writing-plans → subagent-driven-development → verification → finishing
步骤: 5
```

需求理解与设计 → 创建实现计划 → 执行实现 → 验证 → 完成开发

### Everything Claude Code

```
方法论: /plan → /tdd → /code-review → /verify → /security-scan
步骤: 4
```

创建实现计划 → 测试驱动开发 → 代码审查与验证 → 安全审计

### Planning With Files

```
方法论: 3-File Pattern (task_plan.md + findings.md + progress.md)
步骤: 4
```

初始化3文件模式 → 研究与规划 → 执行实现 → 验证完成

### GStack

```
方法论: Think → Plan → Build → Review → Test → Ship → Reflect
步骤: 7
```

需求理解 → CEO级审查 → 工程审查 → 实现 → 代码审查 → 测试与发布 → 回顾

## 模型参数

所有脚本支持通过 `-m` 参数指定模型:

```bash
opencode run -m "openai/gpt-4.5" "你的任务"
```

默认模型: `minimax-m2.5-free`

## 依赖

- [opencode CLI](https://github.com/opencode-ai/opencode)
- Git
- Bash (Linux/macOS) 或 PowerShell (Windows)