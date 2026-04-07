import { LLMGateway } from './llmGateway';
import { ScoreCalculator } from './scoreCalculator';
import { ComplexityAnalyzer } from './complexityAnalyzer';
import { LLMConfig, EvaluationResult, DimensionScore, Dimension } from '../types';

export interface EvaluateInput {
  requirementId: string;
  content: string;
}

export class EvaluationEngine {
  private llmGateway: LLMGateway;
  private scoreCalculator: ScoreCalculator;
  private complexityAnalyzer: ComplexityAnalyzer;

  constructor(llmConfig: LLMConfig) {
    this.llmGateway = new LLMGateway(llmConfig);
    this.scoreCalculator = new ScoreCalculator();
    this.complexityAnalyzer = new ComplexityAnalyzer();
  }

  async evaluate(input: EvaluateInput): Promise<EvaluationResult> {
    const dimensionScores = await this.llmGateway.evaluateDimensions(input.content);
    
    const complexityResult = this.complexityAnalyzer.analyze(input.content);
    const dimensionNumbers = this.calculateDimensions(dimensionScores);

    const evaluationResult = this.scoreCalculator.calculate({
      requirementId: input.requirementId,
      dimensions: dimensionNumbers,
      complexity: complexityResult.level,
      llmProvider: this.llmGateway.getConfig().provider,
      llmModel: this.llmGateway.getConfig().model
    });

    evaluationResult.dimensions = dimensionScores;
    return evaluationResult;
  }

  private calculateDimensions(scores: Record<Dimension, DimensionScore>): Record<Dimension, number> {
    return {
      context: scores.context.score,
      atomicity: scores.atomicity.score,
      boundary: scores.boundary.score,
      verifiability: scores.verifiability.score,
      technical: scores.technical.score
    };
  }
}