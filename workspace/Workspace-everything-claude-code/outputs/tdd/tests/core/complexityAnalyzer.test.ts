import { describe, it, expect } from 'vitest';
import { ComplexityAnalyzer, ComplexityLevel } from '../../src/core/complexityAnalyzer';

describe('ComplexityAnalyzer', () => {
  describe('analyze', () => {
    it('should return simple for basic requirements', () => {
      const analyzer = new ComplexityAnalyzer();
      
      const result = analyzer.analyze('创建用户登录功能，包含用户名和密码验证');
      
      expect(result.level).toBe('simple');
      expect(result.reasoning).toContain('简单需求');
    });

    it('should return medium for requirements with multiple features', () => {
      const analyzer = new ComplexityAnalyzer();
      
      const result = analyzer.analyze(
        '创建用户管理系统，包含用户的增删改查功能，还需要角色权限控制，' +
        '支持批量导入导出，需要与第三方系统对接API'
      );
      
      expect(result.level).toBe('medium');
    });

    it('should return complex for requirements with many integrations', () => {
      const analyzer = new ComplexityAnalyzer();
      
      const result = analyzer.analyze(
        '构建分布式微服务系统，包含用户服务、订单服务、支付服务、库存服务、物流服务等多个服务，' +
        '服务之间通过消息队列通信，需要支持高并发、灰度发布、熔断降级、链路追踪、' +
        '数据分片、读写分离、缓存策略、分布式事务等功能'
      );
      
      expect(result.level).toBe('complex');
    });
  });

  describe('calculateFeaturePoints', () => {
    it('should count CRUD keywords correctly', () => {
      const analyzer = new ComplexityAnalyzer();
      
      expect(analyzer['calculateFeaturePoints']('创建用户、查询用户、修改用户、删除用户')).toBeGreaterThan(0);
    });

    it('should count integration keywords correctly', () => {
      const analyzer = new ComplexityAnalyzer();
      
      const points = analyzer['calculateFeaturePoints']('与第三方API对接、集成微信支付、集成支付宝');
      expect(points).toBeGreaterThan(0);
    });

    it('should count algorithm keywords correctly', () => {
      const analyzer = new ComplexityAnalyzer();
      
      const points = analyzer['calculateFeaturePoints']('使用机器学习算法进行推荐、使用排序算法优化性能');
      expect(points).toBeGreaterThan(0);
    });
  });
});