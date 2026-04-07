import { describe, it, expect } from 'vitest';
import { ReportGenerator } from '../../src/core/reportGenerator';
import { EvaluationResult, OptimizationSuggestion } from '../../src/types';

describe('ReportGenerator', () => {
  describe('generateReport', () => {
    it('should include all required sections', () => {
      const generator = new ReportGenerator();
      
      const mockEvaluation = createMockEvaluation();
      const mockSuggestions: OptimizationSuggestion[] = [];

      const report = generator.generateReport(mockEvaluation, mockSuggestions);

      expect(report.summary).toBeDefined();
      expect(report.dimensions).toBeDefined();
      expect(report.radarData).toBeDefined();
      expect(report.suggestions).toEqual(mockSuggestions);
    });

    it('should calculate correct radar data', () => {
      const generator = new ReportGenerator();
      
      const mockEvaluation = createMockEvaluation();
      const mockSuggestions: OptimizationSuggestion[] = [];

      const report = generator.generateReport(mockEvaluation, mockSuggestions);

      expect(report.radarData).toHaveLength(5);
      expect(report.radarData.map(d => d.axis)).toEqual(['上下文完备性', '逻辑原子性', '边界明确性', '可验证性', '技术约束']);
    });

    it('should include grade and score in summary', () => {
      const generator = new ReportGenerator();
      
      const mockEvaluation = createMockEvaluation();
      const mockSuggestions: OptimizationSuggestion[] = [];

      const report = generator.generateReport(mockEvaluation, mockSuggestions);

      expect(report.summary.finalScore).toBe(80);
      expect(report.summary.grade).toBe('A');
      expect(report.summary.complexity).toBe('medium');
    });

    it('should include complexity info in summary', () => {
      const generator = new ReportGenerator();
      
      const mockEvaluation = createMockEvaluation({
        ...createMockEvaluation(),
        complexity: 'complex',
        complexityPenalty: 0.7
      });
      const mockSuggestions: OptimizationSuggestion[] = [];

      const report = generator.generateReport(mockEvaluation, mockSuggestions);

      expect(report.summary.complexityPenalty).toBe(0.7);
    });

    it('should sort suggestions by priority', () => {
      const generator = new ReportGenerator();
      
      const mockEvaluation = createMockEvaluation();
      const mockSuggestions: OptimizationSuggestion[] = [
        { id: '1', evaluationId: 'eval_1', category: 'context', suggestion: 'medium', priority: 'medium' },
        { id: '2', evaluationId: 'eval_1', category: 'boundary', suggestion: 'high', priority: 'high' },
        { id: '3', evaluationId: 'eval_1', category: 'technical', suggestion: 'low', priority: 'low' }
      ];

      const report = generator.generateReport(mockEvaluation, mockSuggestions);

      expect(report.suggestions[0].priority).toBe('high');
      expect(report.suggestions[1].priority).toBe('medium');
      expect(report.suggestions[2].priority).toBe('low');
    });
  });

  describe('toJSON', () => {
    it('should serialize report to JSON', () => {
      const generator = new ReportGenerator();
      
      const mockEvaluation = createMockEvaluation();
      const mockSuggestions: OptimizationSuggestion[] = [];

      const report = generator.generateReport(mockEvaluation, mockSuggestions);
      const json = generator.toJSON(report);

      expect(() => JSON.parse(json)).not.toThrow();
    });
  });
});

function createMockEvaluation(overrides?: Partial<EvaluationResult>): EvaluationResult {
  return {
    id: 'eval_123',
    requirementId: 'req_456',
    dimensions: {
      context: { dimension: 'context', score: 85, analysis: 'test', riskPoints: [] },
      atomicity: { dimension: 'atomicity', score: 90, analysis: 'test', riskPoints: [] },
      boundary: { dimension: 'boundary', score: 75, analysis: 'test', riskPoints: [] },
      verifiability: { dimension: 'verifiability', score: 80, analysis: 'test', riskPoints: [] },
      technical: { dimension: 'technical', score: 88, analysis: 'test', riskPoints: [] }
    },
    finalScore: 80,
    grade: 'A',
    complexityPenalty: 0.9,
    complexity: 'medium',
    llmProvider: 'openai',
    llmModel: 'gpt-4o',
    analysisDetail: '{}',
    createdAt: new Date(),
    ...overrides
  } as EvaluationResult;
}