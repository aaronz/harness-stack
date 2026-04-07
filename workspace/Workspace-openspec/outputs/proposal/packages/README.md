# AI-Ready Evaluator

AI Coding可落地性评估系统 - 评估需求文档是否能让AI无歧义地生成正确代码。

## 技术栈

- **后端**: Node.js + Express + TypeScript + Prisma + SQLite
- **前端**: Next.js 14 + TypeScript + Tailwind CSS + Zustand + React Query

## 快速开始

### 1. 安装依赖

```bash
cd packages/backend
npm install
cd ../packages/frontend
npm install
```

### 2. 环境配置

编辑 `packages/backend/.env` 文件，设置LLM API密钥：

```
DATABASE_URL="file:./dev.db"
JWT_SECRET="your-secret-key-change-in-production"
PORT=3001
NEXT_PUBLIC_API_URL="http://localhost:3001/api"

# 至少配置一个LLM API Key
OPENAI_API_KEY="your-openai-key"
# ANTHROPIC_API_KEY="your-anthropic-key"
# GOOGLE_API_KEY="your-google-key"
```

### 3. 初始化数据库

```bash
cd packages/backend
npx prisma generate
npx prisma db push
```

### 4. 启动服务

后端：
```bash
cd packages/backend
npm run dev
```

前端：
```bash
cd packages/frontend
npm run dev
```

### 5. 访问系统

- 前端: http://localhost:3000
- 后端API: http://localhost:3001

## 功能特性

1. **需求评估** - 五大维度评估（上下文完备性、逻辑原子性、边界明确性、可验证性、技术约束清晰度）
2. **AI就绪等级** - S/A/B/C四级评分
3. **风险热力图** - 高亮AI可能误解的段落
4. **优化建议** - 针对性改进建议
5. **LLM配置** - 支持OpenAI/Anthropic/Google/本地模型
6. **批量评估** - 一次评估多个需求

## 项目结构

```
packages/
├── backend/
│   ├── src/
│   │   ├── index.ts          # 入口文件
│   │   ├── middleware/       # 中间件
│   │   │   └── auth.ts       # JWT认证
│   │   ├── routes/           # API路由
│   │   │   ├── auth.ts       # 用户认证
│   │   │   ├── evaluation.ts # 评估API
│   │   │   ├── provider.ts   # LLM配置
│   │   │   └── config.ts     # 系统配置
│   │   └── services/         # 业务逻辑
│   │       ├── evaluation.ts # 评估引擎
│   │       └── llm.ts        # LLM集成
│   └── prisma/
│       └── schema.prisma     # 数据库模型
└── frontend/
    ├── src/
    │   ├── app/              # Next.js页面
    │   ├── components/       # UI组件
    │   ├── lib/              # 工具函数
    │   └── store/            # 状态管理
    └── public/               # 静态资源
```

## API文档

### 认证
- `POST /api/auth/register` - 用户注册
- `POST /api/auth/login` - 用户登录
- `POST /api/auth/refresh` - 刷新Token

### 评估
- `POST /api/evaluations` - 创建评估
- `GET /api/evaluations/:id` - 获取评估结果
- `GET /api/evaluations` - 获取评估列表
- `POST /api/evaluations/batch` - 批量评估

### LLM配置
- `GET /api/providers` - 获取LLM提供商列表
- `POST /api/providers` - 添加LLM提供商
- `PUT /api/providers/:id` - 更新LLM提供商
- `DELETE /api/providers/:id` - 删除LLM提供商

### 系统配置
- `GET /api/config` - 获取配置
- `PUT /api/config` - 更新配置

## 评估维度

| 维度 | 权重 | 描述 |
|------|------|------|
| 上下文完备性 | 25% | 业务背景、术语定义、用户旅程、数据实体 |
| 逻辑原子性 | 25% | 功能单一性、上下文窗口、依赖解耦 |
| 边界明确性 | 20% | 输入域、状态转换、并发处理、性能边界 |
| 可验证性 | 15% | 示例驱动、量化标准、自动化测试、行为契约 |
| 技术约束 | 15% | 架构模式、技术栈、接口契约、安全约束 |

## 评分等级

- **S级 (90-100)**: AI可独立完成，无需人工干预
- **A级 (75-89)**: AI可实现，但需人工Review关键边界
- **B级 (60-74)**: AI生成代码后需人工大幅修改
- **C级 (<60)**: 不建议AI实现，风险过高