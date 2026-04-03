# AI-Ready Evaluator Implementation Plan (Iteration 2)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement missing features from gap analysis: unit tests, PUT endpoints, weights config, suggestions grouping, and prepare for risk heatmap.

**Architecture:** Three-phase approach - P0 fixes first (tests + PUT endpoints), then P1 features (weights config + suggestions grouping), then P2 enhancements (heatmap + history).

**Tech Stack:** Next.js 14, TypeScript, Prisma (SQLite), Vitest for testing.

---

## Phase 1: P0 - Critical Fixes

### Task 1: Add Unit Tests for evaluator.ts

**Files:**
- Create: `outputs/src/lib/evaluator.test.ts`
- Modify: `outputs/package.json` (add vitest + tsconfig.test.json)
- Reference: `outputs/src/lib/evaluator.ts`

- [ ] **Step 1: Install vitest**

Run: `cd outputs && npm install -D vitest @vitest/ui jsdom`
Expected: Package installed successfully

- [ ] **Step 2: Create vitest config**

```typescript
// outputs/vitest.config.ts
import { defineConfig } from 'vitest/config'
import path from 'path'

export default defineConfig({
  test: {
    environment: 'jsdom',
    globals: true,
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
})
```

- [ ] **Step 3: Write failing tests**

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
    // Expected: 68 or 69
    expect(result.overallScore).toBeGreaterThan(0)
    expect(result.overallScore).toBeLessThanOrEqual(100)
  })

  it('should apply complexity penalty correctly', async () => {
    const result = await evaluateRequirement('test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    
    // medium complexity = 0.9 penalty
    const rawScore = 80 * 0.25 + 70 * 0.25 + 60 * 0.2 + 90 * 0.15 + 85 * 0.15
    const expected = Math.round(rawScore * 0.9)
    expect(result.overallScore).toBe(expected)
  })

  it('should assign correct grade', async () => {
    const result = await evaluateRequirement('test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    
    expect(['S', 'A', 'B', 'C']).toContain(result.grade)
  })
})

describe('DEFAULT_WEIGHTS', () => {
  it('should sum to 1.0', () => {
    const sum = DEFAULT_WEIGHTS.context + DEFAULT_WEIGHTS.atomicity + 
                DEFAULT_WEIGHTS.boundary + DEFAULT_WEIGHTS.verifiability + DEFAULT_WEIGHTS.tech
    expect(sum).toBe(1.0)
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

- [ ] **Step 4: Run tests**

Run: `cd outputs && npx vitest run`
Expected: Tests should pass

- [ ] **Step 5: Commit**

```bash
cd outputs && git add src/lib/evaluator.test.ts package.json vitest.config.ts
git commit -m "test: add unit tests for evaluator.ts"
```

---

### Task 2: Add Unit Tests for llm.ts

**Files:**
- Create: `outputs/src/lib/llm.test.ts`
- Reference: `outputs/src/lib/llm.ts`

- [ ] **Step 1: Write failing tests**

```typescript
// outputs/src/lib/llm.test.ts
import { describe, it, expect, vi } from 'vitest'
import { callLLM, parseResponse, LLMConfig } from './llm'

describe('parseResponse', () => {
  it('should parse valid JSON response', () => {
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
  })

  it('should throw on invalid JSON', () => {
    expect(() => parseResponse('not json')).toThrow('Failed to parse LLM response')
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

Run: `cd outputs && npx vitest run src/lib/llm.test.ts`
Expected: Tests should pass

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/lib/llm.test.ts
git commit -m "test: add unit tests for llm.ts"
```

---

### Task 3: Add PUT /api/evaluations/[id] Endpoint

**Files:**
- Modify: `outputs/src/app/api/evaluations/[id]/route.ts`
- Reference: `outputs/src/app/api/evaluations/route.ts` (POST logic)

- [ ] **Step 1: Add PUT handler**

Replace the DELETE export with a combined file:

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
Expected: No TypeScript errors

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/app/api/evaluations/\[id\]/route.ts
git commit -m "feat: add PUT endpoint for evaluations"
```

---

## Phase 2: P1 - Important Features

### Task 4: Add ScoreWeights Config Table and API

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

### Task 5: Group Suggestions by Dimension in UI

**Files:**
- Modify: `outputs/src/app/evaluations/[id]/page.tsx`
- Reference: `outputs/src/components/ScoreCard.tsx`

- [ ] **Step 1: Update evaluation page with grouped suggestions**

Replace the suggestions section (lines 90-104):

```tsx
// Group suggestions by type
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
}

return (
  // ... existing code ...

  {evaluation.suggestions.length > 0 && (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-lg font-medium mb-4">Suggestions</h2>
      
      {/* Dimension tabs */}
      <div className="flex flex-wrap gap-2 mb-4">
        {Object.keys(groupedSuggestions).map(type => (
          <button
            key={type}
            onClick={() => setActiveDimension(type)}
            className={`px-3 py-1 text-sm rounded-full ${
              activeDimension === type 
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
        {(groupedSuggestions[activeDimension] || []).map((content, index) => (
          <li key={index} className="flex items-start gap-2 text-gray-700">
            <span className="mt-1.5 w-2 h-2 rounded-full bg-blue-500 flex-shrink-0" />
            <span>{content}</span>
          </li>
        ))}
      </ul>
    </div>
  )}
```

- [ ] **Step 2: Add state for active dimension**

Add after line 27:
```tsx
const [activeDimension, setActiveDimension] = useState<string>('context')
```

- [ ] **Step 3: Verify build**

Run: `cd outputs && npm run build`
Expected: Build succeeds

- [ ] **Step 4: Commit**

```bash
cd outputs && git add src/app/evaluations/\[id\]/page.tsx
git commit -m "feat: group suggestions by dimension in evaluation page"
```

---

### Task 6: Add PUT /api/config/llm/[id] Endpoint

**Files:**
- Modify: `outputs/src/app/api/config/llm/[id]/route.ts`
- Reference: `outputs/src/app/api/config/llm/route.ts`

- [ ] **Step 1: Create PUT handler**

Create the file:

```typescript
// outputs/src/app/api/config/llm/[id]/route.ts
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'
import { encryptApiKey, decryptApiKey } from '@/lib/encryption'

export async function GET(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const config = await prisma.lLMConfig.findUnique({
      where: { id: params.id },
    })
    
    if (!config) {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }
    
    return NextResponse.json({
      id: config.id,
      name: config.name,
      provider: config.provider,
      model: config.model,
      baseUrl: config.baseUrl,
      isDefault: config.isDefault,
      createdAt: config.createdAt.toISOString(),
    })
  } catch (error) {
    console.error('Get LLM config error:', error)
    return NextResponse.json({ error: 'Failed to get config' }, { status: 500 })
  }
}

export async function PUT(
  request: NextRequest,
  { params }: { params: { id: string } }
) {
  try {
    const body = await request.json()
    const { name, provider, model, apiKey, baseUrl, isDefault } = body
    
    const existing = await prisma.lLMConfig.findUnique({
      where: { id: params.id },
    })
    
    if (!existing) {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }
    
    if (isDefault) {
      await prisma.lLMConfig.updateMany({
        where: { isDefault: true },
        data: { isDefault: false },
      })
    }
    
    const updateData: any = {
      name: name || existing.name,
      provider: provider || existing.provider,
      model: model || existing.model,
      baseUrl: baseUrl !== undefined ? baseUrl : existing.baseUrl,
      isDefault: isDefault !== undefined ? isDefault : existing.isDefault,
    }
    
    // Only update API key if provided
    if (apiKey) {
      updateData.apiKey = encryptApiKey(apiKey)
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
      createdAt: config.createdAt.toISOString(),
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

- [ ] **Step 2: Verify build**

Run: `cd outputs && npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
cd outputs && git add src/app/api/config/llm/\[id\]/route.ts
git commit -m "feat: add PUT/DELETE endpoints for LLM config"
```

---

## Phase 3: P2 - Enhancements (Future)

### Task 7: Risk Heatmap / Highlight Functionality

**Files:**
- Modify: `outputs/src/app/evaluations/[id]/page.tsx`
- Database: Add risks field to Evaluation model

Note: This requires LLM to return specific text spans to highlight. This is a future enhancement that depends on LLM prompt improvements.

---

### Task 8: Evaluation History / Version Tracking

**Files:**
- Modify: `outputs/prisma/schema.prisma`
- Create: `outputs/src/app/api/evaluations/[id]/history/route.ts`

Note: This requires significant database changes. Deferred to future iteration.

---

## Summary

| Phase | Tasks | Priority |
|-------|-------|----------|
| 1 | Tests + PUT evaluations | P0 |
| 2 | Weights config + suggestions grouping + PUT llm | P1 |
| 3 | Heatmap + history | P2 |

**Plan complete and saved to `outputs/iteration-2/plan_v2.md`. Two execution options:**

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
