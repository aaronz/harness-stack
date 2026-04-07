import {
  DimensionWeights,
  DimensionScore,
  ComplexityLevel,
  Grade,
  EvaluationResult,
  DEFAULT_WEIGHTS,
  COMPLEXITY_PENALTIES,
  GRADE_THRESHOLDS
} from '../types';

export interface CalculateInput {
  requirementId: string;
  dimensions: {
    context: number;
    atomicity: number;
    boundary: number;
    verifiability: number;
    technical: number;
  };
  complexity: ComplexityLevel;
  llmProvider: string;
  llmModel: string;
}

export function calculateWeightedSum(
  dimensions: Record<string, number>,
  weights: DimensionWeights
): number {
  const keys = Object.keys(weights) as (keyof DimensionWeights)[];
  let sum = 0;
  
  for (const key of keys) {
    sum += dimensions[key] * weights[key];
  }
  
  return sum;
}

export function calculateFinalScore(weightedSum: number, complexity: ComplexityLevel): number {
  const penalty = COMPLEXITY_PENALTIES[complexity];
  return Math.round(weightedSum * penalty * 10) / 10;
}

export function calculateGrade(score: number): Grade {
  if (score >= GRADE_THRESHOLDS.S) return 'S';
  if (score >= GRADE_THRESHOLDS.A) return 'A';
  if (score >= GRADE_THRESHOLDS.B) return 'B';
  return 'C';
}

export class ScoreCalculator {
  private weights: DimensionWeights;

  constructor(weights: DimensionWeights = DEFAULT_WEIGHTS) {
    this.weights = weights;
  }

  calculate(input: CalculateInput): EvaluationResult {
    const weightedSum = calculateWeightedSum(input.dimensions, this.weights);
    const finalScore = calculateFinalScore(weightedSum, input.complexity);
    const grade = calculateGrade(finalScore);
    const complexityPenalty = COMPLEXITY_PENALTIES[input.complexity];

    const dimensionScores: Record<string, DimensionScore> = {};
    const dimensionKeys = Object.keys(input.dimensions) as Array<keyof typeof input.dimensions>;
    
    for (const key of dimensionKeys) {
      dimensionScores[key] = {
        dimension: key,
        score: input.dimensions[key],
        analysis: '',
        riskPoints: []
      };
    }

    return {
      id: `eval_${Date.now()}_${Math.random().toString(36).substring(2, 9)}`,
      requirementId: input.requirementId,
      dimensions: dimensionScores as EvaluationResult['dimensions'],
      finalScore,
      grade,
      complexityPenalty,
      complexity: input.complexity,
      llmProvider: input.llmProvider,
      llmModel: input.llmModel,
      analysisDetail: JSON.stringify({}),
      createdAt: new Date()
    };
  }
}