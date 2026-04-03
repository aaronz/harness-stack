# AI Coding 可落地性评估系统（AI-Ready Evaluator）实施计划

## 1. 项目概述

### 1.1 产品定位
评估开发需求是否可以被AI Coding工具（GitHub Copilot、Cursor、Devin等）无歧义地正确实现，核心从"人能否理解"转向"AI能否无歧义地生成正确代码"。

### 1.2 技术栈
- **前端**：Next.js 14 + TypeScript 5
- **后端**：Node.js + Express + TypeScript
- **数据库**：SQLite（文件型）
- **AI集成**：多LLM支持（OpenAI GPT-4o/GPT-4o-mini、Anthropic Claude 3.5/3.0、Google Gemini、本地Ollama）

---

## 2. 模块划分及依赖关系

### 2.1 模块架构图

```
┌─────────────────────────────────────────────────────────────────┐
│                        前端 (Next.js)                           │
├─────────────┬─────────────┬─────────────┬─────────────────────┤
│  首页模块    │  评估报告    │  历史记录    │  设置模块          │
│  (Home)     │  (Report)   │  (History)  │  (Settings)        │
└──────┬──────┴──────┬──────┴──────┬──────┴──────────┬──────────┘
       │             │             │                 │
       └─────────────┴──────┬──────┴─────────────────┘
                            │ HTTP API
       ┌────────────────────┴────────────────────┐
       │              API Gateway                 │
       │           (Express Router)               │
       ├─────────────┬─────────────┬───────────────┤
       │             │             │               │
┌──────┴──────┐ ┌────┴────┐ ┌─────┴────┐ ┌───────┴───────┐
│  评估引擎    │ │ 用户服务  │ │ LLM配置  │ │ 报告生成器    │
│ (Evaluator) │ │ (User)   │ │ (LLM)    │ │ (Reporter)    │
└──────┬──────┘ └────┬────┘ └─────┬────┘ └───────┬───────┘
       │             │            │              │
       └─────────────┴─────┬──────┴──────────────┘
                           │
                    ┌──────┴──────┐
                    │  SQLite DB   │
                    └─────────────┘
```

### 2.2 核心模块说明

| 模块 | 职责 | 依赖模块 |
|------|------|----------|
| **API Gateway** | 请求路由、参数校验、统一的响应格式 | 无 |
| **评估引擎 (Evaluator)** | 执行五大维度评估、计算AI就绪分 | LLM配置模块 |
| **用户服务 (User)** | 评估记录CRUD、用户认证 | 数据库 |
| **LLM配置服务** | LLMprovider管理、API密钥配置 | 数据库 |
| **报告生成器** | 生成评估报告、风险热力图、优化建议 | 评估引擎 |

### 2.3 依赖关系

```
用户服务 ──► 数据库
LLM配置服务 ──► 数据库
评估引擎 ──► LLM配置服务 ──► 数据库
报告生成器 ──► 评估引擎 ──► LLM配置服务
API Gateway ──► 所有服务
```

---

## 3. 数据模型设计

### 3.1 数据库表结构

```sql
-- 用户表
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- LLM配置表
CREATE TABLE llm_configs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    provider TEXT NOT NULL, -- 'openai' | 'anthropic' | 'gemini' | 'ollama'
    model TEXT NOT NULL,
    api_key TEXT, -- 加密存储
    base_url TEXT, -- 用于Ollama等自定义端点
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- 评估记录表
CREATE TABLE evaluations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    requirement_text TEXT NOT NULL,
    complexity TEXT NOT NULL, -- 'simple' | 'medium' | 'complex'
    
    -- 五大维度得分
    context_score REAL,
    atomicity_score REAL,
    boundary_score REAL,
    verifiability_score REAL,
    technical_score REAL,
    
    -- 最终得分
    final_score REAL,
    grade TEXT, -- 'S' | 'A' | 'B' | 'C'
    
    -- 优化建议 (JSON数组)
    suggestions TEXT,
    
    -- 风险热力图 (JSON)
    risk_heatmap TEXT,
    
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- 评估维度详情表 (可选，用于存储每个维度的详细评估结果)
CREATE TABLE evaluation_details (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    evaluation_id INTEGER NOT NULL,
    dimension TEXT NOT NULL, -- 'context' | 'atomicity' | 'boundary' | 'verifiability' | 'technical'
    score REAL NOT NULL,
    findings TEXT, -- JSON格式，存储检查点评估结果
    FOREIGN KEY (evaluation_id) REFERENCES evaluations(id)
);
```

### 3.2 数据模型TypeScript定义

```typescript
// types/index.ts

export type LLMProvider = 'openai' | 'anthropic' | 'gemini' | 'ollama';

export interface LLMConfig {
  id: number;
  user_id: number;
  provider: LLMProvider;
  model: string;
  api_key?: string;
  base_url?: string;
  is_active: boolean;
  created_at: string;
}

export type Complexity = 'simple' | 'medium' | 'complex';

export type Grade = 'S' | 'A' | 'B' | 'C';

export interface Evaluation {
  id: number;
  user_id: number;
  requirement_text: string;
  complexity: Complexity;
  context_score: number;
  atomicity_score: number;
  boundary_score: number;
  verifiability_score: number;
  technical_score: number;
  final_score: number;
  grade: Grade;
  suggestions: EvaluationSuggestion[];
  risk_heatmap: RiskHeatmapItem[];
  created_at: string;
}

export interface EvaluationSuggestion {
  category: 'context' | 'atomicity' | 'boundary' | 'verifiability' | 'technical';
  priority: 'high' | 'medium' | 'low';
  content: string;
}

export interface RiskHeatmapItem {
  text: string;
  start: number;
  end: number;
  risk_level: 'high' | 'medium' | 'low';
  dimension: string;
}
```

---

## 4. API接口定义

### 4.1 API基础路径
```
Base URL: /api/v1
```

### 4.2 接口列表

#### 4.2.1 评估接口

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /evaluations | 创建新评估 |
| GET | /evaluations/:id | 获取评估详情 |
| GET | /evaluations | 获取评估列表（分页） |
| DELETE | /evaluations/:id | 删除评估记录 |

**POST /evaluations 请求体：**
```typescript
interface CreateEvaluationRequest {
  requirement_text: string;  // 需求文本
  complexity: 'simple' | 'medium' | 'complex';  // 复杂度选择
  model_id?: number;  // 可选，指定使用的LLM配置ID
}
```

**响应：**
```typescript
interface EvaluationResponse {
  id: number;
  requirement_text: string;
  complexity: string;
  context_score: number;
  atomicity_score: number;
  boundary_score: number;
  verifiability_score: number;
  technical_score: number;
  final_score: number;
  grade: 'S' | 'A' | 'B' | 'C';
  suggestions: {
    category: string;
    priority: string;
    content: string;
  }[];
  risk_heatmap: {
    text: string;
    start: number;
    end: number;
    risk_level: string;
    dimension: string;
  }[];
  created_at: string;
}
```

#### 4.2.2 LLM配置接口

| 方法 | 路径 |说明 |
|------|------|------|
| GET | /llm-configs | 获取当前用户的LLM配置列表 |
| POST | /llm-configs | 创建LLM配置 |
| PUT | /llm-configs/:id | 更新LLM配置 |
| DELETE | /llm-configs/:id | 删除LLM配置 |
| POST | /llm-configs/:id/set-active | 设置为默认/活跃配置 |

**POST /llm-configs 请求体：**
```typescript
interface CreateLLMConfigRequest {
  provider: 'openai' | 'anthropic' | 'gemini' | 'ollama';
  model: string;
  api_key?: string;
  base_url?: string;  // 仅Ollama需要
}
```

#### 4.2.3 用户接口

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /auth/register | 用户注册 |
| POST | /auth/login | 用户登录 |
| GET | /auth/me | 获取当前用户信息 |

### 4.3 错误响应格式

```typescript
interface ErrorResponse {
  error: {
    code: string;
    message: string;
    details?: Record<string, unknown>;
  };
}
```

常见错误码：
- `EVAL_001`: 评估服务内部错误
- `EVAL_002`: LLM API调用失败
- `LLM_001`: LLM配置无效
- `LLM_002`: API密钥错误
- `AUTH_001`: 未授权
- `AUTH_002`: 用户名已存在

---

## 5. 前端页面规划

### 5.1 页面结构

```
src/app/
├── page.tsx                    # 首页 - 需求提交
├── layout.tsx                   # 根布局
├── globals.css                  # 全局样式
├── evaluation/
│   └── [id]/
│       └── page.tsx            # 评估报告页
├── history/
│   └── page.tsx                # 历史评估页
├── settings/
│   └── page.tsx                # 设置页 (LLM配置)
└── api/                        # API路由
    ├── evaluations/
    ├── llm-configs/
    └── auth/
```

### 5.2 页面详情

#### 5.2.1 首页 (/)

**功能：**
- 需求文本输入框（富文本，支持Markdown）
- 复杂度选择器（简单/中等/复杂）
- 评估按钮 + 评估中状态动画
- 快速跳转至历史记录

**组件：**
- `RequirementInput` - 需求输入组件
- `ComplexitySelector` - 复杂度选择器
- `EvaluateButton` - 评估按钮
- `QuickHistory` - 最近评估预览

#### 5.2.2 评估报告页 (/evaluation/[id])

**功能：**
- 五大维度得分雷达图
- 最终AI就绪分及等级
- 风险热力图（原文高亮）
- 优化建议列表（按优先级排序）
- 重新评估按钮
- 导出PDF功能

**组件：**
- `ScoreRadar` - 雷达图组件
- `ScoreCircle` - 环形分数显示
- `RiskHeatmap` - 风险热力图（基于原文的风险段落高亮）
- `SuggestionList` - 优化建议列表
- `GradeBadge` - 等级徽章（S/A/B/C）

#### 5.2.3 历史评估页 (/history)

**功能：**
- 评估记录列表（支持分页）
- 筛选：按等级、按时间范围
- 搜索：按需求关键词
- 快速查看评估结果

**组件：**
- `EvaluationTable` - 评估记录表格
- `FilterBar` - 筛选工具栏
- `Pagination` - 分页组件

#### 5.2.4 设置页 (/settings)

**功能：**
- LLM配置列表
- 添加新LLM配置（provider选择、API key输入）
- 激活/禁用配置
- 测试LLM连接

**组件：**
- `LLMConfigList` - LLM配置列表
- `LLMConfigForm` - 配置表单
- `TestConnection` - 连接测试按钮

### 5.3 前端技术要点

- **状态管理**：React Context + useReducer（轻量级）
- **UI组件库**：可使用 shadcn/ui 或 自定义组件
- **图表**：Recharts 或 Chart.js
- **样式**：Tailwind CSS
- **表单验证**：Zod + React Hook Form

---

## 6. 评估引擎核心逻辑

### 6.1 五大维度评估prompt模板

```typescript
const evaluationPrompt = `
你是一个AI需求评估专家。请从以下五个维度评估需求文档的AI可实现性：

## 五大维度及权重
1. 上下文完备性 (25%)：业务背景、术语定义、用户旅程、数据实体关系
2. 逻辑原子性 (25%)：功能单一性、上下文窗口适配、依赖显式化、原子性验证
3. 边界明确性 (20%)：输入域定义、状态转换完整性、并发与竞态、性能边界
4. 可验证性 (15%)：示例驱动、验收标准量化、可自动化测试、行为契约
5. 技术约束清晰度 (15%)：架构合规性、技术栈锁定、接口契约、安全约束

## 需求内容：
{requirement_text}

## 复杂度：{complexity}

请以JSON格式返回评估结果：
{
  "context_score": 0-100,
  "atomicity_score": 0-100,
  "boundary_score": 0-100,
  "verifiability_score": 0-100,
  "technical_score": 0-100,
  "findings": {
    "context": ["发现项1", "发现项2"],
    "atomicity": [...],
    "boundary": [...],
    "verifiability": [...],
    "technical": [...]
  },
  "suggestions": [
    {"category": "context", "priority": "high", "content": "建议内容"},
    ...
  ],
  "risk_heatmap": [
    {"text": "风险片段", "start": 0, "end": 10, "risk_level": "high", "dimension": "context"}
  ]
}
`;
```

### 6.2 评分算法

```typescript
function calculateFinalScore(
  contextScore: number,
  atomicityScore: number,
  boundaryScore: number,
  verifiabilityScore: number,
  technicalScore: number,
  complexity: 'simple' | 'medium' | 'complex'
): { finalScore: number; grade: 'S' | 'A' | 'B' | 'C' } {
  const weights = {
    context: 0.25,
    atomicity: 0.25,
    boundary: 0.20,
    verifiability: 0.15,
    technical: 0.15
  };
  
  const complexityPenalty = {
    simple: 1.0,
    medium: 0.9,
    complex: 0.7
  };
  
  const weightedSum = 
    contextScore * weights.context +
    atomicityScore * weights.atomicity +
    boundaryScore * weights.boundary +
    verifiabilityScore * weights.verifiability +
    technicalScore * weights.technical;
  
  const finalScore = Math.round(weightedSum * complexityPenalty[complexity]);
  
  let grade: 'S' | 'A' | 'B' | 'C';
  if (finalScore >= 90) grade = 'S';
  else if (finalScore >= 75) grade = 'A';
  else if (finalScore >= 60) grade = 'B';
  else grade = 'C';
  
  return { finalScore, grade };
}
```

---

## 7. 实施阶段规划

### 阶段1：基础骨架（第1-2周）
- [x] 项目初始化（Next.js + Express + SQLite）
- [ ] 数据库schema创建
- [ ] 基础API路由实现
- [ ] 用户认证（简单版本）

### 阶段2：评估核心（第3-4周）
- [ ] LLM配置管理
- [ ] 评估引擎实现
- [ ] 评分算法实现
- [ ] 报告生成逻辑

### 阶段3：前端开发（第5-6周）
- [ ] 首页开发
- [ ] 评估报告页开发（图表组件）
- [ ] 历史记录页开发
- [ ] 设置页开发

### 阶段4：完善与优化（第7-8周）
- [ ] 风险热力图优化
- [ ] 优化建议生成质量提升
- [ ] 性能优化
- [ ] 测试与bug修复

---

## 8. 总结

本实施计划完整覆盖了：
1. ✅ 模块划分及依赖关系 - 4个核心模块 + API网关
2. ✅ 数据模型设计 - 4张表 + TypeScript类型定义
3. ✅ API接口定义 - 10+个接口
4. ✅ 前端页面规划 - 4个页面 + 组件列表

下一步：等待确认后启动实施。