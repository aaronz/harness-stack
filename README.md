# My Haness - AI Coding Workspace Framework

基于 opencode CLI 实现 PRD.md 需求的多方法论实现框架。

## 目录结构

```
myhaness/
├── PRD.md                          # 需求文档
├── setup-projects.sh              # 初始化项目脚本
├── run-all-workspaces.sh          # 批量运行所有 workspace
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
./implement-prd.sh

# 指定模型
./implement-prd.sh opencode/minimax-m2.5-free
```

### 3. 运行所有 Workspaces

```bash
./run-all-workspaces.sh

# 指定模型
./run-all-workspaces.sh opencode/minimax-m2.5-free
```

## 脚本特性

### 实现脚本 (implement-prd.sh)

每个 workspace 的 `implement-prd.sh` 包含以下特性：

- **迭代跟踪** - 输出到 `outputs/iteration-{N}/` 版本化目录
- **文件验证** - 自动检查生成文件的存在和内容大小
- **重试机制** - 文件生成失败时自动重试（最多2次）
- **差距分析** - 首先分析 PRD 与当前实现的差距
- **进度格式** - `[1/5]`, `[2/5]`, etc.
- **Build验证** - 要求 "Build必须通过"
- **验证报告** - 包含 P0问题状态, PRD完整度, 遗留问题, 下一步建议

### 批量运行脚本 (run-all-workspaces.sh)

依次运行所有 workspace 实现脚本，自动追踪迭代版本。

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

所有脚本支持通过位置参数指定模型：

```bash
# 默认模型
./implement-prd.sh

# 指定模型
./implement-prd.sh opencode/minimax-m2.5-free

# 或使用环境变量
MODEL=opencode/minimax-m2.5-free ./implement-prd.sh
```

默认模型: `opencode/minimax-m2.5-free`

## 依赖

- [opencode CLI](https://github.com/opencode-ai/opencode)
- Git
- Bash (Linux/macOS)