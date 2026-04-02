# AI-Ready Evaluator 设计文档

**文档版本**：v1.0  
**创建日期**：2026-04-02  
**基于PRD**：PRD.md v1.0  

---

## 1. 系统概述

### 1.1 产品定位

AI-Ready Evaluator 是一个评估开发需求 AI Coding 可实现性的系统。系统通过分析需求文档，从五大维度评估需求是否能让 AI 无歧义地生成正确代码，并给出优化建议。

### 1.2 目标用户

- **技术负责人**：判断哪些需求适合交给 AI 实现
- **产品经理**：优化需求描述，提升 AI 生成成功率

### 1.3 技术栈

| 层级 | 技术选择 |
|------|----------|
| 前端 | Next.js 14 + TypeScript 5 |
| 后端 | Next.js API Routes + Node.js |
| 数据库 | SQLite + Prisma ORM |
| AI 集成 | 多 LLM 支持（OpenAI/Anthropic/Gemini/Ollama） |
| 部署 | Vercel（前端）+ Railway（后端） |

---

## 2. 系统架构

### 2.1 整体架构

```
┌─────────────────────────────────────────────────────────────┐
│                        Client (Browser)                       │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │   Upload    │  │   Report    │  │  Settings   │          │
│  │    Page     │  │    Page     │  │    Page     │          │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘          │
└─────────┼────────────────┼────────────────┼──────────────────┘
          │                │                │
          ▼                ▼                ▼
┌─────────────────────────────────────────────────────────────┐
│                     Next.js Server                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │   API       │  │  AI Service │  │   Config    │          │
│  │   Routes    │  │   (LLM)     │  │   Service   │          │
│  └──────┬──────┘  └──────┬──────┘  └─────────────┘          │
│         │                │                                   │
│         ▼                ▼                                   │
│  ┌─────────────────────────────────────────────────┐        │
│  │              SQLite Database (Prisma)           │        │
│  │   - Evaluation Records                           │        │
│  │   - LLM Config                                   │        │
│  └─────────────────────────────────────────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 设计原则

- **单仓库部署**：Next.js 全栈应用，通过 API Routes 提供后端能力
- **轻量极简**：SQLite 文件型数据库，无需额外运维
- **运行时可配置**：支持动态切换 LLM 提供商

---

## 3. 数据模型

### 3.1 Prisma Schema

```prisma
// schema.prisma

generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "sqlite"
  url      = env("DATABASE_URL")
}

// LLM 配置表
model LLMConfig {
  id          String   @id @default(cuid())
  name        String   // 配置名称，如 "GPT-4o 默认"
  provider    String   // openai / anthropic / google / ollama
  model       String   // 模型名称，如 "gpt-4o"
  apiKey      String   // 加密存储
  baseUrl     String?  // 自定义端点（可选）
  isDefault   Boolean  @default(false)
  createdAt   DateTime @default(now())
  updatedAt   DateTime @updatedAt
}

// 评估记录表
model Evaluation {
  id              String   @id @default(cuid())
  title           String   // 需求标题
  content         String   // 需求原文
  fileName        String?  // 上传的文件名
  fileType        String?  // markdown / txt
  
  // 评估结果
  overallScore    Int      // 0-100 AI就绪分
  grade           String   // S / A / B / C
  complexity      String   // simple / medium / complex
  
  // 五维得分
  contextScore    Int      // 上下文完备性 0-100
  atomicityScore  Int      // 逻辑原子性 0-100
  boundaryScore   Int      // 边界明确性 0-100
  verifiabilityScore Int   // 可验证性 0-100
  techScore       Int      // 技术约束清晰度 0-100
  
  // AI 生成的原始响应
  rawResponse     String   // LLM 完整输出
  
  // 优化建议（JSON 数组）
  suggestions     String   // JSON.stringify([])
  
  // 使用的模型
  modelUsed       String   // 使用的模型名称
  
  createdAt       DateTime @default(now())
}
```

### 3.2 数据说明

| 模型 | 说明 |
|------|------|
| `LLMConfig` | 支持多模型配置，通过 `isDefault` 标记默认模型 |
| `Evaluation` | 存储完整评估结果，便于历史回溯和对比分析 |

---

## 4. API 设计

### 4.1 评估接口

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/evaluations` | 上传需求文档并发起评估 |
| `GET` | `/api/evaluations` | 获取评估历史列表 |
| `GET` | `/api/evaluations/[id]` | 获取单个评估详情 |
| `DELETE` | `/api/evaluations/[id]` | 删除评估记录 |

### 4.2 LLM 配置接口

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/config/llm` | 获取所有 LLM 配置 |
| `POST` | `/api/config/llm` | 创建 LLM 配置 |
| `PUT` | `/api/config/llm/[id]` | 更新 LLM 配置 |
| `DELETE` | `/api/config/llm/[id]` | 删除 LLM 配置 |

### 4.3 请求/响应格式

**POST /api/evaluations**

Request (multipart/form-data):
```
file: File (Markdown 或 TXT)
title: string (可选，从文件提取)
modelId: string (可选，使用指定模型，默认使用默认模型)
```

Response:
```json
{
  "id": "eval_xxx",
  "overallScore": 92,
  "grade": "S",
  "contextScore": 95,
  "atomicityScore": 90,
  "boundaryScore": 88,
  "verifiabilityScore": 100,
  "techScore": 95,
  "suggestions": [],
  "createdAt": "2026-04-02T10:00:00Z"
}
```

---

## 5. 评估 Prompt 设计

### 5.1 核心评估 Prompt

```typescript
const EVALUATION_PROMPT = `
你是一个 AI 需求评估专家。请分析以下需求，评估其 AI Coding 可实现性。

## 五大评估维度

1. 上下文完备性（25%）：业务背景、术语定义、用户旅程
2. 逻辑原子性（25%）：需求颗粒度、功能内聚、依赖解耦
3. 边界明确性（20%）：异常流程、输入校验、状态枚举
4. 可验证性（15%）：验收标准量化、测试场景完备性
5. 技术约束清晰度（15%）：架构约束、技术栈限定

## 需求内容
---
{REQUIREMENT_CONTENT}
---

## 输出要求

请返回 JSON 格式：
{
  "scores": {
    "context": 0-100,
    "atomicity": 0-100,
    "boundary": 0-100,
    "verifiability": 0-100,
    "tech": 0-100
  },
  "overall": 0-100,
  "grade": "S|A|B|C",
  "complexity": "simple|medium|complex",
  "risks": ["风险点1", "风险点2"],
  "suggestions": [
    {"type": "context|boundary|atomicity|verifiability", "content": "具体建议"}
  ]
}
`;
```

### 5.2 评分算法

```typescript
function calculateScore(scores: Scores, complexity: string): Result {
  const weights = {
    context: 0.25,
    atomicity: 0.25,
    boundary: 0.20,
    verifiability: 0.15,
    tech: 0.15,
  };
  
  const complexityPenalty = {
    simple: 1.0,
    medium: 0.9,
    complex: 0.7,
  };
  
  const rawScore = 
    scores.context * weights.context +
    scores.atomicity * weights.atomicity +
    scores.boundary * weights.boundary +
    scores.verifiability * weights.verifiability +
    scores.tech * weights.tech;
  
  const finalScore = Math.round(rawScore * complexityPenalty[complexity]);
  
  const grade = finalScore >= 90 ? 'S' :
                finalScore >= 75 ? 'A' :
                finalScore >= 60 ? 'B' : 'C';
  
  return { score: finalScore, grade };
}
```

---

## 6. 前端设计

### 6.1 页面结构

| 页面 | 路径 | 功能 |
|------|------|------|
| 首页 | `/` | 上传文档、历史评估列表 |
| 报告页 | `/evaluations/[id]` | 评估结果详情 |
| 设置页 | `/settings` | LLM 模型配置 |

### 6.2 首页布局

```
┌────────────────────────────────────────────────────────────┐
│  Header: AI-Ready Evaluator                    [Settings]  │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              上传需求文档区域                         │   │
│  │                                                      │   │
│  │    ┌─────────┐  ┌─────────┐                         │   │
│  │    │  📄 TXT │  │ 📝 MD   │                         │   │
│  │    └─────────┘  └─────────┘                         │   │
│  │                                                      │   │
│  │        [上传文件] 或 [拖拽文件到此处]                │   │
│  │                                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                            │
│  ── 历史评估记录 ─────────────────────────────────────────  │
│                                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ [S] 用户注册功能  │ 92分 │ 2026-04-01 │ [查看] [删除]│   │
│  │ [C] 智能推荐模块  │ 45分 │ 2026-04-01 │ [查看] [删除]│   │
│  │ [A] 订单冻结功能  │ 78分 │ 2026-03-30 │ [查看] [删除]│   │
│  └─────────────────────────────────────────────────────┘   │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### 6.3 评估报告页

```
┌────────────────────────────────────────────────────────────┐
│  ← 返回                    评估报告                         │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  AI就绪等级:  S (92分)                                    │
│  ┌──────────────────────────────────────────────────────   │
│  │ ████████████████████████████████████████ 92/100          │
│  └──────────────────────────────────────────────────────   │
│                                                            │
│  评分明细                                                  │
│  ┌────────────────┬──────┬────────────┐                   │
│  │ 维度           │ 得分 │ 状态       │                   │
│  ├────────────────┼──────┼────────────┤                   │
│  │ 上下文完备性   │ 95   │ ✅ 优秀    │                   │
│  │ 逻辑原子性    │ 90   │ ✅ 优秀    │                   │
│  │ 边界明确性    │ 88   │ ✅ 良好    │                   │
│  │ 可验证性       │ 100  │ ✅ 优秀    │                   │
│  │ 技术约束清晰度│ 95   │ ✅ 优秀    │                   │
│  └────────────────┴──────┴────────────┘                   │
│                                                            │
│  优化建议                                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ ✓ 无需优化，该需求AI可独立完成                         │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

### 6.4 设置页

```
┌────────────────────────────────────────────────────────────┐
│  ← 返回                    LLM 配置                         │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  当前模型: [GPT-4o ▾]                                      │
│                                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ + 添加新模型                                          │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                            │
│  已配置模型                                               │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 🔵 GPT-4o (默认)              [编辑] [删除]          │   │
│  │ ⚪ Claude 3.5                 [编辑] [删除]          │   │
│  │ ⚪ Gemini 2.0                 [编辑] [删除]          │   │
│  │ ⚪ Ollama (本地)              [编辑] [删除]          │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

---

## 7. 业务流程

### 7.1 需求评估流程

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│ 用户上传 │ →  │ 解析文档 │ →  │ 调用LLM │ →  │ 解析结果│ →  │ 保存结果│
│ 需求文档 │    │ 内容提取 │    │ 评估    │    │ 评分    │    │ 返回报告│
└─────────┘    └─────────┘    └─────────┘    └─────────┘    └─────────┘
```

### 7.2 关键实现细节

| 模块 | 实现方案 |
|------|----------|
| 文件上传 | Next.js API Route + `formidable` 解析 multipart |
| LLM 调用 | LangChain.js 或直接 SDK，支持流式输出 |
| 结果解析 | 使用 Zod 验证 LLM 返回的 JSON |
| 错误处理 | LLM 调用超时重试（最多3次），降级到备用模型 |
| 配置加密 | API Key 使用 AES-256 加密存储 |

---

## 8. 成功指标

| 指标 | 目标值 |
|------|--------|
| AI 代码采纳率 | S/A 级需求一次性采纳率 > 80% |
| 返工率降低 | 优化后需求 Bug 率降低 50% |
| 评估效率 | 单需求评估耗时 < 2 分钟 |
| 人工介入预警 | B/C 级需求提前标识 |

---

## 9. 系统边界与限制

### 9.1 不评估范围

- **技术可行性**：不评估需求是否在技术上可实现
- **商业价值**：不评估需求是否值得做
- **架构合理性**：不评估架构设计优劣

### 9.2 已知局限性

- 复杂算法推理：涉及多步数学推导或复杂状态机的设计，AI 仍易出错
- 隐性知识：未文档化的遗留代码约定，AI 无法感知
- 创造性 UI/UX：需主观审美判断的交互设计
- 跨系统一致性：涉及多个子系统协同的复杂集成场景

---

## 10. 部署方案

### 10.1 部署架构

| 环境 | 服务 | 配置 |
|------|------|------|
| 生产 | Vercel | Next.js 全栈应用 |
| 生产 | Railway | SQLite 数据持久化（可选） |
| 开发 | 本地 | `npm run dev` |

### 10.2 环境变量

```env
# 数据库
DATABASE_URL="file:./dev.db"

# LLM 配置（至少配置一个）
OPENAI_API_KEY="sk-xxx"
ANTHROPIC_API_KEY="sk-ant-xxx"
GEMINI_API_KEY="xxx"
OLLAMA_BASE_URL="http://localhost:11434"

# 加密密钥
ENCRYPTION_KEY="your-32-byte-encryption-key"
```

---

**文档状态**：已完成，等待用户审批后进入实施阶段
