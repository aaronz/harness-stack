// 评估维度枚举
export enum EvaluationDimension {
  ContextSufficiency = 'context_sufficiency', // 上下文完备性 (25%)
  Atomicity = 'atomicity', // 逻辑原子性 (25%)
  BoundaryDefiniteness = 'boundary_definiteness', // 边界明确性 (20%)
  Verifiability = 'verifiability', // 可验证性 (15%)
  TechnicalConstraints = 'technical_constraints' // 技术约束清晰度 (15%)
}

// 维度权重配置
export const DIMENSION_WEIGHTS: Record<EvaluationDimension, number> = {
  [EvaluationDimension.ContextSufficiency]: 0.25,
  [EvaluationDimension.Atomicity]: 0.25,
  [EvaluationDimension.BoundaryDefiniteness]: 0.2,
  [EvaluationDimension.Verifiability]: 0.15,
  [EvaluationDimension.TechnicalConstraints]: 0.15
};

// 复杂度等级
export enum ComplexityLevel {
  Simple = 'simple', // CRUD类 - 惩罚系数 1.0
  Medium = 'medium', // 业务逻辑类 - 惩罚系数 0.9
  Complex = 'complex' // 算法/架构类 - 惩罚系数 0.7
}

// 复杂度惩罚系数
export const COMPLEXITY_PENALTY: Record<ComplexityLevel, number> = {
  [ComplexityLevel.Simple]: 1.0,
  [ComplexityLevel.Medium]: 0.9,
  [ComplexityLevel.Complex]: 0.7
};

// AI就绪等级
export enum ReadinessLevel {
  S = 'S', // 90-100分 - AI可独立完成
  A = 'A', // 75-89分 - AI可实现，需人工Review
  B = 'B', // 60-74分 - AI生成后需人工大幅修改
  C = 'C' // <60分 - 不建议AI实现
}

// 单维度评分结果
export interface DimensionScore {
  dimension: EvaluationDimension;
  score: number; // 0-100
  maxScore: number; // 最大得分
  weight: number; // 权重
  analysis: string; // 分析说明
  keyCheckpoints: string[]; // 关键检查点
  riskAreas: string[]; // 风险区域
}

// 完整评估结果
export interface EvaluationResult {
  requirementText: string;
  complexity: ComplexityLevel;
  dimensionScores: DimensionScore[];
  totalScore: number; // 加权总分
  readinessLevel: ReadinessLevel;
  recommendations: Recommendation[];
  riskHeatmap: RiskHighlight[];
  createdAt: Date;
}

// 优化建议
export interface Recommendation {
  dimension: EvaluationDimension;
  category: 'context' | 'boundary' | 'atomicity' | 'verifiability' | 'technical';
  priority: 'high' | 'medium' | 'low';
  title: string;
  description: string;
  example?: string;
}

// 风险热力图
export interface RiskHighlight {
  startIndex: number;
  endIndex: number;
  text: string;
  severity: 'high' | 'medium' | 'low';
  relatedDimension: EvaluationDimension;
  reason: string;
}
