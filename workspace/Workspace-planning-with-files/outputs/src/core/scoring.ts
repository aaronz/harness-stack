import {
  EvaluationDimension,
  DIMENSION_WEIGHTS,
  ComplexityLevel,
  COMPLEXITY_PENALTY,
  ReadinessLevel,
  DimensionScore,
  EvaluationResult,
  Recommendation,
  RiskHighlight
} from '../types/evaluation';

export class ScoringCalculator {
  static calculateWeightedScore(dimensionScores: DimensionScore[]): number {
    let total = 0;
    for (const score of dimensionScores) {
      const weight = score.weight ?? DIMENSION_WEIGHTS[score.dimension];
      total += score.score * weight;
    }
    return Math.round(total * 100) / 100;
  }

  static applyComplexityPenalty(
    weightedScore: number,
    complexity: ComplexityLevel
  ): number {
    const penalty = COMPLEXITY_PENALTY[complexity];
    return Math.round(weightedScore * penalty * 100) / 100;
  }

  static determineReadinessLevel(score: number): ReadinessLevel {
    if (score >= 90) return ReadinessLevel.S;
    if (score >= 75) return ReadinessLevel.A;
    if (score >= 60) return ReadinessLevel.B;
    return ReadinessLevel.C;
  }

  static calculate(
    requirementText: string,
    dimensionScores: DimensionScore[],
    complexity: ComplexityLevel
  ): EvaluationResult {
    const weightedScore = this.calculateWeightedScore(dimensionScores);
    const totalScore = this.applyComplexityPenalty(weightedScore, complexity);
    const readinessLevel = this.determineReadinessLevel(totalScore);

    const recommendations = this.generateRecommendations(dimensionScores);

    const riskHeatmap = this.generateRiskHeatmap(requirementText, dimensionScores);

    return {
      requirementText,
      complexity,
      dimensionScores,
      totalScore,
      readinessLevel,
      recommendations,
      riskHeatmap,
      createdAt: new Date()
    };
  }

  private static generateRecommendations(dimensionScores: DimensionScore[]): Recommendation[] {
    const recommendations: Recommendation[] = [];

    for (const ds of dimensionScores) {
      if (ds.score < 60) {
        recommendations.push({
          dimension: ds.dimension,
          category: this.getCategory(ds.dimension),
          priority: 'high',
          title: this.getRecommendationTitle(ds.dimension),
          description: ds.analysis,
          example: this.getExample(ds.dimension)
        });
      } else if (ds.score < 80) {
        recommendations.push({
          dimension: ds.dimension,
          category: this.getCategory(ds.dimension),
          priority: 'medium',
          title: this.getRecommendationTitle(ds.dimension),
          description: ds.analysis
        });
      }
    }

    return recommendations.sort((a, b) => {
      const priorityOrder = { high: 0, medium: 1, low: 2 };
      return priorityOrder[a.priority] - priorityOrder[b.priority];
    });
  }

  private static getCategory(dimension: EvaluationDimension): Recommendation['category'] {
    const mapping: Record<EvaluationDimension, Recommendation['category']> = {
      [EvaluationDimension.ContextSufficiency]: 'context',
      [EvaluationDimension.Atomicity]: 'atomicity',
      [EvaluationDimension.BoundaryDefiniteness]: 'boundary',
      [EvaluationDimension.Verifiability]: 'verifiability',
      [EvaluationDimension.TechnicalConstraints]: 'technical'
    };
    return mapping[dimension];
  }

  private static getRecommendationTitle(dimension: EvaluationDimension): string {
    const titles: Record<EvaluationDimension, string> = {
      [EvaluationDimension.ContextSufficiency]: '增强上下文完备性',
      [EvaluationDimension.Atomicity]: '优化需求原子性',
      [EvaluationDimension.BoundaryDefiniteness]: '明确边界条件',
      [EvaluationDimension.Verifiability]: '提升可验证性',
      [EvaluationDimension.TechnicalConstraints]: '清晰技术约束'
    };
    return titles[dimension];
  }

  private static getExample(dimension: EvaluationDimension): string | undefined {
    const examples: Record<EvaluationDimension, string> = {
      [EvaluationDimension.ContextSufficiency]: '例如：补充用户旅程图，说明功能在整体流程中的位置',
      [EvaluationDimension.Atomicity]: '例如：建议拆分为两个独立需求：数据获取与展示逻辑分离',
      [EvaluationDimension.BoundaryDefiniteness]: '例如：补充边界测试用例：空输入、超大输入、特殊字符',
      [EvaluationDimension.Verifiability]: '例如：定义量化指标：响应时间<200ms，准确率>95%',
      [EvaluationDimension.TechnicalConstraints]: '例如：明确技术栈版本：Spring Boot 2.7，指定使用BCrypt加密'
    };
    return examples[dimension];
  }

  private static generateRiskHeatmap(
    requirementText: string,
    dimensionScores: DimensionScore[]
  ): RiskHighlight[] {
    const highlights: RiskHighlight[] = [];

    for (const ds of dimensionScores) {
      if (ds.score < 70 && ds.riskAreas.length > 0) {
        for (const risk of ds.riskAreas.slice(0, 2)) {
          const lowerText = requirementText.toLowerCase();
          const keywords = this.getKeywordsForDimension(ds.dimension);

          for (const keyword of keywords) {
            const index = lowerText.indexOf(keyword);
            if (index !== -1) {
              highlights.push({
                startIndex: index,
                endIndex: index + keyword.length,
                text: keyword,
                severity: ds.score < 50 ? 'high' : 'medium',
                relatedDimension: ds.dimension,
                reason: risk
              });
            }
          }
        }
      }
    }

    return highlights;
  }

  private static getKeywordsForDimension(dimension: EvaluationDimension): string[] {
    const keywords: Record<EvaluationDimension, string[]> = {
      [EvaluationDimension.ContextSufficiency]: ['业务', '用户', '需求', '功能', '模块'],
      [EvaluationDimension.Atomicity]: ['实现', '开发', '创建', '构建'],
      [EvaluationDimension.BoundaryDefiniteness]: ['如果', '当', '情况', '异常', '错误', '边界'],
      [EvaluationDimension.Verifiability]: ['测试', '验证', '检查', '确保', '正常'],
      [EvaluationDimension.TechnicalConstraints]: ['技术', '框架', '架构', '数据库', 'API']
    };
    return keywords[dimension];
  }
}
