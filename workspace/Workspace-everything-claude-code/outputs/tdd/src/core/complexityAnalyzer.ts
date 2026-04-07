import { ComplexityLevel } from '../types';

interface AnalysisResult {
  level: ComplexityLevel;
  featurePoints: number;
  reasoning: string;
}

export class ComplexityAnalyzer {
  private static readonly CRUD_KEYWORDS = ['创建', '新增', '查询', '获取', '修改', '更新', '删除', '导出', '导入', '列表', '详情'];
  private static readonly INTEGRATION_KEYWORDS = ['对接', '集成', '接口', 'API', '第三方', '外部', '同步', '回调', 'webhook', '集成'];
  private static readonly ALGORITHM_KEYWORDS = ['算法', '机器学习', 'AI', '智能', '推荐', '预测', '优化', '高性能', '并发', '分布式'];
  private static readonly COMPLEXITY_KEYWORDS = ['微服务', '多服务', '多个服务', '分布式', '高并发', '大数据', '实时', '异步', '消息队列'];

  analyze(requirement: string): AnalysisResult {
    const featurePoints = this.calculateFeaturePoints(requirement);
    const level = this.determineLevel(featurePoints);
    const reasoning = this.generateReasoning(level, featurePoints);

    return { level, featurePoints, reasoning };
  }

  calculateFeaturePoints(requirement: string): number {
    let points = 0;
    const text = requirement.toLowerCase();

    points += this.countKeywordMatches(text, ComplexityAnalyzer.CRUD_KEYWORDS) * 1;
    points += this.countKeywordMatches(text, ComplexityAnalyzer.INTEGRATION_KEYWORDS) * 2;
    points += this.countKeywordMatches(text, ComplexityAnalyzer.ALGORITHM_KEYWORDS) * 3;
    points += this.countKeywordMatches(text, ComplexityAnalyzer.COMPLEXITY_KEYWORDS) * 4;

    const length = requirement.length;
    if (length > 500) points += 3;
    else if (length > 200) points += 1;

    return points;
  }

  private countKeywordMatches(text: string, keywords: string[]): number {
    let count = 0;
    for (const keyword of keywords) {
      if (text.includes(keyword)) {
        count++;
      }
    }
    return count;
  }

  private determineLevel(featurePoints: number): ComplexityLevel {
    if (featurePoints >= 8) return 'complex';
    if (featurePoints >= 4) return 'medium';
    return 'simple';
  }

  private generateReasoning(level: ComplexityLevel, featurePoints: number): string {
    switch (level) {
      case 'simple':
        return `简单需求 (功能点: ${featurePoints})`;
      case 'medium':
        return `中等复杂度需求 (功能点: ${featurePoints})`;
      case 'complex':
        return `复杂需求，包含多个系统集成或算法要求 (功能点: ${featurePoints})`;
    }
  }
}