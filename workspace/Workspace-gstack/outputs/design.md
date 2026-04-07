# AI Coding 可落地性评估系统 - 设计文档

**文档版本**：v1.0  
**创建日期**：2026-04-03  
**项目名称**：AI-Ready Evaluator  
**技术栈**：Next.js 14 + TypeScript + SQLite + OpenAI API

---

## 1. 系统概述

### 1.1 产品定位

AI Coding可落地性评估系统（AI-Ready Evaluator）用于评估单条User Story是否能让AI无歧义地生成正确代码。系统通过静态分析需求文本，从五大维度评估AI实现成功率，并生成针对性的优化建议。

### 1.2 核心设计决策

基于6个强制问题的回答，确定以下核心决策：

| 决策项 | 选择 | 说明 |
|--------|------|------|
| 评估粒度 | 单条User Story | 以单条需求为评估单位 |
| 评估方法 | 静态分析 | 仅分析需求文本，不实际生成代码 |
| 评分验证 | 盲测验证集 | 预设已知结果的需求集，校准评分模型 |
| 输出形式 | Web仪表板 | 可视化报告 + 风险热力图 |
| 技术方案 | Next.js单体 | 单一仓库，Vercel部署 |

---

## 2. 系统架构

### 2.1 整体架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                        用户浏览器                                │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Next.js 14 (App Router)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │
│  │   页面路由    │  │  API Routes  │  │   中间件     │           │
│  └──────────────┘  └──────────────┘  └──────────────┘           │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                ┌──────────┼──────────┐
                ▼          ▼          ▼
         ┌──────────┐ ┌──────────┐ ┌──────────┐
         │ 评估引擎  │ │LLM分析模块│ │  数据库   │
         │ (核心)   │ │(GPT-4o) │ │ (SQLite) │
         └──────────┘ └──────────┘ └──────────┘
```

### 2.2 模块设计

#### 2.2.1 需求输入模块

- **功能**：接收用户输入的User Story文本
- **UI组件**：
  - 文本输入框（支持Markdown）
  - 评估按钮
  - 历史记录列表
- **验证**：输入非空，最大字符数限制（10000字符）

#### 2.2.2 评估引擎（核心模块）

**五大评估维度及权重**：

| 维度 | 权重 | 评分规则 |
|------|------|----------|
| 逻辑原子性 | 30% | 检查需求颗粒度、依赖解耦度、上下文窗口适配 |
| 边界明确性 | 25% | 检查异常流程、输入校验、状态枚举 |
| 上下文完备性 | 20% | 检查业务背景、术语定义、用户旅程 |
| 可验证性 | 15% | 检查验收标准量化度、测试场景完备性 |
| 技术约束清晰度 | 10% | 检查架构约束、技术栈限定、接口契约 |

**评分算法**：

```
AI就绪分 = Σ(维度得分 × 维度权重) × 复杂度惩罚系数

复杂度惩罚系数：
- 简单需求（CRUD类）：1.0
- 中等需求（业务逻辑类）：0.9
- 复杂需求（算法/架构类）：0.7
```

**AI就绪等级**：

| 等级 | 分数区间 | 说明 |
|------|----------|------|
| S级 | 90-100分 | AI可独立完成，无需人工干预 |
| A级 | 75-89分 | AI可实现，但需人工Review关键边界 |
| B级 | 60-74分 | AI生成代码后需人工大幅修改 |
| C级 | <60分 | 不建议AI实现，风险过高 |

#### 2.2.3 LLM分析模块

- **功能**：调用GPT-4o分析需求文本质量
- **调用方式**：OpenAI API (GPT-4o)
- **输入**：需求文本
- **输出**：各维度详细分析、缺失点识别、优化建议
- **配置**：支持运行时切换模型（预留接口）

#### 2.2.4 报告生成器

- **功能**：生成可视化评估报告
- **输出内容**：
  - AI就绪等级及总分
  - 五维雷达图
  - 维度详细得分及说明
  - 风险标记（需求原文中高亮AI可能误解的段落）
  - 优化建议列表
- **UI展示**：Web仪表板（图表使用Recharts）

#### 2.2.5 验证集管理模块

- **功能**：管理盲测验证集，用于评分校准
- **数据来源**：预设20-50条已知AI实现结果的需求
- **更新机制**：管理员可添加新验证集，定期校准模型
- **评估指标**：预测准确率、误判率

---

## 3. 数据库设计

### 3.1 数据库选型

SQLite（文件型数据库），满足轻量极简需求，无需独立部署。

### 3.2 表结构设计

#### 3.2.1 requirements 表

```sql
CREATE TABLE requirements (
  id TEXT PRIMARY KEY,
  content TEXT NOT NULL,
  title TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

#### 3.2.2 evaluation_results 表

```sql
CREATE TABLE evaluation_results (
  id TEXT PRIMARY KEY,
  requirement_id TEXT NOT NULL,
  total_score REAL NOT NULL,
  grade TEXT NOT NULL,
  
  -- 维度得分
  atomicity_score REAL,
  boundary_score REAL,
  context_score REAL,
  verifiability_score REAL,
  technical_score REAL,
  
  -- 复杂度系数
  complexity_factor REAL DEFAULT 1.0,
  
  -- LLM分析结果
  llm_analysis TEXT,
  optimization_suggestions TEXT,
  risk_markers TEXT,
  
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (requirement_id) REFERENCES requirements(id)
);
```

#### 3.2.3 validation_set 表

```sql
CREATE TABLE validation_set (
  id TEXT PRIMARY KEY,
  content TEXT NOT NULL,
  expected_grade TEXT NOT NULL,
  actual_grade TEXT,
  category TEXT NOT NULL,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

## 4. API设计

### 4.1 评估接口

#### POST /api/evaluate

**请求**：

```json
{
  "requirement": "用户注册功能。输入：用户名（6-20位字母数字），密码（8-32位包含大小写字母和数字）。输出：用户对象或错误码..."
}
```

**响应**：

```json
{
  "success": true,
  "data": {
    "id": "eval_xxx",
    "totalScore": 85,
    "grade": "A",
    "dimensions": {
      "atomicity": { "score": 90, "comment": "..." },
      "boundary": { "score": 80, "comment": "..." },
      "context": { "score": 85, "comment": "..." },
      "verifiability": { "score": 85, "comment": "..." },
      "technical": { "score": 80, "comment": "..." }
    },
    "complexityFactor": 1.0,
    "suggestions": [
      "建议补充：密码加密方式（BCrypt）",
      "建议补充：已存在用户名的错误码处理"
    ],
    "riskMarkers": [
      { "text": "密码", "risk": "未明确加密方式" }
    ]
  }
}
```

### 4.2 验证集接口

#### GET /api/validation

获取验证集列表，用于评分校准。

#### POST /api/validation/verify

验证评估结果准确性，管理员接口。

---

## 5. Web界面设计

### 5.1 页面结构

```
/
├── / (首页)
│   └── 需求输入 + 评估按钮
├── /result/[id]
│   └── 评估结果详情页
├── /history
│   └── 历史评估记录
└── /admin/validation
    └── 验证集管理（可选）
```

### 5.2 首页设计

**布局**：

- 顶部：系统标题 + Slogan
- 左侧：需求输入区（大文本框）
- 右侧：评估按钮 + 历史快速入口
- 底部：使用说明

**交互**：

1. 用户粘贴User Story
2. 点击"开始评估"按钮
3. 显示加载状态（调用LLM需5-10秒）
4. 跳转到结果页

### 5.3 结果页设计

**布局**：

- 顶部：AI就绪等级 + 总分（大字展示）
- 中部：五维雷达图（使用Recharts）
- 下方：各维度详细得分卡片
- 底部：优化建议列表 + 风险标记

**交互**：

- 点击雷达图维度，显示详细说明
- 点击优化建议，定位到需求原文对应位置

---

## 6. LLM Prompt设计

### 6.1 评估提示词

```
你是一个AI需求质量评估专家。请分析以下User Story，从5个维度评估其AI实现可行性：

维度定义：
1. 逻辑原子性（30%）：需求是否被拆分为AI可单次处理的合适颗粒度
2. 边界明确性（25%）：异常路径和边界条件是否被穷举
3. 上下文完备性（20%）：业务背景、术语定义是否完整
4. 可验证性（15%）：验收标准是否量化、可自动化测试
5. 技术约束清晰度（10%）：架构约束、技术栈是否明确

评估要求：
- 对每个维度给出0-100的得分
- 识别需求中AI最可能误解的关键词/短语（风险标记）
- 生成针对性的优化建议

需求文本：
---
{requirement}
---

请以JSON格式返回：
{
  "scores": {
    "atomicity": {"score": 85, "comment": "..."},
    "boundary": {"score": 70, "comment": "..."},
    "context": {"score": 80, "comment": "..."},
    "verifiability": {"score": 75, "comment": "..."},
    "technical": {"score": 90, "comment": "..."}
  },
  "riskMarkers": [{"text": "...", "risk": "..."}],
  "suggestions": ["...", "..."]
}
```

---

## 7. 部署方案

### 7.1 部署平台

Vercel（推荐）或Railway

### 7.2 环境变量

```env
# OpenAI API
OPENAI_API_KEY=sk-...

# 可选：模型配置
DEFAULT_MODEL=gpt-4o

# 可选：其他LLM提供商
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_GEMINI_API_KEY=...
```

### 7.3 SQLite部署注意

- Vercel：无持久化存储，需使用Vercel Blob或外部SQLite
- Railway：支持持久化磁盘存储，推荐

---

## 8. 开发计划

### Phase 1：核心功能（2周）

- [ ] 项目初始化（Next.js + TypeScript + SQLite）
- [ ] 评估引擎开发（五大维度评分逻辑）
- [ ] LLM集成（GPT-4o调用）
- [ ] 基础API开发

### Phase 2：Web界面（1周）

- [ ] 首页开发
- [ ] 结果页开发（五维雷达图）
- [ ] 响应式设计

### Phase 3：验证与优化（1周）

- [ ] 验证集建设（20-50条需求）
- [ ] 评分校准
- [ ] Bug修复与体验优化

---

## 9. 成功指标

| 指标 | 目标值 |
|------|--------|
| 评估耗时 | < 15秒（包括LLM调用） |
| AI代码采纳率（S/A级需求） | > 80% |
| 评分准确率（验证集） | > 85% |
| 人工介入预警准确率 | > 90% |

---

## 10. 系统边界与限制

### 10.1 不评估范围

- **技术可行性**：不评估需求是否在技术上可实现
- **商业价值**：不评估需求是否值得做
- **架构合理性**：不评估架构设计优劣

### 10.2 已知局限性

- 复杂算法推理：涉及多步数学推导的设计，AI仍易出错
- 隐性知识：未文档化的遗留代码约定，AI无法感知
- 创造性UI/UX：需主观审美判断的交互设计

---

*设计文档结束*
