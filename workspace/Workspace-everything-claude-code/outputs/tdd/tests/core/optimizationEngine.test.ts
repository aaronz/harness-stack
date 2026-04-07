import { describe, it, expect } from 'vitest';
import { OptimizationEngine, OptimizationSuggestion } from '../../src/core/optimizationEngine';
import { EvaluationResult, DimensionScore } from '../../src/types';

describe('OptimizationEngine', () => {
  describe('generateSuggestions', () => {
    it('should generate suggestions for low context score', () => {
      const engine = new OptimizationEngine();
      
      const mockEvaluation = createMockEvaluation({
        context: 50,
        atomicity: 80,
        boundary: 75,
        verifiability: 80,
        technical: 85
      });

      const suggestions = engine.generateSuggestions(mockEvaluation);

      const contextSuggestions = suggestions.filter(s => s.category === 'context');
      expect(contextSuggestions.length).toBeGreaterThan(0);
    });

    it('should generate suggestions for low boundary score', () => {
      const engine = new OptimizationEngine();
      
      const mockEvaluation = createMockEvaluation({
        context: 85,
        atomicity: 90,
        boundary: 45,
        verifiability: 80,
        technical: 88
      });

      const suggestions = engine.generateSuggestions(mockEvaluation);

      const boundarySuggestions = suggestions.filter(s => s.category === 'boundary');
      expect(boundarySuggestions.length).toBeGreaterThan(0);
    });

    it('should generate suggestions for low atomicity score', () => {
      const engine = new OptimizationEngine();
      
      const mockEvaluation = createMockEvaluation({
        context: 80,
        atomicity: 40,
        boundary: 75,
        verifiability: 85,
        technical: 90
      });

      const suggestions = engine.generateSuggestions(mockEvaluation);

      const atomicitySuggestions = suggestions.filter(s => s.category === 'atomicity');
      expect(atomicitySuggestions.length).toBeGreaterThan(0);
    });

    it('should assign high priority to low scores', () => {
      const engine = new OptimizationEngine();
      
      const mockEvaluation = createMockEvaluation({
        context: 30,
        atomicity: 90,
        boundary: 85,
        verifiability: 80,
        technical: 88
      });

      const suggestions = engine.generateSuggestions(mockEvaluation);
      const contextSuggestions = suggestions.filter(s => s.category === 'context');

      expect(contextSuggestions.some(s => s.priority === 'high')).toBe(true);
    });

    it('should return empty array when all scores are high', () => {
      const engine = new OptimizationEngine();
      
      const mockEvaluation = createMockEvaluation({
        context: 90,
        atomicity: 90,
        boundary: 85,
        verifiability: 85,
        technical: 90
      });

      const suggestions = engine.generateSuggestions(mockEvaluation);

      expect(suggestions).toHaveLength(0);
    });
  });
});

function createMockEvaluation(dimensions: {
  context: number;
  atomicity: number;
  boundary: number;
  verifiability: number;
  technical: number;
}): EvaluationResult {
  return {
    id: 'eval_123',
    requirementId: 'req_456',
    dimensions: {
      context: { dimension: 'context', score: dimensions.context, analysis: '', riskPoints: [] },
      atomicity: { dimension: 'atomicity', score: dimensions.atomicity, analysis: '', riskPoints: [] },
      boundary: { dimension: 'boundary', score: dimensions.boundary, analysis: '', riskPoints: [] },
      verifiability: { dimension: 'verifiability', score: dimensions.verifiability, analysis: '', riskPoints: [] },
      technical: { dimension: 'technical', score: dimensions.technical, analysis: '', riskPoints: [] }
    },
    finalScore: 80,
    grade: 'A',
    complexityPenalty: 0.9,
    complexity: 'medium',
    llmProvider: 'openai',
    llmModel: 'gpt-4o',
    analysisDetail: '{}',
    createdAt: new Date()
  };
}