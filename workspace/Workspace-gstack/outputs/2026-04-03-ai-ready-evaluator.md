# AI-Ready Evaluator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一个AI Coding可落地性评估系统，通过静态分析User Story文本，从五大维度评估AI实现成功率，并生成可视化报告。

**Architecture:** Next.js 14单体架构，使用App Router，SQLite作为数据库，通过OpenAI GPT-4o API进行需求文本分析。Web界面使用Recharts展示五维雷达图。

**Tech Stack:** Next.js 14, TypeScript, SQLite (better-sqlite3), OpenAI API, Recharts, Tailwind CSS

---

## 1. Project Initialization

### Task 1: Initialize Next.js Project

**Files:**
- Create: `package.json`
- Create: `next.config.js`
- Create: `tsconfig.json`
- Create: `tailwind.config.ts`
- Create: `postcss.config.js`

- [ ] **Step 1: Create package.json**

```json
{
  "name": "ai-ready-evaluator",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint"
  },
  "dependencies": {
    "next": "14.2.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "better-sqlite3": "^9.4.0",
    "openai": "^4.28.0",
    "recharts": "^2.12.0",
    "uuid": "^9.0.0",
    "clsx": "^2.1.0"
  },
  "devDependencies": {
    "@types/node": "^20.11.0",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@types/better-sqlite3": "^7.6.0",
    "@types/uuid": "^9.0.0",
    "typescript": "^5.3.0",
    "tailwindcss": "^3.4.0",
    "postcss": "^8.4.0",
    "autoprefixer": "^10.4.0",
    "eslint": "^8.56.0",
    "eslint-config-next": "14.2.0"
  }
}
```

- [ ] **Step 2: Create next.config.js**

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
}

module.exports = nextConfig
```

- [ ] **Step 3: Create tsconfig.json**

```json
{
  "compilerOptions": {
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [{ "name": "next" }],
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
```

- [ ] **Step 4: Create tailwind.config.ts**

```typescript
import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
export default config
```

- [ ] **Step 5: Create postcss.config.js**

```javascript
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

- [ ] **Step 6: Create .env.local**

```env
OPENAI_API_KEY=sk-your-key-here
DEFAULT_MODEL=gpt-4o
```

- [ ] **Step 7: Install dependencies**

```bash
npm install
```

- [ ] **Step 8: Commit**

```bash
git add .
git commit -m "chore: initialize Next.js project with TypeScript and Tailwind"
```

---

## 2. Project Structure

### Task 2: Create Project Directory Structure

**Files:**
- Create: `src/app/layout.tsx`
- Create: `src/app/globals.css`
- Create: `src/lib/db.ts`
- Create: `src/lib/openai.ts`
- Create: `src/types/index.ts`

- [ ] **Step 1: Create src/app/layout.tsx**

```typescript
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import './globals.css'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'AI-Ready Evaluator',
  description: 'AI Coding可落地性评估系统',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="zh-CN">
      <body className={inter.className}>{children}</body>
    </html>
  )
}
```

- [ ] **Step 2: Create src/app/globals.css**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  --foreground-rgb: 0, 0, 0;
  --background-rgb: 255, 255, 255;
}

body {
  color: rgb(var(--foreground-rgb));
  background: rgb(var(--background-rgb));
}

@layer utilities {
  .text-balance {
    text-wrap: balance;
  }
}
```

- [ ] **Step 3: Create src/types/index.ts**

```typescript
export interface Requirement {
  id: string;
  content: string;
  title?: string;
  createdAt: string;
  updatedAt: string;
}

export interface DimensionScore {
  score: number;
  comment: string;
}

export interface EvaluationResult {
  id: string;
  requirementId: string;
  totalScore: number;
  grade: 'S' | 'A' | 'B' | 'C';
  atomicityScore: number;
  boundaryScore: number;
  contextScore: number;
  verifiabilityScore: number;
  technicalScore: number;
  complexityFactor: number;
  llmAnalysis?: string;
  optimizationSuggestions: string[];
  riskMarkers: RiskMarker[];
  createdAt: string;
}

export interface RiskMarker {
  text: string;
  risk: string;
}

export interface ValidationItem {
  id: string;
  content: string;
  expectedGrade: string;
  actualGrade?: string;
  category: string;
  createdAt: string;
}
```

- [ ] **Step 4: Create src/lib/db.ts**

```typescript
import Database from 'better-sqlite3';
import { v4 as uuidv4 } from 'uuid';

let db: Database.Database;

export function getDb() {
  if (!db) {
    db = new Database('ai-evaluator.db');
    initDb();
  }
  return db;
}

function initDb() {
  db.exec(`
    CREATE TABLE IF NOT EXISTS requirements (
      id TEXT PRIMARY KEY,
      content TEXT NOT NULL,
      title TEXT,
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS evaluation_results (
      id TEXT PRIMARY KEY,
      requirement_id TEXT NOT NULL,
      total_score REAL NOT NULL,
      grade TEXT NOT NULL,
      atomicity_score REAL,
      boundary_score REAL,
      context_score REAL,
      verifiability_score REAL,
      technical_score REAL,
      complexity_factor REAL DEFAULT 1.0,
      llm_analysis TEXT,
      optimization_suggestions TEXT,
      risk_markers TEXT,
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (requirement_id) REFERENCES requirements(id)
    );

    CREATE TABLE IF NOT EXISTS validation_set (
      id TEXT PRIMARY KEY,
      content TEXT NOT NULL,
      expected_grade TEXT NOT NULL,
      actual_grade TEXT,
      category TEXT NOT NULL,
      created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );
  `);
}

export function insertRequirement(content: string, title?: string) {
  const db = getDb();
  const id = uuidv4();
  const stmt = db.prepare('INSERT INTO requirements (id, content, title) VALUES (?, ?, ?)');
  stmt.run(id, content, title || null);
  return id;
}

export function insertEvaluationResult(data: {
  requirementId: string;
  totalScore: number;
  grade: string;
  atomicityScore: number;
  boundaryScore: number;
  contextScore: number;
  verifiabilityScore: number;
  technicalScore: number;
  complexityFactor: number;
  llmAnalysis?: string;
  optimizationSuggestions: string[];
  riskMarkers: { text: string; risk: string }[];
}) {
  const db = getDb();
  const id = uuidv4();
  const stmt = db.prepare(`
    INSERT INTO evaluation_results (
      id, requirement_id, total_score, grade,
      atomicity_score, boundary_score, context_score, verifiability_score, technical_score,
      complexity_factor, llm_analysis, optimization_suggestions, risk_markers
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
  `);
  stmt.run(
    id,
    data.requirementId,
    data.totalScore,
    data.grade,
    data.atomicityScore,
    data.boundaryScore,
    data.contextScore,
    data.verifiabilityScore,
    data.technicalScore,
    data.complexityFactor,
    data.llmAnalysis || null,
    JSON.stringify(data.optimizationSuggestions),
    JSON.stringify(data.riskMarkers)
  );
  return id;
}

export function getEvaluationResult(id: string) {
  const db = getDb();
  const stmt = db.prepare('SELECT * FROM evaluation_results WHERE id = ?');
  return stmt.get(id);
}
```

- [ ] **Step 5: Create src/lib/openai.ts**

```typescript
import OpenAI from 'openai';

const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
});

export async function analyzeRequirement(requirement: string) {
  const prompt = `你是一个AI需求质量评估专家。请分析以下User Story，从5个维度评估其AI实现可行性：

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
${requirement}
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
}`;

  const response = await openai.chat.completions.create({
    model: process.env.DEFAULT_MODEL || 'gpt-4o',
    messages: [{ role: 'user', content: prompt }],
    response_format: { type: 'json_object' },
  });

  const content = response.choices[0]?.message?.content;
  if (!content) {
    throw new Error('Failed to get response from OpenAI');
  }

  return JSON.parse(content);
}
```

- [ ] **Step 6: Commit**

```bash
git add .
git commit -m "chore: create project structure and core libraries"
```

---

## 3. API Routes

### Task 3: Create Evaluation API

**Files:**
- Create: `src/app/api/evaluate/route.ts`

- [ ] **Step 1: Create src/app/api/evaluate/route.ts**

```typescript
import { NextRequest, NextResponse } from 'next/server';
import { insertRequirement, insertEvaluationResult } from '@/lib/db';
import { analyzeRequirement } from '@/lib/openai';

interface DimensionScores {
  atomicity: { score: number; comment: string };
  boundary: { score: number; comment: string };
  context: { score: number; comment: string };
  verifiability: { score: number; comment: string };
  technical: { score: number; comment: string };
}

interface RiskMarker {
  text: string;
  risk: string;
}

function calculateGrade(totalScore: number): 'S' | 'A' | 'B' | 'C' {
  if (totalScore >= 90) return 'S';
  if (totalScore >= 75) return 'A';
  if (totalScore >= 60) return 'B';
  return 'C';
}

function detectComplexity(requirement: string): number {
  const complexityKeywords = [
    '算法', '架构', '设计模式', '分布式', '微服务',
    'AI', '机器学习', '大数据', '实时', '并发',
    'transaction', 'distributed', 'concurrent'
  ];
  
  const matches = complexityKeywords.filter(keyword => 
    requirement.toLowerCase().includes(keyword.toLowerCase())
  );
  
  if (matches.length >= 2) return 0.7;
  if (matches.length === 1) return 0.9;
  return 1.0;
}

export async function POST(request: NextRequest) {
  try {
    const { requirement } = await request.json();

    if (!requirement || requirement.trim().length === 0) {
      return NextResponse.json(
        { success: false, error: 'Requirement is required' },
        { status: 400 }
      );
    }

    if (requirement.length > 10000) {
      return NextResponse.json(
        { success: false, error: 'Requirement too long (max 10000 characters)' },
        { status: 400 }
      );
    }

    // Save requirement to DB
    const requirementId = insertRequirement(requirement);

    // Analyze with LLM
    const analysis = await analyzeRequirement(requirement);
    const scores = analysis.scores as DimensionScores;
    const riskMarkers = analysis.riskMarkers as RiskMarker[];
    const suggestions = analysis.suggestions as string[];

    // Calculate weighted score
    const weights = {
      atomicity: 0.30,
      boundary: 0.25,
      context: 0.20,
      verifiability: 0.15,
      technical: 0.10,
    };

    const complexityFactor = detectComplexity(requirement);
    
    const totalScore = (
      scores.atomicity.score * weights.atomicity +
      scores.boundary.score * weights.boundary +
      scores.context.score * weights.context +
      scores.verifiability.score * weights.verifiability +
      scores.technical.score * weights.technical
    ) * complexityFactor;

    const grade = calculateGrade(totalScore);

    // Save evaluation result
    const evaluationId = insertEvaluationResult({
      requirementId,
      totalScore: Math.round(totalScore),
      grade,
      atomicityScore: scores.atomicity.score,
      boundaryScore: scores.boundary.score,
      contextScore: scores.context.score,
      verifiabilityScore: scores.verifiability.score,
      technicalScore: scores.technical.score,
      complexityFactor,
      llmAnalysis: JSON.stringify(scores),
      optimizationSuggestions: suggestions,
      riskMarkers,
    });

    return NextResponse.json({
      success: true,
      data: {
        id: evaluationId,
        totalScore: Math.round(totalScore),
        grade,
        dimensions: {
          atomicity: { score: scores.atomicity.score, comment: scores.atomicity.comment },
          boundary: { score: scores.boundary.score, comment: scores.boundary.comment },
          context: { score: scores.context.score, comment: scores.context.comment },
          verifiability: { score: scores.verifiability.score, comment: scores.verifiability.comment },
          technical: { score: scores.technical.score, comment: scores.technical.comment },
        },
        complexityFactor,
        suggestions,
        riskMarkers,
      },
    });
  } catch (error) {
    console.error('Evaluation error:', error);
    return NextResponse.json(
      { success: false, error: 'Internal server error' },
      { status: 500 }
    );
  }
}
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add evaluation API endpoint"
```

---

## 4. Frontend Components

### Task 4: Create Evaluation Form Component

**Files:**
- Create: `src/components/EvaluationForm.tsx`

- [ ] **Step 1: Create src/components/EvaluationForm.tsx**

```typescript
'use client';

import { useState } from 'react';

interface EvaluationFormProps {
  onSubmit: (requirement: string) => Promise<void>;
}

export default function EvaluationForm({ onSubmit }: EvaluationFormProps) {
  const [requirement, setRequirement] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!requirement.trim()) {
      setError('请输入需求内容');
      return;
    }

    setLoading(true);
    setError('');

    try {
      await onSubmit(requirement);
    } catch (err) {
      setError('评估失败，请重试');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label htmlFor="requirement" className="block text-sm font-medium mb-2">
          输入您的User Story
        </label>
        <textarea
          id="requirement"
          value={requirement}
          onChange={(e) => setRequirement(e.target.value)}
          placeholder="例如：实现用户注册功能。输入：用户名（6-20位字母数字），密码（8-32位包含大小写字母和数字）。输出：用户对象或错误码..."
          className="w-full h-64 p-4 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
          disabled={loading}
        />
      </div>

      {error && (
        <div className="text-red-500 text-sm">{error}</div>
      )}

      <button
        type="submit"
        disabled={loading || !requirement.trim()}
        className="w-full py-3 px-6 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
      >
        {loading ? '评估中...' : '开始评估'}
      </button>
    </form>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add evaluation form component"
```

---

### Task 5: Create Radar Chart Component

**Files:**
- Create: `src/components/RadarChart.tsx`

- [ ] **Step 1: Create src/components/RadarChart.tsx**

```typescript
'use client';

import {
  Radar,
  RadarChart,
  PolarGrid,
  PolarAngleAxis,
  PolarRadiusAxis,
  ResponsiveContainer,
} from 'recharts';

interface RadarChartProps {
  data: {
    subject: string;
    score: number;
    fullMark: number;
  }[];
}

const dimensionLabels = {
  atomicity: '逻辑原子性',
  boundary: '边界明确性',
  context: '上下文完备性',
  verifiability: '可验证性',
  technical: '技术约束清晰度',
};

export default function RadarChartComponent({ data }: RadarChartProps) {
  return (
    <ResponsiveContainer width="100%" height={300}>
      <RadarChart cx="50%" cy="50%" outerRadius="70%" data={data}>
        <PolarGrid stroke="#e5e7eb" />
        <PolarAngleAxis
          dataKey="subject"
          tick={{ fill: '#374151', fontSize: 12 }}
        />
        <PolarRadiusAxis
          angle={30}
          domain={[0, 100]}
          tick={{ fill: '#9ca3af', fontSize: 10 }}
        />
        <Radar
          name="得分"
          dataKey="score"
          stroke="#3b82f6"
          fill="#3b82f6"
          fillOpacity={0.3}
        />
      </RadarChart>
    </ResponsiveContainer>
  );
}

export function getRadarChartData(dimensions: {
  atomicity: { score: number };
  boundary: { score: number };
  context: { score: number };
  verifiability: { score: number };
  technical: { score: number };
}) {
  return [
    { subject: dimensionLabels.atomicity, score: dimensions.atomicity.score, fullMark: 100 },
    { subject: dimensionLabels.boundary, score: dimensions.boundary.score, fullMark: 100 },
    { subject: dimensionLabels.context, score: dimensions.context.score, fullMark: 100 },
    { subject: dimensionLabels.verifiability, score: dimensions.verifiability.score, fullMark: 100 },
    { subject: dimensionLabels.technical, score: dimensions.technical.score, fullMark: 100 },
  ];
}
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add radar chart component"
```

---

### Task 6: Create Result Card Components

**Files:**
- Create: `src/components/ResultCard.tsx`

- [ ] **Step 1: Create src/components/ResultCard.tsx**

```typescript
interface ResultCardProps {
  grade: string;
  totalScore: number;
  dimensions: {
    atomicity: { score: number; comment: string };
    boundary: { score: number; comment: string };
    context: { score: number; comment: string };
    verifiability: { score: number; comment: string };
    technical: { score: number; comment: string };
  };
  suggestions: string[];
}

const gradeColors = {
  S: 'bg-green-500',
  A: 'bg-blue-500',
  B: 'bg-yellow-500',
  C: 'bg-red-500',
};

const gradeLabels = {
  S: 'AI可独立完成',
  A: 'AI可实现，需Review',
  B: '需人工大幅修改',
  C: '不建议AI实现',
};

export default function ResultCard({
  grade,
  totalScore,
  dimensions,
  suggestions,
}: ResultCardProps) {
  return (
    <div className="space-y-6">
      {/* Grade Banner */}
      <div className={`${gradeColors[grade as keyof typeof gradeColors]} text-white rounded-lg p-6 text-center`}>
        <div className="text-6xl font-bold">{grade}级</div>
        <div className="text-xl mt-2">{gradeLabels[grade as keyof typeof gradeLabels]}</div>
        <div className="text-3xl mt-2">总分: {totalScore}</div>
      </div>

      {/* Dimension Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {Object.entries(dimensions).map(([key, value]) => (
          <div key={key} className="border rounded-lg p-4">
            <div className="flex justify-between items-center mb-2">
              <span className="font-medium">{getDimensionName(key)}</span>
              <span className="text-lg font-bold">{value.score}</span>
            </div>
            <p className="text-sm text-gray-600">{value.comment}</p>
          </div>
        ))}
      </div>

      {/* Suggestions */}
      {suggestions.length > 0 && (
        <div className="border rounded-lg p-4">
          <h3 className="font-medium mb-3">优化建议</h3>
          <ul className="space-y-2">
            {suggestions.map((suggestion, index) => (
              <li key={index} className="flex items-start gap-2">
                <span className="text-blue-500">•</span>
                <span>{suggestion}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}

function getDimensionName(key: string): string {
  const names: Record<string, string> = {
    atomicity: '逻辑原子性',
    boundary: '边界明确性',
    context: '上下文完备性',
    verifiability: '可验证性',
    technical: '技术约束清晰度',
  };
  return names[key] || key;
}
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add result card component"
```

---

## 5. Pages

### Task 7: Create Home Page

**Files:**
- Create: `src/app/page.tsx`

- [ ] **Step 1: Create src/app/page.tsx**

```typescript
'use client';

import { useRouter } from 'next/navigation';
import EvaluationForm from '@/components/EvaluationForm';

export default function Home() {
  const router = useRouter();

  const handleSubmit = async (requirement: string) => {
    const response = await fetch('/api/evaluate', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ requirement }),
    });

    const data = await response.json();

    if (data.success) {
      router.push(`/result/${data.data.id}`);
    } else {
      throw new Error(data.error);
    }
  };

  return (
    <main className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto py-12 px-4">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-900 mb-2">
            AI-Ready Evaluator
          </h1>
          <p className="text-gray-600">
            AI Coding可落地性评估系统
          </p>
        </div>

        <div className="bg-white rounded-lg shadow-md p-6">
          <EvaluationForm onSubmit={handleSubmit} />
        </div>

        <div className="mt-8 text-center text-sm text-gray-500">
          <p>评估维度：逻辑原子性 · 边界明确性 · 上下文完备性 · 可验证性 · 技术约束清晰度</p>
        </div>
      </div>
    </main>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add home page"
```

---

### Task 8: Create Result Page

**Files:**
- Create: `src/app/result/[id]/page.tsx`

- [ ] **Step 1: Create src/app/result/[id]/page.tsx**

```typescript
'use client';

import { useEffect, useState } from 'react';
import { useParams, useRouter } from 'next/navigation';
import RadarChartComponent, { getRadarChartData } from '@/components/RadarChart';
import ResultCard from '@/components/ResultCard';

interface EvaluationData {
  id: string;
  totalScore: number;
  grade: string;
  dimensions: {
    atomicity: { score: number; comment: string };
    boundary: { score: number; comment: string };
    context: { score: number; comment: string };
    verifiability: { score: number; comment: string };
    technical: { score: number; comment: string };
  };
  suggestions: string[];
  riskMarkers: { text: string; risk: string }[];
}

export default function ResultPage() {
  const params = useParams();
  const router = useRouter();
  const [data, setData] = useState<EvaluationData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');

  useEffect(() => {
    async function fetchResult() {
      try {
        const response = await fetch(`/api/result/${params.id}`);
        const result = await response.json();

        if (result.success) {
          setData(result.data);
        } else {
          setError(result.error || '获取结果失败');
        }
      } catch (err) {
        setError('加载失败，请重试');
      } finally {
        setLoading(false);
      }
    }

    if (params.id) {
      fetchResult();
    }
  }, [params.id]);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-gray-600">加载中...</div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="min-h-screen flex flex-col items-center justify-center">
        <div className="text-red-500 mb-4">{error || '结果不存在'}</div>
        <button
          onClick={() => router.push('/')}
          className="text-blue-500 hover:underline"
        >
          返回首页
        </button>
      </div>
    );
  }

  const radarData = getRadarChartData(data.dimensions);

  return (
    <main className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto py-8 px-4">
        <button
          onClick={() => router.push('/')}
          className="text-blue-500 hover:underline mb-4"
        >
          ← 评估新需求
        </button>

        <ResultCard
          grade={data.grade}
          totalScore={data.totalScore}
          dimensions={data.dimensions}
          suggestions={data.suggestions}
        />

        {/* Radar Chart */}
        <div className="mt-8 bg-white rounded-lg shadow-md p-6">
          <h2 className="text-xl font-bold mb-4">五维得分图</h2>
          <RadarChartComponent data={radarData} />
        </div>

        {/* Risk Markers */}
        {data.riskMarkers.length > 0 && (
          <div className="mt-8 bg-white rounded-lg shadow-md p-6">
            <h2 className="text-xl font-bold mb-4">风险标记</h2>
            <div className="space-y-2">
              {data.riskMarkers.map((marker, index) => (
                <div key={index} className="flex items-start gap-2 text-sm">
                  <span className="text-red-500 font-medium">⚠️</span>
                  <span>
                    <strong>"{marker.text}"</strong> - {marker.risk}
                  </span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </main>
  );
}
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add result page"
```

---

### Task 9: Create Result API

**Files:**
- Create: `src/app/api/result/[id]/route.ts`

- [ ] **Step 1: Create src/app/api/result/[id]/route.ts**

```typescript
import { NextRequest, NextResponse } from 'next/server';
import { getEvaluationResult } from '@/lib/db';

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const result = getEvaluationResult(params.id);

    if (!result) {
      return NextResponse.json(
        { success: false, error: 'Result not found' },
        { status: 404 }
      );
    }

    const riskMarkers = result.risk_markers ? JSON.parse(result.risk_markers as string) : [];
    const optimizationSuggestions = result.optimization_suggestions 
      ? JSON.parse(result.optimization_suggestions as string) 
      : [];

    return NextResponse.json({
      success: true,
      data: {
        id: result.id,
        totalScore: result.total_score,
        grade: result.grade,
        dimensions: {
          atomicity: { score: result.atomicity_score },
          boundary: { score: result.boundary_score },
          context: { score: result.context_score },
          verifiability: { score: result.verifiability_score },
          technical: { score: result.technical_score },
        },
        complexityFactor: result.complexity_factor,
        suggestions: optimizationSuggestions,
        riskMarkers,
      },
    });
  } catch (error) {
    console.error('Get result error:', error);
    return NextResponse.json(
      { success: false, error: 'Internal server error' },
      { status: 500 }
    );
  }
}
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add result API endpoint"
```

---

## 6. Validation Set

### Task 10: Create Validation Set Seeder

**Files:**
- Create: `src/scripts/seed-validation.ts`

- [ ] **Step 1: Create src/scripts/seed-validation.ts**

```typescript
const validationData = [
  {
    content: '实现用户注册功能。输入：用户名（6-20位字母数字下划线，正则^[a-zA-Z0-9_]{6,20}$），密码（8-32位，必须包含大小写字母和数字）。输出：用户对象（包含userId、username、createdAt）或错误码（1001:用户名已存在，1002:参数非法）。使用Spring Boot 2.7，数据库存储密码需BCrypt加密。提供单元测试覆盖所有错误码分支。',
    expectedGrade: 'S',
    category: 'crud',
  },
  {
    content: '做一个智能推荐模块，根据用户喜好推荐商品，要求推荐准确率高，用户体验好。使用最新技术实现。',
    expectedGrade: 'C',
    category: 'innovation',
  },
  {
    content: '用户登录功能。用户输入账号密码，系统验证通过后返回Token。',
    expectedGrade: 'B',
    category: 'simple',
  },
  {
    content: '实现订单创建API。输入：商品ID、数量、收货地址。验证库存充足后创建订单，扣减库存，发送消息到消息队列。需处理并发超卖问题。使用Redis分布式锁保证原子性。',
    expectedGrade: 'A',
    category: 'business',
  },
  {
    content: '实现用户管理功能，包括用户增删改查。',
    expectedGrade: 'C',
    category: 'vague',
  },
];

export default validationData;
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "feat: add validation set data"
```

---

## 7. Build and Test

### Task 11: Build and Verify

**Files:**
- Modify: `package.json` (add build script check)

- [ ] **Step 1: Run build**

```bash
npm run build
```

Expected: Build should complete without errors

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "chore: build and verify project"
```

---

## 8. Final Setup

### Task 12: Create README

**Files:**
- Create: `README.md`

- [ ] **Step 1: Create README.md**

```markdown
# AI-Ready Evaluator

AI Coding可落地性评估系统 - 评估User Story是否能让AI无歧义地生成正确代码。

## 功能

- 五大维度评估：逻辑原子性、边界明确性、上下文完备性、可验证性、技术约束清晰度
- AI就绪等级：S/A/B/C
- 可视化报告：五维雷达图 + 优化建议 + 风险标记

## 快速开始

1. 克隆项目
2. 安装依赖：`npm install`
3. 配置环境变量：复制`.env.local.example`为`.env.local`，填入`OPENAI_API_KEY`
4. 启动开发服务器：`npm run dev`
5. 访问 http://localhost:3000

## 技术栈

- Next.js 14
- TypeScript
- SQLite (better-sqlite3)
- OpenAI API
- Recharts
- Tailwind CSS
```

- [ ] **Step 2: Commit**

```bash
git add .
git commit -m "docs: add README"
```

---

## 执行方式

**Plan complete and saved to `docs/superpowers/plans/2026-04-03-ai-ready-evaluator.md`.**

**Two execution options:**

1. **Subagent-Driven (recommended)** -  dispatch a fresh subagent per task, review between tasks, fast iteration
2. **Inline Execution** - execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
