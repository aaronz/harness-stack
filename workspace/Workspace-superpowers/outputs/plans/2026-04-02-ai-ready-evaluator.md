# AI-Ready Evaluator Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build an AI-Ready Evaluator system that assesses development requirements for AI Coding implementability, using Next.js fullstack with SQLite.

**Architecture:** Next.js 14 fullstack application with API Routes, Prisma ORM for SQLite, multi-LLM support via configurable providers. Single repository deployment.

**Tech Stack:** Next.js 14, TypeScript 5, Prisma ORM, SQLite, OpenAI/Anthropic/Gemini/Ollama SDKs, Zod for validation.

---

## File Structure

```
├── prisma/
│   └── schema.prisma          # Database schema
├── src/
│   ├── app/
│   │   ├── page.tsx           # Home page (upload + history)
│   │   ├── layout.tsx         # Root layout
│   │   ├── evaluations/
│   │   │   └── [id]/
│   │   │       └── page.tsx   # Evaluation report page
│   │   └── settings/
│   │       └── page.tsx       # LLM settings page
│   ├── components/
│   │   ├── FileUpload.tsx     # File upload component
│   │   ├── EvaluationList.tsx # History list component
│   │   ├── ScoreCard.tsx      # Score display component
│   │   └── LLMConfigForm.tsx  # LLM configuration form
│   ├── lib/
│   │   ├── prisma.ts          # Prisma client singleton
│   │   ├── llm.ts             # LLM service (multi-provider)
│   │   ├── evaluator.ts       # Evaluation prompt & parsing
│   │   └── encryption.ts      # API key encryption utility
│   └── api/
│       ├── evaluations/
│       │   ├── route.ts       # POST/GET /api/evaluations
│       │   └── [id]/
│       │       └── route.ts   # GET/DELETE /api/evaluations/[id]
│       └── config/
│           └── llm/
│               ├── route.ts   # GET/POST /api/config/llm
│               └── [id]/
│                   └── route.ts # PUT/DELETE /api/config/llm/[id]
├── .env                       # Environment variables
├── package.json
├── next.config.js
└── tsconfig.json
```

---

## Task 1: Project Setup & Database Schema

**Files:**
- Create: `package.json`
- Create: `tsconfig.json`
- Create: `next.config.js`
- Create: `prisma/schema.prisma`
- Create: `.env`
- Create: `src/lib/prisma.ts`

- [ ] **Step 1: Create package.json**

```json
{
  "name": "ai-ready-evaluator",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "prisma generate && next build",
    "start": "next start",
    "lint": "next lint",
    "db:push": "prisma db push",
    "db:studio": "prisma studio"
  },
  "dependencies": {
    "next": "14.2.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@prisma/client": "^5.10.0",
    "zod": "^3.22.0",
    "openai": "^4.28.0",
    "@anthropic-ai/sdk": "^0.20.0",
    "@google/generative-ai": "^0.2.0",
    "formidable": "^3.5.0",
    "crypto-js": "^4.2.0"
  },
  "devDependencies": {
    "typescript": "^5.3.0",
    "@types/node": "^20.11.0",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@types/formidable": "^3.4.0",
    "@types/crypto-js": "^4.2.0",
    "prisma": "^5.10.0",
    "eslint": "^8.56.0",
    "eslint-config-next": "14.2.0"
  }
}
```

- [ ] **Step 2: Create tsconfig.json**

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

- [ ] **Step 3: Create next.config.js**

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    serverActions: true,
  },
}

module.exports = nextConfig
```

- [ ] **Step 4: Create prisma/schema.prisma**

```prisma
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "sqlite"
  url      = env("DATABASE_URL")
}

model LLMConfig {
  id        String   @id @default(cuid())
  name      String
  provider  String
  model     String
  apiKey    String
  baseUrl   String?
  isDefault Boolean  @default(false)
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
}

model Evaluation {
  id                  String   @id @default(cuid())
  title               String
  content             String
  fileName            String?
  fileType            String?
  overallScore        Int
  grade               String
  complexity          String
  contextScore        Int
  atomicityScore      Int
  boundaryScore       Int
  verifiabilityScore  Int
  techScore           Int
  rawResponse         String
  suggestions         String
  modelUsed           String
  createdAt           DateTime @default(now())
}
```

- [ ] **Step 5: Create .env**

```env
# Database
DATABASE_URL="file:./dev.db"

# LLM API Keys (at least one required)
OPENAI_API_KEY=""
ANTHROPIC_API_KEY=""
GEMINI_API_KEY=""

# Ollama (optional, for local models)
OLLAMA_BASE_URL="http://localhost:11434"

# Encryption key (32 bytes for AES-256)
ENCRYPTION_KEY="your-32-byte-encryption-key-here"
```

- [ ] **Step 6: Create src/lib/prisma.ts**

```typescript
import { PrismaClient } from '@prisma/client'

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined
}

export const prisma = globalForPrisma.prisma ?? new PrismaClient()

if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma
```

- [ ] **Step 7: Install dependencies and generate Prisma client**

Run: `npm install && npx prisma generate`
Expected: Dependencies installed, Prisma client generated

- [ ] **Step 8: Push database schema**

Run: `npx prisma db push`
Expected: SQLite database created with schema

- [ ] **Step 9: Commit**

```bash
git add package.json tsconfig.json next.config.js prisma/schema.prisma .env src/lib/prisma.ts
git commit -m "feat: project setup with Prisma schema"
```

---

## Task 2: LLM Service (Multi-Provider Support)

**Files:**
- Create: `src/lib/encryption.ts`
- Create: `src/lib/llm.ts`

- [ ] **Step 1: Create src/lib/encryption.ts**

```typescript
import CryptoJS from 'crypto-js'

const ENCRYPTION_KEY = process.env.ENCRYPTION_KEY || ''

export function encryptApiKey(apiKey: string): string {
  return CryptoJS.AES.encrypt(apiKey, ENCRYPTION_KEY).toString()
}

export function decryptApiKey(encrypted: string): string {
  const bytes = CryptoJS.AES.decrypt(encrypted, ENCRYPTION_KEY)
  return bytes.toString(CryptoJS.enc.Utf8)
}
```

- [ ] **Step 2: Create src/lib/llm.ts**

```typescript
import OpenAI from 'openai'
import Anthropic from '@anthropic-ai/sdk'
import { GoogleGenerativeAI } from '@google/generative-ai'
import { decryptApiKey } from './encryption'

export type LLMProvider = 'openai' | 'anthropic' | 'google' | 'ollama'

export interface LLMConfig {
  provider: LLMProvider
  model: string
  apiKey: string
  baseUrl?: string
}

export interface EvaluationResult {
  scores: {
    context: number
    atomicity: number
    boundary: number
    verifiability: number
    tech: number
  }
  overall: number
  grade: 'S' | 'A' | 'B' | 'C'
  complexity: 'simple' | 'medium' | 'complex'
  risks: string[]
  suggestions: Array<{ type: string; content: string }>
}

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
{CONTENT}
---

## 输出要求

请返回 JSON 格式（不要有其他内容）:
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
`

export async function callLLM(config: LLMConfig, content: string): Promise<EvaluationResult> {
  const prompt = EVALUATION_PROMPT.replace('{CONTENT}', content)
  
  switch (config.provider) {
    case 'openai':
      return callOpenAI(config, prompt)
    case 'anthropic':
      return callAnthropic(config, prompt)
    case 'google':
      return callGoogle(config, prompt)
    case 'ollama':
      return callOllama(config, prompt)
    default:
      throw new Error(`Unknown provider: ${config.provider}`)
  }
}

async function callOpenAI(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const client = new OpenAI({
    apiKey: config.apiKey,
    baseURL: config.baseUrl,
  })
  
  const response = await client.chat.completions.create({
    model: config.model,
    messages: [{ role: 'user', content: prompt }],
    temperature: 0.3,
  })
  
  const content = response.choices[0]?.message?.content || ''
  return parseResponse(content)
}

async function callAnthropic(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const client = new Anthropic({
    apiKey: config.apiKey,
    baseURL: config.baseUrl,
  })
  
  const response = await client.messages.create({
    model: config.model,
    max_tokens: 4096,
    messages: [{ role: 'user', content: prompt }],
  })
  
  const content = response.content[0]?.type === 'text' ? response.content[0].text : ''
  return parseResponse(content)
}

async function callGoogle(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const client = new GoogleGenerativeAI(config.apiKey)
  const model = client.getGenerativeModel({ model: config.model })
  
  const result = await model.generateContent(prompt)
  const content = result.response.text()
  return parseResponse(content)
}

async function callOllama(config: LLMConfig, prompt: string): Promise<EvaluationResult> {
  const baseUrl = config.baseUrl || 'http://localhost:11434'
  
  const response = await fetch(`${baseUrl}/api/generate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      model: config.model,
      prompt,
      stream: false,
    }),
  })
  
  const data = await response.json() as { response: string }
  return parseResponse(data.response)
}

function parseResponse(content: string): EvaluationResult {
  // Extract JSON from response
  const jsonMatch = content.match(/\{[\s\S]*\}/)
  if (!jsonMatch) {
    throw new Error('Failed to parse LLM response')
  }
  
  const parsed = JSON.parse(jsonMatch[0])
  
  return {
    scores: {
      context: Math.min(100, Math.max(0, parsed.scores?.context || 0)),
      atomicity: Math.min(100, Math.max(0, parsed.scores?.atomicity || 0)),
      boundary: Math.min(100, Math.max(0, parsed.scores?.boundary || 0)),
      verifiability: Math.min(100, Math.max(0, parsed.scores?.verifiability || 0)),
      tech: Math.min(100, Math.max(0, parsed.scores?.tech || 0)),
    },
    overall: Math.min(100, Math.max(0, parsed.overall || 0)),
    grade: parsed.grade || 'C',
    complexity: parsed.complexity || 'medium',
    risks: Array.isArray(parsed.risks) ? parsed.risks : [],
    suggestions: Array.isArray(parsed.suggestions) ? parsed.suggestions : [],
  }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/lib/encryption.ts src/lib/llm.ts
git commit -m "feat: add LLM service with multi-provider support"
```

---

## Task 3: Evaluator Service

**Files:**
- Create: `src/lib/evaluator.ts`

- [ ] **Step 1: Create src/lib/evaluator.ts**

```typescript
import { callLLM, LLMConfig, EvaluationResult } from './llm'

export interface ScoreWeights {
  context: number
  atomicity: number
  boundary: number
  verifiability: number
  tech: number
}

export const DEFAULT_WEIGHTS: ScoreWeights = {
  context: 0.25,
  atomicity: 0.25,
  boundary: 0.20,
  verifiability: 0.15,
  tech: 0.15,
}

export const COMPLEXITY_PENALTY: Record<string, number> = {
  simple: 1.0,
  medium: 0.9,
  complex: 0.7,
}

export interface EvaluatedResult extends EvaluationResult {
  overallScore: number
}

export async function evaluateRequirement(
  content: string,
  llmConfig: LLMConfig,
  weights: ScoreWeights = DEFAULT_WEIGHTS
): Promise<EvaluatedResult> {
  // Call LLM to get evaluation
  const result = await callLLM(llmConfig, content)
  
  // Calculate weighted score with complexity penalty
  const rawScore =
    result.scores.context * weights.context +
    result.scores.atomicity * weights.atomicity +
    result.scores.boundary * weights.boundary +
    result.scores.verifiability * weights.verifiability +
    result.scores.tech * weights.tech
  
  const penalty = COMPLEXITY_PENALTY[result.complexity] || 1.0
  const overallScore = Math.round(rawScore * penalty)
  
  // Determine grade
  let grade: 'S' | 'A' | 'B' | 'C'
  if (overallScore >= 90) grade = 'S'
  else if (overallScore >= 75) grade = 'A'
  else if (overallScore >= 60) grade = 'B'
  else grade = 'C'
  
  return {
    ...result,
    overallScore,
    grade,
  }
}
```

- [ ] **Step 2: Commit**

```bash
git add src/lib/evaluator.ts
git commit -m "feat: add evaluator service with scoring algorithm"
```

---

## Task 4: API Routes - Evaluations

**Files:**
- Create: `src/app/api/evaluations/route.ts`
- Create: `src/app/api/evaluations/[id]/route.ts`

- [ ] **Step 1: Create src/app/api/evaluations/route.ts**

```typescript
import { NextRequest, NextResponse } from 'next/server'
import { writeFile, mkdir } from 'fs/promises'
import { join } from 'path'
import { prisma } from '@/lib/prisma'
import { evaluateRequirement } from '@/lib/evaluator'
import { decryptApiKey } from '@/lib/encryption'
import { LLMConfig } from '@/lib/llm'

export async function POST(request: NextRequest) {
  try {
    const formData = await request.formData()
    const file = formData.get('file') as File | null
    const title = formData.get('title') as string | null
    const modelId = formData.get('modelId') as string | null
    
    if (!file) {
      return NextResponse.json({ error: 'No file provided' }, { status: 400 })
    }
    
    // Read file content
    const buffer = await file.arrayBuffer()
    const content = Buffer.from(buffer).toString('utf-8')
    
    // Get LLM config
    let llmConfig: LLMConfig
    if (modelId) {
      const config = await prisma.lLMConfig.findUnique({ where: { id: modelId } })
      if (!config) {
        return NextResponse.json({ error: 'Invalid model ID' }, { status: 400 })
      }
      llmConfig = {
        provider: config.provider as LLMConfig['provider'],
        model: config.model,
        apiKey: decryptApiKey(config.apiKey),
        baseUrl: config.baseUrl || undefined,
      }
    } else {
      // Use default config
      const defaultConfig = await prisma.lLMConfig.findFirst({ where: { isDefault: true } })
      if (!defaultConfig) {
        return NextResponse.json({ 
          error: 'No default LLM configured. Please add a model in Settings.' 
        }, { status: 400 })
      }
      llmConfig = {
        provider: defaultConfig.provider as LLMConfig['provider'],
        model: defaultConfig.model,
        apiKey: decryptApiKey(defaultConfig.apiKey),
        baseUrl: defaultConfig.baseUrl || undefined,
      }
    }
    
    // Evaluate
    const result = await evaluateRequirement(content, llmConfig)
    
    // Save to database
    const evaluation = await prisma.evaluation.create({
      data: {
        title: title || file.name.replace(/\.(md|txt)$/, '') || 'Untitled',
        content,
        fileName: file.name,
        fileType: file.name.endsWith('.md') ? 'markdown' : 'txt',
        overallScore: result.overallScore,
        grade: result.grade,
        complexity: result.complexity,
        contextScore: result.scores.context,
        atomicityScore: result.scores.atomicity,
        boundaryScore: result.scores.boundary,
        verifiabilityScore: result.scores.verifiability,
        techScore: result.scores.tech,
        rawResponse: JSON.stringify(result),
        suggestions: JSON.stringify(result.suggestions),
        modelUsed: llmConfig.model,
      },
    })
    
    return NextResponse.json({
      id: evaluation.id,
      overallScore: evaluation.overallScore,
      grade: evaluation.grade,
      contextScore: evaluation.contextScore,
      atomicityScore: evaluation.atomicityScore,
      boundaryScore: evaluation.boundaryScore,
      verifiabilityScore: evaluation.verifiabilityScore,
      techScore: evaluation.techScore,
      suggestions: result.suggestions,
      createdAt: evaluation.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Evaluation error:', error)
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Evaluation failed' },
      { status: 500 }
    )
  }
}

export async function GET() {
  try {
    const evaluations = await prisma.evaluation.findMany({
      orderBy: { createdAt: 'desc' },
      take: 50,
    })
    
    return NextResponse.json(evaluations.map(e => ({
      id: e.id,
      title: e.title,
      fileName: e.fileName,
      overallScore: e.overallScore,
      grade: e.grade,
      complexity: e.complexity,
      createdAt: e.createdAt.toISOString(),
    })))
  } catch (error) {
    console.error('List evaluations error:', error)
    return NextResponse.json({ error: 'Failed to list evaluations' }, { status: 500 })
  }
}
```

- [ ] **Step 2: Create src/app/api/evaluations/[id]/route.ts**

```typescript
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const evaluation = await prisma.evaluation.findUnique({
      where: { id: params.id },
    })
    
    if (!evaluation) {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }
    
    return NextResponse.json({
      id: evaluation.id,
      title: evaluation.title,
      content: evaluation.content,
      fileName: evaluation.fileName,
      fileType: evaluation.fileType,
      overallScore: evaluation.overallScore,
      grade: evaluation.grade,
      complexity: evaluation.complexity,
      contextScore: evaluation.contextScore,
      atomicityScore: evaluation.atomicityScore,
      boundaryScore: evaluation.boundaryScore,
      verifiabilityScore: evaluation.verifiabilityScore,
      techScore: evaluation.techScore,
      rawResponse: evaluation.rawResponse,
      suggestions: JSON.parse(evaluation.suggestions),
      modelUsed: evaluation.modelUsed,
      createdAt: evaluation.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Get evaluation error:', error)
    return NextResponse.json({ error: 'Failed to get evaluation' }, { status: 500 })
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    await prisma.evaluation.delete({
      where: { id: params.id },
    })
    
    return NextResponse.json({ success: true })
  } catch (error) {
    console.error('Delete evaluation error:', error)
    return NextResponse.json({ error: 'Failed to delete evaluation' }, { status: 500 })
  }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/app/api/evaluations/route.ts src/app/api/evaluations/\[id\]/route.ts
git commit -m "feat: add evaluations API routes"
```

---

## Task 5: API Routes - LLM Config

**Files:**
- Create: `src/app/api/config/llm/route.ts`
- Create: `src/app/api/config/llm/[id]/route.ts`

- [ ] **Step 1: Create src/app/api/config/llm/route.ts**

```typescript
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { encryptApiKey, decryptApiKey } from '@/lib/encryption'

export async function GET() {
  try {
    const configs = await prisma.lLMConfig.findMany({
      orderBy: { createdAt: 'desc' },
    })
    
    // Don't return decrypted API keys
    return NextResponse.json(configs.map(c => ({
      id: c.id,
      name: c.name,
      provider: c.provider,
      model: c.model,
      baseUrl: c.baseUrl,
      isDefault: c.isDefault,
      createdAt: c.createdAt.toISOString(),
    })))
  } catch (error) {
    console.error('List LLM configs error:', error)
    return NextResponse.json({ error: 'Failed to list configs' }, { status: 500 })
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json()
    const { name, provider, model, apiKey, baseUrl, isDefault } = body
    
    if (!name || !provider || !model || !apiKey) {
      return NextResponse.json({ error: 'Missing required fields' }, { status: 400 })
    }
    
    // If this is set as default, unset other defaults
    if (isDefault) {
      await prisma.lLMConfig.updateMany({
        where: { isDefault: true },
        data: { isDefault: false },
      })
    }
    
    const config = await prisma.lLMConfig.create({
      data: {
        name,
        provider,
        model,
        apiKey: encryptApiKey(apiKey),
        baseUrl: baseUrl || null,
        isDefault: isDefault || false,
      },
    })
    
    return NextResponse.json({
      id: config.id,
      name: config.name,
      provider: config.provider,
      model: config.model,
      isDefault: config.isDefault,
      createdAt: config.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Create LLM config error:', error)
    return NextResponse.json({ error: 'Failed to create config' }, { status: 500 })
  }
}
```

- [ ] **Step 2: Create src/app/api/config/llm/[id]/route.ts**

```typescript
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { encryptApiKey } from '@/lib/encryption'

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json()
    const { name, provider, model, apiKey, baseUrl, isDefault } = body
    
    const updateData: Record<string, unknown> = {}
    if (name) updateData.name = name
    if (provider) updateData.provider = provider
    if (model) updateData.model = model
    if (apiKey) updateData.apiKey = encryptApiKey(apiKey)
    if (baseUrl !== undefined) updateData.baseUrl = baseUrl
    
    // Handle default flag
    if (isDefault) {
      await prisma.lLMConfig.updateMany({
        where: { isDefault: true },
        data: { isDefault: false },
      })
      updateData.isDefault = true
    }
    
    const config = await prisma.lLMConfig.update({
      where: { id: params.id },
      data: updateData,
    })
    
    return NextResponse.json({
      id: config.id,
      name: config.name,
      provider: config.provider,
      model: config.model,
      isDefault: config.isDefault,
      updatedAt: config.updatedAt.toISOString(),
    })
  } catch (error) {
    console.error('Update LLM config error:', error)
    return NextResponse.json({ error: 'Failed to update config' }, { status: 500 })
  }
}

export async function DELETE(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    await prisma.lLMConfig.delete({
      where: { id: params.id },
    })
    
    return NextResponse.json({ success: true })
  } catch (error) {
    console.error('Delete LLM config error:', error)
    return NextResponse.json({ error: 'Failed to delete config' }, { status: 500 })
  }
}
```

- [ ] **Step 3: Commit**

```bash
git add src/app/api/config/llm/route.ts src/app/api/config/llm/\[id\]/route.ts
git commit -m "feat: add LLM config API routes"
```

---

## Task 6: Frontend - Layout & Home Page

**Files:**
- Create: `src/app/layout.tsx`
- Create: `src/app/page.tsx`
- Create: `src/components/FileUpload.tsx`
- Create: `src/components/EvaluationList.tsx`

- [ ] **Step 1: Create src/app/layout.tsx**

```typescript
import './globals.css'
import type { Metadata } from 'next'

export const metadata: Metadata = {
  title: 'AI-Ready Evaluator',
  description: 'Assess development requirements for AI Coding implementability',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <div className="min-h-screen bg-gray-50">
          <header className="bg-white shadow-sm">
            <div className="max-w-5xl mx-auto px-4 py-4 flex justify-between items-center">
              <h1 className="text-xl font-semibold text-gray-900">
                AI-Ready Evaluator
              </h1>
              <nav className="flex gap-4">
                <a href="/" className="text-gray-600 hover:text-gray-900">
                  Home
                </a>
                <a href="/settings" className="text-gray-600 hover:text-gray-900">
                  Settings
                </a>
              </nav>
            </div>
          </header>
          <main className="max-w-5xl mx-auto px-4 py-8">
            {children}
          </main>
        </div>
      </body>
    </html>
  )
}
```

- [ ] **Step 2: Create src/app/globals.css** (basic styles)

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
}
```

- [ ] **Step 3: Create src/components/FileUpload.tsx**

```typescript
'use client'

import { useState } from 'react'

interface FileUploadProps {
  onUpload: (file: File, title?: string) => Promise<void>
}

export default function FileUpload({ onUpload }: FileUploadProps) {
  const [file, setFile] = useState<File | null>(null)
  const [title, setTitle] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!file) {
      setError('Please select a file')
      return
    }
    
    setLoading(true)
    setError('')
    
    try {
      await onUpload(file, title)
      setFile(null)
      setTitle('')
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Upload failed')
    } finally {
      setLoading(false)
    }
  }
  
  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-lg font-medium mb-4">Upload Requirement Document</h2>
      
      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            File (TXT or Markdown)
          </label>
          <input
            type="file"
            accept=".txt,.md"
            onChange={(e) => setFile(e.target.files?.[0] || null)}
            className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
          />
        </div>
        
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Title (optional)
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Auto-extracted from filename"
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
        
        {error && (
          <div className="text-red-600 text-sm">{error}</div>
        )}
        
        <button
          type="submit"
          disabled={!file || loading}
          className="w-full px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? 'Evaluating...' : 'Evaluate'}
        </button>
      </form>
    </div>
  )
}
```

- [ ] **Step 4: Create src/components/EvaluationList.tsx**

```typescript
'use client'

interface Evaluation {
  id: string
  title: string
  fileName: string | null
  overallScore: number
  grade: string
  complexity: string
  createdAt: string
}

interface EvaluationListProps {
  evaluations: Evaluation[]
  onDelete: (id: string) => void
}

export default function EvaluationList({ evaluations, onDelete }: EvaluationListProps) {
  const gradeColors: Record<string, string> = {
    S: 'bg-green-100 text-green-800',
    A: 'bg-blue-100 text-blue-800',
    B: 'bg-yellow-100 text-yellow-800',
    C: 'bg-red-100 text-red-800',
  }
  
  if (evaluations.length === 0) {
    return (
      <div className="text-center py-8 text-gray-500">
        No evaluations yet. Upload a requirement document to get started.
      </div>
    )
  }
  
  return (
    <div className="bg-white rounded-lg shadow overflow-hidden">
      <table className="min-w-full divide-y divide-gray-200">
        <thead className="bg-gray-50">
          <tr>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Grade
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Title
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Score
            </th>
            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Date
            </th>
            <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
              Actions
            </th>
          </tr>
        </thead>
        <tbody className="bg-white divide-y divide-gray-200">
          {evaluations.map((eval_) => (
            <tr key={eval_.id}>
              <td className="px-6 py-4 whitespace-nowrap">
                <span className={`px-2 py-1 text-xs font-semibold rounded-full ${gradeColors[eval_.grade]}`}>
                  {eval_.grade}
                </span>
              </td>
              <td className="px-6 py-4">
                <div className="text-sm font-medium text-gray-900">{eval_.title}</div>
                {eval_.fileName && (
                  <div className="text-sm text-gray-500">{eval_.fileName}</div>
                )}
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <div className="text-sm text-gray-900">{eval_.overallScore}/100</div>
              </td>
              <td className="px-6 py-4 whitespace-nowrap">
                <div className="text-sm text-gray-500">
                  {new Date(eval_.createdAt).toLocaleDateString()}
                </div>
              </td>
              <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                <a
                  href={`/evaluations/${eval_.id}`}
                  className="text-blue-600 hover:text-blue-900 mr-4"
                >
                  View
                </a>
                <button
                  onClick={() => onDelete(eval_.id)}
                  className="text-red-600 hover:text-red-900"
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
```

- [ ] **Step 5: Create src/app/page.tsx**

```typescript
'use client'

import { useState, useEffect } from 'react'
import FileUpload from '@/components/FileUpload'
import EvaluationList from '@/components/EvaluationList'
import { useRouter } from 'next/navigation'

interface Evaluation {
  id: string
  title: string
  fileName: string | null
  overallScore: number
  grade: string
  complexity: string
  createdAt: string
}

export default function Home() {
  const [evaluations, setEvaluations] = useState<Evaluation[]>([])
  const [loading, setLoading] = useState(true)
  const router = useRouter()
  
  const fetchEvaluations = async () => {
    try {
      const res = await fetch('/api/evaluations')
      if (res.ok) {
        const data = await res.json()
        setEvaluations(data)
      }
    } catch (error) {
      console.error('Failed to fetch evaluations:', error)
    } finally {
      setLoading(false)
    }
  }
  
  useEffect(() => {
    fetchEvaluations()
  }, [])
  
  const handleUpload = async (file: File, title?: string) => {
    const formData = new FormData()
    formData.append('file', file)
    if (title) formData.append('title', title)
    
    const res = await fetch('/api/evaluations', {
      method: 'POST',
      body: formData,
    })
    
    if (!res.ok) {
      const error = await res.json()
      throw new Error(error.error || 'Evaluation failed')
    }
    
    const result = await res.json()
    router.push(`/evaluations/${result.id}`)
  }
  
  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this evaluation?')) return
    
    const res = await fetch(`/api/evaluations/${id}`, {
      method: 'DELETE',
    })
    
    if (res.ok) {
      setEvaluations(evaluations.filter(e => e.id !== id))
    }
  }
  
  return (
    <div className="space-y-8">
      <FileUpload onUpload={handleUpload} />
      
      <div>
        <h2 className="text-lg font-medium mb-4">Recent Evaluations</h2>
        {loading ? (
          <div className="text-center py-8 text-gray-500">Loading...</div>
        ) : (
          <EvaluationList evaluations={evaluations} onDelete={handleDelete} />
        )}
      </div>
    </div>
  )
}
```

- [ ] **Step 6: Add Tailwind CSS**

Run: `npm install -D tailwindcss postcss autoprefixer && npx tailwindcss init -p`

Create `tailwind.config.js`:
```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

Update `src/app/globals.css`:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

- [ ] **Step 7: Commit**

```bash
git add src/app/layout.tsx src/app/page.tsx src/app/globals.css src/components/FileUpload.tsx src/components/EvaluationList.tsx tailwind.config.js postcss.config.js src/app/globals.css
git commit -m "feat: add home page with file upload and evaluation list"
```

---

## Task 7: Frontend - Evaluation Report Page

**Files:**
- Create: `src/app/evaluations/[id]/page.tsx`
- Create: `src/components/ScoreCard.tsx`

- [ ] **Step 1: Create src/components/ScoreCard.tsx**

```typescript
'use client'

interface ScoreCardProps {
  label: string
  score: number
  maxScore?: number
}

export default function ScoreCard({ label, score, maxScore = 100 }: ScoreCardProps) {
  const percentage = (score / maxScore) * 100
  const color = percentage >= 80 ? 'bg-green-500' : percentage >= 60 ? 'bg-yellow-500' : 'bg-red-500'
  
  return (
    <div className="bg-white rounded-lg shadow p-4">
      <div className="flex justify-between items-center mb-2">
        <span className="text-sm font-medium text-gray-700">{label}</span>
        <span className="text-lg font-semibold text-gray-900">{score}</span>
      </div>
      <div className="w-full bg-gray-200 rounded-full h-2">
        <div
          className={`h-2 rounded-full ${color}`}
          style={{ width: `${percentage}%` }}
        />
      </div>
    </div>
  )
}
```

- [ ] **Step 2: Create src/app/evaluations/[id]/page.tsx**

```typescript
'use client'

import { useState, useEffect } from 'next/navigation'
import ScoreCard from '@/components/ScoreCard'
import { useRouter } from 'next/navigation'

interface EvaluationDetail {
  id: string
  title: string
  content: string
  fileName: string | null
  overallScore: number
  grade: string
  complexity: string
  contextScore: number
  atomicityScore: number
  boundaryScore: number
  verifiabilityScore: number
  techScore: number
  suggestions: Array<{ type: string; content: string }>
  modelUsed: string
  createdAt: string
}

export default function EvaluationPage({ params }: { params: { id: string } }) {
  const [evaluation, setEvaluation] = useState<EvaluationDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const router = useRouter()
  
  useEffect(() => {
    fetch(`/api/evaluations/${params.id}`)
      .then(res => res.json())
      .then(data => setEvaluation(data))
      .catch(console.error)
      .finally(() => setLoading(false))
  }, [params.id])
  
  if (loading) {
    return <div className="text-center py-8">Loading...</div>
  }
  
  if (!evaluation) {
    return <div className="text-center py-8">Evaluation not found</div>
  }
  
  const gradeColors: Record<string, string> = {
    S: 'bg-green-100 text-green-800',
    A: 'bg-blue-100 text-blue-800',
    B: 'bg-yellow-100 text-yellow-800',
    C: 'bg-red-100 text-red-800',
  }
  
  return (
    <div className="space-y-6">
      <button
        onClick={() => router.back()}
        className="text-gray-600 hover:text-gray-900 flex items-center gap-2"
      >
        ← Back
      </button>
      
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex items-center justify-between mb-4">
          <h1 className="text-2xl font-bold text-gray-900">{evaluation.title}</h1>
          <span className={`px-3 py-1 text-lg font-semibold rounded-full ${gradeColors[evaluation.grade]}`}>
            {evaluation.grade} ({evaluation.overallScore})
          </span>
        </div>
        
        <div className="w-full bg-gray-200 rounded-full h-4 mb-6">
          <div
            className="h-4 rounded-full bg-blue-600"
            style={{ width: `${evaluation.overallScore}%` }}
          />
        </div>
        
        <div className="text-sm text-gray-500 mb-6">
          Assessed with {evaluation.modelUsed} • {new Date(evaluation.createdAt).toLocaleString()}
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
        <ScoreCard label="Context Score" score={evaluation.contextScore} />
        <ScoreCard label="Atomicity Score" score={evaluation.atomicityScore} />
        <ScoreCard label="Boundary Score" score={evaluation.boundaryScore} />
        <ScoreCard label="Verifiability" score={evaluation.verifiabilityScore} />
        <ScoreCard label="Tech Score" score={evaluation.techScore} />
      </div>
      
      {evaluation.suggestions.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-lg font-medium mb-4">Suggestions</h2>
          <ul className="space-y-2">
            {evaluation.suggestions.map((suggestion, index) => (
              <li key={index} className="flex items-start gap-2">
                <span className="px-2 py-0.5 text-xs font-medium bg-gray-100 rounded">
                  {suggestion.type}
                </span>
                <span className="text-gray-700">{suggestion.content}</span>
              </li>
            ))}
          </ul>
        </div>
      )}
      
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-medium mb-4">Original Content</h2>
        <pre className="whitespace-pre-wrap text-sm text-gray-700 bg-gray-50 p-4 rounded-lg overflow-auto">
          {evaluation.content}
        </pre>
      </div>
    </div>
  )
}
```

- [ ] **Step 3: Commit**

```bash
git add src/app/evaluations/\[id\]/page.tsx src/components/ScoreCard.tsx
git commit -m "feat: add evaluation report page"
```

---

## Task 8: Frontend - Settings Page

**Files:**
- Create: `src/app/settings/page.tsx`
- Create: `src/components/LLMConfigForm.tsx`

- [ ] **Step 1: Create src/components/LLMConfigForm.tsx**

```typescript
'use client'

import { useState } from 'react'

interface LLMConfig {
  id: string
  name: string
  provider: string
  model: string
  baseUrl?: string | null
  isDefault: boolean
}

interface LLMConfigFormProps {
  configs: LLMConfig[]
  onRefresh: () => void
}

export default function LLMConfigForm({ configs, onRefresh }: LLMConfigFormProps) {
  const [showForm, setShowForm] = useState(false)
  const [formData, setFormData] = useState({
    name: '',
    provider: 'openai',
    model: 'gpt-4o',
    apiKey: '',
    baseUrl: '',
    isDefault: false,
  })
  const [saving, setSaving] = useState(false)
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSaving(true)
    
    try {
      await fetch('/api/config/llm', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      })
      setShowForm(false)
      setFormData({
        name: '',
        provider: 'openai',
        model: 'gpt-4o',
        apiKey: '',
        baseUrl: '',
        isDefault: false,
      })
      onRefresh()
    } catch (error) {
      console.error('Failed to save config:', error)
    } finally {
      setSaving(false)
    }
  }
  
  const handleDelete = async (id: string) => {
    if (!confirm('Delete this configuration?')) return
    
    await fetch(`/api/config/llm/${id}`, { method: 'DELETE' })
    onRefresh()
  }
  
  const providerModels: Record<string, string[]> = {
    openai: ['gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo', 'gpt-4'],
    anthropic: ['claude-3-5-sonnet-20241022', 'claude-3-opus-20240229', 'claude-3-sonnet-20240229'],
    google: ['gemini-1.5-pro', 'gemini-1.5-flash', 'gemini-1.5-flash-8b'],
    ollama: ['llama3', 'mistral', 'codellama', 'phi3'],
  }
  
  return (
    <div className="space-y-6">
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-lg font-medium">LLM Configurations</h2>
          <button
            onClick={() => setShowForm(!showForm)}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
          >
            {showForm ? 'Cancel' : '+ Add Model'}
          </button>
        </div>
        
        {showForm && (
          <form onSubmit={handleSubmit} className="space-y-4 mb-6 border-b pb-6">
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Name
                </label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="My GPT-4o"
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                  required
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Provider
                </label>
                <select
                  value={formData.provider}
                  onChange={(e) => setFormData({ ...formData, provider: e.target.value, model: providerModels[e.target.value as keyof typeof providerModels][0] })}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                >
                  <option value="openai">OpenAI</option>
                  <option value="anthropic">Anthropic</option>
                  <option value="google">Google Gemini</option>
                  <option value="ollama">Ollama (Local)</option>
                </select>
              </div>
            </div>
            
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Model
                </label>
                <select
                  value={formData.model}
                  onChange={(e) => setFormData({ ...formData, model: e.target.value })}
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                >
                  {providerModels[formData.provider as keyof typeof providerModels]?.map(m => (
                    <option key={m} value={m}>{m}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Base URL (optional)
                </label>
                <input
                  type="text"
                  value={formData.baseUrl}
                  onChange={(e) => setFormData({ ...formData, baseUrl: e.target.value })}
                  placeholder="For proxy/custom endpoints"
                  className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                />
              </div>
            </div>
            
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                API Key
              </label>
              <input
                type="password"
                value={formData.apiKey}
                onChange={(e) => setFormData({ ...formData, apiKey: e.target.value })}
                placeholder="sk-..."
                className="block w-full px-3 py-2 border border-gray-300 rounded-md"
                required
              />
            </div>
            
            <div className="flex items-center gap-2">
              <input
                type="checkbox"
                id="isDefault"
                checked={formData.isDefault}
                onChange={(e) => setFormData({ ...formData, isDefault: e.target.checked })}
                className="rounded border-gray-300"
              />
              <label htmlFor="isDefault" className="text-sm text-gray-700">
                Set as default model
              </label>
            </div>
            
            <button
              type="submit"
              disabled={saving}
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50"
            >
              {saving ? 'Saving...' : 'Save Configuration'}
            </button>
          </form>
        )}
        
        {configs.length === 0 ? (
          <div className="text-center py-8 text-gray-500">
            No LLM configurations. Add one to get started.
          </div>
        ) : (
          <div className="space-y-2">
            {configs.map((config) => (
              <div
                key={config.id}
                className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
              >
                <div className="flex items-center gap-3">
                  <span className="text-2xl">
                    {config.provider === 'openai' && '🔵'}
                    {config.provider === 'anthropic' && '🟣'}
                    {config.provider === 'google' && '🟠'}
                    {config.provider === 'ollama' && '🟢'}
                  </span>
                  <div>
                    <div className="font-medium">
                      {config.name}
                      {config.isDefault && (
                        <span className="ml-2 px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded">
                          Default
                        </span>
                      )}
                    </div>
                    <div className="text-sm text-gray-500">
                      {config.provider} / {config.model}
                    </div>
                  </div>
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => handleDelete(config.id)}
                    className="text-red-600 hover:text-red-900 text-sm"
                  >
                    Delete
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  )
}
```

- [ ] **Step 2: Create src/app/settings/page.tsx**

```typescript
'use client'

import { useState, useEffect } from 'next/navigation'
import LLMConfigForm from '@/components/LLMConfigForm'
import { useRouter } from 'next/navigation'

interface LLMConfig {
  id: string
  name: string
  provider: string
  model: string
  baseUrl?: string | null
  isDefault: boolean
}

export default function SettingsPage() {
  const [configs, setConfigs] = useState<LLMConfig[]>([])
  const [loading, setLoading] = useState(true)
  const router = useRouter()
  
  const fetchConfigs = async () => {
    try {
      const res = await fetch('/api/config/llm')
      if (res.ok) {
        const data = await res.json()
        setConfigs(data)
      }
    } catch (error) {
      console.error('Failed to fetch configs:', error)
    } finally {
      setLoading(false)
    }
  }
  
  useEffect(() => {
    fetchConfigs()
  }, [])
  
  return (
    <div>
      <button
        onClick={() => router.back()}
        className="text-gray-600 hover:text-gray-900 flex items-center gap-2 mb-6"
      >
        ← Back
      </button>
      
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      
      {loading ? (
        <div className="text-center py-8">Loading...</div>
      ) : (
        <LLMConfigForm configs={configs} onRefresh={fetchConfigs} />
      )}
    </div>
  )
}
```

- [ ] **Step 3: Commit**

```bash
git add src/app/settings/page.tsx src/components/LLMConfigForm.tsx
git commit -m "feat: add settings page with LLM configuration"
```

---

## Task 9: Build & Verify

**Files:**
- Modify: Verify entire project

- [ ] **Step 1: Build the project**

Run: `npm run build`
Expected: Build completes without errors

- [ ] **Step 2: Check for type errors**

Run: `npx tsc --noEmit`
Expected: No type errors

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat: complete AI-Ready Evaluator implementation"
```

---

## Plan Complete

Plan saved to `docs/superpowers/plans/2026-04-02-ai-ready-evaluator.md`.

**Two execution options:**

1. **Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

2. **Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
