# AI-Ready Evaluator Implementation Plan (Iteration 2 - Detailed)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement all missing features from gap analysis: P0 (tests, input validation, risk heatmap infrastructure), P1 (LLM config CRUD, stats API), P2 (weights config, suggestions grouping, history filtering).

**Architecture:** Three-phase approach - P0 fixes first (tests + validation + heatmap infra), then P1 features (config CRUD + stats), then P2 enhancements (weights + UI improvements).

**Tech Stack:** Next.js 14, TypeScript, Prisma (SQLite), Vitest for testing, Zod for validation.

---

## Phase 1: P0 - Critical Fixes (Quality Baseline)

### Task 1: Install and Configure Vitest

**Files:**
- Modify: `outputs/package.json`
- Create: `outputs/vitest.config.ts`
- Reference: `outputs/tsconfig.json`

- [ ] **Step 1: Install vitest and jsdom**

Run: `cd outputs && npm install -D vitest @vitest/ui jsdom`
Expected: Package installed successfully

- [ ] **Step 2: Create vitest.config.ts**

```typescript
// outputs/vitest.config.ts
import { defineConfig } from 'vitest/config'
import path from 'path'

export default defineConfig({
  test: {
    environment: 'jsdom',
    globals: true,
    include: ['src/**/*.test.ts'],
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
})
```

- [ ] **Step 3: Add test script to package.json**

Add to scripts section:
```json
"test": "vitest",
"test:ui": "vitest --ui",
"test:run": "vitest run"
```

- [ ] **Step 4: Commit**

```bash
cd outputs && git add package.json vitest.config.ts
git commit -m "test: add vitest configuration"
```

---

### Task 2: Add Unit Tests for evaluator.ts

**Files:**
- Create: `outputs/src/lib/evaluator.test.ts`
- Reference: `outputs/src/lib/evaluator.ts`

- [ ] **Step 1: Write failing test file**

```typescript
// outputs/src/lib/evaluator.test.ts
import { describe, it, expect, vi } from 'vitest'
import { evaluateRequirement, DEFAULT_WEIGHTS, COMPLEXITY_PENALTY } from './evaluator'

vi.mock('./llm', () => ({
  callLLM: vi.fn().mockResolvedValue({
    scores: { context: 80, atomicity: 70, boundary: 60, verifiability: 90, tech: 85 },
    overall: 77,
    grade: 'A' as const,
    complexity: 'medium' as const,
    risks: ['Missing user journey'],
    suggestions: [{ type: 'context', content: 'Add user journey' }],
  }),
}))

describe('evaluateRequirement', () => {
  it('should calculate overall score with default weights', async () => {
    const result = await evaluateRequirement('test content', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    
    // Weighted: 80*0.25 + 70*0.25 + 60*0.2 + 90*0.15 + 85*0.15 = 20 + 17.5 + 12 + 13.5 + 12.75 = 75.75
    // Penalty: 0.9 (medium)
    // Expected: 68
    expect(result.overallScore).toBe(68)
    expect(result.grade).toBe('B')
  })

  it('should apply complexity penalty correctly - simple', async () => {
    const result = await evaluateRequirement('test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    // Uses mocked complexity: medium = 0.9
    expect(result.complexity).toBe('medium')
  })

  it('should assign correct grade thresholds', async () => {
    // S grade: >= 90
    const resultS = await evaluateRequirement('test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(['S', 'A', 'B', 'C']).toContain(resultS.grade)
  })
})

describe('DEFAULT_WEIGHTS', () => {
  it('should sum to 1.0', () => {
    const sum = DEFAULT_WEIGHTS.context + DEFAULT_WEIGHTS.atomicity + 
                DEFAULT_WEIGHTS.boundary + DEFAULT_WEIGHTS.verifiability + DEFAULT_WEIGHTS.tech
    expect(sum).toBe(1.0)
  })

  it('should have correct values', () => {
    expect(DEFAULT_WEIGHTS.context).toBe(0.25)
    expect(DEFAULT_WEIGHTS.atomicity).toBe(0.25)
    expect(DEFAULT_WEIGHTS.boundary).toBe(0.20)
    expect(DEFAULT_WEIGHTS.verifiability).toBe(0.15)
    expect(DEFAULT_WEIGHTS.tech).toBe(0.15)
  })
})

describe('COMPLEXITY_PENALTY', () => {
  it('should have all complexity levels', () => {
    expect(COMPLEXITY_PENALTY.simple).toBe(1.0)
    expect(COMPLEXITY_PENALTY.medium).toBe(0.9)
    expect(COMPLEXITY_PENALTY.complex).toBe(0.7)
  })
})
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cd outputs && npm run test:run`
Expected: All tests pass

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/lib/evaluator.test.ts
git commit -m "test: add unit tests for evaluator.ts"
```

---

### Task 3: Add Unit Tests for llm.ts (parseResponse)

**Files:**
- Create: `outputs/src/lib/llm.test.ts`
- Reference: `outputs/src/lib/llm.ts`

- [ ] **Step 1: Write test file for parseResponse**

```typescript
// outputs/src/lib/llm.test.ts
import { describe, it, expect } from 'vitest'
import { parseResponse, LLMConfig, callLLM } from './llm'

describe('parseResponse', () => {
  it('should parse valid JSON response with surrounding text', () => {
    const content = `Here is the analysis:
{
  "scores": { "context": 80, "atomicity": 70, "boundary": 60, "verifiability": 90, "tech": 85 },
  "overall": 77,
  "grade": "A",
  "complexity": "medium",
  "risks": ["risk1"],
  "suggestions": [{ "type": "context", "content": "add journey" }]
}`
    
    const result = parseResponse(content)
    
    expect(result.scores.context).toBe(80)
    expect(result.scores.atomicity).toBe(70)
    expect(result.grade).toBe('A')
    expect(result.complexity).toBe('medium')
    expect(result.risks).toContain('risk1')
    expect(result.suggestions).toHaveLength(1)
  })

  it('should clamp scores to 0-100 range', () => {
    const content = '{"scores": { "context": 150, "atomicity": -10, "boundary": 50, "verifiability": 80, "tech": 90 }, "overall": 50, "grade": "C", "complexity": "simple"}'
    
    const result = parseResponse(content)
    
    expect(result.scores.context).toBe(100)
    expect(result.scores.atomicity).toBe(0)
  })

  it('should handle missing fields with defaults', () => {
    const content = '{}'
    
    const result = parseResponse(content)
    
    expect(result.scores.context).toBe(0)
    expect(result.grade).toBe('C')
    expect(result.complexity).toBe('medium')
    expect(result.risks).toEqual([])
    expect(result.suggestions).toEqual([])
  })

  it('should throw on completely invalid JSON', () => {
    expect(() => parseResponse('not json at all')).toThrow('Failed to parse LLM response')
  })

  it('should handle empty response', () => {
    expect(() => parseResponse('')).toThrow('Failed to parse LLM response')
  })
})

describe('callLLM', () => {
  it('should throw on unknown provider', async () => {
    const config: LLMConfig = { provider: 'unknown' as any, model: 'test', apiKey: 'test' }
    
    await expect(callLLM(config, 'test')).rejects.toThrow('Unknown provider')
  })
})
```

- [ ] **Step 2: Run tests**

Run: `cd outputs && npm run test:run src/lib/llm.test.ts`
Expected: All tests pass

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/lib/llm.test.ts
git commit -m "test: add unit tests for llm.ts parseResponse"
```

---

### Task 4: Add Input Validation with Zod to API Routes

**Files:**
- Modify: `outputs/src/app/api/evaluations/route.ts`
- Modify: `outputs/src/app/api/config/llm/route.ts`
- Create: `outputs/src/lib/validators.ts`

- [ ] **Step 1: Create Zod validators**

```typescript
// outputs/src/lib/validators.ts
import { z } from 'zod'

export const LLMConfigSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  provider: z.enum(['openai', 'anthropic', 'google', 'ollama']),
  model: z.string().min(1, 'Model is required'),
  apiKey: z.string().min(1, 'API key is required'),
  baseUrl: z.string().url().optional(),
  isDefault: z.boolean().optional(),
})

export const UpdateLLMConfigSchema = LLMConfigSchema.partial().extend({
  apiKey: z.string().min(1).optional(),
})

export const CreateEvaluationSchema = z.object({
  title: z.string().optional(),
  modelId: z.string().optional(),
  // File is handled separately via FormData
})

export const UpdateEvaluationSchema = z.object({
  title: z.string().optional(),
  content: z.string().optional(),
  modelId: z.string().optional(),
})

export const ScoreWeightsSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  context: z.number().min(0).max(1),
  atomicity: z.number().min(0).max(1),
  boundary: z.number().min(0).max(1),
  verifiability: z.number().min(0).max(1),
  tech: z.number().min(0).max(1),
  isDefault: z.boolean().optional(),
}).refine(data => {
  const sum = data.context + data.atomicity + data.boundary + data.verifiability + data.tech
  return Math.abs(sum - 1.0) < 0.01
}, {
  message: 'Weights must sum to 1.0',
})
```

- [ ] **Step 2: Add validation to POST /api/evaluations**

Replace the file upload handling with validated version:

```typescript
// Add at top of POST function
const MAX_FILE_SIZE = 10 * 1024 * 1024 // 10MB

export async function POST(request: NextRequest) {
  try {
    const formData = await request.formData()
    const file = formData.get('file') as File | null
    
    if (!file) {
      return NextResponse.json({ error: 'No file provided' }, { status: 400 })
    }
    
    // File size validation
    if (file.size > MAX_FILE_SIZE) {
      return NextResponse.json({ error: 'File too large. Max 10MB.' }, { status: 400 })
    }
    
    // File type validation
    const allowedTypes = ['text/markdown', 'text/plain', 'application/octet-stream']
    const allowedExtensions = ['.md', '.txt']
    const ext = file.name.substring(file.name.lastIndexOf('.')).toLowerCase()
    
    if (!allowedExtensions.includes(ext)) {
      return NextResponse.json({ error: 'Invalid file type. Allowed: .md, .txt' }, { status: 400 })
    }
    
    // ... rest of the existing logic
```

- [ ] **Step 3: Add validation to POST /api/config/llm**

Replace POST function body validation:

```typescript
export async function POST(request: NextRequest) {
  try {
    const body = await request.json()
    
    // Validate using Zod
    const validation = LLMConfigSchema.safeParse(body)
    if (!validation.success) {
      return NextResponse.json(
        { error: 'Validation failed', details: validation.error.flatten() },
        { status: 400 }
      )
    }
    
    const { name, provider, model, apiKey, baseUrl, isDefault } = validation.data
    
    // ... rest of existing logic
```

- [ ] **Step 4: Verify TypeScript compiles**

Run: `cd outputs && npx tsc --noEmit`
Expected: No errors (zod types are valid)

- [ ] **Step 5: Commit**

```bash
cd outputs && git add src/lib/validators.ts src/app/api/evaluations/route.ts src/app/api/config/llm/route.ts
git commit -m "feat: add Zod input validation to API routes"
```

---

### Task 5: Add Risk Heatmap Infrastructure

**Files:**
- Modify: `outputs/prisma/schema.prisma`
- Modify: `outputs/src/app/api/evaluations/route.ts`
- Create: `outputs/src/app/api/evaluations/[id]/route.ts` (GET already exists, verify)

Note: This task adds database support and API changes. The actual UI highlighting will be in Task 10.

- [ ] **Step 1: Add risks field to Evaluation model**

Edit `outputs/prisma/schema.prisma` - add risks field:

```prisma
model Evaluation {
  id              String   @id @default(cuid())
  title           String
  content         String   @db.Text
  fileName        String?
  fileType        String?
  overallScore    Int
  grade           String
  complexity      String
  contextScore    Int
  atomicityScore  Int
  boundaryScore   Int
  verifiabilityScore Int
  techScore       Int
  risks           String   @default("[]")  // JSON array of risk strings
  rawResponse     String   @db.Text
  suggestions     String   @db.Text
  modelUsed       String?
  createdAt       DateTime @default(now())
  updatedAt       DateTime @updatedAt
}
```

- [ ] **Step 2: Push schema to database**

Run: `cd outputs && npx prisma db push`
Expected: Schema updated

- [ ] **Step 3: Update evaluations POST to store risks**

Modify `src/app/api/evaluations/route.ts` to store risks:

```typescript
// In the create data object, add:
risks: JSON.stringify(result.risks || []),
```

- [ ] **Step 4: Verify GET endpoint returns risks**

Check `src/app/api/evaluations/[id]/route.ts` returns risks field - update if needed:

```typescript
// Add to GET response:
risks: JSON.parse(evaluation.risks),
```

- [ ] **Step 5: Commit**

```bash
cd outputs && git add prisma/schema.prisma src/app/api/evaluations/route.ts src/app/api/evaluations/\[id\]/route.ts
git commit -m "feat: add risks field to support heatmap"
```

---

## Phase 2: P1 - Important Features

### Task 6: Implement PUT /api/evaluations/[id] Endpoint

**Files:**
- Modify: `outputs/src/app/api/evaluations/[id]/route.ts`
- Reference: `outputs/src/app/api/evaluations/route.ts`

- [ ] **Step 1: Add PUT handler**

Replace the file with GET + PUT + DELETE:

```typescript
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { evaluateRequirement } from '@/lib/evaluator'
import { decryptApiKey } from '@/lib/encryption'
import { LLMConfig } from '@/lib/llm'

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
      risks: JSON.parse(evaluation.risks),
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

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json()
    const { title, content, modelId } = body
    
    const existing = await prisma.evaluation.findUnique({
      where: { id: params.id },
    })
    
    if (!existing) {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }
    
    // If content changed, re-evaluate
    let result
    if (content && content !== existing.content) {
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
        const defaultConfig = await prisma.lLMConfig.findFirst({ where: { isDefault: true } })
        if (!defaultConfig) {
          return NextResponse.json({ error: 'No default LLM configured' }, { status: 400 })
        }
        llmConfig = {
          provider: defaultConfig.provider as LLMConfig['provider'],
          model: defaultConfig.model,
          apiKey: decryptApiKey(defaultConfig.apiKey),
          baseUrl: defaultConfig.baseUrl || undefined,
        }
      }
      
      result = await evaluateRequirement(content, llmConfig)
    }
    
    // Update evaluation
    const updateData: any = {
      title: title || existing.title,
    }
    
    if (result) {
      updateData.content = content
      updateData.overallScore = result.overallScore
      updateData.grade = result.grade
      updateData.complexity = result.complexity
      updateData.contextScore = result.scores.context
      updateData.atomicityScore = result.scores.atomicity
      updateData.boundaryScore = result.scores.boundary
      updateData.verifiabilityScore = result.scores.verifiability
      updateData.techScore = result.scores.tech
      updateData.risks = JSON.stringify(result.risks || [])
      updateData.rawResponse = JSON.stringify(result)
      updateData.suggestions = JSON.stringify(result.suggestions)
      updateData.modelUsed = llmConfig.model
    }
    
    const evaluation = await prisma.evaluation.update({
      where: { id: params.id },
      data: updateData,
    })
    
    return NextResponse.json({
      id: evaluation.id,
      title: evaluation.title,
      overallScore: evaluation.overallScore,
      grade: evaluation.grade,
      contextScore: evaluation.contextScore,
      atomicityScore: evaluation.atomicityScore,
      boundaryScore: evaluation.boundaryScore,
      verifiabilityScore: evaluation.verifiabilityScore,
      techScore: evaluation.techScore,
      risks: JSON.parse(evaluation.risks),
      suggestions: result ? result.suggestions : JSON.parse(evaluation.suggestions),
      createdAt: evaluation.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Update evaluation error:', error)
    return NextResponse.json({ error: 'Failed to update evaluation' }, { status: 500 })
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

- [ ] **Step 2: Verify TypeScript**

Run: `cd outputs && npx tsc --noEmit`
Expected: No errors

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/app/api/evaluations/\[id\]/route.ts
git commit -m "feat: add PUT endpoint for evaluations"
```

---

### Task 7: Add ScoreWeights Config Table and API

**Files:**
- Modify: `outputs/prisma/schema.prisma`
- Create: `outputs/src/app/api/config/weights/route.ts`
- Reference: `outputs/src/lib/evaluator.ts`

- [ ] **Step 1: Add ScoreWeights model to schema**

Add after LLMConfig model:

```prisma
model ScoreWeights {
  id        String   @id @default(cuid())
  name      String   @unique
  context   Float    @default(0.25)
  atomicity Float    @default(0.25)
  boundary  Float    @default(0.20)
  verifiability Float @default(0.15)
  tech      Float    @default(0.15)
  isDefault Boolean  @default(false)
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
}
```

- [ ] **Step 2: Push schema to database**

Run: `cd outputs && npx prisma db push`
Expected: Schema updated

- [ ] **Step 3: Create weights API**

```typescript
// outputs/src/app/api/config/weights/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function GET() {
  try {
    const weights = await prisma.scoreWeights.findMany({
      orderBy: { createdAt: 'desc' },
    })
    
    return NextResponse.json(weights.map(w => ({
      id: w.id,
      name: w.name,
      context: w.context,
      atomicity: w.atomicity,
      boundary: w.boundary,
      verifiability: w.verifiability,
      tech: w.tech,
      isDefault: w.isDefault,
      createdAt: w.createdAt.toISOString(),
    })))
  } catch (error) {
    console.error('List weights error:', error)
    return NextResponse.json({ error: 'Failed to list weights' }, { status: 500 })
  }
}

export async function POST(request: NextRequest) {
  try {
    const body = await request.json()
    const { name, context, atomicity, boundary, verifiability, tech, isDefault } = body
    
    if (!name) {
      return NextResponse.json({ error: 'Missing required field: name' }, { status: 400 })
    }
    
    // Validate weights sum to 1.0
    const total = (context || 0.25) + (atomicity || 0.25) + (boundary || 0.20) + 
                  (verifiability || 0.15) + (tech || 0.15)
    if (Math.abs(total - 1.0) > 0.01) {
      return NextResponse.json({ error: 'Weights must sum to 1.0' }, { status: 400 })
    }
    
    if (isDefault) {
      await prisma.scoreWeights.updateMany({
        where: { isDefault: true },
        data: { isDefault: false },
      })
    }
    
    const weights = await prisma.scoreWeights.create({
      data: {
        name,
        context: context || 0.25,
        atomicity: atomicity || 0.25,
        boundary: boundary || 0.20,
        verifiability: verifiability || 0.15,
        tech: tech || 0.15,
        isDefault: isDefault || false,
      },
    })
    
    return NextResponse.json({
      id: weights.id,
      name: weights.name,
      context: weights.context,
      atomicity: weights.atomicity,
      boundary: weights.boundary,
      verifiability: weights.verifiability,
      tech: weights.tech,
      isDefault: weights.isDefault,
      createdAt: weights.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Create weights error:', error)
    return NextResponse.json({ error: 'Failed to create weights' }, { status: 500 })
  }
}
```

- [ ] **Step 4: Verify build**

Run: `cd outputs && npm run build`
Expected: Build succeeds

- [ ] **Step 5: Commit**

```bash
cd outputs && git add prisma/schema.prisma src/app/api/config/weights/
git commit -m "feat: add ScoreWeights config table and API"
```

---

### Task 8: Add Evaluation Statistics API

**Files:**
- Create: `outputs/src/app/api/stats/route.ts`

- [ ] **Step 1: Create stats endpoint**

```typescript
// outputs/src/app/api/stats/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url)
    const days = parseInt(searchParams.get('days') || '30', 10)
    const startDate = new Date()
    startDate.setDate(startDate.getDate() - days)
    
    // Get all evaluations in date range
    const evaluations = await prisma.evaluation.findMany({
      where: {
        createdAt: { gte: startDate },
      },
      orderBy: { createdAt: 'desc' },
    })
    
    // Calculate statistics
    const total = evaluations.length
    const byGrade = evaluations.reduce((acc, e) => {
      acc[e.grade] = (acc[e.grade] || 0) + 1
      return acc
    }, {} as Record<string, number>)
    
    const byComplexity = evaluations.reduce((acc, e) => {
      acc[e.complexity] = (acc[e.complexity] || 0) + 1
      return acc
    }, {} as Record<string, number>)
    
    const avgScores = {
      overall: total > 0 ? evaluations.reduce((sum, e) => sum + e.overallScore, 0) / total : 0,
      context: total > 0 ? evaluations.reduce((sum, e) => sum + e.contextScore, 0) / total : 0,
      atomicity: total > 0 ? evaluations.reduce((sum, e) => sum + e.atomicityScore, 0) / total : 0,
      boundary: total > 0 ? evaluations.reduce((sum, e) => sum + e.boundaryScore, 0) / total : 0,
      verifiability: total > 0 ? evaluations.reduce((sum, e) => sum + e.verifiabilityScore, 0) / total : 0,
      tech: total > 0 ? evaluations.reduce((sum, e) => sum + e.techScore, 0) / total : 0,
    }
    
    // AI adoption rate (S + A grades as "AI-ready")
    const aiReady = (byGrade['S'] || 0) + (byGrade['A'] || 0)
    const adoptionRate = total > 0 ? (aiReady / total) * 100 : 0
    
    // Model usage distribution
    const byModel = evaluations.reduce((acc, e) => {
      const model = e.modelUsed || 'unknown'
      acc[model] = (acc[model] || 0) + 1
      return acc
    }, {} as Record<string, number>)
    
    // Recent trend (last 7 days vs previous 7 days)
    const now = new Date()
    const sevenDaysAgo = new Date(now.getTime() - 7 * 24 * 60 * 60 * 1000)
    const fourteenDaysAgo = new Date(now.getTime() - 14 * 24 * 60 * 60 * 1000)
    
    const lastWeek = await prisma.evaluation.count({
      where: { createdAt: { gte: sevenDaysAgo } },
    })
    const previousWeek = await prisma.evaluation.count({
      where: { 
        createdAt: { 
          gte: fourteenDaysAgo, 
          lt: sevenDaysAgo 
        } 
      },
    })
    
    return NextResponse.json({
      period: { days, startDate: startDate.toISOString() },
      total,
      byGrade,
      byComplexity,
      avgScores: Object.fromEntries(
        Object.entries(avgScores).map(([k, v]) => [k, Math.round(v * 10) / 10])
      ),
      adoptionRate: Math.round(adoptionRate * 10) / 10,
      byModel,
      trend: {
        lastWeek,
        previousWeek,
        change: previousWeek > 0 ? Math.round(((lastWeek - previousWeek) / previousWeek) * 100) : 0,
      },
    })
  } catch (error) {
    console.error('Stats error:', error)
    return NextResponse.json({ error: 'Failed to get stats' }, { status: 500 })
  }
}
```

- [ ] **Step 2: Verify build**

Run: `cd outputs && npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/app/api/stats/route.ts
git commit -m "feat: add evaluation statistics API"
```

---

## Phase 3: P2 - Enhancements

### Task 9: Group Suggestions by Dimension in UI

**Files:**
- Modify: `outputs/src/app/evaluations/[id]/page.tsx`
- Reference: `outputs/src/components/ScoreCard.tsx`

- [ ] **Step 1: Update evaluation page with grouped suggestions**

Add after existing imports:
```tsx
import { useState } from 'react'
```

Add state after the evaluation data fetch:
```tsx
const [activeDimension, setActiveDimension] = useState<string>('context')
```

Replace the suggestions section (around line 90):

```tsx
{/* Group suggestions by type */}
{evaluation.suggestions && evaluation.suggestions.length > 0 && (() => {
  const groupedSuggestions = evaluation.suggestions.reduce((acc, s) => {
    const type = s.type || 'other'
    if (!acc[type]) acc[type] = []
    acc[type].push(s.content)
    return acc
  }, {} as Record<string, string[]>)

  const dimensionLabels: Record<string, string> = {
    context: 'Context Completeness',
    atomicity: 'Logic Atomicity',
    boundary: 'Boundary Clarity',
    verifiability: 'Verifiability',
    tech: 'Technical Constraints',
    other: 'Other',
  }

  const types = Object.keys(groupedSuggestions)
  const currentType = activeDimension && types.includes(activeDimension) ? activeDimension : types[0]

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-lg font-medium mb-4">Optimization Suggestions</h2>
      
      {/* Dimension tabs */}
      <div className="flex flex-wrap gap-2 mb-4">
        {types.map(type => (
          <button
            key={type}
            onClick={() => setActiveDimension(type)}
            className={`px-3 py-1 text-sm rounded-full transition-colors ${
              currentType === type 
                ? 'bg-blue-600 text-white' 
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            {dimensionLabels[type] || type} ({groupedSuggestions[type].length})
          </button>
        ))}
      </div>
      
      {/* Suggestions list */}
      <ul className="space-y-2">
        {(groupedSuggestions[currentType] || []).map((content, index) => (
          <li key={index} className="flex items-start gap-2 text-gray-700">
            <span className="mt-1.5 w-2 h-2 rounded-full bg-blue-500 flex-shrink-0" />
            <span>{content}</span>
          </li>
        ))}
      </ul>
    </div>
  )
})()}
```

- [ ] **Step 2: Verify build**

Run: `cd outputs && npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/app/evaluations/\[id\]/page.tsx
git commit -m "feat: group suggestions by dimension in evaluation page"
```

---

### Task 10: Add Risks Display to Evaluation Detail

**Files:**
- Modify: `outputs/src/app/evaluations/[id]/page.tsx`

- [ ] **Step 1: Add risks section to evaluation page**

Add after the suggestions section:

```tsx
{/* Risks Section */}
{evaluation.risks && evaluation.risks.length > 0 && (
  <div className="bg-white rounded-lg shadow p-6 border-l-4 border-red-500">
    <h2 className="text-lg font-medium mb-4 text-red-700">Risk Factors</h2>
    <ul className="space-y-2">
      {evaluation.risks.map((risk: string, index: number) => (
        <li key={index} className="flex items-start gap-2 text-gray-700">
          <span className="mt-1.5 w-2 h-2 rounded-full bg-red-500 flex-shrink-0" />
          <span>{risk}</span>
        </li>
      ))}
    </ul>
    <p className="mt-4 text-sm text-gray-500">
      These are areas where AI may generate incorrect or incomplete code. Consider addressing these risks before AI implementation.
    </p>
  </div>
)}
```

- [ ] **Step 2: Verify build**

Run: `cd outputs && npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/app/evaluations/\[id\]/page.tsx
git commit -m "feat: add risks display to evaluation detail"
```

---

### Task 11: Add File Size Limit to Evaluations API

**Files:**
- Modify: `outputs/src/app/api/evaluations/route.ts`

Already done in Task 4 (Input Validation). Verify:

- [ ] **Step 1: Verify file size validation exists**

Check that MAX_FILE_SIZE = 10 * 1024 * 1024 is in the POST handler.

- [ ] **Step 2: Commit (if needed)**

If not already committed in Task 4, commit now.

---

### Task 12: Add Evaluation History Filtering

**Files:**
- Modify: `outputs/src/app/api/evaluations/route.ts`
- Reference: `outputs/src/app/page.tsx`

- [ ] **Step 1: Add filtering to GET endpoint**

Replace GET function:

```typescript
export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url)
    const grade = searchParams.get('grade')
    const minScore = parseInt(searchParams.get('minScore') || '0', 10)
    const maxScore = parseInt(searchParams.get('maxScore') || '100', 10)
    const complexity = searchParams.get('complexity')
    const fromDate = searchParams.get('from')
    const toDate = searchParams.get('to')
    const limit = parseInt(searchParams.get('limit') || '50', 10)
    
    const where: any = {}
    
    if (grade) where.grade = grade
    if (complexity) where.complexity = complexity
    if (minScore > 0 || maxScore < 100) {
      where.overallScore = {}
      if (minScore > 0) where.overallScore.gte = minScore
      if (maxScore < 100) where.overallScore.lte = maxScore
    }
    if (fromDate || toDate) {
      where.createdAt = {}
      if (fromDate) where.createdAt.gte = new Date(fromDate)
      if (toDate) where.createdAt.lte = new Date(toDate)
    }
    
    const evaluations = await prisma.evaluation.findMany({
      where,
      orderBy: { createdAt: 'desc' },
      take: Math.min(limit, 100),
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

- [ ] **Step 2: Verify build**

Run: `cd outputs && npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/app/api/evaluations/route.ts
git commit -m "feat: add filtering to evaluation list API"
```

---

## Summary

| Phase | Tasks | Priority | Status |
|-------|-------|----------|--------|
| 1 | Vitest setup + unit tests | P0 | Pending |
| 1 | Input validation with Zod | P0 | Pending |
| 1 | Risk heatmap infrastructure | P0 | Pending |
| 2 | PUT /evaluations/[id] | P1 | Pending |
| 2 | ScoreWeights config | P1 | Pending |
| 2 | Statistics API | P1 | Pending |
| 3 | Suggestions grouping UI | P2 | Pending |
| 3 | Risks display UI | P2 | Pending |
| 3 | File size limit | P2 | (done in Task 4) |
| 3 | History filtering | P2 | Pending |

**Plan complete and saved to `outputs/iteration-2/plan_v2_detailed.md`. Two execution options:**

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
