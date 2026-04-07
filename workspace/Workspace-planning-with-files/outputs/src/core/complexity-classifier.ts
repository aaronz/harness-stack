import { ComplexityLevel } from '../types/evaluation';

/**
 * 复杂度等级分类器
 * 用于自动判断需求复杂度：CRUD类（简单）、业务逻辑类（中等）、算法/架构类（复杂）
 */
export class ComplexityClassifier {
  // 简单需求关键词
  private static readonly SIMPLE_KEYWORDS = [
    '增删改查', 'crud', 'create', 'read', 'update', 'delete',
    '列表', '查询', '添加', '修改', '删除', '展示',
    '表单', '表格', '导入', '导出', 'basic', 'simple'
  ];

  // 中等复杂度关键词
  private static readonly MEDIUM_KEYWORDS = [
    '业务', '逻辑', '流程', '审批', '状态', '权限',
    '计算', '统计', '报表', '通知', '消息', '定时',
    '规则', '校验', '转换', '同步', '集成', 'workflow'
  ];

  // 复杂需求关键词
  private static readonly COMPLEX_KEYWORDS = [
    '算法', '架构', '设计', '模式', '优化', '性能',
    '并发', '分布式', '微服务', '消息队列', '缓存',
    '机器学习', 'ai', '人工智能', '大数据', '实时',
    '高并发', '高可用', '系统', '平台', '引擎'
  ];

  /**
   * 分析文本复杂度
   */
  static classify(requirementText: string): ComplexityLevel {
    const lowerText = requirementText.toLowerCase();

    const simpleScore = this.calculateScore(lowerText, this.SIMPLE_KEYWORDS);
    const mediumScore = this.calculateScore(lowerText, this.MEDIUM_KEYWORDS);
    const complexScore = this.calculateScore(lowerText, this.COMPLEX_KEYWORDS);

    // 长度惩罚 - 过长文本倾向于更复杂
    const lengthPenalty = this.getLengthPenalty(requirementText.length);

    const adjustedMediumScore = mediumScore * (1 - lengthPenalty * 0.3);
    const adjustedComplexScore = complexScore * (1 - lengthPenalty * 0.5);

    // 确定最高分数的类别
    if (adjustedComplexScore > 2 && adjustedComplexScore >= Math.max(simpleScore, adjustedMediumScore)) {
      return ComplexityLevel.Complex;
    }

    if (adjustedMediumScore > 1.5 && adjustedMediumScore >= simpleScore) {
      return ComplexityLevel.Medium;
    }

    return ComplexityLevel.Simple;
  }

  /**
   * 计算关键词匹配分数
   */
  private static calculateScore(text: string, keywords: string[]): number {
    let score = 0;
    for (const keyword of keywords) {
      if (text.includes(keyword)) {
        score += 1;
      }
    }
    return score;
  }

  /**
   * 获取长度惩罚系数
   */
  private static getLengthPenalty(length: number): number {
    if (length < 500) return 0;
    if (length < 1000) return 0.2;
    if (length < 2000) return 0.4;
    return 0.6;
  }

  /**
   * 获取复杂度描述
   */
  static getComplexityDescription(level: ComplexityLevel): string {
    const descriptions: Record<ComplexityLevel, string> = {
      [ComplexityLevel.Simple]: 'CRUD类需求，功能单一，AI实现成功率高',
      [ComplexityLevel.Medium]: '业务逻辑类需求，需要一定的上下文理解',
      [ComplexityLevel.Complex]: '算法/架构类需求，AI实现难度较高'
    };
    return descriptions[level];
  }

  /**
   * 获取惩罚系数
   */
  static getPenaltyFactor(level: ComplexityLevel): number {
    const factors: Record<ComplexityLevel, number> = {
      [ComplexityLevel.Simple]: 1.0,
      [ComplexityLevel.Medium]: 0.9,
      [ComplexityLevel.Complex]: 0.7
    };
    return factors[level];
  }
}
