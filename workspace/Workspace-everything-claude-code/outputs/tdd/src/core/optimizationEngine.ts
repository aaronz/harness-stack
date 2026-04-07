import { EvaluationResult, OptimizationSuggestion, Dimension } from '../types';

export class OptimizationEngine {
  private static readonly THRESHOLD_HIGH = 70;
  private static readonly THRESHOLD_MEDIUM = 50;

  generateSuggestions(evaluation: EvaluationResult): OptimizationSuggestion[] {
    const suggestions: OptimizationSuggestion[] = [];
    const dimensions = ['context', 'atomicity', 'boundary', 'verifiability', 'technical'] as Dimension[];

    for (const dim of dimensions) {
      const score = evaluation.dimensions[dim].score;
      const priority = this.determinePriority(score);
      const categorySuggestions = this.getSuggestionsForDimension(dim, score);

      if (categorySuggestions.length > 0) {
        for (const suggestion of categorySuggestions) {
          suggestions.push({
            id: `suggestion_${dim}_${Date.now()}_${Math.random().toString(36).substring(2, 5)}`,
            evaluationId: evaluation.id,
            category: dim,
            suggestion,
            priority
          });
        }
      }
    }

    return suggestions;
  }

  private determinePriority(score: number): 'high' | 'medium' | 'low' {
    if (score < OptimizationEngine.THRESHOLD_MEDIUM) return 'high';
    if (score < OptimizationEngine.THRESHOLD_HIGH) return 'medium';
    return 'low';
  }

  private getSuggestionsForDimension(dimension: Dimension, score: number): string[] {
    const suggestions: Record<Dimension, string[]> = {
      context: [
        '补充业务目标和背景说明',
        '添加关键术语定义',
        '说明需求在整体系统中的位置'
      ],
      atomicity: [
        '将需求拆分为更小的独立功能',
        '减少功能耦合',
        '明确功能的唯一职责'
      ],
      boundary: [
        '明确定义输入输出边界',
        '补充异常流程处理',
        '说明与外部系统的交互边界'
      ],
      verifiability: [
        '添加具体的验收标准',
        '提供测试用例示例',
        '明确可度量的指标'
      ],
      technical: [
        '明确技术栈和版本要求',
        '补充性能指标要求',
        '说明安全合规要求'
      ]
    };

    if (score >= OptimizationEngine.THRESHOLD_HIGH) {
      return [];
    }

    return suggestions[dimension];
  }
}