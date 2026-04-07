import { describe, it, expect, vi, beforeEach } from 'vitest';
import { EvaluationEngine } from '../../src/core/evaluationEngine';
import { LLMConfig, EvaluationResult, ComplexityLevel } from '../../src/types';

describe('EvaluationEngine', () => {
  let engine: EvaluationEngine;
  const mockLLMConfig: LLMConfig = {
    id: 'config_1',
    provider: 'openai',
    model: 'gpt-4o',
    apiKey: 'test-key',
    isDefault: true
  };

  beforeEach(() => {
    vi.clearAllMocks();
    engine = new EvaluationEngine(mockLLMConfig);
  });

  describe('evaluate', () => {
    it('should return evaluation result with all dimensions', async () => {
      const result = await engine.evaluate({
        requirementId: 'req_123',
        content: '创建用户登录功能'
      });

      expect(result.requirementId).toBe('req_123');
      expect(result.dimensions).toBeDefined();
      expect(result.finalScore).toBeGreaterThanOrEqual(0);
      expect(result.finalScore).toBeLessThanOrEqual(100);
      expect(['S', 'A', 'B', 'C']).toContain(result.grade);
    });

    it('should include complexity analysis', async () => {
      const result = await engine.evaluate({
        requirementId: 'req_456',
        content: '构建分布式微服务系统需要高并发处理'
      });

      expect(result.complexity).toBeDefined();
      expect(['simple', 'medium', 'complex']).toContain(result.complexity);
    });

    it('should track LLM provider info', async () => {
      const result = await engine.evaluate({
        requirementId: 'req_789',
        content: '测试需求'
      });

      expect(result.llmProvider).toBe('openai');
      expect(result.llmModel).toBe('gpt-4o');
    });
  });

  describe('calculateDimensions', () => {
    it('should extract numeric scores from LLM response', async () => {
      const mockDimensionScores = {
        context: { dimension: 'context' as const, score: 85, analysis: 'test', riskPoints: [] },
        atomicity: { dimension: 'atomicity' as const, score: 90, analysis: 'test', riskPoints: [] },
        boundary: { dimension: 'boundary' as const, score: 75, analysis: 'test', riskPoints: [] },
        verifiability: { dimension: 'verifiability' as const, score: 80, analysis: 'test', riskPoints: [] },
        technical: { dimension: 'technical' as const, score: 88, analysis: 'test', riskPoints: [] }
      };

      const dimensionNumbers = engine['calculateDimensions'](mockDimensionScores);
      
      expect(dimensionNumbers.context).toBe(85);
      expect(dimensionNumbers.atomicity).toBe(90);
      expect(dimensionNumbers.boundary).toBe(75);
      expect(dimensionNumbers.verifiability).toBe(80);
      expect(dimensionNumbers.technical).toBe(88);
    });
  });
});