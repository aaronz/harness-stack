import { describe, it, expect, vi } from 'vitest'
import { callLLM, parseResponse } from './llm'

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
    const config = { provider: 'unknown' as any, model: 'test', apiKey: 'test' }
    
    await expect(callLLM(config, 'test')).rejects.toThrow('Unknown provider')
  })
})