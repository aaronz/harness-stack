// 评估维度类型
export type Dimension = 'context' | 'atomicity' | 'boundary' | 'verifiability' | 'technical';

// 维度得分
export interface DimensionScore {
  dimension: Dimension;
  score: number; // 0-100
  analysis: string;
  riskPoints: string[];
}

// 复杂度等级
export type ComplexityLevel = 'simple' | 'medium' | 'complex';

// 评估等级
export type Grade = 'S' | 'A' | 'B' | 'C';

// 评估结果
export interface EvaluationResult {
  id: string;
  requirementId: string;
  dimensions: {
    context: DimensionScore;
    atomicity: DimensionScore;
    boundary: DimensionScore;
    verifiability: DimensionScore;
    technical: DimensionScore;
  };
  finalScore: number;
  grade: Grade;
  complexityPenalty: number;
  complexity: ComplexityLevel;
  llmProvider: string;
  llmModel: string;
  analysisDetail: string;
  createdAt: Date;
}

// 优化建议
export interface OptimizationSuggestion {
  id: string;
  evaluationId: string;
  category: Dimension;
  suggestion: string;
  priority: 'high' | 'medium' | 'low';
}

// 需求
export interface Requirement {
  id: string;
  title: string;
  content: string;
  complexity?: ComplexityLevel;
  status: 'pending' | 'evaluated' | 'optimized';
  createdAt: Date;
  updatedAt: Date;
}

// LLM配置
export interface LLMConfig {
  id: string;
  provider: 'openai' | 'anthropic' | 'gemini' | 'ollama';
  model: string;
  apiKey?: string;
  baseUrl?: string;
  isDefault: boolean;
}

// 维度权重配置
export interface DimensionWeights {
  context: number;
  atomicity: number;
  boundary: number;
  verifiability: number;
  technical: number;
}

// 默认权重
export const DEFAULT_WEIGHTS: DimensionWeights = {
  context: 0.25,
  atomicity: 0.25,
  boundary: 0.20,
  verifiability: 0.15,
  technical: 0.15
};

// 复杂度惩罚系数
export const COMPLEXITY_PENALTIES: Record<ComplexityLevel, number> = {
  simple: 1.0,
  medium: 0.9,
  complex: 0.7
};

// 等级阈值
export const GRADE_THRESHOLDS: Record<Grade, number> = {
  S: 90,
  A: 75,
  B: 60,
  C: 0
};