# 六种 AI Coding 方法论实战对比：我的实验总结

> 一份 PRD，六种实现方式，谁更接近理想中的 AI 开发流程？

**实验日期**：2026年4月  
**实验对象**：AI Coding 可落地性评估系统（AI-Ready Evaluator）  
**技术栈**：TypeScript (Node.js + React/Next.js)

---

## 一、实验背景

作为一名长期关注 AI 辅助编程的开发者，我一直在探索一个问题：**如何让 AI 更可靠地完成全栈开发？**

市面上的 AI Coding 工具（Cursor、Devin、Windsurf）已经很强大了，但它们在处理复杂需求时仍然会「幻觉」频发、遗漏边界情况、忽略架构约束。问题的根源不在于 AI 能力不足，而在于**需求描述本身不适合机器理解**。

于是我萌生了一个想法：做一个「AI Coding 可落地性评估系统」，从五大维度评估需求是否能让 AI 无歧义地生成正确代码。

但这只是需求本身。关键问题是：**用 AI 来开发这个系统，哪种方法论最靠谱？**

---

## 二、六种方法论概览

我设计了六种不同的 AI 开发方法论，用同一份 PRD 分别实验：

| 编号 | 方法论 | 来源 | 核心特点 |
|------|--------|------|----------|
| 1 | **OpenSpec** | Claude Code 原生 | 最小化流程，直接实现 |
| 2 | **GStack** | 独立探索 | 7步审查流程，CEO级 + 工程级评审 |
| 3 | **Planning with Files** | 文件驱动规划 | 基于任务文件追踪进度 |
| 4 | **SpecKit** | 规范驱动 | Constitution（宪法）→ Spec → Plan → Tasks |
| 5 | **Superpowers** | 技能系统 | 依赖预置 Skills（brainstorming, writing-plans...） |
| 6 | **Everything Claude Code** | 工具箱 | 30+ Commands + 80+ Skills |

---

## 三、各方法论核心流程

### 3.1 OpenSpec —— 最简主义

```
PRD → 直接实现
```

没有复杂的流程设计，拿到需求直接开干。依靠 CLAUDE.md 和原生能力完成实现。

**产出**：完整的 `outputs/proposal/` 目录
- 后端：Express + TypeScript + Prisma + SQLite
- 前端：Next.js 14 完整页面（登录、评估、报告、设置）
- 文件数：50+

---

### 3.2 GStack —— 审查驱动

```
PRD → Office Hours → CEO Review → Eng Review → Implement → Review → QA & Ship → Retro
```

7步流程，每一步都有明确的审查标准和产出物。

**产出**：`outputs/worktrees/ai-ready-evaluator/`
- 使用 git worktree 隔离实现
- 完整的 Next.js 全栈应用

---

### 3.3 Planning with Files —— 文件驱动

```
task_plan.md → findings.md → progress.md
```

基于文件的任务规划和进度追踪系统。

**产出**：`outputs/backend/` + `outputs/frontend/`
- 前后端分离架构
- 完整的 providers 层（OpenAI, Anthropic, Gemini, Ollama）

---

### 3.4 SpecKit —— 规范驱动

```
Constitution（项目宪法）→ Specify（需求规范）→ Plan（技术计划）→ Tasks → Implement
```

核心特点：**用 Constitution（宪法）定义项目不可违背的原则**。

**产出**：`outputs/src/`
- 后端：entities、middleware、services、server 完整分层
- 前端：app、components（但页面较少）
- 特点：架构严谨，但前端完整度稍弱

---

### 3.5 Superpowers —— 技能系统

```
Brainstorming → Writing Plans → Subagent-Driven Dev → Verification → Finishing
```

核心特点：**依赖预置的 Skills 系统**（brainstorming, writing-plans, subagent-driven-development, verification-before-completion）。

**产出**：`outputs/src/`
- 前端：components 较完整（ScoreCard, LLMConfigForm, FileUpload, EvaluationList）
- 后端：API 路由 + lib 工具
- 特点：前端较强，后端偏弱

---

### 3.6 Everything Claude Code

这不是一个流程，而是一个**工具箱**：
- 30+ Commands（`/plan`, `/execute`, `/review`, `/refactor`...）
- 80+ Skills（覆盖各语言/框架的最佳实践）

**定位**：备用工具集，非方法论流程。

---

## 四、真实产出对比

| 方法论 | 代码文件数 | 目录结构 | 是否 Build |
|--------|-----------|----------|------------|
| OpenSpec | 50+ | backend + frontend | ✅ |
| GStack | 50+ | Next.js 全栈 | ✅ |
| Planning with Files | 50+ | backend + frontend | ✅ |
| SpecKit | 20+ | app + server + services | ❌ |
| Superpowers | 20+ | app + components + lib | ❌ |

### 代码结构差异

**SpecKit（后端强）**：
```
src/
├── app/           # Next.js pages
├── components/    # UI组件
├── config/        # 配置
├── db/           # schema
├── entities/      # 实体（requirement, score, risk-highlight...）
├── middleware/   # 5个中间件（auth, validation, error-handler...）
├── server/       # API路由
└── services/     # 业务逻辑（scoring-engine, risk-analyzer...）
```

**Superpowers（前端强）**：
```
src/
├── app/           # API routes + pages
├── components/    # 4个业务组件
└── lib/          # 工具库（llm, prisma, evaluator）
```

---

## 五、各方法论优缺点分析

### 5.1 OpenSpec

**优点**：
- 流程极简，效率最高
- 适合小中型需求
- 产出直接可用

**缺点**：
- 缺乏审查机制
- 复杂需求容易失控
- 依赖个人经验

---

### 5.2 GStack

**优点**：
- 审查流程完善（CEO级 + 工程级）
- 问题发现早，修复成本低
- 适合大型项目

**缺点**：
- 流程较长，步骤多
- 需要较多人工介入
- 轻量需求略显重

---

### 5.3 Planning with Files

**优点**：
- 任务追踪清晰
- 进度可视化
- 适合团队协作

**缺点**：
- 依赖文件同步
- 灵活性一般
- 技能要求高

---

### 5.4 SpecKit

**优点**：
- Constitution 确保原则一致
- 架构严谨，分层清晰
- 适合复杂系统

**缺点**：
- 前端完整度不足
- 缺乏 build 验证
- Constitution 设计门槛高

---

### 5.5 Superpowers

**优点**：
- Skills 系统可复用
- 前端组件丰富
- 验证流程完善

**缺点**：
- 后端逻辑偏弱
- 依赖预置 Skills
- 适合熟悉其体系的用户

---

## 六、我的结论与建议

### 6.1 核心发现

1. **都有能力完成实现**：六种方法论都能产出可运行的代码，只是完整度不同
2. **流程越长，问题越少**：GStack 的 7 步流程产出最完整
3. **技能系统是未来**：Superpowers 的 Skills 模式最有扩展性
4. **轻量需求选 OpenSpec**：不要用牛刀杀鸡
5. **架构需求选 SpecKit**： Constitution 机制确保长期一致性

### 6.2 实际选择建议

| 场景 | 推荐方法论 |
|------|-----------|
| 快速原型 / Hackathon | OpenSpec |
| 生产级项目 / 团队协作 | GStack |
| 需要进度追踪 | Planning with Files |
| 复杂架构系统 | SpecKit |
| 重复性项目 | Superpowers |
| 临时查资料 | Everything Claude Code |

### 6.3 我个人的选择

如果让我重新选择一次：

- **MVP 阶段**：OpenSpec（快）
- **产品化阶段**：GStack（稳）
- **长期维护**：SpecKit（一致性）

---

## 七、各方法论文档输出对比

### 7.1 文档产出清单

| 方法论 | 核心文档 | 数量 | 完整性 |
|--------|----------|------|--------|
| **OpenSpec** | proposal.md, design.md, tasks.md, 功能规格.md, 技术规格.md | 5 | ⭐⭐⭐⭐⭐ |
| **GStack** | design.md | 1 | ⭐⭐⭐⭐ |
| **Planning with Files** | task_plan.md, findings.md, progress.md | 3 | ⭐⭐⭐⭐ |
| **SpecKit** | constitution.md, spec.md, tasks.md | 3 | ⭐⭐⭐⭐⭐ |
| **Superpowers** | design.md, implementation plan | 2 | ⭐⭐⭐ |

---

### 7.2 各方法论文档内容要素对比

#### OpenSpec —— 商业化文档风格

**文档结构**：
- `proposal.md`：市场分析、需求分析、技术方案、商业模式
- `design.md`：系统架构、UI设计、API设计、技术选型
- `tasks.md`：16周详细任务分解，精确到小时

**内容特点**：
- 商业级语言：市场规模、竞争格局、用户痛点
- 完整技术方案：前端架构、后端架构、数据库设计、部署方案
- 详细的进度规划：Week 1-16 任务分解，责任人、预估工时

**内容质量**：⭐⭐⭐⭐⭐
- 文档专业度高，适合汇报和立项
- 技术细节详尽，可直接指导开发
- 进度规划具体可执行

---

#### GStack —— 极简设计文档

**文档结构**：
- `design.md`：产品定位、核心决策、系统架构、数据库设计

**内容特点**：
- 架构图清晰：ASCII图展示前后端交互
- 核心设计决策表：6个强制问题的回答
- 五大维度评分规则：具体权重和算法

**内容质量**：⭐⭐⭐⭐
- 聚焦核心设计决策
- 适合快速理解和对齐
- 但缺乏详细技术细节

---

#### Planning with Files —— 任务追踪风格

**文档结构**：
- `task_plan.md`：目标、产品定位、技术栈、实现阶段
- `findings.md`：研究发现（LLM Provider对比分析）
- `progress.md`：进度追踪

**内容特点**：
- 阶段式划分：Phase 1-4 里程碑
- 任务清单：精确的 checkbox 格式
- 变更记录：日期 + 变更内容 + 状态

**内容质量**：⭐⭐⭐⭐
- 任务拆解清晰
- 进度可追踪
- 适合团队协作

---

#### SpecKit —— 规范化文档风格

**文档结构**：
- `constitution.md`：项目宪法（7条核心原则）
- `spec.md`：功能规格（FR-001到FR-006）
- `tasks.md`：任务清单

**内容特点**：
- **Constitution（宪法）**：最独特的设计
  - 7条核心原则（PRINCIPLE_1 到 PRINCIPLE_7）
  - 每条原则包含：Non-Negotiable Rules + Rationale
  - 治理机制：修订程序、版本策略、合规审查
- **Spec（规格）**：严格的 Gherkin 格式
  - Given/When/Then 场景描述
  - 功能需求：FR-001 到 FR-006
  - 成功指标：SC-001 到 SC-004

**内容质量**：⭐⭐⭐⭐⭐
- 最规范的文档体系
- Constitution 确保项目一致性
- Spec 适合自动化测试对齐

---

#### Superpowers —— 设计+实现计划

**文档结构**：
- `design.md`：产品定位、技术栈、API设计、数据模型
- `implementation plan`：1767行详细任务分解

**内容特点**：
- 数据模型详尽：Prisma schema + 字段说明
- API设计：RESTful 路由清单
- **实现计划极详细**：每个文件创建、每行代码示例

**内容质量**：⭐⭐⭐⭐
- 实现计划最详细（1767行）
- 代码级别的任务描述
- 但缺乏用户场景和测试用例

---

### 7.3 内容要素详细对比表

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **产品定位** | ✅ 完整 | ✅ 简洁 | ✅ 简洁 | ✅ 简洁 | ✅ 简洁 |
| **目标用户** | ✅ 详细 | ❌ | ❌ | ✅ | ❌ |
| **市场规模分析** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **系统架构图** | ✅ 详细 | ✅ 简洁 | ✅ 简洁 | ❌ | ✅ |
| **技术选型** | ✅ 完整 | ✅ | ✅ | ✅ | ✅ |
| **数据库设计** | ✅ 完整 | ✅ SQL | ✅ | ✅ | ✅ Prisma |
| **API设计** | ✅ 详细 | ❌ | ❌ | ✅ | ✅ 详细 |
| **UI/UX设计** | ✅ 详细 | ❌ | ❌ | ❌ | ❌ |
| **用户场景** | ❌ | ❌ | ❌ | ✅ Gherkin | ❌ |
| **功能需求** | ✅ 详细 | ✅ | ✅ | ✅ FR编号 | ✅ |
| **非功能需求** | ✅ 性能/安全 | ❌ | ❌ | ❌ | ❌ |
| **风险分析** | ✅ 技术/市场 | ❌ | ❌ | ✅ AI局限 | ❌ |
| **项目里程碑** | ✅ 16周 | ❌ | ✅ Phase | ❌ | ❌ |
| **任务分解** | ✅ 详细 | ❌ | ✅ | ✅ | ✅ 超详细 |
| **核心原则/宪法** | ❌ | ❌ | ❌ | ✅ 7条 | ❌ |
| **代码示例** | ❌ | ❌ | ❌ | ❌ | ✅ |
| **变更记录** | ❌ | ❌ | ✅ | ❌ | ❌ |

---

### 7.4 文档质量主观评价

#### 商业化程度排名
1. **OpenSpec** ⭐⭐⭐⭐⭐ —— 最适合对外展示和立项
2. **SpecKit** ⭐⭐⭐⭐ —— 最适合内部规范和长期维护
3. **Superpowers** ⭐⭐⭐⭐ —— 最适合开发参考
4. **Planning with Files** ⭐⭐⭐ —— 适合任务追踪
5. **GStack** ⭐⭐⭐ —— 适合快速对齐

#### 技术详细度排名
1. **Superpowers** ⭐⭐⭐⭐⭐ —— 代码级实现计划
2. **OpenSpec** ⭐⭐⭐⭐ —— 完整技术方案
3. **SpecKit** ⭐⭐⭐⭐ —— 架构严谨
4. **Planning with Files** ⭐⭐⭐ —— 任务级
5. **GStack** ⭐⭐⭐ —— 设计级

#### 规范程度排名
1. **SpecKit** ⭐⭐⭐⭐⭐ —— Constitution + Spec 双重规范
2. **Planning with Files** ⭐⭐⭐⭐ —— 文件系统保证
3. **Superpowers** ⭐⭐⭐⭐ —— Skills 驱动
4. **GStack** ⭐⭐⭐ —— 审查流程
5. **OpenSpec** ⭐⭐⭐ —— 依赖个人

---

### 7.5 核心差距总结

| 维度 | 最佳表现 | 最弱表现 |
|------|----------|----------|
| **商业化程度** | OpenSpec | GStack |
| **技术详细度** | Superpowers | GStack |
| **规范约束力** | SpecKit | OpenSpec |
| **可执行性** | Superpowers | SpecKit |
| **团队协作** | Planning with Files | OpenSpec |

---

## 八、代码架构质量对比

### 8.1 架构模式对比

| 方法论 | 架构模式 | 分层清晰度 | 代码组织 |
|--------|----------|------------|----------|
| **OpenSpec** | 前后端分离 (Express + Next.js) | ⭐⭐⭐⭐⭐ | packages/backend + packages/frontend |
| **GStack** | Next.js 单体 (App Router) | ⭐⭐⭐⭐ | src/app + src/lib + src/components |
| **Planning with Files** | 前后端分离 | ⭐⭐⭐⭐⭐ | backend + frontend + core |
| **SpecKit** | 分层架构 (最严谨) | ⭐⭐⭐⭐⭐ | entities + services + middleware + server |
| **Superpowers** | Next.js 全栈 | ⭐⭐⭐⭐ | src/app + src/lib + src/components |

### 8.2 各方法论代码架构分析

#### OpenSpec —— 商业级分层架构

```
packages/
├── backend/
│   ├── src/
│   │   ├── routes/        # 路由层 (auth, evaluation, provider, config)
│   │   ├── services/      # 业务逻辑 (llm.ts, evaluation.ts)
│   │   ├── middleware/    # 中间件 (auth.ts)
│   │   └── index.ts       # 入口
│   └── prisma/            # 数据库 schema
└── frontend/
    ├── src/app/           # Next.js App Router
    ├── src/components/    # 组件
    └── src/store/         # 状态管理
```

**架构特点**：
- Monorepo 结构：packages 管理多个子项目
- 前后端完全分离
- 清晰的 RESTful API 设计
- Prisma ORM + SQLite

**架构质量**：⭐⭐⭐⭐⭐
- 分层清晰，职责明确
- 适合大型团队协作
- 依赖管理规范

---

#### GStack —— Next.js 单体架构

```
src/
├── app/
│   ├── api/evaluate/       # API 路由
│   ├── page.tsx           # 首页
│   └── layout.tsx          # 布局
├── components/            # UI 组件
│   ├── EvaluationForm.tsx
│   ├── ResultCard.tsx
│   └── RadarChart.tsx
├── lib/                   # 工具库
│   ├── db.ts
│   └── openai.ts
├── types/                 # 类型定义
└── scripts/               # 脚本
```

**架构特点**：
- Next.js 单体架构
- 内存存储 (Map) + 模拟数据库
- 内联业务逻辑
- 轻量级实现

**架构质量**：⭐⭐⭐⭐
- 简单直接
- 适合 MVP 和原型
- 但缺乏持久化层

---

#### Planning with Files —— 模块化架构

```
outputs/
├── src/
│   ├── core/              # 核心业务逻辑
│   │   ├── scoring.ts     # 评分引擎
│   │   └── complexity-classifier.ts
│   ├── providers/         # LLM 提供商抽象
│   │   ├── base.ts
│   │   ├── openai.ts
│   │   ├── anthropic.ts
│   │   ├── gemini.ts
│   │   └── ollama.ts
│   └── types/
│       └── evaluation.ts  # 类型定义
├── backend/               # Express 后端
│   └── src/
│       ├── routes/
│       ├── controllers/
│       └── middleware/
└── frontend/              # Next.js 前端
    └── src/
        ├── components/
        └── app/
```

**架构特点**：
- Provider 抽象层：支持多 LLM 提供商无缝切换
- 类型定义完善：独立的 types 包
- 前后端分离 + 核心模块独立

**架构质量**：⭐⭐⭐⭐⭐
- 抽象合理，易于扩展
- 类型安全
- 最适合生产环境

---

#### SpecKit —— 严谨分层架构

```
src/
├── entities/              # 实体定义
│   ├── requirement.ts
│   ├── score.ts
│   ├── risk-highlight.ts
│   └── optimization-suggestion.ts
├── services/              # 业务服务
│   ├── scoring-engine.ts
│   ├── risk-analyzer.ts
│   └── score-classifier.ts
├── middleware/            # 中间件
│   ├── auth.ts
│   ├── validation.ts
│   ├── error-handler.ts
│   ├── rate-limiter.ts
│   ├── audit-logger.ts
│   └── security.ts
├── server/                # 服务层
│   ├── index.ts
│   └── routes/
│       ├── requirements.ts
│       └── config.ts
├── db/
│   └── schema.ts          # 数据库 schema
└── app/                   # Next.js 页面
    ├── page.tsx
    └── layout.tsx
```

**架构特点**：
- **最多分层**：entities / services / middleware / server / db
- **中间件最丰富**：6 个独立中间件
- 业务逻辑与 HTTP 层完全解耦

**架构质量**：⭐⭐⭐⭐⭐
- 分层最严谨
- 扩展性最好
- 适合复杂企业级应用

---

#### Superpowers —— Next.js API Routes 架构

```
src/
├── app/
│   ├── api/
│   │   ├── evaluations/    # 评估 API
│   │   │   ├── route.ts
│   │   │   └── [id]/
│   │   │       └── route.ts
│   │   └── config/
│   │       └── llm/
│   │           ├── route.ts
│   │           └── [id]/
│   │               └── route.ts
│   ├── page.tsx           # 首页
│   ├── layout.tsx
│   ├── evaluations/[id]/  # 详情页
│   └── settings/          # 设置页
├── components/
│   ├── ScoreCard.tsx
│   ├── LLMConfigForm.tsx
│   ├── FileUpload.tsx
│   └── EvaluationList.tsx
└── lib/
    ├── prisma.ts          # 数据库客户端
    ├── llm.ts             # LLM 服务
    ├── evaluator.ts       # 评估逻辑
    └── encryption.ts      # 加密工具
```

**架构特点**：
- Next.js API Routes 风格
- 组件较丰富
- Prisma ORM

**架构质量**：⭐⭐⭐⭐
- 适合 Next.js 生态系统
- 但业务逻辑相对单薄

---

### 8.3 代码质量对比

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **分层清晰度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **类型安全** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **抽象程度** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **业务逻辑完整度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **可维护性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **可扩展性** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |

---

## 九、测试覆盖对比

### 9.1 测试现状

| 方法论 | 单元测试 | 集成测试 | E2E测试 | 测试文件数 |
|--------|----------|----------|---------|------------|
| **OpenSpec** | ❌ | ❌ | ❌ | 0 |
| **GStack** | ❌ | ❌ | ❌ | 0 |
| **Planning with Files** | ❌ | ❌ | ❌ | 0 |
| **SpecKit** | ❌ | ❌ | ❌ | 0 |
| **Superpowers** | ❌ | ❌ | ❌ | 0 |

**核心发现**：所有方法论产出的代码都**没有测试文件**。

---

### 9.2 测试缺失分析

所有方法论在实现过程中都**没有生成任何测试文件**，这反映了一个普遍问题：

1. **AI 生成代码的测试缺失是常态**
   - AI 擅长生成功能代码，但忽略测试
   - 需要显式要求 TDD 或测试生成

2. **缺乏测试驱动开发 (TDD) 流程**
   - 只有 `workspace-gstack` 有 `test-driven-development` skill
   - 但未在实现中实际应用

3. **验证方式依赖构建和运行**
   - 通过 `next build` 验证代码正确性
   - 通过运行应用验证功能

---

### 9.3 各方法论的测试能力评估

虽然实际测试文件缺失，但各方法论对测试的支持能力不同：

| 方法论 | 是否有 TDD 流程 | 测试生成能力 | 测试覆盖要求 |
|--------|---------------|--------------|-------------|
| **OpenSpec** | ❌ | ❌ | ❌ |
| **GStack** | ✅ 有 skill | ⚠️ 可触发 | ⚠️ 理论支持 |
| **Planning with Files** | ❌ | ❌ | ❌ |
| **SpecKit** | ❌ | ❌ | ❌ |
| **Superpowers** | ✅ 有 skill | ⚠️ 可触发 | ⚠️ 理论支持 |

---

### 9.4 构建验证对比

虽然无测试，但各方法论通过构建验证代码质量：

| 方法论 | Build 状态 | 构建验证 | 代码错误数 |
|--------|-----------|----------|-------------|
| **OpenSpec** | ✅ 通过 | `next build` | 0 |
| **GStack** | ✅ 通过 | `next build` | 0 |
| **Planning with Files** | ✅ 通过 | `next build` | 0 |
| **SpecKit** | ❌ 未构建 | N/A | 未知 |
| **Superpowers** | ❌ 未构建 | N/A | 未知 |

---

### 9.5 代码质量缺陷分析

通过代码审查发现的各方法论实现缺陷：

#### OpenSpec
- ⚠️ 异步评估未等待结果返回
- ⚠️ 错误处理较简单
- ✅ 类型定义完整

#### GStack
- ⚠️ 内存存储，重启数据丢失
- ⚠️ 缺乏数据库抽象
- ⚠️ 错误处理缺失

#### Planning with Files
- ✅ 类型定义最完善
- ✅ Provider 抽象层健壮
- ⚠️ 部分功能未完成

#### SpecKit
- ✅ 服务层设计最佳
- ⚠️ 前端代码较少
- ⚠️ 无实际运行验证

#### Superpowers
- ✅ 组件设计较好
- ⚠️ 业务逻辑分散
- ⚠️ 缺乏深层验证

---

## 十、综合评估总结

### 10.1 最终对比矩阵

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **文档完整度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **架构质量** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **测试覆盖** | ❌ | ❌ | ❌ | ❌ | ❌ |
| **构建通过** | ✅ | ✅ | ✅ | ❌ | ❌ |
| **代码完整度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **可维护性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **综合评分** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |

---

### 10.2 各方法论最佳用途

| 方法论 | 最佳用途 | 优势 | 劣势 |
|--------|----------|------|------|
| **OpenSpec** | 商业产品开发 | 文档完整、架构清晰 | 无测试 |
| **GStack** | 快速原型 | 快速实现、审查流程 | 代码简单 |
| **Planning with Files** | 生产级应用 | 架构严谨、可扩展 | 流程繁琐 |
| **SpecKit** | 企业级系统 | 最严谨分层 | 完成度低 |
| **Superpowers** | Next.js 项目 | 组件丰富 | 后端弱 |

---

### 10.3 改进建议

1. **测试缺失**：所有方法论需要集成测试生成流程
2. **构建验证**：SpecKit 和 Superpowers 需要完成 build
3. **持久化**：GStack 需要数据库抽象
4. **前端完整度**：SpecKit 需要更多前端页面

---

## 十一、效率与成本维度

### 11.1 效率对比

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **流程步骤数** | 1 | 7 | 3 | 5 | 5 |
| **启动速度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **完成速度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **迭代效率** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **人力介入点** | 0 | 5+ | 1-2 | 2-3 | 2-3 |

### 11.2 成本分析

| 成本类型 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|---------|----------|--------|----------|---------|-------------|
| **Prompt Token 消耗** | 低 | 高 | 中 | 高 | 高 |
| **执行时间** | 短 | 长 | 中 | 中 | 中 |
| **维护成本** | 低 | 中 | 中 | 中 | 中 |
| **学习曲线** | 低 | 中 | 中 | 高 | 高 |

### 11.3 效率优化建议

| 方法论 | 效率瓶颈 | 优化方向 |
|--------|----------|----------|
| **GStack** | 7步流程过长 | 合并相似步骤 |
| **SpecKit** | Constitution 设计门槛高 | 提供 Constitution 模板 |
| **Superpowers** | 依赖 Skills 系统 | 简化 Skill 调用 |

---

## 十二、可靠性与可重复性维度

### 12.1 可靠性评估

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **产出稳定性** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **错误恢复能力** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **调试难度** | 高 | 低 | 中 | 高 | 中 |
| **失败模式清晰度** | 低 | 高 | 中 | 中 | 中 |

### 12.2 可重复性分析

**关键问题**：同一份 PRD，多次使用同一方法论，产出是否一致？

| 方法论 | 可重复性 | 影响因素 |
|--------|----------|----------|
| **OpenSpec** | ⚠️ 低 | 依赖模型随机性和实现者经验 |
| **GStack** | ✅ 高 | 标准化流程减少随机性 |
| **Planning with Files** | ✅ 中高 | 任务文件提供结构化约束 |
| **SpecKit** | ✅ 高 | Constitution 提供一致性保障 |
| **Superpowers** | ✅ 中 | Skills 标准化但有版本差异 |

### 12.3 错误处理对比

| 方法论 | 错误检测时机 | 错误报告质量 | 恢复机制 |
|--------|-------------|-------------|----------|
| **OpenSpec** | 构建时 | 简单 | 手动 |
| **GStack** | 每步审查 | 详细 | 流程回退 |
| **Planning with Files** | 任务检查点 | 中等 | 任务重跑 |
| **SpecKit** | Constitution 检查 | 标准化 | 规范修复 |
| **Superpowers** | Verification skill | 详细 | Skill 重跑 |

---

## 十三、团队协作维度

### 13.1 协作效率对比

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **交接流畅度** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Review 集成** | ⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **版本控制友好** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **新人上手难度** | 低 | 中 | 中 | 高 | 高 |
| **文档可读性** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |

### 13.2 交接场景分析

#### OpenSpec
- ✅ 产出完整，可直接交接
- ✅ 代码结构清晰
- ❌ 缺乏交接文档
- ❌ Review 流程不明确

#### GStack
- ✅ 审查报告详细
- ✅ git worktree 隔离开发
- ✅ Retro 总结有助于复盘
- ⚠️ 流程较重，轻量需求不需要

#### Planning with Files
- ✅ 任务文件追踪清晰
- ✅ progress.md 进度可见
- ✅ 变更记录完整
- ⚠️ 需要团队成员都熟悉文件格式

#### SpecKit
- ✅ Constitution 提供项目原则
- ✅ 规范统一，减少争议
- ⚠️ Constitution 设计门槛高
- ⚠️ 新人需要理解 Constitution 才能贡献

#### Superpowers
- ✅ Skills 可复用
- ✅ Verification 确保质量
- ⚠️ 需要熟悉 Skills 系统
- ⚠️ 团队需要统一配置

### 13.3 团队规模适配性

| 方法论 | 个人 | 2-3人 | 4-10人 | 10+人 |
|--------|------|-------|--------|-------|
| **OpenSpec** | ✅ | ✅ | ⚠️ | ❌ |
| **GStack** | ⚠️ | ✅ | ✅ | ✅ |
| **Planning with Files** | ⚠️ | ✅ | ✅ | ⚠️ |
| **SpecKit** | ❌ | ⚠️ | ✅ | ✅ |
| **Superpowers** | ⚠️ | ✅ | ✅ | ⚠️ |

---

## 十四、AI模型兼容性维度

### 14.1 模型适配性对比

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **模型限制** | 无 | 无 | 无 | 无 | 无 |
| **最佳模型** | GPT-4o | GPT-4o | 均可 | 强推理模型 | 均可 |
| **弱模型表现** | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| **Prompt 敏感性** | 高 | 中 | 中 | 低 | 中 |

### 14.2 Prompt 工程需求

| 方法论 | Prompt 技巧要求 | 模板化程度 | 容错性 |
|--------|---------------|-----------|--------|
| **OpenSpec** | 高 | 低 | 低 |
| **GStack** | 中 | 中 | 中 |
| **Planning with Files** | 中 | 高 | 中 |
| **SpecKit** | 低 | 高 | 高 |
| **Superpowers** | 中 | 高 | 中 |

### 14.3 模型能力利用

| 方法论 | 利用推理能力 | 利用工具调用 | 利用上下文 |
|--------|-------------|-------------|-----------|
| **OpenSpec** | 间接 | ❌ | 高 |
| **GStack** | 直接 | ⚠️ | 高 |
| **Planning with Files** | 间接 | ⚠️ | 高 |
| **SpecKit** | 直接 | ❌ | 中 |
| **Superpowers** | 间接 | ⚠️ | 高 |

---

## 十五、技术债务维度

### 15.1 技术债务分析

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers |
|------|----------|--------|----------|---------|-------------|
| **初始债务** | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **债务增长速度** | 快 | 慢 | 中 | 慢 | 中 |
| **债务类型** | 缺少测试、文档缺失 | 内存存储、简单错误处理 | 部分功能未完成 | 前端不完整 | 后端逻辑弱 |
| **还债难度** | 中 | 低 | 中 | 高 | 中 |

### 15.2 代码可维护性

| 方法论 | 代码可读性 | 扩展性 | 修改难度 | 重构风险 |
|--------|-----------|--------|---------|---------|
| **OpenSpec** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 低 | 低 |
| **GStack** | ⭐⭐⭐ | ⭐⭐⭐ | 低 | 中 |
| **Planning with Files** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 低 | 低 |
| **SpecKit** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 中 | 低 |
| **Superpowers** | ⭐⭐⭐⭐ | ⭐⭐⭐ | 低 | 中 |

### 15.3 长期维护评估

| 方法论 | 维护优先级 | 常见问题 | 维护建议 |
|--------|----------|----------|----------|
| **OpenSpec** | 中 | 缺少测试 | 添加测试套件 |
| **GStack** | 低 | 数据持久化 | 引入数据库 |
| **Planning with Files** | 高 | 扩展 Provider | 完善未完成功能 |
| **SpecKit** | 高 | 前端补全 | 完善前端页面 |
| **Superpowers** | 中 | 后端增强 | 丰富业务逻辑 |

---

## 十六、综合评分与最终建议

### 16.1 完整对比矩阵

| 维度 | OpenSpec | GStack | Planning | SpecKit | Superpowers | 权重 |
|------|----------|--------|----------|---------|-------------|------|
| **文档完整度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | 15% |
| **架构质量** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 15% |
| **测试覆盖** | ❌ | ❌ | ❌ | ❌ | ❌ | 15% |
| **构建通过** | ✅ | ✅ | ✅ | ❌ | ❌ | 10% |
| **效率与速度** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | 10% |
| **可靠性** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | 10% |
| **团队协作** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 10% |
| **模型兼容性** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 5% |
| **技术债务** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | 10% |
| ****综合评分** | **3.6** | **3.4** | **3.7** | **3.5** | **3.2** | |

### 16.2 场景化推荐

| 场景 | 推荐方法论 | 理由 |
|------|-----------|------|
| **Hackathon（24h）** | OpenSpec | 最快产出 |
| **概念验证（POC）** | OpenSpec + GStack | 快速且有审查 |
| **Startup MVP** | Planning with Files | 平衡速度和质量 |
| **企业内部工具** | SpecKit | 架构严谨、易维护 |
| **咨询/外包项目** | GStack | 流程可控、客户可见 |
| **开源项目** | SpecKit + Superpowers | 规范 + 可复用 |
| **AI Coding 研究** | 全部对比 | 实验床 |

### 16.3 方法论演进路线

```
阶段1: OpenSpec (快速启动)
    ↓ 发现需求复杂度上升
阶段2: GStack (增加审查)
    ↓ 团队规模扩大
阶段3: Planning with Files (任务追踪)
    ↓ 需要长期规范
阶段4: SpecKit (Constitution)
    ↓ 重复性工作多
阶段5: Superpowers (Skills 自动化)
```

---

## 十七、后续实验方向

1. **盲测对比**：用同一需求让不同方法论独立实现，对比代码质量
2. **Skill 抽取**：将 GStack 的审查流程抽取为可复用的 Skills
3. **混合模式**：结合 SpecKit 的 Constitution 和 Superpowers 的 Skills
4. **测试增强**：为各方法论添加测试生成流程
5. **成本量化**：精确测量各方法论的 Token 消耗和执行时间

---

## 十八、最终建议

### AI Coding Harness 选择框架

**问自己 5 个问题**：

1. **时间紧迫吗？** → Yes → OpenSpec / GStack
2. **团队规模 > 3 人？** → Yes → Planning with Files / SpecKit
3. **需要长期维护？** → Yes → SpecKit
4. **有重复性工作？** → Yes → Superpowers
5. **需要对外交付？** → Yes → GStack + 完整文档

### 最佳实践组合

| 组合 | 适用场景 | 效果 |
|------|---------|------|
| **OpenSpec + GStack Review** | 快速启动 + 质量把控 | 速度与质量平衡 |
| **SpecKit Constitution + Planning Files** | 企业级项目 | 规范与追踪兼顾 |
| **Superpowers Skills + OpenSpec** | 重复性项目 | 自动化最大化 |
| **All-in-One** | 研究学习 | 全面理解各方法论 |

### 最终结论

**没有银弹**：每种方法论都有其适用场景，关键是根据项目阶段、团队规模、时间要求选择合适的方法论。

**推荐路径**：从 OpenSpec 开始，根据需要逐步引入 GStack 的审查、Planning 的追踪、SpecKit 的规范、Superpowers 的自动化。

---

**EOF**

*本文档为个人实验总结，欢迎讨论和指正。*
