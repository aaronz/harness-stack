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
  const result = await callLLM(llmConfig, content)
  
  const rawScore =
    result.scores.context * weights.context +
    result.scores.atomicity * weights.atomicity +
    result.scores.boundary * weights.boundary +
    result.scores.verifiability * weights.verifiability +
    result.scores.tech * weights.tech
  
  const penalty = COMPLEXITY_PENALTY[result.complexity] || 1.0
  const overallScore = Math.round(rawScore * penalty)
  
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
