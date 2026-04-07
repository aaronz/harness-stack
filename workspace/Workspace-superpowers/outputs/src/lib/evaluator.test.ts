import { describe, it, expect, vi } from 'vitest'
import { evaluateRequirement, DEFAULT_WEIGHTS, COMPLEXITY_PENALTY } from './evaluator'

vi.mock('./llm')

describe('evaluateRequirement', () => {
  it('should calculate weighted score correctly for simple complexity', async () => {
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 80, atomicity: 90, boundary: 70, verifiability: 85, tech: 75 },
      overall: 80, grade: 'A' as const, complexity: 'simple' as const, risks: ['Risk 1'], suggestions: []
    })
    const result = await evaluateRequirement('Test content', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.overallScore).toBe(81)
    expect(result.grade).toBe('A')
  })

  it('should apply complexity penalty for medium complexity', async () => {
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 100, atomicity: 100, boundary: 100, verifiability: 100, tech: 100 },
      overall: 100, grade: 'S' as const, complexity: 'medium' as const, risks: [], suggestions: []
    })
    const result = await evaluateRequirement('Complex content', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.overallScore).toBe(90)
    expect(result.grade).toBe('A')
  })

  it('should apply complexity penalty for complex', async () => {
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 100, atomicity: 100, boundary: 100, verifiability: 100, tech: 100 },
      overall: 100, grade: 'S' as const, complexity: 'complex' as const, risks: [], suggestions: []
    })
    const result = await evaluateRequirement('Complex content', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.overallScore).toBe(70)
    expect(result.grade).toBe('B')
  })

  it('should assign S grade for score >= 90', async () => {
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 100, atomicity: 100, boundary: 100, verifiability: 100, tech: 100 },
      overall: 100, grade: 'S' as const, complexity: 'simple' as const, risks: [], suggestions: []
    })
    const result = await evaluateRequirement('Test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.grade).toBe('S')
  })

  it('should assign A grade for score >= 75 and < 90', async () => {
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 80, atomicity: 80, boundary: 80, verifiability: 80, tech: 80 },
      overall: 80, grade: 'A' as const, complexity: 'simple' as const, risks: [], suggestions: []
    })
    const result = await evaluateRequirement('Test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.grade).toBe('A')
  })

  it('should assign B grade for score >= 60 and < 75', async () => {
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 70, atomicity: 70, boundary: 70, verifiability: 70, tech: 70 },
      overall: 70, grade: 'B' as const, complexity: 'simple' as const, risks: [], suggestions: []
    })
    const result = await evaluateRequirement('Test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.grade).toBe('B')
  })

  it('should assign C grade for score < 60', async () => {
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 50, atomicity: 50, boundary: 50, verifiability: 50, tech: 50 },
      overall: 50, grade: 'C' as const, complexity: 'simple' as const, risks: [], suggestions: []
    })
    const result = await evaluateRequirement('Test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.grade).toBe('C')
  })

  it('should pass through risks from LLM response', async () => {
    const mockRisks = ['Risk 1', 'Risk 2', 'Risk 3']
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 80, atomicity: 80, boundary: 80, verifiability: 80, tech: 80 },
      overall: 80, grade: 'A' as const, complexity: 'simple' as const, risks: mockRisks, suggestions: []
    })
    const result = await evaluateRequirement('Test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.risks).toEqual(mockRisks)
  })

  it('should pass through suggestions from LLM response', async () => {
    const mockSuggestions = [{ type: 'context', content: 'Add user journey' }]
    vi.mocked(vi.fn()).mockResolvedValue({
      scores: { context: 80, atomicity: 80, boundary: 80, verifiability: 80, tech: 80 },
      overall: 80, grade: 'A' as const, complexity: 'simple' as const, risks: [], suggestions: mockSuggestions
    })
    const result = await evaluateRequirement('Test', { provider: 'openai', model: 'gpt-4o', apiKey: 'test' })
    expect(result.suggestions).toEqual(mockSuggestions)
  })
})

describe('DEFAULT_WEIGHTS', () => {
  it('should sum to 1.0', () => {
    const total = DEFAULT_WEIGHTS.context + DEFAULT_WEIGHTS.atomicity + DEFAULT_WEIGHTS.boundary + DEFAULT_WEIGHTS.verifiability + DEFAULT_WEIGHTS.tech
    expect(total).toBe(1.0)
  })

  it('should have correct individual values', () => {
    expect(DEFAULT_WEIGHTS.context).toBe(0.25)
    expect(DEFAULT_WEIGHTS.atomicity).toBe(0.25)
    expect(DEFAULT_WEIGHTS.boundary).toBe(0.20)
    expect(DEFAULT_WEIGHTS.verifiability).toBe(0.15)
    expect(DEFAULT_WEIGHTS.tech).toBe(0.15)
  })
})

describe('COMPLEXITY_PENALTY', () => {
  it('should have correct penalty values', () => {
    expect(COMPLEXITY_PENALTY.simple).toBe(1.0)
    expect(COMPLEXITY_PENALTY.medium).toBe(0.9)
    expect(COMPLEXITY_PENALTY.complex).toBe(0.7)
  })
})
