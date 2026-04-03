export interface DimensionScores {
  context: number; atomicity: number; boundary: number; verifiability: number; technical: number;
}

const DIMENSION_WEIGHTS = { context: 0.25, atomicity: 0.25, boundary: 0.2, verifiability: 0.15, technical: 0.15 };
const COMPLEXITY_PENALTIES = { simple: 1.0, medium: 0.9, complex: 0.7 };

const CONTEXT_KEYWORDS = ['业务', '用户', '系统', '功能', '实现', '需求', '模块', '接口', '数据'];
const ATOMICITY_KEYWORDS = ['单一', '独立', '单个', '一个', '具体', '明确'];
const BOUNDARY_KEYWORDS = ['如果', '否则', '当', '情况', '异常', '错误', '边界', '空', 'null', 'undefined'];
const VERIFIABILITY_KEYWORDS = ['测试', '验证', '检查', '输入', '输出', '示例', '例子', '标准'];
const TECHNICAL_KEYWORDS = ['技术', '框架', '语言', '版本', 'API', '接口', '协议', '安全', '认证'];

function countMatches(text: string, keywords: string[]): number {
  let count = 0;
  for (const kw of keywords) {
    const matches = text.toLowerCase().match(new RegExp(kw, 'gi'));
    if (matches) count += matches.length;
  }
  return count;
}

function normalize(matches: number, max: number, scoreMax: number): number {
  return Math.round((Math.min(matches / max, 1) * scoreMax) * 10) / 10;
}

export class ScoringEngine {
  calculateScores(text: string): DimensionScores {
    return {
      context: normalize(countMatches(text, CONTEXT_KEYWORDS), 10, 25),
      atomicity: normalize(countMatches(text, ATOMICITY_KEYWORDS), 8, 25),
      boundary: normalize(countMatches(text, BOUNDARY_KEYWORDS), 10, 20),
      verifiability: normalize(countMatches(text, VERIFIABILITY_KEYWORDS), 8, 15),
      technical: normalize(countMatches(text, TECHNICAL_KEYWORDS), 8, 15),
    };
  }

  detectComplexity(text: string): number {
    const hasComplex = /算法|架构|设计模式|并发|分布式|微服务|大数据|机器学习/i.test(text);
    const hasMedium = /业务|逻辑|流程|状态|事务/i.test(text);
    const hasSimple = /增删改查|CRUD|列表|详情|新建|编辑|删除/i.test(text);
    if (hasComplex || text.length > 3000) return COMPLEXITY_PENALTIES.complex;
    if (hasMedium || text.length > 500) return COMPLEXITY_PENALTIES.medium;
    return COMPLEXITY_PENALTIES.simple;
  }

  applyWeights(dimensions: DimensionScores, penalty: number): number {
    const weighted = dimensions.context * DIMENSION_WEIGHTS.context + dimensions.atomicity * DIMENSION_WEIGHTS.atomicity + dimensions.boundary * DIMENSION_WEIGHTS.boundary + dimensions.verifiability * DIMENSION_WEIGHTS.verifiability + dimensions.technical * DIMENSION_WEIGHTS.technical;
    const total = 25 * 0.25 + 25 * 0.25 + 20 * 0.2 + 15 * 0.15 + 15 * 0.15;
    return Math.round((weighted / total) * 100 * penalty * 10) / 10;
  }
}

export const scoringEngine = new ScoringEngine();