import { describe, it, expect, vi, beforeEach } from 'vitest';
import { LLMGateway, LLMProvider } from '../../src/core/llmGateway';
import { LLMConfig, DimensionScore } from '../../src/types';

describe('LLMGateway', () => {
  let gateway: LLMGateway;
  const mockConfig: LLMConfig = {
    id: 'config_1',
    provider: 'openai',
    model: 'gpt-4o',
    apiKey: 'test-key',
    isDefault: true
  };

  beforeEach(() => {
    vi.clearAllMocks();
    gateway = new LLMGateway(mockConfig);
  });

  describe('evaluateDimensions', () => {
    it('should return dimension scores from LLM response', async () => {
      const mockResponse = {
        context: { score: 85, analysis: '业务目标清晰', riskPoints: [] },
        atomicity: { score: 90, analysis: '功能单一', riskPoints: [] },
        boundary: { score: 75, analysis: '边界清晰', riskPoints: ['缺少异常处理'] },
        verifiability: { score: 80, analysis: '验收标准明确', riskPoints: [] },
        technical: { score: 88, analysis: '技术栈明确', riskPoints: [] }
      };

      const mockLLMClient = {
        generate: vi.fn().mockResolvedValue(JSON.stringify(mockResponse))
      };

      gateway['client'] = mockLLMClient as any;

      const result = await gateway.evaluateDimensions('用户登录功能需求');

      expect(result.context.score).toBe(85);
      expect(result.atomicity.score).toBe(90);
      expect(result.boundary.score).toBe(75);
      expect(result.verifiability.score).toBe(80);
      expect(result.technical.score).toBe(88);
    });

    it('should handle JSON parse errors gracefully', async () => {
      const mockLLMClient = {
        generate: vi.fn().mockResolvedValue('invalid json')
      };

      gateway['client'] = mockLLMClient as any;

      await expect(gateway.evaluateDimensions('test')).rejects.toThrow();
    });
  });

  describe('buildPrompt', () => {
    it('should include all five dimensions in prompt', () => {
      const prompt = gateway['buildPrompt']('测试需求');

      expect(prompt).toContain('上下文完备性');
      expect(prompt).toContain('逻辑原子性');
      expect(prompt).toContain('边界明确性');
      expect(prompt).toContain('可验证性');
      expect(prompt).toContain('技术约束');
    });
  });

  describe('parseResponse', () => {
    it('should parse valid JSON response', () => {
      const response = JSON.stringify({
        context: { score: 85, analysis: 'test', riskPoints: [] },
        atomicity: { score: 90, analysis: 'test', riskPoints: [] },
        boundary: { score: 75, analysis: 'test', riskPoints: [] },
        verifiability: { score: 80, analysis: 'test', riskPoints: [] },
        technical: { score: 88, analysis: 'test', riskPoints: [] }
      });

      const result = gateway['parseResponse'](response);

      expect(result.context.score).toBe(85);
    });

    it('should throw on invalid JSON', () => {
      expect(() => gateway['parseResponse']('invalid')).toThrow();
    });
  });

  describe('validateScore', () => {
    it('should accept valid scores', () => {
      expect(gateway['validateScore'](0)).toBe(true);
      expect(gateway['validateScore'](50)).toBe(true);
      expect(gateway['validateScore'](100)).toBe(true);
    });

    it('should reject invalid scores', () => {
      expect(gateway['validateScore'](-1)).toBe(false);
      expect(gateway['validateScore'](101)).toBe(false);
    });
  });
});