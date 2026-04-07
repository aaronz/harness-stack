import { describe, it, expect, beforeEach } from 'vitest';
import { calculateFinalScore, calculateGrade, calculateWeightedSum, ScoreCalculator } from '../../src/core/scoreCalculator';
import { DEFAULT_WEIGHTS, COMPLEXITY_PENALTIES } from '../../src/types';

describe('ScoreCalculator', () => {
  describe('calculateWeightedSum', () => {
    it('should calculate weighted sum correctly with default weights', () => {
      const dimensions = {
        context: 80,
        atomicity: 90,
        boundary: 75,
        verifiability: 85,
        technical: 88
      };
      
      const result = calculateWeightedSum(dimensions, DEFAULT_WEIGHTS);
      
      const expected = 80 * 0.25 + 90 * 0.25 + 75 * 0.20 + 85 * 0.15 + 88 * 0.15;
      expect(result).toBe(expected);
    });

    it('should handle custom weights', () => {
      const dimensions = {
        context: 80,
        atomicity: 90,
        boundary: 75,
        verifiability: 85,
        technical: 88
      };
      
      const customWeights = {
        context: 0.30,
        atomicity: 0.30,
        boundary: 0.15,
        verifiability: 0.15,
        technical: 0.10
      };
      
      const result = calculateWeightedSum(dimensions, customWeights);
      
      const expected = 80 * 0.30 + 90 * 0.30 + 75 * 0.15 + 85 * 0.15 + 88 * 0.10;
      expect(result).toBe(expected);
    });

    it('should return 0 for all zero dimensions', () => {
      const dimensions = {
        context: 0,
        atomicity: 0,
        boundary: 0,
        verifiability: 0,
        technical: 0
      };
      
      const result = calculateWeightedSum(dimensions, DEFAULT_WEIGHTS);
      expect(result).toBe(0);
    });
  });

  describe('calculateFinalScore', () => {
    it('should apply simple penalty correctly', () => {
      const weightedSum = 80;
      const result = calculateFinalScore(weightedSum, 'simple');
      
      expect(result).toBe(80 * COMPLEXITY_PENALTIES.simple);
    });

    it('should apply medium penalty correctly', () => {
      const weightedSum = 80;
      const result = calculateFinalScore(weightedSum, 'medium');
      
      expect(result).toBe(80 * COMPLEXITY_PENALTIES.medium);
    });

    it('should apply complex penalty correctly', () => {
      const weightedSum = 80;
      const result = calculateFinalScore(weightedSum, 'complex');
      
      expect(result).toBe(80 * COMPLEXITY_PENALTIES.complex);
    });

    it('should round to 1 decimal place', () => {
      const weightedSum = 85.123456;
      const result = calculateFinalScore(weightedSum, 'simple');
      
      expect(result).toBe(85.1);
    });
  });

  describe('calculateGrade', () => {
    it('should return S for score >= 90', () => {
      expect(calculateGrade(90)).toBe('S');
      expect(calculateGrade(100)).toBe('S');
    });

    it('should return A for score >= 75 and < 90', () => {
      expect(calculateGrade(75)).toBe('A');
      expect(calculateGrade(89.9)).toBe('A');
    });

    it('should return B for score >= 60 and < 75', () => {
      expect(calculateGrade(60)).toBe('B');
      expect(calculateGrade(74.9)).toBe('B');
    });

    it('should return C for score < 60', () => {
      expect(calculateGrade(0)).toBe('C');
      expect(calculateGrade(59.9)).toBe('C');
    });
  });

  describe('ScoreCalculator class', () => {
    it('should calculate complete evaluation result', () => {
      const calculator = new ScoreCalculator();
      
      const dimensions = {
        context: 80,
        atomicity: 90,
        boundary: 75,
        verifiability: 85,
        technical: 88
      };
      
      const result = calculator.calculate({
        requirementId: 'req_123',
        dimensions,
        complexity: 'medium',
        llmProvider: 'openai',
        llmModel: 'gpt-4o'
      });

      expect(result.finalScore).toBeCloseTo(75.1, 0);
      expect(result.grade).toBe('A');
      expect(result.complexityPenalty).toBe(0.9);
      expect(result.complexity).toBe('medium');
    });

    it('should generate unique id', () => {
      const calculator = new ScoreCalculator();
      
      const result1 = calculator.calculate({
        requirementId: 'req_123',
        dimensions: { context: 80, atomicity: 80, boundary: 80, verifiability: 80, technical: 80 },
        complexity: 'simple',
        llmProvider: 'openai',
        llmModel: 'gpt-4o'
      });

      const result2 = calculator.calculate({
        requirementId: 'req_456',
        dimensions: { context: 80, atomicity: 80, boundary: 80, verifiability: 80, technical: 80 },
        complexity: 'simple',
        llmProvider: 'openai',
        llmModel: 'gpt-4o'
      });

      expect(result1.id).not.toBe(result2.id);
    });
  });
});