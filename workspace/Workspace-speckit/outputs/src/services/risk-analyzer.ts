import { RiskDimension } from '../entities/risk-highlight';
import { riskHighlightRepository } from '../entities/risk-highlight';
import { optimizationSuggestionRepository } from '../entities/optimization-suggestion';
import { DimensionScores } from './scoring-engine';

export interface RiskAnalysisResult {
  highlights: Array<{ dimension: RiskDimension; textSpan: string; reason: string }>;
  suggestions: Array<{ category: 'context' | 'boundary' | 'atomicity' | 'verifiability'; description: string; priority: 'high' | 'medium' | 'low' }>;
}

const RISK_PATTERNS: Record<RiskDimension, { patterns: RegExp[]; reason: string }[]> = {
  context: [{ patterns: [/业务/i, /需求/i, /功能/i], reason: '业务背景未明确说明' }, { patterns: [/用户/i, /客户/i], reason: '用户角色和场景未定义' }],
  atomicity: [{ patterns: [/并且/i, /而且/i], reason: '需求包含多个功能点，可能过大' }],
  boundary: [{ patterns: [/空/i, /null/i], reason: '未定义空值处理' }, { patterns: [/错误/i, /异常/i], reason: '未说明异常处理逻辑' }],
  verifiability: [{ patterns: [/好/i, /高/i], reason: '验收标准未量化（好/高/快无具体数值）' }],
  technical: [{ patterns: [/最新/i, /最先进/i], reason: '技术约束过于模糊（未指定版本）' }],
};

const SUGGESTIONS: Record<RiskDimension, string[]> = {
  context: ['补充用户旅程图', '定义关键业务术语', '说明核心业务问题'],
  atomicity: ['拆分为多个独立需求', '明确前置条件', '确保描述在3000 tokens以内'],
  boundary: ['补充空值处理', '定义异常情况', '补充边界测试用例'],
  verifiability: ['提供量化指标', '补充输入输出示例', '定义自动化验收标准'],
  technical: ['明确技术栈版本', '指定API协议', '说明安全约束'],
};

export class RiskAnalyzer {
  analyze(text: string, scoreId: string, dimensionScores: DimensionScores): RiskAnalysisResult {
    const highlights: RiskAnalysisResult['highlights'] = [];
    const suggestions: RiskAnalysisResult['suggestions'] = [];

    for (const [dim, patterns] of Object.entries(RISK_PATTERNS) as [RiskDimension, typeof RISK_PATTERNS[RiskDimension]][]) {
      for (const { patterns: regexes, reason } of patterns) {
        for (const regex of regexes) {
          if (regex.test(text) && !highlights.find((h) => h.dimension === dim)) {
            const match = text.match(regex);
            highlights.push({ dimension: dim, textSpan: match?.[0]?.slice(0, 50) || '', reason });
            break;
          }
        }
      }
    }

    if (dimensionScores.context < 15) suggestions.push({ category: 'context', description: SUGGESTIONS.context[0], priority: dimensionScores.context < 10 ? 'high' : 'medium' });
    if (dimensionScores.atomicity < 15) suggestions.push({ category: 'atomicity', description: SUGGESTIONS.atomicity[0], priority: dimensionScores.atomicity < 10 ? 'high' : 'medium' });
    if (dimensionScores.boundary < 12) suggestions.push({ category: 'boundary', description: SUGGESTIONS.boundary[0], priority: dimensionScores.boundary < 8 ? 'high' : 'medium' });
    if (dimensionScores.verifiability < 10) suggestions.push({ category: 'verifiability', description: SUGGESTIONS.verifiability[0], priority: dimensionScores.verifiability < 6 ? 'high' : 'medium' });

    if (highlights.length === 0) highlights.push({ dimension: 'context', textSpan: 'N/A', reason: '未发现明显风险点' });
    if (suggestions.length === 0) suggestions.push({ category: 'context', description: '需求质量良好，可直接提交给AI实现', priority: 'low' });

    for (const h of highlights) riskHighlightRepository.create({ scoreId, dimension: h.dimension, textSpan: h.textSpan, reason: h.reason });
    optimizationSuggestionRepository.createBatch(suggestions.map((s) => ({ scoreId, category: s.category, description: s.description, priority: s.priority })));

    return { highlights, suggestions };
  }
}

export const riskAnalyzer = new RiskAnalyzer();