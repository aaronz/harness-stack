# AI-Ready Evaluator - Implementation Plan v2

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement all P0 and P1 gaps identified in gap analysis, plus key P2 items. The system currently has ~75% feature completion.

**Architecture:** Next.js 14 + TypeScript frontend with API routes, Prisma + SQLite backend, multi-LLM support via provider abstraction.

**Tech Stack:** Next.js 14, TypeScript, Prisma (SQLite), Tailwind CSS, OpenAI/Anthropic/Gemini/Ollama SDKs

---

## ✅ IMPLEMENTATION COMPLETED (2026-04-03)

All 8 tasks have been completed and verified:

| Task | Feature | Status |
|------|---------|--------|
| 1 | Risk Highlight Component | ✅ Completed |
| 2 | Weights Configuration UI | ✅ Completed |
| 3 | LLM Config Edit Mode | ✅ Completed |
| 4 | Model Selector in FileUpload | ✅ Completed |
| 5 | Risks Section in Evaluation Page | ✅ Completed |
| 6 | Inline Title Edit | ✅ Completed |
| 7 | LLM Retry Mechanism | ✅ Completed |
| 8 | evaluator.test.ts | ✅ Completed |

Build passes: `npm run build` ✅

---

## Gap Analysis Summary

| Priority | Gap | Status |
|----------|-----|--------|
| P0 | Weights Configuration UI missing | ✅ 已实现 |
| P1 | Risk heatmap/highlight not implemented | ✅ 已实现 |
| P1 | LLM Config Edit functionality missing | ✅ 已实现 |
| P1 | Model selector UI missing | ✅ 已实现 |
| P1 | Risks array not displayed | ✅ 已实现 |
| P2 | Evaluation title edit UI | ✅ 已实现 |
| P2 | LLM retry mechanism | ✅ 已实现 |
| P2 | evaluator.test.ts incomplete | ✅ 已实现 |

---

## File Structure

```
outputs/src/
├── app/
│   ├── settings/page.tsx              # ADD: Weights UI section
│   └── evaluations/[id]/page.tsx      # MODIFY: Risk highlight, title edit, risks display
├── components/
│   ├── LLMConfigForm.tsx              # MODIFY: Add edit mode
│   ├── FileUpload.tsx                 # MODIFY: Add model selector dropdown
│   ├── WeightsForm.tsx                # CREATE: New component for weights CRUD
│   ├── RiskHighlight.tsx              # CREATE: New component for risk text highlighting
│   └── EditableTitle.tsx             # CREATE: New component for inline title editing
└── lib/
    └── llm.ts                         # MODIFY: Add retry mechanism

outputs/prisma/
└── schema.prisma                      # ScoreWeights model already exists
```

---

## Task 1: Add Risk Highlight Component (P1)

**Files:**
- Create: `outputs/src/components/RiskHighlight.tsx`
- Modify: `outputs/src/app/evaluations/[id]/page.tsx`

- [ ] **Step 1: Create RiskHighlight component**

```tsx
// outputs/src/components/RiskHighlight.tsx
'use client'

interface RiskHighlightProps {
  content: string
  risks: string[]
}

export default function RiskHighlight({ content, risks }: RiskHighlightProps) {
  if (!risks || risks.length === 0) {
    return <pre className="whitespace-pre-wrap text-sm text-gray-700 bg-gray-50 p-4 rounded-lg overflow-auto">{content}</pre>
  }

  // Sort risks by length (longer matches first) to avoid partial replacements
  const sortedRisks = [...risks].sort((a, b) => b.length - a.length)
  
  // Escape regex special characters and create highlight pattern
  let highlighted = content
  
  sortedRisks.forEach((risk, index) => {
    const escapedRisk = risk.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
    const regex = new RegExp(escapedRisk, 'gi')
    highlighted = highlighted.replace(regex, `{{RISK_${index}}}`)
  })

  // Split by risk markers and render with highlighting
  const parts = highlighted.split(/({{RISK_\d+}})/g)
  
  return (
    <pre className="whitespace-pre-wrap text-sm text-gray-700 bg-gray-50 p-4 rounded-lg overflow-auto">
      {parts.map((part, i) => {
        const match = part.match(/{{RISK_(\d+)}}/)
        if (match) {
          const riskIndex = parseInt(match[1])
          return (
            <mark key={i} className="bg-red-200 text-red-900 px-1 rounded">
              {sortedRisks[riskIndex]}
            </mark>
          )
        }
        return <span key={i}>{part}</span>
      })}
    </pre>
  )
}
```

- [ ] **Step 2: Import and use RiskHighlight in evaluation page**

Modify `outputs/src/app/evaluations/[id]/page.tsx`:
```tsx
import RiskHighlight from '@/components/RiskHighlight'
```

Replace the Original Content section (around line 137-142):
```tsx
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-medium mb-4">Original Content</h2>
        <RiskHighlight content={evaluation.content} risks={evaluation.risks || []} />
      </div>
```

- [ ] **Step 3: Add risks to EvaluationDetail interface and parse from rawResponse**

Add `risks: string[]` to the interface, and update the useEffect to extract risks:
```tsx
useEffect(() => {
  fetch(`/api/evaluations/${params.id}`)
    .then(res => res.json())
    .then(data => {
      if (data.rawResponse) {
        try {
          const raw = JSON.parse(data.rawResponse)
          data.risks = raw.risks || []
        } catch {
          data.risks = []
        }
      }
      setEvaluation(data)
    })
    .catch(console.error)
    .finally(() => setLoading(false))
}, [params.id])
```

- [ ] **Step 4: Verify build passes**

Run: `cd outputs && npm run build`  
Expected: Build completes without errors

---

## Task 2: Add Weights Configuration UI (P0)

**Files:**
- Create: `outputs/src/components/WeightsForm.tsx`
- Create: `outputs/src/app/api/config/weights/[id]/route.ts`
- Modify: `outputs/src/app/settings/page.tsx`

- [ ] **Step 1: Create WeightsForm component**

```tsx
// outputs/src/components/WeightsForm.tsx
'use client'

import { useState, useEffect } from 'react'

interface ScoreWeights {
  id: string
  name: string
  context: number
  atomicity: number
  boundary: number
  verifiability: number
  tech: number
  isDefault: boolean
  createdAt: string
}

const DEFAULT_VALUES = {
  context: 0.25,
  atomicity: 0.25,
  boundary: 0.20,
  verifiability: 0.15,
  tech: 0.15,
}

export default function WeightsForm() {
  const [weights, setWeights] = useState<ScoreWeights[]>([])
  const [loading, setLoading] = useState(true)
  const [showForm, setShowForm] = useState(false)
  const [editingId, setEditingId] = useState<string | null>(null)
  const [formData, setFormData] = useState({ name: '', ...DEFAULT_VALUES, isDefault: false })
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState('')

  const fetchWeights = async () => {
    try {
      const res = await fetch('/api/config/weights')
      if (res.ok) setWeights(await res.json())
    } catch (err) {
      console.error('Failed to fetch weights:', err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => { fetchWeights() }, [])

  const total = formData.context + formData.atomicity + formData.boundary + 
                 formData.verifiability + formData.tech

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError('')
    if (Math.abs(total - 1.0) > 0.01) {
      setError(`Weights must sum to 1.0 (current: ${total.toFixed(2)})`)
      return
    }
    setSaving(true)
    try {
      const url = editingId ? `/api/config/weights/${editingId}` : '/api/config/weights'
      const method = editingId ? 'PUT' : 'POST'
      await fetch(url, { method, headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(formData) })
      setShowForm(false)
      setEditingId(null)
      setFormData({ name: '', ...DEFAULT_VALUES, isDefault: false })
      fetchWeights()
    } catch { setError('Failed to save weights') } finally { setSaving(false) }
  }

  const handleEdit = (w: ScoreWeights) => {
    setEditingId(w.id)
    setFormData({ name: w.name, context: w.context, atomicity: w.atomicity, boundary: w.boundary, verifiability: w.verifiability, tech: w.tech, isDefault: w.isDefault })
    setShowForm(true)
  }

  const handleDelete = async (id: string) => {
    if (!confirm('Delete this weights preset?')) return
    await fetch(`/api/config/weights/${id}`, { method: 'DELETE' })
    fetchWeights()
  }

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-medium">Score Weights Configuration</h2>
        <button onClick={() => setShowForm(!showForm)} className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700">
          {showForm ? 'Cancel' : '+ Add Preset'}
        </button>
      </div>

      {error && <div className="text-red-600 text-sm mb-4">{error}</div>}

      {showForm && (
        <form onSubmit={handleSubmit} className="space-y-4 mb-6 border-b pb-6">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">Preset Name</label>
            <input type="text" value={formData.name} onChange={(e) => setFormData({ ...formData, name: e.target.value })} placeholder="e.g., Strict Evaluation" className="block w-full px-3 py-2 border border-gray-300 rounded-md" required />
          </div>
          <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
            {[{ key: 'context', label: 'Context (25%)' }, { key: 'atomicity', label: 'Atomicity (25%)' }, { key: 'boundary', label: 'Boundary (20%)' }, { key: 'verifiability', label: 'Verifiability (15%)' }, { key: 'tech', label: 'Tech (15%)' }].map(({ key, label }) => (
              <div key={key}>
                <label className="block text-sm font-medium text-gray-700 mb-1">{label}</label>
                <input type="number" step="0.01" min="0" max="1" value={formData[key as keyof typeof formData]} onChange={(e) => setFormData({ ...formData, [key]: parseFloat(e.target.value) || 0 })} className="block w-full px-3 py-2 border border-gray-300 rounded-md" />
              </div>
            ))}
          </div>
          <div className="text-sm text-gray-500">Total: {total.toFixed(2)} {Math.abs(total - 1.0) > 0.01 && <span className="text-red-600">(must be 1.0)</span>}</div>
          <div className="flex items-center gap-2">
            <input type="checkbox" id="isDefault" checked={formData.isDefault} onChange={(e) => setFormData({ ...formData, isDefault: e.target.checked })} className="rounded border-gray-300" />
            <label htmlFor="isDefault" className="text-sm text-gray-700">Set as default preset</label>
          </div>
          <div className="flex gap-2">
            <button type="submit" disabled={saving} className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50">
              {saving ? 'Saving...' : editingId ? 'Update Preset' : 'Save Preset'}
            </button>
            {showForm && <button type="button" onClick={() => { setShowForm(false); setEditingId(null); setFormData({ name: '', ...DEFAULT_VALUES, isDefault: false }); setError('') }} className="px-4 py-2 bg-gray-300 text-gray-700 rounded-md hover:bg-gray-400">Cancel</button>}
          </div>
        </form>
      )}

      {loading ? <div className="text-center py-8 text-gray-500">Loading...</div> : weights.length === 0 ? <div className="text-center py-8 text-gray-500">No weight presets configured.</div> : (
        <div className="space-y-2">
          {weights.map((w) => (
            <div key={w.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <div>
                <div className="font-medium">{w.name}{w.isDefault && <span className="ml-2 px-2 py-0.5 text-xs bg-green-100 text-green-800 rounded">Default</span>}</div>
                <div className="text-sm text-gray-500">C:{w.context} | A:{w.atomicity} | B:{w.boundary} | V:{w.verifiability} | T:{w.tech}</div>
              </div>
              <div className="flex gap-2">
                <button onClick={() => handleEdit(w)} className="text-blue-600 hover:text-blue-900 text-sm">Edit</button>
                <button onClick={() => handleDelete(w.id)} className="text-red-600 hover:text-red-900 text-sm">Delete</button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
```

- [ ] **Step 2: Create DELETE/PUT endpoint for weights**

Create: `outputs/src/app/api/config/weights/[id]/route.ts`
```tsx
import { NextRequest, NextResponse } from 'next/server'
import { prisma } from '@/lib/prisma'

export async function DELETE(request: NextRequest, { params }: { params: { id: string } }) {
  try {
    await prisma.scoreWeights.delete({ where: { id: params.id } })
    return NextResponse.json({ success: true })
  } catch (error) {
    return NextResponse.json({ error: 'Failed to delete weights' }, { status: 500 })
  }
}

export async function PUT(request: NextRequest, { params }: { params: { id: string } }) {
  try {
    const body = await request.json()
    const { name, context, atomicity, boundary, verifiability, tech, isDefault } = body
    const updateData: Record<string, unknown> = {}
    if (name !== undefined) updateData.name = name
    if (context !== undefined) updateData.context = context
    if (atomicity !== undefined) updateData.atomicity = atomicity
    if (boundary !== undefined) updateData.boundary = boundary
    if (verifiability !== undefined) updateData.verifiability = verifiability
    if (tech !== undefined) updateData.tech = tech
    if (isDefault) {
      await prisma.scoreWeights.updateMany({ where: { isDefault: true }, data: { isDefault: false } })
      updateData.isDefault = true
    }
    const weights = await prisma.scoreWeights.update({ where: { id: params.id }, data: updateData })
    return NextResponse.json({ id: weights.id, name: weights.name, context: weights.context, atomicity: weights.atomicity, boundary: weights.boundary, verifiability: weights.verifiability, tech: weights.tech, isDefault: weights.isDefault })
  } catch (error) {
    return NextResponse.json({ error: 'Failed to update weights' }, { status: 500 })
  }
}
```

- [ ] **Step 3: Import WeightsForm in settings page**

Modify `outputs/src/app/settings/page.tsx`:
```tsx
import WeightsForm from '@/components/WeightsForm'
// Add in JSX after LLMConfigForm:
<WeightsForm />
```

- [ ] **Step 4: Verify build passes**

Run: `cd outputs && npm run build`  
Expected: Build completes without errors

---

## Task 3: Add Edit Mode to LLMConfigForm (P1)

**Files:**
- Modify: `outputs/src/components/LLMConfigForm.tsx`

- [ ] **Step 1: Add editing state and handleEdit function**

Add to state:
```tsx
const [editingId, setEditingId] = useState<string | null>(null)
```

Add handleEdit function:
```tsx
const handleEdit = (config: LLMConfig) => {
  setEditingId(config.id)
  setFormData({ name: config.name, provider: config.provider, model: config.model, apiKey: '', baseUrl: config.baseUrl || '', isDefault: config.isDefault })
  setShowForm(true)
}
```

- [ ] **Step 2: Update handleSubmit for edit mode**

```tsx
const handleSubmit = async (e: React.FormEvent) => {
  e.preventDefault()
  setSaving(true)
  try {
    const url = editingId ? `/api/config/llm/${editingId}` : '/api/config/llm'
    const method = editingId ? 'PUT' : 'POST'
    const body = editingId ? { ...formData, apiKey: formData.apiKey || undefined } : formData
    await fetch(url, { method, headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) })
    setShowForm(false)
    setEditingId(null)
    setFormData({ name: '', provider: 'openai', model: 'gpt-4o', apiKey: '', baseUrl: '', isDefault: false })
    onRefresh()
  } catch (error) { console.error('Failed to save config:', error) } finally { setSaving(false) }
}
```

- [ ] **Step 3: Update form button text**

Replace submit button text:
```tsx
{saving ? 'Saving...' : editingId ? 'Update Configuration' : 'Save Configuration'}
```

- [ ] **Step 4: Add Edit button next to Delete**

Add Edit button:
```tsx
<button onClick={() => handleEdit(config)} className="text-blue-600 hover:text-blue-900 text-sm">Edit</button>
```

- [ ] **Step 5: Verify build passes**

Run: `cd outputs && npm run build`  
Expected: Build completes without errors

---

## Task 4: Add Model Selector to FileUpload (P1)

**Files:**
- Modify: `outputs/src/components/FileUpload.tsx`
- Modify: `outputs/src/app/page.tsx`

- [ ] **Step 1: Update FileUpload props and add model selector**

Update interface:
```tsx
interface FileUploadProps {
  onUpload: (file: File, title?: string, modelId?: string) => Promise<void>
  models?: Array<{ id: string; name: string; model: string }>
}
```

Add state:
```tsx
const [selectedModelId, setSelectedModelId] = useState<string>('')
```

Add dropdown after title input:
```tsx
{props.models && props.models.length > 1 && (
  <div>
    <label className="block text-sm font-medium text-gray-700 mb-2">Model (optional)</label>
    <select value={selectedModelId} onChange={(e) => setSelectedModelId(e.target.value)} className="block w-full px-3 py-2 border border-gray-300 rounded-md">
      <option value="">Use default model</option>
      {props.models.map(m => <option key={m.id} value={m.id}>{m.name} ({m.model})</option>)}
    </select>
  </div>
)}
```

Update handleSubmit:
```tsx
await onUpload(file, title, selectedModelId || undefined)
```

- [ ] **Step 2: Update page.tsx to fetch and pass models**

Add state:
```tsx
const [models, setModels] = useState<Model[]>([])
```

Add fetchModels:
```tsx
const fetchModels = async () => {
  try {
    const res = await fetch('/api/config/llm')
    if (res.ok) setModels(await res.json())
  } catch (error) { console.error('Failed to fetch models:', error) }
}
```

Update useEffect:
```tsx
useEffect(() => { fetchEvaluations(); fetchModels(); }, [])
```

Update FileUpload:
```tsx
<FileUpload onUpload={handleUpload} models={models} />
```

- [ ] **Step 3: Verify build passes**

Run: `cd outputs && npm run build`  
Expected: Build completes without errors

---

## Task 5: Display Risks Section in Evaluation Page (P1)

**Files:**
- Modify: `outputs/src/app/evaluations/[id]/page.tsx`

- [ ] **Step 1: Add RisksSection before Original Content**

```tsx
{evaluation.risks && evaluation.risks.length > 0 && (
  <div className="bg-white rounded-lg shadow p-6">
    <h2 className="text-lg font-medium mb-4">Context Gaps & Risks</h2>
    <p className="text-sm text-gray-600 mb-4">These areas may cause AI to generate incorrect or incomplete code:</p>
    <ul className="space-y-2">
      {evaluation.risks.map((risk, index) => (
        <li key={index} className="flex items-start gap-2 text-gray-700">
          <span className="mt-1.5 w-2 h-2 rounded-full bg-red-500 flex-shrink-0" />
          <span>{risk}</span>
        </li>
      ))}
    </ul>
  </div>
)}
```

- [ ] **Step 2: Verify build passes**

Run: `cd outputs && npm run build`  
Expected: Build completes without errors

---

## Task 6: Add Inline Title Edit (P2)

**Files:**
- Create: `outputs/src/components/EditableTitle.tsx`
- Modify: `outputs/src/app/evaluations/[id]/page.tsx`

- [ ] **Step 1: Create EditableTitle component**

```tsx
// outputs/src/components/EditableTitle.tsx
'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'

interface EditableTitleProps {
  title: string
  evaluationId: string
}

export default function EditableTitle({ title, evaluationId }: EditableTitleProps) {
  const [isEditing, setIsEditing] = useState(false)
  const [editValue, setEditValue] = useState(title)
  const [saving, setSaving] = useState(false)
  const router = useRouter()

  const handleSave = async () => {
    if (editValue.trim() === title) { setIsEditing(false); return }
    setSaving(true)
    try {
      await fetch(`/api/evaluations/${evaluationId}`, { method: 'PUT', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ title: editValue.trim() }) })
      setIsEditing(false)
      router.refresh()
    } catch (error) { console.error('Failed to update title:', error) } finally { setSaving(false) }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') handleSave()
    else if (e.key === 'Escape') { setEditValue(title); setIsEditing(false) }
  }

  if (isEditing) {
    return (
      <div className="flex items-center gap-2">
        <input type="text" value={editValue} onChange={(e) => setEditValue(e.target.value)} onKeyDown={handleKeyDown} onBlur={handleSave} autoFocus className="text-2xl font-bold text-gray-900 border border-blue-300 rounded px-2 py-1" />
        <button onClick={handleSave} disabled={saving} className="text-blue-600 hover:text-blue-800 text-sm">{saving ? 'Saving...' : 'Save'}</button>
      </div>
    )
  }

  return (
    <div className="flex items-center gap-2">
      <h1 className="text-2xl font-bold text-gray-900">{title}</h1>
      <button onClick={() => setIsEditing(true)} className="text-gray-400 hover:text-gray-600 text-sm" title="Edit title">✏️</button>
    </div>
  )
}
```

- [ ] **Step 2: Import and use in evaluation page**

Add import:
```tsx
import EditableTitle from '@/components/EditableTitle'
```

Replace title section:
```tsx
<EditableTitle title={evaluation.title} evaluationId={evaluation.id} />
```

- [ ] **Step 3: Verify build passes**

Run: `cd outputs && npm run build`  
Expected: Build completes without errors

---

## Task 7: Add LLM Retry Mechanism (P2)

**Files:**
- Modify: `outputs/src/lib/llm.ts`

- [ ] **Step 1: Add withRetry function**

Add before the provider functions:
```tsx
async function withRetry<T>(fn: () => Promise<T>, maxRetries: number = 3, baseDelayMs: number = 1000): Promise<T> {
  let lastError: Error | null = null
  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await fn()
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error))
      if (lastError.message.includes('Invalid model ID') || lastError.message.includes('api key')) throw lastError
      if (attempt < maxRetries) {
        const delay = baseDelayMs * Math.pow(2, attempt) + Math.random() * 1000
        await new Promise(resolve => setTimeout(resolve, delay))
      }
    }
  }
  throw lastError || new Error('Max retries exceeded')
}
```

- [ ] **Step 2: Wrap all provider functions**

Wrap each function body with `return withRetry(async () => { ... })`

- [ ] **Step 3: Verify build passes**

Run: `cd outputs && npm run build`  
Expected: Build completes without errors

---

## Task 8: Update evaluator.test.ts (P2)

**Files:**
- Modify: `outputs/src/lib/evaluator.test.ts`

- [ ] **Step 1: Write comprehensive tests**

```tsx
import { describe, it, expect, vi } from 'vitest'
import { evaluateRequirement, DEFAULT_WEIGHTS, COMPLEXITY_PENALTY } from './evaluator'
import * as llm from './llm'

vi.mock('./llm')

describe('evaluateRequirement', () => {
  it('should calculate weighted score correctly', async () => {
    vi.mocked(llm.callLLM).mockResolvedValue({
      scores: { context: 80, atomicity: 90, boundary: 70, verifiability: 85, tech: 75 },
      overall: 80, grade: 'A' as const, complexity: 'simple' as const, risks: ['Risk 1'], suggestions: []
    })
    const result = await evaluateRequirement('Test content', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    // 80*0.25 + 90*0.25 + 70*0.20 + 85*0.15 + 75*0.15 = 20 + 22.5 + 14 + 12.75 + 11.25 = 80.5 -> 81
    expect(result.overallScore).toBe(81)
    expect(result.grade).toBe('A')
  })

  it('should apply complexity penalty', async () => {
    vi.mocked(llm.callLLM).mockResolvedValue({
      scores: { context: 100, atomicity: 100, boundary: 100, verifiability: 100, tech: 100 },
      overall: 100, grade: 'S' as const, complexity: 'complex' as const, risks: [], suggestions: []
    })
    const result = await evaluateRequirement('Complex content', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.overallScore).toBe(70) // 100 * 0.7
    expect(result.grade).toBe('B')
  })

  it('should assign correct grades', async () => {
    const cases = [{ score: 95, grade: 'S' }, { score: 80, grade: 'A' }, { score: 65, grade: 'B' }, { score: 50, grade: 'C' }]
    for (const { score, grade } of cases) {
      vi.mocked(llm.callLLM).mockResolvedValue({
        scores: { context: score, atomicity: score, boundary: score, verifiability: score, tech: score },
        overall: score, grade: grade as 'S' | 'A' | 'B' | 'C', complexity: 'simple' as const, risks: [], suggestions: []
      })
      const result = await evaluateRequirement('Test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
      expect(result.grade).toBe(grade)
    }
  })
})

describe('DEFAULT_WEIGHTS', () => {
  it('should sum to 1.0', () => {
    const total = DEFAULT_WEIGHTS.context + DEFAULT_WEIGHTS.atomicity + DEFAULT_WEIGHTS.boundary + DEFAULT_WEIGHTS.verifiability + DEFAULT_WEIGHTS.tech
    expect(total).toBe(1.0)
  })
})

describe('COMPLEXITY_PENALTY', () => {
  it('should have correct values', () => {
    expect(COMPLEXITY_PENALTY.simple).toBe(1.0)
    expect(COMPLEXITY_PENALTY.medium).toBe(0.9)
    expect(COMPLEXITY_PENALTY.complex).toBe(0.7)
  })
})
```

- [ ] **Step 2: Run tests**

Run: `cd outputs && npm test`  
Expected: All tests pass

---

## Verification Checklist

- [ ] Task 1: Risk highlight shows in evaluation detail page
- [ ] Task 2: Weights UI appears in Settings page with CRUD operations
- [ ] Task 3: LLMConfigForm shows Edit button and supports editing
- [ ] Task 4: FileUpload shows model dropdown when multiple models exist
- [ ] Task 5: Risks section appears in evaluation detail page
- [ ] Task 6: Title can be edited inline in evaluation detail page
- [ ] Task 7: LLM calls retry on transient failures
- [ ] Task 8: All evaluator tests pass

---

## Self-Review Checklist

1. **Spec coverage**: All P0 and P1 gaps from gap analysis are addressed
2. **Placeholder scan**: All code blocks are complete with actual implementation
3. **Type consistency**: All interfaces properly extended
4. **File paths**: All paths use exact locations under `outputs/src/`

---

## Plan Complete

**Plan saved to `./outputs/iteration-2/plan_v2.md`.**

Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**
