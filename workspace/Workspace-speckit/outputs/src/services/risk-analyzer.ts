import { RiskDimension, riskHighlightRepository } from '../entities/risk-highlight';
import { OptimizationSuggestion, optimizationSuggestionRepository } from '../entities/optimization-suggestion';
import { DimensionScores } from '../services/scoring-engine';

export interface RiskAnalysisResult {
  highlights: Array<{
    dimension: RiskDimension;
    textSpan: string;
    reason: string;
  }>;
  suggestions: Array<{
    category: 'context' | 'boundary' | 'atomicity' | 'verifiability';
    description: string;
    priority: 'high' | 'medium' | 'low';
  }>;
}

const RISK_PATTERNS: Record<RiskDimension, { patterns: RegExp[]; reason: string }[]> = {
  context: [
    { patterns: [/业务/i, /需求/i, /功能/i], reason: '业务背景未明确说明' },
    { patterns: [/用户/i, /客户/i], reason: '用户角色和场景未定义' },
    { patterns: [/数据/i, /数据库/i], reason: '数据来源和结构未说明' },
  ],
  atomicity: [
    { patterns: [/并且/i, /而且/i], reason: '需求包含多个功能点，可能过大' },
    { patterns: [/\n.+\n.+\n/], reason: '需求描述过长，可能超出AI上下文窗口' },
  ],
  boundary: [
    { patterns: [/空/i, /null/i, /undefined/i], reason: '未定义空值处理' },
    { patterns: [/错误/i, /异常/i], reason: '未说明异常处理逻辑' },
    { patterns: [/如果/i], reason: '条件分支未完整列举' },
  ],
  verifiability: [
    { patterns: [/好/i, /高/i, /快/i], reason: '验收标准未量化（好/高/快无具体数值）' },
    { patterns: [/正常/i], reason: '未说明正常工作的具体标准' },
    { patterns: [/测试/i], reason: '未提供测试用例或验证方式' },
  ],
  technical: [
    { patterns: [/最新/i, /最先进/i], reason: '技术约束过于模糊（未指定版本）' },
    { patterns: [/简单/i, /容易/i], reason: '技术实现方式未明确' },
  ],
};

const SUGGESTION_TEMPLATES: Record<RiskDimension, string[]> = {
  context: [
    '补充用户旅程图，说明此功能在整体流程中的位置',
    '定义关键业务术语的含义',
    '说明该需求要解决的核心业务问题',
  ],
  atomicity: [
    '建议拆分为多个独立需求',
    '明确前置条件和依赖关系',
    '确保需求描述在3000 tokens以内',
  ],
  boundary: [
    '补充空值处理逻辑',
    '定义所有异常情况',
    '补充边界值测试用例',
  ],
  verifiability: [
    '提供具体量化指标（如响应时间<200ms）',
    '补充输入/输出示例',
    '定义可自动化测试的验收标准',
  ],
  technical: [
    '明确技术栈版本',
    '指定API协议和数据格式',
    '说明安全约束和认证方式',
  ],
};

export class RiskAnalyzer {
  analyze(text: string, scoreId: string, dimensionScores: DimensionScores): RiskAnalysisResult {
    const highlights: RiskAnalysisResult['highlights'] = [];
    const suggestions: RiskAnalysisResult['suggestions'] = [];

    for (const [dimension, patterns] of Object.entries(RISK_PATTERNS) as [RiskDimension, typeof RISK_PATTERNS[RiskDimension]][]) {
      for (const { patterns: regexes, reason } of patterns) {
        for (const regex of regexes) {
          const match = text.match(regex);
          if (match) {
            const textSpan = match[0];
            const existingHighlight = highlights.find((h) => h.textSpan === textSpan);
            if (!existingHighlight) {
              highlights.push({
                dimension,
                textSpan: textSpan.length > 50 ? textSpan.substring(0, 50) + '...' : textSpan,
                reason,
              });
              break;
            }
          }
        }
      }
    }

    if (dimensionScores.context < 15) {
      suggestions.push({
        category: 'context',
        description: SUGGESTION_TEMPLATES.context[Math.floor(Math.random() * SUGGESTION_TEMPLATES.context.length)],
        priority: dimensionScores.context < 10 ? 'high' : 'medium',
      });
    }
    if (dimensionScores.atomicity < 15) {
      suggestions.push({
        category: 'atomicity',
        description: SUGGESTION_TEMPLATES.atomicity[Math.floor(Math.random() * SUGGESTION_TEMPLATES.atomicity.length)],
        priority: dimensionScores.atomicity < 10 ? 'high' : 'medium',
      });
    }
    if (dimensionScores.boundary < 12) {
      suggestions.push({
        category: 'boundary',
        description: SUGGESTION_TEMPLATES.boundary[Math.floor(Math.random() * SUGGESTION_TEMPLATES.boundary.length)],
        priority: dimensionScores.boundary < 8 ? 'high' : 'medium',
      });
    }
    if (dimensionScores.verifiability < 10) {
      suggestions.push({
        category: 'verifiability',
        description: SUGGESTION_TEMPLATES.verifiability[Math.floor(Math.random() * SUGGESTION_TEMPLATES.verifiability.length)],
        priority: dimensionScores.verifiability < 6 ? 'high' : 'medium',
      });
    }

    if (highlights.length === 0) {
      highlights.push({
        dimension: 'context' as RiskDimension,
        textSpan: 'N/A',
        reason: '未发现明显风险点',
      });
    }

    if (suggestions.length === 0) {
      suggestions.push({
        category: 'context',
        description: '需求质量良好，可直接提交给AI实现',
        priority: 'low',
      });
    }

    for (const h of highlights) {
      riskHighlightRepository.create({
        scoreId,
        dimension: h.dimension,
        textSpan: h.textSpan,
        reason: h.reason,
      });
    }

    optimizationSuggestionRepository.createBatch(
      suggestions.map((s) => ({
        scoreId,
        category: s.category,
        description: s.description,
        priority: s.priority,
      }))
    );

    return { highlights, suggestions };
  }
}

export const riskAnalyzer = new RiskAnalyzer();
