# AI-Ready Evaluator 设计文档 v2

**文档版本**：v2.0  
**创建日期**：2026-04-03  
**基于PRD**：PRD.md v1.0  
**状态**：已实现

---

## 1. 设计决策摘要

### 1.1 用户确认的决策

| 决策项 | 选择方案 | 理由 |
|--------|----------|------|
| 权重配置管理方式 | **命名预设方案** | 用户可创建多个命名预设（如"严格模式"、"业务优先"），符合PRD的权重配置要求 |
| 风险高亮渲染方式 | **Inline高亮** | 在原文对应位置用颜色标记，hover显示风险说明，符合PRD"高风险区域标记"语义 |
| 模型选择器交互 | **下拉选择器** | 上传区域上方显示模型下拉选择器，默认使用默认模型，简单直观 |

### 1.2 实施策略

采用 **MVP优先策略**，分阶段交付：

| 阶段 | 内容 | 优先级 |
|------|------|--------|
| Phase 1 | 权重配置UI + 模型选择器 | P0/P1 |
| Phase 2 | 风险热图Inline高亮 + 上下文缺口展示 | P1 |
| Phase 3 | LLM编辑增强 + 重试机制 + 测试 | P2 |

---

## 2. 架构变更

**无架构调整**，基于现有Next.js全栈架构。

```
┌─────────────────────────────────────────────────────────────┐
│  Upload Page                  │  Report Page    │ Settings │
│  ├─ 模型下拉选择器 ✓          │  ├─ 风险Inline  │ ├─权重预设│
│  └─ 保持不变                 │  │  高亮 ✓      │ │管理 ✓   │
│                               │  ├─ 上下文缺口  │ ├─LLM编辑 │
│                               │  │  展示 ✓      │ │增强 ✓   │
│                               │  └─ 标题编辑 ✓  │ └─        │
│                               │  ✓              │           │
└─────────────────────────────────────────────────────────────┘
```

**向后兼容**：所有变更都是增量添加，不破坏现有API契约。

---

## 3. 权重预设管理系统

### 3.1 数据模型

Prisma Schema已包含 `ScoreWeights` 模型：

```prisma
model ScoreWeights {
  id             String   @id @default(cuid())
  name           String   @unique
  context        Float    @default(0.25)
  atomicity      Float    @default(0.25)
  boundary       Float    @default(0.20)
  verifiability  Float    @default(0.15)
  tech           Float    @default(0.15)
  isDefault      Boolean  @default(false)
  createdAt     DateTime @default(now())
  updatedAt     DateTime @updatedAt
}
```

### 3.2 默认预设值

| 预设名称 | Context | Atomicity | Boundary | Verifiability | Tech |
|----------|---------|-----------|----------|---------------|------|
| 标准模式 | 25% | 25% | 20% | 15% | 15% |

### 3.3 API扩展

| Method | Path | 说明 |
|--------|------|------|
| `GET` | `/api/config/weights` | 获取所有权重预设 |
| `POST` | `/api/config/weights` | 创建新预设 |
| `PUT` | `/api/config/weights/[id]` | 更新预设 |
| `DELETE` | `/api/config/weights/[id]` | 删除预设 |

### 3.4 前端组件

**WeightsForm.tsx** - 完整的CRUD管理界面：
- 创建/编辑/删除权重预设
- 权重分布可视化（进度条）
- 权重总和验证（必须=1.0）
- 默认预设标记

---

## 4. 模型选择器

### 4.1 设计

在文件上传区域上方添加模型下拉选择器：
- 显示所有已配置的LLM模型
- 默认使用标记为`isDefault`的模型
- 用户可选择其他模型进行评估对比

### 4.2 数据流

```
用户选择模型 → FileUpload传递modelId → API保存 → 评估使用选定模型
```

### 4.3 文件变更

- `FileUpload.tsx` - 新增 `models`, `selectedModelId`, `onModelChange` props
- `page.tsx` - 获取模型列表并传递给FileUpload
- API接受 `modelId` 参数优先于默认模型

---

## 5. 风险热图 Inline高亮

### 5.1 设计

在评估报告页，原文展示区域使用 `RiskHighlight` 组件：
- 将 `risks` 数组中的风险描述与原文匹配
- 用 `<mark>` 标签高亮匹配文本
- 风险文本默认使用红色背景 (`bg-red-200`)

### 5.2 匹配逻辑

1. 按风险描述长度降序排序（避免短匹配覆盖长匹配）
2. 转义正则特殊字符
3. 全局替换匹配文本为标记

### 5.3 组件

**RiskHighlight.tsx**：
```tsx
interface RiskHighlightProps {
  content: string
  risks: string[]
}
```

---

## 6. 上下文缺口展示

### 6.1 设计

在评估报告页添加独立区块 **"Context Gaps & Risks"**：
- 显示 `risks` 数组内容
- 每个风险项使用红色圆点标记
- 仅在存在风险时显示

### 6.2 数据来源

从 `rawResponse` JSON中解析 `risks` 字段：
```tsx
data.risks = raw.risks || []
```

---

## 7. LLM配置编辑增强

### 7.1 功能

在现有 `LLMConfigForm` 组件上添加编辑模式：
- Edit按钮填充现有数据到表单
- API Key留空表示保持不变
- 支持创建/编辑/删除三种操作

### 7.2 编辑交互

1. 点击Edit按钮 → 表单填充现有数据
2. 修改后Save → PUT请求
3. API Key为空时不更新Key

---

## 8. 标题编辑

### 8.1 组件

**EditableTitle.tsx**：
- 点击铅笔图标进入编辑模式
- Enter保存，Escape取消
- 失焦自动保存
- 支持键盘操作

### 8.2 API

使用已有的 `PUT /api/evaluations/[id]` 接口，仅更新 `title` 字段。

---

## 9. LLM重试机制

### 9.1 设计

在 `llm.ts` 中添加 `withRetry` 函数：
- 最多3次重试
- 指数退避（1s, 2s, 4s）+ 随机抖动
- 不对API Key错误和Invalid model ID重试

### 9.2 代码

```typescript
async function withRetry<T>(
  fn: () => Promise<T>,
  maxRetries: number = 3,
  baseDelayMs: number = 1000
): Promise<T>
```

---

## 10. 文件结构

```
outputs/
├── prisma/schema.prisma          # ScoreWeights模型已存在
├── src/
│   ├── app/
│   │   ├── api/
│   │   │   ├── config/
│   │   │   │   ├── weights/
│   │   │   │   │   ├── route.ts      # GET/POST
│   │   │   │   │   └── [id]/route.ts # PUT/DELETE (新增)
│   │   │   │   └── llm/
│   │   │   │       ├── route.ts      # GET/POST
│   │   │   │       └── [id]/route.ts # PUT/DELETE
│   │   │   └── evaluations/
│   │   │       ├── route.ts          # GET/POST
│   │   │       └── [id]/route.ts     # GET/PUT/DELETE
│   │   ├── settings/page.tsx          # 添加WeightsForm
│   │   └── evaluations/[id]/page.tsx  # RiskHighlight, Risks Section, EditableTitle
│   ├── components/
│   │   ├── RiskHighlight.tsx          # NEW - Inline风险高亮
│   │   ├── WeightsForm.tsx            # NEW - 权重预设管理
│   │   ├── EditableTitle.tsx          # NEW - 标题编辑
│   │   ├── LLMConfigForm.tsx          # MODIFY - 添加编辑模式
│   │   ├── FileUpload.tsx             # MODIFY - 添加模型选择器
│   │   └── ScoreCard.tsx              # 保持不变
│   └── lib/
│       ├── llm.ts                     # MODIFY - 添加withRetry
│       ├── evaluator.ts                # 保持不变
│       └── evaluator.test.ts           # MODIFY - 增强测试
```

---

## 11. 验证清单

- [x] 权重预设可创建/编辑/删除
- [x] 模型选择器显示所有已配置模型
- [x] 评估使用选定的模型
- [x] 风险在原文中高亮显示
- [x] Context Gaps区块正确显示风险列表
- [x] LLM配置可编辑
- [x] 标题可内联编辑
- [x] LLM调用失败时自动重试
- [x] 构建通过无错误

---

**文档状态**：已完成实现
