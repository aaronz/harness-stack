# Harness Stack - AI Coding Workspace Framework

基于 opencode CLI 实现 PRD.md 需求的多方法论实现框架。

## 目录结构

```
harness-stack/
├── PRD.md                          # 需求文档
├── setup-projects.sh              # 初始化项目脚本
├── install-methodology.sh         # 安装方法论脚本 (Bash)
├── install-methodology.ps1        # 安装方法论脚本 (PowerShell)
├── lib/                           # 共享脚本库
│   └── common.sh                 # 通用函数
├── skill-source/                 # 克隆的源码仓库
│   ├── openspec/
│   ├── speckit/
│   ├── superpowers/
│   ├── everything-claude-code/
│   ├── planning-with-files/
│   └── gstack/
└── workspace/                    # 工作区
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
./setup-projects.sh
```

### 2. 运行单个 Workspace

```bash
cd workspace/Workspace-openspec
./iterate-prd.sh

# 指定模型
./iterate-prd.sh --model opencode/minimax-m2.5-free

# 恢复迭代
./iterate-prd.sh --resume 2
```

## 脚本特性

### 迭代脚本 (iterate-prd.sh)

每个 workspace 的 `iterate-prd.sh` 包含以下特性：

- **迭代跟踪** - 输出到 `outputs/iteration-{N}/` 版本化目录
- **动态迭代** - 自动检测上次迭代号，无需硬编码
- **Checkpoint系统** - 每个阶段保存检查点，支持中断恢复
- **差距分析** - 首先分析 PRD 与当前实现的差距
- **进度格式** - `[1/5]`, `[2/5]`, etc.
- **Build验证** - 要求 "Build必须通过"
- **验证报告** - 包含 P0问题状态, PRD完整度, 遗留问题, 下一步建议

### 安装方法论脚本 (install-methodology.sh)

将指定方法论安装到目标项目文件夹：

```bash
# 基本用法
./install-methodology.sh <methodology> <target-folder>

# 选项
# -c, --clean    安装前清空 .opencode/skills、.opencode/commands 和脚本文件

# 示例
./install-methodology.sh superpowers ~/projects/my-app
./install-methodology.sh speckit ./my-workspace
./install-methodology.sh planning-with-files ../other-project

# 清理后安装（清空现有文件后再安装新方法论）
./install-methodology.sh --clean speckit ./my-workspace
./install-methodology.sh -c superpowers ~/projects/my-app
```

**可用方法论：**
- `openspec` - 使用 `openspec init` 初始化
- `speckit` - 使用 `specify init` 初始化
- `superpowers` - 复制 skills 和 commands
- `everything-claude-code` - 复制 skills 和 commands
- `planning-with-files` - 复制 skills 和 commands
- `gstack` - 复制 skills 和 commands

**安装内容：**
- **openspec/speckit**: 运行 CLI init 命令自动设置
- **其他方法论**: 复制 skills 到 `.opencode/skills/`，commands 到 `.opencode/commands/`
- **所有方法论**: 复制 `iterate-prd.sh`, `iterate-prd.ps1`, `prompts.md`

## Workspace 详细

### OpenSpec

```
方法论: Gap Analysis → Propose → Apply
步骤: 3
```

差距分析 → 创建需求提案 → 执行实现

### Spec Kit

```
方法论: Gap Analysis → Constitution → Specify → Plan → Tasks → Implement
步骤: 6
```

差距分析 → 建立项目原则 → 定义需求规范 → 创建技术计划 → 生成任务清单 → 执行实现

### Superpowers

```
方法论: Gap Analysis → Brainstorming → Plans → SDD → Verification
步骤: 5
```

差距分析 → 需求理解与设计 → 创建实现计划 → 执行实现 → 验证

### Everything Claude Code

```
方法论: Gap Analysis → Plan → TDD → Review → Verify → Security Scan
步骤: 6
```

差距分析 → 创建实现计划 → 测试驱动开发 → 代码审查与验��� → 安全审计

### Planning With Files

```
方法论: Gap Analysis → 3-File Init → Research → Implement → Verify
步骤: 5
```

差距分析 → 初始化3文件模式 → 研究与规划 → 执行实现 → 验证完成

### GStack

```
方法论: Gap Analysis → Office Hours → CEO Review → Eng Review → Implement → Review → QA → Retro
步骤: 8
```

差距分析 → 需求理解 → CEO级审查 → 工程审查 → 实现 → 代码审查 → 测试与发布 → 回顾

## 产出目录

每次运行会在 `outputs/iteration-{N}/` 下生成以下文件：

```
outputs/iteration-1/
├── gap-analysis.md          # PRD 差距分析
├── design.md             # 设计文档
├── plan.md              # 实现计划
├── spec.md              # 规格文档
├── tasks.md             # 任务清单
├── verification-report.md  # 验证报告
└── ...
```

## 模型参数

所有迭代脚本支持通过 `--model` 参数指定模型：

```bash
# 默认模型
./iterate-prd.sh

# 指定模型
./iterate-prd.sh --model opencode/minimax-m2.5-free

# 其他选项
./iterate-prd.sh --verbose --resume 2 --prd ./my-prd.md
```

默认模型: `opencode/minimax-m2.5-free`

## 依赖

- [opencode CLI](https://github.com/opencode-ai/opencode)
- Git
- Bash (Linux/macOS) / PowerShell (Windows)

## 前置要求

根据安装的方法论，可能需要以下工具：

### 通用依赖
- **Git** - 版本控制
- **Bash 4+** (Linux/macOS) 或 **PowerShell 5+** (Windows)

### openspec 方法论
- **Node.js 20.19.0+**
- **npm**

安装:
```bash
npm install -g @fission-ai/openspec
```

### speckit 方法论
- **Python 3.11+**
- **uv** (推荐) 或 **pip** 或 **pipx**

推荐安装:
```bash
# 使用 uv (推荐)
curl -LsSf https://astral.sh/uv/install.sh | sh
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git

# 或使用 pipx
pipx install specify-cli

# 或使用 pip
pip install specify-cli
```

### superpowers / everything-claude-code / planning-with-files / gstack
无额外依赖 (只需复制文件)