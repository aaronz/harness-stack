export interface DimensionScores {
  context: number;
  atomicity: number;
  boundary: number;
  verifiability: number;
  technical: number;
}

export interface ScoringResult extends DimensionScores {
  totalScore: number;
  complexityPenalty: number;
}

const DIMENSION_WEIGHTS = {
  context: 0.25,
  atomicity: 0.25,
  boundary: 0.2,
  verifiability: 0.15,
  technical: 0.15,
};

const COMPLEXITY_PENALTIES = {
  simple: 1.0,
  medium: 0.9,
  complex: 0.7,
};

type ComplexityType = 'simple' | 'medium' | 'complex';

const CONTEXT_KEYWORDS = ['业务', '用户', '系统', '功能', '实现', '需求', '模块', '接口', '数据'];
const ATOMICITY_KEYWORDS = ['单一', '独立', '单个', '一个', '具体', '明确'];
const BOUNDARY_KEYWORDS = ['如果', '否则', '当', '情况', '异常', '错误', '边界', '空', 'null', 'undefined'];
const VERIFIABILITY_KEYWORDS = ['测试', '验证', '检查', '输入', '输出', '示例', '例子', '标准'];
const TECHNICAL_KEYWORDS = ['技术', '框架', '语言', '版本', 'API', '接口', '协议', '安全', '认证'];

function countKeywordMatches(text: string, keywords: string[]): number {
  let count = 0;
  const lowerText = text.toLowerCase();
  for (const keyword of keywords) {
    const regex = new RegExp(keyword, 'gi');
    const matches = lowerText.match(regex);
    if (matches) count += matches.length;
  }
  return count;
}

function normalizeScore(matches: number, maxMatches: number, maxScore: number = 25): number {
  if (maxMatches === 0) return 0;
  const ratio = Math.min(matches / maxMatches, 1);
  return Math.round(ratio * maxScore * 10) / 10;
}

export class ScoringEngine {
  calculateScores(text: string): DimensionScores {
    const contextMatches = countKeywordMatches(text, CONTEXT_KEYWORDS);
    const atomicityMatches = countKeywordMatches(text, ATOMICITY_KEYWORDS);
    const boundaryMatches = countKeywordMatches(text, BOUNDARY_KEYWORDS);
    const verifiabilityMatches = countKeywordMatches(text, VERIFIABILITY_KEYWORDS);
    const technicalMatches = countKeywordMatches(text, TECHNICAL_KEYWORDS);

    return {
      context: normalizeScore(contextMatches, 10, 25),
      atomicity: normalizeScore(atomicityMatches, 8, 25),
      boundary: normalizeScore(boundaryMatches, 10, 20),
      verifiability: normalizeScore(verifiabilityMatches, 8, 15),
      technical: normalizeScore(technicalMatches, 8, 15),
    };
  }

  detectComplexity(text: string): number {
    const complexityType = this.classifyComplexity(text);
    return COMPLEXITY_PENALTIES[complexityType];
  }

  private classifyComplexity(text: string): ComplexityType {
    const length = text.length;
    const hasComplexPatterns = /算法|架构|设计模式|并发|分布式|微服务|大数据|机器学习/i.test(text);
    const hasMediumPatterns = /业务|逻辑|流程|状态|事务/i.test(text);
    const hasSimplePatterns = /增删改查|CRUD|列表|详情|新建|编辑|删除/i.test(text);

    if (hasComplexPatterns || length > 3000) return 'complex';
    if (hasMediumPatterns || length > 500) return 'medium';
    if (hasSimplePatterns && length < 500) return 'simple';

    return hasMediumPatterns ? 'medium' : 'simple';
  }

  applyWeights(dimensions: DimensionScores, complexityPenalty: number): number {
    const weightedScore =
      dimensions.context * DIMENSION_WEIGHTS.context +
      dimensions.atomicity * DIMENSION_WEIGHTS.atomicity +
      dimensions.boundary * DIMENSION_WEIGHTS.boundary +
      dimensions.verifiability * DIMENSION_WEIGHTS.verifiability +
      dimensions.technical * DIMENSION_WEIGHTS.technical;

    const totalPossible =
      25 * DIMENSION_WEIGHTS.context +
      25 * DIMENSION_WEIGHTS.atomicity +
      20 * DIMENSION_WEIGHTS.boundary +
      15 * DIMENSION_WEIGHTS.verifiability +
      15 * DIMENSION_WEIGHTS.technical;

    const normalizedScore = (weightedScore / totalPossible) * 100;
    const finalScore = normalizedScore * complexityPenalty;

    return Math.round(finalScore * 10) / 10;
  }

  getDimensionWeights(): typeof DIMENSION_WEIGHTS {
    return { ...DIMENSION_WEIGHTS };
  }

  getComplexityPenalties(): typeof COMPLEXITY_PENALTIES {
    return { ...COMPLEXITY_PENALTIES };
  }
}

export const scoringEngine = new ScoringEngine();
