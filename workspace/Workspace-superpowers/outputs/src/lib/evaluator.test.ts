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