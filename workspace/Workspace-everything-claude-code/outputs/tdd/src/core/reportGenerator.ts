import { EvaluationResult, OptimizationSuggestion, Dimension } from '../types';

export interface RadarDataPoint {
  axis: string;
  value: number;
}

export interface DimensionData {
  dimension: string;
  score: number;
  analysis: string;
  riskPoints: string[];
}

export interface ReportSummary {
  finalScore: number;
  grade: string;
  complexity: string;
  complexityPenalty: number;
  llmProvider: string;
  llmModel: string;
}

export interface EvaluationReport {
  summary: ReportSummary;
  dimensions: DimensionData[];
  radarData: RadarDataPoint[];
  suggestions: OptimizationSuggestion[];
}

export class ReportGenerator {
  private static readonly DIMENSION_LABELS: Record<Dimension, string> = {
    context: '上下文完备性',
    atomicity: '逻辑原子性',
    boundary: '边界明确性',
    verifiability: '可验证性',
    technical: '技术约束'
  };

  generateReport(evaluation: EvaluationResult, suggestions: OptimizationSuggestion[]): EvaluationReport {
    const sortedSuggestions = this.sortSuggestionsByPriority(suggestions);
    const dimensionLabels = Object.keys(ReportGenerator.DIMENSION_LABELS) as Dimension[];

    return {
      summary: {
        finalScore: evaluation.finalScore,
        grade: evaluation.grade,
        complexity: evaluation.complexity,
        complexityPenalty: evaluation.complexityPenalty,
        llmProvider: evaluation.llmProvider,
        llmModel: evaluation.llmModel
      },
      dimensions: dimensionLabels.map(dim => ({
        dimension: ReportGenerator.DIMENSION_LABELS[dim],
        score: evaluation.dimensions[dim].score,
        analysis: evaluation.dimensions[dim].analysis,
        riskPoints: evaluation.dimensions[dim].riskPoints
      })),
      radarData: dimensionLabels.map(dim => ({
        axis: ReportGenerator.DIMENSION_LABELS[dim],
        value: evaluation.dimensions[dim].score
      })),
      suggestions: sortedSuggestions
    };
  }

  private sortSuggestionsByPriority(suggestions: OptimizationSuggestion[]): OptimizationSuggestion[] {
    const priorityOrder = { high: 0, medium: 1, low: 2 };
    return [...suggestions].sort((a, b) => priorityOrder[a.priority] - priorityOrder[b.priority]);
  }

  toJSON(report: EvaluationReport): string {
    return JSON.stringify(report, null, 2);
  }
}